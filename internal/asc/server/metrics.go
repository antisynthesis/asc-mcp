// Package server provides the MCP server implementation for App Store Connect.
package server

import (
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/collectors"
)

// metrics is the Prometheus surface for the HTTP transport. It is
// always allocated; if no Prometheus registerer is provided the
// metrics still update, they just are not scraped.
type metrics struct {
	registry        *prometheus.Registry
	httpRequests    *prometheus.CounterVec
	httpDuration    *prometheus.HistogramVec
	toolCalls       *prometheus.CounterVec
	toolDuration    *prometheus.HistogramVec
	authFailures    prometheus.Counter
	sessionsActive  prometheus.Gauge
	sessionsCreated prometheus.Counter
	sessionsReaped  prometheus.Counter
}

// newMetrics constructs a metrics surface and registers all collectors
// against its own Registry. The Registry is exposed via /metrics by
// the HTTPServer; callers that want to share a different registry can
// reach in via Registry().
func newMetrics() *metrics {
	reg := prometheus.NewRegistry()
	// Standard Go runtime + process collectors so /metrics includes the
	// usual go_* and process_* time series operators expect.
	reg.MustRegister(collectors.NewGoCollector())
	reg.MustRegister(collectors.NewProcessCollector(collectors.ProcessCollectorOpts{}))

	m := &metrics{registry: reg}
	m.httpRequests = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Namespace: "asc_mcp",
			Subsystem: "http",
			Name:      "requests_total",
			Help:      "Total number of HTTP requests handled by the MCP transport, partitioned by HTTP method and status code.",
		},
		[]string{"method", "status"},
	)
	m.httpDuration = prometheus.NewHistogramVec(
		prometheus.HistogramOpts{
			Namespace: "asc_mcp",
			Subsystem: "http",
			Name:      "request_duration_seconds",
			Help:      "Wall-clock latency of HTTP requests.",
			Buckets:   prometheus.DefBuckets,
		},
		[]string{"method"},
	)
	m.toolCalls = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Namespace: "asc_mcp",
			Subsystem: "tool",
			Name:      "calls_total",
			Help:      "Total number of MCP tool invocations, partitioned by tool name and outcome (ok|error|unknown).",
		},
		[]string{"tool", "result"},
	)
	m.toolDuration = prometheus.NewHistogramVec(
		prometheus.HistogramOpts{
			Namespace: "asc_mcp",
			Subsystem: "tool",
			Name:      "duration_seconds",
			Help:      "Wall-clock latency of MCP tool invocations.",
			Buckets:   prometheus.DefBuckets,
		},
		[]string{"tool"},
	)
	m.authFailures = prometheus.NewCounter(
		prometheus.CounterOpts{
			Namespace: "asc_mcp",
			Subsystem: "http",
			Name:      "auth_failures_total",
			Help:      "Total number of HTTP requests rejected for missing or invalid Bearer tokens.",
		},
	)
	m.sessionsActive = prometheus.NewGauge(
		prometheus.GaugeOpts{
			Namespace: "asc_mcp",
			Subsystem: "session",
			Name:      "active",
			Help:      "Number of currently active MCP sessions on this process.",
		},
	)
	m.sessionsCreated = prometheus.NewCounter(
		prometheus.CounterOpts{
			Namespace: "asc_mcp",
			Subsystem: "session",
			Name:      "created_total",
			Help:      "Total number of MCP sessions established since process start.",
		},
	)
	m.sessionsReaped = prometheus.NewCounter(
		prometheus.CounterOpts{
			Namespace: "asc_mcp",
			Subsystem: "session",
			Name:      "reaped_total",
			Help:      "Total number of MCP sessions discarded due to idle timeout.",
		},
	)
	reg.MustRegister(
		m.httpRequests,
		m.httpDuration,
		m.toolCalls,
		m.toolDuration,
		m.authFailures,
		m.sessionsActive,
		m.sessionsCreated,
		m.sessionsReaped,
	)
	return m
}

// Registry exposes the Prometheus registry the HTTPServer scrapes from.
// Tests use this to read counter values directly.
func (m *metrics) Registry() *prometheus.Registry { return m.registry }
