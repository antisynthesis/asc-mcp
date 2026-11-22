//! stdio transport tests, ported from internal/asc/server/server_test.go
//! (the `Run`-level cases).

mod common;

use std::sync::Arc;
use std::time::Duration;

use asc_mcp::api::Client;
use asc_mcp::config::Config;
use asc_mcp::mcp::{self, CallToolResult};
use asc_mcp::server::dispatch::Dispatcher;
use asc_mcp::server::stdio::StdioServer;
use asc_mcp::tools::Registry;
use common::*;
use serde_json::{json, Value};
use tokio::io::{AsyncWriteExt, BufReader, DuplexStream};

/// Runs a server over a fixed input to EOF and returns its output.
async fn run_with_input(registry: Registry, input: Vec<u8>) -> SharedBuf {
    let out = SharedBuf::default();
    let srv = StdioServer::new(Arc::new(Dispatcher::new(Arc::new(registry))));
    tokio::time::timeout(Duration::from_secs(10), srv.run(&input[..], out.clone()))
        .await
        .expect("Run did not return after EOF")
        .expect("run");
    out
}

/// Go `waitForRun` + `io.Pipe`: spawns a server reading from a duplex
/// pipe; the caller writes requests and drops the writer for EOF.
fn spawn_piped(registry: Registry) -> (DuplexStream, SharedBuf, tokio::task::JoinHandle<std::io::Result<()>>) {
    let (client, server_end) = tokio::io::duplex(64 * 1024);
    let out = SharedBuf::default();
    let srv = StdioServer::new(Arc::new(Dispatcher::new(Arc::new(registry))));
    let o = out.clone();
    let task = tokio::spawn(async move { srv.run(BufReader::new(server_end), o).await });
    (client, out, task)
}

async fn wait_for_run(task: tokio::task::JoinHandle<std::io::Result<()>>) {
    tokio::time::timeout(Duration::from_secs(10), task)
        .await
        .expect("Run did not return after EOF")
        .expect("run task")
        .expect("Run failed");
}

async fn wait_notified(n: &tokio::sync::Notify, what: &str) {
    tokio::time::timeout(Duration::from_secs(5), n.notified())
        .await
        .unwrap_or_else(|_| panic!("{what}"));
}

// Go: TestNew. The Rust server is assembled from parts: a client built from
// the config's key, the full registry and a dispatcher.
#[tokio::test]
async fn new_from_config() {
    let (_dir, key) = write_test_key();
    let cfg = Config {
        issuer_id: "test-issuer".into(),
        key_id: "TESTKEY123".into(),
        private_key_path: key.to_string_lossy().into_owned(),
    };
    let client = Client::new(&cfg.issuer_id, &cfg.key_id, &cfg.private_key_path).expect("client");
    let registry = Arc::new(Registry::new(Some(Arc::new(client))));
    assert!(!registry.list_tools().is_empty(), "registry not initialized");
    let d = Arc::new(Dispatcher::new(registry));
    let _srv = StdioServer::new(d.clone());
    // A fresh server's session is not initialized: legacy tools/list is
    // gated until initialize.
    let out = run_with_input(Registry::new(None), line(json!({"jsonrpc":"2.0","id":1,"method":"tools/list"}))).await;
    assert_eq!(out.lines()[0]["error"]["code"], mcp::ERR_INVALID_REQUEST);
}

// Go: TestServer_Run_PipelinedHandshake
#[tokio::test]
async fn pipelined_handshake() {
    let mut input = line(json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":init_params("2025-06-18")}));
    input.extend(line(json!({"jsonrpc":"2.0","method":"notifications/initialized"})));
    input.extend(line(json!({"jsonrpc":"2.0","id":2,"method":"tools/list"})));
    let out = run_with_input(Registry::new(None), input).await;
    let resp = out.lines().into_iter().find(|v| v["id"] == 2).expect("no response for the pipelined tools/list request");
    assert!(resp.get("error").is_none(), "tools/list after pipelined handshake: {resp}");
}

// Go: TestServer_Run_ParseError
#[tokio::test]
async fn parse_error() {
    let out = run_with_input(Registry::new(None), b"not valid json\n".to_vec()).await;
    let lines = out.lines();
    assert_eq!(lines[0]["error"]["code"], mcp::ERR_PARSE);
    assert!(lines[0].get("id").is_none() || lines[0]["id"].is_null());
}

// Go: TestServer_Run_EmptyLines
#[tokio::test]
async fn empty_lines() {
    let out = run_with_input(Registry::new(None), b"\n\n\n".to_vec()).await;
    assert_eq!(out.contents(), "");
}

// Go: TestServer_ConcurrentWrites. The Rust transport has no exported
// send method, so the property is pinned end to end: many concurrent
// responses are each written as one intact line.
#[tokio::test]
async fn concurrent_writes() {
    let mut r = empty_registry();
    register_text_tool(&mut r, "quick_tool", "done");
    let mut input = line(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":init_params("2025-06-18")}));
    for i in 1..=10 {
        input.extend(line(json!({"jsonrpc":"2.0","id":i,"method":"tools/call","params":{"name":"quick_tool"}})));
    }
    let out = run_with_input(r, input).await;
    let lines = out.lines();
    assert_eq!(lines.len(), 11, "expected 11 responses: {:?}", out.contents());
    let mut ids: Vec<i64> = lines.iter().filter_map(|v| v["id"].as_i64()).filter(|i| *i > 0).collect();
    ids.sort();
    assert_eq!(ids, (1..=10).collect::<Vec<_>>());
}

// Go: TestServer_Discover_PreInitialize
#[tokio::test]
async fn discover_pre_initialize() {
    let mut input = line(json!({"jsonrpc":"2.0","id":1,"method":"server/discover"}));
    // Probe the session afterwards: discover must not initialize it.
    input.extend(line(json!({"jsonrpc":"2.0","id":2,"method":"tools/list"})));
    let out = run_with_input(Registry::new(None), input).await;
    let lines = out.lines();
    let resp = &lines[0];
    assert!(resp.get("error").is_none(), "unexpected error: {resp}");
    let r = &resp["result"];
    let versions: Vec<&str> = r["supportedVersions"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
    for want in ["2026-07-28", "2025-06-18"] {
        assert!(versions.contains(&want), "supportedVersions missing {want}: {versions:?}");
    }
    assert!(r["capabilities"]["tools"].is_object(), "expected capabilities.tools");
    assert_eq!(r["resultType"], mcp::RESULT_TYPE_COMPLETE);
    assert!(r["ttlMs"].as_i64().unwrap() > 0);
    assert_eq!(r["cacheScope"], mcp::CACHE_SCOPE_PUBLIC);
    assert_eq!(r["_meta"][mcp::META_SERVER_INFO]["name"], asc_mcp::server::dispatch::SERVER_NAME);
    assert_eq!(lines[1]["error"]["code"], mcp::ERR_INVALID_REQUEST, "server/discover must not mark the session initialized");
}

/// subscriptions/listen over stdio: the acknowledgement notification is
/// written before the completion result.
#[tokio::test]
async fn subscriptions_listen_writes_ack_then_result() {
    let input = line(
        json!({"jsonrpc":"2.0","id":9,"method":"subscriptions/listen","params":modern_params("2026-07-28", json!({"notifications":{}}))}),
    );
    let out = run_with_input(Registry::new(None), input).await;
    let lines = out.lines();
    assert_eq!(lines.len(), 2, "{}", out.contents());
    assert_eq!(lines[0]["method"], "notifications/subscriptions/acknowledged");
    assert_eq!(lines[0]["params"]["_meta"][mcp::META_SUBSCRIPTION_ID], 9);
    assert_eq!(lines[1]["id"], 9);
    assert_eq!(lines[1]["result"]["resultType"], mcp::RESULT_TYPE_COMPLETE);
}

// Go: TestServer_Run_CancelledRequest_NoResponse
#[tokio::test]
async fn cancelled_request_no_response() {
    let started = Arc::new(tokio::sync::Notify::new());
    let mut r = empty_registry();
    {
        let started = started.clone();
        register_fake(&mut r, "slow_tool", move || {
            let started = started.clone();
            async move {
                started.notify_one();
                // Never finishes on its own; a late completion after the
                // cancellation would be suppressed anyway.
                tokio::time::sleep(Duration::from_secs(3600)).await;
                Ok(CallToolResult::success("finished after cancel"))
            }
        });
    }
    let (mut w, out, task) = spawn_piped(r);
    w.write_all(&line(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":init_params("2025-06-18")}))).await.unwrap();
    w.write_all(&line(json!({"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"slow_tool"}}))).await.unwrap();
    // The read loop must stay free while the call is in flight.
    wait_notified(&started, "tool call never started; requests are not dispatched concurrently").await;
    w.write_all(&line(json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":7,"reason":"user gave up"}})))
        .await
        .unwrap();
    drop(w);
    wait_for_run(task).await;
    // Only the initialize response: nothing for the cancelled request and
    // nothing for the notification.
    let lines = out.lines();
    assert_eq!(lines.len(), 1, "expected no output for cancelled request, got {:?}", out.contents());
    assert_eq!(lines[0]["id"], 0);
}

// Go: TestServer_Run_CancelUnknownID_Ignored
#[tokio::test]
async fn cancel_unknown_id_ignored() {
    let mut input = Vec::new();
    for params in [r#"{"requestId":999,"reason":"never existed"}"#, r#"{"reason":"missing requestId"}"#] {
        input.extend(format!(r#"{{"jsonrpc":"2.0","method":"notifications/cancelled","params":{params}}}"#).into_bytes());
        input.push(b'\n');
    }
    // Go's malformed `{` params case cannot be expressed as a valid JSON
    // line; the closest is params of the wrong type.
    input.extend(line(json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":"{"})));
    let out = run_with_input(Registry::new(None), input).await;
    assert_eq!(out.contents(), "", "expected no output for ignored cancellations");
}

// Go: TestServer_Run_CancelIDTypeMismatch
#[tokio::test]
async fn cancel_id_type_mismatch() {
    let started = Arc::new(tokio::sync::Notify::new());
    let release = Arc::new(tokio::sync::Notify::new());
    let mut r = empty_registry();
    {
        let (started, release) = (started.clone(), release.clone());
        register_fake(&mut r, "slow_tool", move || {
            let (started, release) = (started.clone(), release.clone());
            async move {
                started.notify_one();
                release.notified().await;
                Ok(CallToolResult::success("completed"))
            }
        });
    }
    let (mut w, out, task) = spawn_piped(r);
    w.write_all(&line(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":init_params("2025-06-18")}))).await.unwrap();
    w.write_all(&line(json!({"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"slow_tool"}}))).await.unwrap();
    wait_notified(&started, "tool call never started").await;
    // The string id "7" must not match the in-flight number id 7.
    w.write_all(&line(json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":"7"}}))).await.unwrap();
    w.flush().await.unwrap();
    // Give the read loop time to process the cancellation before release.
    tokio::time::sleep(Duration::from_millis(50)).await;
    release.notify_one();
    drop(w);
    wait_for_run(task).await;
    let lines = out.lines();
    let resp = lines.iter().find(|v| v["id"] == 7).expect("number-id request got no response");
    assert!(resp.get("error").is_none(), "unexpected error: {resp}");
    assert!(resp["result"].get("isError").is_none(), "string requestId \"7\" cancelled the number-id request: {resp}");
    assert_eq!(resp["result"]["content"][0]["text"], "completed");
}

// Go: TestServer_Run_ConcurrentRequests
#[tokio::test]
async fn concurrent_requests() {
    let second_started = Arc::new(tokio::sync::Notify::new());
    let mut r = empty_registry();
    {
        let s = second_started.clone();
        register_fake(&mut r, "first_tool", move || {
            let s = s.clone();
            async move {
                // Refuses to finish until second_tool started, so both
                // responses arriving proves the requests interleaved.
                match tokio::time::timeout(Duration::from_secs(5), s.notified()).await {
                    Ok(()) => Ok(CallToolResult::success("first")),
                    Err(_) => Ok(CallToolResult::error("second request never started; dispatch is sequential")),
                }
            }
        });
    }
    {
        let s = second_started.clone();
        register_fake(&mut r, "second_tool", move || {
            let s = s.clone();
            async move {
                s.notify_one();
                Ok(CallToolResult::success("second"))
            }
        });
    }
    let (mut w, out, task) = spawn_piped(r);
    w.write_all(&line(json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":init_params("2025-06-18")}))).await.unwrap();
    w.write_all(&line(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"first_tool"}}))).await.unwrap();
    w.write_all(&line(json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"second_tool"}}))).await.unwrap();
    drop(w);
    wait_for_run(task).await;
    let lines: Vec<Value> = out.lines().into_iter().filter(|v| v["id"] != 0).collect();
    assert_eq!(lines.len(), 2, "expected 2 responses, got {:?}", out.contents());
    let mut seen = Vec::new();
    for resp in &lines {
        assert!(resp.get("error").is_none(), "unexpected error: {resp}");
        assert!(resp["result"].get("isError").is_none(), "tool call failed: {resp}");
        seen.push(resp["id"].as_i64().unwrap());
    }
    seen.sort();
    assert_eq!(seen, vec![1, 2]);
}
