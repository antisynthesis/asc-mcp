//! Multi-user HTTP deployment: OAuth resource-server behaviour, per-request
//! App Store Connect credentials, upload restrictions, per-caller limits
//! and session ownership.

mod common;

use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use asc_mcp::api::testutil::{MockApi, MockResponse, TEST_PRIVATE_KEY};
use asc_mcp::context::UploadPolicy;
use asc_mcp::mcp;
use asc_mcp::server::http::{CredentialsMode, HttpOptions, HttpServer};
use asc_mcp::server::oauth::{OAuthConfig, Verifier};
use asc_mcp::tools::Registry;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use base64::Engine;
use common::*;
use jsonwebtoken::{EncodingKey, Header};
use serde_json::{Value, json};
use tower::ServiceExt;

const ISSUER: &str = "https://auth.example.com";
const AUDIENCE: &str = "https://mcp.example.com/mcp";
const LATEST: &str = mcp::LATEST_PROTOCOL_VERSION;

struct Resp {
    status: StatusCode,
    headers: axum::http::HeaderMap,
    body: Vec<u8>,
}

impl Resp {
    fn header(&self, name: &str) -> String {
        self.headers
            .get(name)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string()
    }
    fn json(&self) -> Value {
        serde_json::from_slice(&self.body).unwrap_or_else(|e| {
            panic!(
                "decode: {e} (body={:?})",
                String::from_utf8_lossy(&self.body)
            )
        })
    }
}

async fn send(srv: &HttpServer, req: Request<Body>) -> Resp {
    let r = srv.router().oneshot(req).await.expect("infallible");
    let status = r.status();
    let headers = r.headers().clone();
    let body = axum::body::to_bytes(r.into_body(), usize::MAX)
        .await
        .expect("body")
        .to_vec();
    Resp {
        status,
        headers,
        body,
    }
}

fn b64url(b: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(b)
}

/// The JWKS for the test signing key.
fn jwks() -> jsonwebtoken::jwk::JwkSet {
    use p256::elliptic_curve::sec1::ToEncodedPoint;
    use p256::pkcs8::DecodePrivateKey;
    let sk = p256::SecretKey::from_pkcs8_pem(TEST_PRIVATE_KEY).expect("test key");
    let point = sk.public_key().to_encoded_point(false);
    serde_json::from_value(json!({"keys": [{
        "kty": "EC", "crv": "P-256", "alg": "ES256", "use": "sig", "kid": "k1",
        "x": b64url(point.x().expect("x")), "y": b64url(point.y().expect("y")),
    }]}))
    .expect("jwks")
}

fn oauth_cfg() -> OAuthConfig {
    OAuthConfig {
        issuer: ISSUER.into(),
        audience: AUDIENCE.into(),
        jwks_url: None,
        read_scope: "asc:read".into(),
        write_scope: "asc:write".into(),
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_secs()
}

/// Mints an access token; `claims` overrides the defaults.
fn token(claims: Value) -> String {
    let mut c = json!({
        "iss": ISSUER, "aud": AUDIENCE, "sub": "user-1", "client_id": "agent-a",
        "exp": now() + 600, "iat": now(), "scope": "asc:read asc:write",
    });
    for (k, v) in claims.as_object().expect("object") {
        if v.is_null() {
            c.as_object_mut().expect("object").remove(k);
        } else {
            c[k] = v.clone();
        }
    }
    let mut h = Header::new(jsonwebtoken::Algorithm::ES256);
    h.kid = Some("k1".into());
    let key = EncodingKey::from_ec_pem(TEST_PRIVATE_KEY.as_bytes()).expect("encoding key");
    jsonwebtoken::encode(&h, &c, &key).expect("sign")
}

fn oauth_server(registry: Registry, mut opts: HttpOptions) -> HttpServer {
    opts.oauth = Some(Verifier::with_keys(oauth_cfg(), jwks()));
    HttpServer::new(Arc::new(registry), opts)
}

fn modern_req(id: i64, method: &str, params: Value) -> Value {
    let mut p = params;
    p["_meta"] = json!({
        "io.modelcontextprotocol/protocolVersion": LATEST,
        "io.modelcontextprotocol/clientCapabilities": {},
    });
    json!({"jsonrpc": "2.0", "id": id, "method": method, "params": p})
}

/// POSTs a modern request with the mirrored headers plus `extra`.
async fn post_modern(srv: &HttpServer, body: Value, extra: &[(&str, String)]) -> Resp {
    let method = body["method"].as_str().unwrap_or("").to_string();
    let mut b = Request::builder()
        .method(Method::POST)
        .uri("/mcp")
        .header("Content-Type", "application/json")
        .header("MCP-Protocol-Version", LATEST)
        .header("Mcp-Method", &method);
    if let Some(name) = body["params"]["name"].as_str() {
        b = b.header("Mcp-Name", name);
    }
    for (k, v) in extra {
        b = b.header(*k, v);
    }
    send(srv, b.body(Body::from(body.to_string())).expect("request")).await
}

fn bearer(t: &str) -> (&'static str, String) {
    ("Authorization", format!("Bearer {t}"))
}

#[tokio::test]
async fn protected_resource_metadata_is_served() {
    let srv = oauth_server(Registry::new(None), HttpOptions::default());
    for path in [
        "/.well-known/oauth-protected-resource",
        "/.well-known/oauth-protected-resource/mcp",
    ] {
        let r = send(&srv, Request::get(path).body(Body::empty()).expect("req")).await;
        assert_eq!(r.status, StatusCode::OK, "{path}");
        let doc = r.json();
        assert_eq!(doc["resource"], AUDIENCE);
        assert_eq!(doc["authorization_servers"], json!([ISSUER]));
        assert_eq!(doc["scopes_supported"], json!(["asc:read", "asc:write"]));
        assert_eq!(doc["bearer_methods_supported"], json!(["header"]));
    }
}

#[tokio::test]
async fn missing_token_gets_a_discovery_challenge() {
    let srv = oauth_server(Registry::new(None), HttpOptions::default());
    let r = post_modern(&srv, modern_req(1, "tools/list", json!({})), &[]).await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
    let www = r.header("www-authenticate");
    assert!(www.starts_with("Bearer "), "{www}");
    assert!(
        www.contains(
            "resource_metadata=\"https://mcp.example.com/.well-known/oauth-protected-resource/mcp\""
        ),
        "{www}"
    );
    assert!(www.contains("scope=\"asc:read\""), "{www}");
}

#[tokio::test]
async fn invalid_tokens_are_rejected_with_401() {
    let srv = oauth_server(Registry::new(None), HttpOptions::default());
    let cases = [
        ("garbage", "not-a-jwt".to_string()),
        (
            "wrong audience",
            token(json!({"aud": "https://other.example.com/mcp"})),
        ),
        (
            "wrong issuer",
            token(json!({"iss": "https://evil.example.com"})),
        ),
        ("expired", token(json!({"exp": now() - 3600}))),
        ("not yet valid", token(json!({"nbf": now() + 3600}))),
        ("no expiry", token(json!({"exp": null}))),
    ];
    for (name, t) in cases {
        let r = post_modern(&srv, modern_req(1, "tools/list", json!({})), &[bearer(&t)]).await;
        assert_eq!(r.status, StatusCode::UNAUTHORIZED, "{name}");
        assert!(
            r.header("www-authenticate")
                .contains("error=\"invalid_token\""),
            "{name}"
        );
    }
    // An HS256 token signed with the public key as an HMAC secret (the
    // classic algorithm-confusion attack) must not verify.
    let mut h = Header::new(jsonwebtoken::Algorithm::HS256);
    h.kid = Some("k1".into());
    let forged = jsonwebtoken::encode(
        &h,
        &json!({"iss": ISSUER, "aud": AUDIENCE, "sub": "x", "exp": now() + 600}),
        &EncodingKey::from_secret(b"secret"),
    )
    .expect("sign");
    let r = post_modern(
        &srv,
        modern_req(1, "tools/list", json!({})),
        &[bearer(&forged)],
    )
    .await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn valid_token_is_accepted() {
    let srv = oauth_server(Registry::new(None), HttpOptions::default());
    let r = post_modern(
        &srv,
        modern_req(1, "tools/list", json!({})),
        &[bearer(&token(json!({})))],
    )
    .await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(
        r.json()["result"]["tools"].as_array().map(Vec::len),
        Some(356)
    );
}

#[tokio::test]
async fn write_tools_require_the_write_scope() {
    let srv = oauth_server(Registry::new(None), HttpOptions::default());
    let read_only = token(json!({"scope": "asc:read"}));
    // delete_app_store_version changes state: step-up challenge.
    let r = post_modern(
        &srv,
        modern_req(
            1,
            "tools/call",
            json!({"name": "delete_app_store_version", "arguments": {}}),
        ),
        &[bearer(&read_only)],
    )
    .await;
    assert_eq!(r.status, StatusCode::FORBIDDEN);
    let www = r.header("www-authenticate");
    assert!(www.contains("error=\"insufficient_scope\""), "{www}");
    assert!(www.contains("scope=\"asc:write\""), "{www}");
    assert!(www.contains("resource_metadata="), "{www}");
    // get_app is read-only: allowed (reaches the handler, which validates).
    let r = post_modern(
        &srv,
        modern_req(2, "tools/call", json!({"name": "get_app", "arguments": {}})),
        &[bearer(&read_only)],
    )
    .await;
    assert_eq!(r.status, StatusCode::OK);
    // A token without any granted scope cannot even list tools.
    let none = token(json!({"scope": "profile"}));
    let r = post_modern(
        &srv,
        modern_req(3, "tools/list", json!({})),
        &[bearer(&none)],
    )
    .await;
    assert_eq!(r.status, StatusCode::FORBIDDEN);
    assert!(r.header("www-authenticate").contains("scope=\"asc:read\""));
    // The write scope implies read; `scp` arrays are accepted too.
    let scp = token(json!({"scope": null, "scp": ["asc:write"]}));
    let r = post_modern(
        &srv,
        modern_req(4, "tools/list", json!({})),
        &[bearer(&scp)],
    )
    .await;
    assert_eq!(r.status, StatusCode::OK);
}

#[tokio::test]
async fn per_request_credentials_are_used_and_isolated() {
    let mock = MockApi::start(|_| {
        MockResponse::json(
            200,
            r#"{"data":{"type":"apps","id":"1","attributes":{"name":"Demo"}}}"#,
        )
    })
    .await;
    // SAFETY: this is the only test in this binary that reads the base URL.
    unsafe { std::env::set_var(asc_mcp::api::client::BASE_URL_ENV, &mock.base_url) };
    let srv = oauth_server(
        Registry::new(None),
        HttpOptions {
            credentials: CredentialsMode::Request,
            ..Default::default()
        },
    );
    let key_b64 = base64::engine::general_purpose::STANDARD.encode(TEST_PRIVATE_KEY);
    let call = |id: i64| {
        modern_req(
            id,
            "tools/call",
            json!({"name": "get_app", "arguments": {"app_id": "1"}}),
        )
    };
    let creds = |kid: &str| {
        vec![
            bearer(&token(json!({}))),
            ("X-ASC-Issuer-ID", "issuer-1".to_string()),
            ("X-ASC-Key-ID", kid.to_string()),
            ("X-ASC-Private-Key", key_b64.clone()),
        ]
    };
    // Two callers with different keys: each upstream request is signed
    // with the caller's own key id.
    for (i, kid) in ["KEYAAAA111", "KEYBBBB222"].iter().enumerate() {
        let r = post_modern(&srv, call(i as i64), &creds(kid)).await;
        assert_eq!(r.status, StatusCode::OK);
        assert_eq!(r.json()["result"]["isError"], Value::Null, "{}", r.json());
    }
    let kids: Vec<String> = mock
        .requests()
        .iter()
        .map(|req| {
            let jwt = req
                .header("authorization")
                .unwrap_or("")
                .trim_start_matches("Bearer ");
            let header = base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(jwt.split('.').next().unwrap_or(""))
                .expect("jwt header");
            serde_json::from_slice::<Value>(&header).expect("json")["kid"]
                .as_str()
                .unwrap_or("")
                .to_string()
        })
        .collect();
    assert_eq!(kids, vec!["KEYAAAA111", "KEYBBBB222"]);
    // The caller's OAuth token is never forwarded upstream.
    for req in mock.requests() {
        let auth = req.header("authorization").unwrap_or("");
        assert!(
            !auth.contains(&token(json!({}))[..20]),
            "access token must not be passed through"
        );
    }
    // Missing credentials: a tool error naming the headers, no API call.
    let before = mock.requests().len();
    let r = post_modern(&srv, call(9), &[bearer(&token(json!({})))]).await;
    let res = r.json()["result"].clone();
    assert_eq!(res["isError"], true);
    assert!(
        res["content"][0]["text"]
            .as_str()
            .unwrap_or("")
            .contains("X-ASC-Private-Key")
    );
    // Undecodable key material is reported, not used.
    let mut bad = creds("KEYCCCC333");
    bad[3].1 = "%%%".into();
    let r = post_modern(&srv, call(10), &bad).await;
    assert_eq!(r.json()["result"]["isError"], true);
    assert_eq!(mock.requests().len(), before);
}

#[tokio::test]
async fn file_path_uploads_follow_the_upload_policy() {
    let dir = tempfile::tempdir().expect("tempdir");
    let inside = dir.path().join("shot.png");
    std::fs::write(&inside, b"png").expect("write");
    let outside_dir = tempfile::tempdir().expect("tempdir");
    let outside = outside_dir.path().join("secret.txt");
    std::fs::write(&outside, b"secret").expect("write");
    let link = dir.path().join("escape.png");
    std::os::unix::fs::symlink(&outside, &link).expect("symlink");

    let upload = |id: i64, path: &std::path::Path| {
        modern_req(
            id,
            "tools/call",
            json!({"name": "upload_app_screenshot", "arguments": {
                "screenshot_set_id": "set-1", "file_name": "a.png", "file_path": path.to_string_lossy(),
            }}),
        )
    };
    let text = |r: &Resp| {
        r.json()["result"]["content"][0]["text"]
            .as_str()
            .unwrap_or("")
            .to_string()
    };

    // Default over HTTP: local paths are refused.
    let srv = HttpServer::new(
        Arc::new(Registry::new(Some(dummy_client()))),
        HttpOptions::default(),
    );
    let r = post_modern(&srv, upload(1, &inside), &[]).await;
    assert!(
        text(&r).contains("file_path is not available"),
        "{}",
        text(&r)
    );

    // With an upload directory: inside passes the policy (and then fails
    // against the unreachable dummy API), outside and symlink escapes fail.
    let root = std::fs::canonicalize(dir.path()).expect("canonical");
    let srv = HttpServer::new(
        Arc::new(Registry::new(Some(dummy_client()))),
        HttpOptions {
            uploads: UploadPolicy::Within(root),
            ..Default::default()
        },
    );
    let r = post_modern(&srv, upload(2, &inside), &[]).await;
    assert!(
        text(&r).starts_with("Failed to upload screenshot"),
        "{}",
        text(&r)
    );
    for p in [&outside, &link] {
        let r = post_modern(&srv, upload(3, p), &[]).await;
        assert!(
            text(&r).contains("must be inside the server's upload directory"),
            "{}",
            text(&r)
        );
    }
}

#[tokio::test]
async fn concurrent_requests_are_limited_per_caller() {
    let mut reg = empty_registry();
    register_fake(&mut reg, "slow", || async {
        tokio::time::sleep(Duration::from_millis(300)).await;
        Ok(asc_mcp::mcp::CallToolResult::success("done"))
    });
    let srv = HttpServer::new(
        Arc::new(reg),
        HttpOptions {
            auth_tokens: vec!["tok-a".into(), "tok-b".into()],
            max_concurrent_per_caller: 1,
            ..Default::default()
        },
    );
    let call = || modern_req(1, "tools/call", json!({"name": "slow", "arguments": {}}));
    let a_headers = [bearer("tok-a")];
    let a1 = post_modern(&srv, call(), &a_headers);
    let a2 = async {
        tokio::time::sleep(Duration::from_millis(50)).await;
        post_modern(&srv, call(), &[bearer("tok-a")]).await
    };
    let b1 = async {
        tokio::time::sleep(Duration::from_millis(50)).await;
        post_modern(&srv, call(), &[bearer("tok-b")]).await
    };
    let (a1, a2, b1) = tokio::join!(a1, a2, b1);
    assert_eq!(a1.status, StatusCode::OK);
    assert_eq!(a2.status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(a2.header("retry-after"), "1");
    assert_eq!(b1.status, StatusCode::OK, "another caller is not affected");
}

#[tokio::test]
async fn sessions_belong_to_their_creator() {
    let srv = HttpServer::new(
        Arc::new(Registry::new(None)),
        HttpOptions {
            auth_tokens: vec!["tok-a".into(), "tok-b".into()],
            ..Default::default()
        },
    );
    let legacy = |body: Value, headers: Vec<(&'static str, String)>| {
        let mut b = Request::builder()
            .method(Method::POST)
            .uri("/mcp")
            .header("Content-Type", "application/json");
        for (k, v) in headers {
            b = b.header(k, v);
        }
        b.body(Body::from(body.to_string())).expect("request")
    };
    let init = json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-11-25"}});
    let r = send(&srv, legacy(init, vec![bearer("tok-a")])).await;
    let sid = r.header("mcp-session-id");
    assert!(!sid.is_empty());
    let ping = json!({"jsonrpc": "2.0", "id": 2, "method": "ping"});
    let r = send(
        &srv,
        legacy(
            ping.clone(),
            vec![bearer("tok-b"), ("Mcp-Session-Id", sid.clone())],
        ),
    )
    .await;
    assert_eq!(
        r.status,
        StatusCode::NOT_FOUND,
        "another caller cannot use the session"
    );
    let del = Request::builder()
        .method(Method::DELETE)
        .uri("/mcp")
        .header("Authorization", "Bearer tok-b")
        .header("Mcp-Session-Id", &sid)
        .body(Body::empty())
        .expect("request");
    assert_eq!(
        send(&srv, del).await.status,
        StatusCode::NOT_FOUND,
        "or end it"
    );
    let r = send(
        &srv,
        legacy(ping, vec![bearer("tok-a"), ("Mcp-Session-Id", sid)]),
    )
    .await;
    assert_eq!(r.status, StatusCode::OK);
}

#[tokio::test]
async fn oversized_bodies_respect_the_configured_limit() {
    let srv = HttpServer::new(
        Arc::new(Registry::new(None)),
        HttpOptions {
            max_body_bytes: 64,
            ..Default::default()
        },
    );
    let r = post_modern(
        &srv,
        modern_req(1, "tools/list", json!({"pad": "x".repeat(100)})),
        &[],
    )
    .await;
    assert_eq!(r.status, StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn jwks_is_discovered_from_issuer_metadata() {
    let good = mock_issuer(None).await;
    let v = Verifier::new(OAuthConfig {
        issuer: good.base_url.clone(),
        ..oauth_cfg()
    });
    let caller = v
        .verify(&token(json!({"iss": good.base_url, "scope": "asc:read"})))
        .await
        .expect("valid");
    assert_eq!(caller.subject.as_deref(), Some("user-1"));
    assert_eq!(caller.client_id.as_deref(), Some("agent-a"));
    assert_eq!(caller.scopes, vec!["asc:read"]);

    // Metadata whose `issuer` differs from the configured issuer must be
    // ignored (RFC 8414 §3.3), so no keys are trusted.
    let spoofed = mock_issuer(Some("https://other.example.com")).await;
    let v = Verifier::new(OAuthConfig {
        issuer: spoofed.base_url.clone(),
        ..oauth_cfg()
    });
    let err = v
        .verify(&token(json!({"iss": spoofed.base_url})))
        .await
        .unwrap_err();
    assert_eq!(err.0, "authorization server keys are unavailable");
}

/// A mock authorization server. Its metadata names itself as issuer, or
/// `claimed_issuer` when given.
async fn mock_issuer(claimed_issuer: Option<&'static str>) -> MockApi {
    let jwks_doc = serde_json::to_string(&jwks()).expect("jwks");
    let base = Arc::new(std::sync::OnceLock::<String>::new());
    let b2 = base.clone();
    let mock = MockApi::start(move |req| {
        let origin = b2.get().cloned().unwrap_or_default();
        let issuer = claimed_issuer
            .map(str::to_string)
            .unwrap_or_else(|| origin.clone());
        match req.path.as_str() {
            "/.well-known/oauth-authorization-server" => MockResponse::json(
                200,
                json!({"issuer": issuer, "jwks_uri": format!("{origin}/jwks")}).to_string(),
            ),
            "/jwks" => MockResponse::json(200, jwks_doc.clone()),
            _ => MockResponse::empty(404),
        }
    })
    .await;
    let _ = base.set(mock.base_url.clone());
    mock
}
