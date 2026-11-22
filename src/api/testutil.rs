//! Test support: an in-process mock App Store Connect API.
//!
//! ```ignore
//! let mock = MockApi::start(|req| {
//!     assert_eq!(req.path, "/v1/apps/123/webhooks");
//!     MockResponse::json(200, r#"{"data":[]}"#)
//! })
//! .await;
//! let resp = mock.client().list_webhooks("123", None).await.unwrap();
//! assert_eq!(mock.requests().len(), 1);
//! ```

use std::sync::{Arc, Mutex};

use axum::body::Bytes;
use axum::extract::{Request, State};
use axum::response::{IntoResponse, Response};
use axum::Router;

use super::auth::{parse_private_key, TokenProvider};
use super::Client;

/// A throwaway P-256 key used only by tests.
pub const TEST_PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgig0ZxzLHwty1KYW6
Rp9306m3B/ApS18QixiphSQaHbmhRANCAASZ1FZnsq5MBJZLIUaZ1E2hKP91Vd1f
F0/2CD8o78kLt+HscMinCoX4VzgQesmeeIkuqHorgRmewtK0pYSODykS
-----END PRIVATE KEY-----
";

/// A request received by the mock.
#[derive(Debug, Clone)]
pub struct RecordedRequest {
    /// HTTP method.
    pub method: String,
    /// Path (no query).
    pub path: String,
    /// Raw query string (no `?`).
    pub query: String,
    /// Header name/value pairs (names lowercase).
    pub headers: Vec<(String, String)>,
    /// Body bytes.
    pub body: Vec<u8>,
}

impl RecordedRequest {
    /// The body parsed as JSON (panics if it is not JSON).
    pub fn json(&self) -> serde_json::Value {
        serde_json::from_slice(&self.body).expect("request body is JSON")
    }
    /// A header value by (case-insensitive) name.
    pub fn header(&self, name: &str) -> Option<&str> {
        let name = name.to_ascii_lowercase();
        self.headers.iter().find(|(k, _)| *k == name).map(|(_, v)| v.as_str())
    }
    /// A decoded query parameter (first value), if present.
    pub fn query_param(&self, key: &str) -> Option<String> {
        let v = crate::go::url::Values::parse(&self.query).ok()?;
        v.has(key).then(|| v.get(key).to_string())
    }
}

/// A scripted response.
#[derive(Debug, Clone)]
pub struct MockResponse {
    /// Status code.
    pub status: u16,
    /// Content type (empty for none).
    pub content_type: String,
    /// Body.
    pub body: Vec<u8>,
    /// Extra headers.
    pub headers: Vec<(String, String)>,
}

impl MockResponse {
    /// A JSON response.
    pub fn json(status: u16, body: impl Into<String>) -> Self {
        Self { status, content_type: "application/json".into(), body: body.into().into_bytes(), headers: vec![] }
    }
    /// An empty response (e.g. 204).
    pub fn empty(status: u16) -> Self {
        Self { status, content_type: String::new(), body: vec![], headers: vec![] }
    }
    /// Adds a header.
    pub fn with_header(mut self, k: &str, v: &str) -> Self {
        self.headers.push((k.into(), v.into()));
        self
    }
}

type HandlerFn = dyn Fn(&RecordedRequest) -> MockResponse + Send + Sync;

#[derive(Clone)]
struct MockState {
    handler: Arc<HandlerFn>,
    log: Arc<Mutex<Vec<RecordedRequest>>>,
}

/// A running mock API server.
pub struct MockApi {
    /// Base URL, e.g. `http://127.0.0.1:41234`.
    pub base_url: String,
    log: Arc<Mutex<Vec<RecordedRequest>>>,
    _task: tokio::task::JoinHandle<()>,
}

impl MockApi {
    /// Starts a mock answering every request with `handler`.
    pub async fn start(handler: impl Fn(&RecordedRequest) -> MockResponse + Send + Sync + 'static) -> Self {
        let state = MockState { handler: Arc::new(handler), log: Arc::new(Mutex::new(Vec::new())) };
        let log = state.log.clone();
        let app = Router::new().fallback(serve).with_state(state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("bind mock");
        let addr = listener.local_addr().expect("mock addr");
        let task = tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });
        Self { base_url: format!("http://{addr}"), log, _task: task }
    }

    /// A client pointed at this mock, authenticated with [`TEST_PRIVATE_KEY`].
    pub fn client(&self) -> Client {
        Client::with_token_provider(test_token_provider(), self.base_url.clone())
    }

    /// Requests received so far.
    pub fn requests(&self) -> Vec<RecordedRequest> {
        self.log.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }
}

/// A token provider for tests.
pub fn test_token_provider() -> TokenProvider {
    TokenProvider::from_key("test-issuer", "TESTKEY123", parse_private_key(TEST_PRIVATE_KEY.as_bytes()).expect("test key"))
}

async fn serve(State(st): State<MockState>, req: Request) -> Response {
    let (parts, body) = req.into_parts();
    let body: Bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap_or_default();
    let rec = RecordedRequest {
        method: parts.method.to_string(),
        path: parts.uri.path().to_string(),
        query: parts.uri.query().unwrap_or("").to_string(),
        headers: parts
            .headers
            .iter()
            .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
            .collect(),
        body: body.to_vec(),
    };
    let resp = (st.handler)(&rec);
    st.log.lock().unwrap_or_else(|e| e.into_inner()).push(rec);
    let mut r = (axum::http::StatusCode::from_u16(resp.status).unwrap_or(axum::http::StatusCode::OK), resp.body).into_response();
    if !resp.content_type.is_empty() {
        if let Ok(v) = resp.content_type.parse() {
            r.headers_mut().insert(axum::http::header::CONTENT_TYPE, v);
        }
    }
    for (k, v) in resp.headers {
        if let (Ok(k), Ok(v)) = (k.parse::<axum::http::HeaderName>(), v.parse()) {
            r.headers_mut().insert(k, v);
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn list_apps_round_trip() {
        let mock = MockApi::start(|req| {
            assert_eq!(req.path, "/v1/apps");
            MockResponse::json(200, r#"{"data":[{"type":"apps","id":"1","attributes":{"name":"Demo"}}],"links":{}}"#)
        })
        .await;
        let opts = crate::api::ListOptions { limit: 5, ..Default::default() };
        let resp = mock.client().list_apps(Some(&opts)).await.unwrap();
        assert_eq!(resp.data[0].attributes.name, "Demo");
        let reqs = mock.requests();
        assert_eq!(reqs[0].query, "limit=5");
        assert!(reqs[0].header("authorization").unwrap().starts_with("Bearer "));
    }

    #[tokio::test]
    async fn api_errors_are_structured() {
        let mock = MockApi::start(|_| {
            MockResponse::json(404, r#"{"errors":[{"status":"404","code":"NOT_FOUND","title":"Not Found","detail":"nope"}]}"#)
        })
        .await;
        let err = mock.client().get_app("x").await.unwrap_err();
        assert_eq!(err.to_string(), "API error (404): Not Found: nope");
        assert!(err.api().unwrap().is_not_found());
    }
}
