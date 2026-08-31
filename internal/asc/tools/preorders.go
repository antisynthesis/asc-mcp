package tools

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerPreOrderTools registers pre-order tools.
func (r *Registry) registerPreOrderTools() {
	// End pre-order availability
	r.register(mcp.Tool{
		Name:        "end_app_availability_pre_order",
		Description: "End pre-order availability for one or more territory availabilities, releasing the app in those territories",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"territory_availability_ids": {
					Type:        "array",
					Description: "Territory availability IDs to end pre-order for (from list_territory_availabilities)",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"territory_availability_ids"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "End App Availability Pre Order",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleEndAppAvailabilityPreOrder)
}

func (r *Registry) handleEndAppAvailabilityPreOrder(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		TerritoryAvailabilityIDs []string `json:"territory_availability_ids"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if len(params.TerritoryAvailabilityIDs) == 0 {
		return mcp.NewErrorResult("territory_availability_ids is required"), nil
	}

	var availabilities []api.ResourceIdentifier
	for _, id := range params.TerritoryAvailabilityIDs {
		availabilities = append(availabilities, api.ResourceIdentifier{Type: "territoryAvailabilities", ID: id})
	}

	req := &api.EndAppAvailabilityPreOrderCreateRequest{
		Data: api.EndAppAvailabilityPreOrderCreateData{
			Type: "endAppAvailabilityPreOrders",
			Relationships: api.EndAppAvailabilityPreOrderCreateRelationships{
				TerritoryAvailabilities: api.RelationshipDataList{
					Data: availabilities,
				},
			},
		},
	}

	resp, err := r.client.EndAppAvailabilityPreOrder(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to end pre-order availability: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Pre-order ended for %d territory availabilities (request ID: %s)", len(params.TerritoryAvailabilityIDs), resp.Data.ID), resp.Data), nil
}
