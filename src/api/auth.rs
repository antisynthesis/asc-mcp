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
        f.debug_struct("TokenProvider")
            .field("issuer_id", &self.issuer_id)
            .field("key_id", &self.key_id)
            .finish()
    }
}

impl TokenProvider {
    /// Loads the `.p8` key at `private_key_path`.
    pub fn new(issuer_id: &str, key_id: &str, private_key_path: &str) -> Result<Self> {
        let data = std::fs::read(private_key_path).map_err(|e| {
            Error::msg(format!(
                "failed to read private key: {}",
                crate::wire::os::path_error("open", private_key_path, &e)
            ))
        })?;
        let key = parse_private_key(&data).map_err(|e| e.wrap("failed to parse private key"))?;
        Ok(Self::from_key(issuer_id, key_id, key))
    }

    /// Builds a provider from an already parsed key.
    pub fn from_key(issuer_id: &str, key_id: &str, key: SigningKey) -> Self {
        Self {
            issuer_id: issuer_id.into(),
            key_id: key_id.into(),
            key,
            cache: Mutex::new(None),
        }
    }

    /// Returns a valid token, generating a new one when the cached token is
    /// missing or about to expire.
    pub fn get_token(&self) -> Result<String> {
        let mut cache = self.cache.lock().unwrap_or_else(|e| e.into_inner());
        if let Some((token, expires)) = cache.as_ref()
            && SystemTime::now() + TOKEN_REFRESH_BUFFER < *expires
        {
            return Ok(token.clone());
        }
        let (token, expires) = self.generate_token()?;
        *cache = Some((token.clone(), expires));
        Ok(token)
    }

    fn generate_token(&self) -> Result<(String, SystemTime)> {
        let now = SystemTime::now();
        let expires = now + TOKEN_DURATION;
        let secs = |t: SystemTime| {
            t.duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        };
        // Header and claims are encoded with sorted keys.
        let header = serde_json::json!({"alg": "ES256", "kid": self.key_id, "typ": "JWT"});
        let payload = serde_json::json!({
            "aud": "appstoreconnect-v1",
            "exp": secs(expires),
            "iat": secs(now),
            "iss": self.issuer_id,
        });
        let enc = |v: &serde_json::Value| {
            base64::engine::general_purpose::URL_SAFE_NO_PAD
                .encode(crate::wire::json::to_json_bytes(v))
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
        self.key
            .verifying_key()
            .verify(input.as_bytes(), &sig)
            .is_ok()
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
            // Distinguish "valid PKCS#8 but not an EC key" from malformed input.
            if let Ok(info) = p256::pkcs8::PrivateKeyInfo::try_from(der.as_slice())
                && info.algorithm.oid
                    != p256::pkcs8::ObjectIdentifier::new_unwrap("1.2.840.10045.2.1")
            {
                return Err(Error::msg("private key is not ECDSA"));
            }
            Err(Error::msg(format!(
                "failed to parse PKCS8 private key: {e}"
            )))
        }
    }
}

/// Decodes the first PEM block (any label).
fn pem_first_block(text: &str) -> Option<Vec<u8>> {
    let start = text.find("-----BEGIN ")?;
    let rest = &text[start..];
    let header_end = rest.find("-----\n").or_else(|| rest.find("-----\r\n"))?;
    let label = &rest["-----BEGIN ".len()..header_end];
    let body_start = rest[header_end..].find('\n')? + header_end + 1;
    let end_marker = format!("-----END {label}-----");
    let body_end = rest.find(&end_marker)?;
    let body: String = rest[body_start..body_end]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
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
        let header = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(t1.split('.').next().unwrap())
            .unwrap();
        assert_eq!(
            String::from_utf8(header).unwrap(),
            r#"{"alg":"ES256","kid":"KEYID","typ":"JWT"}"#
        );
    }

    /// A fresh P-256 key as PKCS#8 PEM.
    fn generate_test_key() -> String {
        use p256::pkcs8::{EncodePrivateKey, LineEnding};
        let key = p256::SecretKey::random(&mut rand::rngs::OsRng);
        key.to_pkcs8_pem(LineEnding::LF)
            .expect("encode key")
            .to_string()
    }

    fn create_test_key_file(content: &str) -> (tempfile::TempDir, String) {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("test_key.p8");
        std::fs::write(&path, content).expect("write key");
        (dir, path.to_string_lossy().into_owned())
    }

    fn b64(part: &str) -> serde_json::Value {
        let raw = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(part)
            .expect("base64url");
        serde_json::from_slice(&raw).expect("json")
    }

    #[test]
    fn new_token_provider() {
        let (_dir, path) = create_test_key_file(&generate_test_key());
        let tp =
            TokenProvider::new("test-issuer-id", "TESTKEY123", &path).expect("valid configuration");
        assert_eq!(tp.issuer_id, "test-issuer-id");
        assert_eq!(tp.key_id, "TESTKEY123");
        let err = TokenProvider::new("test-issuer-id", "TESTKEY123", "/nonexistent/path/key.p8")
            .unwrap_err();
        assert!(
            err.to_string().contains("failed to read private key"),
            "{err}"
        );
        // A file that is not a key is wrapped as a parse failure.
        let (_d2, bad) = create_test_key_file("not a key");
        let err = TokenProvider::new("i", "k", &bad).unwrap_err();
        assert_eq!(
            err.to_string(),
            "failed to parse private key: no PEM block found in private key"
        );
    }

    #[test]
    fn parse_private_key_cases() {
        // valid P-256 key (the curve is fixed by the p256 key type)
        let key = parse_private_key(generate_test_key().as_bytes()).expect("valid key");
        let _: &p256::ecdsa::VerifyingKey = key.verifying_key();
        // no PEM block
        let err = parse_private_key(b"not a PEM block").unwrap_err();
        assert!(err.to_string().contains("no PEM block found"), "{err}");
        // valid base64 but invalid PKCS#8
        let err = parse_private_key(
            b"-----BEGIN PRIVATE KEY-----\nYWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXo=\n-----END PRIVATE KEY-----\n",
        )
        .unwrap_err();
        assert!(err.to_string().contains("failed to parse PKCS8"), "{err}");
    }

    #[test]
    fn get_token_is_cached() {
        let (_dir, path) = create_test_key_file(&generate_test_key());
        let tp = TokenProvider::new("test-issuer", "TESTKEY123", &path).unwrap();
        let t1 = tp.get_token().unwrap();
        assert!(!t1.is_empty());
        assert_eq!(t1.split('.').count(), 3, "expected 3 parts in JWT");
        assert_eq!(
            t1,
            tp.get_token().unwrap(),
            "expected same token from cache"
        );
    }

    /// A cached token inside the refresh buffer is regenerated.
    #[test]
    fn get_token_refreshes_near_expiry() {
        let tp = TokenProvider::from_key("i", "k", parse_private_key(TEST_KEY.as_bytes()).unwrap());
        *tp.cache.lock().unwrap() =
            Some(("stale".into(), SystemTime::now() + TOKEN_REFRESH_BUFFER / 2));
        let t = tp.get_token().unwrap();
        assert_ne!(t, "stale");
        assert!(tp.verify_token(&t));
    }

    #[test]
    fn generate_token_claims() {
        let (_dir, path) = create_test_key_file(&generate_test_key());
        let tp = TokenProvider::new("test-issuer-id", "TESTKEY123", &path).unwrap();
        let (token, expires) = tp.generate_token().unwrap();
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);
        let header = b64(parts[0]);
        assert_eq!(header["alg"], "ES256");
        assert_eq!(header["typ"], "JWT");
        assert_eq!(header["kid"], "TESTKEY123");
        let payload = b64(parts[1]);
        assert_eq!(payload["iss"], "test-issuer-id");
        assert_eq!(payload["aud"], "appstoreconnect-v1");
        assert_eq!(
            payload["exp"].as_u64().unwrap() - payload["iat"].as_u64().unwrap(),
            TOKEN_DURATION.as_secs()
        );
        let now = SystemTime::now();
        assert!(expires > now, "token already expired");
        let expected = now + TOKEN_DURATION;
        let minute = Duration::from_secs(60);
        assert!(
            expires > expected - minute && expires < expected + minute,
            "expiry not ~15 minutes out"
        );
        // Claims are encoded with sorted keys.
        let raw = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(parts[1])
            .unwrap();
        let raw = String::from_utf8(raw).unwrap();
        assert!(
            raw.starts_with(r#"{"aud":"appstoreconnect-v1","exp":"#),
            "{raw}"
        );
    }

    // Malformed, tampered and foreign-key tokens all fail verification.
    #[test]
    fn verify_token_cases() {
        let (_dir, path) = create_test_key_file(&generate_test_key());
        let tp = TokenProvider::new("test-issuer", "TESTKEY123", &path).unwrap();
        let (token, _) = tp.generate_token().unwrap();
        assert!(tp.verify_token(&token), "valid token should verify");
        let tampered = format!("{}XXXXX", &token[..token.len() - 5]);
        assert!(
            !tp.verify_token(&tampered),
            "tampered token should not verify"
        );
        assert!(
            !tp.verify_token("invalid.token"),
            "invalid format should not verify"
        );
        assert!(!tp.verify_token(""), "empty token should not verify");
        assert!(
            !tp.verify_token(&format!("{token}.extra")),
            "four parts should not verify"
        );
        // A token signed by another key does not verify.
        let other = TokenProvider::from_key(
            "test-issuer",
            "TESTKEY123",
            parse_private_key(TEST_KEY.as_bytes()).unwrap(),
        );
        assert!(!other.verify_token(&token));
    }

    // JWT segments use URL-safe, unpadded base64; pin the vectors and the
    // token encoding.
    #[test]
    fn base64_url_encoding() {
        let enc = |b: &[u8]| base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(b);
        assert_eq!(enc(b""), "");
        assert_eq!(enc(b"hello"), "aGVsbG8");
        assert_eq!(enc(&[0xff, 0xfe, 0xfd]), "__79");
        let tp = TokenProvider::from_key("i", "k", parse_private_key(TEST_KEY.as_bytes()).unwrap());
        let t = tp.get_token().unwrap();
        assert!(
            !t.contains('=') && !t.contains('+') && !t.contains('/'),
            "token must be unpadded base64url: {t}"
        );
        // ES256 signatures are the fixed 64-byte r||s form.
        let sig = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(t.split('.').nth(2).unwrap())
            .unwrap();
        assert_eq!(sig.len(), 64);
    }

    #[test]
    fn concurrent_access() {
        let (_dir, path) = create_test_key_file(&generate_test_key());
        let tp =
            std::sync::Arc::new(TokenProvider::new("test-issuer", "TESTKEY123", &path).unwrap());
        let handles: Vec<_> = (0..100)
            .map(|_| {
                let tp = tp.clone();
                std::thread::spawn(move || tp.get_token())
            })
            .collect();
        let tokens: Vec<String> = handles
            .into_iter()
            .map(|h| h.join().unwrap().expect("get_token"))
            .collect();
        assert!(
            tokens.iter().all(|t| *t == tokens[0]),
            "all tokens should be the same (cached)"
        );
    }

    #[test]
    fn key_errors() {
        assert_eq!(
            parse_private_key(b"nope").unwrap_err().to_string(),
            "no PEM block found in private key"
        );
        let err = TokenProvider::new("i", "k", "/nonexistent/key.p8").unwrap_err();
        assert_eq!(
            err.to_string(),
            "failed to read private key: open /nonexistent/key.p8: no such file or directory"
        );
    }
}
