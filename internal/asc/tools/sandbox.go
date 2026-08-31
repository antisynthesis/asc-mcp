package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerSandboxTools registers sandbox tester tools.
func (r *Registry) registerSandboxTools() {
	// List sandbox testers
	r.register(mcp.Tool{
		Name:        "list_sandbox_testers",
		Description: "List sandbox testers for the account",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"limit": {
					Type:        "integer",
					Description: "Maximum number of testers to return (default 50)",
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
			Title:         "List Sandbox Testers",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListSandboxTesters)

	// Update sandbox tester
	r.register(mcp.Tool{
		Name:        "update_sandbox_tester",
		Description: "Update a sandbox tester's territory or interruptable setting",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"tester_id": {
					Type:        "string",
					Description: "The sandbox tester ID",
				},
				"territory": {
					Type:        "string",
					Description: "New App Store territory code",
				},
				"interruptable": {
					Type:        "boolean",
					Description: "Whether purchases can be interrupted for testing",
				},
				"subscription_renewal_rate": {
					Type:        "string",
					Description: "Subscription renewal rate: MONTHLY_RENEWAL_EVERY_ONE_HOUR, etc.",
				},
			},
			Required: []string{"tester_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update Sandbox Tester",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateSandboxTester)

	// Clear sandbox tester purchase history
	r.register(mcp.Tool{
		Name:        "clear_sandbox_tester_purchase_history",
		Description: "Clear the purchase history for one or more sandbox testers",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"tester_ids": {
					Type:        "array",
					Description: "The sandbox tester IDs whose purchase history should be cleared",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"tester_ids"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Clear Sandbox Tester Purchase History",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleClearSandboxTesterPurchaseHistory)
}

func (r *Registry) handleListSandboxTesters(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
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

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.SandboxTestersResponse, error) {
		return r.client.ListSandboxTesters(ctx, listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list sandbox testers: %v", err)), nil
	}

	return newListResult(formatSandboxTesters(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleUpdateSandboxTester(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		TesterID                string `json:"tester_id"`
		Territory               string `json:"territory"`
		Interruptable           *bool  `json:"interruptable"`
		SubscriptionRenewalRate string `json:"subscription_renewal_rate"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.TesterID == "" {
		return mcp.NewErrorResult("tester_id is required"), nil
	}

	req := &api.SandboxTesterUpdateRequest{
		Data: api.SandboxTesterUpdateData{
			Type: "sandboxTesters",
			ID:   params.TesterID,
			Attributes: api.SandboxTesterUpdateAttributes{
				Territory:               params.Territory,
				Interruptable:           params.Interruptable,
				SubscriptionRenewalRate: params.SubscriptionRenewalRate,
			},
		},
	}

	resp, err := r.client.UpdateSandboxTester(ctx, params.TesterID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update sandbox tester: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Sandbox tester updated:\n%s", formatSandboxTester(resp.Data)), resp.Data), nil
}

func (r *Registry) handleClearSandboxTesterPurchaseHistory(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		TesterIDs []string `json:"tester_ids"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if len(params.TesterIDs) == 0 {
		return mcp.NewErrorResult("tester_ids is required"), nil
	}

	var testers []api.ResourceIdentifier
	for _, id := range params.TesterIDs {
		testers = append(testers, api.ResourceIdentifier{Type: "sandboxTesters", ID: id})
	}

	req := &api.SandboxTesterClearPurchaseHistoryRequest{
		Data: api.SandboxTesterClearPurchaseHistoryData{
			Type: "sandboxTestersClearPurchaseHistoryRequest",
			Relationships: api.SandboxTesterClearPurchaseHistoryRelationships{
				SandboxTesters: api.RelationshipDataList{
					Data: testers,
				},
			},
		},
	}

	resp, err := r.client.ClearSandboxTesterPurchaseHistory(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to clear sandbox tester purchase history: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Purchase history cleared for %d sandbox testers (request ID: %s)", len(params.TesterIDs), resp.Data.ID), resp.Data), nil
}

func formatSandboxTesters(testers []api.SandboxTester) string {
	if len(testers) == 0 {
		return "No sandbox testers found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d sandbox testers:\n\n", len(testers)))

	for _, tester := range testers {
		sb.WriteString(formatSandboxTester(tester))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatSandboxTester(tester api.SandboxTester) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", tester.ID))
	sb.WriteString(fmt.Sprintf("Email: %s\n", tester.Attributes.Email))
	sb.WriteString(fmt.Sprintf("Name: %s %s\n", tester.Attributes.FirstName, tester.Attributes.LastName))
	if tester.Attributes.AppStoreTerritory != "" {
		sb.WriteString(fmt.Sprintf("Territory: %s\n", tester.Attributes.AppStoreTerritory))
	}
	sb.WriteString(fmt.Sprintf("Interruptable: %t\n", tester.Attributes.Interruptable))
	return sb.String()
}
