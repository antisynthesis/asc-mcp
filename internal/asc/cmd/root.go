// Package cmd provides the command-line interface for asc-mcp.
package cmd

import (
	"os"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "asc-mcp",
	Short: "MCP server for Apple App Store Connect",
	Long: `asc-mcp is a Model Context Protocol (MCP) server that provides
tools for managing Apple App Store Connect resources.

It supports app management, build management, TestFlight beta testing,
and provisioning operations.`,
}

// Execute runs the root command.
func Execute() {
	if err := rootCmd.Execute(); err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.AddCommand(serveCmd)
	rootCmd.AddCommand(versionCmd)
	rootCmd.AddCommand(validateCmd)
	rootCmd.AddCommand(toolsCmd)
}
