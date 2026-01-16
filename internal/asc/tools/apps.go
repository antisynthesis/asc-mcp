// Package tools provides MCP tool implementations for App Store Connect.
package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerAppTools registers app management tools.
func (r *Registry) registerAppTools() {
	r.register(
		mcp.Tool{
			Name:        "list_apps",
			Description: "List all apps in your App Store Connect account. Returns app name, bundle ID, SKU, and primary locale for each app.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"limit": {
						Type:        "integer",
						Description: "Maximum number of apps to return (default: 50, max: 200)",
						Default:     50,
					},
				},
			},
		},
		r.handleListApps,
	)

	r.register(
		mcp.Tool{
			Name:        "get_app",
			Description: "Get detailed information about a specific app by its App Store Connect ID.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"app_id": {
						Type:        "string",
						Description: "The App Store Connect ID of the app",
					},
				},
				Required: []string{"app_id"},
			},
		},
		r.handleGetApp,
	)

	r.register(
		mcp.Tool{
			Name:        "get_app_versions",
			Description: "Get all App Store versions for a specific app, including version string, platform, state, and release information.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"app_id": {
						Type:        "string",
						Description: "The App Store Connect ID of the app",
					},
					"limit": {
						Type:        "integer",
						Description: "Maximum number of versions to return (default: 20)",
						Default:     20,
					},
				},
				Required: []string{"app_id"},
			},
		},
		r.handleGetAppVersions,
	)
}

// handleListApps handles the list_apps tool.
func (r *Registry) handleListApps(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Limit int `json:"limit"`
	}
	params.Limit = 50

	if args != nil {
		if err := json.Unmarshal(args, &params); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
	}

	if params.Limit <= 0 {
		params.Limit = 50
	}
	if params.Limit > 200 {
		params.Limit = 200
	}

	ctx := context.Background()
	resp, err := r.client.ListApps(ctx, params.Limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list apps: %v", err)), nil
	}

	if len(resp.Data) == 0 {
		return mcp.NewSuccessResult("No apps found in your App Store Connect account."), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d apps:\n\n", len(resp.Data)))

	for _, app := range resp.Data {
		sb.WriteString(fmt.Sprintf("**%s**\n", app.Attributes.Name))
		sb.WriteString(fmt.Sprintf("  - ID: %s\n", app.ID))
		sb.WriteString(fmt.Sprintf("  - Bundle ID: %s\n", app.Attributes.BundleID))
		sb.WriteString(fmt.Sprintf("  - SKU: %s\n", app.Attributes.SKU))
		sb.WriteString(fmt.Sprintf("  - Primary Locale: %s\n", app.Attributes.PrimaryLocale))
		sb.WriteString("\n")
	}

	return mcp.NewSuccessResult(sb.String()), nil
}

// handleGetApp handles the get_app tool.
func (r *Registry) handleGetApp(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}

	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}

	ctx := context.Background()
	resp, err := r.client.GetApp(ctx, params.AppID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app: %v", err)), nil
	}

	app := resp.Data
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("**%s**\n\n", app.Attributes.Name))
	sb.WriteString(fmt.Sprintf("- ID: %s\n", app.ID))
	sb.WriteString(fmt.Sprintf("- Bundle ID: %s\n", app.Attributes.BundleID))
	sb.WriteString(fmt.Sprintf("- SKU: %s\n", app.Attributes.SKU))
	sb.WriteString(fmt.Sprintf("- Primary Locale: %s\n", app.Attributes.PrimaryLocale))
	sb.WriteString(fmt.Sprintf("- Made for Kids: %v\n", app.Attributes.IsOrEverWasMadeForKids))

	if app.Attributes.ContentRightsDeclaration != "" {
		sb.WriteString(fmt.Sprintf("- Content Rights: %s\n", app.Attributes.ContentRightsDeclaration))
	}

	return mcp.NewSuccessResult(sb.String()), nil
}

// handleGetAppVersions handles the get_app_versions tool.
func (r *Registry) handleGetAppVersions(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
		Limit int    `json:"limit"`
	}
	params.Limit = 20

	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}

	ctx := context.Background()
	resp, err := r.client.GetAppVersions(ctx, params.AppID, params.Limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app versions: %v", err)), nil
	}

	if len(resp.Data) == 0 {
		return mcp.NewSuccessResult("No versions found for this app."), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d versions:\n\n", len(resp.Data)))

	for _, version := range resp.Data {
		sb.WriteString(fmt.Sprintf("**Version %s** (%s)\n", version.Attributes.VersionString, version.Attributes.Platform))
		sb.WriteString(fmt.Sprintf("  - ID: %s\n", version.ID))
		sb.WriteString(fmt.Sprintf("  - State: %s\n", version.Attributes.AppStoreState))
		sb.WriteString(fmt.Sprintf("  - Release Type: %s\n", version.Attributes.ReleaseType))
		sb.WriteString(fmt.Sprintf("  - Downloadable: %v\n", version.Attributes.Downloadable))
		if version.Attributes.CreatedDate != nil {
			sb.WriteString(fmt.Sprintf("  - Created: %s\n", version.Attributes.CreatedDate.Format("2006-01-02 15:04")))
		}
		sb.WriteString("\n")
	}

	return mcp.NewSuccessResult(sb.String()), nil
}
