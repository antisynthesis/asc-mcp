//! Configuration loaded from the environment.

use std::path::Path;

/// App Store Connect API credentials.
#[derive(Debug, Clone)]
pub struct Config {
    /// API Issuer ID (`ASC_ISSUER_ID`).
    pub issuer_id: String,
    /// API Key ID (`ASC_KEY_ID`).
    pub key_id: String,
    /// Path to the `.p8` private key (`ASC_PRIVATE_KEY_PATH`).
    pub private_key_path: String,
}

impl Config {
    /// Loads and validates configuration from environment variables.
    pub fn load() -> Result<Self, String> {
        let get = |k: &str| std::env::var(k).unwrap_or_default();
        let cfg = Self {
            issuer_id: get("ASC_ISSUER_ID"),
            key_id: get("ASC_KEY_ID"),
            private_key_path: get("ASC_PRIVATE_KEY_PATH"),
        };
        if cfg.issuer_id.is_empty() {
            return Err("ASC_ISSUER_ID environment variable is required".into());
        }
        if cfg.key_id.is_empty() {
            return Err("ASC_KEY_ID environment variable is required".into());
        }
        if cfg.private_key_path.is_empty() {
            return Err("ASC_PRIVATE_KEY_PATH environment variable is required".into());
        }
        if !Path::new(&cfg.private_key_path).exists() {
            return Err(format!(
                "private key file not found: {}",
                cfg.private_key_path
            ));
        }
        Ok(cfg)
    }
}
