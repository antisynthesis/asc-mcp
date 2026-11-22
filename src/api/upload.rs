//! The byte-transfer step of App Store Connect's three-step asset upload.
//!
//! 1. Reservation: POST the asset with file name and size; the response
//!    lists upload operations.
//! 2. Upload: send each operation's slice of the file to its pre-signed
//!    URL (no App Store Connect bearer token).
//! 3. Commit: PATCH the asset with the SHA-256 checksum and
//!    `uploaded=true`.
//!
//! [`Client::perform_upload_operations`] is step 2; the resource-specific
//! upload helpers compose the three steps.

use std::time::Duration;

use sha2::{Digest, Sha256};

use super::Client;
use super::error::{Error, Result};
use super::model::UploadOperation;

/// Largest asset accepted for upload.
pub const MAX_UPLOAD_SIZE: usize = 600 * 1024 * 1024;

/// Per-chunk timeout for asset uploads.
pub const UPLOAD_HTTP_TIMEOUT: Duration = Duration::from_secs(5 * 60);

impl Client {
    /// Sends each upload operation's chunk to its pre-authorized URL.
    pub async fn perform_upload_operations(
        &self,
        ops: &[UploadOperation],
        body: &[u8],
    ) -> Result<()> {
        if body.len() > MAX_UPLOAD_SIZE {
            return Err(Error::msg(format!(
                "upload body exceeds max size of {MAX_UPLOAD_SIZE} bytes"
            )));
        }
        if ops.is_empty() {
            return Err(Error::msg("no upload operations to perform"));
        }
        let hc = reqwest::Client::builder()
            .timeout(UPLOAD_HTTP_TIMEOUT)
            .build()
            .unwrap_or_default();
        for (i, op) in ops.iter().enumerate() {
            if op.offset < 0 || op.length <= 0 || (op.offset + op.length) as usize > body.len() {
                return Err(Error::msg(format!(
                    "upload op {i} out of bounds: offset={} length={} body={}",
                    op.offset,
                    op.length,
                    body.len()
                )));
            }
            let chunk = body[op.offset as usize..(op.offset + op.length) as usize].to_vec();
            let method = if op.method.is_empty() {
                "PUT"
            } else {
                op.method.as_str()
            };
            let method = reqwest::Method::from_bytes(method.as_bytes()).map_err(|_| {
                Error::msg(format!(
                    "upload op {i}: build request: net/http: invalid method {}",
                    crate::go::fmt::quote(method)
                ))
            })?;
            // Pre-signed storage URLs get our User-Agent plus the operation's
            // headers, each replacing any earlier value (Go's Header.Set).
            let mut req = hc
                .request(method, &op.url)
                .body(chunk)
                .build()
                .map_err(|e| Error::msg(format!("upload op {i}: build request: {e}")))?;
            let headers = req.headers_mut();
            headers.insert(
                reqwest::header::USER_AGENT,
                reqwest::header::HeaderValue::from_static(super::client::USER_AGENT),
            );
            for h in op.request_headers.iter() {
                match (
                    reqwest::header::HeaderName::from_bytes(h.name.as_bytes()),
                    reqwest::header::HeaderValue::from_str(&h.value),
                ) {
                    (Ok(k), Ok(v)) => {
                        headers.insert(k, v);
                    }
                    _ => {
                        return Err(Error::msg(format!(
                            "upload op {i}: net/http: invalid header field {}",
                            crate::go::fmt::quote(&h.name)
                        )));
                    }
                }
            }
            let resp = hc
                .execute(req)
                .await
                .map_err(|e| Error::msg(format!("upload op {i}: {e}")))?;
            let status = resp.status().as_u16();
            let text = resp
                .bytes()
                .await
                .map(|b| b[..b.len().min(64 * 1024)].to_vec())
                .unwrap_or_default();
            if status >= 400 {
                return Err(Error::msg(format!(
                    "upload op {i} failed with {status}: {}",
                    String::from_utf8_lossy(&text)
                )));
            }
        }
        Ok(())
    }
}

/// Lowercase hex SHA-256 of `body` (the commit checksum Apple expects).
pub fn sha256_hex(body: &[u8]) -> String {
    hex::encode(Sha256::digest(body))
}
