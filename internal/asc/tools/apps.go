// Package tools provides MCP tool implementations for App Store Connect.
package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
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
					"cursor": {
						Type:        "string",
						Description: "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page.",
					},
					"filter": {
						Type:        "object",
						Description: "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"platform\": [\"IOS\"]} becomes filter[platform]=IOS.",
					},
					"sort": {
						Type:        "array",
						Description: "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
						Items:       &mcp.Property{Type: "string"},
					},
					"fields": {
						Type:        "object",
						Description: "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return.",
					},
					"include": {
						Type:        "array",
						Description: "Related resource names to include in the response.",
						Items:       &mcp.Property{Type: "string"},
					},
				},
			},
			Annotations: &mcp.ToolAnnotations{
				Title:         "List Apps",
				ReadOnlyHint:  mcp.BoolPtr(true),
				OpenWorldHint: mcp.BoolPtr(true),
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
			Annotations: &mcp.ToolAnnotations{
				Title:         "Get App",
				ReadOnlyHint:  mcp.BoolPtr(true),
				OpenWorldHint: mcp.BoolPtr(true),
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
			Annotations: &mcp.ToolAnnotations{
				Title:         "Get App Versions",
				ReadOnlyHint:  mcp.BoolPtr(true),
				OpenWorldHint: mcp.BoolPtr(true),
			},
		},
		r.handleGetAppVersions,
	)
}

// handleListApps handles the list_apps tool.
func (r *Registry) handleListApps(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Limit   int                 `json:"limit"`
		Cursor  string              `json:"cursor"`
		Filter  map[string][]string `json:"filter"`
		Sort    []string            `json:"sort"`
		Fields  map[string][]string `json:"fields"`
		Include []string            `json:"include"`
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AppsResponse, error) {
		return r.client.ListApps(ctx, listOpts(params.Limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list apps: %v", err)), nil
	}

	if len(resp.Data) == 0 {
		return newListResult("No apps found in your App Store Connect account.", resp.Data, resp.Links), nil
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

	return newListResult(sb.String(), resp.Data, resp.Links), nil
}

// handleGetApp handles the get_app tool.
func (r *Registry) handleGetApp(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}

	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}

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

	return newDataResult(sb.String(), resp.Data), nil
}

// handleGetAppVersions handles the get_app_versions tool.
func (r *Registry) handleGetAppVersions(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := r.client.GetAppVersions(ctx, params.AppID, listOpts(params.Limit, nil, nil, nil, nil))
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app versions: %v", err)), nil
	}

	if len(resp.Data) == 0 {
		return newDataResult("No versions found for this app.", resp.Data), nil
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

	return newDataResult(sb.String(), resp.Data), nil
}
