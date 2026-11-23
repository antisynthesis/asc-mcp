//! API client errors.

use std::fmt;

use super::model::APIErrorDetail;

/// The structured error returned by the App Store Connect API.
///
/// Callers can inspect the status to
/// distinguish authentication failures, missing resources, validation
/// errors and transient server errors.
#[derive(Debug, Clone, Default)]
pub struct ApiError {
    /// HTTP status code.
    pub status_code: u16,
    /// Parsed `errors` array, when the body carried one.
    pub errors: Vec<APIErrorDetail>,
    /// Raw body, when it was not a JSON:API error document.
    pub raw_body: String,
    /// `Retry-After` hint (seconds) on 429 responses.
    pub(crate) retry_after: Option<std::time::Duration>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.errors.is_empty() {
            if !self.raw_body.is_empty() {
                return write!(f, "API error ({}): {}", self.status_code, self.raw_body);
            }
            return write!(f, "API error ({})", self.status_code);
        }
        let parts: Vec<String> = self
            .errors
            .iter()
            .map(|d| format!("{}: {}", d.title, d.detail))
            .collect();
        write!(f, "API error ({}): {}", self.status_code, parts.join("; "))
    }
}

impl ApiError {
    /// Reports whether the API rejected the request for authentication or
    /// authorization reasons.
    pub fn is_auth(&self) -> bool {
        self.status_code == 401 || self.status_code == 403
    }
    /// Reports whether the requested resource was not found.
    pub fn is_not_found(&self) -> bool {
        self.status_code == 404
    }
    /// Reports whether the API throttled the request.
    pub fn is_rate_limited(&self) -> bool {
        self.status_code == 429
    }
}

/// An error from the API layer: a message (including any wrapping
/// prefixes) plus the underlying [`ApiError`] when one is in the chain.
#[derive(Debug, Clone)]
pub struct Error {
    msg: String,
    api: Option<Box<ApiError>>,
    network: bool,
}

/// Result alias for API calls.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// A plain message error.
    pub fn msg(msg: impl Into<String>) -> Self {
        Self {
            msg: msg.into(),
            api: None,
            network: false,
        }
    }

    /// A network/transport failure (retryable for idempotent methods).
    pub(crate) fn network(msg: impl Into<String>) -> Self {
        Self {
            msg: msg.into(),
            api: None,
            network: true,
        }
    }

    /// Prefixes the message (`<prefix>: <msg>`) and keeps the wrapped API
    /// error reachable.
    pub fn wrap(self, prefix: impl fmt::Display) -> Self {
        Self {
            msg: format!("{prefix}: {}", self.msg),
            api: self.api,
            network: self.network,
        }
    }

    /// The API error in the chain, if any.
    pub fn api(&self) -> Option<&ApiError> {
        self.api.as_deref()
    }

    /// Reports whether this is a transport failure (no HTTP response).
    pub fn is_network(&self) -> bool {
        self.network
    }
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        Self {
            msg: e.to_string(),
            api: Some(Box::new(e)),
            network: false,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)
    }
}

impl std::error::Error for Error {}

/// Decodes a response body, wrapping failures as
/// `failed to unmarshal response: <json error>`.
pub fn decode<T: crate::wire::JsonDecode + Default>(data: &[u8]) -> Result<T> {
    crate::wire::json::from_slice(data)
        .map_err(|e| Error::msg(format!("failed to unmarshal response: {e}")))
}
