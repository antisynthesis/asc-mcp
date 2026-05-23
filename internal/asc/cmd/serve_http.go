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
}

func runServeHTTP(_ *cobra.Command, _ []string) error {
	cfg, err := config.Load()
	if err != nil {
		return err
	}

	var origins []string
	if httpAllowedOrigins != "" {
		for _, o := range strings.Split(httpAllowedOrigins, ",") {
			o = strings.TrimSpace(o)
			if o != "" {
				origins = append(origins, o)
			}
		}
	}

	srv, err := server.NewHTTPServer(cfg, origins)
	if err != nil {
		return err
	}

	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()

	log.Printf("serving MCP over HTTP on %s", httpAddr)
	return srv.ListenAndServe(ctx, httpAddr)
}
