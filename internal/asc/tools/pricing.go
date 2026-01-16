package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerPricingTools registers app pricing tools.
func (r *Registry) registerPricingTools() {
	// Get app price schedule
	r.register(mcp.Tool{
		Name:        "get_app_price_schedule",
		Description: "Get the price schedule for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleGetAppPriceSchedule)

	// List app price points
	r.register(mcp.Tool{
		Name:        "list_app_price_points",
		Description: "List available price points for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of price points to return (default 100)",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleListAppPricePoints)

	// List territories
	r.register(mcp.Tool{
		Name:        "list_territories",
		Description: "List all available App Store territories",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"limit": {
					Type:        "integer",
					Description: "Maximum number of territories to return (default 200)",
				},
			},
		},
	}, r.handleListTerritories)

	// List subscription price points
	r.register(mcp.Tool{
		Name:        "list_subscription_price_points",
		Description: "List price points for a subscription",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"subscription_id": {
					Type:        "string",
					Description: "The subscription ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of price points to return (default 100)",
				},
			},
			Required: []string{"subscription_id"},
		},
	}, r.handleListSubscriptionPricePoints)
}

func (r *Registry) handleGetAppPriceSchedule(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}

	resp, err := r.client.GetAppPriceSchedule(context.Background(), params.AppID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app price schedule: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppPriceSchedule(resp.Data)), nil
}

func (r *Registry) handleListAppPricePoints(args json.RawMessage) (*mcp.ToolsCallResult, error) {
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
		limit = 100
	}

	resp, err := r.client.ListAppPricePoints(context.Background(), params.AppID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list app price points: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppPricePoints(resp.Data)), nil
}

func (r *Registry) handleListTerritories(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Limit int `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 200
	}

	resp, err := r.client.ListTerritories(context.Background(), limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list territories: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatTerritories(resp.Data)), nil
}

func (r *Registry) handleListSubscriptionPricePoints(args json.RawMessage) (*mcp.ToolsCallResult, error) {
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
		limit = 100
	}

	resp, err := r.client.ListSubscriptionPricePoints(context.Background(), params.SubscriptionID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list subscription price points: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatSubscriptionPricePoints(resp.Data)), nil
}

func formatAppPriceSchedule(schedule api.AppPriceSchedule) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("App Price Schedule ID: %s\n", schedule.ID))
	return sb.String()
}

func formatAppPricePoints(pricePoints []api.AppPricePoint) string {
	if len(pricePoints) == 0 {
		return "No price points found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d price points:\n\n", len(pricePoints)))

	for _, pp := range pricePoints {
		sb.WriteString(formatAppPricePoint(pp))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAppPricePoint(pp api.AppPricePoint) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", pp.ID))
	sb.WriteString(fmt.Sprintf("Customer Price: %s\n", pp.Attributes.CustomerPrice))
	sb.WriteString(fmt.Sprintf("Proceeds: %s\n", pp.Attributes.Proceeds))
	return sb.String()
}

func formatTerritories(territories []api.Territory) string {
	if len(territories) == 0 {
		return "No territories found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d territories:\n\n", len(territories)))

	for _, t := range territories {
		sb.WriteString(fmt.Sprintf("ID: %s - Currency: %s\n", t.ID, t.Attributes.Currency))
	}

	return sb.String()
}

func formatSubscriptionPricePoints(pricePoints []api.SubscriptionPricePoint) string {
	if len(pricePoints) == 0 {
		return "No subscription price points found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d subscription price points:\n\n", len(pricePoints)))

	for _, pp := range pricePoints {
		sb.WriteString(formatSubscriptionPricePoint(pp))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatSubscriptionPricePoint(pp api.SubscriptionPricePoint) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", pp.ID))
	sb.WriteString(fmt.Sprintf("Customer Price: %s\n", pp.Attributes.CustomerPrice))
	sb.WriteString(fmt.Sprintf("Proceeds: %s\n", pp.Attributes.Proceeds))
	sb.WriteString(fmt.Sprintf("Proceeds Year 2: %s\n", pp.Attributes.ProceedsYear2))
	return sb.String()
}
