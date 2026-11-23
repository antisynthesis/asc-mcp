//! OAuth 2.1 resource-server support for the HTTP transport.
//!
//! The server validates JWT access tokens issued by the platform's
//! authorization server, as the MCP authorization specification requires:
//!
//! * tokens are signed JWTs verified against the issuer's JWKS, discovered
//!   through RFC 8414 / OpenID Connect metadata (or configured directly);
//! * `iss` must equal the configured issuer and `aud` must contain this
//!   server's canonical resource URI (RFC 8707), so tokens minted for other
//!   services are rejected;
//! * `exp`/`nbf` are enforced with a small clock-skew leeway;
//! * only asymmetric algorithms are accepted.
//!
//! Protected Resource Metadata (RFC 9728) is served so clients can discover
//! the authorization server, and challenges carry `resource_metadata` and
//! `scope` parameters.

use std::sync::Arc;
use std::time::{Duration, Instant};

use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::sync::RwLock;

use crate::context::Caller;

/// Accepted signature algorithms (asymmetric only).
const ALLOWED_ALGS: &[Algorithm] = &[
    Algorithm::RS256,
    Algorithm::RS384,
    Algorithm::RS512,
    Algorithm::PS256,
    Algorithm::PS384,
    Algorithm::PS512,
    Algorithm::ES256,
    Algorithm::ES384,
    Algorithm::EdDSA,
];

/// How long a fetched key set is trusted before it is refreshed.
const JWKS_TTL: Duration = Duration::from_secs(10 * 60);

/// Minimum spacing between refreshes triggered by an unknown `kid`.
const JWKS_MIN_REFRESH: Duration = Duration::from_secs(30);

/// Allowed clock skew for `exp`/`nbf`.
const LEEWAY_SECS: u64 = 60;

/// Resource-server configuration.
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    /// Authorization server issuer identifier.
    pub issuer: String,
    /// This server's canonical resource URI (the required audience), e.g.
    /// `https://mcp.example.com/mcp`.
    pub audience: String,
    /// JWKS URL; discovered from the issuer's metadata when `None`.
    pub jwks_url: Option<String>,
    /// Scope required for read-only tools and other read operations.
    pub read_scope: String,
    /// Scope required for tools that change state. Implies `read_scope`.
    pub write_scope: String,
}

impl OAuthConfig {
    /// The Protected Resource Metadata URL (RFC 9728 §3.1): the
    /// well-known path inserted between the origin and the resource path.
    pub fn metadata_url(&self) -> String {
        let (origin, path) = split_origin(&self.audience);
        format!("{origin}/.well-known/oauth-protected-resource{path}")
    }

    /// Paths at which the metadata document is served: the path-specific
    /// location and the root location.
    pub fn metadata_paths(&self) -> Vec<String> {
        let (_, path) = split_origin(&self.audience);
        let mut v = vec!["/.well-known/oauth-protected-resource".to_string()];
        if !path.is_empty() {
            v.push(format!("/.well-known/oauth-protected-resource{path}"));
        }
        v
    }

    /// The Protected Resource Metadata document.
    pub fn metadata_document(&self) -> Value {
        json!({
            "resource": self.audience,
            "authorization_servers": [self.issuer],
            "scopes_supported": [self.read_scope, self.write_scope],
            "bearer_methods_supported": ["header"],
            "resource_name": "App Store Connect MCP server",
        })
    }

    /// Reports whether `caller` holds `required`, honouring the hierarchy
    /// (the write scope implies the read scope).
    pub fn has_scope(&self, caller: &Caller, required: &str) -> bool {
        caller
            .scopes
            .iter()
            .any(|s| s == required || (required == self.read_scope && *s == self.write_scope))
    }
}

/// Splits `https://host[:port]/path` into origin and path (no trailing `/`).
fn split_origin(uri: &str) -> (String, String) {
    let after_scheme = uri.find("://").map(|i| i + 3).unwrap_or(0);
    match uri[after_scheme..].find('/') {
        Some(i) => {
            let (o, p) = uri.split_at(after_scheme + i);
            (o.to_string(), p.trim_end_matches('/').to_string())
        }
        None => (uri.to_string(), String::new()),
    }
}

/// Why a token was rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenError(pub String);

#[derive(Default)]
struct Keys {
    set: Option<JwkSet>,
    fetched: Option<Instant>,
    jwks_url: Option<String>,
}

/// Validates access tokens.
pub struct Verifier {
    cfg: OAuthConfig,
    http: reqwest::Client,
    keys: RwLock<Keys>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ScopeClaim {
    Str(String),
    List(Vec<String>),
}

#[derive(Deserialize)]
struct Claims {
    sub: Option<String>,
    scope: Option<ScopeClaim>,
    scp: Option<ScopeClaim>,
    client_id: Option<String>,
    azp: Option<String>,
}

impl Verifier {
    /// Creates a verifier. Keys are fetched lazily on first use.
    pub fn new(cfg: OAuthConfig) -> Arc<Self> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();
        let jwks_url = cfg.jwks_url.clone();
        Arc::new(Self {
            cfg,
            http,
            keys: RwLock::new(Keys {
                jwks_url,
                ..Default::default()
            }),
        })
    }

    /// Creates a verifier with a fixed key set (tests, air-gapped setups).
    pub fn with_keys(cfg: OAuthConfig, set: JwkSet) -> Arc<Self> {
        let v = Self::new(cfg);
        if let Ok(mut k) = v.keys.try_write() {
            k.set = Some(set);
            k.fetched = Some(Instant::now() + Duration::from_secs(100 * 365 * 24 * 3600));
        }
        v
    }

    /// The configuration.
    pub fn config(&self) -> &OAuthConfig {
        &self.cfg
    }

    /// Verifies `token` and returns the caller it identifies.
    pub async fn verify(&self, token: &str) -> Result<Caller, TokenError> {
        let header = jsonwebtoken::decode_header(token)
            .map_err(|_| TokenError("malformed access token".into()))?;
        if !ALLOWED_ALGS.contains(&header.alg) {
            return Err(TokenError("unsupported token signing algorithm".into()));
        }
        let key = self.key_for(header.kid.as_deref(), header.alg).await?;
        let mut v = Validation::new(header.alg);
        v.algorithms = vec![header.alg];
        v.set_issuer(&[&self.cfg.issuer]);
        v.set_audience(&[&self.cfg.audience]);
        v.set_required_spec_claims(&["exp", "iss", "aud"]);
        v.validate_nbf = true;
        v.leeway = LEEWAY_SECS;
        let data = jsonwebtoken::decode::<Claims>(token, &key, &v).map_err(|e| {
            use jsonwebtoken::errors::ErrorKind;
            TokenError(
                match e.kind() {
                    ErrorKind::ExpiredSignature => "access token expired",
                    ErrorKind::ImmatureSignature => "access token not yet valid",
                    ErrorKind::InvalidAudience => "access token was not issued for this server",
                    ErrorKind::InvalidIssuer => "access token issuer is not trusted",
                    ErrorKind::InvalidSignature => "access token signature is invalid",
                    ErrorKind::MissingRequiredClaim(_) => {
                        "access token is missing a required claim"
                    }
                    _ => "access token is invalid",
                }
                .into(),
            )
        })?;
        let c = data.claims;
        let mut scopes = Vec::new();
        for claim in [c.scope, c.scp].into_iter().flatten() {
            match claim {
                ScopeClaim::Str(s) => scopes.extend(s.split_whitespace().map(str::to_string)),
                ScopeClaim::List(l) => scopes.extend(l),
            }
        }
        Ok(Caller {
            subject: c.sub,
            client_id: c.client_id.or(c.azp),
            scopes,
        })
    }

    /// Finds the verification key, refreshing the key set when it is stale
    /// or does not contain `kid`.
    async fn key_for(&self, kid: Option<&str>, alg: Algorithm) -> Result<DecodingKey, TokenError> {
        let lookup = |set: &JwkSet| -> Option<DecodingKey> {
            set.keys
                .iter()
                .filter(|k| kid.is_none() || k.common.key_id.as_deref() == kid)
                .filter(|k| {
                    k.common
                        .key_algorithm
                        .is_none_or(|ka| format!("{ka:?}") == format!("{alg:?}"))
                })
                .find_map(|k| DecodingKey::from_jwk(k).ok())
        };
        {
            // `Instant::elapsed` saturates to zero for instants in the
            // future, which is how a fixed key set never goes stale.
            let k = self.keys.read().await;
            if let (Some(set), Some(fetched)) = (&k.set, k.fetched)
                && fetched.elapsed() < JWKS_TTL
            {
                if let Some(key) = lookup(set) {
                    return Ok(key);
                }
                // Unknown kid: refetch, but not more often than
                // JWKS_MIN_REFRESH, so bad tokens cannot hammer the IdP.
                if fetched.elapsed() < JWKS_MIN_REFRESH {
                    return Err(TokenError("access token signing key is unknown".into()));
                }
            }
        }
        let mut k = self.keys.write().await;
        let set = self.fetch_jwks(&mut k).await?;
        let key = lookup(&set);
        k.set = Some(set);
        k.fetched = Some(Instant::now());
        key.ok_or_else(|| TokenError("access token signing key is unknown".into()))
    }

    async fn fetch_jwks(&self, k: &mut Keys) -> Result<JwkSet, TokenError> {
        let unavailable = || TokenError("authorization server keys are unavailable".into());
        if k.jwks_url.is_none() {
            k.jwks_url = Some(self.discover_jwks_url().await.ok_or_else(unavailable)?);
        }
        let url = k.jwks_url.clone().ok_or_else(unavailable)?;
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|_| unavailable())?;
        if !resp.status().is_success() {
            return Err(unavailable());
        }
        resp.json::<JwkSet>().await.map_err(|_| unavailable())
    }

    /// RFC 8414 / OpenID Connect discovery, with the issuer check both
    /// specifications require.
    async fn discover_jwks_url(&self) -> Option<String> {
        let issuer = self.cfg.issuer.trim_end_matches('/');
        let (origin, path) = split_origin(issuer);
        let mut candidates = vec![format!(
            "{origin}/.well-known/oauth-authorization-server{path}"
        )];
        candidates.push(format!("{origin}/.well-known/openid-configuration{path}"));
        if !path.is_empty() {
            candidates.push(format!("{issuer}/.well-known/openid-configuration"));
        }
        for url in candidates {
            let Ok(resp) = self.http.get(&url).send().await else {
                continue;
            };
            if !resp.status().is_success() {
                continue;
            }
            let Ok(doc) = resp.json::<Value>().await else {
                continue;
            };
            if doc["issuer"].as_str() != Some(self.cfg.issuer.as_str()) {
                continue;
            }
            if let Some(j) = doc["jwks_uri"].as_str() {
                return Some(j.to_string());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(aud: &str) -> OAuthConfig {
        OAuthConfig {
            issuer: "https://auth.example.com".into(),
            audience: aud.into(),
            jwks_url: None,
            read_scope: "asc:read".into(),
            write_scope: "asc:write".into(),
        }
    }

    #[test]
    fn metadata_locations() {
        let c = cfg("https://mcp.example.com/mcp");
        assert_eq!(
            c.metadata_url(),
            "https://mcp.example.com/.well-known/oauth-protected-resource/mcp"
        );
        assert_eq!(
            c.metadata_paths(),
            vec![
                "/.well-known/oauth-protected-resource",
                "/.well-known/oauth-protected-resource/mcp"
            ]
        );
        let c = cfg("https://mcp.example.com");
        assert_eq!(
            c.metadata_url(),
            "https://mcp.example.com/.well-known/oauth-protected-resource"
        );
        assert_eq!(c.metadata_paths().len(), 1);
    }

    #[test]
    fn scope_hierarchy() {
        let c = cfg("https://mcp.example.com/mcp");
        let w = Caller {
            scopes: vec!["asc:write".into()],
            ..Default::default()
        };
        let r = Caller {
            scopes: vec!["asc:read".into()],
            ..Default::default()
        };
        assert!(c.has_scope(&w, "asc:read") && c.has_scope(&w, "asc:write"));
        assert!(c.has_scope(&r, "asc:read") && !c.has_scope(&r, "asc:write"));
    }
}
