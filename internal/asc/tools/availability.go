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
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get App Availability",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
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
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create App Availability",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
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
			Required: []string{"availability_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Territory Availabilities",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListTerritoryAvailabilities)
}

func (r *Registry) handleGetAppAvailability(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}

	resp, err := r.client.GetAppAvailability(ctx, params.AppID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app availability: %v", err)), nil
	}

	return newDataResult(formatAppAvailability(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateAppAvailability(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID                     string   `json:"app_id"`
		AvailableInNewTerritories *bool    `json:"available_in_new_territories"`
		TerritoryIDs              []string `json:"territory_ids"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}

	availInNew := true
	if params.AvailableInNewTerritories != nil {
		availInNew = *params.AvailableInNewTerritories
	}

	// The v2 endpoint takes territory availabilities as inline-created
	// resources referenced by temporary IDs.
	available := true
	var refs []api.ResourceIdentifier
	var included []api.TerritoryAvailabilityInlineCreate
	for _, tid := range params.TerritoryIDs {
		tempID := "${" + tid + "}"
		refs = append(refs, api.ResourceIdentifier{Type: "territoryAvailabilities", ID: tempID})
		included = append(included, api.TerritoryAvailabilityInlineCreate{
			Type: "territoryAvailabilities",
			ID:   tempID,
			Attributes: &api.TerritoryAvailabilityInlineAttributes{
				Available: &available,
			},
			Relationships: &api.TerritoryAvailabilityInlineCreateRelationships{
				Territory: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "territories", ID: tid},
				},
			},
		})
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
				TerritoryAvailabilities: api.RelationshipDataList{
					Data: refs,
				},
			},
		},
		Included: included,
	}

	resp, err := r.client.CreateAppAvailability(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create app availability: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("App availability created:\n%s", formatAppAvailability(resp.Data)), resp.Data), nil
}

func (r *Registry) handleListTerritoryAvailabilities(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AvailabilityID string              `json:"availability_id"`
		Limit          int                 `json:"limit"`
		Cursor         string              `json:"cursor"`
		Filter         map[string][]string `json:"filter"`
		Sort           []string            `json:"sort"`
		Fields         map[string][]string `json:"fields"`
		Include        []string            `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AvailabilityID == "" {
		return mcp.NewErrorResult("availability_id is required"), nil
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 100
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.TerritoryAvailabilitiesResponse, error) {
		return r.client.ListTerritoryAvailabilities(ctx, params.AvailabilityID, listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list territory availabilities: %v", err)), nil
	}

	return newListResult(formatTerritoryAvailabilities(resp.Data), resp.Data, resp.Links), nil
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
