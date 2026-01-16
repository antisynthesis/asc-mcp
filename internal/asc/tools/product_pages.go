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
			},
			Required: []string{"app_id"},
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
	}, r.handleDeleteAppCustomProductPage)

	// List app store version experiments
	r.register(mcp.Tool{
		Name:        "list_app_store_version_experiments",
		Description: "List A/B testing experiments for an app store version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The app store version ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of experiments to return (default 50)",
				},
			},
			Required: []string{"version_id"},
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
	}, r.handleGetAppStoreVersionExperiment)

	// Create app store version experiment
	r.register(mcp.Tool{
		Name:        "create_app_store_version_experiment",
		Description: "Create a new A/B testing experiment",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The app store version ID",
				},
				"name": {
					Type:        "string",
					Description: "Name of the experiment",
				},
				"traffic_proportion": {
					Type:        "integer",
					Description: "Percentage of traffic for the experiment (1-100)",
				},
			},
			Required: []string{"version_id", "name"},
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
	}, r.handleDeleteAppStoreVersionExperiment)
}

func (r *Registry) handleListAppCustomProductPages(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
		Limit int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListAppCustomProductPages(context.Background(), params.AppID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list custom product pages: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppCustomProductPages(resp.Data)), nil
}

func (r *Registry) handleGetAppCustomProductPage(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PageID string `json:"page_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PageID == "" {
		return nil, fmt.Errorf("page_id is required")
	}

	resp, err := r.client.GetAppCustomProductPage(context.Background(), params.PageID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get custom product page: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppCustomProductPage(resp.Data)), nil
}

func (r *Registry) handleCreateAppCustomProductPage(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
		Name  string `json:"name"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" || params.Name == "" {
		return nil, fmt.Errorf("app_id and name are required")
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

	resp, err := r.client.CreateAppCustomProductPage(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create custom product page: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Custom product page created:\n%s", formatAppCustomProductPage(resp.Data))), nil
}

func (r *Registry) handleUpdateAppCustomProductPage(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PageID  string `json:"page_id"`
		Name    string `json:"name"`
		Visible *bool  `json:"visible"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PageID == "" {
		return nil, fmt.Errorf("page_id is required")
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

	resp, err := r.client.UpdateAppCustomProductPage(context.Background(), params.PageID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update custom product page: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Custom product page updated:\n%s", formatAppCustomProductPage(resp.Data))), nil
}

func (r *Registry) handleDeleteAppCustomProductPage(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PageID string `json:"page_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PageID == "" {
		return nil, fmt.Errorf("page_id is required")
	}

	err := r.client.DeleteAppCustomProductPage(context.Background(), params.PageID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete custom product page: %v", err)), nil
	}

	return mcp.NewSuccessResult("Custom product page deleted"), nil
}

func (r *Registry) handleListAppStoreVersionExperiments(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
		Limit     int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return nil, fmt.Errorf("version_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListAppStoreVersionExperiments(context.Background(), params.VersionID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list experiments: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppStoreVersionExperiments(resp.Data)), nil
}

func (r *Registry) handleGetAppStoreVersionExperiment(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ExperimentID string `json:"experiment_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ExperimentID == "" {
		return nil, fmt.Errorf("experiment_id is required")
	}

	resp, err := r.client.GetAppStoreVersionExperiment(context.Background(), params.ExperimentID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get experiment: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppStoreVersionExperiment(resp.Data)), nil
}

func (r *Registry) handleCreateAppStoreVersionExperiment(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID         string `json:"version_id"`
		Name              string `json:"name"`
		TrafficProportion int    `json:"traffic_proportion"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" || params.Name == "" {
		return nil, fmt.Errorf("version_id and name are required")
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
				TrafficProportion: traffic,
			},
			Relationships: api.AppStoreVersionExperimentCreateRelationships{
				AppStoreVersion: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "appStoreVersions", ID: params.VersionID},
				},
			},
		},
	}

	resp, err := r.client.CreateAppStoreVersionExperiment(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create experiment: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Experiment created:\n%s", formatAppStoreVersionExperiment(resp.Data))), nil
}

func (r *Registry) handleUpdateAppStoreVersionExperiment(args json.RawMessage) (*mcp.ToolsCallResult, error) {
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
		return nil, fmt.Errorf("experiment_id is required")
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

	resp, err := r.client.UpdateAppStoreVersionExperiment(context.Background(), params.ExperimentID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update experiment: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Experiment updated:\n%s", formatAppStoreVersionExperiment(resp.Data))), nil
}

func (r *Registry) handleDeleteAppStoreVersionExperiment(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ExperimentID string `json:"experiment_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ExperimentID == "" {
		return nil, fmt.Errorf("experiment_id is required")
	}

	err := r.client.DeleteAppStoreVersionExperiment(context.Background(), params.ExperimentID)
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
