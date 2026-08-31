package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerAppTagTools registers Apple-created app tag tools.
func (r *Registry) registerAppTagTools() {
	// List app tags
	r.register(mcp.Tool{
		Name:        "list_app_tags",
		Description: "List the Apple-created tags applied to an app on the App Store",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to list tags for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of tags to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
				"filter": {
					Type:        "object",
					Description: "JSON:API filter map. Supported key: visibleInAppStore. Values are arrays, e.g. {\"visibleInAppStore\": [\"true\"]}.",
				},
				"sort": {
					Type:        "array",
					Description: "Sort fields; prefix with - for descending. Supported: name, -name.",
					Items:       &mcp.Property{Type: "string"},
				},
				"fields": {
					Type:        "object",
					Description: "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return.",
				},
				"include": {
					Type:        "array",
					Description: "Related resource names to include in the response (supported: territories).",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List App Tags",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAppTags)

	// Update app tag
	r.register(mcp.Tool{
		Name:        "update_app_tag",
		Description: "Update an Apple-created app tag's App Store visibility; set visible_in_app_store=false to opt the app out of the tag",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_tag_id": {
					Type:        "string",
					Description: "The app tag ID",
				},
				"visible_in_app_store": {
					Type:        "boolean",
					Description: "Whether the tag should be visible for the app on the App Store",
				},
			},
			Required: []string{"app_tag_id", "visible_in_app_store"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update App Tag",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateAppTag)

	// List app tag territories
	r.register(mcp.Tool{
		Name:        "list_app_tag_territories",
		Description: "List the territories where an Apple-created app tag applies",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_tag_id": {
					Type:        "string",
					Description: "The app tag ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of territories to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
			},
			Required: []string{"app_tag_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List App Tag Territories",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAppTagTerritories)
}

func (r *Registry) handleListAppTags(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID   string              `json:"app_id"`
		Limit   int                 `json:"limit"`
		Cursor  string              `json:"cursor"`
		Filter  map[string][]string `json:"filter"`
		Sort    []string            `json:"sort"`
		Fields  map[string][]string `json:"fields"`
		Include []string            `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AppTagsResponse, error) {
		return r.client.ListAppTags(ctx, params.AppID, listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list app tags: %v", err)), nil
	}

	return newListResult(formatAppTags(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleUpdateAppTag(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppTagID          string `json:"app_tag_id"`
		VisibleInAppStore *bool  `json:"visible_in_app_store"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppTagID == "" {
		return mcp.NewErrorResult("app_tag_id is required"), nil
	}
	if params.VisibleInAppStore == nil {
		return mcp.NewErrorResult("visible_in_app_store is required"), nil
	}

	req := &api.AppTagUpdateRequest{
		Data: api.AppTagUpdateData{
			Type: "appTags",
			ID:   params.AppTagID,
			Attributes: api.AppTagUpdateAttributes{
				VisibleInAppStore: params.VisibleInAppStore,
			},
		},
	}

	resp, err := r.client.UpdateAppTag(ctx, params.AppTagID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update app tag: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("App tag updated:\n%s", formatAppTag(resp.Data)), resp.Data), nil
}

func (r *Registry) handleListAppTagTerritories(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppTagID string `json:"app_tag_id"`
		Limit    int    `json:"limit"`
		Cursor   string `json:"cursor"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppTagID == "" {
		return mcp.NewErrorResult("app_tag_id is required"), nil
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.TerritoriesResponse, error) {
		return r.client.ListAppTagTerritories(ctx, params.AppTagID, listOpts(limit, nil, nil, nil, nil))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list app tag territories: %v", err)), nil
	}

	return newListResult(formatTerritories(resp.Data), resp.Data, resp.Links), nil
}

func formatAppTags(tags []api.AppTag) string {
	if len(tags) == 0 {
		return "No app tags found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d app tags:\n\n", len(tags)))

	for _, tag := range tags {
		sb.WriteString(formatAppTag(tag))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAppTag(tag api.AppTag) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("App Tag ID: %s\n", tag.ID))
	if tag.Attributes.Name != "" {
		sb.WriteString(fmt.Sprintf("Name: %s\n", tag.Attributes.Name))
	}
	if tag.Attributes.VisibleInAppStore != nil {
		sb.WriteString(fmt.Sprintf("Visible In App Store: %t\n", *tag.Attributes.VisibleInAppStore))
	}
	return sb.String()
}
