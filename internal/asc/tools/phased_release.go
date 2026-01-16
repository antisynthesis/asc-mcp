package tools

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerPhasedReleaseTools registers phased release tools.
func (r *Registry) registerPhasedReleaseTools() {
	// Get phased release
	r.register(mcp.Tool{
		Name:        "get_phased_release",
		Description: "Get phased release info for an App Store version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The App Store version ID",
				},
			},
			Required: []string{"version_id"},
		},
	}, r.handleGetPhasedRelease)

	// Create phased release
	r.register(mcp.Tool{
		Name:        "create_phased_release",
		Description: "Enable phased release for an App Store version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The App Store version ID",
				},
				"state": {
					Type:        "string",
					Description: "Initial state (INACTIVE, ACTIVE)",
				},
			},
			Required: []string{"version_id"},
		},
	}, r.handleCreatePhasedRelease)

	// Update phased release
	r.register(mcp.Tool{
		Name:        "update_phased_release",
		Description: "Update phased release (pause/resume)",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"phased_release_id": {
					Type:        "string",
					Description: "The phased release ID",
				},
				"state": {
					Type:        "string",
					Description: "New state (INACTIVE, ACTIVE, PAUSED, COMPLETE)",
				},
			},
			Required: []string{"phased_release_id", "state"},
		},
	}, r.handleUpdatePhasedRelease)

	// Delete phased release
	r.register(mcp.Tool{
		Name:        "delete_phased_release",
		Description: "Delete phased release (release to all users immediately)",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"phased_release_id": {
					Type:        "string",
					Description: "The phased release ID",
				},
			},
			Required: []string{"phased_release_id"},
		},
	}, r.handleDeletePhasedRelease)
}

func (r *Registry) handleGetPhasedRelease(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return nil, fmt.Errorf("version_id is required")
	}

	resp, err := r.client.GetAppStoreVersionPhasedRelease(context.Background(), params.VersionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get phased release: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatPhasedRelease(resp.Data)), nil
}

func (r *Registry) handleCreatePhasedRelease(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
		State     string `json:"state"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return nil, fmt.Errorf("version_id is required")
	}

	req := &api.AppStoreVersionPhasedReleaseCreateRequest{
		Data: api.AppStoreVersionPhasedReleaseCreateData{
			Type: "appStoreVersionPhasedReleases",
			Attributes: api.AppStoreVersionPhasedReleaseCreateAttributes{
				PhasedReleaseState: params.State,
			},
			Relationships: api.AppStoreVersionPhasedReleaseCreateRelationships{
				AppStoreVersion: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "appStoreVersions",
						ID:   params.VersionID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateAppStoreVersionPhasedRelease(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create phased release: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Created phased release: %s (state: %s)", resp.Data.ID, resp.Data.Attributes.PhasedReleaseState)), nil
}

func (r *Registry) handleUpdatePhasedRelease(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PhasedReleaseID string `json:"phased_release_id"`
		State           string `json:"state"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PhasedReleaseID == "" {
		return nil, fmt.Errorf("phased_release_id is required")
	}
	if params.State == "" {
		return nil, fmt.Errorf("state is required")
	}

	req := &api.AppStoreVersionPhasedReleaseUpdateRequest{
		Data: api.AppStoreVersionPhasedReleaseUpdateData{
			Type: "appStoreVersionPhasedReleases",
			ID:   params.PhasedReleaseID,
			Attributes: api.AppStoreVersionPhasedReleaseUpdateAttributes{
				PhasedReleaseState: params.State,
			},
		},
	}

	resp, err := r.client.UpdateAppStoreVersionPhasedRelease(context.Background(), params.PhasedReleaseID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update phased release: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Updated phased release: %s (state: %s)", resp.Data.ID, resp.Data.Attributes.PhasedReleaseState)), nil
}

func (r *Registry) handleDeletePhasedRelease(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PhasedReleaseID string `json:"phased_release_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.PhasedReleaseID == "" {
		return nil, fmt.Errorf("phased_release_id is required")
	}

	err := r.client.DeleteAppStoreVersionPhasedRelease(context.Background(), params.PhasedReleaseID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete phased release: %v", err)), nil
	}

	return mcp.NewSuccessResult("Phased release deleted - app will release to all users"), nil
}

func formatPhasedRelease(pr api.AppStoreVersionPhasedRelease) string {
	result := fmt.Sprintf("Phased Release ID: %s\n", pr.ID)
	result += fmt.Sprintf("State: %s\n", pr.Attributes.PhasedReleaseState)
	if pr.Attributes.StartDate != nil {
		result += fmt.Sprintf("Start Date: %s\n", pr.Attributes.StartDate.Format("2006-01-02"))
	}
	result += fmt.Sprintf("Current Day: %d\n", pr.Attributes.CurrentDayNumber)
	result += fmt.Sprintf("Total Pause Duration: %d days\n", pr.Attributes.TotalPauseDuration)
	return result
}
