// Package cmd provides the command-line interface for asc-mcp.
package cmd

import (
	"fmt"

	"github.com/spf13/cobra"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/tools"
)

var toolsCmd = &cobra.Command{
	Use:   "tools",
	Short: "List available MCP tools",
	Long: `List all MCP tools that are available when the server is running.

This command displays the tool names and descriptions without
requiring API credentials.`,
	Run: runTools,
}

func runTools(cmd *cobra.Command, args []string) {
	// Create a nil client registry just to list tools
	// The actual API calls won't work, but we can still list tool definitions
	registry := tools.NewRegistry((*api.Client)(nil))
	toolList := registry.ListTools()

	fmt.Printf("Available MCP Tools (%d total):\n\n", len(toolList))

	// Group by category
	categories := map[string][]string{
		"App Management": {},
		"Builds":         {},
		"TestFlight":     {},
		"Provisioning":   {},
	}

	for _, tool := range toolList {
		switch tool.Name {
		case "list_apps", "get_app", "get_app_versions":
			categories["App Management"] = append(categories["App Management"], fmt.Sprintf("  %s - %s", tool.Name, tool.Description))
		case "list_builds", "get_build":
			categories["Builds"] = append(categories["Builds"], fmt.Sprintf("  %s - %s", tool.Name, tool.Description))
		case "list_beta_groups", "create_beta_group", "delete_beta_group", "list_beta_testers", "invite_beta_tester", "remove_beta_tester", "add_tester_to_group":
			categories["TestFlight"] = append(categories["TestFlight"], fmt.Sprintf("  %s - %s", tool.Name, tool.Description))
		default:
			categories["Provisioning"] = append(categories["Provisioning"], fmt.Sprintf("  %s - %s", tool.Name, tool.Description))
		}
	}

	order := []string{"App Management", "Builds", "TestFlight", "Provisioning"}
	for _, cat := range order {
		if len(categories[cat]) > 0 {
			fmt.Printf("%s:\n", cat)
			for _, t := range categories[cat] {
				fmt.Println(t)
			}
			fmt.Println()
		}
	}
}
