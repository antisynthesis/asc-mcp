//! API client tests, ported from internal/asc/api/client_test.go and the
//! PerformUploadOperations / Sha256Hex parts of upload_test.go.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use asc_mcp::api::client::{MAX_RESPONSE_SIZE, USER_AGENT};
use asc_mcp::api::model::{RequestHeader, UploadOperation};
use asc_mcp::api::testutil::{MockApi, MockResponse, test_token_provider};
use asc_mcp::api::{Client, ListOptions, sha256_hex};
use asc_mcp::go::url::Values;
use serde_json::{Value, json};

// Go: TestClient_Get
#[tokio::test]
async fn get() {
    let mock = MockApi::start(|_| MockResponse::json(200, r#"{"status":"ok"}"#)).await;
    let data = mock.client().get("/test", None).await.expect("get");
    let v: Value = serde_json::from_slice(&data).unwrap();
    assert_eq!(v["status"], "ok");
    let r = &mock.requests()[0];
    assert_eq!(r.method, "GET");
    let auth = r
        .header("authorization")
        .expect("missing Authorization header");
    assert!(
        auth.starts_with("Bearer ") && auth.len() > 7,
        "Authorization = {auth:?}"
    );
    assert_eq!(r.header("content-type"), Some("application/json"));
}

// Go: TestClient_Get_WithQuery
#[tokio::test]
async fn get_with_query() {
    let mock = MockApi::start(|_| MockResponse::json(200, "{}")).await;
    let mut q = Values::new();
    q.set("limit", "10");
    q.set("filter", "test");
    mock.client().get("/test", Some(&q)).await.expect("get");
    let r = &mock.requests()[0];
    assert_eq!(r.query_param("limit").as_deref(), Some("10"));
    assert_eq!(r.query_param("filter").as_deref(), Some("test"));
}

// Go: TestClient_Post
#[tokio::test]
async fn post() {
    let mock = MockApi::start(|_| MockResponse::json(201, r#"{"id":"123"}"#)).await;
    let data = mock
        .client()
        .post("/test", Some(&json!({"name": "test"})))
        .await
        .expect("post");
    assert_eq!(serde_json::from_slice::<Value>(&data).unwrap()["id"], "123");
    let r = &mock.requests()[0];
    assert_eq!(r.method, "POST");
    assert_eq!(r.json()["name"], "test");
}

// Go: TestClient_Delete
#[tokio::test]
async fn delete() {
    let mock = MockApi::start(|_| MockResponse::empty(204)).await;
    mock.client().delete("/test/123").await.expect("delete");
    let r = &mock.requests()[0];
    assert_eq!(
        (r.method.as_str(), r.path.as_str()),
        ("DELETE", "/test/123")
    );
}

// Go: TestClient_ErrorResponse
#[tokio::test]
async fn error_response() {
    for (name, status, body, contains) in [
        (
            "API error with details",
            400,
            r#"{"errors":[{"title":"Invalid Parameter","detail":"The value is not valid"}]}"#,
            "Invalid Parameter: The value is not valid",
        ),
        (
            "API error without details",
            400,
            r#"{"message": "internal error"}"#,
            "API error (400)",
        ),
        ("not found", 404, "{}", "API error (404)"),
    ] {
        // Go used 500 for the second case; 5xx is retried with backoff, so
        // a non-retryable status keeps this test fast. The 5xx path is
        // covered by `retries_5xx`.
        let mock = MockApi::start(move |_| MockResponse::json(status, body)).await;
        let err = mock.client().get("/test", None).await.expect_err(name);
        assert!(
            err.to_string().contains(contains),
            "{name}: error {err} does not contain {contains:?}"
        );
    }
}

/// The raw body is kept when it is not a JSON:API error document.
#[tokio::test]
async fn error_response_raw_body() {
    let mock = MockApi::start(|_| MockResponse::json(404, r#"{"message": "gone"}"#)).await;
    let err = mock.client().get("/test", None).await.unwrap_err();
    assert_eq!(err.to_string(), r#"API error (404): {"message": "gone"}"#);
}

// Go: TestClient_ContextCancellation. Rust callers cancel by dropping the
// future; a deadline around a request to a server that never answers
// must resolve promptly.
#[tokio::test]
async fn cancellation() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let _hold = tokio::spawn(async move {
        let mut conns = Vec::new();
        while let Ok((s, _)) = listener.accept().await {
            conns.push(s); // accept, never respond
        }
    });
    let client = Client::with_token_provider(test_token_provider(), format!("http://{addr}"));
    let started = std::time::Instant::now();
    let r = tokio::time::timeout(Duration::from_millis(10), client.get("/test", None)).await;
    assert!(r.is_err(), "expected the request to be cancelled");
    assert!(started.elapsed() < Duration::from_secs(5));
}

// Go: TestClient_ListApps
#[tokio::test]
async fn list_apps() {
    let mock = MockApi::start(|req| {
        assert_eq!(req.path, "/v1/apps");
        MockResponse::json(
            200,
            r#"{"data":[{"type":"apps","id":"123","attributes":{"name":"Test App","bundleId":"com.test.app","sku":"TEST123","primaryLocale":"en-US"}}],"links":{}}"#,
        )
    })
    .await;
    let resp = mock
        .client()
        .list_apps(Some(&ListOptions {
            limit: 50,
            ..Default::default()
        }))
        .await
        .expect("list");
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].attributes.name, "Test App");
    assert_eq!(
        mock.requests()[0].query_param("limit").as_deref(),
        Some("50")
    );
}

// Go: TestClient_GetApp
#[tokio::test]
async fn get_app() {
    let mock = MockApi::start(|req| {
        assert_eq!(req.path, "/v1/apps/123");
        MockResponse::json(200, r#"{"data":{"type":"apps","id":"123","attributes":{"name":"Test App","bundleId":"com.test.app"}}}"#)
    })
    .await;
    let resp = mock.client().get_app("123").await.expect("get");
    assert_eq!(resp.data.id, "123");
}

// Go: TestClient_ListBuilds (endpoint ported outside the core layer)
#[tokio::test]
#[ignore = "endpoint method ported outside the protocol/core layer"]
async fn list_builds() {
    let mock = MockApi::start(|req| {
        assert_eq!(req.path, "/v1/builds");
        assert_eq!(req.query_param("filter[app]").as_deref(), Some("app123"));
        MockResponse::json(
            200,
            r#"{"data":[{"type":"builds","id":"build1","attributes":{"version":"1.0.0","processingState":"VALID"}}],"links":{}}"#,
        )
    })
    .await;
    let resp = mock
        .client()
        .list_builds(
            "app123",
            Some(&ListOptions {
                limit: 50,
                ..Default::default()
            }),
        )
        .await
        .unwrap();
    assert_eq!(resp.data.len(), 1);
}

// Go: TestClient_ListBetaGroups (endpoint ported outside the core layer)
#[tokio::test]
#[ignore = "endpoint method ported outside the protocol/core layer"]
async fn list_beta_groups() {
    let mock = MockApi::start(|_| {
        MockResponse::json(
            200,
            r#"{"data":[{"type":"betaGroups","id":"group1","attributes":{"name":"External Testers","publicLinkEnabled":true}}],"links":{}}"#,
        )
    })
    .await;
    let resp = mock
        .client()
        .list_beta_groups(
            "",
            Some(&ListOptions {
                limit: 50,
                ..Default::default()
            }),
        )
        .await
        .unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].attributes.name, "External Testers");
}

// Go: TestClient_ListDevices (endpoint ported outside the core layer)
#[tokio::test]
#[ignore = "endpoint method ported outside the protocol/core layer"]
async fn list_devices() {
    let mock = MockApi::start(|_| {
        MockResponse::json(
            200,
            r#"{"data":[{"type":"devices","id":"device1","attributes":{"name":"iPhone 15","udid":"00000000-0000-0000-0000-000000000001","platform":"IOS","status":"ENABLED"}}],"links":{}}"#,
        )
    })
    .await;
    let resp = mock
        .client()
        .list_devices(Some(&ListOptions {
            limit: 50,
            ..Default::default()
        }))
        .await
        .unwrap();
    assert_eq!(resp.data.len(), 1);
}

// Go: TestClient_SendsAcceptAndUserAgent
#[tokio::test]
async fn sends_accept_and_user_agent() {
    let mock = MockApi::start(|_| MockResponse::json(200, "{}")).await;
    mock.client().get("/anything", None).await.expect("get");
    let r = &mock.requests()[0];
    assert_eq!(r.header("accept"), Some("application/json"));
    assert_eq!(r.header("user-agent"), Some(USER_AGENT));
    assert!(USER_AGENT.starts_with("asc-mcp/"));
}

// Go: TestClient_APIErrorIsTyped
#[tokio::test]
async fn api_error_is_typed() {
    let mock = MockApi::start(|_| {
        MockResponse::json(403, r#"{"errors":[{"title":"FORBIDDEN","detail":"nope"}]}"#)
    })
    .await;
    let err = mock.client().get("/x", None).await.unwrap_err();
    let api = err.api().expect("expected an ApiError in the chain");
    assert!(api.is_auth(), "403 should be classified as auth error");
    assert_eq!(api.status_code, 403);
    // Wrapping keeps the typed error reachable (Go errors.As through %w).
    let wrapped = err.wrap("failed to list");
    assert_eq!(wrapped.api().map(|a| a.status_code), Some(403));
    assert!(
        wrapped
            .to_string()
            .starts_with("failed to list: API error (403)")
    );
}

// Go: TestClient_Retries5xx
#[tokio::test]
async fn retries_5xx() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let a = attempts.clone();
    let mock = MockApi::start(move |_| {
        if a.fetch_add(1, Ordering::SeqCst) + 1 < 3 {
            MockResponse::empty(502)
        } else {
            MockResponse::json(200, r#"{"ok":true}"#)
        }
    })
    .await;
    mock.client()
        .get("/x", None)
        .await
        .expect("expected success after retries");
    assert_eq!(attempts.load(Ordering::SeqCst), 3);
}

/// 429 is retried too, honouring Retry-After.
#[tokio::test]
async fn retries_429_with_retry_after() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let a = attempts.clone();
    let mock = MockApi::start(move |_| {
        if a.fetch_add(1, Ordering::SeqCst) == 0 {
            MockResponse::json(429, "{}").with_header("Retry-After", "1")
        } else {
            MockResponse::json(200, "{}")
        }
    })
    .await;
    let started = std::time::Instant::now();
    mock.client()
        .get("/x", None)
        .await
        .expect("success after retry");
    assert_eq!(attempts.load(Ordering::SeqCst), 2);
    assert!(
        started.elapsed() >= Duration::from_millis(900),
        "Retry-After not honoured"
    );
}

// Go: TestClient_NoRetryOn400
#[tokio::test]
async fn no_retry_on_400() {
    let attempts = Arc::new(AtomicUsize::new(0));
    let a = attempts.clone();
    let mock = MockApi::start(move |_| {
        a.fetch_add(1, Ordering::SeqCst);
        MockResponse::json(400, "{}")
    })
    .await;
    assert!(mock.client().get("/x", None).await.is_err());
    assert_eq!(attempts.load(Ordering::SeqCst), 1, "4xx should not retry");
}

/// Non-idempotent methods are not retried on transport errors.
#[tokio::test]
async fn no_retry_post_on_transport_error() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let accepted = Arc::new(AtomicUsize::new(0));
    let acc = accepted.clone();
    tokio::spawn(async move {
        while let Ok((s, _)) = listener.accept().await {
            acc.fetch_add(1, Ordering::SeqCst);
            drop(s); // reset without answering
        }
    });
    let client = Client::with_token_provider(test_token_provider(), format!("http://{addr}"));
    assert!(client.post("/x", Some(&json!({}))).await.is_err());
    assert_eq!(accepted.load(Ordering::SeqCst), 1);
}

// Go: TestClient_RespectsMaxResponseSize
#[tokio::test]
async fn respects_max_response_size() {
    let big = Arc::new("a".repeat(MAX_RESPONSE_SIZE + 2048));
    let mock = MockApi::start(move |_| MockResponse::json(200, big.as_str())).await;
    // A POST is not retried, keeping the test fast.
    let err = mock
        .client()
        .post("/x", Some(&json!({})))
        .await
        .expect_err("expected size limit error");
    assert_eq!(
        err.to_string(),
        format!("response exceeds max size of {MAX_RESPONSE_SIZE} bytes")
    );
}

// Go: TestClient_EscapesPathIDs
#[tokio::test]
async fn escapes_path_ids() {
    let mock =
        MockApi::start(|_| MockResponse::json(200, r#"{"data":{"type":"apps","id":"x"}}"#)).await;
    mock.client()
        .get_app("../../etc/passwd?leak")
        .await
        .expect("get");
    assert_eq!(
        mock.requests()[0].path,
        "/v1/apps/..%2F..%2Fetc%2Fpasswd%3Fleak"
    );
}

// Go: TestClient_GetURL_FollowsSameHost
#[tokio::test]
async fn get_url_follows_same_host() {
    let mock = MockApi::start(|_| MockResponse::json(200, r#"{"ok":true}"#)).await;
    let full = format!("{}/v1/apps?cursor=ABCXYZ&limit=200", mock.base_url);
    mock.client().get_url(&full).await.expect("get_url");
    let reqs = mock.requests();
    assert_eq!(reqs.len(), 1);
    assert_eq!(
        format!("{}?{}", reqs[0].path, reqs[0].query),
        "/v1/apps?cursor=ABCXYZ&limit=200"
    );
}

// Go: TestClient_GetURL_RejectsCrossHost
#[tokio::test]
async fn get_url_rejects_cross_host() {
    let mock = MockApi::start(|_| panic!("server should not be hit for cross-host cursor")).await;
    let client = mock.client();
    let err = client
        .get_url("https://evil.example.com/v1/apps?cursor=X")
        .await
        .expect_err("cross-host URL");
    assert!(
        err.to_string().starts_with("cursor URL host mismatch"),
        "{err}"
    );
    // Same host, different scheme is rejected as well.
    let https_same_host = mock.base_url.replacen("http://", "https://", 1) + "/v1/apps";
    assert!(client.get_url(&https_same_host).await.is_err());
    assert_eq!(
        client.get_url("").await.unwrap_err().to_string(),
        "empty URL"
    );
    assert!(mock.requests().is_empty());
}

fn op(url: String, offset: i64, length: i64, header: Option<&str>) -> UploadOperation {
    let mut o = UploadOperation {
        method: "PUT".into(),
        url,
        offset,
        length,
        ..Default::default()
    };
    if let Some(v) = header {
        o.request_headers = vec![RequestHeader {
            name: "X-Test".into(),
            value: v.into(),
        }]
        .into();
    }
    o
}

// Go: TestPerformUploadOperations_HappyPath
#[tokio::test]
async fn perform_upload_operations_happy_path() {
    let storage = MockApi::start(|_| MockResponse::empty(200)).await;
    let body = b"AAAABBBBCCCC";
    let ops = vec![
        op(format!("{}/0", storage.base_url), 0, 4, Some("first")),
        op(format!("{}/1", storage.base_url), 4, 4, Some("second")),
        op(format!("{}/2", storage.base_url), 8, 4, Some("third")),
    ];
    storage
        .client()
        .perform_upload_operations(&ops, body)
        .await
        .expect("upload");
    let reqs = storage.requests();
    assert_eq!(reqs.len(), 3, "received {} chunks, want 3", reqs.len());
    for (i, want) in ["AAAA", "BBBB", "CCCC"].iter().enumerate() {
        assert_eq!(reqs[i].method, "PUT");
        assert_eq!(reqs[i].path, format!("/{i}"));
        assert_eq!(String::from_utf8_lossy(&reqs[i].body), *want, "chunk {i}");
    }
    assert_eq!(reqs[0].header("x-test"), Some("first"));
    assert_eq!(reqs[2].header("x-test"), Some("third"));
    // Pre-signed URLs never get the App Store Connect bearer token.
    assert!(reqs.iter().all(|r| r.header("authorization").is_none()));
}

// Go: TestPerformUploadOperations_BoundsCheck
#[tokio::test]
async fn perform_upload_operations_bounds_check() {
    let client = Client::with_token_provider(test_token_provider(), "http://localhost");
    let body = b"ABCD";
    for (name, offset, length) in [
        ("negative offset", -1, 2),
        ("zero length", 0, 0),
        ("past end", 2, 10),
    ] {
        let err = client
            .perform_upload_operations(&[op("http://example".into(), offset, length, None)], body)
            .await
            .expect_err(name);
        assert!(
            err.to_string().contains("out of bounds"),
            "{name}: error {err} does not mention bounds"
        );
    }
    assert_eq!(
        client
            .perform_upload_operations(&[], body)
            .await
            .unwrap_err()
            .to_string(),
        "no upload operations to perform"
    );
}

// Go: TestPerformUploadOperations_ServerError
#[tokio::test]
async fn perform_upload_operations_server_error() {
    let srv = MockApi::start(|_| MockResponse {
        status: 500,
        content_type: String::new(),
        body: b"nope".to_vec(),
        headers: vec![],
    })
    .await;
    let err = srv
        .client()
        .perform_upload_operations(&[op(srv.base_url.clone(), 0, 4, None)], b"AAAA")
        .await
        .expect_err("expected error");
    assert_eq!(err.to_string(), "upload op 0 failed with 500: nope");
}

/// An empty method defaults to PUT.
#[tokio::test]
async fn perform_upload_operations_default_method() {
    let storage = MockApi::start(|_| MockResponse::empty(200)).await;
    let mut o = op(storage.base_url.clone(), 0, 2, None);
    o.method = String::new();
    storage
        .client()
        .perform_upload_operations(&[o], b"AB")
        .await
        .unwrap();
    assert_eq!(storage.requests()[0].method, "PUT");
}

// Go: TestSha256Hex_KnownValue
#[test]
fn sha256_hex_known_value() {
    // SHA-256 of "abc" per FIPS 180-4 test vectors.
    assert_eq!(
        sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}
