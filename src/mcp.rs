//! Model Context Protocol types.
//!
//! The server speaks the stateless 2026-07-28 revision (selected per
//! request through `_meta`) and the legacy handshake revisions
//! 2024-11-05 through 2025-11-25.

use serde::Serialize;
use serde_json::{Map, Value};

/// JSON-RPC version.
pub const JSONRPC_VERSION: &str = "2.0";

/// The newest revision: stateless, no `initialize` handshake. Clients
/// select it per request via `_meta` and the `MCP-Protocol-Version` header.
pub const LATEST_PROTOCOL_VERSION: &str = "2026-07-28";

/// The preferred legacy handshake version, returned by `initialize` when
/// the client requests a version this server does not speak.
pub const PROTOCOL_VERSION: &str = "2025-11-25";

/// Every revision this server speaks, newest first.
pub const SUPPORTED_PROTOCOL_VERSIONS: &[&str] = &[
    "2026-07-28",
    "2025-11-25",
    "2025-06-18",
    "2025-03-26",
    "2024-11-05",
];

/// Revisions negotiable through `initialize`, newest first.
pub const LEGACY_HANDSHAKE_VERSIONS: &[&str] =
    &["2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05"];

/// Revisions accepted per request through `_meta`.
pub const MODERN_PROTOCOL_VERSIONS: &[&str] = &["2026-07-28"];

/// JSON-RPC parse error.
pub const ERR_PARSE: i64 = -32700;
/// JSON-RPC invalid request.
pub const ERR_INVALID_REQUEST: i64 = -32600;
/// JSON-RPC method not found.
pub const ERR_METHOD_NOT_FOUND: i64 = -32601;
/// JSON-RPC invalid params.
pub const ERR_INVALID_PARAMS: i64 = -32602;
/// JSON-RPC internal error.
pub const ERR_INTERNAL: i64 = -32603;
/// An HTTP header disagrees with the request body (2026-07-28).
pub const ERR_HEADER_MISMATCH: i64 = -32020;
/// The server needs a client capability the client did not declare.
pub const ERR_MISSING_CLIENT_CAPABILITY: i64 = -32021;
/// The requested protocol version is not supported.
pub const ERR_UNSUPPORTED_PROTOCOL_VERSION: i64 = -32022;

/// `_meta` key: protocol revision (required on modern requests).
pub const META_PROTOCOL_VERSION: &str = "io.modelcontextprotocol/protocolVersion";
/// `_meta` key: client capabilities (required on modern requests).
pub const META_CLIENT_CAPABILITIES: &str = "io.modelcontextprotocol/clientCapabilities";
/// `_meta` key: client implementation info.
pub const META_CLIENT_INFO: &str = "io.modelcontextprotocol/clientInfo";
/// `_meta` key: server implementation info, stamped into modern results.
pub const META_SERVER_INFO: &str = "io.modelcontextprotocol/serverInfo";
/// `_meta` key: requested log level.
pub const META_LOG_LEVEL: &str = "io.modelcontextprotocol/logLevel";
/// `_meta` key: subscription identifier.
pub const META_SUBSCRIPTION_ID: &str = "io.modelcontextprotocol/subscriptionId";

/// `resultType` for ordinary results.
pub const RESULT_TYPE_COMPLETE: &str = "complete";

/// `cacheScope` for results identical for every caller.
pub const CACHE_SCOPE_PUBLIC: &str = "public";

/// A JSON-RPC request or notification.
#[derive(Debug, Clone, Default)]
pub struct Request {
    /// The `jsonrpc` member.
    pub jsonrpc: String,
    /// The raw `id` (absent for notifications). Kept as a value so string
    /// and number ids never collide.
    pub id: Option<Value>,
    /// The method name.
    pub method: String,
    /// The raw params.
    pub params: Option<Value>,
}

impl Request {
    /// Parses a request object. Returns an error message for documents
    /// that are not a JSON object or whose members have the wrong types.
    pub fn from_value(v: &Value) -> Result<Self, String> {
        let Value::Object(m) = v else {
            return Err(format!(
                "json: cannot unmarshal {} into Go value of type mcp.Request",
                crate::wire::json::kind(v)
            ));
        };
        let mut r = Request::default();
        for (k, val) in m {
            match k.as_str() {
                "jsonrpc" => match val {
                    Value::String(s) => r.jsonrpc = s.clone(),
                    Value::Null => {}
                    other => return Err(field_type_err(other, "jsonrpc", "string")),
                },
                "method" => match val {
                    Value::String(s) => r.method = s.clone(),
                    Value::Null => {}
                    other => return Err(field_type_err(other, "method", "string")),
                },
                // A JSON null id is a (malformed) request, not a
                // notification: keep it.
                "id" => r.id = Some(val.clone()),
                "params" => {
                    r.params = if val.is_null() {
                        None
                    } else {
                        Some(val.clone())
                    }
                }
                _ => {}
            }
        }
        Ok(r)
    }

    /// Reports whether this is a notification (no `id`).
    pub fn is_notification(&self) -> bool {
        self.id.is_none()
    }

    /// The raw JSON text of the id (the cancellation key).
    pub fn id_key(&self) -> String {
        self.id.as_ref().map(|v| v.to_string()).unwrap_or_default()
    }
}

fn field_type_err(v: &Value, field: &str, ty: &str) -> String {
    format!(
        "json: cannot unmarshal {} into Go struct field Request.{field} of type {ty}",
        crate::wire::json::kind(v)
    )
}

/// A JSON-RPC error object.
#[derive(Debug, Clone, Serialize)]
pub struct RpcError {
    /// Error code.
    pub code: i64,
    /// Short description.
    pub message: String,
    /// Additional data (a string or a structured value).
    #[serde(skip_serializing_if = "Value::is_null")]
    pub data: Value,
}

/// A JSON-RPC response.
#[derive(Debug, Clone, Serialize)]
pub struct Response {
    /// Always `"2.0"`.
    pub jsonrpc: &'static str,
    /// The request id (`null` when it could not be determined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    /// The result, for successes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// The error, for failures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
}

impl Response {
    /// A successful response.
    pub fn result(id: Option<Value>, result: Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION,
            id,
            result: Some(result),
            error: None,
        }
    }

    /// An error response.
    pub fn error(
        id: Option<Value>,
        code: i64,
        message: impl Into<String>,
        data: impl Into<Value>,
    ) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION,
            id,
            result: None,
            error: Some(RpcError {
                code,
                message: message.into(),
                data: data.into(),
            }),
        }
    }

    /// Serializes to compact bytes with HTML-safe escaping.
    pub fn to_bytes(&self) -> Vec<u8> {
        let v = serde_json::to_value(self).unwrap_or(Value::Null);
        crate::wire::json::to_json_bytes(&v)
    }
}

/// Server implementation info.
#[derive(Debug, Clone, Serialize)]
pub struct ServerInfo {
    /// Programmatic name.
    pub name: &'static str,
    /// Display title.
    pub title: &'static str,
    /// Version.
    pub version: &'static str,
}

/// A tool definition. The definition is kept as the exact JSON object the
/// server advertises (field order: name, title, description, inputSchema,
/// outputSchema, annotations, _meta).
#[derive(Debug, Clone)]
pub struct Tool {
    /// The tool's name.
    pub name: String,
    /// The serialized definition.
    pub def: Value,
}

impl Tool {
    /// Parses a definition from JSON text.
    ///
    /// # Panics
    /// Panics if the JSON is malformed or has no string `name`; tool
    /// definitions are compiled into the binary.
    pub fn from_json(json: &str) -> Self {
        let def: Value = serde_json::from_str(json).expect("valid tool definition JSON");
        let name = def["name"]
            .as_str()
            .expect("tool definition has a name")
            .to_string();
        Self { name, def }
    }

    /// The tool's description.
    pub fn description(&self) -> &str {
        self.def["description"].as_str().unwrap_or("")
    }

    /// The tool's input schema.
    pub fn input_schema(&self) -> &Value {
        &self.def["inputSchema"]
    }
}

/// A text content block.
pub fn text_content(text: impl Into<String>) -> Value {
    let mut m = Map::new();
    m.insert("type".into(), Value::String("text".into()));
    let t: String = text.into();
    if !t.is_empty() {
        m.insert("text".into(), Value::String(t));
    }
    Value::Object(m)
}

/// The result of `tools/call`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CallToolResult {
    /// Content blocks.
    pub content: Vec<Value>,
    /// Structured content. `Some(Value::Null)` serializes as an explicit
    /// `null`; `None` omits the member.
    pub structured_content: Option<Value>,
    /// Whether the tool reported an execution error.
    pub is_error: bool,
    /// `resultType` (modern revision only).
    pub result_type: Option<&'static str>,
    /// `_meta`.
    pub meta: Option<Map<String, Value>>,
}

impl CallToolResult {
    /// A successful text result.
    pub fn success(text: impl Into<String>) -> Self {
        Self {
            content: vec![text_content(text)],
            ..Default::default()
        }
    }

    /// An error result: a successful JSON-RPC response with `isError: true`
    /// so the model can self-correct.
    pub fn error(text: impl Into<String>) -> Self {
        Self {
            content: vec![text_content(text)],
            is_error: true,
            ..Default::default()
        }
    }

    /// Returns the first text block (convenient in tests).
    pub fn text(&self) -> &str {
        self.content
            .first()
            .and_then(|c| c["text"].as_str())
            .unwrap_or("")
    }

    /// Serializes in wire field order: `content`, `structuredContent`,
    /// `isError`, `resultType`, `_meta` (optional members omitted when unset).
    pub fn to_value(&self) -> Value {
        let mut m = Map::new();
        m.insert("content".into(), Value::Array(self.content.clone()));
        if let Some(sc) = &self.structured_content {
            m.insert("structuredContent".into(), sc.clone());
        }
        if self.is_error {
            m.insert("isError".into(), Value::Bool(true));
        }
        if let Some(rt) = self.result_type {
            m.insert("resultType".into(), Value::String(rt.into()));
        }
        if let Some(meta) = &self.meta
            && !meta.is_empty()
        {
            m.insert("_meta".into(), Value::Object(meta.clone()));
        }
        Value::Object(m)
    }
}

/// Reserved request metadata from `params._meta`.
#[derive(Debug, Clone, Default)]
pub struct RequestMeta {
    /// `io.modelcontextprotocol/protocolVersion` (empty when absent).
    pub protocol_version: String,
    /// `io.modelcontextprotocol/clientCapabilities` (`None` when absent).
    pub client_capabilities: Option<Map<String, Value>>,
    /// `io.modelcontextprotocol/clientInfo`.
    pub client_info: Option<Value>,
    /// `progressToken`.
    pub progress_token: Option<Value>,
}

impl RequestMeta {
    /// Extracts the reserved `_meta` keys. Returns an error for malformed
    /// values: `_meta` not an object, a non-string protocol version, or
    /// client capabilities that are not an object.
    pub fn parse(params: Option<&Value>) -> Result<Self, String> {
        let mut meta = RequestMeta::default();
        let Some(params) = params else {
            return Ok(meta);
        };
        let Value::Object(p) = params else {
            return Err(format!(
                "json: cannot unmarshal {} into Go value of type struct {{ Meta map[string]json.RawMessage \"json:\\\"_meta\\\"\" }}",
                crate::wire::json::kind(params)
            ));
        };
        let Some(m) = p.get("_meta") else {
            return Ok(meta);
        };
        let m = match m {
            Value::Null => return Ok(meta),
            Value::Object(m) => m,
            other => {
                return Err(format!(
                    "json: cannot unmarshal {} into Go struct field .Meta of type map[string]json.RawMessage",
                    crate::wire::json::kind(other)
                ));
            }
        };
        if let Some(v) = m.get(META_PROTOCOL_VERSION) {
            match v {
                Value::String(s) => meta.protocol_version = s.clone(),
                other => {
                    return Err(format!(
                        "_meta key {META_PROTOCOL_VERSION} must be a string, got {}",
                        crate::wire::json::kind(other)
                    ));
                }
            }
        }
        if let Some(v) = m.get(META_CLIENT_CAPABILITIES) {
            match v {
                Value::Object(c) => meta.client_capabilities = Some(c.clone()),
                other => {
                    return Err(format!(
                        "_meta key {META_CLIENT_CAPABILITIES} must be an object, got {}",
                        crate::wire::json::kind(other)
                    ));
                }
            }
        }
        meta.client_info = m.get(META_CLIENT_INFO).cloned();
        meta.progress_token = m.get("progressToken").cloned();
        Ok(meta)
    }
}

/// Negotiates the handshake version: echoes a legacy version this server
/// knows, else answers the preferred legacy version. 2026-07-28 is never
/// negotiated here because it has no handshake.
pub fn negotiate_protocol_version(requested: &str) -> &'static str {
    LEGACY_HANDSHAKE_VERSIONS
        .iter()
        .find(|v| **v == requested)
        .copied()
        .unwrap_or(PROTOCOL_VERSION)
}
