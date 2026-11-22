//! TLS tests for the HTTP transport.

mod common;

use std::sync::Arc;
use std::time::{Duration, Instant};

use asc_mcp::server::http::{HttpOptions, HttpServer, validate_tls_pair};
use asc_mcp::tools::Registry;
use serde_json::json;

// A self-signed P-256 cert for 127.0.0.1/localhost. Without an X.509
// builder in the dependency tree, a fixture (generated with openssl, valid
// until 2126, SEC1 "EC PRIVATE KEY") is embedded instead of generating one
// at test time.
const TEST_CERT: &str = "-----BEGIN CERTIFICATE-----
MIIBxDCCAWugAwIBAgIULE0y+t9DAAqvKmaN5ZbDamxlTcAwCgYIKoZIzj0EAwIw
FzEVMBMGA1UEAwwMYXNjLW1jcC10ZXN0MCAXDTI2MTEyMjE2MTcyMVoYDzIxMjYx
MDI5MTYxNzIxWjAXMRUwEwYDVQQDDAxhc2MtbWNwLXRlc3QwWTATBgcqhkjOPQIB
BggqhkjOPQMBBwNCAARnB3sKh2cyvo5So4icF4YoHmbSnLNOmBRJJDvYTxb1mXa/
NDYG6ZgyFqrblMIF9x0c1vXGeGskEU8HNExLlaAco4GSMIGPMB0GA1UdDgQWBBSP
JibG9JJxcTl+at18YtzaH6+fxjAfBgNVHSMEGDAWgBSPJibG9JJxcTl+at18Ytza
H6+fxjAPBgNVHRMBAf8EBTADAQH/MBoGA1UdEQQTMBGHBH8AAAGCCWxvY2FsaG9z
dDATBgNVHSUEDDAKBggrBgEFBQcDATALBgNVHQ8EBAMCBaAwCgYIKoZIzj0EAwID
RwAwRAIgNM9124eRVdvlgog+UgRtmvgDfceWRUNpItmeLrw+454CIEZz9yKX3/t/
munD5EA3Fnk7UsRCoOHl+UQ0hzUgMA5+
-----END CERTIFICATE-----
";

const TEST_KEY: &str = "-----BEGIN EC PRIVATE KEY-----
MHcCAQEEIHAsiATQWTs64WapF8mLCT+woApGnkKDpB3dY6BbKe2moAoGCCqGSM49
AwEHoUQDQgAEZwd7CodnMr6OUqOInBeGKB5m0pyzTpgUSSQ72E8W9Zl2vzQ2BumY
Mhaq25TCBfcdHNb1xnhrJBFPBzRMS5WgHA==
-----END EC PRIVATE KEY-----
";

/// Writes the cert and key into a temp dir; returns (dir, cert, key).
fn write_cert() -> (tempfile::TempDir, String, String) {
    let dir = tempfile::tempdir().unwrap();
    let cert = dir.path().join("c.pem");
    let key = dir.path().join("k.pem");
    std::fs::write(&cert, TEST_CERT).unwrap();
    std::fs::write(&key, TEST_KEY).unwrap();
    (
        dir,
        cert.to_string_lossy().into_owned(),
        key.to_string_lossy().into_owned(),
    )
}

fn plain_client() -> reqwest::Client {
    reqwest::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap()
}

fn server() -> HttpServer {
    HttpServer::new(Arc::new(Registry::new(None)), HttpOptions::default())
}

#[test]
fn validate_tls_pair_cases() {
    let (dir, cert, key) = write_cert();
    // neither set means plain HTTP
    assert_eq!(validate_tls_pair("", ""), Ok(false));
    // only one is an error
    assert!(validate_tls_pair(&cert, "").is_err(), "only cert");
    assert!(validate_tls_pair("", &key).is_err(), "only key");
    // missing cert file
    let missing = dir.path().join("nope.pem").to_string_lossy().into_owned();
    let err = validate_tls_pair(&missing, &key).unwrap_err();
    assert!(
        err.contains("tls cert"),
        "error {err:?} does not mention tls cert"
    );
    // both set and readable
    assert_eq!(validate_tls_pair(&cert, &key), Ok(true));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn listen_and_serve_tls() {
    let (_dir, cert, key) = write_cert();
    // Pick an ephemeral port by binding briefly.
    let addr = {
        let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        l.local_addr().unwrap().to_string()
    };
    let srv = server();
    let (stop_tx, stop_rx) = tokio::sync::oneshot::channel::<()>();
    let serve = {
        let (addr, cert, key) = (addr.clone(), cert.clone(), key.clone());
        tokio::spawn(async move {
            srv.listen_and_serve(&addr, &cert, &key, async {
                let _ = stop_rx.await;
            })
            .await
        })
    };

    // rustls never negotiates below TLS 1.2; pinning the client minimum
    // makes the ">= TLS 1.2" requirement explicit.
    let client = reqwest::Client::builder()
        .no_proxy()
        .danger_accept_invalid_certs(true)
        .min_tls_version(reqwest::tls::Version::TLS_1_2)
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();
    let url = format!("https://{addr}/healthz");
    let deadline = Instant::now() + Duration::from_secs(5);
    let resp = loop {
        match client.get(&url).send().await {
            Ok(r) => break r,
            Err(e) if Instant::now() > deadline => panic!("never reachable: {e}"),
            Err(_) => tokio::time::sleep(Duration::from_millis(20)).await,
        }
    };
    assert_eq!(resp.status(), 200);
    // Plain HTTP against the TLS port must not be served.
    assert!(
        plain_client()
            .get(format!("http://{addr}/healthz"))
            .send()
            .await
            .map(|r| r.status() != 200)
            .unwrap_or(true)
    );

    let body = json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":common::init_params("2025-06-18")});
    let mcp = client
        .post(format!("https://{addr}/mcp"))
        .json(&body)
        .send()
        .await
        .expect("post");
    assert_eq!(mcp.status(), 200);
    let text = mcp.text().await.unwrap();
    assert!(
        text.contains("2025-06-18"),
        "response body does not include negotiated protocol: {text}"
    );

    stop_tx.send(()).unwrap();
    let res = tokio::time::timeout(Duration::from_secs(15), serve)
        .await
        .expect("server did not shut down in time");
    res.unwrap().expect("shutdown error");
}

#[tokio::test]
async fn listen_and_serve_tls_config_error() {
    let err = server()
        .listen_and_serve(
            "127.0.0.1:0",
            "/does/not/matter.pem",
            "",
            std::future::pending(),
        )
        .await
        .unwrap_err();
    assert!(
        err.contains("tls config invalid"),
        "error {err:?} does not mention tls config"
    );
}

/// Port-only ":port" addresses are accepted (and listen on all interfaces)
/// even though the default is 127.0.0.1:8080.
#[tokio::test]
async fn listen_accepts_port_only_addr() {
    let port = {
        let l = std::net::TcpListener::bind("0.0.0.0:0").unwrap();
        l.local_addr().unwrap().port()
    };
    let (stop_tx, stop_rx) = tokio::sync::oneshot::channel::<()>();
    let srv = server();
    let serve = tokio::spawn(async move {
        srv.listen_and_serve(&format!(":{port}"), "", "", async {
            let _ = stop_rx.await;
        })
        .await
    });
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        match plain_client()
            .get(format!("http://127.0.0.1:{port}/healthz"))
            .send()
            .await
        {
            Ok(r) => {
                assert_eq!(r.status(), 200);
                break;
            }
            Err(e) if Instant::now() > deadline => panic!("never reachable: {e}"),
            Err(_) => tokio::time::sleep(Duration::from_millis(20)).await,
        }
    }
    stop_tx.send(()).unwrap();
    tokio::time::timeout(Duration::from_secs(15), serve)
        .await
        .expect("shutdown")
        .unwrap()
        .unwrap();
}
