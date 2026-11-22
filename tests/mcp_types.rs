//! MCP type tests.
//!
//! Most result shapes are built in the dispatcher rather than as dedicated
//! types, so their wire encoding is pinned here on the dispatcher's output.

mod common;

use std::sync::{Arc, Mutex};

use asc_mcp::mcp::{self, CallToolResult, Request, RequestMeta, Response, Tool, text_content};
use asc_mcp::server::dispatch::{self, Dispatcher, Session, server_info_meta};
use asc_mcp::tools::{Handler, HandlerFuture, Registry};
use asc_mcp::wire::json::to_json_bytes;
use common::*;
use serde_json::{Value, json};

#[test]
fn constants() {
    assert_eq!(mcp::JSONRPC_VERSION, "2.0");
    assert_eq!(mcp::LATEST_PROTOCOL_VERSION, "2026-07-28");
    assert_eq!(mcp::PROTOCOL_VERSION, "2025-11-25");
    assert_eq!(
        mcp::SUPPORTED_PROTOCOL_VERSIONS[0],
        mcp::LATEST_PROTOCOL_VERSION
    );
    assert_eq!(
        mcp::SUPPORTED_PROTOCOL_VERSIONS,
        [
            "2026-07-28",
            "2025-11-25",
            "2025-06-18",
            "2025-03-26",
            "2024-11-05"
        ]
    );
    assert_eq!(
        mcp::LEGACY_HANDSHAKE_VERSIONS,
        ["2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05"]
    );
    assert_eq!(mcp::LEGACY_HANDSHAKE_VERSIONS[0], mcp::PROTOCOL_VERSION);
    assert_eq!(mcp::MODERN_PROTOCOL_VERSIONS, ["2026-07-28"]);
}

#[test]
fn negotiate_protocol_version() {
    for (name, requested, want) in [
        ("latest legacy", "2025-11-25", "2025-11-25"),
        ("prior legacy", "2025-06-18", "2025-06-18"),
        ("older supported", "2024-11-05", "2024-11-05"),
        ("interim supported", "2025-03-26", "2025-03-26"),
        // 2026-07-28 has no handshake: never echoed here.
        (
            "modern falls back to legacy",
            "2026-07-28",
            mcp::PROTOCOL_VERSION,
        ),
        (
            "unknown falls back to latest legacy",
            "1999-01-01",
            mcp::PROTOCOL_VERSION,
        ),
        (
            "empty falls back to latest legacy",
            "",
            mcp::PROTOCOL_VERSION,
        ),
    ] {
        assert_eq!(mcp::negotiate_protocol_version(requested), want, "{name}");
    }
}

#[test]
fn request_is_notification() {
    assert!(
        !Request::from_value(&json!({"id": 1}))
            .unwrap()
            .is_notification(),
        "with id"
    );
    assert!(
        Request::from_value(&json!({})).unwrap().is_notification(),
        "without id"
    );
    // A JSON null id is a (malformed) request, not a notification.
    assert!(
        !Request::from_value(&json!({"id": null}))
            .unwrap()
            .is_notification(),
        "null id"
    );
}

#[test]
fn error_codes() {
    assert_eq!(mcp::ERR_PARSE, -32700);
    assert_eq!(mcp::ERR_INVALID_REQUEST, -32600);
    assert_eq!(mcp::ERR_METHOD_NOT_FOUND, -32601);
    assert_eq!(mcp::ERR_INVALID_PARAMS, -32602);
    assert_eq!(mcp::ERR_INTERNAL, -32603);
    assert_eq!(mcp::ERR_HEADER_MISMATCH, -32020);
    assert_eq!(mcp::ERR_MISSING_CLIENT_CAPABILITY, -32021);
    assert_eq!(mcp::ERR_UNSUPPORTED_PROTOCOL_VERSION, -32022);
}

#[test]
fn request_json() {
    let r = Request::from_value(&json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05"}})).unwrap();
    assert_eq!(
        (r.jsonrpc.as_str(), r.method.as_str()),
        ("2.0", "initialize")
    );
    assert_eq!(r.id_key(), "1");
    assert!(r.params.is_some());
    let r =
        Request::from_value(&json!({"jsonrpc":"2.0","id":"abc","method":"tools/list"})).unwrap();
    assert_eq!(
        (r.jsonrpc.as_str(), r.method.as_str()),
        ("2.0", "tools/list")
    );
    // String and number ids never collide.
    assert_eq!(r.id_key(), r#""abc""#);
    let r = Request::from_value(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}))
        .unwrap();
    assert_eq!(r.method, "notifications/initialized");
    assert!(r.is_notification());
    // Type errors use the established decoder wording.
    assert_eq!(
        Request::from_value(&json!([1])).unwrap_err(),
        "json: cannot unmarshal array into Go value of type mcp.Request"
    );
    assert_eq!(
        Request::from_value(&json!({"method": 5})).unwrap_err(),
        "json: cannot unmarshal number into Go struct field Request.method of type string"
    );
}

#[test]
fn response_json() {
    let ok: Value = serde_json::from_slice(
        &Response::result(Some(json!(1)), json!({"status":"ok"})).to_bytes(),
    )
    .unwrap();
    assert_eq!(ok["jsonrpc"], mcp::JSONRPC_VERSION);
    assert!(ok.get("error").is_none());
    let err: Value = serde_json::from_slice(
        &Response::error(
            Some(json!(1)),
            mcp::ERR_METHOD_NOT_FOUND,
            "Method not found",
            "unknown_method",
        )
        .to_bytes(),
    )
    .unwrap();
    assert_eq!(err["error"]["code"], mcp::ERR_METHOD_NOT_FOUND);
    assert_eq!(err["error"]["message"], "Method not found");
    assert_eq!(err["error"]["data"], "unknown_method");
    // Null data is omitted (omitempty).
    let bare = String::from_utf8(
        Response::error(None, mcp::ERR_PARSE, "Parse error", Value::Null).to_bytes(),
    )
    .unwrap();
    assert_eq!(
        bare,
        r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"}}"#
    );
}

// Initialize params and result, through the dispatcher that owns the
// InitializeParams type.
#[tokio::test]
async fn initialize_params_and_result() {
    let d = Dispatcher::new(Arc::new(Registry::new(None)));
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{
            "protocolVersion":"2024-11-05",
            "capabilities":{"roots":{"listChanged":true}},
            "clientInfo":{"name":"test-client","version":"1.0.0"}}}),
    )
    .await;
    let r = &v["result"];
    assert_eq!(r["protocolVersion"], "2024-11-05");
    assert!(
        r["capabilities"]["tools"].is_object(),
        "expected tools capability"
    );
    assert_eq!(r["serverInfo"]["name"], dispatch::SERVER_NAME);
    assert_eq!(r["serverInfo"]["title"], dispatch::SERVER_TITLE);
    assert_eq!(r["serverInfo"]["version"], dispatch::SERVER_VERSION);
    // Malformed params are invalid params with the decoder's message.
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":2,"method":"initialize","params":{"protocolVersion": 5}}),
    )
    .await;
    assert_eq!(v["error"]["code"], mcp::ERR_INVALID_PARAMS);
}

#[test]
fn tool_json() {
    let t = Tool::from_json(
        r#"{"name":"test_tool","description":"A test tool","inputSchema":{"type":"object",
            "properties":{"param1":{"type":"string","description":"First parameter"},"param2":{"type":"integer","default":10}},
            "required":["param1"]}}"#,
    );
    assert_eq!(t.name, "test_tool");
    assert_eq!(t.description(), "A test tool");
    assert_eq!(t.input_schema()["type"], "object");
    assert_eq!(t.input_schema()["properties"].as_object().unwrap().len(), 2);
    assert_eq!(t.input_schema()["required"], json!(["param1"]));
    // The definition is advertised verbatim, field order included.
    assert_eq!(
        String::from_utf8(to_json_bytes(&t.def)).unwrap(),
        r#"{"name":"test_tool","description":"A test tool","inputSchema":{"type":"object","properties":{"param1":{"type":"string","description":"First parameter"},"param2":{"type":"integer","default":10}},"required":["param1"]}}"#
    );
}

#[tokio::test]
async fn tools_call_params() {
    let seen: Arc<Mutex<Option<Value>>> = Arc::default();
    let mut r = empty_registry();
    let s = seen.clone();
    let handler: Handler = Arc::new(move |_c, args| {
        *s.lock().unwrap() = args;
        Box::pin(async { Ok(CallToolResult::success("ok")) }) as HandlerFuture
    });
    r.register(
        Tool::from_json(
            r#"{"name":"list_apps","description":"d","inputSchema":{"type":"object"}}"#,
        ),
        handler,
    );
    let d = Dispatcher::new(Arc::new(r));
    let sess = Session::new();
    initialize(&d, &sess).await;
    dispatch(&d, &sess, json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"list_apps","arguments":{"limit":50}}})).await;
    assert_eq!(*seen.lock().unwrap(), Some(json!({"limit": 50})));
    dispatch(
        &d,
        &sess,
        json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"list_apps"}}),
    )
    .await;
    assert_eq!(*seen.lock().unwrap(), None, "absent arguments stay absent");
}

#[test]
fn tools_call_result_json() {
    let ok = CallToolResult::success("ok");
    assert_eq!(
        String::from_utf8(to_json_bytes(&ok.to_value())).unwrap(),
        r#"{"content":[{"type":"text","text":"ok"}]}"#
    );
    let err = CallToolResult::error("Error message").to_value();
    assert_eq!(err["isError"], true);
    assert_eq!(err["content"][0]["text"], "Error message");
    let modern = CallToolResult {
        result_type: Some(mcp::RESULT_TYPE_COMPLETE),
        ..CallToolResult::success("ok")
    }
    .to_value();
    assert_eq!(modern["resultType"], mcp::RESULT_TYPE_COMPLETE);
    // Structured content: Some(Null) is an explicit `null`; None omits.
    let typed_nil = CallToolResult {
        structured_content: Some(Value::Null),
        ..CallToolResult::success("x")
    }
    .to_value();
    assert!(
        typed_nil
            .as_object()
            .unwrap()
            .contains_key("structuredContent")
    );
    assert!(
        !ok.to_value()
            .as_object()
            .unwrap()
            .contains_key("structuredContent")
    );
}

#[test]
fn new_text_content() {
    let c = text_content("Hello, World!");
    assert_eq!(c["type"], "text");
    assert_eq!(c["text"], "Hello, World!");
    // An empty text is omitted (omitempty).
    assert_eq!(text_content(""), json!({"type": "text"}));
}

#[test]
fn new_success_result() {
    let r = CallToolResult::success("Operation completed");
    assert!(!r.is_error);
    assert_eq!(r.content.len(), 1);
    assert_eq!(r.content[0]["type"], "text");
    assert_eq!(r.text(), "Operation completed");
}

#[test]
fn new_error_result() {
    let r = CallToolResult::error("Something went wrong");
    assert!(r.is_error);
    assert_eq!(r.content.len(), 1);
    assert_eq!(r.content[0]["type"], "text");
    assert_eq!(r.text(), "Something went wrong");
}

#[tokio::test]
async fn tools_list_result_shapes() {
    let mut r = empty_registry();
    r.register(
        Tool::from_json(
            r#"{"name":"tool1","description":"First tool","inputSchema":{"type":"object"}}"#,
        ),
        Arc::new(|_c, _a| Box::pin(async { Ok(CallToolResult::success("")) }) as HandlerFuture),
    );
    let d = Dispatcher::new(Arc::new(r));
    let sess = Session::new();
    initialize(&d, &sess).await;
    // The legacy shape is pinned byte for byte.
    let out = d
        .dispatch(
            &req(json!({"jsonrpc":"2.0","id":1,"method":"tools/list"})),
            &sess,
        )
        .await
        .unwrap();
    assert_eq!(
        String::from_utf8(out.response.to_bytes()).unwrap(),
        r#"{"jsonrpc":"2.0","id":1,"result":{"tools":[{"name":"tool1","description":"First tool","inputSchema":{"type":"object"}}]}}"#
    );
    // Modern fields.
    let v = dispatch(&d, &Session::new(), json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":modern_meta("2026-07-28", true)})).await;
    let keys: Vec<&String> = v["result"].as_object().unwrap().keys().collect();
    assert_eq!(
        keys,
        ["tools", "resultType", "ttlMs", "cacheScope", "_meta"]
    );
    assert_eq!(v["result"]["ttlMs"], dispatch::CACHE_TTL_MS);
}

#[test]
fn parse_request_meta() {
    // full modern _meta
    let p = json!({"name":"list_apps","_meta":{
        "io.modelcontextprotocol/protocolVersion":"2026-07-28",
        "io.modelcontextprotocol/clientCapabilities":{"elicitation":{},"extensions":{"io.example/thing":{"x":1}}},
        "io.modelcontextprotocol/clientInfo":{"name":"test-client","version":"1.0.0"},
        "progressToken":"tok-1"}});
    let m = RequestMeta::parse(Some(&p)).unwrap();
    assert_eq!(m.protocol_version, "2026-07-28");
    let caps = m.client_capabilities.expect("client capabilities");
    assert!(caps.contains_key("elicitation"));
    assert!(
        caps["extensions"].get("io.example/thing").is_some(),
        "extension preserved"
    );
    assert_eq!(m.client_info.unwrap()["name"], "test-client");
    assert_eq!(m.progress_token, Some(json!("tok-1")));

    // empty params
    let m = RequestMeta::parse(None).unwrap();
    assert!(
        m.protocol_version.is_empty() && m.client_capabilities.is_none() && m.client_info.is_none()
    );

    // params without _meta
    assert!(
        RequestMeta::parse(Some(&json!({"name":"list_apps"})))
            .unwrap()
            .client_capabilities
            .is_none()
    );

    // clientCapabilities present as {}
    let m = RequestMeta::parse(Some(
        &json!({"_meta":{"io.modelcontextprotocol/clientCapabilities":{}}}),
    ))
    .unwrap();
    assert!(
        m.client_capabilities.is_some(),
        "present as {{}} must be Some"
    );

    // clientCapabilities absent
    let m = RequestMeta::parse(Some(
        &json!({"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}),
    ))
    .unwrap();
    assert!(m.client_capabilities.is_none());

    // malformed protocolVersion type
    assert!(
        RequestMeta::parse(Some(
            &json!({"_meta":{"io.modelcontextprotocol/protocolVersion":42}})
        ))
        .is_err()
    );

    // Syntactically malformed params (`{`) never reach this parser, which
    // takes parsed values; params of the wrong type are rejected instead.
    assert!(RequestMeta::parse(Some(&json!("{"))).is_err());
    assert!(RequestMeta::parse(Some(&json!({"_meta": 5}))).is_err());

    // Non-object (including null) clientCapabilities are rejected.
    assert!(
        RequestMeta::parse(Some(
            &json!({"_meta":{"io.modelcontextprotocol/clientCapabilities":null}})
        ))
        .is_err()
    );
    assert!(
        RequestMeta::parse(Some(
            &json!({"_meta":{"io.modelcontextprotocol/clientCapabilities":[]}})
        ))
        .is_err()
    );
    // A null _meta is treated as absent.
    assert!(
        RequestMeta::parse(Some(&json!({"_meta": null})))
            .unwrap()
            .protocol_version
            .is_empty()
    );
}

// Result type and cache scope constants (only "complete" is defined; the
// server never returns input_required).
#[test]
fn result_type_constants() {
    assert_eq!(mcp::RESULT_TYPE_COMPLETE, "complete");
    assert_eq!(mcp::CACHE_SCOPE_PUBLIC, "public");
}

#[test]
fn meta_key_constants() {
    assert_eq!(
        mcp::META_PROTOCOL_VERSION,
        "io.modelcontextprotocol/protocolVersion"
    );
    assert_eq!(
        mcp::META_CLIENT_CAPABILITIES,
        "io.modelcontextprotocol/clientCapabilities"
    );
    assert_eq!(mcp::META_CLIENT_INFO, "io.modelcontextprotocol/clientInfo");
    assert_eq!(mcp::META_SERVER_INFO, "io.modelcontextprotocol/serverInfo");
    assert_eq!(mcp::META_LOG_LEVEL, "io.modelcontextprotocol/logLevel");
    assert_eq!(
        mcp::META_SUBSCRIPTION_ID,
        "io.modelcontextprotocol/subscriptionId"
    );
}

#[tokio::test]
async fn discover_result_json() {
    let d = Dispatcher::new(Arc::new(Registry::new(None)));
    let v = dispatch(
        &d,
        &Session::new(),
        json!({"jsonrpc":"2.0","id":1,"method":"server/discover"}),
    )
    .await;
    let r = v["result"].as_object().unwrap();
    for key in [
        "resultType",
        "supportedVersions",
        "capabilities",
        "instructions",
        "ttlMs",
        "cacheScope",
        "_meta",
    ] {
        assert!(r.contains_key(key), "discover result missing key {key}");
    }
    assert_eq!(r["resultType"], mcp::RESULT_TYPE_COMPLETE);
    assert_eq!(
        r["supportedVersions"].as_array().unwrap().len(),
        mcp::SUPPORTED_PROTOCOL_VERSIONS.len()
    );
    assert_eq!(r["ttlMs"], dispatch::CACHE_TTL_MS);
    assert_eq!(r["cacheScope"], mcp::CACHE_SCOPE_PUBLIC);
    assert_eq!(r["instructions"], dispatch::SERVER_INSTRUCTIONS);
    assert!(r["_meta"].get(mcp::META_SERVER_INFO).is_some());
}

#[tokio::test]
async fn unsupported_protocol_version_data_json() {
    let d = Dispatcher::new(Arc::new(Registry::new(None)));
    let v = dispatch(&d, &Session::new(), json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":modern_meta("1999-01-01", true)})).await;
    let data = v["error"]["data"].as_object().unwrap();
    assert!(data.contains_key("supported"));
    assert_eq!(data["requested"], "1999-01-01");
}

#[test]
fn server_info_meta_shape() {
    let m = server_info_meta();
    assert_eq!(m[mcp::META_SERVER_INFO]["name"], "asc-mcp");
    assert_eq!(
        m[mcp::META_SERVER_INFO]["version"],
        dispatch::SERVER_VERSION
    );
}
