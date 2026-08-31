package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerProductPagesTools registers custom product page and experiment tools.
func (r *Registry) registerProductPagesTools() {
	// List app custom product pages
	r.register(mcp.Tool{
		Name:        "list_app_custom_product_pages",
		Description: "List custom product pages for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of pages to return (default 50)",
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
			Title:         "List App Custom Product Pages",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAppCustomProductPages)

	// Get app custom product page
	r.register(mcp.Tool{
		Name:        "get_app_custom_product_page",
		Description: "Get details of a specific custom product page",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"page_id": {
					Type:        "string",
					Description: "The custom product page ID",
				},
			},
			Required: []string{"page_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get App Custom Product Page",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetAppCustomProductPage)

	// Create app custom product page
	r.register(mcp.Tool{
		Name:        "create_app_custom_product_page",
		Description: "Create a new custom product page",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
				"name": {
					Type:        "string",
					Description: "Name of the custom product page",
				},
			},
			Required: []string{"app_id", "name"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create App Custom Product Page",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateAppCustomProductPage)

	// Update app custom product page
	r.register(mcp.Tool{
		Name:        "update_app_custom_product_page",
		Description: "Update a custom product page",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"page_id": {
					Type:        "string",
					Description: "The custom product page ID",
				},
				"name": {
					Type:        "string",
					Description: "New name for the page",
				},
				"visible": {
					Type:        "boolean",
					Description: "Whether the page is visible",
				},
			},
			Required: []string{"page_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update App Custom Product Page",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateAppCustomProductPage)

	// Delete app custom product page
	r.register(mcp.Tool{
		Name:        "delete_app_custom_product_page",
		Description: "Delete a custom product page",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"page_id": {
					Type:        "string",
					Description: "The custom product page ID to delete",
				},
			},
			Required: []string{"page_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete App Custom Product Page",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteAppCustomProductPage)

	// List app store version experiments
	r.register(mcp.Tool{
		Name:        "list_app_store_version_experiments",
		Description: "List A/B testing experiments for an app or an app store version. Provide app_id or version_id.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to list experiments for (preferred; v2 experiments attach to an app)",
				},
				"version_id": {
					Type:        "string",
					Description: "The app store version ID to list experiments for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of experiments to return (default 50)",
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
			Title:         "List App Store Version Experiments",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAppStoreVersionExperiments)

	// Get app store version experiment
	r.register(mcp.Tool{
		Name:        "get_app_store_version_experiment",
		Description: "Get details of a specific experiment",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"experiment_id": {
					Type:        "string",
					Description: "The experiment ID",
				},
			},
			Required: []string{"experiment_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get App Store Version Experiment",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetAppStoreVersionExperiment)

	// Create app store version experiment
	r.register(mcp.Tool{
		Name:        "create_app_store_version_experiment",
		Description: "Create a new A/B testing experiment for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID (v2 experiments attach to an app, not a version)",
				},
				"name": {
					Type:        "string",
					Description: "Name of the experiment",
				},
				"platform": {
					Type:        "string",
					Description: "Platform for the experiment (IOS, MAC_OS, TV_OS, VISION_OS)",
				},
				"traffic_proportion": {
					Type:        "integer",
					Description: "Percentage of traffic for the experiment (1-100)",
				},
			},
			Required: []string{"app_id", "name", "platform"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create App Store Version Experiment",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateAppStoreVersionExperiment)

	// Update app store version experiment
	r.register(mcp.Tool{
		Name:        "update_app_store_version_experiment",
		Description: "Update an experiment",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"experiment_id": {
					Type:        "string",
					Description: "The experiment ID",
				},
				"name": {
					Type:        "string",
					Description: "New name for the experiment",
				},
				"traffic_proportion": {
					Type:        "integer",
					Description: "Percentage of traffic for the experiment (1-100)",
				},
				"started": {
					Type:        "boolean",
					Description: "Whether the experiment is running",
				},
			},
			Required: []string{"experiment_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update App Store Version Experiment",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateAppStoreVersionExperiment)

	// Delete app store version experiment
	r.register(mcp.Tool{
		Name:        "delete_app_store_version_experiment",
		Description: "Delete an experiment",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"experiment_id": {
					Type:        "string",
					Description: "The experiment ID to delete",
				},
			},
			Required: []string{"experiment_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete App Store Version Experiment",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteAppStoreVersionExperiment)
}

func (r *Registry) handleListAppCustomProductPages(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AppCustomProductPagesResponse, error) {
		return r.client.ListAppCustomProductPages(ctx, params.AppID, listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list custom product pages: %v", err)), nil
	}

	return newListResult(formatAppCustomProductPages(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetAppCustomProductPage(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PageID string `json:"page_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PageID == "" {
		return mcp.NewErrorResult("page_id is required"), nil
	}

	resp, err := r.client.GetAppCustomProductPage(ctx, params.PageID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get custom product page: %v", err)), nil
	}

	return newDataResult(formatAppCustomProductPage(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateAppCustomProductPage(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
		Name  string `json:"name"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" || params.Name == "" {
		return mcp.NewErrorResult("app_id and name are required"), nil
	}

	req := &api.AppCustomProductPageCreateRequest{
		Data: api.AppCustomProductPageCreateData{
			Type: "appCustomProductPages",
			Attributes: api.AppCustomProductPageCreateAttributes{
				Name: params.Name,
			},
			Relationships: api.AppCustomProductPageCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "apps", ID: params.AppID},
				},
			},
		},
	}

	resp, err := r.client.CreateAppCustomProductPage(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create custom product page: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Custom product page created:\n%s", formatAppCustomProductPage(resp.Data)), resp.Data), nil
}

func (r *Registry) handleUpdateAppCustomProductPage(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PageID  string `json:"page_id"`
		Name    string `json:"name"`
		Visible *bool  `json:"visible"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PageID == "" {
		return mcp.NewErrorResult("page_id is required"), nil
	}

	req := &api.AppCustomProductPageUpdateRequest{
		Data: api.AppCustomProductPageUpdateData{
			Type: "appCustomProductPages",
			ID:   params.PageID,
			Attributes: api.AppCustomProductPageUpdateAttributes{
				Name:    params.Name,
				Visible: params.Visible,
			},
		},
	}

	resp, err := r.client.UpdateAppCustomProductPage(ctx, params.PageID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update custom product page: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Custom product page updated:\n%s", formatAppCustomProductPage(resp.Data)), resp.Data), nil
}

func (r *Registry) handleDeleteAppCustomProductPage(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PageID string `json:"page_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PageID == "" {
		return mcp.NewErrorResult("page_id is required"), nil
	}

	err := r.client.DeleteAppCustomProductPage(ctx, params.PageID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete custom product page: %v", err)), nil
	}

	return mcp.NewSuccessResult("Custom product page deleted"), nil
}

func (r *Registry) handleListAppStoreVersionExperiments(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID     string              `json:"app_id"`
		VersionID string              `json:"version_id"`
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

	if params.AppID == "" && params.VersionID == "" {
		return mcp.NewErrorResult("app_id or version_id is required"), nil
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AppStoreVersionExperimentsResponse, error) {
		opts := listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include)
		if params.VersionID != "" {
			return r.client.ListAppStoreVersionExperiments(ctx, params.VersionID, opts)
		}
		return r.client.ListAppStoreVersionExperimentsForApp(ctx, params.AppID, opts)
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list experiments: %v", err)), nil
	}

	return newListResult(formatAppStoreVersionExperiments(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetAppStoreVersionExperiment(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ExperimentID string `json:"experiment_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ExperimentID == "" {
		return mcp.NewErrorResult("experiment_id is required"), nil
	}

	resp, err := r.client.GetAppStoreVersionExperiment(ctx, params.ExperimentID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get experiment: %v", err)), nil
	}

	return newDataResult(formatAppStoreVersionExperiment(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateAppStoreVersionExperiment(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID             string `json:"app_id"`
		Name              string `json:"name"`
		Platform          string `json:"platform"`
		TrafficProportion int    `json:"traffic_proportion"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" || params.Name == "" || params.Platform == "" {
		return mcp.NewErrorResult("app_id, name, and platform are required"), nil
	}

	traffic := params.TrafficProportion
	if traffic <= 0 {
		traffic = 50
	}

	req := &api.AppStoreVersionExperimentCreateRequest{
		Data: api.AppStoreVersionExperimentCreateData{
			Type: "appStoreVersionExperiments",
			Attributes: api.AppStoreVersionExperimentCreateAttributes{
				Name:              params.Name,
				Platform:          params.Platform,
				TrafficProportion: traffic,
			},
			Relationships: api.AppStoreVersionExperimentCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "apps", ID: params.AppID},
				},
			},
		},
	}

	resp, err := r.client.CreateAppStoreVersionExperiment(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create experiment: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Experiment created:\n%s", formatAppStoreVersionExperiment(resp.Data)), resp.Data), nil
}

func (r *Registry) handleUpdateAppStoreVersionExperiment(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ExperimentID      string `json:"experiment_id"`
		Name              string `json:"name"`
		TrafficProportion *int   `json:"traffic_proportion"`
		Started           *bool  `json:"started"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ExperimentID == "" {
		return mcp.NewErrorResult("experiment_id is required"), nil
	}

	req := &api.AppStoreVersionExperimentUpdateRequest{
		Data: api.AppStoreVersionExperimentUpdateData{
			Type: "appStoreVersionExperiments",
			ID:   params.ExperimentID,
			Attributes: api.AppStoreVersionExperimentUpdateAttributes{
				Name:              params.Name,
				TrafficProportion: params.TrafficProportion,
				Started:           params.Started,
			},
		},
	}

	resp, err := r.client.UpdateAppStoreVersionExperiment(ctx, params.ExperimentID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update experiment: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Experiment updated:\n%s", formatAppStoreVersionExperiment(resp.Data)), resp.Data), nil
}

func (r *Registry) handleDeleteAppStoreVersionExperiment(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ExperimentID string `json:"experiment_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ExperimentID == "" {
		return mcp.NewErrorResult("experiment_id is required"), nil
	}

	err := r.client.DeleteAppStoreVersionExperiment(ctx, params.ExperimentID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete experiment: %v", err)), nil
	}

	return mcp.NewSuccessResult("Experiment deleted"), nil
}

func formatAppCustomProductPages(pages []api.AppCustomProductPage) string {
	if len(pages) == 0 {
		return "No custom product pages found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d custom product pages:\n\n", len(pages)))

	for _, page := range pages {
		sb.WriteString(formatAppCustomProductPage(page))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAppCustomProductPage(page api.AppCustomProductPage) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", page.ID))
	sb.WriteString(fmt.Sprintf("Name: %s\n", page.Attributes.Name))
	if page.Attributes.URL != "" {
		sb.WriteString(fmt.Sprintf("URL: %s\n", page.Attributes.URL))
	}
	sb.WriteString(fmt.Sprintf("Visible: %t\n", page.Attributes.Visible))
	return sb.String()
}

func formatAppStoreVersionExperiments(experiments []api.AppStoreVersionExperiment) string {
	if len(experiments) == 0 {
		return "No experiments found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d experiments:\n\n", len(experiments)))

	for _, exp := range experiments {
		sb.WriteString(formatAppStoreVersionExperiment(exp))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAppStoreVersionExperiment(exp api.AppStoreVersionExperiment) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", exp.ID))
	sb.WriteString(fmt.Sprintf("Name: %s\n", exp.Attributes.Name))
	sb.WriteString(fmt.Sprintf("State: %s\n", exp.Attributes.State))
	sb.WriteString(fmt.Sprintf("Traffic Proportion: %d%%\n", exp.Attributes.TrafficProportion))
	if exp.Attributes.StartDate != nil {
		sb.WriteString(fmt.Sprintf("Start Date: %s\n", exp.Attributes.StartDate.Format("2006-01-02")))
	}
	if exp.Attributes.EndDate != nil {
		sb.WriteString(fmt.Sprintf("End Date: %s\n", exp.Attributes.EndDate.Format("2006-01-02")))
	}
	return sb.String()
}
