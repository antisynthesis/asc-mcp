package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerCommercePricingTools registers commerce pricing and
// availability tools.
//
// Pricing in App Store Connect is set by replacing a price schedule, not
// by editing individual prices: POST a new schedule naming a base
// territory plus the manual prices you want, and Apple derives the rest.
// Availability works the same way for in-app purchases; subscriptions use
// the per-plan subscriptionPlanAvailability resource (4.4), which
// replaced subscriptionAvailability because it can distinguish the
// monthly plan from the pre-paid (upfront) one.
func (r *Registry) registerCommercePricingTools() {
	// List in-app purchase price points
	r.register(mcp.Tool{
		Name:        "list_in_app_purchase_price_points",
		Description: "List the price points available to an in-app purchase. Filter by territory, e.g. {\"territory\": [\"USA\"]}.",
		InputSchema: commerceVersionListSchema("iap_id", "The in-app purchase ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List In App Purchase Price Points",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListInAppPurchasePricePoints)

	// Get in-app purchase price schedule
	r.register(mcp.Tool{
		Name:        "get_in_app_purchase_price_schedule",
		Description: "Get the price schedule of an in-app purchase",
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
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get In App Purchase Price Schedule",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetInAppPurchasePriceSchedule)

	// Set in-app purchase price schedule
	r.register(mcp.Tool{
		Name:        "set_in_app_purchase_price_schedule",
		Description: "Replace an in-app purchase's price schedule. Prices in territories you do not list are derived automatically from the base territory.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"iap_id": {
					Type:        "string",
					Description: "The in-app purchase ID",
				},
				"base_territory_id": {
					Type:        "string",
					Description: "The territory the automatic prices are derived from, e.g. USA",
				},
				"prices": {
					Type:        "array",
					Description: "Manual prices, each an object with price_point_id and optional start_date and end_date (YYYY-MM-DD), e.g. [{\"price_point_id\": \"eyJz...\"}].",
					Items:       &mcp.Property{Type: "object"},
				},
			},
			Required: []string{"iap_id", "base_territory_id", "prices"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Set In App Purchase Price Schedule",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleSetInAppPurchasePriceSchedule)

	// List in-app purchase price schedule prices
	r.register(mcp.Tool{
		Name:        "list_in_app_purchase_price_schedule_prices",
		Description: "List the prices of an in-app purchase price schedule. Manual prices are the ones set explicitly; automatic prices are the ones Apple derived for other territories.",
		InputSchema: priceScheduleListSchema("price_schedule_id", "The in-app purchase price schedule ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List In App Purchase Price Schedule Prices",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListInAppPurchasePriceSchedulePrices)

	// Get in-app purchase availability
	r.register(mcp.Tool{
		Name:        "get_in_app_purchase_availability",
		Description: "Get the territory availability of an in-app purchase",
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
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get In App Purchase Availability",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetInAppPurchaseAvailability)

	// Set in-app purchase availability
	r.register(mcp.Tool{
		Name:        "set_in_app_purchase_availability",
		Description: "Set the territories an in-app purchase is available in",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"iap_id": {
					Type:        "string",
					Description: "The in-app purchase ID",
				},
				"territory_ids": {
					Type:        "array",
					Description: "Territory IDs the in-app purchase is available in, e.g. [\"USA\", \"CAN\"]",
					Items:       &mcp.Property{Type: "string"},
				},
				"available_in_new_territories": {
					Type:        "boolean",
					Description: "Whether the in-app purchase becomes available automatically in territories Apple adds later",
				},
			},
			Required: []string{"iap_id", "territory_ids"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Set In App Purchase Availability",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleSetInAppPurchaseAvailability)

	// List in-app purchase available territories
	r.register(mcp.Tool{
		Name:        "list_in_app_purchase_available_territories",
		Description: "List the territories covered by an in-app purchase availability",
		InputSchema: commerceVersionListSchema("availability_id", "The in-app purchase availability ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List In App Purchase Available Territories",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListInAppPurchaseAvailableTerritories)

	// List subscription plan availabilities
	r.register(mcp.Tool{
		Name:        "list_subscription_plan_availabilities",
		Description: "List the plan availabilities of a subscription, one per billing plan. Replaces the deprecated subscription availability resource.",
		InputSchema: commerceVersionListSchema("subscription_id", "The subscription ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Subscription Plan Availabilities",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListSubscriptionPlanAvailabilities)

	// Get subscription plan availability
	r.register(mcp.Tool{
		Name:        "get_subscription_plan_availability",
		Description: "Get a subscription plan availability",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"availability_id": {
					Type:        "string",
					Description: "The subscription plan availability ID",
				},
			},
			Required: []string{"availability_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Subscription Plan Availability",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetSubscriptionPlanAvailability)

	// Create subscription plan availability
	r.register(mcp.Tool{
		Name:        "create_subscription_plan_availability",
		Description: "Configure the territories one billing plan of a subscription is available in",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"subscription_id": {
					Type:        "string",
					Description: "The subscription ID",
				},
				"plan_type": {
					Type:        "string",
					Description: "The billing plan: MONTHLY or UPFRONT",
				},
				"territory_ids": {
					Type:        "array",
					Description: "Territory IDs the plan is available in, e.g. [\"USA\", \"CAN\"]",
					Items:       &mcp.Property{Type: "string"},
				},
				"available_in_new_territories": {
					Type:        "boolean",
					Description: "Whether the plan becomes available automatically in territories Apple adds later",
				},
			},
			Required: []string{"subscription_id", "plan_type", "territory_ids"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Subscription Plan Availability",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateSubscriptionPlanAvailability)

	// Update subscription plan availability
	r.register(mcp.Tool{
		Name:        "update_subscription_plan_availability",
		Description: "Update a subscription plan's territory availability",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"availability_id": {
					Type:        "string",
					Description: "The subscription plan availability ID",
				},
				"territory_ids": {
					Type:        "array",
					Description: "Replacement territory IDs the plan is available in",
					Items:       &mcp.Property{Type: "string"},
				},
				"available_in_new_territories": {
					Type:        "boolean",
					Description: "Whether the plan becomes available automatically in territories Apple adds later",
				},
			},
			Required: []string{"availability_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update Subscription Plan Availability",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateSubscriptionPlanAvailability)

	// List subscription plan available territories
	r.register(mcp.Tool{
		Name:        "list_subscription_plan_available_territories",
		Description: "List the territories covered by a subscription plan availability",
		InputSchema: commerceVersionListSchema("availability_id", "The subscription plan availability ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Subscription Plan Available Territories",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListSubscriptionPlanAvailableTerritories)

	// Get subscription price point
	r.register(mcp.Tool{
		Name:        "get_subscription_price_point",
		Description: "Get a subscription price point",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"price_point_id": {
					Type:        "string",
					Description: "The subscription price point ID",
				},
			},
			Required: []string{"price_point_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Subscription Price Point",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetSubscriptionPricePoint)

	// List subscription price point equalizations
	r.register(mcp.Tool{
		Name:        "list_subscription_price_point_equalizations",
		Description: "List the price points in other territories equivalent to a subscription price point",
		InputSchema: commerceVersionListSchema("price_point_id", "The subscription price point ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Subscription Price Point Equalizations",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListSubscriptionPricePointEqualizations)

	// List subscription price point adjusted equalizations
	r.register(mcp.Tool{
		Name:        "list_subscription_price_point_adjusted_equalizations",
		Description: "List equalized subscription price points adjusted for a pre-paid plan, so a monthly price point can be matched against the upfront plan's price point",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"price_point_id": {
					Type:        "string",
					Description: "The subscription price point ID to equalize from",
				},
				"upfront_price_point_id": {
					Type:        "string",
					Description: "The upfront (pre-paid) price point ID the equalization is adjusted against",
				},
				"plan_type": {
					Type:        "string",
					Description: "The billing plan the adjusted prices apply to: MONTHLY or UPFRONT",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of results to return (default 100)",
				},
				"cursor": cursorProperty(),
				"filter": {
					Type:        "object",
					Description: "Additional JSON:API filters, e.g. {\"territory\": [\"USA\"]} becomes filter[territory]=USA.",
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
			Required: []string{"price_point_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Subscription Price Point Adjusted Equalizations",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListSubscriptionPricePointAdjustedEqualizations)

	// Set app price schedule
	r.register(mcp.Tool{
		Name:        "set_app_price_schedule",
		Description: "Replace an app's price schedule. Prices in territories you do not list are derived automatically from the base territory; pass an empty prices array to keep the app free.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
				"base_territory_id": {
					Type:        "string",
					Description: "The territory the automatic prices are derived from, e.g. USA",
				},
				"prices": {
					Type:        "array",
					Description: "Manual prices, each an object with price_point_id and optional start_date and end_date (YYYY-MM-DD), e.g. [{\"price_point_id\": \"eyJz...\", \"start_date\": \"2026-01-01\"}].",
					Items:       &mcp.Property{Type: "object"},
				},
			},
			Required: []string{"app_id", "base_territory_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Set App Price Schedule",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleSetAppPriceSchedule)

	// Get app price point
	r.register(mcp.Tool{
		Name:        "get_app_price_point",
		Description: "Get an app price point",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"price_point_id": {
					Type:        "string",
					Description: "The app price point ID",
				},
			},
			Required: []string{"price_point_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get App Price Point",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetAppPricePoint)

	// List app price point equalizations
	r.register(mcp.Tool{
		Name:        "list_app_price_point_equalizations",
		Description: "List the app price points in other territories equivalent to a given price point",
		InputSchema: commerceVersionListSchema("price_point_id", "The app price point ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List App Price Point Equalizations",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAppPricePointEqualizations)
}

// priceScheduleListSchema builds the input schema for the price schedule
// price listing, which needs a manual/automatic selector on top of the
// usual list knobs.
func priceScheduleListSchema(idKey, idDescription string) mcp.JSONSchema {
	schema := commerceVersionListSchema(idKey, idDescription)
	schema.Properties["price_type"] = mcp.Property{
		Type:        "string",
		Description: "Which prices to list: MANUAL (default) or AUTOMATIC",
	}
	return schema
}

// inlinePriceParams is one entry of the prices array accepted by the two
// price schedule write tools.
type inlinePriceParams struct {
	PricePointID string `json:"price_point_id"`
	StartDate    string `json:"start_date"`
	EndDate      string `json:"end_date"`
}

func (r *Registry) handleListInAppPurchasePricePoints(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.InAppPurchasePricePointsResponse, error) {
		return r.client.ListInAppPurchasePricePoints(ctx, params.IAPID, params.opts(100))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list in-app purchase price points: %v", err)), nil
	}

	return newListResult(formatInAppPurchasePricePoints(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetInAppPurchasePriceSchedule(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		IAPID string `json:"iap_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.IAPID == "" {
		return mcp.NewErrorResult("iap_id is required"), nil
	}

	resp, err := r.client.GetInAppPurchasePriceSchedule(ctx, params.IAPID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get in-app purchase price schedule: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("In-App Purchase Price Schedule ID: %s\n", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleSetInAppPurchasePriceSchedule(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		IAPID           string              `json:"iap_id"`
		BaseTerritoryID string              `json:"base_territory_id"`
		Prices          []inlinePriceParams `json:"prices"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.IAPID == "" {
		return mcp.NewErrorResult("iap_id is required"), nil
	}
	if params.BaseTerritoryID == "" {
		return mcp.NewErrorResult("base_territory_id is required"), nil
	}
	if len(params.Prices) == 0 {
		return mcp.NewErrorResult("prices is required"), nil
	}

	priceRefs := make([]api.ResourceIdentifier, 0, len(params.Prices))
	included := make([]api.InAppPurchasePriceInlineCreate, 0, len(params.Prices))
	for i, price := range params.Prices {
		if price.PricePointID == "" {
			return mcp.NewErrorResult("each price requires price_point_id"), nil
		}

		id := fmt.Sprintf("price-%d", i+1)
		priceRefs = append(priceRefs, api.ResourceIdentifier{Type: "inAppPurchasePrices", ID: id})
		included = append(included, api.InAppPurchasePriceInlineCreate{
			Type:       "inAppPurchasePrices",
			ID:         id,
			Attributes: inlinePriceAttributes(price),
			Relationships: &api.InAppPurchasePriceInlineCreateRelationships{
				InAppPurchasePricePoint: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "inAppPurchasePricePoints", ID: price.PricePointID},
				},
			},
		})
	}

	req := &api.InAppPurchasePriceScheduleCreateRequest{
		Data: api.InAppPurchasePriceScheduleCreateData{
			Type: "inAppPurchasePriceSchedules",
			Relationships: api.InAppPurchasePriceScheduleCreateRelationships{
				InAppPurchase: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "inAppPurchases", ID: params.IAPID},
				},
				BaseTerritory: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "territories", ID: params.BaseTerritoryID},
				},
				ManualPrices: api.RelationshipDataList{Data: priceRefs},
			},
		},
		Included: included,
	}

	resp, err := r.client.CreateInAppPurchasePriceSchedule(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to set in-app purchase price schedule: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("In-app purchase price schedule set: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleListInAppPurchasePriceSchedulePrices(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PriceScheduleID string `json:"price_schedule_id"`
		PriceType       string `json:"price_type"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PriceScheduleID == "" {
		return mcp.NewErrorResult("price_schedule_id is required"), nil
	}

	automatic := strings.EqualFold(params.PriceType, "AUTOMATIC")

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.InAppPurchasePricesResponse, error) {
		return r.client.ListInAppPurchasePriceSchedulePrices(ctx, params.PriceScheduleID, automatic, params.opts(100))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list in-app purchase price schedule prices: %v", err)), nil
	}

	return newListResult(formatInAppPurchasePrices(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetInAppPurchaseAvailability(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		IAPID string `json:"iap_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.IAPID == "" {
		return mcp.NewErrorResult("iap_id is required"), nil
	}

	resp, err := r.client.GetInAppPurchaseAvailability(ctx, params.IAPID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get in-app purchase availability: %v", err)), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", resp.Data.ID))
	sb.WriteString(fmt.Sprintf("Available In New Territories: %t\n", resp.Data.Attributes.AvailableInNewTerritories))

	// Best-effort: also show the territories the availability covers.
	if territories, err := r.client.ListInAppPurchaseAvailableTerritories(ctx, resp.Data.ID, nil); err == nil && len(territories.Data) > 0 {
		sb.WriteString(fmt.Sprintf("\n%s", formatTerritories(territories.Data)))
	}

	return newDataResult(sb.String(), resp.Data), nil
}

func (r *Registry) handleSetInAppPurchaseAvailability(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		IAPID                     string   `json:"iap_id"`
		TerritoryIDs              []string `json:"territory_ids"`
		AvailableInNewTerritories bool     `json:"available_in_new_territories"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.IAPID == "" {
		return mcp.NewErrorResult("iap_id is required"), nil
	}
	if len(params.TerritoryIDs) == 0 {
		return mcp.NewErrorResult("territory_ids is required"), nil
	}

	req := &api.InAppPurchaseAvailabilityCreateRequest{
		Data: api.InAppPurchaseAvailabilityCreateData{
			Type: "inAppPurchaseAvailabilities",
			Attributes: api.InAppPurchaseAvailabilityAttributes{
				AvailableInNewTerritories: params.AvailableInNewTerritories,
			},
			Relationships: api.InAppPurchaseAvailabilityCreateRelationships{
				InAppPurchase: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "inAppPurchases", ID: params.IAPID},
				},
				AvailableTerritories: api.RelationshipDataList{
					Data: territoryIdentifiers(params.TerritoryIDs),
				},
			},
		},
	}

	resp, err := r.client.CreateInAppPurchaseAvailability(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to set in-app purchase availability: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("In-app purchase availability set: %s (%d territories)", resp.Data.ID, len(params.TerritoryIDs)), resp.Data), nil
}

func (r *Registry) handleListInAppPurchaseAvailableTerritories(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AvailabilityID string `json:"availability_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AvailabilityID == "" {
		return mcp.NewErrorResult("availability_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.TerritoriesResponse, error) {
		return r.client.ListInAppPurchaseAvailableTerritories(ctx, params.AvailabilityID, params.opts(200))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list in-app purchase available territories: %v", err)), nil
	}

	return newListResult(formatTerritories(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleListSubscriptionPlanAvailabilities(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubscriptionID string `json:"subscription_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubscriptionID == "" {
		return mcp.NewErrorResult("subscription_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.SubscriptionPlanAvailabilitiesResponse, error) {
		return r.client.ListSubscriptionPlanAvailabilities(ctx, params.SubscriptionID, params.opts(50))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list subscription plan availabilities: %v", err)), nil
	}

	return newListResult(formatSubscriptionPlanAvailabilities(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetSubscriptionPlanAvailability(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AvailabilityID string `json:"availability_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AvailabilityID == "" {
		return mcp.NewErrorResult("availability_id is required"), nil
	}

	resp, err := r.client.GetSubscriptionPlanAvailability(ctx, params.AvailabilityID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get subscription plan availability: %v", err)), nil
	}

	var sb strings.Builder
	sb.WriteString(formatSubscriptionPlanAvailability(resp.Data))

	// Best-effort: also show the territories the plan is available in.
	if territories, err := r.client.ListSubscriptionPlanAvailableTerritories(ctx, resp.Data.ID, nil); err == nil && len(territories.Data) > 0 {
		sb.WriteString(fmt.Sprintf("\n%s", formatTerritories(territories.Data)))
	}

	return newDataResult(sb.String(), resp.Data), nil
}

func (r *Registry) handleCreateSubscriptionPlanAvailability(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubscriptionID            string   `json:"subscription_id"`
		PlanType                  string   `json:"plan_type"`
		TerritoryIDs              []string `json:"territory_ids"`
		AvailableInNewTerritories *bool    `json:"available_in_new_territories"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubscriptionID == "" {
		return mcp.NewErrorResult("subscription_id is required"), nil
	}
	if params.PlanType == "" {
		return mcp.NewErrorResult("plan_type is required"), nil
	}
	if len(params.TerritoryIDs) == 0 {
		return mcp.NewErrorResult("territory_ids is required"), nil
	}

	req := &api.SubscriptionPlanAvailabilityCreateRequest{
		Data: api.SubscriptionPlanAvailabilityCreateData{
			Type: "subscriptionPlanAvailabilities",
			Attributes: api.SubscriptionPlanAvailabilityAttributes{
				PlanType:                  params.PlanType,
				AvailableInNewTerritories: params.AvailableInNewTerritories,
			},
			Relationships: api.SubscriptionPlanAvailabilityCreateRelationships{
				Subscription: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "subscriptions", ID: params.SubscriptionID},
				},
				AvailableTerritories: api.RelationshipDataList{
					Data: territoryIdentifiers(params.TerritoryIDs),
				},
			},
		},
	}

	resp, err := r.client.CreateSubscriptionPlanAvailability(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create subscription plan availability: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Subscription plan availability created:\n%s", formatSubscriptionPlanAvailability(resp.Data)), resp.Data), nil
}

func (r *Registry) handleUpdateSubscriptionPlanAvailability(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AvailabilityID            string   `json:"availability_id"`
		TerritoryIDs              []string `json:"territory_ids"`
		AvailableInNewTerritories *bool    `json:"available_in_new_territories"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AvailabilityID == "" {
		return mcp.NewErrorResult("availability_id is required"), nil
	}
	if len(params.TerritoryIDs) == 0 && params.AvailableInNewTerritories == nil {
		return mcp.NewErrorResult("one of territory_ids or available_in_new_territories is required"), nil
	}

	data := api.SubscriptionPlanAvailabilityUpdateData{
		Type: "subscriptionPlanAvailabilities",
		ID:   params.AvailabilityID,
	}
	if params.AvailableInNewTerritories != nil {
		data.Attributes = &api.SubscriptionPlanAvailabilityUpdateAttributes{
			AvailableInNewTerritories: params.AvailableInNewTerritories,
		}
	}
	if len(params.TerritoryIDs) > 0 {
		data.Relationships = &api.SubscriptionPlanAvailabilityUpdateRelationships{
			AvailableTerritories: &api.RelationshipDataList{
				Data: territoryIdentifiers(params.TerritoryIDs),
			},
		}
	}

	req := &api.SubscriptionPlanAvailabilityUpdateRequest{Data: data}

	resp, err := r.client.UpdateSubscriptionPlanAvailability(ctx, params.AvailabilityID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update subscription plan availability: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Subscription plan availability updated:\n%s", formatSubscriptionPlanAvailability(resp.Data)), resp.Data), nil
}

func (r *Registry) handleListSubscriptionPlanAvailableTerritories(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AvailabilityID string `json:"availability_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AvailabilityID == "" {
		return mcp.NewErrorResult("availability_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.TerritoriesResponse, error) {
		return r.client.ListSubscriptionPlanAvailableTerritories(ctx, params.AvailabilityID, params.opts(200))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list subscription plan available territories: %v", err)), nil
	}

	return newListResult(formatTerritories(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetSubscriptionPricePoint(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PricePointID string `json:"price_point_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PricePointID == "" {
		return mcp.NewErrorResult("price_point_id is required"), nil
	}

	resp, err := r.client.GetSubscriptionPricePoint(ctx, params.PricePointID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get subscription price point: %v", err)), nil
	}

	return newDataResult(formatSubscriptionPricePoint(resp.Data), resp.Data), nil
}

func (r *Registry) handleListSubscriptionPricePointEqualizations(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PricePointID string `json:"price_point_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PricePointID == "" {
		return mcp.NewErrorResult("price_point_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.SubscriptionPricePointsResponse, error) {
		return r.client.ListSubscriptionPricePointEqualizations(ctx, params.PricePointID, params.opts(100))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list subscription price point equalizations: %v", err)), nil
	}

	return newListResult(formatSubscriptionPricePoints(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleListSubscriptionPricePointAdjustedEqualizations(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PricePointID        string `json:"price_point_id"`
		UpfrontPricePointID string `json:"upfront_price_point_id"`
		PlanType            string `json:"plan_type"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PricePointID == "" {
		return mcp.NewErrorResult("price_point_id is required"), nil
	}

	// upfrontPricePointId and planType are ordinary JSON:API filters;
	// fold the dedicated arguments into whatever filter map the caller
	// already supplied.
	filter := make(map[string][]string, len(params.Filter)+2)
	for k, v := range params.Filter {
		filter[k] = v
	}
	if params.UpfrontPricePointID != "" {
		filter["upfrontPricePointId"] = []string{params.UpfrontPricePointID}
	}
	if params.PlanType != "" {
		filter["planType"] = []string{params.PlanType}
	}
	params.Filter = filter

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.SubscriptionPricePointsResponse, error) {
		return r.client.ListSubscriptionPricePointAdjustedEqualizations(ctx, params.PricePointID, params.opts(100))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list subscription price point adjusted equalizations: %v", err)), nil
	}

	return newListResult(formatSubscriptionPricePoints(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleSetAppPriceSchedule(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID           string              `json:"app_id"`
		BaseTerritoryID string              `json:"base_territory_id"`
		Prices          []inlinePriceParams `json:"prices"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}
	if params.BaseTerritoryID == "" {
		return mcp.NewErrorResult("base_territory_id is required"), nil
	}

	priceRefs := make([]api.ResourceIdentifier, 0, len(params.Prices))
	included := make([]api.AppPriceInlineCreate, 0, len(params.Prices))
	for i, price := range params.Prices {
		if price.PricePointID == "" {
			return mcp.NewErrorResult("each price requires price_point_id"), nil
		}

		id := fmt.Sprintf("price-%d", i+1)
		priceRefs = append(priceRefs, api.ResourceIdentifier{Type: "appPrices", ID: id})
		included = append(included, api.AppPriceInlineCreate{
			Type:       "appPrices",
			ID:         id,
			Attributes: inlinePriceAttributes(price),
			Relationships: &api.AppPriceInlineCreateRelationships{
				AppPricePoint: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "appPricePoints", ID: price.PricePointID},
				},
			},
		})
	}

	req := &api.AppPriceScheduleCreateRequest{
		Data: api.AppPriceScheduleCreateData{
			Type: "appPriceSchedules",
			Relationships: api.AppPriceScheduleCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "apps", ID: params.AppID},
				},
				BaseTerritory: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "territories", ID: params.BaseTerritoryID},
				},
				ManualPrices: api.RelationshipDataList{Data: priceRefs},
			},
		},
		Included: included,
	}

	resp, err := r.client.CreateAppPriceSchedule(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to set app price schedule: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("App price schedule set: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleGetAppPricePoint(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PricePointID string `json:"price_point_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PricePointID == "" {
		return mcp.NewErrorResult("price_point_id is required"), nil
	}

	resp, err := r.client.GetAppPricePoint(ctx, params.PricePointID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app price point: %v", err)), nil
	}

	return newDataResult(formatAppPricePoint(resp.Data), resp.Data), nil
}

func (r *Registry) handleListAppPricePointEqualizations(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PricePointID string `json:"price_point_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PricePointID == "" {
		return mcp.NewErrorResult("price_point_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AppPricePointsResponse, error) {
		return r.client.ListAppPricePointEqualizations(ctx, params.PricePointID, params.opts(100))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list app price point equalizations: %v", err)), nil
	}

	return newListResult(formatAppPricePoints(resp.Data), resp.Data, resp.Links), nil
}

// inlinePriceAttributes renders the optional scheduling window of an
// inline price, returning nil when the caller gave neither date so the
// price takes effect immediately.
func inlinePriceAttributes(price inlinePriceParams) *api.CommercePriceInlineAttributes {
	if price.StartDate == "" && price.EndDate == "" {
		return nil
	}
	return &api.CommercePriceInlineAttributes{
		StartDate: price.StartDate,
		EndDate:   price.EndDate,
	}
}

// territoryIdentifiers turns territory IDs into JSON:API linkages.
func territoryIdentifiers(ids []string) []api.ResourceIdentifier {
	identifiers := make([]api.ResourceIdentifier, 0, len(ids))
	for _, id := range ids {
		identifiers = append(identifiers, api.ResourceIdentifier{Type: "territories", ID: id})
	}
	return identifiers
}

func formatInAppPurchasePricePoints(pricePoints []api.InAppPurchasePricePoint) string {
	if len(pricePoints) == 0 {
		return "No in-app purchase price points found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d in-app purchase price points:\n\n", len(pricePoints)))

	for _, pp := range pricePoints {
		sb.WriteString(fmt.Sprintf("ID: %s\n", pp.ID))
		sb.WriteString(fmt.Sprintf("Customer Price: %s\n", pp.Attributes.CustomerPrice))
		sb.WriteString(fmt.Sprintf("Proceeds: %s\n", pp.Attributes.Proceeds))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatInAppPurchasePrices(prices []api.InAppPurchasePrice) string {
	if len(prices) == 0 {
		return "No prices found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d prices:\n\n", len(prices)))

	for _, p := range prices {
		sb.WriteString(fmt.Sprintf("ID: %s\n", p.ID))
		if p.Attributes.StartDate != "" {
			sb.WriteString(fmt.Sprintf("Start Date: %s\n", p.Attributes.StartDate))
		}
		if p.Attributes.EndDate != "" {
			sb.WriteString(fmt.Sprintf("End Date: %s\n", p.Attributes.EndDate))
		}
		sb.WriteString(fmt.Sprintf("Manual: %t\n", p.Attributes.Manual))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatSubscriptionPlanAvailabilities(availabilities []api.SubscriptionPlanAvailability) string {
	if len(availabilities) == 0 {
		return "No subscription plan availabilities found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d subscription plan availabilities:\n\n", len(availabilities)))

	for _, a := range availabilities {
		sb.WriteString(formatSubscriptionPlanAvailability(a))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatSubscriptionPlanAvailability(a api.SubscriptionPlanAvailability) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", a.ID))
	if a.Attributes.PlanType != "" {
		sb.WriteString(fmt.Sprintf("Plan Type: %s\n", a.Attributes.PlanType))
	}
	if a.Attributes.AvailableInNewTerritories != nil {
		sb.WriteString(fmt.Sprintf("Available In New Territories: %t\n", *a.Attributes.AvailableInNewTerritories))
	}
	return sb.String()
}
