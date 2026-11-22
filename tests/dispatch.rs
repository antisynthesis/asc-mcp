//! Dispatcher tests, ported from internal/asc/server/server_test.go (the
//! `handleRequest`-level cases) plus the 2026-07-28 behaviours the Rust
//! dispatcher adds.

mod common;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use asc_mcp::mcp::{self, Response};
use asc_mcp::server::dispatch::{self, Dispatcher, Session};
use asc_mcp::tools::Registry;
use common::*;
use serde_json::{Value, json};

fn full_dispatcher() -> Dispatcher {
    Dispatcher::new(Arc::new(Registry::new(None)))
}

fn err_code(v: &Value) -> i64 {
    v["error"]["code"]
        .as_i64()
        .unwrap_or_else(|| panic!("expected an error, got {v}"))
}

// Go: TestServer_HandleInitialize
#[tokio::test]
async fn handle_initialize() {
    let d = full_dispatcher();
    let sess = Session::new();
    let v = dispatch(
        &d,
        &sess,
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{
            "protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}),
    )
    .await;
    assert!(v.get("error").is_none(), "unexpected error: {v}");
    assert!(
        sess.is_initialized(),
        "session should be initialized after initialize"
    );
    // 2024-11-05 is supported, so it is echoed back.
    assert_eq!(v["result"]["protocolVersion"], "2024-11-05");
    assert_eq!(v["result"]["serverInfo"]["name"], dispatch::SERVER_NAME);
    assert!(
        !v["result"]["instructions"]
            .as_str()
            .unwrap_or("")
            .is_empty(),
        "instructions must be set"
    );
}

// Go: TestServer_HandleInitialize_NegotiatesLatest
#[tokio::test]
async fn handle_initialize_negotiates_latest() {
    let d = full_dispatcher();
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":init_params("2099-01-01")}),
    )
    .await;
    assert_eq!(v["result"]["protocolVersion"], mcp::PROTOCOL_VERSION);
}

// Go: TestServer_HandleInitialize_ModernVersionFallsBack
#[tokio::test]
async fn handle_initialize_modern_version_falls_back() {
    let d = full_dispatcher();
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":init_params("2026-07-28")}),
    )
    .await;
    // 2026-07-28 has no handshake: negotiate down to the newest legacy one.
    assert_eq!(v["result"]["protocolVersion"], "2025-11-25");
}

/// initialize is always legacy, even when its params carry a modern
/// `_meta` protocol version.
#[tokio::test]
async fn initialize_with_modern_meta_is_still_legacy() {
    let d = full_dispatcher();
    let sess = Session::new();
    let mut params = init_params("2025-06-18");
    params["_meta"] = modern_meta("2026-07-28", true)["_meta"].clone();
    let v = dispatch(
        &d,
        &sess,
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":params}),
    )
    .await;
    assert_eq!(v["result"]["protocolVersion"], "2025-06-18");
    assert!(v["result"].get("resultType").is_none());
    assert!(sess.is_initialized());
}

// Go: TestServer_HandlePing
#[tokio::test]
async fn handle_ping() {
    let d = full_dispatcher();
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":42,"method":"ping"}),
    )
    .await;
    assert!(v.get("error").is_none(), "unexpected error: {v}");
    assert_eq!(v["result"], json!({}));
    assert_eq!(v["id"], 42);
}

// Go: TestServer_HandleToolsList
#[tokio::test]
async fn handle_tools_list() {
    let d = full_dispatcher();
    let sess = Session::new();
    initialize(&d, &sess).await;
    let v = dispatch(
        &d,
        &sess,
        json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
    )
    .await;
    assert!(v.get("error").is_none(), "unexpected error: {v}");
    let tools = v["result"]["tools"].as_array().expect("tools array");
    // 221 base + 3 upload + 6 build upload + 8 beta feedback + 10
    // background asset + 49 Game Center content + 59 commerce tools.
    assert_eq!(tools.len(), 356);
}

// Go: TestServer_HandleToolsList_NotInitialized
#[tokio::test]
async fn handle_tools_list_not_initialized() {
    let d = full_dispatcher();
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"tools/list"}),
    )
    .await;
    assert_eq!(err_code(&v), mcp::ERR_INVALID_REQUEST);
}

// Go: TestServer_HandleToolsCall_NotInitialized
#[tokio::test]
async fn handle_tools_call_not_initialized() {
    let d = full_dispatcher();
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"list_apps"}}),
    )
    .await;
    assert_eq!(err_code(&v), mcp::ERR_INVALID_REQUEST);
}

// Go: TestServer_HandleRequest_InvalidJSONRPC
#[tokio::test]
async fn handle_request_invalid_jsonrpc() {
    let d = full_dispatcher();
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"1.0","id":1,"method":"initialize"}),
    )
    .await;
    assert_eq!(err_code(&v), mcp::ERR_INVALID_REQUEST);
}

// Go: TestServer_HandleRequest_MethodNotFound
#[tokio::test]
async fn handle_request_method_not_found() {
    let d = full_dispatcher();
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"unknown/method"}),
    )
    .await;
    assert_eq!(err_code(&v), mcp::ERR_METHOD_NOT_FOUND);
}

// Go: TestServer_SendResult
#[test]
fn send_result_shape() {
    let r = Response::result(Some(json!(1)), json!({"status": "ok"}));
    let v: Value = serde_json::from_slice(&r.to_bytes()).unwrap();
    assert_eq!(v["jsonrpc"], mcp::JSONRPC_VERSION);
    assert!(v.get("error").is_none());
    assert_eq!(v["result"]["status"], "ok");
}

// Go: TestServer_SendError
#[test]
fn send_error_shape() {
    let r = Response::error(
        Some(json!(1)),
        mcp::ERR_INTERNAL,
        "Internal error",
        "details",
    );
    let v: Value = serde_json::from_slice(&r.to_bytes()).unwrap();
    assert_eq!(v["error"]["code"], mcp::ERR_INTERNAL);
    assert_eq!(v["error"]["message"], "Internal error");
    assert_eq!(v["error"]["data"], "details");
    assert!(v.get("result").is_none());
}

// Go: TestServer_NotificationsInitialized
#[tokio::test]
async fn notifications_initialized_gets_no_response() {
    let d = full_dispatcher();
    let out = d
        .dispatch(
            &req(json!({"jsonrpc":"2.0","method":"notifications/initialized"})),
            &Session::new(),
        )
        .await;
    assert!(out.is_none(), "expected no response for notification");
}

// Go: TestServer_ModernToolsList_NoHandshake
#[tokio::test]
async fn modern_tools_list_no_handshake() {
    let d = full_dispatcher();
    let sess = Session::new();
    let v = dispatch(
        &d,
        &sess,
        json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":modern_meta("2026-07-28", true)}),
    )
    .await;
    assert!(v.get("error").is_none(), "unexpected error: {v}");
    let r = &v["result"];
    assert!(!r["tools"].as_array().unwrap().is_empty());
    assert_eq!(r["resultType"], mcp::RESULT_TYPE_COMPLETE);
    assert!(
        r["ttlMs"].is_i64(),
        "ttlMs must be set on modern tools/list"
    );
    assert_eq!(r["cacheScope"], mcp::CACHE_SCOPE_PUBLIC);
    assert_eq!(
        r["_meta"][mcp::META_SERVER_INFO]["name"],
        dispatch::SERVER_NAME
    );
    assert!(
        !sess.is_initialized(),
        "modern requests must not touch the handshake state"
    );
}

// Go: TestServer_Modern_UnsupportedProtocolVersion
#[tokio::test]
async fn modern_unsupported_protocol_version() {
    let d = full_dispatcher();
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":modern_meta("2025-06-18", true)}),
    )
    .await;
    assert_eq!(err_code(&v), mcp::ERR_UNSUPPORTED_PROTOCOL_VERSION);
    assert_eq!(v["error"]["data"]["requested"], "2025-06-18");
    assert_eq!(
        v["error"]["data"]["supported"],
        json!(mcp::SUPPORTED_PROTOCOL_VERSIONS)
    );
    // Intentional difference from Go: a legacy version sent via _meta gets
    // a message that says how to use it (the handshake).
    assert_eq!(
        v["error"]["message"],
        "Unsupported protocol version: 2025-06-18 must be negotiated with the initialize handshake"
    );
}

#[tokio::test]
async fn modern_unknown_protocol_version_message() {
    let d = full_dispatcher();
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":modern_meta("1999-01-01", true)}),
    )
    .await;
    assert_eq!(err_code(&v), mcp::ERR_UNSUPPORTED_PROTOCOL_VERSION);
    assert_eq!(v["error"]["message"], "Unsupported protocol version");
    assert_eq!(v["error"]["data"]["requested"], "1999-01-01");
}

// Go: TestServer_Modern_MissingClientCapabilities
#[tokio::test]
async fn modern_missing_client_capabilities() {
    let d = full_dispatcher();
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":modern_meta("2026-07-28", false)}),
    )
    .await;
    assert_eq!(err_code(&v), mcp::ERR_INVALID_PARAMS);
}

// Go: TestServer_Modern_PingRemoved
#[tokio::test]
async fn modern_ping_removed() {
    let d = full_dispatcher();
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"ping","params":modern_meta("2026-07-28", true)}),
    )
    .await;
    assert_eq!(err_code(&v), mcp::ERR_METHOD_NOT_FOUND);
}

// Go: TestServer_LegacyToolsList_NoModernKeys
#[tokio::test]
async fn legacy_tools_list_no_modern_keys() {
    let d = full_dispatcher();
    let sess = Session::new();
    initialize(&d, &sess).await;
    let out = d
        .dispatch(
            &req(json!({"jsonrpc":"2.0","id":1,"method":"tools/list"})),
            &sess,
        )
        .await
        .unwrap();
    let raw = String::from_utf8(out.response.to_bytes()).unwrap();
    for key in [r#""resultType""#, r#""ttlMs""#, r#""cacheScope""#] {
        assert!(
            !raw.contains(key),
            "legacy tools/list response contains modern key {key}"
        );
    }
}

/// Intentional difference from Go (which ignored `_meta` on discover): an
/// explicit unsupported version on server/discover gets -32022, while a
/// discover without any version is still answered.
#[tokio::test]
async fn discover_validates_explicit_protocol_version() {
    let d = full_dispatcher();
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"server/discover","params":modern_meta("2099-01-01", true)}),
    )
    .await;
    assert_eq!(err_code(&v), mcp::ERR_UNSUPPORTED_PROTOCOL_VERSION);

    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":2,"method":"server/discover"}),
    )
    .await;
    assert!(v.get("error").is_none(), "unexpected error: {v}");
    assert_eq!(
        v["result"]["supportedVersions"],
        json!(mcp::SUPPORTED_PROTOCOL_VERSIONS)
    );

    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":3,"method":"server/discover","params":modern_meta("2026-07-28", true)}),
    )
    .await;
    assert!(v.get("error").is_none(), "unexpected error: {v}");
}

/// Modern tools/list: this server never issues cursors (2026-07-28).
#[tokio::test]
async fn modern_tools_list_rejects_cursor() {
    let d = full_dispatcher();
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":modern_params("2026-07-28", json!({"cursor":"abc"}))}),
    )
    .await;
    assert_eq!(err_code(&v), mcp::ERR_INVALID_PARAMS);
}

/// Modern tools/call requires a string name and object arguments.
#[tokio::test]
async fn modern_tools_call_param_types() {
    let mut r = empty_registry();
    register_text_tool(&mut r, "echo_tool", "ok");
    let d = Dispatcher::new(Arc::new(r));
    for extra in [
        json!({"name": 5}),
        json!({"name": "echo_tool", "arguments": [1, 2]}),
        json!({"name": "echo_tool", "arguments": "x"}),
    ] {
        let v = dispatch(
            &d,
            &Session::new(),
            json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":modern_params("2026-07-28", extra.clone())}),
        )
        .await;
        assert_eq!(err_code(&v), mcp::ERR_INVALID_PARAMS, "params {extra}");
    }
    // Well-formed: the call runs and gets the modern envelope.
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":modern_params("2026-07-28", json!({"name":"echo_tool","arguments":{}}))}),
    )
    .await;
    assert!(v.get("error").is_none(), "unexpected error: {v}");
    assert_eq!(v["result"]["content"][0]["text"], "ok");
    assert_eq!(v["result"]["resultType"], mcp::RESULT_TYPE_COMPLETE);
    assert_eq!(
        v["result"]["_meta"][mcp::META_SERVER_INFO]["name"],
        dispatch::SERVER_NAME
    );
}

/// Unknown tools: -32602 on the modern path, the historical -32601 on the
/// legacy path.
#[tokio::test]
async fn unknown_tool_codes_per_era() {
    let d = Dispatcher::new(Arc::new(empty_registry()));
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":modern_params("2026-07-28", json!({"name":"nope"}))}),
    )
    .await;
    assert_eq!(err_code(&v), mcp::ERR_INVALID_PARAMS);
    let sess = Session::new();
    initialize(&d, &sess).await;
    let v = dispatch(
        &d,
        &sess,
        json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"nope"}}),
    )
    .await;
    assert_eq!(err_code(&v), mcp::ERR_METHOD_NOT_FOUND);
}

/// Malformed reserved `_meta` values are invalid params.
#[tokio::test]
async fn malformed_meta_is_invalid_params() {
    let d = full_dispatcher();
    for meta in [
        json!({ mcp::META_PROTOCOL_VERSION: "2026-07-28", mcp::META_CLIENT_CAPABILITIES: null }),
        json!({ mcp::META_PROTOCOL_VERSION: "2026-07-28", mcp::META_CLIENT_CAPABILITIES: [] }),
        json!({ mcp::META_PROTOCOL_VERSION: 42, mcp::META_CLIENT_CAPABILITIES: {} }),
    ] {
        let v = dispatch(
            &d,
            &Session::new(),
            json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":{"_meta": meta.clone()}}),
        )
        .await;
        assert_eq!(err_code(&v), mcp::ERR_INVALID_PARAMS, "_meta {meta}");
    }
}

/// subscriptions/listen: an acknowledgement with an empty filter, then a
/// completion result (this server never sends list-changed notifications).
#[tokio::test]
async fn subscriptions_listen_acknowledges_and_completes() {
    let d = full_dispatcher();
    let out = d
        .dispatch(
            &req(json!({"jsonrpc":"2.0","id":"sub-1","method":"subscriptions/listen","params":modern_params("2026-07-28", json!({"notifications":{"toolsListChanged":true}}))})),
            &Session::new(),
        )
        .await
        .expect("a response");
    assert_eq!(out.notifications.len(), 1);
    let ack = &out.notifications[0];
    assert_eq!(ack["jsonrpc"], "2.0");
    assert_eq!(ack["method"], "notifications/subscriptions/acknowledged");
    assert!(
        ack.get("id").is_none(),
        "the acknowledgement is a notification"
    );
    assert_eq!(ack["params"]["notifications"], json!({}));
    assert_eq!(ack["params"]["_meta"][mcp::META_SUBSCRIPTION_ID], "sub-1");
    let v = resp_json(&out);
    assert_eq!(v["id"], "sub-1");
    assert_eq!(v["result"]["resultType"], mcp::RESULT_TYPE_COMPLETE);
    assert_eq!(v["result"]["_meta"][mcp::META_SUBSCRIPTION_ID], "sub-1");
    assert_eq!(
        v["result"]["_meta"][mcp::META_SERVER_INFO]["name"],
        dispatch::SERVER_NAME
    );
}

/// A drop guard that records when the handler's future is torn down.
struct SetOnDrop(Arc<AtomicBool>);
impl Drop for SetOnDrop {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

// Go: TestDispatcher_ContextCancellationAbortsToolCall. The Go transport
// cancels a context; in Rust the transport drops the dispatch future
// (stdio cancellation, a closed HTTP connection). Dropping it must abort
// the tool handler instead of leaving it running in the background.
#[tokio::test]
async fn dropping_dispatch_aborts_tool_call() {
    let started = Arc::new(tokio::sync::Notify::new());
    let dropped = Arc::new(AtomicBool::new(false));
    let mut r = empty_registry();
    {
        let (started, dropped) = (started.clone(), dropped.clone());
        register_fake(&mut r, "hang_tool", move || {
            let (started, guard) = (started.clone(), SetOnDrop(dropped.clone()));
            async move {
                let _guard = guard;
                started.notify_one();
                std::future::pending::<()>().await;
                unreachable!()
            }
        });
    }
    let d = Dispatcher::new(Arc::new(r));
    let sess = Session::new();
    initialize(&d, &sess).await;
    let request =
        req(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"hang_tool"}}));
    let out = tokio::select! {
        out = d.dispatch(&request, &sess) => Some(out),
        _ = started.notified() => None,
    };
    assert!(out.is_none(), "hang_tool must not complete");
    // The dispatch future is gone; the handler task must be aborted.
    tokio::time::timeout(Duration::from_secs(5), async {
        while !dropped.load(Ordering::SeqCst) {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("tool handler kept running after its dispatch was dropped");
}

/// A failing handler is reported as an isError result, not a protocol
/// error; a panicking one likewise.
#[tokio::test]
async fn handler_errors_and_panics_are_tool_errors() {
    let mut r = empty_registry();
    register_fake(&mut r, "fail_tool", || async { Err("boom".to_string()) });
    register_fake(&mut r, "panic_tool", || async { panic!("handler bug") });
    let d = Dispatcher::new(Arc::new(r));
    let sess = Session::new();
    initialize(&d, &sess).await;
    for (name, text) in [
        ("fail_tool", "boom"),
        ("panic_tool", "internal error: tool handler panicked"),
    ] {
        let v = dispatch(
            &d,
            &sess,
            json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":name}}),
        )
        .await;
        assert!(v.get("error").is_none(), "unexpected protocol error: {v}");
        assert_eq!(v["result"]["isError"], true);
        assert_eq!(v["result"]["content"][0]["text"], text);
    }
}
