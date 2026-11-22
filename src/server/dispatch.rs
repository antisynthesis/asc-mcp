//! Transport-independent JSON-RPC dispatch.
//!
//! One [`Dispatcher`] serves both protocol eras:
//!
//! * **Legacy** (2024-11-05 … 2025-11-25): `initialize` handshake, gated
//!   per [`Session`].
//! * **Modern** (2026-07-28): stateless; every request carries its
//!   protocol version and client capabilities in `params._meta`, and
//!   `server/discover` advertises what the server speaks.
//!
//! The era is chosen per request, never per connection.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde_json::{json, Map, Value};

use crate::go::json::unmarshal_value;
use crate::go::{GoMap, RawMessage};
use crate::go_struct;
use crate::logging;
use crate::mcp::{self, CallToolResult, Request, RequestMeta, Response};
use crate::tools::{CallError, Registry};

use super::metrics::Metrics;

/// Server name reported to clients.
pub const SERVER_NAME: &str = "asc-mcp";
/// Server display title.
pub const SERVER_TITLE: &str = "App Store Connect";
/// Server version.
pub const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Upper bound on a single tool invocation.
pub const TOOL_CALL_TIMEOUT: Duration = Duration::from_secs(60);

/// Freshness hint for cacheable modern results: the tool set is fixed for
/// the life of the process.
pub const CACHE_TTL_MS: i64 = 3_600_000;

/// Instructions returned by `initialize` and `server/discover`.
pub const SERVER_INSTRUCTIONS: &str = "This server provides tools for managing Apple App Store Connect resources, including apps, builds, TestFlight, provisioning, in-app purchases, subscriptions, reviews, screenshots, localizations, analytics, Game Center, Xcode Cloud, and more. Tool names use snake_case and accept JSON arguments. Call tools/list to discover the available tools and their input schemas.";

/// Per-connection state for the legacy handshake.
#[derive(Debug)]
pub struct Session {
    initialized: AtomicBool,
    /// When the session was created.
    pub created_at: Instant,
    last_active: Mutex<Instant>,
}

impl Default for Session {
    fn default() -> Self {
        let now = Instant::now();
        Self { initialized: AtomicBool::new(false), created_at: now, last_active: Mutex::new(now) }
    }
}

impl Session {
    /// A fresh, uninitialized session.
    pub fn new() -> Self {
        Self::default()
    }
    /// Records activity.
    pub fn touch(&self) {
        *self.last_active.lock().unwrap_or_else(|e| e.into_inner()) = Instant::now();
    }
    /// Most recent activity.
    pub fn last_active(&self) -> Instant {
        *self.last_active.lock().unwrap_or_else(|e| e.into_inner())
    }
    /// Whether `initialize` has completed on this session.
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }
}

/// What to send back for a request: zero or more notifications that
/// precede the response (only `subscriptions/listen` produces any), then
/// the response itself.
#[derive(Debug, Clone)]
pub struct Outcome {
    /// Notifications to deliver before the response, in order.
    pub notifications: Vec<Value>,
    /// The response.
    pub response: Response,
}

impl From<Response> for Outcome {
    fn from(response: Response) -> Self {
        Self { notifications: Vec::new(), response }
    }
}

/// Reports whether a request should take the modern (2026-07-28) path:
/// it carries a `_meta` protocol version or is `server/discover`.
/// `initialize` always selects legacy semantics.
pub fn is_modern(method: &str, meta: &RequestMeta) -> bool {
    method != "initialize" && (!meta.protocol_version.is_empty() || method == "server/discover")
}

go_struct! {
    struct RootsCapability ("mcp.RootsCapability", "RootsCapability") {
        list_changed: bool => "listChanged", omitempty;
    }
}
go_struct! {
    struct EmptyCapability ("mcp.EmptyCapability", "EmptyCapability") {}
}
go_struct! {
    struct ClientCapability ("mcp.ClientCapability", "ClientCapability") {
        roots: Option<RootsCapability> => "roots", omitempty;
        sampling: Option<EmptyCapability> => "sampling", omitempty;
        elicitation: Option<EmptyCapability> => "elicitation", omitempty;
        experimental: GoMap<Value> => "experimental", omitempty;
        extensions: GoMap<RawMessage> => "extensions", omitempty;
    }
}
go_struct! {
    struct ClientInfo ("mcp.ClientInfo", "ClientInfo") {
        name: String => "name";
        title: String => "title", omitempty;
        version: String => "version";
    }
}
go_struct! {
    struct InitializeParams ("mcp.InitializeParams", "InitializeParams") {
        protocol_version: String => "protocolVersion";
        capabilities: ClientCapability => "capabilities";
        client_info: ClientInfo => "clientInfo";
    }
}
go_struct! {
    struct ToolsCallParams ("mcp.ToolsCallParams", "ToolsCallParams") {
        name: String => "name";
        arguments: RawMessage => "arguments", omitempty;
    }
}

/// Handles JSON-RPC requests against the tool registry.
pub struct Dispatcher {
    registry: Arc<Registry>,
    metrics: Option<Arc<Metrics>>,
}

impl Dispatcher {
    /// Creates a dispatcher over `registry`.
    pub fn new(registry: Arc<Registry>) -> Self {
        Self { registry, metrics: None }
    }

    /// Enables tool-call metrics.
    pub fn with_metrics(mut self, m: Arc<Metrics>) -> Self {
        self.metrics = Some(m);
        self
    }

    /// The tool registry.
    pub fn registry(&self) -> &Arc<Registry> {
        &self.registry
    }

    /// Handles one request. Returns `None` for notifications and anything
    /// else that must not be answered.
    pub async fn dispatch(&self, req: &Request, sess: &Session) -> Option<Outcome> {
        sess.touch();
        if req.jsonrpc != mcp::JSONRPC_VERSION {
            if req.is_notification() {
                return None;
            }
            return Some(err(req, mcp::ERR_INVALID_REQUEST, "Invalid Request", "jsonrpc must be 2.0"));
        }
        let meta = match RequestMeta::parse(req.params.as_ref()) {
            Ok(m) => m,
            Err(e) => {
                if req.is_notification() {
                    return None;
                }
                return Some(err(req, mcp::ERR_INVALID_PARAMS, "Invalid params", e));
            }
        };
        if is_modern(&req.method, &meta) {
            return self.dispatch_modern(req, &meta).await;
        }
        match req.method.as_str() {
            "initialize" => Some(self.handle_initialize(req, sess)),
            // No response is permitted to these notifications. The stdio
            // transport intercepts notifications/cancelled before dispatch.
            "notifications/initialized" | "notifications/cancelled" => None,
            "ping" => {
                if req.is_notification() {
                    None
                } else {
                    Some(Response::result(req.id.clone(), json!({})).into())
                }
            }
            "tools/list" => {
                if !sess.is_initialized() {
                    return Some(not_initialized(req));
                }
                Some(Response::result(req.id.clone(), json!({ "tools": self.tool_defs() })).into())
            }
            "tools/call" => {
                if !sess.is_initialized() {
                    return Some(not_initialized(req));
                }
                Some(self.call_tool(req, false).await)
            }
            _ => {
                if req.is_notification() {
                    return None;
                }
                Some(err(req, mcp::ERR_METHOD_NOT_FOUND, "Method not found", req.method.clone()))
            }
        }
    }

    /// Handles a request on the modern path regardless of its `_meta`
    /// (the HTTP transport selected the era from headers).
    pub async fn dispatch_modern_forced(&self, req: &Request, sess: &Session) -> Option<Outcome> {
        sess.touch();
        if req.jsonrpc != mcp::JSONRPC_VERSION {
            if req.is_notification() {
                return None;
            }
            return Some(err(req, mcp::ERR_INVALID_REQUEST, "Invalid Request", "jsonrpc must be 2.0"));
        }
        let meta = match RequestMeta::parse(req.params.as_ref()) {
            Ok(m) => m,
            Err(e) => {
                if req.is_notification() {
                    return None;
                }
                return Some(err(req, mcp::ERR_INVALID_PARAMS, "Invalid params", e));
            }
        };
        self.dispatch_modern(req, &meta).await
    }

    fn tool_defs(&self) -> Vec<Value> {
        self.registry.list_tools().iter().map(|t| t.def.clone()).collect()
    }

    /// Validates the modern `_meta` envelope: a supported protocol version
    /// and a declared (possibly empty) client capability set.
    fn check_modern_meta(&self, req: &Request, meta: &RequestMeta) -> Option<Outcome> {
        if meta.protocol_version.is_empty() {
            return Some(err(
                req,
                mcp::ERR_INVALID_PARAMS,
                "Invalid params",
                format!("missing required _meta key {}", mcp::META_PROTOCOL_VERSION),
            ));
        }
        if !mcp::MODERN_PROTOCOL_VERSIONS.contains(&meta.protocol_version.as_str()) {
            let message = if mcp::LEGACY_HANDSHAKE_VERSIONS.contains(&meta.protocol_version.as_str()) {
                format!(
                    "Unsupported protocol version: {} must be negotiated with the initialize handshake",
                    meta.protocol_version
                )
            } else {
                "Unsupported protocol version".to_string()
            };
            return Some(err(
                req,
                mcp::ERR_UNSUPPORTED_PROTOCOL_VERSION,
                message,
                json!({ "supported": mcp::SUPPORTED_PROTOCOL_VERSIONS, "requested": meta.protocol_version }),
            ));
        }
        if meta.client_capabilities.is_none() {
            return Some(err(
                req,
                mcp::ERR_INVALID_PARAMS,
                "Invalid params",
                format!("missing required _meta key {}", mcp::META_CLIENT_CAPABILITIES),
            ));
        }
        None
    }

    async fn dispatch_modern(&self, req: &Request, meta: &RequestMeta) -> Option<Outcome> {
        // server/discover doubles as the version probe: a request without a
        // protocol version is still answered, but an explicit unsupported
        // version gets UnsupportedProtocolVersionError.
        if req.method == "server/discover" {
            if req.is_notification() {
                return None;
            }
            if !meta.protocol_version.is_empty() {
                if let Some(e) = self.check_modern_meta(req, meta) {
                    return Some(e);
                }
            }
            return Some(self.handle_discover(req));
        }
        if let Some(e) = self.check_modern_meta(req, meta) {
            return if req.is_notification() { None } else { Some(e) };
        }
        match req.method.as_str() {
            // Cancellation is transport-specific: stdio intercepts it before
            // dispatch, HTTP clients close the connection.
            "notifications/cancelled" => None,
            "tools/list" => {
                if req.is_notification() {
                    return None;
                }
                if let Some(Value::Object(p)) = &req.params {
                    if p.contains_key("cursor") {
                        // This server never issues cursors for tools/list.
                        return Some(err(req, mcp::ERR_INVALID_PARAMS, "Invalid params", "invalid cursor"));
                    }
                }
                let result = json!({
                    "tools": self.tool_defs(),
                    "resultType": mcp::RESULT_TYPE_COMPLETE,
                    "ttlMs": CACHE_TTL_MS,
                    "cacheScope": mcp::CACHE_SCOPE_PUBLIC,
                    "_meta": server_info_meta(),
                });
                Some(Response::result(req.id.clone(), result).into())
            }
            "tools/call" => {
                if req.is_notification() {
                    return None;
                }
                Some(self.call_tool(req, true).await)
            }
            "subscriptions/listen" => {
                if req.is_notification() {
                    return None;
                }
                Some(self.handle_listen(req))
            }
            // ping and logging/setLevel were removed in 2026-07-28.
            _ => {
                if req.is_notification() {
                    return None;
                }
                Some(err(req, mcp::ERR_METHOD_NOT_FOUND, "Method not found", req.method.clone()))
            }
        }
    }

    /// `subscriptions/listen`: the tool set never changes, so the server
    /// honours no notification types. It acknowledges with an empty filter
    /// and then ends the subscription gracefully with a completion result.
    fn handle_listen(&self, req: &Request) -> Outcome {
        let sub_id = req.id.clone().unwrap_or(Value::Null);
        let ack = json!({
            "jsonrpc": mcp::JSONRPC_VERSION,
            "method": "notifications/subscriptions/acknowledged",
            "params": {
                "_meta": { mcp::META_SUBSCRIPTION_ID: sub_id },
                "notifications": {},
            },
        });
        let mut meta = server_info_meta();
        meta.insert(mcp::META_SUBSCRIPTION_ID.into(), sub_id);
        let result = json!({ "resultType": mcp::RESULT_TYPE_COMPLETE, "_meta": meta });
        Outcome { notifications: vec![ack], response: Response::result(req.id.clone(), result) }
    }

    fn handle_discover(&self, req: &Request) -> Outcome {
        let result = json!({
            "resultType": mcp::RESULT_TYPE_COMPLETE,
            "supportedVersions": mcp::SUPPORTED_PROTOCOL_VERSIONS,
            "capabilities": { "tools": {} },
            "instructions": SERVER_INSTRUCTIONS,
            "ttlMs": CACHE_TTL_MS,
            "cacheScope": mcp::CACHE_SCOPE_PUBLIC,
            "_meta": server_info_meta(),
        });
        Response::result(req.id.clone(), result).into()
    }

    fn handle_initialize(&self, req: &Request, sess: &Session) -> Outcome {
        let mut params = InitializeParams::default();
        if let Some(p) = &req.params {
            if let Err(e) = unmarshal_value(p, &mut params) {
                return err(req, mcp::ERR_INVALID_PARAMS, "Invalid params", e);
            }
        }
        let negotiated = mcp::negotiate_protocol_version(&params.protocol_version);
        sess.initialized.store(true, Ordering::SeqCst);
        let result = json!({
            "protocolVersion": negotiated,
            "capabilities": { "tools": {} },
            "serverInfo": { "name": SERVER_NAME, "title": SERVER_TITLE, "version": SERVER_VERSION },
            "instructions": SERVER_INSTRUCTIONS,
        });
        Response::result(req.id.clone(), result).into()
    }

    /// Executes `tools/call`. `modern` selects the 2026-07-28 result
    /// envelope and error codes.
    async fn call_tool(&self, req: &Request, modern: bool) -> Outcome {
        let mut params = ToolsCallParams::default();
        let parsed = match &req.params {
            None => Err("unexpected end of JSON input".to_string()),
            Some(p) => unmarshal_value(p, &mut params),
        };
        if let Err(e) = parsed {
            return err(req, mcp::ERR_INVALID_PARAMS, "Invalid params", e);
        }
        if modern {
            // CallToolRequest requires a string name and object arguments.
            if let Some(Value::Object(p)) = &req.params {
                if !matches!(p.get("name"), Some(Value::String(_))) {
                    return err(req, mcp::ERR_INVALID_PARAMS, "Invalid params", "params.name must be a string");
                }
                if let Some(a) = p.get("arguments") {
                    if !a.is_object() {
                        return err(req, mcp::ERR_INVALID_PARAMS, "Invalid params", "params.arguments must be an object");
                    }
                }
            }
        }
        let name = params.name.clone();
        let start = Instant::now();
        let call = self.registry.call_tool(&name, params.arguments.0.clone());
        let outcome = match tokio::time::timeout(TOOL_CALL_TIMEOUT, call).await {
            Ok(r) => r,
            Err(_) => Err(CallError::Failed("context deadline exceeded".into())),
        };
        let elapsed = start.elapsed();
        let label = match &outcome {
            Err(CallError::UnknownTool(_)) => "unknown",
            Err(CallError::Failed(_)) => "error",
            Ok(r) if r.is_error => "error",
            Ok(_) => "ok",
        };
        if let Some(m) = &self.metrics {
            m.observe_tool(&name, label, elapsed);
        }
        logging::info(
            "tool_call",
            &[("tool", name.as_str().into()), ("result", label.into()), ("duration", elapsed.into())],
        );
        let result = match outcome {
            Ok(r) => r,
            Err(CallError::UnknownTool(_)) => {
                // 2026-07-28 defines an unknown tool as invalid params; the
                // legacy path keeps its historical -32601.
                let code = if modern { mcp::ERR_INVALID_PARAMS } else { mcp::ERR_METHOD_NOT_FOUND };
                return err(req, code, "Unknown tool", name);
            }
            Err(CallError::Failed(msg)) => CallToolResult::error(msg),
        };
        Response::result(req.id.clone(), stamp_modern(result, modern).to_value()).into()
    }
}

/// The `_meta` map carrying server identity, stamped into modern results.
pub fn server_info_meta() -> Map<String, Value> {
    let mut m = Map::new();
    m.insert(
        mcp::META_SERVER_INFO.into(),
        json!({ "name": SERVER_NAME, "title": SERVER_TITLE, "version": SERVER_VERSION }),
    );
    m
}

fn stamp_modern(mut r: CallToolResult, modern: bool) -> CallToolResult {
    if !modern {
        return r;
    }
    r.result_type = Some(mcp::RESULT_TYPE_COMPLETE);
    let meta = r.meta.get_or_insert_with(Map::new);
    for (k, v) in server_info_meta() {
        meta.entry(k).or_insert(v);
    }
    r
}

fn err(req: &Request, code: i64, message: impl Into<String>, data: impl Into<Value>) -> Outcome {
    Response::error(req.id.clone(), code, message, data).into()
}

fn not_initialized(req: &Request) -> Outcome {
    err(req, mcp::ERR_INVALID_REQUEST, "Not initialized", "initialize must be called first")
}
