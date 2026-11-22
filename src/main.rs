//! asc-mcp binary entry point.

#[tokio::main]
async fn main() {
    // Behave like a Go binary on a closed pipe (e.g. `asc-mcp tools | head`):
    // exit quietly via SIGPIPE instead of panicking on a failed write.
    #[cfg(unix)]
    // SAFETY: restoring the default disposition of SIGPIPE before any other
    // threads exist has no memory-safety implications.
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    std::process::exit(asc_mcp::cli::run().await);
}
