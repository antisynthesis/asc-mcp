//! Captures build metadata for `asc-mcp version`.

use std::process::Command;

fn main() {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    let version = Command::new(rustc)
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=ASC_MCP_RUSTC_VERSION={version}");
    for var in ["ASC_MCP_VERSION", "ASC_MCP_COMMIT", "ASC_MCP_BUILD_DATE"] {
        println!("cargo:rerun-if-env-changed={var}");
    }
}
