// Package cmd provides the command-line interface for asc-mcp.
package cmd

import (
	"context"
	"log"
	"log/slog"
	"os"
	"os/signal"
	"strings"
	"syscall"

	"github.com/spf13/cobra"

	"github.com/antisynthesis/asc-mcp/internal/asc/config"
	"github.com/antisynthesis/asc-mcp/internal/asc/server"
)

var (
	httpAddr           string
	httpAllowedOrigins string
	httpAuthTokens     string
	httpLogFormat      string
	httpLogLevel       string
)

var serveHTTPCmd = &cobra.Command{
	Use:   "serve-http",
	Short: "Start the MCP server over Streamable HTTP",
	Long: `Start the MCP server bound to an HTTP address, implementing the
Streamable HTTP transport from the 2025-06-18 MCP specification.

Endpoints:
  POST   /mcp        JSON-RPC submission
  DELETE /mcp        terminate a session (requires Mcp-Session-Id)
  GET    /healthz    readiness probe
  GET    /metrics    Prometheus metrics

Sessions are identified by an Mcp-Session-Id header that the server
assigns on initialize and the client echoes on every subsequent request.

Example:
  export ASC_ISSUER_ID=... ASC_KEY_ID=... ASC_PRIVATE_KEY_PATH=...
  asc-mcp serve-http --addr :8080 \
    --allowed-origins https://app.example \
    --auth-tokens "$ASC_MCP_AUTH_TOKENS" \
    --log-format json`,
	RunE: runServeHTTP,
}

func init() {
	serveHTTPCmd.Flags().StringVar(&httpAddr, "addr", ":8080", "address to bind (e.g. :8080 or 127.0.0.1:8080)")
	serveHTTPCmd.Flags().StringVar(&httpAllowedOrigins, "allowed-origins", "",
		"comma-separated list of allowed Origin headers; empty disables the check")
	serveHTTPCmd.Flags().StringVar(&httpAuthTokens, "auth-tokens", "",
		"comma-separated list of accepted Bearer tokens. Empty leaves the endpoint open. "+
			"Reads from ASC_MCP_AUTH_TOKENS env var when the flag is unset.")
	serveHTTPCmd.Flags().StringVar(&httpLogFormat, "log-format", "json",
		"log output format: json or text")
	serveHTTPCmd.Flags().StringVar(&httpLogLevel, "log-level", "info",
		"log level: debug, info, warn, error")
}

func runServeHTTP(_ *cobra.Command, _ []string) error {
	cfg, err := config.Load()
	if err != nil {
		return err
	}

	logger, err := buildLogger(httpLogFormat, httpLogLevel)
	if err != nil {
		return err
	}
	slog.SetDefault(logger)

	opts := server.HTTPOptions{
		AllowedOrigins: splitCSV(httpAllowedOrigins),
		AuthTokens:     splitCSV(authTokensValue()),
		Logger:         logger,
	}
	if len(opts.AuthTokens) == 0 {
		logger.Warn("serve-http running without auth tokens; the /mcp endpoint is open to anyone who can reach it")
	}

	srv, err := server.NewHTTPServer(cfg, opts)
	if err != nil {
		return err
	}

	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()

	log.Printf("serving MCP over HTTP on %s", httpAddr)
	return srv.ListenAndServe(ctx, httpAddr)
}

// authTokensValue returns the configured auth tokens, falling back to
// the ASC_MCP_AUTH_TOKENS env var when --auth-tokens is empty.
func authTokensValue() string {
	if httpAuthTokens != "" {
		return httpAuthTokens
	}
	return os.Getenv("ASC_MCP_AUTH_TOKENS")
}

// splitCSV trims whitespace and drops empty entries from a
// comma-separated string.
func splitCSV(s string) []string {
	if s == "" {
		return nil
	}
	var out []string
	for _, v := range strings.Split(s, ",") {
		v = strings.TrimSpace(v)
		if v != "" {
			out = append(out, v)
		}
	}
	return out
}

// buildLogger returns a slog logger configured with the requested
// format and level. Logs always go to stderr so they never collide with
// any stdio JSON-RPC stream when both transports run together.
func buildLogger(format, level string) (*slog.Logger, error) {
	var lvl slog.Level
	switch strings.ToLower(level) {
	case "debug":
		lvl = slog.LevelDebug
	case "info", "":
		lvl = slog.LevelInfo
	case "warn", "warning":
		lvl = slog.LevelWarn
	case "error":
		lvl = slog.LevelError
	default:
		return nil, errUnknownLogLevel(level)
	}
	opts := &slog.HandlerOptions{Level: lvl}
	var h slog.Handler
	switch strings.ToLower(format) {
	case "json", "":
		h = slog.NewJSONHandler(os.Stderr, opts)
	case "text":
		h = slog.NewTextHandler(os.Stderr, opts)
	default:
		return nil, errUnknownLogFormat(format)
	}
	return slog.New(h), nil
}

type errUnknownLogLevel string

func (e errUnknownLogLevel) Error() string { return "unknown log level: " + string(e) }

type errUnknownLogFormat string

func (e errUnknownLogFormat) Error() string { return "unknown log format: " + string(e) }
