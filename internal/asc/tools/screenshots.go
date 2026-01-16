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
			},
			Required: []string{"localization_id"},
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
			},
			Required: []string{"screenshot_set_id"},
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
			},
			Required: []string{"localization_id"},
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
			},
			Required: []string{"preview_set_id"},
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
	}, r.handleDeletePreview)
}

func (r *Registry) handleListScreenshotSets(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
		Limit          int    `json:"limit"`
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

	resp, err := r.client.ListAppScreenshotSets(context.Background(), params.LocalizationID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list screenshot sets: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatScreenshotSets(resp.Data)), nil
}

func (r *Registry) handleListScreenshots(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ScreenshotSetID string `json:"screenshot_set_id"`
		Limit           int    `json:"limit"`
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

	resp, err := r.client.ListAppScreenshots(context.Background(), params.ScreenshotSetID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list screenshots: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatScreenshots(resp.Data)), nil
}

func (r *Registry) handleGetScreenshot(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ScreenshotID string `json:"screenshot_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ScreenshotID == "" {
		return nil, fmt.Errorf("screenshot_id is required")
	}

	resp, err := r.client.GetAppScreenshot(context.Background(), params.ScreenshotID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get screenshot: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatScreenshot(resp.Data)), nil
}

func (r *Registry) handleDeleteScreenshot(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ScreenshotID string `json:"screenshot_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ScreenshotID == "" {
		return nil, fmt.Errorf("screenshot_id is required")
	}

	err := r.client.DeleteAppScreenshot(context.Background(), params.ScreenshotID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete screenshot: %v", err)), nil
	}

	return mcp.NewSuccessResult("Screenshot deleted successfully"), nil
}

func (r *Registry) handleListPreviewSets(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
		Limit          int    `json:"limit"`
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

	resp, err := r.client.ListAppPreviewSets(context.Background(), params.LocalizationID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list preview sets: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatPreviewSets(resp.Data)), nil
}

func (r *Registry) handleListPreviews(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PreviewSetID string `json:"preview_set_id"`
		Limit        int    `json:"limit"`
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

	resp, err := r.client.ListAppPreviews(context.Background(), params.PreviewSetID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list previews: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatPreviews(resp.Data)), nil
}

func (r *Registry) handleGetPreview(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PreviewID string `json:"preview_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PreviewID == "" {
		return nil, fmt.Errorf("preview_id is required")
	}

	resp, err := r.client.GetAppPreview(context.Background(), params.PreviewID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get preview: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatPreview(resp.Data)), nil
}

func (r *Registry) handleDeletePreview(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PreviewID string `json:"preview_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PreviewID == "" {
		return nil, fmt.Errorf("preview_id is required")
	}

	err := r.client.DeleteAppPreview(context.Background(), params.PreviewID)
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
