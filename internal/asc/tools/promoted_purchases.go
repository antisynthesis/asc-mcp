package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerPromotedPurchasesTools registers promoted purchases and offer code tools.
func (r *Registry) registerPromotedPurchasesTools() {
	// List promoted purchases
	r.register(mcp.Tool{
		Name:        "list_promoted_purchases",
		Description: "List promoted in-app purchases for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of promoted purchases to return (default 50)",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleListPromotedPurchases)

	// Get promoted purchase
	r.register(mcp.Tool{
		Name:        "get_promoted_purchase",
		Description: "Get details of a specific promoted purchase",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"promoted_purchase_id": {
					Type:        "string",
					Description: "The promoted purchase ID",
				},
			},
			Required: []string{"promoted_purchase_id"},
		},
	}, r.handleGetPromotedPurchase)

	// Create promoted purchase
	r.register(mcp.Tool{
		Name:        "create_promoted_purchase",
		Description: "Create a promoted in-app purchase",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
				"in_app_purchase_id": {
					Type:        "string",
					Description: "The in-app purchase ID to promote",
				},
				"enabled": {
					Type:        "boolean",
					Description: "Whether promotion is enabled (default true)",
				},
				"visibility_type": {
					Type:        "string",
					Description: "Visibility type: SHOW_FOR_ALL_USERS, APP_STORE_CONNECT_ONLY",
				},
			},
			Required: []string{"app_id", "in_app_purchase_id"},
		},
	}, r.handleCreatePromotedPurchase)

	// Update promoted purchase
	r.register(mcp.Tool{
		Name:        "update_promoted_purchase",
		Description: "Update a promoted purchase's settings",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"promoted_purchase_id": {
					Type:        "string",
					Description: "The promoted purchase ID",
				},
				"enabled": {
					Type:        "boolean",
					Description: "Whether promotion is enabled",
				},
				"visibility_type": {
					Type:        "string",
					Description: "Visibility type: SHOW_FOR_ALL_USERS, APP_STORE_CONNECT_ONLY",
				},
			},
			Required: []string{"promoted_purchase_id"},
		},
	}, r.handleUpdatePromotedPurchase)

	// Delete promoted purchase
	r.register(mcp.Tool{
		Name:        "delete_promoted_purchase",
		Description: "Delete a promoted purchase",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"promoted_purchase_id": {
					Type:        "string",
					Description: "The promoted purchase ID to delete",
				},
			},
			Required: []string{"promoted_purchase_id"},
		},
	}, r.handleDeletePromotedPurchase)

	// List subscription offer codes
	r.register(mcp.Tool{
		Name:        "list_subscription_offer_codes",
		Description: "List offer codes for a subscription",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"subscription_id": {
					Type:        "string",
					Description: "The subscription ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of offer codes to return (default 50)",
				},
			},
			Required: []string{"subscription_id"},
		},
	}, r.handleListSubscriptionOfferCodes)

	// Get subscription offer code
	r.register(mcp.Tool{
		Name:        "get_subscription_offer_code",
		Description: "Get details of a specific subscription offer code",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"offer_code_id": {
					Type:        "string",
					Description: "The offer code ID",
				},
			},
			Required: []string{"offer_code_id"},
		},
	}, r.handleGetSubscriptionOfferCode)

	// Create subscription offer code
	r.register(mcp.Tool{
		Name:        "create_subscription_offer_code",
		Description: "Create a new subscription offer code",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"subscription_id": {
					Type:        "string",
					Description: "The subscription ID",
				},
				"name": {
					Type:        "string",
					Description: "Name of the offer code",
				},
				"customer_eligibility": {
					Type:        "string",
					Description: "Customer eligibility: NEW, EXISTING, EXPIRED",
				},
				"offer_eligibility_type": {
					Type:        "string",
					Description: "Eligibility type: STACK_WITH_INTRO_OFFERS, REPLACE_INTRO_OFFERS",
				},
				"duration_type": {
					Type:        "string",
					Description: "Duration type: PAY_AS_YOU_GO, PAY_UP_FRONT, FREE",
				},
				"offer_mode": {
					Type:        "string",
					Description: "Offer mode: PAY_AS_YOU_GO, PAY_UP_FRONT, FREE",
				},
				"number_of_periods": {
					Type:        "integer",
					Description: "Number of periods for the offer",
				},
				"total_number_of_codes": {
					Type:        "integer",
					Description: "Total number of codes to generate",
				},
				"is_active": {
					Type:        "boolean",
					Description: "Whether the offer code is active",
				},
			},
			Required: []string{"subscription_id", "name", "customer_eligibility"},
		},
	}, r.handleCreateSubscriptionOfferCode)

	// Update subscription offer code
	r.register(mcp.Tool{
		Name:        "update_subscription_offer_code",
		Description: "Update a subscription offer code",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"offer_code_id": {
					Type:        "string",
					Description: "The offer code ID",
				},
				"is_active": {
					Type:        "boolean",
					Description: "Whether the offer code is active",
				},
			},
			Required: []string{"offer_code_id"},
		},
	}, r.handleUpdateSubscriptionOfferCode)

	// List win-back offers
	r.register(mcp.Tool{
		Name:        "list_win_back_offers",
		Description: "List win-back offers for a subscription",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"subscription_id": {
					Type:        "string",
					Description: "The subscription ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of offers to return (default 50)",
				},
			},
			Required: []string{"subscription_id"},
		},
	}, r.handleListWinBackOffers)

	// Get win-back offer
	r.register(mcp.Tool{
		Name:        "get_win_back_offer",
		Description: "Get details of a specific win-back offer",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"offer_id": {
					Type:        "string",
					Description: "The win-back offer ID",
				},
			},
			Required: []string{"offer_id"},
		},
	}, r.handleGetWinBackOffer)

	// Create win-back offer
	r.register(mcp.Tool{
		Name:        "create_win_back_offer",
		Description: "Create a new win-back offer for a subscription",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"subscription_id": {
					Type:        "string",
					Description: "The subscription ID",
				},
				"reference_name": {
					Type:        "string",
					Description: "Reference name for the offer",
				},
				"offer_id": {
					Type:        "string",
					Description: "Unique identifier for the offer",
				},
				"duration": {
					Type:        "string",
					Description: "Duration of the offer",
				},
				"offer_mode": {
					Type:        "string",
					Description: "Offer mode: PAY_AS_YOU_GO, PAY_UP_FRONT, FREE",
				},
				"period_count": {
					Type:        "integer",
					Description: "Number of periods",
				},
				"priority": {
					Type:        "string",
					Description: "Priority: HIGH, NORMAL",
				},
				"promotion_intent": {
					Type:        "string",
					Description: "Promotion intent: NOT_PROMOTED, USE_CUSTOM_PRODUCT_PAGE",
				},
			},
			Required: []string{"subscription_id", "reference_name", "offer_id"},
		},
	}, r.handleCreateWinBackOffer)

	// Update win-back offer
	r.register(mcp.Tool{
		Name:        "update_win_back_offer",
		Description: "Update a win-back offer",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"offer_id": {
					Type:        "string",
					Description: "The win-back offer ID",
				},
				"priority": {
					Type:        "string",
					Description: "Priority: HIGH, NORMAL",
				},
				"promotion_intent": {
					Type:        "string",
					Description: "Promotion intent: NOT_PROMOTED, USE_CUSTOM_PRODUCT_PAGE",
				},
			},
			Required: []string{"offer_id"},
		},
	}, r.handleUpdateWinBackOffer)

	// Delete win-back offer
	r.register(mcp.Tool{
		Name:        "delete_win_back_offer",
		Description: "Delete a win-back offer",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"offer_id": {
					Type:        "string",
					Description: "The win-back offer ID to delete",
				},
			},
			Required: []string{"offer_id"},
		},
	}, r.handleDeleteWinBackOffer)
}

func (r *Registry) handleListPromotedPurchases(args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := r.client.ListPromotedPurchases(context.Background(), params.AppID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list promoted purchases: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatPromotedPurchases(resp.Data)), nil
}

func (r *Registry) handleGetPromotedPurchase(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PromotedPurchaseID string `json:"promoted_purchase_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PromotedPurchaseID == "" {
		return nil, fmt.Errorf("promoted_purchase_id is required")
	}

	resp, err := r.client.GetPromotedPurchase(context.Background(), params.PromotedPurchaseID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get promoted purchase: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatPromotedPurchase(resp.Data)), nil
}

func (r *Registry) handleCreatePromotedPurchase(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID              string `json:"app_id"`
		InAppPurchaseID    string `json:"in_app_purchase_id"`
		Enabled            *bool  `json:"enabled"`
		VisibleForAllUsers *bool  `json:"visible_for_all_users"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" || params.InAppPurchaseID == "" {
		return nil, fmt.Errorf("app_id and in_app_purchase_id are required")
	}

	enabled := true
	if params.Enabled != nil {
		enabled = *params.Enabled
	}

	visibleForAll := true
	if params.VisibleForAllUsers != nil {
		visibleForAll = *params.VisibleForAllUsers
	}

	req := &api.PromotedPurchaseCreateRequest{
		Data: api.PromotedPurchaseCreateData{
			Type: "promotedPurchases",
			Attributes: api.PromotedPurchaseCreateAttributes{
				Enabled:            enabled,
				VisibleForAllUsers: visibleForAll,
			},
			Relationships: api.PromotedPurchaseCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "apps", ID: params.AppID},
				},
				InAppPurchase: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "inAppPurchases", ID: params.InAppPurchaseID},
				},
			},
		},
	}

	resp, err := r.client.CreatePromotedPurchase(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create promoted purchase: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Promoted purchase created:\n%s", formatPromotedPurchase(resp.Data))), nil
}

func (r *Registry) handleUpdatePromotedPurchase(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PromotedPurchaseID string `json:"promoted_purchase_id"`
		Enabled            *bool  `json:"enabled"`
		VisibleForAllUsers *bool  `json:"visible_for_all_users"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PromotedPurchaseID == "" {
		return nil, fmt.Errorf("promoted_purchase_id is required")
	}

	req := &api.PromotedPurchaseUpdateRequest{
		Data: api.PromotedPurchaseUpdateData{
			Type: "promotedPurchases",
			ID:   params.PromotedPurchaseID,
			Attributes: api.PromotedPurchaseUpdateAttributes{
				Enabled:            params.Enabled,
				VisibleForAllUsers: params.VisibleForAllUsers,
			},
		},
	}

	resp, err := r.client.UpdatePromotedPurchase(context.Background(), params.PromotedPurchaseID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update promoted purchase: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Promoted purchase updated:\n%s", formatPromotedPurchase(resp.Data))), nil
}

func (r *Registry) handleDeletePromotedPurchase(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PromotedPurchaseID string `json:"promoted_purchase_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PromotedPurchaseID == "" {
		return nil, fmt.Errorf("promoted_purchase_id is required")
	}

	err := r.client.DeletePromotedPurchase(context.Background(), params.PromotedPurchaseID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete promoted purchase: %v", err)), nil
	}

	return mcp.NewSuccessResult("Promoted purchase deleted"), nil
}

func (r *Registry) handleListSubscriptionOfferCodes(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubscriptionID string `json:"subscription_id"`
		Limit          int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubscriptionID == "" {
		return nil, fmt.Errorf("subscription_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListSubscriptionOfferCodes(context.Background(), params.SubscriptionID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list subscription offer codes: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatSubscriptionOfferCodes(resp.Data)), nil
}

func (r *Registry) handleGetSubscriptionOfferCode(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		OfferCodeID string `json:"offer_code_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.OfferCodeID == "" {
		return nil, fmt.Errorf("offer_code_id is required")
	}

	resp, err := r.client.GetSubscriptionOfferCode(context.Background(), params.OfferCodeID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get subscription offer code: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatSubscriptionOfferCode(resp.Data)), nil
}

func (r *Registry) handleCreateSubscriptionOfferCode(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubscriptionID          string   `json:"subscription_id"`
		Name                    string   `json:"name"`
		CustomerEligibilities   []string `json:"customer_eligibilities"`
		OfferEligibility        string   `json:"offer_eligibility"`
		Duration                string   `json:"duration"`
		OfferMode               string   `json:"offer_mode"`
		NumberOfPeriods         int      `json:"number_of_periods"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubscriptionID == "" || params.Name == "" {
		return nil, fmt.Errorf("subscription_id and name are required")
	}

	req := &api.SubscriptionOfferCodeCreateRequest{
		Data: api.SubscriptionOfferCodeCreateData{
			Type: "subscriptionOfferCodes",
			Attributes: api.SubscriptionOfferCodeCreateAttributes{
				Name:                  params.Name,
				CustomerEligibilities: params.CustomerEligibilities,
				OfferEligibility:      params.OfferEligibility,
				Duration:              params.Duration,
				OfferMode:             params.OfferMode,
				NumberOfPeriods:       params.NumberOfPeriods,
			},
			Relationships: api.SubscriptionOfferCodeCreateRelationships{
				Subscription: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "subscriptions", ID: params.SubscriptionID},
				},
			},
		},
	}

	resp, err := r.client.CreateSubscriptionOfferCode(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create subscription offer code: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Subscription offer code created:\n%s", formatSubscriptionOfferCode(resp.Data))), nil
}

func (r *Registry) handleUpdateSubscriptionOfferCode(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		OfferCodeID string `json:"offer_code_id"`
		Active      *bool  `json:"active"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.OfferCodeID == "" {
		return nil, fmt.Errorf("offer_code_id is required")
	}

	req := &api.SubscriptionOfferCodeUpdateRequest{
		Data: api.SubscriptionOfferCodeUpdateData{
			Type: "subscriptionOfferCodes",
			ID:   params.OfferCodeID,
			Attributes: api.SubscriptionOfferCodeUpdateAttributes{
				Active: params.Active,
			},
		},
	}

	resp, err := r.client.UpdateSubscriptionOfferCode(context.Background(), params.OfferCodeID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update subscription offer code: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Subscription offer code updated:\n%s", formatSubscriptionOfferCode(resp.Data))), nil
}

func (r *Registry) handleListWinBackOffers(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubscriptionID string `json:"subscription_id"`
		Limit          int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubscriptionID == "" {
		return nil, fmt.Errorf("subscription_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListWinBackOffers(context.Background(), params.SubscriptionID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list win-back offers: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatWinBackOffers(resp.Data)), nil
}

func (r *Registry) handleGetWinBackOffer(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		OfferID string `json:"offer_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.OfferID == "" {
		return nil, fmt.Errorf("offer_id is required")
	}

	resp, err := r.client.GetWinBackOffer(context.Background(), params.OfferID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get win-back offer: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatWinBackOffer(resp.Data)), nil
}

func (r *Registry) handleCreateWinBackOffer(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubscriptionID  string   `json:"subscription_id"`
		ReferenceName   string   `json:"reference_name"`
		OfferID         string   `json:"offer_id"`
		Duration        string   `json:"duration"`
		OfferMode       string   `json:"offer_mode"`
		PeriodCount     int      `json:"period_count"`
		Priority        string   `json:"priority"`
		PromotionIntent string   `json:"promotion_intent"`
		PriceIDs        []string `json:"price_ids"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubscriptionID == "" || params.ReferenceName == "" || params.OfferID == "" {
		return nil, fmt.Errorf("subscription_id, reference_name, and offer_id are required")
	}

	var prices []api.ResourceIdentifier
	for _, pid := range params.PriceIDs {
		prices = append(prices, api.ResourceIdentifier{Type: "winBackOfferPrices", ID: pid})
	}

	req := &api.WinBackOfferCreateRequest{
		Data: api.WinBackOfferCreateData{
			Type: "winBackOffers",
			Attributes: api.WinBackOfferCreateAttributes{
				ReferenceName:   params.ReferenceName,
				OfferID:         params.OfferID,
				Duration:        params.Duration,
				OfferMode:       params.OfferMode,
				PeriodCount:     params.PeriodCount,
				Priority:        params.Priority,
				PromotionIntent: params.PromotionIntent,
			},
			Relationships: api.WinBackOfferCreateRelationships{
				Subscription: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "subscriptions", ID: params.SubscriptionID},
				},
				Prices: api.RelationshipDataList{
					Data: prices,
				},
			},
		},
	}

	resp, err := r.client.CreateWinBackOffer(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create win-back offer: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Win-back offer created:\n%s", formatWinBackOffer(resp.Data))), nil
}

func (r *Registry) handleUpdateWinBackOffer(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		OfferID         string `json:"offer_id"`
		Priority        string `json:"priority"`
		PromotionIntent string `json:"promotion_intent"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.OfferID == "" {
		return nil, fmt.Errorf("offer_id is required")
	}

	req := &api.WinBackOfferUpdateRequest{
		Data: api.WinBackOfferUpdateData{
			Type: "winBackOffers",
			ID:   params.OfferID,
			Attributes: api.WinBackOfferUpdateAttributes{
				Priority:        params.Priority,
				PromotionIntent: params.PromotionIntent,
			},
		},
	}

	resp, err := r.client.UpdateWinBackOffer(context.Background(), params.OfferID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update win-back offer: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Win-back offer updated:\n%s", formatWinBackOffer(resp.Data))), nil
}

func (r *Registry) handleDeleteWinBackOffer(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		OfferID string `json:"offer_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.OfferID == "" {
		return nil, fmt.Errorf("offer_id is required")
	}

	err := r.client.DeleteWinBackOffer(context.Background(), params.OfferID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete win-back offer: %v", err)), nil
	}

	return mcp.NewSuccessResult("Win-back offer deleted"), nil
}

func formatPromotedPurchases(purchases []api.PromotedPurchase) string {
	if len(purchases) == 0 {
		return "No promoted purchases found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d promoted purchases:\n\n", len(purchases)))

	for _, p := range purchases {
		sb.WriteString(formatPromotedPurchase(p))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatPromotedPurchase(p api.PromotedPurchase) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", p.ID))
	sb.WriteString(fmt.Sprintf("Enabled: %t\n", p.Attributes.Enabled))
	sb.WriteString(fmt.Sprintf("Visible For All Users: %t\n", p.Attributes.VisibleForAllUsers))
	if p.Attributes.State != "" {
		sb.WriteString(fmt.Sprintf("State: %s\n", p.Attributes.State))
	}
	return sb.String()
}

func formatSubscriptionOfferCodes(codes []api.SubscriptionOfferCode) string {
	if len(codes) == 0 {
		return "No subscription offer codes found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d subscription offer codes:\n\n", len(codes)))

	for _, c := range codes {
		sb.WriteString(formatSubscriptionOfferCode(c))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatSubscriptionOfferCode(c api.SubscriptionOfferCode) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", c.ID))
	sb.WriteString(fmt.Sprintf("Name: %s\n", c.Attributes.Name))
	sb.WriteString(fmt.Sprintf("Active: %t\n", c.Attributes.Active))
	if len(c.Attributes.CustomerEligibilities) > 0 {
		sb.WriteString(fmt.Sprintf("Customer Eligibilities: %s\n", strings.Join(c.Attributes.CustomerEligibilities, ", ")))
	}
	if c.Attributes.OfferMode != "" {
		sb.WriteString(fmt.Sprintf("Offer Mode: %s\n", c.Attributes.OfferMode))
	}
	if c.Attributes.NumberOfPeriods > 0 {
		sb.WriteString(fmt.Sprintf("Periods: %d\n", c.Attributes.NumberOfPeriods))
	}
	if c.Attributes.TotalNumberOfCodes > 0 {
		sb.WriteString(fmt.Sprintf("Total Codes: %d\n", c.Attributes.TotalNumberOfCodes))
	}
	return sb.String()
}

func formatWinBackOffers(offers []api.WinBackOffer) string {
	if len(offers) == 0 {
		return "No win-back offers found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d win-back offers:\n\n", len(offers)))

	for _, o := range offers {
		sb.WriteString(formatWinBackOffer(o))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatWinBackOffer(o api.WinBackOffer) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", o.ID))
	sb.WriteString(fmt.Sprintf("Reference Name: %s\n", o.Attributes.ReferenceName))
	sb.WriteString(fmt.Sprintf("Offer ID: %s\n", o.Attributes.OfferID))
	if o.Attributes.OfferMode != "" {
		sb.WriteString(fmt.Sprintf("Offer Mode: %s\n", o.Attributes.OfferMode))
	}
	if o.Attributes.Duration != "" {
		sb.WriteString(fmt.Sprintf("Duration: %s\n", o.Attributes.Duration))
	}
	if o.Attributes.PeriodCount > 0 {
		sb.WriteString(fmt.Sprintf("Period Count: %d\n", o.Attributes.PeriodCount))
	}
	if o.Attributes.Priority != "" {
		sb.WriteString(fmt.Sprintf("Priority: %s\n", o.Attributes.Priority))
	}
	if o.Attributes.PromotionIntent != "" {
		sb.WriteString(fmt.Sprintf("Promotion Intent: %s\n", o.Attributes.PromotionIntent))
	}
	return sb.String()
}
