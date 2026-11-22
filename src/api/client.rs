//! HTTP client for the App Store Connect API.

use std::time::Duration;

use reqwest::Method;

use super::auth::TokenProvider;
use super::error::{ApiError, Error, Result};
use super::model::ErrorResponse;
use crate::wire::JsonEncode;
use crate::wire::url::Values;

/// The App Store Connect API base URL.
pub const BASE_URL: &str = "https://api.appstoreconnect.apple.com";

/// Environment variable that overrides [`BASE_URL`]. Intended for tests and
/// mock servers; the cursor host check follows the override.
pub const BASE_URL_ENV: &str = "ASC_API_BASE_URL";

/// Default per-request timeout.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Upper bound on a response body read into memory.
pub const MAX_RESPONSE_SIZE: usize = 25 * 1024 * 1024;

/// Maximum retries for transient failures (the first attempt is not
/// counted).
pub const MAX_RETRIES: u32 = 3;

/// Starting delay for exponential backoff.
pub const RETRY_BASE_DELAY: Duration = Duration::from_millis(500);

/// Cap on the backoff between retries.
pub const RETRY_MAX_DELAY: Duration = Duration::from_secs(10);

/// User agent sent with every API request.
pub const USER_AGENT: &str = "asc-mcp/1.0.0 (+https://github.com/antisynthesis/asc-mcp)";

/// An authenticated App Store Connect API client.
#[derive(Debug)]
pub struct Client {
    http: reqwest::Client,
    tokens: TokenProvider,
    base_url: String,
}

impl Client {
    /// Creates a client from API key credentials.
    pub fn new(issuer_id: &str, key_id: &str, private_key_path: &str) -> Result<Self> {
        let tokens = TokenProvider::new(issuer_id, key_id, private_key_path)
            .map_err(|e| e.wrap("failed to create token provider"))?;
        let base = std::env::var(BASE_URL_ENV)
            .ok()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| BASE_URL.into());
        Ok(Self::with_token_provider(tokens, base))
    }

    /// Creates a client with an explicit token provider and base URL.
    pub fn with_token_provider(tokens: TokenProvider, base_url: impl Into<String>) -> Self {
        let http = reqwest::Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self {
            http,
            tokens,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    /// The base URL requests are sent to.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Performs an authenticated request, retrying transient failures with
    /// exponential backoff (429, 5xx, and transport errors on idempotent
    /// methods).
    pub async fn do_request(
        &self,
        method: Method,
        path: &str,
        query: Option<&Values>,
        body: Option<Vec<u8>>,
    ) -> Result<Vec<u8>> {
        let mut url = format!("{}{}", self.base_url, path);
        if let Some(q) = query.filter(|q| !q.is_empty()) {
            url.push('?');
            url.push_str(&q.encode());
        }
        let mut last_err: Option<Error> = None;
        for attempt in 0..=MAX_RETRIES {
            if attempt > 0 {
                tokio::time::sleep(backoff_delay(attempt, last_err.as_ref())).await;
            }
            match self.attempt(&method, &url, body.as_deref()).await {
                Ok(b) => return Ok(b),
                Err(e) => {
                    let retry = is_retryable(&e, &method);
                    if !retry {
                        return Err(e);
                    }
                    last_err = Some(e);
                }
            }
        }
        Err(last_err.unwrap_or_else(|| Error::msg("request failed")))
    }

    async fn attempt(&self, method: &Method, url: &str, body: Option<&[u8]>) -> Result<Vec<u8>> {
        let token = self
            .tokens
            .get_token()
            .map_err(|e| e.wrap("failed to get token"))?;
        let mut req = self
            .http
            .request(method.clone(), url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("User-Agent", USER_AGENT);
        if let Some(b) = body {
            req = req.body(b.to_vec());
        }
        let resp = req
            .send()
            .await
            .map_err(|e| Error::network(format!("request failed: {e}")))?;
        let status = resp.status().as_u16();
        let retry_after = resp
            .headers()
            .get("Retry-After")
            .and_then(|v| v.to_str().ok())
            .map(str::to_string);
        let body = read_limited(resp, MAX_RESPONSE_SIZE).await?;
        if status >= 400 {
            let mut api = ApiError {
                status_code: status,
                ..Default::default()
            };
            match crate::wire::json::from_slice::<ErrorResponse>(&body) {
                Ok(er) if !er.errors.is_empty() => api.errors = er.errors.into_vec(),
                _ => api.raw_body = String::from_utf8_lossy(&body).into_owned(),
            }
            if status == 429 {
                if let Some(ra) = retry_after {
                    api.retry_after = Some(parse_retry_after(&ra));
                }
            }
            return Err(api.into());
        }
        Ok(body)
    }

    /// GET `path` with an optional query.
    pub async fn get(&self, path: &str, query: Option<&Values>) -> Result<Vec<u8>> {
        self.do_request(Method::GET, path, query, None).await
    }

    /// POST a JSON body.
    pub async fn post<B: JsonEncode + ?Sized>(
        &self,
        path: &str,
        body: Option<&B>,
    ) -> Result<Vec<u8>> {
        self.do_request(
            Method::POST,
            path,
            None,
            body.map(crate::wire::json::marshal),
        )
        .await
    }

    /// PATCH a JSON body.
    pub async fn patch<B: JsonEncode + ?Sized>(
        &self,
        path: &str,
        body: Option<&B>,
    ) -> Result<Vec<u8>> {
        self.do_request(
            Method::PATCH,
            path,
            None,
            body.map(crate::wire::json::marshal),
        )
        .await
    }

    /// DELETE `path`.
    pub async fn delete(&self, path: &str) -> Result<()> {
        self.do_request(Method::DELETE, path, None, None)
            .await
            .map(|_| ())
    }

    /// DELETE with a JSON body (relationship removals).
    pub async fn delete_with_body<B: JsonEncode + ?Sized>(
        &self,
        path: &str,
        body: Option<&B>,
    ) -> Result<()> {
        self.do_request(
            Method::DELETE,
            path,
            None,
            body.map(crate::wire::json::marshal),
        )
        .await
        .map(|_| ())
    }

    /// GET a fully qualified URL returned by the API (a pagination
    /// `links.next`). The URL must have the same scheme and host as the
    /// base URL so a tampered response cannot redirect the bearer token.
    pub async fn get_url(&self, full_url: &str) -> Result<Vec<u8>> {
        if full_url.is_empty() {
            return Err(Error::msg("empty URL"));
        }
        let parsed = ParsedUrl::parse(full_url)
            .map_err(|e| Error::msg(format!("invalid cursor URL: {e}")))?;
        let base = ParsedUrl::parse(&self.base_url)
            .map_err(|e| Error::msg(format!("invalid client base URL: {e}")))?;
        if parsed.scheme != base.scheme || parsed.host != base.host {
            return Err(Error::msg(format!(
                "cursor URL host mismatch: got {}, want {}",
                crate::wire::fmt::quote(&parsed.host),
                crate::wire::fmt::quote(&base.host)
            )));
        }
        let path = if parsed.path.is_empty() {
            "/".to_string()
        } else {
            parsed.path.clone()
        };
        match parsed.query {
            Some(q) => {
                let query = Values::parse(&q)
                    .map_err(|e| Error::msg(format!("invalid cursor query: {e}")))?;
                self.do_request(Method::GET, &path, Some(&query), None)
                    .await
            }
            None => self.do_request(Method::GET, &path, None, None).await,
        }
    }
}

async fn read_limited(mut resp: reqwest::Response, limit: usize) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    loop {
        match resp.chunk().await {
            Ok(Some(c)) => {
                out.extend_from_slice(&c);
                if out.len() > limit {
                    return Err(Error::msg(format!(
                        "response exceeds max size of {limit} bytes"
                    )));
                }
            }
            Ok(None) => return Ok(out),
            Err(e) => return Err(Error::network(format!("failed to read response: {e}"))),
        }
    }
}

/// Whether a failed attempt is retried: 429 and 5xx always; other non-API
/// failures only for idempotent methods.
fn is_retryable(err: &Error, method: &Method) -> bool {
    if let Some(api) = err.api() {
        return api.status_code == 429 || api.status_code >= 500;
    }
    matches!(
        *method,
        Method::GET | Method::DELETE | Method::PATCH | Method::PUT
    )
}

/// Delay before retry `attempt`, honouring a positive `Retry-After`.
fn backoff_delay(attempt: u32, last: Option<&Error>) -> Duration {
    if let Some(ra) = last.and_then(|e| e.api()).and_then(|a| a.retry_after) {
        if !ra.is_zero() {
            return ra.min(RETRY_MAX_DELAY);
        }
    }
    let d = RETRY_BASE_DELAY.saturating_mul(1 << (attempt - 1));
    d.min(RETRY_MAX_DELAY)
}

/// Parses `Retry-After` as delta-seconds or an HTTP date.
fn parse_retry_after(v: &str) -> Duration {
    if let Ok(secs) = v.trim().parse::<i64>() {
        return if secs > 0 {
            Duration::from_secs(secs as u64)
        } else {
            Duration::ZERO
        };
    }
    let fmt = time::format_description::well_known::Rfc2822;
    if let Ok(t) = time::OffsetDateTime::parse(v.trim(), &fmt) {
        let now = time::OffsetDateTime::now_utc();
        if t > now {
            return Duration::from_secs((t - now).whole_seconds().max(0) as u64);
        }
    }
    Duration::ZERO
}

/// A minimal URL split (scheme, host, path, query) used by the cursor check.
struct ParsedUrl {
    scheme: String,
    host: String,
    path: String,
    query: Option<String>,
}

impl ParsedUrl {
    fn parse(raw: &str) -> std::result::Result<Self, String> {
        let raw = raw.split('#').next().unwrap_or("");
        let (mut scheme, mut rest) = (String::new(), raw);
        if let Some(i) = raw.find(':') {
            let cand = &raw[..i];
            let valid = cand.chars().enumerate().all(|(j, c)| {
                c.is_ascii_alphabetic()
                    || (j > 0 && (c.is_ascii_digit() || matches!(c, '+' | '-' | '.')))
            });
            if i == 0 {
                return Err(format!(
                    "parse {}: missing protocol scheme",
                    crate::wire::fmt::quote(raw)
                ));
            }
            if valid {
                scheme = cand.to_ascii_lowercase();
                rest = &raw[i + 1..];
            }
        }
        let (rest, query) = match rest.split_once('?') {
            Some((a, q)) => (a, Some(q.to_string())),
            None => (rest, None),
        };
        let (host, path) = if let Some(r) = rest.strip_prefix("//") {
            let end = r.find('/').unwrap_or(r.len());
            let auth = &r[..end];
            let host = auth.rsplit_once('@').map(|(_, h)| h).unwrap_or(auth);
            (host.to_string(), r[end..].to_string())
        } else {
            (String::new(), rest.to_string())
        };
        let query = query.filter(|q| !q.is_empty());
        Ok(Self {
            scheme,
            host,
            path,
            query,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_and_retry_after() {
        assert_eq!(backoff_delay(1, None), Duration::from_millis(500));
        assert_eq!(backoff_delay(3, None), Duration::from_secs(2));
        assert_eq!(backoff_delay(10, None), RETRY_MAX_DELAY);
        assert_eq!(parse_retry_after(" 7 "), Duration::from_secs(7));
        assert_eq!(parse_retry_after("garbage"), Duration::ZERO);
    }

    #[test]
    fn url_parse() {
        let u =
            ParsedUrl::parse("https://api.appstoreconnect.apple.com/v1/apps?cursor=abc&limit=5")
                .unwrap();
        assert_eq!(
            (u.scheme.as_str(), u.host.as_str(), u.path.as_str()),
            ("https", "api.appstoreconnect.apple.com", "/v1/apps")
        );
        assert_eq!(u.query.as_deref(), Some("cursor=abc&limit=5"));
        let u = ParsedUrl::parse("not a url").unwrap();
        assert_eq!((u.scheme.as_str(), u.host.as_str()), ("", ""));
    }
}
