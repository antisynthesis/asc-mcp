// Package cmd provides the command-line interface for asc-mcp.
package cmd

import (
	"context"
	"log"
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

Sessions are identified by an Mcp-Session-Id header that the server
assigns on initialize and the client echoes on every subsequent request.

Example:
  export ASC_ISSUER_ID=... ASC_KEY_ID=... ASC_PRIVATE_KEY_PATH=...
  asc-mcp serve-http --addr :8080 --allowed-origins https://app.example`,
	RunE: runServeHTTP,
}

func init() {
	serveHTTPCmd.Flags().StringVar(&httpAddr, "addr", ":8080", "address to bind (e.g. :8080 or 127.0.0.1:8080)")
	serveHTTPCmd.Flags().StringVar(&httpAllowedOrigins, "allowed-origins", "",
		"comma-separated list of allowed Origin headers; empty disables the check")
	serveHTTPCmd.Flags().StringVar(&httpAuthTokens, "auth-tokens", "",
		"comma-separated list of accepted Bearer tokens. Empty leaves the endpoint open. "+
			"Reads from ASC_MCP_AUTH_TOKENS env var when the flag is unset.")
}

func runServeHTTP(_ *cobra.Command, _ []string) error {
	cfg, err := config.Load()
	if err != nil {
		return err
	}

	opts := server.HTTPOptions{
		AllowedOrigins: splitCSV(httpAllowedOrigins),
		AuthTokens:     splitCSV(authTokensValue()),
	}
	if len(opts.AuthTokens) == 0 {
		log.Printf("WARNING: serve-http running without --auth-tokens; the /mcp endpoint is open to anyone who can reach it.")
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
