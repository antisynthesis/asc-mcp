package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerGameCenterLeaderboardSetTools registers the v2 leaderboard set
// tools. A leaderboard set groups leaderboards under one name in the
// Game Center UI. On the v2 tree the set owns versions, a version owns
// the set's localizations, and a localization owns the set's image;
// member leaderboards are linked to the set itself and can carry a
// per-set localized name of their own.
func (r *Registry) registerGameCenterLeaderboardSetTools() {
	// List leaderboard sets
	r.register(mcp.Tool{
		Name:        "list_game_center_leaderboard_sets",
		Description: "List the Game Center leaderboard sets for an app via the v2 Game Center API",
		InputSchema: gcListSchema("game_center_detail_id", "The Game Center detail ID", "leaderboard sets"),
		Annotations: readOnlyGameCenterTool("List Game Center Leaderboard Sets"),
	}, r.handleListGameCenterLeaderboardSets)

	// Get leaderboard set
	r.register(mcp.Tool{
		Name:        "get_game_center_leaderboard_set",
		Description: "Get details of a specific Game Center leaderboard set",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"leaderboard_set_id": {
					Type:        "string",
					Description: "The leaderboard set ID",
				},
			},
			Required: []string{"leaderboard_set_id"},
		},
		Annotations: readOnlyGameCenterTool("Get Game Center Leaderboard Set"),
	}, r.handleGetGameCenterLeaderboardSet)

	// Create leaderboard set
	r.register(mcp.Tool{
		Name:        "create_game_center_leaderboard_set",
		Description: "Create a Game Center leaderboard set via the v2 Game Center API. Provide exactly one of game_center_detail_id (app-owned) or game_center_group_id (shared across a group). An initial version is created with the set; add leaderboards with add_game_center_leaderboard_set_members.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"game_center_detail_id": {
					Type:        "string",
					Description: "The Game Center detail ID to own the set",
				},
				"game_center_group_id": {
					Type:        "string",
					Description: "The Game Center group ID to own the set instead of a single app",
				},
				"reference_name": {
					Type:        "string",
					Description: "Internal reference name",
				},
				"vendor_identifier": {
					Type:        "string",
					Description: "Unique identifier for the leaderboard set",
				},
			},
			Required: []string{"reference_name", "vendor_identifier"},
		},
		Annotations: writeGameCenterTool("Create Game Center Leaderboard Set"),
	}, r.handleCreateGameCenterLeaderboardSet)

	// Update leaderboard set
	r.register(mcp.Tool{
		Name:        "update_game_center_leaderboard_set",
		Description: "Update a Game Center leaderboard set's reference name. The vendor identifier is immutable once the set exists.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"leaderboard_set_id": {
					Type:        "string",
					Description: "The leaderboard set ID",
				},
				"reference_name": {
					Type:        "string",
					Description: "Updated reference name",
				},
			},
			Required: []string{"leaderboard_set_id", "reference_name"},
		},
		Annotations: mutateGameCenterTool("Update Game Center Leaderboard Set"),
	}, r.handleUpdateGameCenterLeaderboardSet)

	// Delete leaderboard set
	r.register(mcp.Tool{
		Name:        "delete_game_center_leaderboard_set",
		Description: "Delete a Game Center leaderboard set",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"leaderboard_set_id": {
					Type:        "string",
					Description: "The leaderboard set ID",
				},
			},
			Required: []string{"leaderboard_set_id"},
		},
		Annotations: mutateGameCenterTool("Delete Game Center Leaderboard Set"),
	}, r.handleDeleteGameCenterLeaderboardSet)

	// List leaderboard set members
	r.register(mcp.Tool{
		Name:        "list_game_center_leaderboard_set_members",
		Description: "List the leaderboards that belong to a Game Center leaderboard set",
		InputSchema: gcListSchema("leaderboard_set_id", "The leaderboard set ID", "member leaderboards"),
		Annotations: readOnlyGameCenterTool("List Game Center Leaderboard Set Members"),
	}, r.handleListGameCenterLeaderboardSetMembers)

	// Add leaderboard set members
	r.register(mcp.Tool{
		Name:        "add_game_center_leaderboard_set_members",
		Description: "Add leaderboards to a Game Center leaderboard set",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"leaderboard_set_id": {
					Type:        "string",
					Description: "The leaderboard set ID",
				},
				"leaderboard_ids": {
					Type:        "array",
					Description: "Leaderboard IDs to add to the set",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"leaderboard_set_id", "leaderboard_ids"},
		},
		Annotations: writeGameCenterTool("Add Game Center Leaderboard Set Members"),
	}, r.handleAddGameCenterLeaderboardSetMembers)

	// Remove leaderboard set members
	r.register(mcp.Tool{
		Name:        "remove_game_center_leaderboard_set_members",
		Description: "Remove leaderboards from a Game Center leaderboard set. The leaderboards themselves are not deleted.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"leaderboard_set_id": {
					Type:        "string",
					Description: "The leaderboard set ID",
				},
				"leaderboard_ids": {
					Type:        "array",
					Description: "Leaderboard IDs to remove from the set",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"leaderboard_set_id", "leaderboard_ids"},
		},
		Annotations: mutateGameCenterTool("Remove Game Center Leaderboard Set Members"),
	}, r.handleRemoveGameCenterLeaderboardSetMembers)

	// List leaderboard set versions
	r.register(mcp.Tool{
		Name:        "list_game_center_leaderboard_set_versions",
		Description: "List the versions of a Game Center leaderboard set, including each version's review state",
		InputSchema: gcListSchema("leaderboard_set_id", "The leaderboard set ID", "versions"),
		Annotations: readOnlyGameCenterTool("List Game Center Leaderboard Set Versions"),
	}, r.handleListGameCenterLeaderboardSetVersions)

	// Get leaderboard set version
	r.register(mcp.Tool{
		Name:        "get_game_center_leaderboard_set_version",
		Description: "Get a specific Game Center leaderboard set version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The leaderboard set version ID",
				},
			},
			Required: []string{"version_id"},
		},
		Annotations: readOnlyGameCenterTool("Get Game Center Leaderboard Set Version"),
	}, r.handleGetGameCenterLeaderboardSetVersion)

	// Create leaderboard set version
	r.register(mcp.Tool{
		Name:        "create_game_center_leaderboard_set_version",
		Description: "Open a new editable version of a Game Center leaderboard set. Attach localizations to the new version, then submit it for review with add_review_submission_item.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"leaderboard_set_id": {
					Type:        "string",
					Description: "The leaderboard set ID to open a version for",
				},
			},
			Required: []string{"leaderboard_set_id"},
		},
		Annotations: writeGameCenterTool("Create Game Center Leaderboard Set Version"),
	}, r.handleCreateGameCenterLeaderboardSetVersion)

	// List leaderboard set localizations
	r.register(mcp.Tool{
		Name:        "list_game_center_leaderboard_set_localizations",
		Description: "List the localizations of a Game Center leaderboard set version",
		InputSchema: gcListSchema("version_id", "The leaderboard set version ID", "localizations"),
		Annotations: readOnlyGameCenterTool("List Game Center Leaderboard Set Localizations"),
	}, r.handleListGameCenterLeaderboardSetLocalizations)

	// Create leaderboard set localization
	r.register(mcp.Tool{
		Name:        "create_game_center_leaderboard_set_localization",
		Description: "Add a localized name for a Game Center leaderboard set version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The leaderboard set version ID",
				},
				"locale": {
					Type:        "string",
					Description: "The locale code (e.g. en-US)",
				},
				"name": {
					Type:        "string",
					Description: "The localized leaderboard set name shown to players",
				},
			},
			Required: []string{"version_id", "locale", "name"},
		},
		Annotations: writeGameCenterTool("Create Game Center Leaderboard Set Localization"),
	}, r.handleCreateGameCenterLeaderboardSetLocalization)

	// Update leaderboard set localization
	r.register(mcp.Tool{
		Name:        "update_game_center_leaderboard_set_localization",
		Description: "Update the localized name of a Game Center leaderboard set",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The leaderboard set localization ID",
				},
				"name": {
					Type:        "string",
					Description: "The updated localized name",
				},
			},
			Required: []string{"localization_id", "name"},
		},
		Annotations: mutateGameCenterTool("Update Game Center Leaderboard Set Localization"),
	}, r.handleUpdateGameCenterLeaderboardSetLocalization)

	// Delete leaderboard set localization
	r.register(mcp.Tool{
		Name:        "delete_game_center_leaderboard_set_localization",
		Description: "Delete a Game Center leaderboard set localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The leaderboard set localization ID",
				},
			},
			Required: []string{"localization_id"},
		},
		Annotations: mutateGameCenterTool("Delete Game Center Leaderboard Set Localization"),
	}, r.handleDeleteGameCenterLeaderboardSetLocalization)

	// Upload leaderboard set image
	r.register(mcp.Tool{
		Name:        "upload_game_center_leaderboard_set_image",
		Description: "Reserve, upload, and commit the image for a Game Center leaderboard set localization. Provide exactly one of file_path or file_data_base64.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The leaderboard set localization ID the image belongs to",
				},
				"file_name": {
					Type:        "string",
					Description: "The original file name (e.g. leaderboard-set.png)",
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
			Required: []string{"localization_id", "file_name"},
		},
		Annotations: writeGameCenterTool("Upload Game Center Leaderboard Set Image"),
	}, r.handleUploadGameCenterLeaderboardSetImage)

	// List leaderboard set member localizations
	r.register(mcp.Tool{
		Name:        "list_game_center_leaderboard_set_member_localizations",
		Description: "List the per-set localized names of a leaderboard within a leaderboard set. Both leaderboard_set_id and leaderboard_id are required.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"leaderboard_set_id": {
					Type:        "string",
					Description: "The leaderboard set the member localizations belong to",
				},
				"leaderboard_id": {
					Type:        "string",
					Description: "The leaderboard within that set",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of member localizations to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
				"include": {
					Type:        "array",
					Description: "Related resource names to include in the response (supported: gameCenterLeaderboard, gameCenterLeaderboardSet).",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"leaderboard_set_id", "leaderboard_id"},
		},
		Annotations: readOnlyGameCenterTool("List Game Center Leaderboard Set Member Localizations"),
	}, r.handleListGameCenterLeaderboardSetMemberLocalizations)

	// Create leaderboard set member localization
	r.register(mcp.Tool{
		Name:        "create_game_center_leaderboard_set_member_localization",
		Description: "Name a leaderboard as it appears inside a leaderboard set for one locale. This overrides the leaderboard's own localized name within that set.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"leaderboard_set_id": {
					Type:        "string",
					Description: "The leaderboard set ID",
				},
				"leaderboard_id": {
					Type:        "string",
					Description: "The member leaderboard ID",
				},
				"locale": {
					Type:        "string",
					Description: "The locale code (e.g. en-US)",
				},
				"name": {
					Type:        "string",
					Description: "The localized name for this leaderboard within the set",
				},
			},
			Required: []string{"leaderboard_set_id", "leaderboard_id", "locale", "name"},
		},
		Annotations: writeGameCenterTool("Create Game Center Leaderboard Set Member Localization"),
	}, r.handleCreateGameCenterLeaderboardSetMemberLocalization)

	// Update leaderboard set member localization
	r.register(mcp.Tool{
		Name:        "update_game_center_leaderboard_set_member_localization",
		Description: "Update the per-set localized name of a leaderboard",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"member_localization_id": {
					Type:        "string",
					Description: "The member localization ID",
				},
				"name": {
					Type:        "string",
					Description: "The updated localized name",
				},
			},
			Required: []string{"member_localization_id", "name"},
		},
		Annotations: mutateGameCenterTool("Update Game Center Leaderboard Set Member Localization"),
	}, r.handleUpdateGameCenterLeaderboardSetMemberLocalization)

	// Delete leaderboard set member localization
	r.register(mcp.Tool{
		Name:        "delete_game_center_leaderboard_set_member_localization",
		Description: "Delete a per-set localized leaderboard name",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"member_localization_id": {
					Type:        "string",
					Description: "The member localization ID",
				},
			},
			Required: []string{"member_localization_id"},
		},
		Annotations: mutateGameCenterTool("Delete Game Center Leaderboard Set Member Localization"),
	}, r.handleDeleteGameCenterLeaderboardSetMemberLocalization)
}

func (r *Registry) handleListGameCenterLeaderboardSets(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.GameCenterLeaderboardSetsResponse, error) {
		return r.client.ListGameCenterLeaderboardSets(ctx, params.GameCenterDetailID, params.opts())
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list leaderboard sets: %v", err)), nil
	}

	return newListResult(formatGameCenterLeaderboardSets(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetGameCenterLeaderboardSet(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LeaderboardSetID string `json:"leaderboard_set_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LeaderboardSetID == "" {
		return mcp.NewErrorResult("leaderboard_set_id is required"), nil
	}

	resp, err := r.client.GetGameCenterLeaderboardSet(ctx, params.LeaderboardSetID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get leaderboard set: %v", err)), nil
	}

	return newDataResult(formatGameCenterLeaderboardSet(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateGameCenterLeaderboardSet(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		GameCenterDetailID string `json:"game_center_detail_id"`
		GameCenterGroupID  string `json:"game_center_group_id"`
		ReferenceName      string `json:"reference_name"`
		VendorIdentifier   string `json:"vendor_identifier"`
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

	rels := api.GameCenterLeaderboardSetCreateRelationships{
		// The v2 API requires an initial leaderboard set version, created
		// inline with a client-chosen temporary ID.
		Versions: api.RelationshipDataList{
			Data: []api.ResourceIdentifier{
				{Type: "gameCenterLeaderboardSetVersions", ID: "${new-version}"},
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

	req := &api.GameCenterLeaderboardSetCreateRequest{
		Data: api.GameCenterLeaderboardSetCreateData{
			Type: "gameCenterLeaderboardSets",
			Attributes: api.GameCenterLeaderboardSetCreateAttributes{
				ReferenceName:    params.ReferenceName,
				VendorIdentifier: params.VendorIdentifier,
			},
			Relationships: rels,
		},
		Included: []api.GameCenterVersionInlineCreate{
			{Type: "gameCenterLeaderboardSetVersions", ID: "${new-version}"},
		},
	}

	resp, err := r.client.CreateGameCenterLeaderboardSet(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create leaderboard set: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created leaderboard set: %s (ID: %s)", resp.Data.Attributes.ReferenceName, resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleUpdateGameCenterLeaderboardSet(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LeaderboardSetID string `json:"leaderboard_set_id"`
		ReferenceName    string `json:"reference_name"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LeaderboardSetID == "" {
		return mcp.NewErrorResult("leaderboard_set_id is required"), nil
	}
	if params.ReferenceName == "" {
		return mcp.NewErrorResult("reference_name is required"), nil
	}

	req := &api.GameCenterLeaderboardSetUpdateRequest{
		Data: api.GameCenterLeaderboardSetUpdateData{
			Type: "gameCenterLeaderboardSets",
			ID:   params.LeaderboardSetID,
			Attributes: api.GameCenterLeaderboardSetUpdateAttributes{
				ReferenceName: params.ReferenceName,
			},
		},
	}

	resp, err := r.client.UpdateGameCenterLeaderboardSet(ctx, params.LeaderboardSetID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update leaderboard set: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Updated leaderboard set: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleDeleteGameCenterLeaderboardSet(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LeaderboardSetID string `json:"leaderboard_set_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LeaderboardSetID == "" {
		return mcp.NewErrorResult("leaderboard_set_id is required"), nil
	}

	if err := r.client.DeleteGameCenterLeaderboardSet(ctx, params.LeaderboardSetID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete leaderboard set: %v", err)), nil
	}

	return mcp.NewSuccessResult("Leaderboard set deleted successfully"), nil
}

func (r *Registry) handleListGameCenterLeaderboardSetMembers(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LeaderboardSetID string `json:"leaderboard_set_id"`
		gcListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LeaderboardSetID == "" {
		return mcp.NewErrorResult("leaderboard_set_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.GameCenterLeaderboardsResponse, error) {
		return r.client.ListGameCenterLeaderboardSetMembers(ctx, params.LeaderboardSetID, params.opts())
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list leaderboard set members: %v", err)), nil
	}

	return newListResult(formatGameCenterLeaderboards(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleAddGameCenterLeaderboardSetMembers(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	setID, ids, errResult, err := parseLeaderboardSetMembership(args)
	if err != nil {
		return nil, err
	}
	if errResult != nil {
		return errResult, nil
	}

	if err := r.client.AddGameCenterLeaderboardSetMembers(ctx, setID, ids); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to add leaderboard set members: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Added %d leaderboard(s) to set %s", len(ids.Data), setID)), nil
}

func (r *Registry) handleRemoveGameCenterLeaderboardSetMembers(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	setID, ids, errResult, err := parseLeaderboardSetMembership(args)
	if err != nil {
		return nil, err
	}
	if errResult != nil {
		return errResult, nil
	}

	if err := r.client.RemoveGameCenterLeaderboardSetMembers(ctx, setID, ids); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to remove leaderboard set members: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Removed %d leaderboard(s) from set %s", len(ids.Data), setID)), nil
}

// parseLeaderboardSetMembership decodes the arguments shared by the add
// and remove member tools into a linkages document. The returned
// *mcp.ToolsCallResult is non-nil when the arguments failed validation.
func parseLeaderboardSetMembership(args json.RawMessage) (string, *api.RelationshipDataList, *mcp.ToolsCallResult, error) {
	var params struct {
		LeaderboardSetID string   `json:"leaderboard_set_id"`
		LeaderboardIDs   []string `json:"leaderboard_ids"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return "", nil, nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LeaderboardSetID == "" {
		return "", nil, mcp.NewErrorResult("leaderboard_set_id is required"), nil
	}
	if len(params.LeaderboardIDs) == 0 {
		return "", nil, mcp.NewErrorResult("leaderboard_ids is required"), nil
	}

	linkages := &api.RelationshipDataList{Data: make([]api.ResourceIdentifier, 0, len(params.LeaderboardIDs))}
	for _, id := range params.LeaderboardIDs {
		if id == "" {
			continue
		}
		linkages.Data = append(linkages.Data, api.ResourceIdentifier{Type: "gameCenterLeaderboards", ID: id})
	}
	if len(linkages.Data) == 0 {
		return "", nil, mcp.NewErrorResult("leaderboard_ids is required"), nil
	}

	return params.LeaderboardSetID, linkages, nil, nil
}

func (r *Registry) handleListGameCenterLeaderboardSetVersions(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LeaderboardSetID string `json:"leaderboard_set_id"`
		gcListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LeaderboardSetID == "" {
		return mcp.NewErrorResult("leaderboard_set_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.GameCenterLeaderboardSetVersionsResponse, error) {
		return r.client.ListGameCenterLeaderboardSetVersions(ctx, params.LeaderboardSetID, params.opts())
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list leaderboard set versions: %v", err)), nil
	}

	return newListResult(formatGameCenterLeaderboardSetVersions(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetGameCenterLeaderboardSetVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	resp, err := r.client.GetGameCenterLeaderboardSetVersion(ctx, params.VersionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get leaderboard set version: %v", err)), nil
	}

	return newDataResult(formatGameCenterLeaderboardSetVersion(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateGameCenterLeaderboardSetVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LeaderboardSetID string `json:"leaderboard_set_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LeaderboardSetID == "" {
		return mcp.NewErrorResult("leaderboard_set_id is required"), nil
	}

	req := &api.GameCenterLeaderboardSetVersionCreateRequest{
		Data: api.GameCenterLeaderboardSetVersionCreateData{
			Type: "gameCenterLeaderboardSetVersions",
			Relationships: api.GameCenterLeaderboardSetVersionCreateRelationships{
				LeaderboardSet: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "gameCenterLeaderboardSets", ID: params.LeaderboardSetID},
				},
			},
		},
	}

	resp, err := r.client.CreateGameCenterLeaderboardSetVersion(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create leaderboard set version: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created leaderboard set version: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleListGameCenterLeaderboardSetLocalizations(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.GameCenterLeaderboardSetLocalizationsResponse, error) {
		return r.client.ListGameCenterLeaderboardSetLocalizations(ctx, params.VersionID, params.opts())
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list leaderboard set localizations: %v", err)), nil
	}

	return newListResult(formatGameCenterLeaderboardSetLocalizations(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleCreateGameCenterLeaderboardSetLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
		Locale    string `json:"locale"`
		Name      string `json:"name"`
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

	req := &api.GameCenterLeaderboardSetLocalizationCreateRequest{
		Data: api.GameCenterLeaderboardSetLocalizationCreateData{
			Type: "gameCenterLeaderboardSetLocalizations",
			Attributes: api.GameCenterLeaderboardSetLocalizationCreateAttributes{
				Locale: params.Locale,
				Name:   params.Name,
			},
			Relationships: api.GameCenterLeaderboardSetLocalizationCreateRelationships{
				Version: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "gameCenterLeaderboardSetVersions", ID: params.VersionID},
				},
			},
		},
	}

	resp, err := r.client.CreateGameCenterLeaderboardSetLocalization(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create leaderboard set localization: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created leaderboard set localization %s for %s", resp.Data.ID, params.Locale), resp.Data), nil
}

func (r *Registry) handleUpdateGameCenterLeaderboardSetLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
		Name           string `json:"name"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}
	if params.Name == "" {
		return mcp.NewErrorResult("name is required"), nil
	}

	req := &api.GameCenterLeaderboardSetLocalizationUpdateRequest{
		Data: api.GameCenterLeaderboardSetLocalizationUpdateData{
			Type: "gameCenterLeaderboardSetLocalizations",
			ID:   params.LocalizationID,
			Attributes: api.GameCenterLeaderboardSetLocalizationUpdateAttributes{
				Name: params.Name,
			},
		},
	}

	resp, err := r.client.UpdateGameCenterLeaderboardSetLocalization(ctx, params.LocalizationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update leaderboard set localization: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Updated leaderboard set localization: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleDeleteGameCenterLeaderboardSetLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	if err := r.client.DeleteGameCenterLeaderboardSetLocalization(ctx, params.LocalizationID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete leaderboard set localization: %v", err)), nil
	}

	return mcp.NewSuccessResult("Leaderboard set localization deleted successfully"), nil
}

func (r *Registry) handleUploadGameCenterLeaderboardSetImage(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
		FileName       string `json:"file_name"`
		FilePath       string `json:"file_path"`
		FileDataBase64 string `json:"file_data_base64"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}
	if params.FileName == "" {
		return mcp.NewErrorResult("file_name is required"), nil
	}

	body, err := loadUploadBody(params.FilePath, params.FileDataBase64)
	if err != nil {
		return mcp.NewErrorResult(err.Error()), nil
	}

	resp, err := r.client.UploadGameCenterLeaderboardSetImage(ctx, params.LocalizationID, params.FileName, body)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to upload leaderboard set image: %v", err)), nil
	}

	text := fmt.Sprintf("Uploaded leaderboard set image %s (%s, %d bytes) to localization %s.",
		resp.Data.ID, params.FileName, len(body), params.LocalizationID)
	return newDataResult(text, resp.Data), nil
}

func (r *Registry) handleListGameCenterLeaderboardSetMemberLocalizations(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LeaderboardSetID string   `json:"leaderboard_set_id"`
		LeaderboardID    string   `json:"leaderboard_id"`
		Limit            int      `json:"limit"`
		Cursor           string   `json:"cursor"`
		Include          []string `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	// The collection endpoint is addressable only through both of its
	// relationship filters; the API requires each one.
	if params.LeaderboardSetID == "" {
		return mcp.NewErrorResult("leaderboard_set_id is required"), nil
	}
	if params.LeaderboardID == "" {
		return mcp.NewErrorResult("leaderboard_id is required"), nil
	}
	filter := map[string][]string{
		"gameCenterLeaderboardSet": {params.LeaderboardSetID},
		"gameCenterLeaderboard":    {params.LeaderboardID},
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.GameCenterLeaderboardSetMemberLocalizationsResponse, error) {
		return r.client.ListGameCenterLeaderboardSetMemberLocalizations(ctx, listOpts(gcLimit(params.Limit), filter, nil, nil, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list member localizations: %v", err)), nil
	}

	return newListResult(formatGameCenterLeaderboardSetMemberLocalizations(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleCreateGameCenterLeaderboardSetMemberLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LeaderboardSetID string `json:"leaderboard_set_id"`
		LeaderboardID    string `json:"leaderboard_id"`
		Locale           string `json:"locale"`
		Name             string `json:"name"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LeaderboardSetID == "" {
		return mcp.NewErrorResult("leaderboard_set_id is required"), nil
	}
	if params.LeaderboardID == "" {
		return mcp.NewErrorResult("leaderboard_id is required"), nil
	}
	if params.Locale == "" {
		return mcp.NewErrorResult("locale is required"), nil
	}
	if params.Name == "" {
		return mcp.NewErrorResult("name is required"), nil
	}

	req := &api.GameCenterLeaderboardSetMemberLocalizationCreateRequest{
		Data: api.GameCenterLeaderboardSetMemberLocalizationCreateData{
			Type: "gameCenterLeaderboardSetMemberLocalizations",
			Attributes: api.GameCenterLeaderboardSetMemberLocalizationCreateAttributes{
				Locale: params.Locale,
				Name:   params.Name,
			},
			Relationships: api.GameCenterLeaderboardSetMemberLocalizationCreateRelationships{
				GameCenterLeaderboardSet: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "gameCenterLeaderboardSets", ID: params.LeaderboardSetID},
				},
				GameCenterLeaderboard: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "gameCenterLeaderboards", ID: params.LeaderboardID},
				},
			},
		},
	}

	resp, err := r.client.CreateGameCenterLeaderboardSetMemberLocalization(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create member localization: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created member localization %s for %s", resp.Data.ID, params.Locale), resp.Data), nil
}

func (r *Registry) handleUpdateGameCenterLeaderboardSetMemberLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		MemberLocalizationID string `json:"member_localization_id"`
		Name                 string `json:"name"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.MemberLocalizationID == "" {
		return mcp.NewErrorResult("member_localization_id is required"), nil
	}
	if params.Name == "" {
		return mcp.NewErrorResult("name is required"), nil
	}

	req := &api.GameCenterLeaderboardSetMemberLocalizationUpdateRequest{
		Data: api.GameCenterLeaderboardSetMemberLocalizationUpdateData{
			Type: "gameCenterLeaderboardSetMemberLocalizations",
			ID:   params.MemberLocalizationID,
			Attributes: api.GameCenterLeaderboardSetMemberLocalizationUpdateAttributes{
				Name: params.Name,
			},
		},
	}

	resp, err := r.client.UpdateGameCenterLeaderboardSetMemberLocalization(ctx, params.MemberLocalizationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update member localization: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Updated member localization: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleDeleteGameCenterLeaderboardSetMemberLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		MemberLocalizationID string `json:"member_localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.MemberLocalizationID == "" {
		return mcp.NewErrorResult("member_localization_id is required"), nil
	}

	if err := r.client.DeleteGameCenterLeaderboardSetMemberLocalization(ctx, params.MemberLocalizationID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete member localization: %v", err)), nil
	}

	return mcp.NewSuccessResult("Member localization deleted successfully"), nil
}

func formatGameCenterLeaderboardSets(sets []api.GameCenterLeaderboardSet) string {
	if len(sets) == 0 {
		return "No Game Center leaderboard sets found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d leaderboard sets:\n\n", len(sets)))

	for _, set := range sets {
		sb.WriteString(formatGameCenterLeaderboardSet(set))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatGameCenterLeaderboardSet(set api.GameCenterLeaderboardSet) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", set.ID))
	sb.WriteString(fmt.Sprintf("Reference Name: %s\n", set.Attributes.ReferenceName))
	sb.WriteString(fmt.Sprintf("Vendor ID: %s\n", set.Attributes.VendorIdentifier))
	return sb.String()
}

func formatGameCenterLeaderboardSetVersions(versions []api.GameCenterLeaderboardSetVersion) string {
	if len(versions) == 0 {
		return "No Game Center leaderboard set versions found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d leaderboard set versions:\n\n", len(versions)))

	for _, version := range versions {
		sb.WriteString(formatGameCenterLeaderboardSetVersion(version))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatGameCenterLeaderboardSetVersion(version api.GameCenterLeaderboardSetVersion) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", version.ID))
	sb.WriteString(fmt.Sprintf("Version: %d\n", version.Attributes.Version))
	sb.WriteString(fmt.Sprintf("State: %s\n", version.Attributes.State))
	return sb.String()
}

func formatGameCenterLeaderboardSetLocalizations(localizations []api.GameCenterLeaderboardSetLocalization) string {
	if len(localizations) == 0 {
		return "No Game Center leaderboard set localizations found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d leaderboard set localizations:\n\n", len(localizations)))

	for _, localization := range localizations {
		sb.WriteString(fmt.Sprintf("ID: %s\n", localization.ID))
		sb.WriteString(fmt.Sprintf("Locale: %s\n", localization.Attributes.Locale))
		sb.WriteString(fmt.Sprintf("Name: %s\n", localization.Attributes.Name))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatGameCenterLeaderboardSetMemberLocalizations(localizations []api.GameCenterLeaderboardSetMemberLocalization) string {
	if len(localizations) == 0 {
		return "No Game Center leaderboard set member localizations found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d member localizations:\n\n", len(localizations)))

	for _, localization := range localizations {
		sb.WriteString(fmt.Sprintf("ID: %s\n", localization.ID))
		sb.WriteString(fmt.Sprintf("Locale: %s\n", localization.Attributes.Locale))
		sb.WriteString(fmt.Sprintf("Name: %s\n", localization.Attributes.Name))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}
