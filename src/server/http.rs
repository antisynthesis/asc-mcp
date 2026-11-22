//! The Streamable HTTP transport.
//!
//! Dual-era on one endpoint (`/mcp`):
//!
//! * **2026-07-28** requests — identified by a `_meta` protocol version in
//!   the body, a modern `MCP-Protocol-Version` header, or the
//!   `server/discover` method — are served statelessly. The mirrored
//!   `Mcp-Method`, `MCP-Protocol-Version` and `Mcp-Name` headers are
//!   validated against the body, and no session ids are minted or read.
//! * **Legacy** (handshake) requests get the session flow: `initialize`
//!   mints an `Mcp-Session-Id`, which every later request must echo.

use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::Router;
use axum::body::{Body, Bytes};
use axum::extract::{ConnectInfo, Request as AxRequest, State};
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, header};
use axum::response::{IntoResponse, Response as AxResponse};
use axum::routing::any;
use base64::Engine;
// hyper panics on the first connection if a header read timeout is set
// without a timer.
use hyper_util::rt::TokioTimer;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

use super::dispatch::{Dispatcher, Outcome, Session, is_modern};
use super::metrics::Metrics;
use crate::logging;
use crate::mcp::{self, Request, RequestMeta, Response};
use crate::tools::Registry;

/// Legacy session id header.
pub const SESSION_HEADER: &str = "Mcp-Session-Id";
/// Protocol version header.
pub const PROTOCOL_VERSION_HEADER: &str = "MCP-Protocol-Version";
/// Mirrors the JSON-RPC method on modern requests.
pub const METHOD_HEADER: &str = "Mcp-Method";
/// Mirrors `params.name` on modern `tools/call` requests.
pub const NAME_HEADER: &str = "Mcp-Name";
/// Largest accepted request body.
pub const MAX_HTTP_BODY_SIZE: usize = 4 * 1024 * 1024;
/// Idle time after which a legacy session is discarded.
pub const SESSION_IDLE_TIMEOUT: Duration = Duration::from_secs(30 * 60);

/// Transport options.
#[derive(Debug, Clone, Default)]
pub struct HttpOptions {
    /// Extra allowed `Origin` values. Loopback origins and the server's own
    /// origin are always allowed; requests without `Origin` are allowed.
    pub allowed_origins: Vec<String>,
    /// Accepted Bearer tokens; empty leaves `/mcp` open.
    pub auth_tokens: Vec<String>,
}

struct Inner {
    dispatcher: Dispatcher,
    sessions: Mutex<HashMap<String, Arc<Session>>>,
    allowed_origins: Vec<String>,
    auth_tokens: Vec<[u8; 32]>,
    metrics: Arc<Metrics>,
}

/// The HTTP server.
#[derive(Clone)]
pub struct HttpServer {
    inner: Arc<Inner>,
}

impl HttpServer {
    /// Creates the transport over `registry`.
    pub fn new(registry: Arc<Registry>, opts: HttpOptions) -> Self {
        let metrics = Arc::new(Metrics::new());
        let tokens = opts
            .auth_tokens
            .iter()
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .map(|t| Sha256::digest(t.as_bytes()).into())
            .collect();
        let inner = Inner {
            dispatcher: Dispatcher::new(registry).with_metrics(metrics.clone()),
            sessions: Mutex::new(HashMap::new()),
            allowed_origins: opts
                .allowed_origins
                .iter()
                .map(|o| o.to_ascii_lowercase())
                .collect(),
            auth_tokens: tokens,
            metrics,
        };
        Self {
            inner: Arc::new(inner),
        }
    }

    /// The metrics surface.
    pub fn metrics(&self) -> &Arc<Metrics> {
        &self.inner.metrics
    }

    /// Number of live legacy sessions.
    pub fn session_count(&self) -> usize {
        self.inner
            .sessions
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .len()
    }

    /// The router: `/mcp`, `/healthz`, `/metrics`.
    pub fn router(&self) -> Router {
        Router::new()
            .route("/mcp", any(handle_mcp))
            .route("/healthz", any(|| async { "ok" }))
            .route("/metrics", any(handle_metrics))
            .fallback(|| async {
                (
                    StatusCode::NOT_FOUND,
                    plain_headers(),
                    "404 page not found\n",
                )
            })
            .with_state(self.clone())
    }

    /// Discards legacy sessions idle longer than `max_idle`.
    pub fn reap_idle(&self, max_idle: Duration) -> usize {
        let now = Instant::now();
        let mut sessions = self
            .inner
            .sessions
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let before = sessions.len();
        sessions.retain(|_, s| now.duration_since(s.last_active()) < max_idle);
        let reaped = before - sessions.len();
        if reaped > 0 {
            self.inner.metrics.sessions_reaped.inc_by(reaped as f64);
            self.inner
                .metrics
                .sessions_active
                .set(sessions.len() as f64);
        }
        reaped
    }

    /// Serves on `addr` until `shutdown` resolves. TLS (1.2+) is used when
    /// both certificate and key are given; giving only one is an error.
    pub async fn listen_and_serve(
        &self,
        addr: &str,
        tls_cert: &str,
        tls_key: &str,
        shutdown: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), String> {
        let use_tls = validate_tls_pair(tls_cert, tls_key)?;
        let addr = resolve_addr(addr)?;
        let reaper = self.clone();
        tokio::spawn(async move {
            let mut t = tokio::time::interval(Duration::from_secs(5 * 60));
            t.tick().await;
            loop {
                t.tick().await;
                reaper.reap_idle(SESSION_IDLE_TIMEOUT);
            }
        });
        let handle = axum_server::Handle::new();
        let h2 = handle.clone();
        tokio::spawn(async move {
            shutdown.await;
            h2.graceful_shutdown(Some(Duration::from_secs(10)));
        });
        let app = self
            .router()
            .into_make_service_with_connect_info::<SocketAddr>();
        let result = if use_tls {
            let _ = rustls::crypto::ring::default_provider().install_default();
            let cfg = axum_server::tls_rustls::RustlsConfig::from_pem_file(tls_cert, tls_key)
                .await
                .map_err(|e| format!("tls config: {e}"))?;
            let mut srv = axum_server::bind_rustls(addr, cfg).handle(handle);
            srv.http_builder()
                .http1()
                .timer(TokioTimer::new())
                .header_read_timeout(Duration::from_secs(10));
            srv.serve(app).await
        } else {
            let mut srv = axum_server::bind(addr).handle(handle);
            srv.http_builder()
                .http1()
                .timer(TokioTimer::new())
                .header_read_timeout(Duration::from_secs(10));
            srv.serve(app).await
        };
        result.map_err(|e| e.to_string())
    }
}

fn resolve_addr(addr: &str) -> Result<SocketAddr, String> {
    // Accept ":8080" (no host) as all interfaces.
    let a = if let Some(port) = addr.strip_prefix(':') {
        format!("0.0.0.0:{port}")
    } else {
        addr.to_string()
    };
    use std::net::ToSocketAddrs;
    a.to_socket_addrs()
        .map_err(|e| format!("listen tcp {addr}: {e}"))?
        .next()
        .ok_or_else(|| format!("listen tcp {addr}: no address"))
}

/// Requires both or neither of the TLS files (no silent downgrade).
pub fn validate_tls_pair(cert: &str, key: &str) -> Result<bool, String> {
    match (cert.is_empty(), key.is_empty()) {
        (true, true) => return Ok(false),
        (true, false) | (false, true) => {
            return Err(
                "tls config invalid: both --tls-cert and --tls-key must be set, or neither".into(),
            );
        }
        _ => {}
    }
    std::fs::metadata(cert).map_err(|e| {
        format!(
            "tls cert: {}",
            crate::wire::os::path_error("stat", cert, &e)
        )
    })?;
    std::fs::metadata(key)
        .map_err(|e| format!("tls key: {}", crate::wire::os::path_error("stat", key, &e)))?;
    Ok(true)
}

fn plain_headers() -> [(header::HeaderName, &'static str); 2] {
    [
        (header::CONTENT_TYPE, "text/plain; charset=utf-8"),
        (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
    ]
}

/// A plain-text error response with a trailing newline.
fn http_error(status: StatusCode, msg: &str) -> AxResponse {
    (status, plain_headers(), format!("{msg}\n")).into_response()
}

fn json_response(status: StatusCode, resp: &Response) -> AxResponse {
    (
        status,
        [(header::CONTENT_TYPE, "application/json")],
        resp.to_bytes(),
    )
        .into_response()
}

async fn handle_metrics(State(srv): State<HttpServer>) -> AxResponse {
    let (ct, body) = srv.inner.metrics.render();
    ([(header::CONTENT_TYPE, ct)], body).into_response()
}

fn new_hex_id(bytes: usize) -> String {
    use rand::RngCore;
    let mut b = vec![0u8; bytes];
    rand::thread_rng().fill_bytes(&mut b);
    hex::encode(b)
}

/// Logging and metrics around the MCP endpoint.
async fn handle_mcp(State(srv): State<HttpServer>, req: AxRequest) -> AxResponse {
    let start = Instant::now();
    let request_id = new_hex_id(8);
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let session_id = header_str(req.headers(), SESSION_HEADER).to_string();
    let remote = req
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|c| c.0.to_string())
        .unwrap_or_default();
    let resp = serve_mcp(&srv, req).await;
    let elapsed = start.elapsed();
    let status = resp.status().as_u16();
    let m = &srv.inner.metrics;
    m.http_requests
        .with_label_values(&[method.as_str(), &status.to_string()])
        .inc();
    m.http_duration
        .with_label_values(&[method.as_str()])
        .observe(elapsed.as_secs_f64());
    logging::info(
        "http_request",
        &[
            ("request_id", request_id.into()),
            ("method", method.as_str().into()),
            ("path", path.into()),
            ("session_id", session_id.into()),
            ("status", status.into()),
            ("duration", elapsed.into()),
            ("remote", remote.into()),
        ],
    );
    resp
}

fn header_str<'a>(h: &'a HeaderMap, name: &str) -> &'a str {
    h.get(name).and_then(|v| v.to_str().ok()).unwrap_or("")
}

async fn serve_mcp(srv: &HttpServer, req: AxRequest) -> AxResponse {
    if !check_origin(&srv.inner, req.headers()) {
        return http_error(StatusCode::FORBIDDEN, "origin not allowed");
    }
    if !check_auth(&srv.inner, req.headers()) {
        srv.inner.metrics.auth_failures.inc();
        let mut r = http_error(StatusCode::UNAUTHORIZED, "unauthorized");
        r.headers_mut().insert(
            header::WWW_AUTHENTICATE,
            HeaderValue::from_static("Bearer realm=\"asc-mcp\""),
        );
        return r;
    }
    match *req.method() {
        Method::POST => handle_post(srv, req).await,
        Method::DELETE => handle_delete(srv, req.headers()),
        Method::OPTIONS => {
            let mut r = StatusCode::NO_CONTENT.into_response();
            r.headers_mut().insert(
                header::ALLOW,
                HeaderValue::from_static("POST, DELETE, OPTIONS"),
            );
            r
        }
        Method::GET => {
            // No server-initiated stream is offered (2026-07-28 removed it).
            let mut r = http_error(StatusCode::METHOD_NOT_ALLOWED, "method not allowed");
            r.headers_mut()
                .insert(header::ALLOW, HeaderValue::from_static("POST, DELETE"));
            r
        }
        _ => {
            let mut r = http_error(StatusCode::METHOD_NOT_ALLOWED, "method not allowed");
            r.headers_mut().insert(
                header::ALLOW,
                HeaderValue::from_static("POST, DELETE, OPTIONS"),
            );
            r
        }
    }
}

/// Origin validation (DNS-rebinding defence). A present Origin must be a
/// loopback origin, the server's own origin (matching `Host`), or listed
/// in `--allowed-origins`; otherwise the request is rejected with 403.
fn check_origin(inner: &Inner, h: &HeaderMap) -> bool {
    let origin = header_str(h, "Origin").to_ascii_lowercase();
    if origin.is_empty() {
        return true;
    }
    if inner.allowed_origins.contains(&origin) {
        return true;
    }
    let Some(rest) = origin
        .strip_prefix("http://")
        .or_else(|| origin.strip_prefix("https://"))
    else {
        return false;
    };
    let host = header_str(h, "Host").to_ascii_lowercase();
    if !host.is_empty() && rest == host {
        return true;
    }
    let hostname = if rest.starts_with('[') {
        rest.split(']')
            .next()
            .map(|s| format!("{s}]"))
            .unwrap_or_default()
    } else {
        rest.split(':').next().unwrap_or("").to_string()
    };
    matches!(hostname.as_str(), "localhost" | "127.0.0.1" | "[::1]")
}

/// Bearer-token check: SHA-256 digests compared in constant time.
fn check_auth(inner: &Inner, h: &HeaderMap) -> bool {
    if inner.auth_tokens.is_empty() {
        return true;
    }
    let Some(tok) = header_str(h, "Authorization").strip_prefix("Bearer ") else {
        return false;
    };
    let presented: [u8; 32] = Sha256::digest(tok.trim().as_bytes()).into();
    let mut ok = subtle::Choice::from(0);
    for d in &inner.auth_tokens {
        ok |= presented.ct_eq(d);
    }
    ok.into()
}

async fn handle_post(srv: &HttpServer, req: AxRequest) -> AxResponse {
    let headers = req.headers().clone();
    let body = match read_body_limited(req.into_body(), MAX_HTTP_BODY_SIZE + 1).await {
        Ok(b) => b,
        Err(_) => return http_error(StatusCode::BAD_REQUEST, "failed to read request body"),
    };
    if body.len() > MAX_HTTP_BODY_SIZE {
        return http_error(StatusCode::PAYLOAD_TOO_LARGE, "request body too large");
    }
    let req = match crate::wire::json::parse(&body).and_then(|v| Request::from_value(&v)) {
        Ok(r) => r,
        Err(e) => {
            return json_response(
                StatusCode::BAD_REQUEST,
                &Response::error(None, mcp::ERR_PARSE, "Parse error", e),
            );
        }
    };
    // Era detection. A malformed _meta cannot name a version, so the
    // header and method decide; initialize is always legacy.
    let meta = RequestMeta::parse(req.params.as_ref()).unwrap_or_default();
    let header_pv = header_str(&headers, PROTOCOL_VERSION_HEADER);
    let modern = is_modern(&req.method, &meta)
        || (req.method != "initialize" && mcp::MODERN_PROTOCOL_VERSIONS.contains(&header_pv));
    if modern {
        return handle_modern_post(srv, &headers, req, &meta).await;
    }

    let inner = &srv.inner;
    let is_init = req.method == "initialize";
    let (sess_id, sess) = if is_init {
        let id = new_hex_id(16);
        let sess = Arc::new(Session::new());
        let active = {
            let mut s = inner.sessions.lock().unwrap_or_else(|e| e.into_inner());
            s.insert(id.clone(), sess.clone());
            s.len()
        };
        inner.metrics.sessions_created.inc();
        inner.metrics.sessions_active.set(active as f64);
        (id, sess)
    } else {
        let id = header_str(&headers, SESSION_HEADER).to_string();
        if id.is_empty() {
            return http_error(
                StatusCode::BAD_REQUEST,
                &format!("missing {SESSION_HEADER} header"),
            );
        }
        let sess = inner
            .sessions
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&id)
            .cloned();
        let Some(sess) = sess else {
            return http_error(StatusCode::NOT_FOUND, "unknown session");
        };
        if !header_pv.is_empty() && !mcp::SUPPORTED_PROTOCOL_VERSIONS.contains(&header_pv) {
            return http_error(
                StatusCode::BAD_REQUEST,
                &format!("unsupported {PROTOCOL_VERSION_HEADER}"),
            );
        }
        (id, sess)
    };

    let Some(out) = inner.dispatcher.dispatch(&req, &sess).await else {
        if is_init {
            inner
                .sessions
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .remove(&sess_id);
        }
        return StatusCode::ACCEPTED.into_response();
    };
    let mut r = json_response(StatusCode::OK, &out.response);
    if is_init {
        if let Ok(v) = HeaderValue::from_str(&sess_id) {
            r.headers_mut().insert(SESSION_HEADER, v);
        }
    }
    r.headers_mut().insert(
        PROTOCOL_VERSION_HEADER,
        HeaderValue::from_static(mcp::PROTOCOL_VERSION),
    );
    r
}

/// Reads at most `limit` bytes and stops there, so an oversized body is
/// truncated (and then rejected with 413 by the caller) instead of failing
/// as a read error.
/// (`axum::body::to_bytes` errors once the limit is exceeded, which turned
/// oversized bodies into 400s.)
async fn read_body_limited(body: Body, limit: usize) -> Result<Vec<u8>, axum::Error> {
    use futures::StreamExt;
    let mut stream = body.into_data_stream();
    let mut out = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let take = chunk.len().min(limit - out.len());
        out.extend_from_slice(&chunk[..take]);
        if out.len() >= limit {
            break;
        }
    }
    Ok(out)
}

/// Decodes the `=?base64?...?=` sentinel permitted on `Mcp-Name` and
/// `Mcp-Param-*` (standard padded base64 of UTF-8; case-sensitive markers).
/// Returns `None` when a sentinel-wrapped value is not valid base64 UTF-8.
pub fn decode_header_value(v: &str) -> Option<String> {
    const PREFIX: &str = "=?base64?";
    const SUFFIX: &str = "?=";
    if v.len() >= PREFIX.len() + SUFFIX.len() && v.starts_with(PREFIX) && v.ends_with(SUFFIX) {
        let inner = &v[PREFIX.len()..v.len() - SUFFIX.len()];
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(inner)
            .ok()?;
        return String::from_utf8(bytes).ok();
    }
    Some(v.to_string())
}

fn mismatch(req: &Request, header: &str, detail: String) -> AxResponse {
    let resp = Response::error(
        req.id.clone(),
        mcp::ERR_HEADER_MISMATCH,
        format!("Header mismatch: {detail}"),
        header,
    );
    modern_json(StatusCode::BAD_REQUEST, &resp)
}

fn modern_json(status: StatusCode, resp: &Response) -> AxResponse {
    let mut r = json_response(status, resp);
    r.headers_mut().insert(
        PROTOCOL_VERSION_HEADER,
        HeaderValue::from_static(mcp::LATEST_PROTOCOL_VERSION),
    );
    r
}

/// Serves a 2026-07-28 POST. Sessions do not exist in this revision: any
/// `Mcp-Session-Id` is ignored and none is minted.
async fn handle_modern_post(
    srv: &HttpServer,
    headers: &HeaderMap,
    req: Request,
    meta: &RequestMeta,
) -> AxResponse {
    // Mcp-Method must mirror the body byte-for-byte (no decoding).
    let hm = headers.get(METHOD_HEADER).map(|v| v.as_bytes());
    if hm != Some(req.method.as_bytes()) {
        let got = hm
            .map(|b| String::from_utf8_lossy(b).into_owned())
            .unwrap_or_default();
        return mismatch(
            &req,
            METHOD_HEADER,
            format!(
                "{METHOD_HEADER} header value '{got}' does not match body value '{}'",
                req.method
            ),
        );
    }
    // MCP-Protocol-Version must mirror _meta. A body without a version is
    // left to the dispatcher, which rejects it as invalid params (except
    // for the server/discover probe).
    if !meta.protocol_version.is_empty() {
        let hp = headers.get(PROTOCOL_VERSION_HEADER).map(|v| v.as_bytes());
        if hp != Some(meta.protocol_version.as_bytes()) {
            let got = hp
                .map(|b| String::from_utf8_lossy(b).into_owned())
                .unwrap_or_default();
            return mismatch(
                &req,
                PROTOCOL_VERSION_HEADER,
                format!(
                    "{PROTOCOL_VERSION_HEADER} header value '{got}' does not match body value '{}'",
                    meta.protocol_version
                ),
            );
        }
    }
    // Mcp-Name is required on tools/call and must mirror params.name.
    if req.method == "tools/call" {
        let name = req
            .params
            .as_ref()
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("");
        let raw = headers.get(NAME_HEADER).and_then(|v| v.to_str().ok());
        let decoded = raw.map(decode_header_value);
        match decoded {
            Some(Some(h)) if !h.is_empty() && h == name => {}
            _ => {
                return mismatch(
                    &req,
                    NAME_HEADER,
                    format!(
                        "{NAME_HEADER} header value '{}' does not match body value '{name}'",
                        raw.unwrap_or("")
                    ),
                );
            }
        }
    }

    let sess = Session::new();
    let out = srv
        .inner
        .dispatcher
        .dispatch_modern_forced(&req, &sess)
        .await;
    let Some(out) = out else {
        let mut r = StatusCode::ACCEPTED.into_response();
        r.headers_mut().insert(
            PROTOCOL_VERSION_HEADER,
            HeaderValue::from_static(mcp::LATEST_PROTOCOL_VERSION),
        );
        return r;
    };
    let status = match &out.response.error {
        Some(e) => status_for_modern_error(e.code),
        None => StatusCode::OK,
    };
    if !out.notifications.is_empty() {
        return sse_response(status, &out);
    }
    modern_json(status, &out.response)
}

/// Streams the outcome's notifications and response as SSE, then closes.
fn sse_response(status: StatusCode, out: &Outcome) -> AxResponse {
    let mut body = String::new();
    for n in &out.notifications {
        body.push_str("event: message\ndata: ");
        body.push_str(&String::from_utf8_lossy(&crate::wire::json::to_json_bytes(
            n,
        )));
        body.push_str("\n\n");
    }
    body.push_str("event: message\ndata: ");
    body.push_str(&String::from_utf8_lossy(&out.response.to_bytes()));
    body.push_str("\n\n");
    let mut r = (status, Body::from(Bytes::from(body))).into_response();
    let h = r.headers_mut();
    h.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/event-stream"),
    );
    h.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    h.insert("X-Accel-Buffering", HeaderValue::from_static("no"));
    h.insert(
        PROTOCOL_VERSION_HEADER,
        HeaderValue::from_static(mcp::LATEST_PROTOCOL_VERSION),
    );
    r
}

/// HTTP status for a modern JSON-RPC error. `isError` tool results are
/// results, not errors, and stay 200.
pub fn status_for_modern_error(code: i64) -> StatusCode {
    match code {
        mcp::ERR_HEADER_MISMATCH
        | mcp::ERR_MISSING_CLIENT_CAPABILITY
        | mcp::ERR_UNSUPPORTED_PROTOCOL_VERSION
        | mcp::ERR_INVALID_PARAMS
        | mcp::ERR_INVALID_REQUEST
        | mcp::ERR_PARSE => StatusCode::BAD_REQUEST,
        mcp::ERR_METHOD_NOT_FOUND => StatusCode::NOT_FOUND,
        _ => StatusCode::OK,
    }
}

fn handle_delete(srv: &HttpServer, h: &HeaderMap) -> AxResponse {
    let id = header_str(h, SESSION_HEADER);
    if id.is_empty() {
        return http_error(
            StatusCode::BAD_REQUEST,
            &format!("missing {SESSION_HEADER} header"),
        );
    }
    let (existed, active) = {
        let mut s = srv.inner.sessions.lock().unwrap_or_else(|e| e.into_inner());
        let existed = s.remove(id).is_some();
        (existed, s.len())
    };
    if !existed {
        return http_error(StatusCode::NOT_FOUND, "unknown session");
    }
    srv.inner.metrics.sessions_active.set(active as f64);
    StatusCode::NO_CONTENT.into_response()
}
