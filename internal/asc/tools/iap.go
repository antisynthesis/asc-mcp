package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerInAppPurchaseTools registers in-app purchase tools.
func (r *Registry) registerInAppPurchaseTools() {
	// List in-app purchases
	r.register(mcp.Tool{
		Name:        "list_in_app_purchases",
		Description: "List in-app purchases for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to list in-app purchases for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of in-app purchases to return (default 50)",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleListInAppPurchases)

	// Get in-app purchase
	r.register(mcp.Tool{
		Name:        "get_in_app_purchase",
		Description: "Get details of a specific in-app purchase",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"iap_id": {
					Type:        "string",
					Description: "The in-app purchase ID",
				},
			},
			Required: []string{"iap_id"},
		},
	}, r.handleGetInAppPurchase)

	// Create in-app purchase
	r.register(mcp.Tool{
		Name:        "create_in_app_purchase",
		Description: "Create a new in-app purchase",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to create the in-app purchase for",
				},
				"name": {
					Type:        "string",
					Description: "The name of the in-app purchase",
				},
				"product_id": {
					Type:        "string",
					Description: "The product identifier",
				},
				"iap_type": {
					Type:        "string",
					Description: "The type (CONSUMABLE, NON_CONSUMABLE, NON_RENEWING_SUBSCRIPTION)",
				},
				"review_note": {
					Type:        "string",
					Description: "Notes for App Review",
				},
				"family_sharable": {
					Type:        "boolean",
					Description: "Whether the IAP is sharable with family",
				},
			},
			Required: []string{"app_id", "name", "product_id", "iap_type"},
		},
	}, r.handleCreateInAppPurchase)

	// Update in-app purchase
	r.register(mcp.Tool{
		Name:        "update_in_app_purchase",
		Description: "Update an existing in-app purchase",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"iap_id": {
					Type:        "string",
					Description: "The in-app purchase ID",
				},
				"name": {
					Type:        "string",
					Description: "The updated name",
				},
				"review_note": {
					Type:        "string",
					Description: "Updated notes for App Review",
				},
				"family_sharable": {
					Type:        "boolean",
					Description: "Whether the IAP is sharable with family",
				},
			},
			Required: []string{"iap_id"},
		},
	}, r.handleUpdateInAppPurchase)

	// Delete in-app purchase
	r.register(mcp.Tool{
		Name:        "delete_in_app_purchase",
		Description: "Delete an in-app purchase",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"iap_id": {
					Type:        "string",
					Description: "The in-app purchase ID",
				},
			},
			Required: []string{"iap_id"},
		},
	}, r.handleDeleteInAppPurchase)
}

func (r *Registry) handleListInAppPurchases(args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := r.client.ListInAppPurchases(context.Background(), params.AppID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list in-app purchases: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatInAppPurchases(resp.Data)), nil
}

func (r *Registry) handleGetInAppPurchase(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		IAPID string `json:"iap_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.IAPID == "" {
		return nil, fmt.Errorf("iap_id is required")
	}

	resp, err := r.client.GetInAppPurchase(context.Background(), params.IAPID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get in-app purchase: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatInAppPurchase(resp.Data)), nil
}

func (r *Registry) handleCreateInAppPurchase(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID          string `json:"app_id"`
		Name           string `json:"name"`
		ProductID      string `json:"product_id"`
		IAPType        string `json:"iap_type"`
		ReviewNote     string `json:"review_note"`
		FamilySharable bool   `json:"family_sharable"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}
	if params.Name == "" {
		return nil, fmt.Errorf("name is required")
	}
	if params.ProductID == "" {
		return nil, fmt.Errorf("product_id is required")
	}
	if params.IAPType == "" {
		return nil, fmt.Errorf("iap_type is required")
	}

	req := &api.InAppPurchaseCreateRequest{
		Data: api.InAppPurchaseCreateData{
			Type: "inAppPurchases",
			Attributes: api.InAppPurchaseCreateAttributes{
				Name:              params.Name,
				ProductID:         params.ProductID,
				InAppPurchaseType: params.IAPType,
				ReviewNote:        params.ReviewNote,
				FamilySharable:    params.FamilySharable,
			},
			Relationships: api.InAppPurchaseCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "apps",
						ID:   params.AppID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateInAppPurchase(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create in-app purchase: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Created in-app purchase: %s (ID: %s)", resp.Data.Attributes.Name, resp.Data.ID)), nil
}

func (r *Registry) handleUpdateInAppPurchase(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		IAPID          string `json:"iap_id"`
		Name           string `json:"name"`
		ReviewNote     string `json:"review_note"`
		FamilySharable *bool  `json:"family_sharable"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.IAPID == "" {
		return nil, fmt.Errorf("iap_id is required")
	}

	req := &api.InAppPurchaseUpdateRequest{
		Data: api.InAppPurchaseUpdateData{
			Type: "inAppPurchases",
			ID:   params.IAPID,
			Attributes: api.InAppPurchaseUpdateAttributes{
				Name:           params.Name,
				ReviewNote:     params.ReviewNote,
				FamilySharable: params.FamilySharable,
			},
		},
	}

	resp, err := r.client.UpdateInAppPurchase(context.Background(), params.IAPID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update in-app purchase: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Updated in-app purchase: %s", resp.Data.ID)), nil
}

func (r *Registry) handleDeleteInAppPurchase(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		IAPID string `json:"iap_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.IAPID == "" {
		return nil, fmt.Errorf("iap_id is required")
	}

	err := r.client.DeleteInAppPurchase(context.Background(), params.IAPID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete in-app purchase: %v", err)), nil
	}

	return mcp.NewSuccessResult("In-app purchase deleted successfully"), nil
}

func formatInAppPurchases(iaps []api.InAppPurchase) string {
	if len(iaps) == 0 {
		return "No in-app purchases found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d in-app purchases:\n\n", len(iaps)))

	for _, iap := range iaps {
		sb.WriteString(formatInAppPurchase(iap))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatInAppPurchase(iap api.InAppPurchase) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", iap.ID))
	sb.WriteString(fmt.Sprintf("Name: %s\n", iap.Attributes.Name))
	sb.WriteString(fmt.Sprintf("Product ID: %s\n", iap.Attributes.ProductID))
	sb.WriteString(fmt.Sprintf("Type: %s\n", iap.Attributes.InAppPurchaseType))
	sb.WriteString(fmt.Sprintf("State: %s\n", iap.Attributes.State))
	sb.WriteString(fmt.Sprintf("Family Sharable: %t\n", iap.Attributes.FamilySharable))
	if iap.Attributes.ReviewNote != "" {
		sb.WriteString(fmt.Sprintf("Review Note: %s\n", iap.Attributes.ReviewNote))
	}
	return sb.String()
}
