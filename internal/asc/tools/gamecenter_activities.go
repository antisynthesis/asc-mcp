package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerGameCenterActivityTools registers Game Center activity tools
// (App Store Connect API 4.0+). An activity describes a play mode Game
// Center can launch into: solo or multiplayer, party-code capable, with
// a player count range and an arbitrary property bag handed to the game.
// Activities are versioned like every other Game Center content type,
// and the initial version — the only place a fallback URL can be set at
// creation time — is created inline with the activity.
func (r *Registry) registerGameCenterActivityTools() {
	// List activities
	r.register(mcp.Tool{
		Name:        "list_game_center_activities",
		Description: "List the Game Center activities for an app",
		InputSchema: gcListSchema("game_center_detail_id", "The Game Center detail ID", "activities"),
		Annotations: readOnlyGameCenterTool("List Game Center Activities"),
	}, r.handleListGameCenterActivities)

	// Get activity
	r.register(mcp.Tool{
		Name:        "get_game_center_activity",
		Description: "Get details of a specific Game Center activity",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"activity_id": {
					Type:        "string",
					Description: "The activity ID",
				},
			},
			Required: []string{"activity_id"},
		},
		Annotations: readOnlyGameCenterTool("Get Game Center Activity"),
	}, r.handleGetGameCenterActivity)

	// Create activity
	r.register(mcp.Tool{
		Name:        "create_game_center_activity",
		Description: "Create a Game Center activity along with its initial version. Provide exactly one of game_center_detail_id (app-owned) or game_center_group_id (shared across a group). fallback_url is set on the inline initial version and is opened when a player's device cannot launch the activity.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"game_center_detail_id": {
					Type:        "string",
					Description: "The Game Center detail ID to own the activity",
				},
				"game_center_group_id": {
					Type:        "string",
					Description: "The Game Center group ID to own the activity instead of a single app",
				},
				"reference_name": {
					Type:        "string",
					Description: "Internal reference name",
				},
				"vendor_identifier": {
					Type:        "string",
					Description: "Unique identifier for the activity",
				},
				"play_style": {
					Type:        "string",
					Description: "Play style: ASYNCHRONOUS or SYNCHRONOUS",
				},
				"minimum_players_count": {
					Type:        "integer",
					Description: "Minimum number of players the activity supports",
				},
				"maximum_players_count": {
					Type:        "integer",
					Description: "Maximum number of players the activity supports",
				},
				"supports_party_code": {
					Type:        "boolean",
					Description: "Whether players can join the activity with a party code",
				},
				"properties": {
					Type:        "object",
					Description: "Arbitrary string key/value pairs handed to the game when the activity launches",
				},
				"fallback_url": {
					Type:        "string",
					Description: "URL opened when the player's device cannot launch the activity. Set on the inline initial version.",
				},
			},
			Required: []string{"reference_name", "vendor_identifier"},
		},
		Annotations: writeGameCenterTool("Create Game Center Activity"),
	}, r.handleCreateGameCenterActivity)

	// Update activity
	r.register(mcp.Tool{
		Name:        "update_game_center_activity",
		Description: "Update a Game Center activity. The vendor identifier is immutable; use update_game_center_activity_version to change the fallback URL.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"activity_id": {
					Type:        "string",
					Description: "The activity ID",
				},
				"reference_name": {
					Type:        "string",
					Description: "Updated reference name",
				},
				"play_style": {
					Type:        "string",
					Description: "Updated play style: ASYNCHRONOUS or SYNCHRONOUS",
				},
				"minimum_players_count": {
					Type:        "integer",
					Description: "Updated minimum number of players",
				},
				"maximum_players_count": {
					Type:        "integer",
					Description: "Updated maximum number of players",
				},
				"supports_party_code": {
					Type:        "boolean",
					Description: "Whether players can join the activity with a party code",
				},
				"properties": {
					Type:        "object",
					Description: "Replacement string key/value pairs handed to the game when the activity launches",
				},
				"archived": {
					Type:        "boolean",
					Description: "Archive the activity",
				},
			},
			Required: []string{"activity_id"},
		},
		Annotations: mutateGameCenterTool("Update Game Center Activity"),
	}, r.handleUpdateGameCenterActivity)

	// Delete activity
	r.register(mcp.Tool{
		Name:        "delete_game_center_activity",
		Description: "Delete a Game Center activity",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"activity_id": {
					Type:        "string",
					Description: "The activity ID",
				},
			},
			Required: []string{"activity_id"},
		},
		Annotations: mutateGameCenterTool("Delete Game Center Activity"),
	}, r.handleDeleteGameCenterActivity)

	// List activity versions
	r.register(mcp.Tool{
		Name:        "list_game_center_activity_versions",
		Description: "List the versions of a Game Center activity, including each version's review state and fallback URL",
		InputSchema: gcListSchema("activity_id", "The activity ID", "versions"),
		Annotations: readOnlyGameCenterTool("List Game Center Activity Versions"),
	}, r.handleListGameCenterActivityVersions)

	// Get activity version
	r.register(mcp.Tool{
		Name:        "get_game_center_activity_version",
		Description: "Get a specific Game Center activity version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The activity version ID",
				},
			},
			Required: []string{"version_id"},
		},
		Annotations: readOnlyGameCenterTool("Get Game Center Activity Version"),
	}, r.handleGetGameCenterActivityVersion)

	// Create activity version
	r.register(mcp.Tool{
		Name:        "create_game_center_activity_version",
		Description: "Open a new editable version of a Game Center activity. Attach localizations to the new version, then submit it for review with add_review_submission_item.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"activity_id": {
					Type:        "string",
					Description: "The activity ID to open a version for",
				},
				"fallback_url": {
					Type:        "string",
					Description: "URL opened when the player's device cannot launch the activity",
				},
			},
			Required: []string{"activity_id"},
		},
		Annotations: writeGameCenterTool("Create Game Center Activity Version"),
	}, r.handleCreateGameCenterActivityVersion)

	// Update activity version
	r.register(mcp.Tool{
		Name:        "update_game_center_activity_version",
		Description: "Update the fallback URL of a Game Center activity version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The activity version ID",
				},
				"fallback_url": {
					Type:        "string",
					Description: "URL opened when the player's device cannot launch the activity",
				},
			},
			Required: []string{"version_id", "fallback_url"},
		},
		Annotations: mutateGameCenterTool("Update Game Center Activity Version"),
	}, r.handleUpdateGameCenterActivityVersion)

	// List activity localizations
	r.register(mcp.Tool{
		Name:        "list_game_center_activity_localizations",
		Description: "List the localizations of a Game Center activity version",
		InputSchema: gcListSchema("version_id", "The activity version ID", "localizations"),
		Annotations: readOnlyGameCenterTool("List Game Center Activity Localizations"),
	}, r.handleListGameCenterActivityLocalizations)

	// Create activity localization
	r.register(mcp.Tool{
		Name:        "create_game_center_activity_localization",
		Description: "Add a localized name and description for a Game Center activity version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The activity version ID",
				},
				"locale": {
					Type:        "string",
					Description: "The locale code (e.g. en-US)",
				},
				"name": {
					Type:        "string",
					Description: "The localized activity name shown to players",
				},
				"description": {
					Type:        "string",
					Description: "The localized activity description",
				},
			},
			Required: []string{"version_id", "locale", "name"},
		},
		Annotations: writeGameCenterTool("Create Game Center Activity Localization"),
	}, r.handleCreateGameCenterActivityLocalization)

	// Update activity localization
	r.register(mcp.Tool{
		Name:        "update_game_center_activity_localization",
		Description: "Update the localized name or description of a Game Center activity",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The activity localization ID",
				},
				"name": {
					Type:        "string",
					Description: "The updated localized name",
				},
				"description": {
					Type:        "string",
					Description: "The updated localized description",
				},
			},
			Required: []string{"localization_id"},
		},
		Annotations: mutateGameCenterTool("Update Game Center Activity Localization"),
	}, r.handleUpdateGameCenterActivityLocalization)

	// Delete activity localization
	r.register(mcp.Tool{
		Name:        "delete_game_center_activity_localization",
		Description: "Delete a Game Center activity localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The activity localization ID",
				},
			},
			Required: []string{"localization_id"},
		},
		Annotations: mutateGameCenterTool("Delete Game Center Activity Localization"),
	}, r.handleDeleteGameCenterActivityLocalization)

	// Upload activity image
	r.register(mcp.Tool{
		Name:        "upload_game_center_activity_image",
		Description: "Reserve, upload, and commit a Game Center activity image. Provide exactly one of localization_id (that locale's image) or version_id (the version's default image), and exactly one of file_path or file_data_base64.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The activity localization ID to attach the image to",
				},
				"version_id": {
					Type:        "string",
					Description: "The activity version ID to attach the image to as its default image",
				},
				"file_name": {
					Type:        "string",
					Description: "The original file name (e.g. activity.png)",
				},
				"file_path": {
					Type:        "string",
					Description: "Absolute path to the file on the server's local filesystem. Use this when the MCP server and the file are on the same host (most stdio scenarios).",
				},
				"file_data_base64": {
					Type:        "string",
					Description: "Standard base64 (RFC 4648) encoded bytes of the file. Use this when the client must transport the file across a remote transport such as HTTP.",
				},
			},
			Required: []string{"file_name"},
		},
		Annotations: writeGameCenterTool("Upload Game Center Activity Image"),
	}, r.handleUploadGameCenterActivityImage)
}

func (r *Registry) handleListGameCenterActivities(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		GameCenterDetailID string `json:"game_center_detail_id"`
		gcListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.GameCenterDetailID == "" {
		return mcp.NewErrorResult("game_center_detail_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.GameCenterActivitiesResponse, error) {
		return r.client.ListGameCenterActivities(ctx, params.GameCenterDetailID, params.opts())
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list activities: %v", err)), nil
	}

	return newListResult(formatGameCenterActivities(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetGameCenterActivity(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ActivityID string `json:"activity_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ActivityID == "" {
		return mcp.NewErrorResult("activity_id is required"), nil
	}

	resp, err := r.client.GetGameCenterActivity(ctx, params.ActivityID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get activity: %v", err)), nil
	}

	return newDataResult(formatGameCenterActivity(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateGameCenterActivity(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		GameCenterDetailID  string            `json:"game_center_detail_id"`
		GameCenterGroupID   string            `json:"game_center_group_id"`
		ReferenceName       string            `json:"reference_name"`
		VendorIdentifier    string            `json:"vendor_identifier"`
		PlayStyle           string            `json:"play_style"`
		MinimumPlayersCount *int              `json:"minimum_players_count"`
		MaximumPlayersCount *int              `json:"maximum_players_count"`
		SupportsPartyCode   *bool             `json:"supports_party_code"`
		Properties          map[string]string `json:"properties"`
		FallbackURL         string            `json:"fallback_url"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ReferenceName == "" {
		return mcp.NewErrorResult("reference_name is required"), nil
	}
	if params.VendorIdentifier == "" {
		return mcp.NewErrorResult("vendor_identifier is required"), nil
	}
	if (params.GameCenterDetailID == "") == (params.GameCenterGroupID == "") {
		return mcp.NewErrorResult("exactly one of game_center_detail_id or game_center_group_id is required"), nil
	}

	// The initial activity version is declared inline with a
	// client-chosen temporary ID; it carries the fallback URL.
	rels := api.GameCenterActivityCreateRelationships{
		Versions: &api.RelationshipDataList{
			Data: []api.ResourceIdentifier{
				{Type: "gameCenterActivityVersions", ID: "${new-version}"},
			},
		},
	}
	if params.GameCenterDetailID != "" {
		rels.GameCenterDetail = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "gameCenterDetails", ID: params.GameCenterDetailID},
		}
	} else {
		rels.GameCenterGroup = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "gameCenterGroups", ID: params.GameCenterGroupID},
		}
	}

	inline := api.GameCenterActivityVersionInlineCreate{
		Type: "gameCenterActivityVersions",
		ID:   "${new-version}",
	}
	if params.FallbackURL != "" {
		inline.Attributes = &api.GameCenterActivityVersionInlineCreateAttributes{
			FallbackURL: params.FallbackURL,
		}
	}

	req := &api.GameCenterActivityCreateRequest{
		Data: api.GameCenterActivityCreateData{
			Type: "gameCenterActivities",
			Attributes: api.GameCenterActivityCreateAttributes{
				ReferenceName:       params.ReferenceName,
				VendorIdentifier:    params.VendorIdentifier,
				PlayStyle:           params.PlayStyle,
				MinimumPlayersCount: params.MinimumPlayersCount,
				MaximumPlayersCount: params.MaximumPlayersCount,
				SupportsPartyCode:   params.SupportsPartyCode,
				Properties:          params.Properties,
			},
			Relationships: rels,
		},
		Included: []api.GameCenterActivityVersionInlineCreate{inline},
	}

	resp, err := r.client.CreateGameCenterActivity(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create activity: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created activity: %s (ID: %s)", resp.Data.Attributes.ReferenceName, resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleUpdateGameCenterActivity(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ActivityID          string            `json:"activity_id"`
		ReferenceName       string            `json:"reference_name"`
		PlayStyle           string            `json:"play_style"`
		MinimumPlayersCount *int              `json:"minimum_players_count"`
		MaximumPlayersCount *int              `json:"maximum_players_count"`
		SupportsPartyCode   *bool             `json:"supports_party_code"`
		Properties          map[string]string `json:"properties"`
		Archived            *bool             `json:"archived"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ActivityID == "" {
		return mcp.NewErrorResult("activity_id is required"), nil
	}

	req := &api.GameCenterActivityUpdateRequest{
		Data: api.GameCenterActivityUpdateData{
			Type: "gameCenterActivities",
			ID:   params.ActivityID,
			Attributes: api.GameCenterActivityUpdateAttributes{
				ReferenceName:       params.ReferenceName,
				PlayStyle:           params.PlayStyle,
				MinimumPlayersCount: params.MinimumPlayersCount,
				MaximumPlayersCount: params.MaximumPlayersCount,
				SupportsPartyCode:   params.SupportsPartyCode,
				Properties:          params.Properties,
				Archived:            params.Archived,
			},
		},
	}

	resp, err := r.client.UpdateGameCenterActivity(ctx, params.ActivityID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update activity: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Updated activity: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleDeleteGameCenterActivity(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ActivityID string `json:"activity_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ActivityID == "" {
		return mcp.NewErrorResult("activity_id is required"), nil
	}

	if err := r.client.DeleteGameCenterActivity(ctx, params.ActivityID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete activity: %v", err)), nil
	}

	return mcp.NewSuccessResult("Activity deleted successfully"), nil
}

func (r *Registry) handleListGameCenterActivityVersions(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ActivityID string `json:"activity_id"`
		gcListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ActivityID == "" {
		return mcp.NewErrorResult("activity_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.GameCenterActivityVersionsResponse, error) {
		return r.client.ListGameCenterActivityVersions(ctx, params.ActivityID, params.opts())
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list activity versions: %v", err)), nil
	}

	return newListResult(formatGameCenterActivityVersions(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetGameCenterActivityVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	resp, err := r.client.GetGameCenterActivityVersion(ctx, params.VersionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get activity version: %v", err)), nil
	}

	return newDataResult(formatGameCenterActivityVersion(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateGameCenterActivityVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ActivityID  string `json:"activity_id"`
		FallbackURL string `json:"fallback_url"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ActivityID == "" {
		return mcp.NewErrorResult("activity_id is required"), nil
	}

	req := &api.GameCenterActivityVersionCreateRequest{
		Data: api.GameCenterActivityVersionCreateData{
			Type: "gameCenterActivityVersions",
			Relationships: api.GameCenterActivityVersionCreateRelationships{
				Activity: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "gameCenterActivities", ID: params.ActivityID},
				},
			},
		},
	}
	if params.FallbackURL != "" {
		req.Data.Attributes = &api.GameCenterActivityVersionCreateAttributes{
			FallbackURL: params.FallbackURL,
		}
	}

	resp, err := r.client.CreateGameCenterActivityVersion(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create activity version: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created activity version: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleUpdateGameCenterActivityVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID   string `json:"version_id"`
		FallbackURL string `json:"fallback_url"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}
	if params.FallbackURL == "" {
		return mcp.NewErrorResult("fallback_url is required"), nil
	}

	req := &api.GameCenterActivityVersionUpdateRequest{
		Data: api.GameCenterActivityVersionUpdateData{
			Type: "gameCenterActivityVersions",
			ID:   params.VersionID,
			Attributes: api.GameCenterActivityVersionUpdateAttributes{
				FallbackURL: params.FallbackURL,
			},
		},
	}

	resp, err := r.client.UpdateGameCenterActivityVersion(ctx, params.VersionID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update activity version: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Updated activity version: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleListGameCenterActivityLocalizations(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
		gcListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.GameCenterActivityLocalizationsResponse, error) {
		return r.client.ListGameCenterActivityLocalizations(ctx, params.VersionID, params.opts())
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list activity localizations: %v", err)), nil
	}

	return newListResult(formatGameCenterActivityLocalizations(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleCreateGameCenterActivityLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID   string `json:"version_id"`
		Locale      string `json:"locale"`
		Name        string `json:"name"`
		Description string `json:"description"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}
	if params.Locale == "" {
		return mcp.NewErrorResult("locale is required"), nil
	}
	if params.Name == "" {
		return mcp.NewErrorResult("name is required"), nil
	}

	req := &api.GameCenterActivityLocalizationCreateRequest{
		Data: api.GameCenterActivityLocalizationCreateData{
			Type: "gameCenterActivityLocalizations",
			Attributes: api.GameCenterActivityLocalizationCreateAttributes{
				Locale:      params.Locale,
				Name:        params.Name,
				Description: params.Description,
			},
			Relationships: api.GameCenterActivityLocalizationCreateRelationships{
				Version: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "gameCenterActivityVersions", ID: params.VersionID},
				},
			},
		},
	}

	resp, err := r.client.CreateGameCenterActivityLocalization(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create activity localization: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created activity localization %s for %s", resp.Data.ID, params.Locale), resp.Data), nil
}

func (r *Registry) handleUpdateGameCenterActivityLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
		Name           string `json:"name"`
		Description    string `json:"description"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}
	if params.Name == "" && params.Description == "" {
		return mcp.NewErrorResult("at least one of name or description is required"), nil
	}

	req := &api.GameCenterActivityLocalizationUpdateRequest{
		Data: api.GameCenterActivityLocalizationUpdateData{
			Type: "gameCenterActivityLocalizations",
			ID:   params.LocalizationID,
			Attributes: api.GameCenterActivityLocalizationUpdateAttributes{
				Name:        params.Name,
				Description: params.Description,
			},
		},
	}

	resp, err := r.client.UpdateGameCenterActivityLocalization(ctx, params.LocalizationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update activity localization: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Updated activity localization: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleDeleteGameCenterActivityLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	if err := r.client.DeleteGameCenterActivityLocalization(ctx, params.LocalizationID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete activity localization: %v", err)), nil
	}

	return mcp.NewSuccessResult("Activity localization deleted successfully"), nil
}

func (r *Registry) handleUploadGameCenterActivityImage(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
		VersionID      string `json:"version_id"`
		FileName       string `json:"file_name"`
		FilePath       string `json:"file_path"`
		FileDataBase64 string `json:"file_data_base64"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.FileName == "" {
		return mcp.NewErrorResult("file_name is required"), nil
	}
	if (params.LocalizationID == "") == (params.VersionID == "") {
		return mcp.NewErrorResult("exactly one of localization_id or version_id is required"), nil
	}

	body, err := loadUploadBody(params.FilePath, params.FileDataBase64)
	if err != nil {
		return mcp.NewErrorResult(err.Error()), nil
	}

	resp, err := r.client.UploadGameCenterActivityImage(ctx, params.LocalizationID, params.VersionID, params.FileName, body)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to upload activity image: %v", err)), nil
	}

	text := fmt.Sprintf("Uploaded activity image %s (%s, %d bytes).", resp.Data.ID, params.FileName, len(body))
	return newDataResult(text, resp.Data), nil
}

func formatGameCenterActivities(activities []api.GameCenterActivity) string {
	if len(activities) == 0 {
		return "No Game Center activities found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d activities:\n\n", len(activities)))

	for _, activity := range activities {
		sb.WriteString(formatGameCenterActivity(activity))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatGameCenterActivity(activity api.GameCenterActivity) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", activity.ID))
	sb.WriteString(fmt.Sprintf("Reference Name: %s\n", activity.Attributes.ReferenceName))
	sb.WriteString(fmt.Sprintf("Vendor ID: %s\n", activity.Attributes.VendorIdentifier))
	if activity.Attributes.PlayStyle != "" {
		sb.WriteString(fmt.Sprintf("Play Style: %s\n", activity.Attributes.PlayStyle))
	}
	if activity.Attributes.MinimumPlayersCount > 0 || activity.Attributes.MaximumPlayersCount > 0 {
		sb.WriteString(fmt.Sprintf("Players: %d - %d\n", activity.Attributes.MinimumPlayersCount, activity.Attributes.MaximumPlayersCount))
	}
	sb.WriteString(fmt.Sprintf("Supports Party Code: %t\n", activity.Attributes.SupportsPartyCode))
	sb.WriteString(fmt.Sprintf("Archived: %t\n", activity.Attributes.Archived))
	return sb.String()
}

func formatGameCenterActivityVersions(versions []api.GameCenterActivityVersion) string {
	if len(versions) == 0 {
		return "No Game Center activity versions found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d activity versions:\n\n", len(versions)))

	for _, version := range versions {
		sb.WriteString(formatGameCenterActivityVersion(version))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatGameCenterActivityVersion(version api.GameCenterActivityVersion) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", version.ID))
	sb.WriteString(fmt.Sprintf("Version: %d\n", version.Attributes.Version))
	sb.WriteString(fmt.Sprintf("State: %s\n", version.Attributes.State))
	if version.Attributes.FallbackURL != "" {
		sb.WriteString(fmt.Sprintf("Fallback URL: %s\n", version.Attributes.FallbackURL))
	}
	return sb.String()
}

func formatGameCenterActivityLocalizations(localizations []api.GameCenterActivityLocalization) string {
	if len(localizations) == 0 {
		return "No Game Center activity localizations found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d activity localizations:\n\n", len(localizations)))

	for _, localization := range localizations {
		sb.WriteString(fmt.Sprintf("ID: %s\n", localization.ID))
		sb.WriteString(fmt.Sprintf("Locale: %s\n", localization.Attributes.Locale))
		sb.WriteString(fmt.Sprintf("Name: %s\n", localization.Attributes.Name))
		if localization.Attributes.Description != "" {
			sb.WriteString(fmt.Sprintf("Description: %s\n", localization.Attributes.Description))
		}
		sb.WriteString("\n---\n")
	}

	return sb.String()
}
