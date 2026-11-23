//! Prometheus metrics for the HTTP transport.

use std::time::Duration;

use prometheus::{
    Counter, CounterVec, Encoder, Gauge, HistogramOpts, HistogramVec, Opts, Registry, TextEncoder,
};

/// The metric surface exposed at `/metrics`.
pub struct Metrics {
    registry: Registry,
    /// `asc_mcp_http_requests_total{method,status}`.
    pub http_requests: CounterVec,
    /// `asc_mcp_http_request_duration_seconds{method}`.
    pub http_duration: HistogramVec,
    /// `asc_mcp_tool_calls_total{tool,result}`.
    pub tool_calls: CounterVec,
    /// `asc_mcp_tool_duration_seconds{tool}`.
    pub tool_duration: HistogramVec,
    /// `asc_mcp_http_auth_failures_total`.
    pub auth_failures: Counter,
    /// `asc_mcp_session_active`.
    pub sessions_active: Gauge,
    /// `asc_mcp_session_created_total`.
    pub sessions_created: Counter,
    /// `asc_mcp_session_reaped_total`.
    pub sessions_reaped: Counter,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    /// Creates and registers every collector, plus the process collector.
    pub fn new() -> Self {
        let registry = Registry::new();
        #[cfg(target_os = "linux")]
        {
            let _ = registry.register(Box::new(
                prometheus::process_collector::ProcessCollector::for_self(),
            ));
        }
        let opts = |subsystem: &str, name: &str, help: &str| {
            Opts::new(name, help)
                .namespace("asc_mcp")
                .subsystem(subsystem)
        };
        let hopts = |subsystem: &str, name: &str, help: &str| {
            HistogramOpts::new(name, help)
                .namespace("asc_mcp")
                .subsystem(subsystem)
        };
        let m = Self {
            http_requests: CounterVec::new(
                opts("http", "requests_total", "Total number of HTTP requests handled by the MCP transport, partitioned by HTTP method and status code."),
                &["method", "status"],
            )
            .expect("valid metric"),
            http_duration: HistogramVec::new(hopts("http", "request_duration_seconds", "Wall-clock latency of HTTP requests."), &["method"])
                .expect("valid metric"),
            tool_calls: CounterVec::new(
                opts("tool", "calls_total", "Total number of MCP tool invocations, partitioned by tool name and outcome (ok|error|unknown)."),
                &["tool", "result"],
            )
            .expect("valid metric"),
            tool_duration: HistogramVec::new(hopts("tool", "duration_seconds", "Wall-clock latency of MCP tool invocations."), &["tool"])
                .expect("valid metric"),
            auth_failures: Counter::with_opts(opts("http", "auth_failures_total", "Total number of HTTP requests rejected for missing or invalid Bearer tokens."))
                .expect("valid metric"),
            sessions_active: Gauge::with_opts(opts("session", "active", "Number of currently active MCP sessions on this process."))
                .expect("valid metric"),
            sessions_created: Counter::with_opts(opts("session", "created_total", "Total number of MCP sessions established since process start."))
                .expect("valid metric"),
            sessions_reaped: Counter::with_opts(opts("session", "reaped_total", "Total number of MCP sessions discarded due to idle timeout."))
                .expect("valid metric"),
            registry,
        };
        for c in [
            Box::new(m.http_requests.clone()) as Box<dyn prometheus::core::Collector>,
            Box::new(m.http_duration.clone()),
            Box::new(m.tool_calls.clone()),
            Box::new(m.tool_duration.clone()),
            Box::new(m.auth_failures.clone()),
            Box::new(m.sessions_active.clone()),
            Box::new(m.sessions_created.clone()),
            Box::new(m.sessions_reaped.clone()),
        ] {
            m.registry.register(c).expect("unique metric");
        }
        m
    }

    /// Records one tool invocation.
    pub fn observe_tool(&self, tool: &str, result: &str, elapsed: Duration) {
        self.tool_calls.with_label_values(&[tool, result]).inc();
        self.tool_duration
            .with_label_values(&[tool])
            .observe(elapsed.as_secs_f64());
    }

    /// The registry backing `/metrics`.
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Renders the Prometheus text exposition format.
    pub fn render(&self) -> (String, Vec<u8>) {
        let enc = TextEncoder::new();
        let mut buf = Vec::new();
        let _ = enc.encode(&self.registry.gather(), &mut buf);
        (enc.format_type().to_string(), buf)
    }
}
