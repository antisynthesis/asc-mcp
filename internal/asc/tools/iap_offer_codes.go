package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerInAppPurchaseOfferCodeTools registers in-app purchase offer
// code tools (App Store Connect API 4.2).
//
// Offer codes came to non-subscription in-app purchases in 4.2, mirroring
// the subscription offer codes that already existed. An offer code owns
// per-territory prices plus two kinds of redeemable batch: custom codes
// (one memorable string redeemable many times) and one-time-use codes
// (a generated batch downloaded as CSV). Issued batches cannot be edited,
// only deactivated.
func (r *Registry) registerInAppPurchaseOfferCodeTools() {
	// List in-app purchase offer codes
	r.register(mcp.Tool{
		Name:        "list_in_app_purchase_offer_codes",
		Description: "List the offer codes of an in-app purchase",
		InputSchema: commerceVersionListSchema("iap_id", "The in-app purchase ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List In App Purchase Offer Codes",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListInAppPurchaseOfferCodes)

	// Get in-app purchase offer code
	r.register(mcp.Tool{
		Name:        "get_in_app_purchase_offer_code",
		Description: "Get an in-app purchase offer code and its redemption counts",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"offer_code_id": {
					Type:        "string",
					Description: "The in-app purchase offer code ID",
				},
			},
			Required: []string{"offer_code_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get In App Purchase Offer Code",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetInAppPurchaseOfferCode)

	// Create in-app purchase offer code
	r.register(mcp.Tool{
		Name:        "create_in_app_purchase_offer_code",
		Description: "Create an offer code for an in-app purchase. Supply one price per territory you want the offer available in; use list_in_app_purchase_price_points to find price point IDs.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"iap_id": {
					Type:        "string",
					Description: "The in-app purchase ID",
				},
				"name": {
					Type:        "string",
					Description: "The offer code reference name",
				},
				"customer_eligibilities": {
					Type:        "array",
					Description: "Customer eligibilities: NEW, EXISTING, EXPIRED",
					Items:       &mcp.Property{Type: "string"},
				},
				"prices": {
					Type:        "array",
					Description: "Offer prices, each an object with territory_id and price_point_id, e.g. [{\"territory_id\": \"USA\", \"price_point_id\": \"eyJz...\"}].",
					Items:       &mcp.Property{Type: "object"},
				},
			},
			Required: []string{"iap_id", "name", "customer_eligibilities", "prices"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create In App Purchase Offer Code",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateInAppPurchaseOfferCode)

	// Update in-app purchase offer code
	r.register(mcp.Tool{
		Name:        "update_in_app_purchase_offer_code",
		Description: "Activate or deactivate an in-app purchase offer code",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"offer_code_id": {
					Type:        "string",
					Description: "The in-app purchase offer code ID",
				},
				"active": {
					Type:        "boolean",
					Description: "Whether the offer code is active",
				},
			},
			Required: []string{"offer_code_id", "active"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update In App Purchase Offer Code",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateInAppPurchaseOfferCode)

	// List in-app purchase offer code prices
	r.register(mcp.Tool{
		Name:        "list_in_app_purchase_offer_code_prices",
		Description: "List the per-territory prices of an in-app purchase offer code",
		InputSchema: commerceVersionListSchema("offer_code_id", "The in-app purchase offer code ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List In App Purchase Offer Code Prices",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListInAppPurchaseOfferCodePrices)

	// List in-app purchase offer code custom codes
	r.register(mcp.Tool{
		Name:        "list_in_app_purchase_offer_code_custom_codes",
		Description: "List the custom code batches of an in-app purchase offer code",
		InputSchema: commerceVersionListSchema("offer_code_id", "The in-app purchase offer code ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List In App Purchase Offer Code Custom Codes",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListInAppPurchaseOfferCodeCustomCodes)

	// Create in-app purchase offer code custom code
	r.register(mcp.Tool{
		Name:        "create_in_app_purchase_offer_code_custom_code",
		Description: "Issue a custom (memorable) code batch for an in-app purchase offer code",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"offer_code_id": {
					Type:        "string",
					Description: "The in-app purchase offer code ID",
				},
				"custom_code": {
					Type:        "string",
					Description: "The code customers redeem, e.g. LAUNCH2026",
				},
				"number_of_codes": {
					Type:        "integer",
					Description: "How many times the code can be redeemed",
				},
				"expiration_date": {
					Type:        "string",
					Description: "Expiration date in YYYY-MM-DD format",
				},
			},
			Required: []string{"offer_code_id", "custom_code", "number_of_codes"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create In App Purchase Offer Code Custom Code",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateInAppPurchaseOfferCodeCustomCode)

	// Update in-app purchase offer code custom code
	r.register(mcp.Tool{
		Name:        "update_in_app_purchase_offer_code_custom_code",
		Description: "Activate or deactivate a custom code batch",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"custom_code_id": {
					Type:        "string",
					Description: "The custom code batch ID",
				},
				"active": {
					Type:        "boolean",
					Description: "Whether the custom code is active",
				},
			},
			Required: []string{"custom_code_id", "active"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update In App Purchase Offer Code Custom Code",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateInAppPurchaseOfferCodeCustomCode)

	// List in-app purchase offer code one-time-use codes
	r.register(mcp.Tool{
		Name:        "list_in_app_purchase_offer_code_one_time_use_codes",
		Description: "List the one-time-use code batches of an in-app purchase offer code",
		InputSchema: commerceVersionListSchema("offer_code_id", "The in-app purchase offer code ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List In App Purchase Offer Code One Time Use Codes",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListInAppPurchaseOfferCodeOneTimeUseCodes)

	// Create in-app purchase offer code one-time-use code
	r.register(mcp.Tool{
		Name:        "create_in_app_purchase_offer_code_one_time_use_code",
		Description: "Generate a batch of one-time-use codes for an in-app purchase offer code. Download the generated codes with get_in_app_purchase_offer_code_one_time_use_code_values.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"offer_code_id": {
					Type:        "string",
					Description: "The in-app purchase offer code ID",
				},
				"number_of_codes": {
					Type:        "integer",
					Description: "How many unique codes to generate",
				},
				"expiration_date": {
					Type:        "string",
					Description: "Expiration date in YYYY-MM-DD format",
				},
				"environment": {
					Type:        "string",
					Description: "Environment: PRODUCTION or SANDBOX",
				},
			},
			Required: []string{"offer_code_id", "number_of_codes", "expiration_date"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create In App Purchase Offer Code One Time Use Code",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateInAppPurchaseOfferCodeOneTimeUseCode)

	// Update in-app purchase offer code one-time-use code
	r.register(mcp.Tool{
		Name:        "update_in_app_purchase_offer_code_one_time_use_code",
		Description: "Activate or deactivate a one-time-use code batch",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"one_time_use_code_id": {
					Type:        "string",
					Description: "The one-time-use code batch ID",
				},
				"active": {
					Type:        "boolean",
					Description: "Whether the code batch is active",
				},
			},
			Required: []string{"one_time_use_code_id", "active"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update In App Purchase Offer Code One Time Use Code",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateInAppPurchaseOfferCodeOneTimeUseCode)

	// Get in-app purchase offer code one-time-use code values
	r.register(mcp.Tool{
		Name:        "get_in_app_purchase_offer_code_one_time_use_code_values",
		Description: "Download the generated codes of a one-time-use batch. Apple returns them as CSV.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"one_time_use_code_id": {
					Type:        "string",
					Description: "The one-time-use code batch ID",
				},
			},
			Required: []string{"one_time_use_code_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get In App Purchase Offer Code One Time Use Code Values",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetInAppPurchaseOfferCodeOneTimeUseCodeValues)
}

func (r *Registry) handleListInAppPurchaseOfferCodes(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		IAPID string `json:"iap_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.IAPID == "" {
		return mcp.NewErrorResult("iap_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.InAppPurchaseOfferCodesResponse, error) {
		return r.client.ListInAppPurchaseOfferCodes(ctx, params.IAPID, params.opts(50))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list in-app purchase offer codes: %v", err)), nil
	}

	return newListResult(formatInAppPurchaseOfferCodes(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetInAppPurchaseOfferCode(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		OfferCodeID string `json:"offer_code_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.OfferCodeID == "" {
		return mcp.NewErrorResult("offer_code_id is required"), nil
	}

	resp, err := r.client.GetInAppPurchaseOfferCode(ctx, params.OfferCodeID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get in-app purchase offer code: %v", err)), nil
	}

	return newDataResult(formatInAppPurchaseOfferCode(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateInAppPurchaseOfferCode(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		IAPID                 string   `json:"iap_id"`
		Name                  string   `json:"name"`
		CustomerEligibilities []string `json:"customer_eligibilities"`
		Prices                []struct {
			TerritoryID  string `json:"territory_id"`
			PricePointID string `json:"price_point_id"`
		} `json:"prices"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.IAPID == "" {
		return mcp.NewErrorResult("iap_id is required"), nil
	}
	if params.Name == "" {
		return mcp.NewErrorResult("name is required"), nil
	}
	if len(params.CustomerEligibilities) == 0 {
		return mcp.NewErrorResult("customer_eligibilities is required"), nil
	}
	if len(params.Prices) == 0 {
		return mcp.NewErrorResult("prices is required"), nil
	}

	// Each inline price gets a synthetic client ID that the prices
	// relationship references, the same pattern Apple uses for inline
	// resource creation.
	priceRefs := make([]api.ResourceIdentifier, 0, len(params.Prices))
	included := make([]api.InAppPurchaseOfferPriceInlineCreate, 0, len(params.Prices))
	for i, price := range params.Prices {
		if price.TerritoryID == "" {
			return mcp.NewErrorResult("each price requires territory_id"), nil
		}
		if price.PricePointID == "" {
			return mcp.NewErrorResult("each price requires price_point_id"), nil
		}

		id := fmt.Sprintf("price-%d", i+1)
		priceRefs = append(priceRefs, api.ResourceIdentifier{Type: "inAppPurchaseOfferPrices", ID: id})
		included = append(included, api.InAppPurchaseOfferPriceInlineCreate{
			Type: "inAppPurchaseOfferPrices",
			ID:   id,
			Relationships: &api.InAppPurchaseOfferPriceInlineCreateRelationships{
				Territory: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "territories", ID: price.TerritoryID},
				},
				PricePoint: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "inAppPurchasePricePoints", ID: price.PricePointID},
				},
			},
		})
	}

	req := &api.InAppPurchaseOfferCodeCreateRequest{
		Data: api.InAppPurchaseOfferCodeCreateData{
			Type: "inAppPurchaseOfferCodes",
			Attributes: api.InAppPurchaseOfferCodeCreateAttributes{
				Name:                  params.Name,
				CustomerEligibilities: params.CustomerEligibilities,
			},
			Relationships: api.InAppPurchaseOfferCodeCreateRelationships{
				InAppPurchase: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "inAppPurchases", ID: params.IAPID},
				},
				Prices: api.RelationshipDataList{Data: priceRefs},
			},
		},
		Included: included,
	}

	resp, err := r.client.CreateInAppPurchaseOfferCode(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create in-app purchase offer code: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("In-app purchase offer code created:\n%s", formatInAppPurchaseOfferCode(resp.Data)), resp.Data), nil
}

func (r *Registry) handleUpdateInAppPurchaseOfferCode(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		OfferCodeID string `json:"offer_code_id"`
		Active      *bool  `json:"active"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.OfferCodeID == "" {
		return mcp.NewErrorResult("offer_code_id is required"), nil
	}
	if params.Active == nil {
		return mcp.NewErrorResult("active is required"), nil
	}

	req := &api.InAppPurchaseOfferCodeUpdateRequest{
		Data: api.InAppPurchaseOfferCodeUpdateData{
			Type:       "inAppPurchaseOfferCodes",
			ID:         params.OfferCodeID,
			Attributes: api.OfferCodeActiveAttributes{Active: params.Active},
		},
	}

	resp, err := r.client.UpdateInAppPurchaseOfferCode(ctx, params.OfferCodeID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update in-app purchase offer code: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("In-app purchase offer code updated:\n%s", formatInAppPurchaseOfferCode(resp.Data)), resp.Data), nil
}

func (r *Registry) handleListInAppPurchaseOfferCodePrices(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		OfferCodeID string `json:"offer_code_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.OfferCodeID == "" {
		return mcp.NewErrorResult("offer_code_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.InAppPurchaseOfferPricesResponse, error) {
		return r.client.ListInAppPurchaseOfferCodePrices(ctx, params.OfferCodeID, params.opts(50))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list in-app purchase offer code prices: %v", err)), nil
	}

	return newListResult(formatInAppPurchaseOfferPrices(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleListInAppPurchaseOfferCodeCustomCodes(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		OfferCodeID string `json:"offer_code_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.OfferCodeID == "" {
		return mcp.NewErrorResult("offer_code_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.InAppPurchaseOfferCodeCustomCodesResponse, error) {
		return r.client.ListInAppPurchaseOfferCodeCustomCodes(ctx, params.OfferCodeID, params.opts(50))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list in-app purchase offer code custom codes: %v", err)), nil
	}

	return newListResult(formatInAppPurchaseOfferCodeCustomCodes(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleCreateInAppPurchaseOfferCodeCustomCode(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		OfferCodeID    string `json:"offer_code_id"`
		CustomCode     string `json:"custom_code"`
		NumberOfCodes  int    `json:"number_of_codes"`
		ExpirationDate string `json:"expiration_date"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.OfferCodeID == "" {
		return mcp.NewErrorResult("offer_code_id is required"), nil
	}
	if params.CustomCode == "" {
		return mcp.NewErrorResult("custom_code is required"), nil
	}
	if params.NumberOfCodes <= 0 {
		return mcp.NewErrorResult("number_of_codes is required"), nil
	}

	req := &api.InAppPurchaseOfferCodeCustomCodeCreateRequest{
		Data: api.InAppPurchaseOfferCodeCustomCodeCreateData{
			Type: "inAppPurchaseOfferCodeCustomCodes",
			Attributes: api.InAppPurchaseOfferCodeCustomCodeCreateAttributes{
				CustomCode:     params.CustomCode,
				NumberOfCodes:  params.NumberOfCodes,
				ExpirationDate: params.ExpirationDate,
			},
			Relationships: api.OfferCodeRelationship{
				OfferCode: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "inAppPurchaseOfferCodes", ID: params.OfferCodeID},
				},
			},
		},
	}

	resp, err := r.client.CreateInAppPurchaseOfferCodeCustomCode(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create in-app purchase offer code custom code: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Custom code created:\n%s", formatInAppPurchaseOfferCodeCustomCode(resp.Data)), resp.Data), nil
}

func (r *Registry) handleUpdateInAppPurchaseOfferCodeCustomCode(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		CustomCodeID string `json:"custom_code_id"`
		Active       *bool  `json:"active"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.CustomCodeID == "" {
		return mcp.NewErrorResult("custom_code_id is required"), nil
	}
	if params.Active == nil {
		return mcp.NewErrorResult("active is required"), nil
	}

	req := &api.InAppPurchaseOfferCodeCustomCodeUpdateRequest{
		Data: api.InAppPurchaseOfferCodeCustomCodeUpdateData{
			Type:       "inAppPurchaseOfferCodeCustomCodes",
			ID:         params.CustomCodeID,
			Attributes: api.OfferCodeActiveAttributes{Active: params.Active},
		},
	}

	resp, err := r.client.UpdateInAppPurchaseOfferCodeCustomCode(ctx, params.CustomCodeID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update in-app purchase offer code custom code: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Custom code updated:\n%s", formatInAppPurchaseOfferCodeCustomCode(resp.Data)), resp.Data), nil
}

func (r *Registry) handleListInAppPurchaseOfferCodeOneTimeUseCodes(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		OfferCodeID string `json:"offer_code_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.OfferCodeID == "" {
		return mcp.NewErrorResult("offer_code_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.InAppPurchaseOfferCodeOneTimeUseCodesResponse, error) {
		return r.client.ListInAppPurchaseOfferCodeOneTimeUseCodes(ctx, params.OfferCodeID, params.opts(50))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list in-app purchase offer code one-time-use codes: %v", err)), nil
	}

	return newListResult(formatInAppPurchaseOfferCodeOneTimeUseCodes(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleCreateInAppPurchaseOfferCodeOneTimeUseCode(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		OfferCodeID    string `json:"offer_code_id"`
		NumberOfCodes  int    `json:"number_of_codes"`
		ExpirationDate string `json:"expiration_date"`
		Environment    string `json:"environment"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.OfferCodeID == "" {
		return mcp.NewErrorResult("offer_code_id is required"), nil
	}
	if params.NumberOfCodes <= 0 {
		return mcp.NewErrorResult("number_of_codes is required"), nil
	}
	if params.ExpirationDate == "" {
		return mcp.NewErrorResult("expiration_date is required"), nil
	}

	req := &api.InAppPurchaseOfferCodeOneTimeUseCodeCreateRequest{
		Data: api.InAppPurchaseOfferCodeOneTimeUseCodeCreateData{
			Type: "inAppPurchaseOfferCodeOneTimeUseCodes",
			Attributes: api.InAppPurchaseOfferCodeOneTimeUseCodeCreateAttributes{
				NumberOfCodes:  params.NumberOfCodes,
				ExpirationDate: params.ExpirationDate,
				Environment:    params.Environment,
			},
			Relationships: api.OfferCodeRelationship{
				OfferCode: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "inAppPurchaseOfferCodes", ID: params.OfferCodeID},
				},
			},
		},
	}

	resp, err := r.client.CreateInAppPurchaseOfferCodeOneTimeUseCode(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create in-app purchase offer code one-time-use code: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("One-time-use code batch created:\n%s", formatInAppPurchaseOfferCodeOneTimeUseCode(resp.Data)), resp.Data), nil
}

func (r *Registry) handleUpdateInAppPurchaseOfferCodeOneTimeUseCode(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		OneTimeUseCodeID string `json:"one_time_use_code_id"`
		Active           *bool  `json:"active"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.OneTimeUseCodeID == "" {
		return mcp.NewErrorResult("one_time_use_code_id is required"), nil
	}
	if params.Active == nil {
		return mcp.NewErrorResult("active is required"), nil
	}

	req := &api.InAppPurchaseOfferCodeOneTimeUseCodeUpdateRequest{
		Data: api.InAppPurchaseOfferCodeOneTimeUseCodeUpdateData{
			Type:       "inAppPurchaseOfferCodeOneTimeUseCodes",
			ID:         params.OneTimeUseCodeID,
			Attributes: api.OfferCodeActiveAttributes{Active: params.Active},
		},
	}

	resp, err := r.client.UpdateInAppPurchaseOfferCodeOneTimeUseCode(ctx, params.OneTimeUseCodeID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update in-app purchase offer code one-time-use code: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("One-time-use code batch updated:\n%s", formatInAppPurchaseOfferCodeOneTimeUseCode(resp.Data)), resp.Data), nil
}

func (r *Registry) handleGetInAppPurchaseOfferCodeOneTimeUseCodeValues(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		OneTimeUseCodeID string `json:"one_time_use_code_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.OneTimeUseCodeID == "" {
		return mcp.NewErrorResult("one_time_use_code_id is required"), nil
	}

	csv, err := r.client.GetInAppPurchaseOfferCodeOneTimeUseCodeValues(ctx, params.OneTimeUseCodeID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get one-time-use code values: %v", err)), nil
	}

	if strings.TrimSpace(csv) == "" {
		return mcp.NewSuccessResult("No codes available yet. Apple generates the batch asynchronously; retry shortly."), nil
	}

	return newDataResult(csv, map[string]string{"csv": csv}), nil
}

func formatInAppPurchaseOfferCodes(codes []api.InAppPurchaseOfferCode) string {
	if len(codes) == 0 {
		return "No in-app purchase offer codes found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d in-app purchase offer codes:\n\n", len(codes)))

	for _, c := range codes {
		sb.WriteString(formatInAppPurchaseOfferCode(c))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatInAppPurchaseOfferCode(c api.InAppPurchaseOfferCode) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", c.ID))
	sb.WriteString(fmt.Sprintf("Name: %s\n", c.Attributes.Name))
	if len(c.Attributes.CustomerEligibilities) > 0 {
		sb.WriteString(fmt.Sprintf("Customer Eligibilities: %s\n", strings.Join(c.Attributes.CustomerEligibilities, ", ")))
	}
	sb.WriteString(fmt.Sprintf("Active: %t\n", c.Attributes.Active))
	sb.WriteString(fmt.Sprintf("Production Codes: %d\n", c.Attributes.ProductionCodeCount))
	sb.WriteString(fmt.Sprintf("Sandbox Codes: %d\n", c.Attributes.SandboxCodeCount))
	return sb.String()
}

func formatInAppPurchaseOfferPrices(prices []api.InAppPurchaseOfferPrice) string {
	if len(prices) == 0 {
		return "No offer prices found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d offer prices:\n\n", len(prices)))

	for _, p := range prices {
		sb.WriteString(fmt.Sprintf("ID: %s\n", p.ID))
		if p.Relationships.Territory != nil {
			sb.WriteString(fmt.Sprintf("Territory: %s\n", p.Relationships.Territory.Data.ID))
		}
		if p.Relationships.PricePoint != nil {
			sb.WriteString(fmt.Sprintf("Price Point: %s\n", p.Relationships.PricePoint.Data.ID))
		}
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatInAppPurchaseOfferCodeCustomCodes(codes []api.InAppPurchaseOfferCodeCustomCode) string {
	if len(codes) == 0 {
		return "No custom codes found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d custom codes:\n\n", len(codes)))

	for _, c := range codes {
		sb.WriteString(formatInAppPurchaseOfferCodeCustomCode(c))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatInAppPurchaseOfferCodeCustomCode(c api.InAppPurchaseOfferCodeCustomCode) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", c.ID))
	sb.WriteString(fmt.Sprintf("Code: %s\n", c.Attributes.CustomCode))
	sb.WriteString(fmt.Sprintf("Number of Codes: %d\n", c.Attributes.NumberOfCodes))
	if c.Attributes.ExpirationDate != "" {
		sb.WriteString(fmt.Sprintf("Expires: %s\n", c.Attributes.ExpirationDate))
	}
	sb.WriteString(fmt.Sprintf("Active: %t\n", c.Attributes.Active))
	return sb.String()
}

func formatInAppPurchaseOfferCodeOneTimeUseCodes(codes []api.InAppPurchaseOfferCodeOneTimeUseCode) string {
	if len(codes) == 0 {
		return "No one-time-use code batches found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d one-time-use code batches:\n\n", len(codes)))

	for _, c := range codes {
		sb.WriteString(formatInAppPurchaseOfferCodeOneTimeUseCode(c))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatInAppPurchaseOfferCodeOneTimeUseCode(c api.InAppPurchaseOfferCodeOneTimeUseCode) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", c.ID))
	sb.WriteString(fmt.Sprintf("Number of Codes: %d\n", c.Attributes.NumberOfCodes))
	if c.Attributes.ExpirationDate != "" {
		sb.WriteString(fmt.Sprintf("Expires: %s\n", c.Attributes.ExpirationDate))
	}
	if c.Attributes.Environment != "" {
		sb.WriteString(fmt.Sprintf("Environment: %s\n", c.Attributes.Environment))
	}
	sb.WriteString(fmt.Sprintf("Active: %t\n", c.Attributes.Active))
	return sb.String()
}
