// Package cmd provides the command-line interface for asc-mcp.
package cmd

import (
	"log"
	"os"

	"github.com/spf13/cobra"

	"github.com/antisynthesis/asc-mcp/internal/asc/config"
	"github.com/antisynthesis/asc-mcp/internal/asc/server"
)

var serveCmd = &cobra.Command{
	Use:   "serve",
	Short: "Start the MCP server",
	Long: `Start the MCP server and listen for JSON-RPC requests on stdin/stdout.

The server requires App Store Connect API credentials to be configured
via environment variables:

  ASC_ISSUER_ID        Your App Store Connect API Issuer ID
  ASC_KEY_ID           Your App Store Connect API Key ID
  ASC_PRIVATE_KEY_PATH Path to your .p8 private key file

Example:
  export ASC_ISSUER_ID="xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
  export ASC_KEY_ID="XXXXXXXXXX"
  export ASC_PRIVATE_KEY_PATH="/path/to/AuthKey.p8"
  asc-mcp serve`,
	RunE: runServe,
}

func runServe(cmd *cobra.Command, args []string) error {
	cfg, err := config.Load()
	if err != nil {
		return err
	}

	srv, err := server.New(cfg, os.Stdin, os.Stdout)
	if err != nil {
		return err
	}

	log.Printf("starting MCP server")
	return srv.Run()
}
