package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerSubscriptionTools registers subscription tools.
func (r *Registry) registerSubscriptionTools() {
	// List subscription groups
	r.register(mcp.Tool{
		Name:        "list_subscription_groups",
		Description: "List subscription groups for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to list subscription groups for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of groups to return (default 50)",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleListSubscriptionGroups)

	// Get subscription group
	r.register(mcp.Tool{
		Name:        "get_subscription_group",
		Description: "Get details of a specific subscription group",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"group_id": {
					Type:        "string",
					Description: "The subscription group ID",
				},
			},
			Required: []string{"group_id"},
		},
	}, r.handleGetSubscriptionGroup)

	// List subscriptions
	r.register(mcp.Tool{
		Name:        "list_subscriptions",
		Description: "List subscriptions for a subscription group",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"group_id": {
					Type:        "string",
					Description: "The subscription group ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of subscriptions to return (default 50)",
				},
			},
			Required: []string{"group_id"},
		},
	}, r.handleListSubscriptions)

	// Get subscription
	r.register(mcp.Tool{
		Name:        "get_subscription",
		Description: "Get details of a specific subscription",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"subscription_id": {
					Type:        "string",
					Description: "The subscription ID",
				},
			},
			Required: []string{"subscription_id"},
		},
	}, r.handleGetSubscription)
}

func (r *Registry) handleListSubscriptionGroups(args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := r.client.ListSubscriptionGroups(context.Background(), params.AppID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list subscription groups: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatSubscriptionGroups(resp.Data)), nil
}

func (r *Registry) handleGetSubscriptionGroup(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		GroupID string `json:"group_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.GroupID == "" {
		return nil, fmt.Errorf("group_id is required")
	}

	resp, err := r.client.GetSubscriptionGroup(context.Background(), params.GroupID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get subscription group: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatSubscriptionGroup(resp.Data)), nil
}

func (r *Registry) handleListSubscriptions(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		GroupID string `json:"group_id"`
		Limit   int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.GroupID == "" {
		return nil, fmt.Errorf("group_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListSubscriptions(context.Background(), params.GroupID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list subscriptions: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatSubscriptions(resp.Data)), nil
}

func (r *Registry) handleGetSubscription(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubscriptionID string `json:"subscription_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubscriptionID == "" {
		return nil, fmt.Errorf("subscription_id is required")
	}

	resp, err := r.client.GetSubscription(context.Background(), params.SubscriptionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get subscription: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatSubscription(resp.Data)), nil
}

func formatSubscriptionGroups(groups []api.SubscriptionGroup) string {
	if len(groups) == 0 {
		return "No subscription groups found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d subscription groups:\n\n", len(groups)))

	for _, group := range groups {
		sb.WriteString(formatSubscriptionGroup(group))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatSubscriptionGroup(group api.SubscriptionGroup) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Group ID: %s\n", group.ID))
	sb.WriteString(fmt.Sprintf("Reference Name: %s\n", group.Attributes.ReferenceName))
	return sb.String()
}

func formatSubscriptions(subs []api.Subscription) string {
	if len(subs) == 0 {
		return "No subscriptions found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d subscriptions:\n\n", len(subs)))

	for _, sub := range subs {
		sb.WriteString(formatSubscription(sub))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatSubscription(sub api.Subscription) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", sub.ID))
	sb.WriteString(fmt.Sprintf("Name: %s\n", sub.Attributes.Name))
	sb.WriteString(fmt.Sprintf("Product ID: %s\n", sub.Attributes.ProductID))
	sb.WriteString(fmt.Sprintf("State: %s\n", sub.Attributes.State))
	sb.WriteString(fmt.Sprintf("Period: %s\n", sub.Attributes.SubscriptionPeriod))
	sb.WriteString(fmt.Sprintf("Family Sharable: %t\n", sub.Attributes.FamilySharable))
	sb.WriteString(fmt.Sprintf("Group Level: %d\n", sub.Attributes.GroupLevel))
	if sub.Attributes.ReviewNote != "" {
		sb.WriteString(fmt.Sprintf("Review Note: %s\n", sub.Attributes.ReviewNote))
	}
	return sb.String()
}
