// Package tools provides MCP tool implementations for App Store Connect.
package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerBuildTools registers build management tools.
func (r *Registry) registerBuildTools() {
	r.register(
		mcp.Tool{
			Name:        "list_builds",
			Description: "List builds for your apps. Can filter by app ID. Returns version, processing state, upload date, and expiration information.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"app_id": {
						Type:        "string",
						Description: "Optional: Filter builds by app ID",
					},
					"limit": {
						Type:        "integer",
						Description: "Maximum number of builds to return (default: 20, max: 200)",
						Default:     20,
					},
				},
			},
		},
		r.handleListBuilds,
	)

	r.register(
		mcp.Tool{
			Name:        "get_build",
			Description: "Get detailed information about a specific build by its ID, including version, processing state, and TestFlight information.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"build_id": {
						Type:        "string",
						Description: "The App Store Connect ID of the build",
					},
				},
				Required: []string{"build_id"},
			},
		},
		r.handleGetBuild,
	)
}

// handleListBuilds handles the list_builds tool.
func (r *Registry) handleListBuilds(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
		Limit int    `json:"limit"`
	}
	params.Limit = 20

	if args != nil {
		if err := json.Unmarshal(args, &params); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
	}

	if params.Limit <= 0 {
		params.Limit = 20
	}
	if params.Limit > 200 {
		params.Limit = 200
	}

	ctx := context.Background()
	resp, err := r.client.ListBuilds(ctx, params.AppID, params.Limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list builds: %v", err)), nil
	}

	if len(resp.Data) == 0 {
		return mcp.NewSuccessResult("No builds found."), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d builds:\n\n", len(resp.Data)))

	for _, build := range resp.Data {
		sb.WriteString(fmt.Sprintf("**Build %s**\n", build.Attributes.Version))
		sb.WriteString(fmt.Sprintf("  - ID: %s\n", build.ID))
		sb.WriteString(fmt.Sprintf("  - Processing State: %s\n", build.Attributes.ProcessingState))
		sb.WriteString(fmt.Sprintf("  - Min OS Version: %s\n", build.Attributes.MinOsVersion))
		sb.WriteString(fmt.Sprintf("  - Expired: %v\n", build.Attributes.Expired))
		if build.Attributes.UploadedDate != nil {
			sb.WriteString(fmt.Sprintf("  - Uploaded: %s\n", build.Attributes.UploadedDate.Format("2006-01-02 15:04")))
		}
		if build.Attributes.ExpirationDate != nil {
			sb.WriteString(fmt.Sprintf("  - Expires: %s\n", build.Attributes.ExpirationDate.Format("2006-01-02")))
		}
		sb.WriteString("\n")
	}

	return mcp.NewSuccessResult(sb.String()), nil
}

// handleGetBuild handles the get_build tool.
func (r *Registry) handleGetBuild(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BuildID string `json:"build_id"`
	}

	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BuildID == "" {
		return mcp.NewErrorResult("build_id is required"), nil
	}

	ctx := context.Background()
	resp, err := r.client.GetBuild(ctx, params.BuildID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get build: %v", err)), nil
	}

	build := resp.Data
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("**Build %s**\n\n", build.Attributes.Version))
	sb.WriteString(fmt.Sprintf("- ID: %s\n", build.ID))
	sb.WriteString(fmt.Sprintf("- Processing State: %s\n", build.Attributes.ProcessingState))
	sb.WriteString(fmt.Sprintf("- Min OS Version: %s\n", build.Attributes.MinOsVersion))
	sb.WriteString(fmt.Sprintf("- Build Audience Type: %s\n", build.Attributes.BuildAudienceType))
	sb.WriteString(fmt.Sprintf("- Uses Non-Exempt Encryption: %v\n", build.Attributes.UsesNonExemptEncryption))
	sb.WriteString(fmt.Sprintf("- Expired: %v\n", build.Attributes.Expired))

	if build.Attributes.UploadedDate != nil {
		sb.WriteString(fmt.Sprintf("- Uploaded: %s\n", build.Attributes.UploadedDate.Format("2006-01-02 15:04:05")))
	}
	if build.Attributes.ExpirationDate != nil {
		sb.WriteString(fmt.Sprintf("- Expires: %s\n", build.Attributes.ExpirationDate.Format("2006-01-02")))
	}

	return mcp.NewSuccessResult(sb.String()), nil
}
