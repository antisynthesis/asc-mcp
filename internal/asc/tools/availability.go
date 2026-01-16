package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerAvailabilityTools registers app availability tools.
func (r *Registry) registerAvailabilityTools() {
	// Get app availability
	r.register(mcp.Tool{
		Name:        "get_app_availability",
		Description: "Get the availability settings for an app",
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
	}, r.handleGetAppAvailability)

	// Create app availability
	r.register(mcp.Tool{
		Name:        "create_app_availability",
		Description: "Create or update availability settings for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
				"available_in_new_territories": {
					Type:        "boolean",
					Description: "Whether app should be available in new territories by default",
				},
				"territory_ids": {
					Type:        "array",
					Description: "List of territory IDs where the app should be available",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleCreateAppAvailability)

	// List territory availabilities
	r.register(mcp.Tool{
		Name:        "list_territory_availabilities",
		Description: "List territory availability settings for an app availability",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"availability_id": {
					Type:        "string",
					Description: "The app availability ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of results to return (default 100)",
				},
			},
			Required: []string{"availability_id"},
		},
	}, r.handleListTerritoryAvailabilities)
}

func (r *Registry) handleGetAppAvailability(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}

	resp, err := r.client.GetAppAvailability(context.Background(), params.AppID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app availability: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppAvailability(resp.Data)), nil
}

func (r *Registry) handleCreateAppAvailability(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID                     string   `json:"app_id"`
		AvailableInNewTerritories *bool    `json:"available_in_new_territories"`
		TerritoryIDs              []string `json:"territory_ids"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}

	availInNew := true
	if params.AvailableInNewTerritories != nil {
		availInNew = *params.AvailableInNewTerritories
	}

	var territories []api.ResourceIdentifier
	for _, tid := range params.TerritoryIDs {
		territories = append(territories, api.ResourceIdentifier{Type: "territories", ID: tid})
	}

	req := &api.AppAvailabilityCreateRequest{
		Data: api.AppAvailabilityCreateData{
			Type: "appAvailabilities",
			Attributes: api.AppAvailabilityCreateAttributes{
				AvailableInNewTerritories: availInNew,
			},
			Relationships: api.AppAvailabilityCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "apps", ID: params.AppID},
				},
				AvailableTerritories: api.RelationshipDataList{
					Data: territories,
				},
			},
		},
	}

	resp, err := r.client.CreateAppAvailability(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create app availability: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("App availability created:\n%s", formatAppAvailability(resp.Data))), nil
}

func (r *Registry) handleListTerritoryAvailabilities(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AvailabilityID string `json:"availability_id"`
		Limit          int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AvailabilityID == "" {
		return nil, fmt.Errorf("availability_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 100
	}

	resp, err := r.client.ListTerritoryAvailabilities(context.Background(), params.AvailabilityID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list territory availabilities: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatTerritoryAvailabilities(resp.Data)), nil
}

func formatAppAvailability(avail api.AppAvailability) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", avail.ID))
	sb.WriteString(fmt.Sprintf("Available in New Territories: %t\n", avail.Attributes.AvailableInNewTerritories))
	return sb.String()
}

func formatTerritoryAvailabilities(availabilities []api.TerritoryAvailability) string {
	if len(availabilities) == 0 {
		return "No territory availabilities found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d territory availabilities:\n\n", len(availabilities)))

	for _, avail := range availabilities {
		sb.WriteString(fmt.Sprintf("ID: %s\n", avail.ID))
		sb.WriteString(fmt.Sprintf("Available: %t\n", avail.Attributes.Available))
		sb.WriteString(fmt.Sprintf("Pre-Order Enabled: %t\n", avail.Attributes.PreOrderEnabled))
		if avail.Attributes.ReleaseDate != nil {
			sb.WriteString(fmt.Sprintf("Release Date: %s\n", avail.Attributes.ReleaseDate.Format("2006-01-02")))
		}
		sb.WriteString("\n---\n")
	}

	return sb.String()
}
