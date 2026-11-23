//! Per-request context for multi-user deployments.
//!
//! The HTTP transport authenticates each request and resolves the App
//! Store Connect client it must use. That information travels with the
//! request through dispatch and into the tool handler as a task-local
//! [`RequestContext`], so handlers never see another caller's identity or
//! credentials. Requests without a context (stdio, tests) use the
//! registry's own client and allow local file uploads.

use std::path::PathBuf;
use std::sync::Arc;

use crate::api::Client;

tokio::task_local! {
    static CONTEXT: Arc<RequestContext>;
}

/// Who made the request.
#[derive(Debug, Clone, Default)]
pub struct Caller {
    /// Stable identity: the token subject, a static-token fingerprint, or
    /// `None` for unauthenticated transports.
    pub subject: Option<String>,
    /// The OAuth client that obtained the token (`client_id`/`azp`).
    pub client_id: Option<String>,
    /// Granted scopes.
    pub scopes: Vec<String>,
}

/// Where `file_path` upload arguments may read from.
#[derive(Debug, Clone, Default)]
pub enum UploadPolicy {
    /// Any local path (single-user stdio).
    #[default]
    Unrestricted,
    /// Local paths are refused; callers must send `file_data_base64`.
    Disabled,
    /// Only paths inside this (canonical) directory.
    Within(PathBuf),
}

/// Request-scoped state visible to tool handlers.
#[derive(Debug, Clone, Default)]
pub struct RequestContext {
    /// Per-request id (also in access logs).
    pub request_id: String,
    /// W3C trace id from `traceparent`, when the caller sent one.
    pub trace_id: Option<String>,
    /// The authenticated caller.
    pub caller: Caller,
    /// The App Store Connect client for this caller, when it differs from
    /// the server-wide one.
    pub client: Option<Arc<Client>>,
    /// Why no client is available (e.g. missing per-request credentials);
    /// reported to the caller as a tool error.
    pub client_error: Option<String>,
    /// Upload source restrictions.
    pub uploads: UploadPolicy,
}

impl RequestContext {
    /// Runs `fut` with this context installed.
    pub async fn scope<F: Future>(self: Arc<Self>, fut: F) -> F::Output {
        CONTEXT.scope(self, fut).await
    }

    /// The context of the current request, if any.
    pub fn current() -> Option<Arc<RequestContext>> {
        CONTEXT.try_with(Arc::clone).ok()
    }
}

/// Extracts the trace id from a W3C `traceparent` value
/// (`version-traceid-parentid-flags`).
pub fn trace_id_from_traceparent(v: &str) -> Option<String> {
    let mut parts = v.trim().split('-');
    let (_version, trace) = (parts.next()?, parts.next()?);
    let valid = trace.len() == 32
        && trace.bytes().all(|b| b.is_ascii_hexdigit())
        && trace.bytes().any(|b| b != b'0');
    valid.then(|| trace.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traceparent_parsing() {
        assert_eq!(
            trace_id_from_traceparent("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01")
                .as_deref(),
            Some("4bf92f3577b34da6a3ce929d0e0e4736")
        );
        assert_eq!(
            trace_id_from_traceparent("00-00000000000000000000000000000000-00f067aa0ba902b7-01"),
            None
        );
        assert_eq!(trace_id_from_traceparent("garbage"), None);
    }

    #[tokio::test]
    async fn context_is_scoped() {
        assert!(RequestContext::current().is_none());
        let ctx = Arc::new(RequestContext {
            request_id: "r1".into(),
            ..Default::default()
        });
        let id = ctx
            .scope(async { RequestContext::current().map(|c| c.request_id.clone()) })
            .await;
        assert_eq!(id.as_deref(), Some("r1"));
    }
}
