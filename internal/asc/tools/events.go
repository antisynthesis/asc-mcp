package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerAppEventTools registers app event tools.
func (r *Registry) registerAppEventTools() {
	// List app events
	r.register(mcp.Tool{
		Name:        "list_app_events",
		Description: "List promotional app events for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of events to return (default 50)",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleListAppEvents)

	// Get app event
	r.register(mcp.Tool{
		Name:        "get_app_event",
		Description: "Get details of a specific app event",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"event_id": {
					Type:        "string",
					Description: "The app event ID",
				},
			},
			Required: []string{"event_id"},
		},
	}, r.handleGetAppEvent)

	// Create app event
	r.register(mcp.Tool{
		Name:        "create_app_event",
		Description: "Create a new promotional app event",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID",
				},
				"reference_name": {
					Type:        "string",
					Description: "Internal reference name",
				},
				"badge": {
					Type:        "string",
					Description: "Event badge (LIVE_EVENT, PREMIERE, CHALLENGE, COMPETITION, NEW_SEASON, MAJOR_UPDATE, SPECIAL_EVENT)",
				},
				"deep_link": {
					Type:        "string",
					Description: "Deep link URL",
				},
				"purchase_requirement": {
					Type:        "string",
					Description: "Purchase requirement (NO_COST_ASSOCIATED, IN_APP_PURCHASE, SUBSCRIPTION, IN_APP_PURCHASE_AND_SUBSCRIPTION, IN_APP_PURCHASE_OR_SUBSCRIPTION)",
				},
				"primary_locale": {
					Type:        "string",
					Description: "Primary locale (e.g., en-US)",
				},
				"priority": {
					Type:        "string",
					Description: "Event priority (HIGH, NORMAL)",
				},
				"purpose": {
					Type:        "string",
					Description: "Event purpose (APPROPRIATE_FOR_ALL_USERS, ATTRACT_NEW_USERS, KEEP_ACTIVE_USERS_INFORMED, BRING_BACK_LAPSED_USERS)",
				},
			},
			Required: []string{"app_id", "reference_name"},
		},
	}, r.handleCreateAppEvent)

	// Update app event
	r.register(mcp.Tool{
		Name:        "update_app_event",
		Description: "Update an existing app event",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"event_id": {
					Type:        "string",
					Description: "The app event ID",
				},
				"reference_name": {
					Type:        "string",
					Description: "Updated internal reference name",
				},
				"badge": {
					Type:        "string",
					Description: "Updated event badge",
				},
				"deep_link": {
					Type:        "string",
					Description: "Updated deep link URL",
				},
				"purchase_requirement": {
					Type:        "string",
					Description: "Updated purchase requirement",
				},
				"priority": {
					Type:        "string",
					Description: "Updated event priority",
				},
				"purpose": {
					Type:        "string",
					Description: "Updated event purpose",
				},
			},
			Required: []string{"event_id"},
		},
	}, r.handleUpdateAppEvent)

	// Delete app event
	r.register(mcp.Tool{
		Name:        "delete_app_event",
		Description: "Delete an app event",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"event_id": {
					Type:        "string",
					Description: "The app event ID",
				},
			},
			Required: []string{"event_id"},
		},
	}, r.handleDeleteAppEvent)
}

func (r *Registry) handleListAppEvents(args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := r.client.ListAppEvents(context.Background(), params.AppID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list app events: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppEvents(resp.Data)), nil
}

func (r *Registry) handleGetAppEvent(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		EventID string `json:"event_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.EventID == "" {
		return nil, fmt.Errorf("event_id is required")
	}

	resp, err := r.client.GetAppEvent(context.Background(), params.EventID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app event: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppEvent(resp.Data)), nil
}

func (r *Registry) handleCreateAppEvent(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID               string `json:"app_id"`
		ReferenceName       string `json:"reference_name"`
		Badge               string `json:"badge"`
		DeepLink            string `json:"deep_link"`
		PurchaseRequirement string `json:"purchase_requirement"`
		PrimaryLocale       string `json:"primary_locale"`
		Priority            string `json:"priority"`
		Purpose             string `json:"purpose"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}
	if params.ReferenceName == "" {
		return nil, fmt.Errorf("reference_name is required")
	}

	req := &api.AppEventCreateRequest{
		Data: api.AppEventCreateData{
			Type: "appEvents",
			Attributes: api.AppEventCreateAttributes{
				ReferenceName:       params.ReferenceName,
				Badge:               params.Badge,
				DeepLink:            params.DeepLink,
				PurchaseRequirement: params.PurchaseRequirement,
				PrimaryLocale:       params.PrimaryLocale,
				Priority:            params.Priority,
				Purpose:             params.Purpose,
			},
			Relationships: api.AppEventCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "apps",
						ID:   params.AppID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateAppEvent(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create app event: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Created app event: %s (ID: %s)", resp.Data.Attributes.ReferenceName, resp.Data.ID)), nil
}

func (r *Registry) handleUpdateAppEvent(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		EventID             string `json:"event_id"`
		ReferenceName       string `json:"reference_name"`
		Badge               string `json:"badge"`
		DeepLink            string `json:"deep_link"`
		PurchaseRequirement string `json:"purchase_requirement"`
		Priority            string `json:"priority"`
		Purpose             string `json:"purpose"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.EventID == "" {
		return nil, fmt.Errorf("event_id is required")
	}

	req := &api.AppEventUpdateRequest{
		Data: api.AppEventUpdateData{
			Type: "appEvents",
			ID:   params.EventID,
			Attributes: api.AppEventUpdateAttributes{
				ReferenceName:       params.ReferenceName,
				Badge:               params.Badge,
				DeepLink:            params.DeepLink,
				PurchaseRequirement: params.PurchaseRequirement,
				Priority:            params.Priority,
				Purpose:             params.Purpose,
			},
		},
	}

	resp, err := r.client.UpdateAppEvent(context.Background(), params.EventID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update app event: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Updated app event: %s", resp.Data.ID)), nil
}

func (r *Registry) handleDeleteAppEvent(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		EventID string `json:"event_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.EventID == "" {
		return nil, fmt.Errorf("event_id is required")
	}

	err := r.client.DeleteAppEvent(context.Background(), params.EventID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete app event: %v", err)), nil
	}

	return mcp.NewSuccessResult("App event deleted successfully"), nil
}

func formatAppEvents(events []api.AppEvent) string {
	if len(events) == 0 {
		return "No app events found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d app events:\n\n", len(events)))

	for _, event := range events {
		sb.WriteString(formatAppEvent(event))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAppEvent(event api.AppEvent) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", event.ID))
	sb.WriteString(fmt.Sprintf("Reference Name: %s\n", event.Attributes.ReferenceName))
	if event.Attributes.Badge != "" {
		sb.WriteString(fmt.Sprintf("Badge: %s\n", event.Attributes.Badge))
	}
	if event.Attributes.EventState != "" {
		sb.WriteString(fmt.Sprintf("State: %s\n", event.Attributes.EventState))
	}
	if event.Attributes.Priority != "" {
		sb.WriteString(fmt.Sprintf("Priority: %s\n", event.Attributes.Priority))
	}
	if event.Attributes.Purpose != "" {
		sb.WriteString(fmt.Sprintf("Purpose: %s\n", event.Attributes.Purpose))
	}
	if event.Attributes.DeepLink != "" {
		sb.WriteString(fmt.Sprintf("Deep Link: %s\n", event.Attributes.DeepLink))
	}
	if event.Attributes.PurchaseRequirement != "" {
		sb.WriteString(fmt.Sprintf("Purchase Requirement: %s\n", event.Attributes.PurchaseRequirement))
	}
	return sb.String()
}
