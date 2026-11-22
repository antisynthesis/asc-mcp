//! ES256 JWT generation for App Store Connect API authentication.

use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::Engine;
use p256::ecdsa::{Signature, SigningKey, signature::Signer};
use p256::pkcs8::DecodePrivateKey;

use super::error::{Error, Result};

/// Validity period for generated tokens (App Store Connect allows up to
/// 20 minutes).
pub const TOKEN_DURATION: Duration = Duration::from_secs(15 * 60);

/// How early a cached token is refreshed before it expires.
pub const TOKEN_REFRESH_BUFFER: Duration = Duration::from_secs(2 * 60);

/// Generates and caches App Store Connect API tokens.
pub struct TokenProvider {
    issuer_id: String,
    key_id: String,
    key: SigningKey,
    cache: Mutex<Option<(String, SystemTime)>>,
}

impl std::fmt::Debug for TokenProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenProvider").field("issuer_id", &self.issuer_id).field("key_id", &self.key_id).finish()
    }
}

impl TokenProvider {
    /// Loads the `.p8` key at `private_key_path`.
    pub fn new(issuer_id: &str, key_id: &str, private_key_path: &str) -> Result<Self> {
        let data = std::fs::read(private_key_path).map_err(|e| {
            Error::msg(format!(
                "failed to read private key: {}",
                crate::go::os::path_error("open", private_key_path, &e)
            ))
        })?;
        let key = parse_private_key(&data).map_err(|e| e.wrap("failed to parse private key"))?;
        Ok(Self::from_key(issuer_id, key_id, key))
    }

    /// Builds a provider from an already parsed key.
    pub fn from_key(issuer_id: &str, key_id: &str, key: SigningKey) -> Self {
        Self { issuer_id: issuer_id.into(), key_id: key_id.into(), key, cache: Mutex::new(None) }
    }

    /// Returns a valid token, generating a new one when the cached token is
    /// missing or about to expire.
    pub fn get_token(&self) -> Result<String> {
        let mut cache = self.cache.lock().unwrap_or_else(|e| e.into_inner());
        if let Some((token, expires)) = cache.as_ref() {
            if SystemTime::now() + TOKEN_REFRESH_BUFFER < *expires {
                return Ok(token.clone());
            }
        }
        let (token, expires) = self.generate_token()?;
        *cache = Some((token.clone(), expires));
        Ok(token)
    }

    fn generate_token(&self) -> Result<(String, SystemTime)> {
        let now = SystemTime::now();
        let expires = now + TOKEN_DURATION;
        let secs = |t: SystemTime| t.duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        // Go marshals map[string]... with sorted keys.
        let header = serde_json::json!({"alg": "ES256", "kid": self.key_id, "typ": "JWT"});
        let payload = serde_json::json!({
            "aud": "appstoreconnect-v1",
            "exp": secs(expires),
            "iat": secs(now),
            "iss": self.issuer_id,
        });
        let enc = |v: &serde_json::Value| {
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(crate::go::json::to_go_json_bytes(v))
        };
        let signing_input = format!("{}.{}", enc(&header), enc(&payload));
        let sig: Signature = self.key.sign(signing_input.as_bytes());
        let sig_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(sig.to_bytes());
        Ok((format!("{signing_input}.{sig_b64}"), expires))
    }

    /// Verifies a token's signature with this provider's key (tests).
    pub fn verify_token(&self, token: &str) -> bool {
        use p256::ecdsa::signature::Verifier;
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return false;
        }
        let Ok(sig) = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(parts[2]) else {
            return false;
        };
        let Ok(sig) = Signature::from_slice(&sig) else {
            return false;
        };
        let input = format!("{}.{}", parts[0], parts[1]);
        self.key.verifying_key().verify(input.as_bytes(), &sig).is_ok()
    }
}

/// Parses a PEM-encoded PKCS#8 ECDSA P-256 private key.
pub fn parse_private_key(data: &[u8]) -> Result<SigningKey> {
    let text = String::from_utf8_lossy(data);
    let Some(der) = pem_first_block(&text) else {
        return Err(Error::msg("no PEM block found in private key"));
    };
    match p256::SecretKey::from_pkcs8_der(&der) {
        Ok(k) => Ok(SigningKey::from(k)),
        Err(e) => {
            // Distinguish "valid PKCS#8 but not an EC key" like Go does.
            if let Ok(info) = p256::pkcs8::PrivateKeyInfo::try_from(der.as_slice()) {
                if info.algorithm.oid != p256::pkcs8::ObjectIdentifier::new_unwrap("1.2.840.10045.2.1") {
                    return Err(Error::msg("private key is not ECDSA"));
                }
            }
            Err(Error::msg(format!("failed to parse PKCS8 private key: {e}")))
        }
    }
}

/// Decodes the first PEM block (any label), like Go's `pem.Decode`.
fn pem_first_block(text: &str) -> Option<Vec<u8>> {
    let start = text.find("-----BEGIN ")?;
    let rest = &text[start..];
    let header_end = rest.find("-----\n").or_else(|| rest.find("-----\r\n"))?;
    let label = &rest["-----BEGIN ".len()..header_end];
    let body_start = rest[header_end..].find('\n')? + header_end + 1;
    let end_marker = format!("-----END {label}-----");
    let body_end = rest.find(&end_marker)?;
    let body: String = rest[body_start..body_end].chars().filter(|c| !c.is_whitespace()).collect();
    base64::engine::general_purpose::STANDARD.decode(body).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A throwaway P-256 key generated for these tests.
    pub(crate) const TEST_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgig0ZxzLHwty1KYW6
Rp9306m3B/ApS18QixiphSQaHbmhRANCAASZ1FZnsq5MBJZLIUaZ1E2hKP91Vd1f
F0/2CD8o78kLt+HscMinCoX4VzgQesmeeIkuqHorgRmewtK0pYSODykS
-----END PRIVATE KEY-----
";

    #[test]
    fn token_round_trip() {
        let key = parse_private_key(TEST_KEY.as_bytes()).unwrap();
        let tp = TokenProvider::from_key("issuer", "KEYID", key);
        let t1 = tp.get_token().unwrap();
        assert!(tp.verify_token(&t1));
        assert_eq!(t1, tp.get_token().unwrap(), "token is cached");
        let header = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(t1.split('.').next().unwrap()).unwrap();
        assert_eq!(String::from_utf8(header).unwrap(), r#"{"alg":"ES256","kid":"KEYID","typ":"JWT"}"#);
    }

    #[test]
    fn key_errors() {
        assert_eq!(parse_private_key(b"nope").unwrap_err().to_string(), "no PEM block found in private key");
        let err = TokenProvider::new("i", "k", "/nonexistent/key.p8").unwrap_err();
        assert_eq!(err.to_string(), "failed to read private key: open /nonexistent/key.p8: no such file or directory");
    }
}
