//! Shared helpers for the protocol-layer integration tests (ported from
//! the Go `server` package's test helpers).

#![allow(dead_code)]

use std::io::Write as _;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use asc_mcp::api::testutil::{test_token_provider, TEST_PRIVATE_KEY};
use asc_mcp::api::Client;
use asc_mcp::mcp::{self, CallToolResult, Request, Tool};
use asc_mcp::server::dispatch::{Dispatcher, Outcome, Session};
use asc_mcp::tools::{Handler, HandlerFuture, HandlerResult, Registry};
use serde_json::{json, Value};

/// Writes [`TEST_PRIVATE_KEY`] to a temp dir (Go `testSetup`); keep the
/// returned dir alive for as long as the path is used.
pub fn write_test_key() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("test_key.p8");
    let mut f = std::fs::File::create(&path).expect("create key");
    f.write_all(TEST_PRIVATE_KEY.as_bytes()).expect("write key");
    (dir, path)
}

/// A client whose API calls go nowhere useful (fake tools never call it).
pub fn dummy_client() -> Arc<Client> {
    Arc::new(Client::with_token_provider(test_token_provider(), "http://127.0.0.1:9"))
}

/// An empty registry that fake tools can be added to.
pub fn empty_registry() -> Registry {
    Registry::empty(Some(dummy_client()))
}

/// Go `registerFakeTool`: installs a test-only tool.
pub fn register_fake<F, Fut>(r: &mut Registry, name: &str, f: F)
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = HandlerResult> + Send + 'static,
{
    let tool = Tool::from_json(
        &json!({
            "name": name,
            "description": "test-only tool",
            "inputSchema": {"type": "object"},
        })
        .to_string(),
    );
    let handler: Handler = Arc::new(move |_c, _a| Box::pin(f()) as HandlerFuture);
    r.register(tool, handler);
}

/// A fake tool that returns a fixed success text.
pub fn register_text_tool(r: &mut Registry, name: &str, text: &'static str) {
    register_fake(r, name, move || async move { Ok(CallToolResult::success(text)) });
}

/// Parses a request from JSON text.
pub fn req(v: Value) -> Request {
    Request::from_value(&v).expect("valid request")
}

/// Go `modernMeta`: params carrying the 2026-07-28 `_meta` keys.
pub fn modern_meta(version: &str, with_caps: bool) -> Value {
    let mut meta = serde_json::Map::new();
    meta.insert(mcp::META_PROTOCOL_VERSION.into(), json!(version));
    if with_caps {
        meta.insert(mcp::META_CLIENT_CAPABILITIES.into(), json!({}));
    }
    json!({ "_meta": meta })
}

/// Modern params with extra top-level members.
pub fn modern_params(version: &str, extra: Value) -> Value {
    let mut p = modern_meta(version, true);
    if let (Value::Object(p), Value::Object(e)) = (&mut p, extra) {
        p.extend(e);
    }
    p
}

/// Initialize params for a legacy handshake.
pub fn init_params(version: &str) -> Value {
    json!({"protocolVersion": version, "capabilities": {}, "clientInfo": {"name": "t", "version": "1"}})
}

/// The response of an outcome as JSON.
pub fn resp_json(out: &Outcome) -> Value {
    serde_json::from_slice(&out.response.to_bytes()).expect("response JSON")
}

/// Dispatches and returns the response JSON, panicking on no response.
pub async fn dispatch(d: &Dispatcher, sess: &Session, r: Value) -> Value {
    let out = d.dispatch(&req(r), sess).await.expect("a response");
    resp_json(&out)
}

/// Dispatches `initialize` so legacy-gated methods are served.
pub async fn initialize(d: &Dispatcher, sess: &Session) {
    let v = dispatch(d, sess, json!({"jsonrpc":"2.0","id":0,"method":"initialize","params":init_params("2025-06-18")})).await;
    assert!(v.get("error").is_none(), "initialize failed: {v}");
}

/// An `AsyncWrite` sink whose bytes stay readable after the server took
/// ownership of the writer.
#[derive(Clone, Default)]
pub struct SharedBuf(pub Arc<Mutex<Vec<u8>>>);

impl SharedBuf {
    pub fn contents(&self) -> String {
        String::from_utf8(self.0.lock().unwrap().clone()).expect("utf8 output")
    }
    /// Each non-empty output line parsed as JSON.
    pub fn lines(&self) -> Vec<Value> {
        self.contents()
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| serde_json::from_str(l).unwrap_or_else(|e| panic!("bad line {l:?}: {e}")))
            .collect()
    }
}

impl tokio::io::AsyncWrite for SharedBuf {
    fn poll_write(self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &[u8]) -> Poll<std::io::Result<usize>> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Poll::Ready(Ok(buf.len()))
    }
    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

/// Serializes one request as a stdio line.
pub fn line(v: Value) -> Vec<u8> {
    let mut b = serde_json::to_vec(&v).unwrap();
    b.push(b'\n');
    b
}
