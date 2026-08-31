package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerAppClipTools registers app clip tools.
func (r *Registry) registerAppClipTools() {
	// List app clips
	r.register(mcp.Tool{
		Name:        "list_app_clips",
		Description: "List app clips for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of app clips to return (default 50)",
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
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List App Clips",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAppClips)

	// Get app clip
	r.register(mcp.Tool{
		Name:        "get_app_clip",
		Description: "Get details of a specific app clip",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_clip_id": {
					Type:        "string",
					Description: "The app clip ID",
				},
			},
			Required: []string{"app_clip_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get App Clip",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetAppClip)

	// List app clip default experiences
	r.register(mcp.Tool{
		Name:        "list_app_clip_default_experiences",
		Description: "List default experiences for an app clip",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_clip_id": {
					Type:        "string",
					Description: "The app clip ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of experiences to return (default 50)",
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
			Required: []string{"app_clip_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List App Clip Default Experiences",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAppClipDefaultExperiences)

	// Get app clip default experience
	r.register(mcp.Tool{
		Name:        "get_app_clip_default_experience",
		Description: "Get details of a specific app clip default experience",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"experience_id": {
					Type:        "string",
					Description: "The default experience ID",
				},
			},
			Required: []string{"experience_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get App Clip Default Experience",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetAppClipDefaultExperience)

	// List app clip advanced experiences
	r.register(mcp.Tool{
		Name:        "list_app_clip_advanced_experiences",
		Description: "List advanced experiences for an app clip",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_clip_id": {
					Type:        "string",
					Description: "The app clip ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of experiences to return (default 50)",
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
			Required: []string{"app_clip_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List App Clip Advanced Experiences",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAppClipAdvancedExperiences)

	// Get app clip advanced experience
	r.register(mcp.Tool{
		Name:        "get_app_clip_advanced_experience",
		Description: "Get details of a specific app clip advanced experience",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"experience_id": {
					Type:        "string",
					Description: "The advanced experience ID",
				},
			},
			Required: []string{"experience_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get App Clip Advanced Experience",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetAppClipAdvancedExperience)
}

func (r *Registry) handleListAppClips(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AppClipsResponse, error) {
		return r.client.ListAppClips(ctx, params.AppID, listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list app clips: %v", err)), nil
	}

	return newListResult(formatAppClips(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetAppClip(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppClipID string `json:"app_clip_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppClipID == "" {
		return mcp.NewErrorResult("app_clip_id is required"), nil
	}

	resp, err := r.client.GetAppClip(ctx, params.AppClipID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app clip: %v", err)), nil
	}

	return newDataResult(formatAppClip(resp.Data), resp.Data), nil
}

func (r *Registry) handleListAppClipDefaultExperiences(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppClipID string              `json:"app_clip_id"`
		Limit     int                 `json:"limit"`
		Cursor    string              `json:"cursor"`
		Filter    map[string][]string `json:"filter"`
		Sort      []string            `json:"sort"`
		Fields    map[string][]string `json:"fields"`
		Include   []string            `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppClipID == "" {
		return mcp.NewErrorResult("app_clip_id is required"), nil
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AppClipDefaultExperiencesResponse, error) {
		return r.client.ListAppClipDefaultExperiences(ctx, params.AppClipID, listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list app clip default experiences: %v", err)), nil
	}

	return newListResult(formatAppClipDefaultExperiences(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetAppClipDefaultExperience(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ExperienceID string `json:"experience_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ExperienceID == "" {
		return mcp.NewErrorResult("experience_id is required"), nil
	}

	resp, err := r.client.GetAppClipDefaultExperience(ctx, params.ExperienceID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app clip default experience: %v", err)), nil
	}

	return newDataResult(formatAppClipDefaultExperience(resp.Data), resp.Data), nil
}

func (r *Registry) handleListAppClipAdvancedExperiences(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppClipID string              `json:"app_clip_id"`
		Limit     int                 `json:"limit"`
		Cursor    string              `json:"cursor"`
		Filter    map[string][]string `json:"filter"`
		Sort      []string            `json:"sort"`
		Fields    map[string][]string `json:"fields"`
		Include   []string            `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppClipID == "" {
		return mcp.NewErrorResult("app_clip_id is required"), nil
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AppClipAdvancedExperiencesResponse, error) {
		return r.client.ListAppClipAdvancedExperiences(ctx, params.AppClipID, listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list app clip advanced experiences: %v", err)), nil
	}

	return newListResult(formatAppClipAdvancedExperiences(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetAppClipAdvancedExperience(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ExperienceID string `json:"experience_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ExperienceID == "" {
		return mcp.NewErrorResult("experience_id is required"), nil
	}

	resp, err := r.client.GetAppClipAdvancedExperience(ctx, params.ExperienceID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app clip advanced experience: %v", err)), nil
	}

	return newDataResult(formatAppClipAdvancedExperience(resp.Data), resp.Data), nil
}

func formatAppClips(clips []api.AppClip) string {
	if len(clips) == 0 {
		return "No app clips found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d app clips:\n\n", len(clips)))

	for _, clip := range clips {
		sb.WriteString(formatAppClip(clip))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAppClip(clip api.AppClip) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", clip.ID))
	sb.WriteString(fmt.Sprintf("Bundle ID: %s\n", clip.Attributes.BundleID))
	return sb.String()
}

func formatAppClipDefaultExperiences(experiences []api.AppClipDefaultExperience) string {
	if len(experiences) == 0 {
		return "No app clip default experiences found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d default experiences:\n\n", len(experiences)))

	for _, exp := range experiences {
		sb.WriteString(formatAppClipDefaultExperience(exp))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAppClipDefaultExperience(exp api.AppClipDefaultExperience) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", exp.ID))
	if exp.Attributes.Action != "" {
		sb.WriteString(fmt.Sprintf("Action: %s\n", exp.Attributes.Action))
	}
	return sb.String()
}

func formatAppClipAdvancedExperiences(experiences []api.AppClipAdvancedExperience) string {
	if len(experiences) == 0 {
		return "No app clip advanced experiences found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d advanced experiences:\n\n", len(experiences)))

	for _, exp := range experiences {
		sb.WriteString(formatAppClipAdvancedExperience(exp))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAppClipAdvancedExperience(exp api.AppClipAdvancedExperience) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", exp.ID))
	if exp.Attributes.Action != "" {
		sb.WriteString(fmt.Sprintf("Action: %s\n", exp.Attributes.Action))
	}
	if exp.Attributes.Link != "" {
		sb.WriteString(fmt.Sprintf("Link: %s\n", exp.Attributes.Link))
	}
	if exp.Attributes.Status != "" {
		sb.WriteString(fmt.Sprintf("Status: %s\n", exp.Attributes.Status))
	}
	if exp.Attributes.BusinessCategory != "" {
		sb.WriteString(fmt.Sprintf("Business Category: %s\n", exp.Attributes.BusinessCategory))
	}
	if exp.Attributes.DefaultLanguage != "" {
		sb.WriteString(fmt.Sprintf("Default Language: %s\n", exp.Attributes.DefaultLanguage))
	}
	sb.WriteString(fmt.Sprintf("Is Powered By: %t\n", exp.Attributes.IsPoweredBy))
	return sb.String()
}
