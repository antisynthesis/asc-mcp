package tools

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerPreOrderTools registers pre-order tools.
func (r *Registry) registerPreOrderTools() {
	// Get pre-order
	r.register(mcp.Tool{
		Name:        "get_pre_order",
		Description: "Get pre-order info for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleGetPreOrder)

	// Create pre-order
	r.register(mcp.Tool{
		Name:        "create_pre_order",
		Description: "Enable pre-order for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID",
				},
				"app_release_date": {
					Type:        "string",
					Description: "The planned release date (YYYY-MM-DD)",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleCreatePreOrder)

	// Update pre-order
	r.register(mcp.Tool{
		Name:        "update_pre_order",
		Description: "Update pre-order release date",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"pre_order_id": {
					Type:        "string",
					Description: "The pre-order ID",
				},
				"app_release_date": {
					Type:        "string",
					Description: "The updated release date (YYYY-MM-DD)",
				},
			},
			Required: []string{"pre_order_id"},
		},
	}, r.handleUpdatePreOrder)

	// Delete pre-order
	r.register(mcp.Tool{
		Name:        "delete_pre_order",
		Description: "Disable pre-order for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"pre_order_id": {
					Type:        "string",
					Description: "The pre-order ID",
				},
			},
			Required: []string{"pre_order_id"},
		},
	}, r.handleDeletePreOrder)
}

func (r *Registry) handleGetPreOrder(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}

	resp, err := r.client.GetAppPreOrder(context.Background(), params.AppID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get pre-order: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatPreOrder(resp.Data)), nil
}

func (r *Registry) handleCreatePreOrder(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID          string `json:"app_id"`
		AppReleaseDate string `json:"app_release_date"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}

	req := &api.AppPreOrderCreateRequest{
		Data: api.AppPreOrderCreateData{
			Type: "appPreOrders",
			Attributes: api.AppPreOrderCreateAttributes{
				AppReleaseDate: params.AppReleaseDate,
			},
			Relationships: api.AppPreOrderCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "apps",
						ID:   params.AppID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateAppPreOrder(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create pre-order: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Created pre-order: %s", resp.Data.ID)), nil
}

func (r *Registry) handleUpdatePreOrder(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PreOrderID     string `json:"pre_order_id"`
		AppReleaseDate string `json:"app_release_date"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PreOrderID == "" {
		return nil, fmt.Errorf("pre_order_id is required")
	}

	req := &api.AppPreOrderUpdateRequest{
		Data: api.AppPreOrderUpdateData{
			Type: "appPreOrders",
			ID:   params.PreOrderID,
			Attributes: api.AppPreOrderUpdateAttributes{
				AppReleaseDate: params.AppReleaseDate,
			},
		},
	}

	resp, err := r.client.UpdateAppPreOrder(context.Background(), params.PreOrderID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update pre-order: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Updated pre-order: %s", resp.Data.ID)), nil
}

func (r *Registry) handleDeletePreOrder(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PreOrderID string `json:"pre_order_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PreOrderID == "" {
		return nil, fmt.Errorf("pre_order_id is required")
	}

	err := r.client.DeleteAppPreOrder(context.Background(), params.PreOrderID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete pre-order: %v", err)), nil
	}

	return mcp.NewSuccessResult("Pre-order deleted successfully"), nil
}

func formatPreOrder(po api.AppPreOrder) string {
	result := fmt.Sprintf("Pre-Order ID: %s\n", po.ID)
	if po.Attributes.PreOrderAvailableDate != "" {
		result += fmt.Sprintf("Pre-Order Available: %s\n", po.Attributes.PreOrderAvailableDate)
	}
	if po.Attributes.AppReleaseDate != "" {
		result += fmt.Sprintf("Release Date: %s\n", po.Attributes.AppReleaseDate)
	}
	return result
}
