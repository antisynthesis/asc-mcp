//! Command-line interface.

use std::sync::Arc;

use clap::{Args, Parser, Subcommand};

use crate::api::Client;
use crate::config::Config;
use crate::logging;
use crate::server::dispatch::Dispatcher;
use crate::server::http::{HttpOptions, HttpServer};
use crate::server::stdio::StdioServer;
use crate::tools::Registry;

/// Version string, overridable at build time with `ASC_MCP_VERSION`.
pub const VERSION: &str = match option_env!("ASC_MCP_VERSION") {
    Some(v) => v,
    None => "dev",
};
/// Commit hash, overridable at build time with `ASC_MCP_COMMIT`.
pub const COMMIT: &str = match option_env!("ASC_MCP_COMMIT") {
    Some(v) => v,
    None => "unknown",
};
/// Build date, overridable at build time with `ASC_MCP_BUILD_DATE`.
pub const BUILD_DATE: &str = match option_env!("ASC_MCP_BUILD_DATE") {
    Some(v) => v,
    None => "unknown",
};

#[derive(Parser)]
#[command(
    name = "asc-mcp",
    about = "MCP server for Apple App Store Connect",
    long_about = "asc-mcp is a Model Context Protocol (MCP) server that provides\ntools for managing Apple App Store Connect resources.\n\nIt supports app management, build management, TestFlight beta testing,\nand provisioning operations.",
    disable_version_flag = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start the MCP server
    #[command(long_about = SERVE_LONG)]
    Serve,
    /// Start the MCP server over Streamable HTTP
    #[command(name = "serve-http", long_about = SERVE_HTTP_LONG)]
    ServeHttp(ServeHttpArgs),
    /// Print version information
    #[command(long_about = "Print the version, commit hash, and build date of asc-mcp.")]
    Version,
    /// Validate configuration
    #[command(long_about = "Validate that all required environment variables are set and the\nprivate key file exists and is readable.\n\nThis command does not make any API calls - it only validates local configuration.")]
    Validate,
    /// List available MCP tools
    #[command(long_about = "List all MCP tools that are available when the server is running.\n\nThis command displays the tool names and descriptions without\nrequiring API credentials.")]
    Tools,
}

const SERVE_LONG: &str = "Start the MCP server and listen for JSON-RPC requests on stdin/stdout.

The server requires App Store Connect API credentials to be configured
via environment variables:

  ASC_ISSUER_ID        Your App Store Connect API Issuer ID
  ASC_KEY_ID           Your App Store Connect API Key ID
  ASC_PRIVATE_KEY_PATH Path to your .p8 private key file

Example:
  export ASC_ISSUER_ID=\"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx\"
  export ASC_KEY_ID=\"XXXXXXXXXX\"
  export ASC_PRIVATE_KEY_PATH=\"/path/to/AuthKey.p8\"
  asc-mcp serve";

const SERVE_HTTP_LONG: &str = "Start the MCP server bound to an HTTP address, implementing the
Streamable HTTP transport. Both the stateless 2026-07-28 revision and the
legacy session-based revisions (2024-11-05 through 2025-11-25) are served.

Endpoints:
  POST   /mcp        JSON-RPC submission
  DELETE /mcp        terminate a legacy session (requires Mcp-Session-Id)
  GET    /healthz    readiness probe
  GET    /metrics    Prometheus metrics

Legacy sessions are identified by an Mcp-Session-Id header that the server
assigns on initialize and the client echoes on every subsequent request.
2026-07-28 requests carry their protocol version in _meta and the
MCP-Protocol-Version header and use no session.

Example:
  export ASC_ISSUER_ID=... ASC_KEY_ID=... ASC_PRIVATE_KEY_PATH=...
  asc-mcp serve-http --addr 127.0.0.1:8080 \\
    --allowed-origins https://app.example \\
    --auth-tokens \"$ASC_MCP_AUTH_TOKENS\" \\
    --log-format json";

#[derive(Args)]
struct ServeHttpArgs {
    /// address to bind (e.g. 127.0.0.1:8080, or 0.0.0.0:8080 inside a container)
    #[arg(long, default_value = "127.0.0.1:8080")]
    addr: String,
    /// comma-separated list of additional allowed Origin headers; loopback origins are always allowed
    #[arg(long = "allowed-origins", default_value = "")]
    allowed_origins: String,
    /// comma-separated list of accepted Bearer tokens. Empty leaves the endpoint open. Reads from ASC_MCP_AUTH_TOKENS env var when the flag is unset.
    #[arg(long = "auth-tokens", default_value = "")]
    auth_tokens: String,
    /// log output format: json or text
    #[arg(long = "log-format", default_value = "json")]
    log_format: String,
    /// log level: debug, info, warn, error
    #[arg(long = "log-level", default_value = "info")]
    log_level: String,
    /// PEM-encoded TLS certificate file. Must be set together with --tls-key. If unset, the server listens HTTP; use a TLS-terminating reverse proxy in production.
    #[arg(long = "tls-cert", default_value = "")]
    tls_cert: String,
    /// PEM-encoded TLS private key file. Must be set together with --tls-cert.
    #[arg(long = "tls-key", default_value = "")]
    tls_key: String,
}

/// Runs the CLI and returns the process exit code.
pub async fn run() -> i32 {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Serve => serve().await,
        Command::ServeHttp(a) => serve_http(a).await,
        Command::Version => {
            version();
            Ok(())
        }
        Command::Validate => validate(),
        Command::Tools => {
            tools();
            Ok(())
        }
    };
    match result {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

fn build_registry(cfg: &Config) -> Result<Arc<Registry>, String> {
    let client = Client::new(&cfg.issuer_id, &cfg.key_id, &cfg.private_key_path)
        .map_err(|e| format!("failed to create dispatcher: {e}"))?;
    Ok(Arc::new(Registry::new(Some(Arc::new(client)))))
}

async fn serve() -> Result<(), String> {
    let cfg = Config::load()?;
    let registry = build_registry(&cfg)?;
    let srv = StdioServer::new(Arc::new(Dispatcher::new(registry)));
    logging::printf("starting MCP server");
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    srv.run(stdin, tokio::io::stdout()).await.map_err(|e| format!("failed to read request: {e}"))
}

fn split_csv(s: &str) -> Vec<String> {
    s.split(',').map(str::trim).filter(|v| !v.is_empty()).map(str::to_string).collect()
}

async fn serve_http(a: ServeHttpArgs) -> Result<(), String> {
    let cfg = Config::load()?;
    let level = logging::Level::parse(&a.log_level)?;
    let format = logging::parse_format(&a.log_format)?;
    logging::init(format, level);
    let tokens = if a.auth_tokens.is_empty() {
        std::env::var("ASC_MCP_AUTH_TOKENS").unwrap_or_default()
    } else {
        a.auth_tokens.clone()
    };
    let opts = HttpOptions { allowed_origins: split_csv(&a.allowed_origins), auth_tokens: split_csv(&tokens) };
    if opts.auth_tokens.is_empty() {
        logging::warn(
            "serve-http running without auth tokens; the /mcp endpoint is open to anyone who can reach it",
            &[],
        );
    }
    let registry = build_registry(&cfg)?;
    let srv = HttpServer::new(registry, opts);
    let use_tls = !a.tls_cert.is_empty() && !a.tls_key.is_empty();
    if use_tls {
        logging::printf(format!("serving MCP over HTTPS on {}", a.addr));
    } else {
        logging::printf(format!(
            "serving MCP over HTTP on {} (no TLS; expect a reverse proxy to terminate TLS in production)",
            a.addr
        ));
    }
    srv.listen_and_serve(&a.addr, &a.tls_cert, &a.tls_key, shutdown_signal()).await
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let term = async {
        if let Ok(mut s) = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            s.recv().await;
        }
    };
    #[cfg(not(unix))]
    let term = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = term => {},
    }
}

fn version() {
    println!("asc-mcp version {VERSION}");
    println!("  Commit:     {COMMIT}");
    println!("  Built:      {BUILD_DATE}");
    println!("  Rust:       {}", env!("ASC_MCP_RUSTC_VERSION"));
    println!("  OS/Arch:    {}/{}", std::env::consts::OS, std::env::consts::ARCH);
}

fn validate() -> Result<(), String> {
    println!("Validating configuration...");
    println!();
    let get = |k: &str| std::env::var(k).unwrap_or_default();
    let (issuer, key_id, key_path) = (get("ASC_ISSUER_ID"), get("ASC_KEY_ID"), get("ASC_PRIVATE_KEY_PATH"));
    let mut has_errors = false;
    if issuer.is_empty() {
        println!("[FAIL] ASC_ISSUER_ID is not set");
        has_errors = true;
    } else {
        let prefix: String = issuer.chars().take(8).collect();
        println!("[OK]   ASC_ISSUER_ID is set ({prefix}...)");
    }
    if key_id.is_empty() {
        println!("[FAIL] ASC_KEY_ID is not set");
        has_errors = true;
    } else {
        println!("[OK]   ASC_KEY_ID is set ({key_id})");
    }
    if key_path.is_empty() {
        println!("[FAIL] ASC_PRIVATE_KEY_PATH is not set");
        has_errors = true;
    } else {
        match std::fs::metadata(&key_path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                println!("[FAIL] ASC_PRIVATE_KEY_PATH file not found: {key_path}");
                has_errors = true;
            }
            Err(e) => {
                println!("[FAIL] ASC_PRIVATE_KEY_PATH error: {}", crate::go::os::path_error("stat", &key_path, &e));
                has_errors = true;
            }
            Ok(_) => println!("[OK]   ASC_PRIVATE_KEY_PATH exists ({key_path})"),
        }
    }
    println!();
    if has_errors {
        return Err("configuration validation failed".into());
    }
    let cfg = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            println!("[FAIL] Configuration load error: {e}");
            return Err(e);
        }
    };
    // Parse the key too, so a malformed .p8 is caught without an API call.
    if let Err(e) = crate::api::auth::TokenProvider::new(&cfg.issuer_id, &cfg.key_id, &cfg.private_key_path) {
        println!("[FAIL] Private key error: {e}");
        return Err(e.to_string());
    }
    println!("[OK]   Configuration is valid");
    Ok(())
}

fn tools() {
    let registry = Registry::new(None);
    let list = registry.list_tools();
    println!("Available MCP Tools ({} total):\n", list.len());
    let mut cats: [(&str, Vec<String>); 4] =
        [("App Management", vec![]), ("Builds", vec![]), ("TestFlight", vec![]), ("Provisioning", vec![])];
    for t in list {
        let line = format!("  {} - {}", t.name, t.description());
        let idx = match t.name.as_str() {
            "list_apps" | "get_app" | "get_app_versions" => 0,
            "list_builds" | "get_build" => 1,
            "list_beta_groups" | "create_beta_group" | "delete_beta_group" | "list_beta_testers"
            | "invite_beta_tester" | "remove_beta_tester" | "add_tester_to_group" => 2,
            _ => 3,
        };
        cats[idx].1.push(line);
    }
    for (name, lines) in &cats {
        if !lines.is_empty() {
            println!("{name}:");
            for l in lines {
                println!("{l}");
            }
            println!();
        }
    }
}
