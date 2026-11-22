//! Streamable HTTP transport tests, ported from
//! internal/asc/server/http_test.go, plus the 2026-07-28 behaviours that
//! intentionally differ from Go.

mod common;

use std::sync::Arc;

use asc_mcp::api::testutil::{MockApi, MockResponse};
use asc_mcp::mcp;
use asc_mcp::server::http::{
    HttpOptions, HttpServer, MAX_HTTP_BODY_SIZE, METHOD_HEADER, NAME_HEADER,
    PROTOCOL_VERSION_HEADER, SESSION_HEADER, decode_header_value, status_for_modern_error,
};
use asc_mcp::tools::Registry;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use base64::Engine;
use common::*;
use serde_json::{Value, json};
use tower::ServiceExt;

const LATEST: &str = mcp::LATEST_PROTOCOL_VERSION;

struct Resp {
    status: StatusCode,
    headers: axum::http::HeaderMap,
    body: Vec<u8>,
}

impl Resp {
    fn header(&self, name: &str) -> &str {
        self.headers
            .get(name)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
    }
    fn json(&self) -> Value {
        serde_json::from_slice(&self.body).unwrap_or_else(|e| {
            panic!(
                "decode: {e} (body={:?})",
                String::from_utf8_lossy(&self.body)
            )
        })
    }
    fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }
}

fn server() -> HttpServer {
    server_with(HttpOptions::default())
}

fn server_with(opts: HttpOptions) -> HttpServer {
    HttpServer::new(Arc::new(Registry::new(None)), opts)
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

/// Go `postJSON`.
async fn post(srv: &HttpServer, body: impl Into<Body>, headers: &[(&str, &str)]) -> Resp {
    let mut b = Request::builder()
        .method(Method::POST)
        .uri("/mcp")
        .header("Content-Type", "application/json");
    for (k, v) in headers {
        b = b.header(*k, *v);
    }
    send(srv, b.body(body.into()).unwrap()).await
}

async fn post_json(srv: &HttpServer, body: Value, headers: &[(&str, &str)]) -> Resp {
    post(srv, serde_json::to_vec(&body).unwrap(), headers).await
}

async fn get(srv: &HttpServer, path: &str) -> Resp {
    send(
        srv,
        Request::builder().uri(path).body(Body::empty()).unwrap(),
    )
    .await
}

fn init_req() -> Value {
    json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":init_params("2025-06-18")})
}

async fn new_session(srv: &HttpServer) -> String {
    let r = post_json(srv, init_req(), &[]).await;
    let sess = r.header(SESSION_HEADER).to_string();
    assert!(!sess.is_empty(), "no session id");
    sess
}

fn modern_req(method: &str, extra: Value) -> Value {
    json!({"jsonrpc":"2.0","id":1,"method":method,"params":modern_params(LATEST, extra)})
}

fn err_code(v: &Value) -> i64 {
    v["error"]["code"]
        .as_i64()
        .unwrap_or_else(|| panic!("expected error, got {v}"))
}

// Go: TestHTTP_InitializeAssignsSessionID
#[tokio::test]
async fn initialize_assigns_session_id() {
    let srv = server();
    let r = post_json(&srv, init_req(), &[]).await;
    assert_eq!(r.status, StatusCode::OK);
    assert!(
        !r.header(SESSION_HEADER).is_empty(),
        "missing Mcp-Session-Id response header"
    );
    assert_eq!(r.header(PROTOCOL_VERSION_HEADER), mcp::PROTOCOL_VERSION);
    assert!(r.json().get("error").is_none());
    assert_eq!(srv.session_count(), 1);
}

// Go: TestHTTP_RequiresSessionAfterInit
#[tokio::test]
async fn requires_session_after_init() {
    let r = post_json(
        &server(),
        json!({"jsonrpc":"2.0","id":1,"method":"tools/list"}),
        &[],
    )
    .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
}

// Go: TestHTTP_UnknownSession
#[tokio::test]
async fn unknown_session() {
    let r = post_json(
        &server(),
        json!({"jsonrpc":"2.0","id":1,"method":"tools/list"}),
        &[(SESSION_HEADER, "bogus")],
    )
    .await;
    assert_eq!(r.status, StatusCode::NOT_FOUND);
}

// Go: TestHTTP_EndToEndToolsList
#[tokio::test]
async fn end_to_end_tools_list() {
    let srv = server();
    let sess = new_session(&srv).await;
    let note = post_json(
        &srv,
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        &[
            (SESSION_HEADER, &sess),
            (PROTOCOL_VERSION_HEADER, mcp::PROTOCOL_VERSION),
        ],
    )
    .await;
    assert_eq!(note.status, StatusCode::ACCEPTED);
    assert!(note.body.is_empty());
    let list = post_json(
        &srv,
        json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
        &[
            (SESSION_HEADER, &sess),
            (PROTOCOL_VERSION_HEADER, mcp::PROTOCOL_VERSION),
        ],
    )
    .await;
    let v = list.json();
    assert!(v.get("error").is_none(), "unexpected error: {v}");
    assert!(v["result"]["tools"].as_array().unwrap().len() >= 200);
}

/// A legacy request with an unsupported MCP-Protocol-Version header.
#[tokio::test]
async fn legacy_unsupported_protocol_version_header() {
    let srv = server();
    let sess = new_session(&srv).await;
    let r = post_json(
        &srv,
        json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
        &[
            (SESSION_HEADER, &sess),
            (PROTOCOL_VERSION_HEADER, "1999-01-01"),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
}

// Go: TestHTTP_PingWithoutInit
#[tokio::test]
async fn ping_without_init() {
    let srv = server();
    let sess = new_session(&srv).await;
    let v = post_json(
        &srv,
        json!({"jsonrpc":"2.0","id":2,"method":"ping"}),
        &[(SESSION_HEADER, &sess)],
    )
    .await
    .json();
    assert!(v.get("error").is_none(), "unexpected error: {v}");
    assert_eq!(v["result"], json!({}));
}

// Go: TestHTTP_DeleteSession
#[tokio::test]
async fn delete_session() {
    let srv = server();
    let sess = new_session(&srv).await;
    let del = send(
        &srv,
        Request::builder()
            .method(Method::DELETE)
            .uri("/mcp")
            .header(SESSION_HEADER, &sess)
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(del.status, StatusCode::NO_CONTENT);
    let follow = post_json(
        &srv,
        json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
        &[(SESSION_HEADER, &sess)],
    )
    .await;
    assert_eq!(follow.status, StatusCode::NOT_FOUND);
    assert_eq!(srv.session_count(), 0);
}

// Go: TestHTTP_GetReturns405
#[tokio::test]
async fn get_returns_405() {
    let r = get(&server(), "/mcp").await;
    assert_eq!(r.status, StatusCode::METHOD_NOT_ALLOWED);
}

// Go: TestHTTP_RejectsOversizedBody
#[tokio::test]
async fn rejects_oversized_body() {
    let big = "x".repeat(MAX_HTTP_BODY_SIZE + 100);
    let r = post(&server(), big, &[]).await;
    assert_eq!(r.status, StatusCode::PAYLOAD_TOO_LARGE);
}

// Go: TestHTTP_HealthEndpoint
#[tokio::test]
async fn health_endpoint() {
    let r = get(&server(), "/healthz").await;
    assert_eq!(r.status, StatusCode::OK);
}

// Go: TestHTTP_OriginAllowlist
#[tokio::test]
async fn origin_allowlist() {
    let srv = server_with(HttpOptions {
        allowed_origins: vec!["https://allowed.example".into()],
        ..Default::default()
    });
    let r = post(&srv, "{}", &[("Origin", "https://evil.example")]).await;
    assert_eq!(r.status, StatusCode::FORBIDDEN);
    let r = post_json(&srv, init_req(), &[("Origin", "https://allowed.example")]).await;
    assert_eq!(r.status, StatusCode::OK);
}

/// Intentional difference from Go, which allowed every Origin when the
/// allow-list was empty: a present Origin must be loopback (any port), the
/// server's own origin, or listed; a missing Origin is allowed.
#[tokio::test]
async fn origin_defaults_reject_foreign_origins() {
    let srv = server();
    let r = post_json(&srv, init_req(), &[("Origin", "https://evil.example")]).await;
    assert_eq!(r.status, StatusCode::FORBIDDEN);
    assert_eq!(r.text(), "origin not allowed\n");
    for origin in [
        "http://localhost",
        "http://localhost:3000",
        "https://127.0.0.1:8443",
        "http://[::1]:9000",
        "http://127.0.0.1",
    ] {
        let r = post_json(&srv, init_req(), &[("Origin", origin)]).await;
        assert_eq!(
            r.status,
            StatusCode::OK,
            "loopback origin {origin} rejected"
        );
    }
    // The server's own origin (matching Host) is allowed.
    let r = post_json(
        &srv,
        init_req(),
        &[
            ("Host", "mcp.example:8080"),
            ("Origin", "https://mcp.example:8080"),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::OK);
    let r = post_json(
        &srv,
        init_req(),
        &[
            ("Host", "mcp.example:8080"),
            ("Origin", "https://mcp.example:9090"),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::FORBIDDEN);
    // Look-alike hosts are not loopback.
    for origin in [
        "http://localhost.evil.example",
        "http://127.0.0.1.evil.example",
        "null",
    ] {
        let r = post_json(&srv, init_req(), &[("Origin", origin)]).await;
        assert_eq!(r.status, StatusCode::FORBIDDEN, "origin {origin} allowed");
    }
    let r = post_json(&srv, init_req(), &[]).await;
    assert_eq!(r.status, StatusCode::OK, "missing Origin must be allowed");
}

// Go: TestHTTP_Auth_MissingToken
#[tokio::test]
async fn auth_missing_token() {
    let srv = server_with(HttpOptions {
        auth_tokens: vec!["s3cret".into()],
        ..Default::default()
    });
    let r = post_json(&srv, init_req(), &[]).await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
    assert!(
        r.header("WWW-Authenticate").starts_with("Bearer"),
        "WWW-Authenticate = {:?}",
        r.header("WWW-Authenticate")
    );
}

// Go: TestHTTP_Auth_WrongToken
#[tokio::test]
async fn auth_wrong_token() {
    let srv = server_with(HttpOptions {
        auth_tokens: vec!["s3cret".into()],
        ..Default::default()
    });
    let r = post_json(&srv, init_req(), &[("Authorization", "Bearer nope")]).await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
}

// Go: TestHTTP_Auth_CorrectToken
#[tokio::test]
async fn auth_correct_token() {
    let srv = server_with(HttpOptions {
        auth_tokens: vec!["primary".into(), "rotating".into()],
        ..Default::default()
    });
    for tok in ["primary", "rotating"] {
        let r = post_json(
            &srv,
            init_req(),
            &[("Authorization", &format!("Bearer {tok}"))],
        )
        .await;
        assert_eq!(r.status, StatusCode::OK, "token {tok}");
    }
}

// Go: TestHTTP_Auth_DoesNotGateHealthz
#[tokio::test]
async fn auth_does_not_gate_healthz() {
    let srv = server_with(HttpOptions {
        auth_tokens: vec!["s3cret".into()],
        ..Default::default()
    });
    assert_eq!(get(&srv, "/healthz").await.status, StatusCode::OK);
}

// Go: TestHTTP_Modern_DiscoverWithoutProtocolVersionHeader
#[tokio::test]
async fn modern_discover_without_protocol_version_header() {
    let r = post_json(
        &server(),
        json!({"jsonrpc":"2.0","id":1,"method":"server/discover"}),
        &[(METHOD_HEADER, "server/discover")],
    )
    .await;
    assert_eq!(r.status, StatusCode::OK);
    let v = r.json();
    assert!(v.get("error").is_none(), "unexpected error: {v}");
    assert!(
        !v["result"]["supportedVersions"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}

/// Intentional difference from Go: MCP-Protocol-Version is validated on
/// a modern server/discover too, once the body names a version.
#[tokio::test]
async fn modern_discover_validates_protocol_version_header() {
    let srv = server();
    let r = post_json(
        &srv,
        modern_req("server/discover", json!({})),
        &[(METHOD_HEADER, "server/discover")],
    )
    .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    assert_eq!(err_code(&r.json()), mcp::ERR_HEADER_MISMATCH);
    let r = post_json(
        &srv,
        modern_req("server/discover", json!({})),
        &[
            (METHOD_HEADER, "server/discover"),
            (PROTOCOL_VERSION_HEADER, LATEST),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::OK);
    // An explicit unsupported version is -32022 (difference 1).
    let body = json!({"jsonrpc":"2.0","id":1,"method":"server/discover","params":modern_meta("2099-01-01", true)});
    let r = post_json(
        &srv,
        body,
        &[
            (METHOD_HEADER, "server/discover"),
            (PROTOCOL_VERSION_HEADER, "2099-01-01"),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    assert_eq!(err_code(&r.json()), mcp::ERR_UNSUPPORTED_PROTOCOL_VERSION);
}

// Go: TestHTTP_Modern_ToolsListSessionless
#[tokio::test]
async fn modern_tools_list_sessionless() {
    let srv = server();
    let r = post_json(
        &srv,
        modern_req("tools/list", json!({})),
        &[
            (PROTOCOL_VERSION_HEADER, LATEST),
            (METHOD_HEADER, "tools/list"),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(
        r.header(SESSION_HEADER),
        "",
        "the server must not mint a session"
    );
    assert_eq!(r.header(PROTOCOL_VERSION_HEADER), LATEST);
    let v = r.json();
    assert!(v.get("error").is_none(), "unexpected error: {v}");
    assert_eq!(v["result"]["resultType"], mcp::RESULT_TYPE_COMPLETE);
    assert!(v["result"]["ttlMs"].is_i64(), "ttlMs missing");
    assert_eq!(srv.session_count(), 0);
}

// Go: TestHTTP_Modern_IgnoresSessionHeader
#[tokio::test]
async fn modern_ignores_session_header() {
    let r = post_json(
        &server(),
        modern_req("tools/list", json!({})),
        &[
            (PROTOCOL_VERSION_HEADER, LATEST),
            (METHOD_HEADER, "tools/list"),
            (SESSION_HEADER, "bogus-session-id"),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::OK, "session header must be ignored");
    assert_eq!(r.header(SESSION_HEADER), "", "never echoed");
}

// Go: TestHTTP_Modern_MethodHeaderMismatch
#[tokio::test]
async fn modern_method_header_mismatch() {
    let r = post_json(
        &server(),
        modern_req("tools/list", json!({})),
        &[
            (PROTOCOL_VERSION_HEADER, LATEST),
            (METHOD_HEADER, "tools/call"),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    assert_eq!(err_code(&r.json()), mcp::ERR_HEADER_MISMATCH);
}

// Go: TestHTTP_Modern_MissingMethodHeader
#[tokio::test]
async fn modern_missing_method_header() {
    let r = post_json(
        &server(),
        modern_req("tools/list", json!({})),
        &[(PROTOCOL_VERSION_HEADER, LATEST)],
    )
    .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    assert_eq!(r.header(PROTOCOL_VERSION_HEADER), LATEST);
    assert_eq!(err_code(&r.json()), mcp::ERR_HEADER_MISMATCH);
}

/// Intentional difference from Go: Mcp-Method and MCP-Protocol-Version
/// are compared byte-exactly; only Mcp-Name honours the base64 sentinel.
#[tokio::test]
async fn modern_method_and_version_headers_are_not_decoded() {
    let enc = |s: &str| {
        format!(
            "=?base64?{}?=",
            base64::engine::general_purpose::STANDARD.encode(s)
        )
    };
    let srv = server();
    let r = post_json(
        &srv,
        modern_req("tools/list", json!({})),
        &[
            (PROTOCOL_VERSION_HEADER, LATEST),
            (METHOD_HEADER, &enc("tools/list")),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    assert_eq!(err_code(&r.json()), mcp::ERR_HEADER_MISMATCH);
    let r = post_json(
        &srv,
        modern_req("tools/list", json!({})),
        &[
            (PROTOCOL_VERSION_HEADER, &enc(LATEST)),
            (METHOD_HEADER, "tools/list"),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    assert_eq!(err_code(&r.json()), mcp::ERR_HEADER_MISMATCH);
}

/// Intentional difference from Go: a modern MCP-Protocol-Version header
/// routes the request to the stateless path even without body `_meta`; it
/// then fails as invalid params, never with the legacy session errors.
#[tokio::test]
async fn modern_header_without_body_meta() {
    let srv = server();
    let r = post_json(
        &srv,
        json!({"jsonrpc":"2.0","id":1,"method":"tools/list"}),
        &[
            (PROTOCOL_VERSION_HEADER, LATEST),
            (METHOD_HEADER, "tools/list"),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    assert_eq!(r.header("Content-Type"), "application/json");
    assert_eq!(err_code(&r.json()), mcp::ERR_INVALID_PARAMS);
    // initialize stays legacy even with the modern header.
    let r = post_json(&srv, init_req(), &[(PROTOCOL_VERSION_HEADER, LATEST)]).await;
    assert_eq!(r.status, StatusCode::OK);
    assert!(
        !r.header(SESSION_HEADER).is_empty(),
        "initialize must mint a legacy session"
    );
    assert_eq!(r.json()["result"]["protocolVersion"], "2025-06-18");
}

/// A server whose list_apps call fails at the API (401), so tools/call is
/// dispatched end to end without real credentials.
async fn server_with_failing_api() -> (HttpServer, MockApi) {
    let mock = MockApi::start(|_| {
        MockResponse::json(
            401,
            r#"{"errors":[{"title":"Unauthorized","detail":"bad token"}]}"#,
        )
    })
    .await;
    let registry = Registry::new(Some(Arc::new(mock.client())));
    (
        HttpServer::new(Arc::new(registry), HttpOptions::default()),
        mock,
    )
}

// Go: TestHTTP_Modern_ToolsCallMissingNameHeader
#[tokio::test]
async fn modern_tools_call_missing_name_header() {
    let r = post_json(
        &server(),
        modern_req("tools/call", json!({"name":"list_apps"})),
        &[
            (PROTOCOL_VERSION_HEADER, LATEST),
            (METHOD_HEADER, "tools/call"),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    assert_eq!(err_code(&r.json()), mcp::ERR_HEADER_MISMATCH);
}

// Go: TestHTTP_Modern_ToolsCallNameHeaderMatch
#[tokio::test]
async fn modern_tools_call_name_header_match() {
    let (srv, mock) = server_with_failing_api().await;
    let r = post_json(
        &srv,
        modern_req("tools/call", json!({"name":"list_apps"})),
        &[
            (PROTOCOL_VERSION_HEADER, LATEST),
            (METHOD_HEADER, "tools/call"),
            (NAME_HEADER, "list_apps"),
        ],
    )
    .await;
    // The API failure surfaces as an isError tool RESULT, which stays 200.
    assert_eq!(r.status, StatusCode::OK);
    let v = r.json();
    assert!(v.get("error").is_none(), "unexpected protocol error: {v}");
    assert_eq!(v["result"]["isError"], true);
    assert_eq!(v["result"]["resultType"], mcp::RESULT_TYPE_COMPLETE);
    assert_eq!(mock.requests().len(), 1, "the call must reach the API");
}

// Go: TestHTTP_Modern_NameHeaderBase64
#[tokio::test]
async fn modern_name_header_base64() {
    let (srv, _mock) = server_with_failing_api().await;
    let encoded = format!(
        "=?base64?{}?=",
        base64::engine::general_purpose::STANDARD.encode("list_apps")
    );
    let r = post_json(
        &srv,
        modern_req("tools/call", json!({"name":"list_apps"})),
        &[
            (PROTOCOL_VERSION_HEADER, LATEST),
            (METHOD_HEADER, "tools/call"),
            (NAME_HEADER, &encoded),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::OK, "sentinel-encoded name accepted");
}

// Go: TestHTTP_Modern_ProtocolVersionHeaderMismatch
#[tokio::test]
async fn modern_protocol_version_header_mismatch() {
    let r = post_json(
        &server(),
        modern_req("tools/list", json!({})),
        &[
            (PROTOCOL_VERSION_HEADER, "2025-06-18"),
            (METHOD_HEADER, "tools/list"),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    assert_eq!(err_code(&r.json()), mcp::ERR_HEADER_MISMATCH);
}

// Go: TestHTTP_Modern_UnsupportedProtocolVersion
#[tokio::test]
async fn modern_unsupported_protocol_version() {
    let body = json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":modern_meta("2024-11-05", true)});
    let r = post_json(
        &server(),
        body,
        &[
            (PROTOCOL_VERSION_HEADER, "2024-11-05"),
            (METHOD_HEADER, "tools/list"),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    let v = r.json();
    assert_eq!(err_code(&v), mcp::ERR_UNSUPPORTED_PROTOCOL_VERSION);
    assert!(
        !v["error"]["data"]["supported"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}

// Go: TestHTTP_Modern_UnknownMethod
#[tokio::test]
async fn modern_unknown_method() {
    let r = post_json(
        &server(),
        modern_req("ping", json!({})),
        &[(PROTOCOL_VERSION_HEADER, LATEST), (METHOD_HEADER, "ping")],
    )
    .await;
    assert_eq!(r.status, StatusCode::NOT_FOUND);
    assert_eq!(err_code(&r.json()), mcp::ERR_METHOD_NOT_FOUND);
}

/// Modern -32600 maps to HTTP 400.
#[tokio::test]
async fn modern_invalid_request_is_400() {
    let mut body = modern_req("tools/list", json!({}));
    body["jsonrpc"] = json!("1.0");
    let r = post_json(
        &server(),
        body,
        &[
            (PROTOCOL_VERSION_HEADER, LATEST),
            (METHOD_HEADER, "tools/list"),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    assert_eq!(err_code(&r.json()), mcp::ERR_INVALID_REQUEST);
}

/// A body that is not JSON is a -32700 with HTTP 400.
#[tokio::test]
async fn parse_error_is_400() {
    let r = post(
        &server(),
        "not json",
        &[
            (PROTOCOL_VERSION_HEADER, LATEST),
            (METHOD_HEADER, "tools/list"),
        ],
    )
    .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    assert_eq!(err_code(&r.json()), mcp::ERR_PARSE);
}

/// subscriptions/listen over HTTP is an SSE stream: the acknowledgement
/// notification, then the completion result.
#[tokio::test]
async fn modern_subscriptions_listen_is_sse() {
    let r = post_json(
        &server(),
        json!({"jsonrpc":"2.0","id":5,"method":"subscriptions/listen","params":modern_params(LATEST, json!({"notifications":{}}))}),
        &[(PROTOCOL_VERSION_HEADER, LATEST), (METHOD_HEADER, "subscriptions/listen")],
    )
    .await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(r.header("Content-Type"), "text/event-stream");
    assert_eq!(r.header(PROTOCOL_VERSION_HEADER), LATEST);
    let events: Vec<Value> = r
        .text()
        .split("\n\n")
        .filter(|e| !e.is_empty())
        .map(|e| {
            let data = e
                .lines()
                .find_map(|l| l.strip_prefix("data: "))
                .expect("data line");
            assert!(e.starts_with("event: message\n"), "event {e:?}");
            serde_json::from_str(data).unwrap()
        })
        .collect();
    assert_eq!(events.len(), 2, "{}", r.text());
    assert_eq!(
        events[0]["method"],
        "notifications/subscriptions/acknowledged"
    );
    assert_eq!(events[0]["params"]["notifications"], json!({}));
    assert_eq!(events[0]["params"]["_meta"][mcp::META_SUBSCRIPTION_ID], 5);
    assert_eq!(events[1]["id"], 5);
    assert_eq!(events[1]["result"]["resultType"], mcp::RESULT_TYPE_COMPLETE);
    assert_eq!(events[1]["result"]["_meta"][mcp::META_SUBSCRIPTION_ID], 5);
}

// Go: TestDecodeHeaderValue
#[test]
fn decode_header_value_cases() {
    let utf8 = format!(
        "=?base64?{}?=",
        base64::engine::general_purpose::STANDARD.encode("outil-été")
    );
    let cases: &[(&str, &str, Option<&str>)] = &[
        ("plain", "tools/list", Some("tools/list")),
        ("empty", "", Some("")),
        ("sentinel", "=?base64?bGlzdF9hcHBz?=", Some("list_apps")),
        ("sentinel utf8", &utf8, Some("outil-été")),
        ("malformed base64", "=?base64?!!!?=", None),
        (
            "prefix only is verbatim",
            "=?base64?abc",
            Some("=?base64?abc"),
        ),
    ];
    for (name, input, want) in cases {
        let got = decode_header_value(input);
        match want {
            Some(w) => assert_eq!(got.as_deref(), Ok(*w), "{name}"),
            None => assert!(got.is_err(), "{name}: got {got:?}, want error"),
        }
    }
}

// Go: TestStatusForModernError (plus -32600/-32700, which Rust maps to 400)
#[test]
fn status_for_modern_error_cases() {
    for (code, want) in [
        (mcp::ERR_HEADER_MISMATCH, StatusCode::BAD_REQUEST),
        (mcp::ERR_MISSING_CLIENT_CAPABILITY, StatusCode::BAD_REQUEST),
        (
            mcp::ERR_UNSUPPORTED_PROTOCOL_VERSION,
            StatusCode::BAD_REQUEST,
        ),
        (mcp::ERR_INVALID_PARAMS, StatusCode::BAD_REQUEST),
        (mcp::ERR_INVALID_REQUEST, StatusCode::BAD_REQUEST),
        (mcp::ERR_PARSE, StatusCode::BAD_REQUEST),
        (mcp::ERR_METHOD_NOT_FOUND, StatusCode::NOT_FOUND),
        (mcp::ERR_INTERNAL, StatusCode::OK),
    ] {
        assert_eq!(status_for_modern_error(code), want, "code {code}");
    }
}

// Go: TestHTTP_MetricsEndpoint. Intentional difference: no go_* runtime
// metrics (there is no Go runtime); asc_mcp_* and process_* remain.
#[tokio::test]
async fn metrics_endpoint() {
    let srv = server();
    post_json(&srv, init_req(), &[]).await;
    let r = get(&srv, "/metrics").await;
    assert_eq!(r.status, StatusCode::OK);
    let body = r.text();
    for want in [
        "asc_mcp_http_requests_total",
        "asc_mcp_http_request_duration_seconds",
        "asc_mcp_session_active",
    ] {
        assert!(body.contains(want), "metrics output missing {want}");
    }
    assert!(
        !body
            .lines()
            .any(|l| l.starts_with("go_") || l.starts_with("# HELP go_")),
        "unexpected go_* metric"
    );
    #[cfg(target_os = "linux")]
    assert!(body.contains("process_"), "process metrics missing");
}

// Go: TestHTTP_MetricsRecordRequests
#[tokio::test]
async fn metrics_record_requests() {
    let srv = server();
    let sess = new_session(&srv).await;
    post_json(
        &srv,
        json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
        &[(SESSION_HEADER, &sess)],
    )
    .await;
    let m = srv.metrics();
    assert!(m.http_requests.with_label_values(&["POST", "200"]).get() >= 2.0);
    assert_eq!(m.sessions_active.get(), 1.0);
    assert_eq!(m.sessions_created.get(), 1.0);
}

// Go: TestHTTP_MetricsAuthFailure
#[tokio::test]
async fn metrics_auth_failure() {
    let srv = server_with(HttpOptions {
        auth_tokens: vec!["secret".into()],
        ..Default::default()
    });
    post_json(
        &srv,
        json!({"jsonrpc":"2.0","id":1,"method":"initialize"}),
        &[],
    )
    .await;
    assert_eq!(srv.metrics().auth_failures.get(), 1.0);
}

/// Idle legacy sessions are reaped and counted.
#[tokio::test]
async fn reap_idle_sessions() {
    let srv = server();
    new_session(&srv).await;
    assert_eq!(srv.reap_idle(std::time::Duration::from_secs(3600)), 0);
    assert_eq!(srv.reap_idle(std::time::Duration::ZERO), 1);
    assert_eq!(srv.session_count(), 0);
    assert_eq!(srv.metrics().sessions_reaped.get(), 1.0);
    assert_eq!(srv.metrics().sessions_active.get(), 0.0);
}
