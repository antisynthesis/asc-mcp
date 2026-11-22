//! MCP tool implementations for App Store Connect.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;

use crate::api::Client;
use crate::mcp::{CallToolResult, Tool};

mod all;
pub mod pagination;
include!("modules.rs");

/// Raw tool arguments: Go's `json.RawMessage` (`None` when the request had
/// no `arguments` member).
pub type Args = Option<Value>;

/// A handler outcome. `Err` carries a message that is reported to the
/// client as an `isError` tool result (Go handlers' `return nil, err`).
pub type HandlerResult = Result<CallToolResult, String>;

/// A boxed handler future.
pub type HandlerFuture = Pin<Box<dyn Future<Output = HandlerResult> + Send>>;

/// A tool handler.
pub type Handler = Arc<dyn Fn(Arc<Client>, Args) -> HandlerFuture + Send + Sync>;

/// Error returned by [`Registry::call_tool`] for unregistered names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallError {
    /// No tool with that name is registered.
    UnknownTool(String),
    /// The tool ran and failed (reported as an `isError` result).
    Failed(String),
}

/// Tool definitions and their handlers, in registration order.
pub struct Registry {
    client: Option<Arc<Client>>,
    tools: Vec<Tool>,
    handlers: HashMap<String, Handler>,
}

impl Registry {
    /// Creates a registry with every built-in tool. `client` may be `None`
    /// when the registry is only used to list tools.
    pub fn new(client: Option<Arc<Client>>) -> Self {
        let mut r = Self { client, tools: Vec::new(), handlers: HashMap::new() };
        all::register_all(&mut r);
        r
    }

    /// Creates a registry with no tools (tests).
    pub fn empty(client: Option<Arc<Client>>) -> Self {
        Self { client, tools: Vec::new(), handlers: HashMap::new() }
    }

    /// All tool definitions, in deterministic registration order.
    pub fn list_tools(&self) -> &[Tool] {
        &self.tools
    }

    /// Looks up a tool definition.
    pub fn get(&self, name: &str) -> Option<&Tool> {
        self.tools.iter().find(|t| t.name == name)
    }

    /// Executes a tool by name.
    pub async fn call_tool(&self, name: &str, args: Args) -> Result<CallToolResult, CallError> {
        let Some(handler) = self.handlers.get(name) else {
            return Err(CallError::UnknownTool(format!("unknown tool: {name}")));
        };
        let Some(client) = self.client.clone() else {
            return Err(CallError::Failed("no App Store Connect client configured".into()));
        };
        // Run on its own task so a panicking handler is reported as a tool
        // error instead of tearing down the connection.
        match tokio::spawn(handler(client, args)).await {
            Ok(r) => r.map_err(CallError::Failed),
            Err(e) if e.is_panic() => Err(CallError::Failed("internal error: tool handler panicked".into())),
            Err(_) => Err(CallError::Failed("context canceled".into())),
        }
    }

    /// Adds a tool beyond the built-in set (tests inject fakes this way).
    pub fn register(&mut self, tool: Tool, handler: Handler) {
        self.handlers.insert(tool.name.clone(), handler);
        self.tools.push(tool);
    }
}

/// Parses a compiled-in tool definition.
pub(crate) fn tool(json: &str) -> Tool {
    Tool::from_json(json)
}

/// Wraps an `async fn(Arc<Client>, Args) -> HandlerResult` as a [`Handler`].
#[macro_export]
#[doc(hidden)]
macro_rules! handler {
    ($f:path) => {
        ::std::sync::Arc::new(|c, a| ::std::boxed::Box::pin($f(c, a)) as $crate::tools::HandlerFuture)
            as $crate::tools::Handler
    };
}
pub(crate) use handler;

/// Everything tool modules need.
#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use std::fmt::Write as _;
    pub(crate) use std::sync::Arc;

    pub(crate) use serde_json::{json, Value};

    pub(crate) use super::pagination::*;
    pub(crate) use super::{handler, tool, Args, HandlerResult, Registry};
    pub(crate) use crate::api::model::*;
    pub(crate) use crate::api::{self, sha256_hex, Client, ListOptions};
    pub(crate) use crate::go::fmt::{bool_v, float_prec, float_v, quote, slice_v};
    pub(crate) use crate::go::json::{self as gojson, unmarshal_args};
    pub(crate) use crate::go::url::Values;
    pub(crate) use crate::go::{GoBytes, GoDecode, GoEncode, GoMap, GoSlice, GoTime, RawMessage};
    pub(crate) use crate::go_struct;
    pub(crate) use crate::mcp::CallToolResult;

    // Types shared across tool files (one Go package).
    pub(crate) use super::commerce_versions::CommerceListParams;
    pub(crate) use super::gamecenter_content::GcListParams;
}
