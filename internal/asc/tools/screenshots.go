package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerScreenshotTools registers screenshot and preview tools.
func (r *Registry) registerScreenshotTools() {
	// List screenshot sets
	r.register(mcp.Tool{
		Name:        "list_screenshot_sets",
		Description: "List screenshot sets for a version localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The version localization ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of sets to return (default 50)",
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
			Required: []string{"localization_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Screenshot Sets",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListScreenshotSets)

	// List screenshots
	r.register(mcp.Tool{
		Name:        "list_screenshots",
		Description: "List screenshots in a screenshot set",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"screenshot_set_id": {
					Type:        "string",
					Description: "The screenshot set ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of screenshots to return (default 50)",
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
			Required: []string{"screenshot_set_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Screenshots",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListScreenshots)

	// Get screenshot
	r.register(mcp.Tool{
		Name:        "get_screenshot",
		Description: "Get details of a specific screenshot",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"screenshot_id": {
					Type:        "string",
					Description: "The screenshot ID",
				},
			},
			Required: []string{"screenshot_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Screenshot",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetScreenshot)

	// Delete screenshot
	r.register(mcp.Tool{
		Name:        "delete_screenshot",
		Description: "Delete a screenshot",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"screenshot_id": {
					Type:        "string",
					Description: "The screenshot ID",
				},
			},
			Required: []string{"screenshot_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Screenshot",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteScreenshot)

	// List preview sets
	r.register(mcp.Tool{
		Name:        "list_preview_sets",
		Description: "List app preview sets for a version localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The version localization ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of sets to return (default 50)",
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
			Required: []string{"localization_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Preview Sets",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListPreviewSets)

	// List previews
	r.register(mcp.Tool{
		Name:        "list_previews",
		Description: "List app previews in a preview set",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"preview_set_id": {
					Type:        "string",
					Description: "The preview set ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of previews to return (default 50)",
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
			Required: []string{"preview_set_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Previews",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListPreviews)

	// Get preview
	r.register(mcp.Tool{
		Name:        "get_preview",
		Description: "Get details of a specific app preview",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"preview_id": {
					Type:        "string",
					Description: "The preview ID",
				},
			},
			Required: []string{"preview_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Preview",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetPreview)

	// Delete preview
	r.register(mcp.Tool{
		Name:        "delete_preview",
		Description: "Delete an app preview",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"preview_id": {
					Type:        "string",
					Description: "The preview ID",
				},
			},
			Required: []string{"preview_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Preview",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeletePreview)
}

func (r *Registry) handleListScreenshotSets(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string              `json:"localization_id"`
		Limit          int                 `json:"limit"`
		Cursor         string              `json:"cursor"`
		Filter         map[string][]string `json:"filter"`
		Sort           []string            `json:"sort"`
		Fields         map[string][]string `json:"fields"`
		Include        []string            `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return nil, fmt.Errorf("localization_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AppScreenshotSetsResponse, error) {
		return r.client.ListAppScreenshotSets(ctx, params.LocalizationID, listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list screenshot sets: %v", err)), nil
	}

	return newListResult(formatScreenshotSets(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleListScreenshots(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ScreenshotSetID string              `json:"screenshot_set_id"`
		Limit           int                 `json:"limit"`
		Cursor          string              `json:"cursor"`
		Filter          map[string][]string `json:"filter"`
		Sort            []string            `json:"sort"`
		Fields          map[string][]string `json:"fields"`
		Include         []string            `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ScreenshotSetID == "" {
		return nil, fmt.Errorf("screenshot_set_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AppScreenshotsResponse, error) {
		return r.client.ListAppScreenshots(ctx, params.ScreenshotSetID, listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list screenshots: %v", err)), nil
	}

	return newListResult(formatScreenshots(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetScreenshot(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ScreenshotID string `json:"screenshot_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ScreenshotID == "" {
		return nil, fmt.Errorf("screenshot_id is required")
	}

	resp, err := r.client.GetAppScreenshot(ctx, params.ScreenshotID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get screenshot: %v", err)), nil
	}

	return newDataResult(formatScreenshot(resp.Data), resp.Data), nil
}

func (r *Registry) handleDeleteScreenshot(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ScreenshotID string `json:"screenshot_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ScreenshotID == "" {
		return nil, fmt.Errorf("screenshot_id is required")
	}

	err := r.client.DeleteAppScreenshot(ctx, params.ScreenshotID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete screenshot: %v", err)), nil
	}

	return mcp.NewSuccessResult("Screenshot deleted successfully"), nil
}

func (r *Registry) handleListPreviewSets(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string              `json:"localization_id"`
		Limit          int                 `json:"limit"`
		Cursor         string              `json:"cursor"`
		Filter         map[string][]string `json:"filter"`
		Sort           []string            `json:"sort"`
		Fields         map[string][]string `json:"fields"`
		Include        []string            `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return nil, fmt.Errorf("localization_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AppPreviewSetsResponse, error) {
		return r.client.ListAppPreviewSets(ctx, params.LocalizationID, listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list preview sets: %v", err)), nil
	}

	return newListResult(formatPreviewSets(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleListPreviews(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PreviewSetID string              `json:"preview_set_id"`
		Limit        int                 `json:"limit"`
		Cursor       string              `json:"cursor"`
		Filter       map[string][]string `json:"filter"`
		Sort         []string            `json:"sort"`
		Fields       map[string][]string `json:"fields"`
		Include      []string            `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PreviewSetID == "" {
		return nil, fmt.Errorf("preview_set_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AppPreviewsResponse, error) {
		return r.client.ListAppPreviews(ctx, params.PreviewSetID, listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list previews: %v", err)), nil
	}

	return newListResult(formatPreviews(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetPreview(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PreviewID string `json:"preview_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PreviewID == "" {
		return nil, fmt.Errorf("preview_id is required")
	}

	resp, err := r.client.GetAppPreview(ctx, params.PreviewID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get preview: %v", err)), nil
	}

	return newDataResult(formatPreview(resp.Data), resp.Data), nil
}

func (r *Registry) handleDeletePreview(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PreviewID string `json:"preview_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PreviewID == "" {
		return nil, fmt.Errorf("preview_id is required")
	}

	err := r.client.DeleteAppPreview(ctx, params.PreviewID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete preview: %v", err)), nil
	}

	return mcp.NewSuccessResult("Preview deleted successfully"), nil
}

func formatScreenshotSets(sets []api.AppScreenshotSet) string {
	if len(sets) == 0 {
		return "No screenshot sets found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d screenshot sets:\n\n", len(sets)))

	for _, set := range sets {
		sb.WriteString(fmt.Sprintf("ID: %s\n", set.ID))
		sb.WriteString(fmt.Sprintf("Display Type: %s\n", set.Attributes.ScreenshotDisplayType))
		sb.WriteString("---\n")
	}

	return sb.String()
}

func formatScreenshots(screenshots []api.AppScreenshot) string {
	if len(screenshots) == 0 {
		return "No screenshots found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d screenshots:\n\n", len(screenshots)))

	for _, ss := range screenshots {
		sb.WriteString(formatScreenshot(ss))
		sb.WriteString("---\n")
	}

	return sb.String()
}

func formatScreenshot(ss api.AppScreenshot) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", ss.ID))
	sb.WriteString(fmt.Sprintf("File Name: %s\n", ss.Attributes.FileName))
	sb.WriteString(fmt.Sprintf("File Size: %d bytes\n", ss.Attributes.FileSize))
	if ss.Attributes.ImageAsset != nil {
		sb.WriteString(fmt.Sprintf("Dimensions: %dx%d\n", ss.Attributes.ImageAsset.Width, ss.Attributes.ImageAsset.Height))
	}
	if ss.Attributes.AssetDeliveryState != nil {
		sb.WriteString(fmt.Sprintf("State: %s\n", ss.Attributes.AssetDeliveryState.State))
	}
	return sb.String()
}

func formatPreviewSets(sets []api.AppPreviewSet) string {
	if len(sets) == 0 {
		return "No preview sets found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d preview sets:\n\n", len(sets)))

	for _, set := range sets {
		sb.WriteString(fmt.Sprintf("ID: %s\n", set.ID))
		sb.WriteString(fmt.Sprintf("Preview Type: %s\n", set.Attributes.PreviewType))
		sb.WriteString("---\n")
	}

	return sb.String()
}

func formatPreviews(previews []api.AppPreview) string {
	if len(previews) == 0 {
		return "No previews found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d previews:\n\n", len(previews)))

	for _, p := range previews {
		sb.WriteString(formatPreview(p))
		sb.WriteString("---\n")
	}

	return sb.String()
}

func formatPreview(p api.AppPreview) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", p.ID))
	sb.WriteString(fmt.Sprintf("File Name: %s\n", p.Attributes.FileName))
	sb.WriteString(fmt.Sprintf("File Size: %d bytes\n", p.Attributes.FileSize))
	if p.Attributes.MimeType != "" {
		sb.WriteString(fmt.Sprintf("MIME Type: %s\n", p.Attributes.MimeType))
	}
	if p.Attributes.VideoURL != "" {
		sb.WriteString(fmt.Sprintf("Video URL: %s\n", p.Attributes.VideoURL))
	}
	if p.Attributes.AssetDeliveryState != nil {
		sb.WriteString(fmt.Sprintf("State: %s\n", p.Attributes.AssetDeliveryState.State))
	}
	return sb.String()
}
