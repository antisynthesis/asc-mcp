package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerGameCenterChallengeTools registers Game Center challenge tools
// (App Store Connect API 4.0+). A challenge is a competition between
// players scored by a leaderboard, so every challenge points at a v2
// leaderboard. Challenges are versioned like activities: the initial
// version is created inline, later versions are opened explicitly, and
// localizations hang off a version. The allowedDurations attribute was
// removed in API 4.1 and is deliberately not exposed.
func (r *Registry) registerGameCenterChallengeTools() {
	// List challenges
	r.register(mcp.Tool{
		Name:        "list_game_center_challenges",
		Description: "List the Game Center challenges for an app",
		InputSchema: gcListSchema("game_center_detail_id", "The Game Center detail ID", "challenges"),
		Annotations: readOnlyGameCenterTool("List Game Center Challenges"),
	}, r.handleListGameCenterChallenges)

	// Get challenge
	r.register(mcp.Tool{
		Name:        "get_game_center_challenge",
		Description: "Get details of a specific Game Center challenge",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"challenge_id": {
					Type:        "string",
					Description: "The challenge ID",
				},
			},
			Required: []string{"challenge_id"},
		},
		Annotations: readOnlyGameCenterTool("Get Game Center Challenge"),
	}, r.handleGetGameCenterChallenge)

	// Create challenge
	r.register(mcp.Tool{
		Name:        "create_game_center_challenge",
		Description: "Create a Game Center challenge along with its initial version. Provide exactly one of game_center_detail_id (app-owned) or game_center_group_id (shared across a group). A challenge is scored by the leaderboard given in leaderboard_id.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"game_center_detail_id": {
					Type:        "string",
					Description: "The Game Center detail ID to own the challenge",
				},
				"game_center_group_id": {
					Type:        "string",
					Description: "The Game Center group ID to own the challenge instead of a single app",
				},
				"reference_name": {
					Type:        "string",
					Description: "Internal reference name",
				},
				"vendor_identifier": {
					Type:        "string",
					Description: "Unique identifier for the challenge",
				},
				"challenge_type": {
					Type:        "string",
					Description: "Challenge type. Only LEADERBOARD is supported; defaults to LEADERBOARD.",
				},
				"leaderboard_id": {
					Type:        "string",
					Description: "The Game Center leaderboard ID the challenge scores against",
				},
				"repeatable": {
					Type:        "boolean",
					Description: "Whether players can take on the challenge more than once",
				},
			},
			Required: []string{"reference_name", "vendor_identifier"},
		},
		Annotations: writeGameCenterTool("Create Game Center Challenge"),
	}, r.handleCreateGameCenterChallenge)

	// Update challenge
	r.register(mcp.Tool{
		Name:        "update_game_center_challenge",
		Description: "Update a Game Center challenge. The vendor identifier and challenge type are immutable.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"challenge_id": {
					Type:        "string",
					Description: "The challenge ID",
				},
				"reference_name": {
					Type:        "string",
					Description: "Updated reference name",
				},
				"leaderboard_id": {
					Type:        "string",
					Description: "Point the challenge at a different Game Center leaderboard",
				},
				"repeatable": {
					Type:        "boolean",
					Description: "Whether players can take on the challenge more than once",
				},
				"archived": {
					Type:        "boolean",
					Description: "Archive the challenge",
				},
			},
			Required: []string{"challenge_id"},
		},
		Annotations: mutateGameCenterTool("Update Game Center Challenge"),
	}, r.handleUpdateGameCenterChallenge)

	// Delete challenge
	r.register(mcp.Tool{
		Name:        "delete_game_center_challenge",
		Description: "Delete a Game Center challenge",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"challenge_id": {
					Type:        "string",
					Description: "The challenge ID",
				},
			},
			Required: []string{"challenge_id"},
		},
		Annotations: mutateGameCenterTool("Delete Game Center Challenge"),
	}, r.handleDeleteGameCenterChallenge)

	// List challenge versions
	r.register(mcp.Tool{
		Name:        "list_game_center_challenge_versions",
		Description: "List the versions of a Game Center challenge, including each version's review state",
		InputSchema: gcListSchema("challenge_id", "The challenge ID", "versions"),
		Annotations: readOnlyGameCenterTool("List Game Center Challenge Versions"),
	}, r.handleListGameCenterChallengeVersions)

	// Get challenge version
	r.register(mcp.Tool{
		Name:        "get_game_center_challenge_version",
		Description: "Get a specific Game Center challenge version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The challenge version ID",
				},
			},
			Required: []string{"version_id"},
		},
		Annotations: readOnlyGameCenterTool("Get Game Center Challenge Version"),
	}, r.handleGetGameCenterChallengeVersion)

	// Create challenge version
	r.register(mcp.Tool{
		Name:        "create_game_center_challenge_version",
		Description: "Open a new editable version of a Game Center challenge. Attach localizations to the new version, then submit it for review with add_review_submission_item.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"challenge_id": {
					Type:        "string",
					Description: "The challenge ID to open a version for",
				},
			},
			Required: []string{"challenge_id"},
		},
		Annotations: writeGameCenterTool("Create Game Center Challenge Version"),
	}, r.handleCreateGameCenterChallengeVersion)

	// List challenge localizations
	r.register(mcp.Tool{
		Name:        "list_game_center_challenge_localizations",
		Description: "List the localizations of a Game Center challenge version",
		InputSchema: gcListSchema("version_id", "The challenge version ID", "localizations"),
		Annotations: readOnlyGameCenterTool("List Game Center Challenge Localizations"),
	}, r.handleListGameCenterChallengeLocalizations)

	// Create challenge localization
	r.register(mcp.Tool{
		Name:        "create_game_center_challenge_localization",
		Description: "Add a localized name and description for a Game Center challenge version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The challenge version ID",
				},
				"locale": {
					Type:        "string",
					Description: "The locale code (e.g. en-US)",
				},
				"name": {
					Type:        "string",
					Description: "The localized challenge name shown to players",
				},
				"description": {
					Type:        "string",
					Description: "The localized challenge description",
				},
			},
			Required: []string{"version_id", "locale", "name"},
		},
		Annotations: writeGameCenterTool("Create Game Center Challenge Localization"),
	}, r.handleCreateGameCenterChallengeLocalization)

	// Update challenge localization
	r.register(mcp.Tool{
		Name:        "update_game_center_challenge_localization",
		Description: "Update the localized name or description of a Game Center challenge",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The challenge localization ID",
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
		Annotations: mutateGameCenterTool("Update Game Center Challenge Localization"),
	}, r.handleUpdateGameCenterChallengeLocalization)

	// Delete challenge localization
	r.register(mcp.Tool{
		Name:        "delete_game_center_challenge_localization",
		Description: "Delete a Game Center challenge localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The challenge localization ID",
				},
			},
			Required: []string{"localization_id"},
		},
		Annotations: mutateGameCenterTool("Delete Game Center Challenge Localization"),
	}, r.handleDeleteGameCenterChallengeLocalization)

	// Upload challenge image
	r.register(mcp.Tool{
		Name:        "upload_game_center_challenge_image",
		Description: "Reserve, upload, and commit a Game Center challenge image. Provide exactly one of localization_id (that locale's image) or version_id (the version's default image), and exactly one of file_path or file_data_base64.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The challenge localization ID to attach the image to",
				},
				"version_id": {
					Type:        "string",
					Description: "The challenge version ID to attach the image to as its default image",
				},
				"file_name": {
					Type:        "string",
					Description: "The original file name (e.g. challenge.png)",
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
		Annotations: writeGameCenterTool("Upload Game Center Challenge Image"),
	}, r.handleUploadGameCenterChallengeImage)
}

func (r *Registry) handleListGameCenterChallenges(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.GameCenterChallengesResponse, error) {
		return r.client.ListGameCenterChallenges(ctx, params.GameCenterDetailID, params.opts())
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list challenges: %v", err)), nil
	}

	return newListResult(formatGameCenterChallenges(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetGameCenterChallenge(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ChallengeID string `json:"challenge_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ChallengeID == "" {
		return mcp.NewErrorResult("challenge_id is required"), nil
	}

	resp, err := r.client.GetGameCenterChallenge(ctx, params.ChallengeID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get challenge: %v", err)), nil
	}

	return newDataResult(formatGameCenterChallenge(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateGameCenterChallenge(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		GameCenterDetailID string `json:"game_center_detail_id"`
		GameCenterGroupID  string `json:"game_center_group_id"`
		ReferenceName      string `json:"reference_name"`
		VendorIdentifier   string `json:"vendor_identifier"`
		ChallengeType      string `json:"challenge_type"`
		LeaderboardID      string `json:"leaderboard_id"`
		Repeatable         *bool  `json:"repeatable"`
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

	// LEADERBOARD is the only challenge type the API accepts today.
	challengeType := params.ChallengeType
	if challengeType == "" {
		challengeType = "LEADERBOARD"
	}

	// The initial challenge version is declared inline with a
	// client-chosen temporary ID.
	rels := api.GameCenterChallengeCreateRelationships{
		Versions: &api.RelationshipDataList{
			Data: []api.ResourceIdentifier{
				{Type: "gameCenterChallengeVersions", ID: "${new-version}"},
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
	if params.LeaderboardID != "" {
		rels.LeaderboardV2 = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "gameCenterLeaderboards", ID: params.LeaderboardID},
		}
	}

	req := &api.GameCenterChallengeCreateRequest{
		Data: api.GameCenterChallengeCreateData{
			Type: "gameCenterChallenges",
			Attributes: api.GameCenterChallengeCreateAttributes{
				ReferenceName:    params.ReferenceName,
				VendorIdentifier: params.VendorIdentifier,
				ChallengeType:    challengeType,
				Repeatable:       params.Repeatable,
			},
			Relationships: rels,
		},
		Included: []api.GameCenterVersionInlineCreate{
			{Type: "gameCenterChallengeVersions", ID: "${new-version}"},
		},
	}

	resp, err := r.client.CreateGameCenterChallenge(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create challenge: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created challenge: %s (ID: %s)", resp.Data.Attributes.ReferenceName, resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleUpdateGameCenterChallenge(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ChallengeID   string `json:"challenge_id"`
		ReferenceName string `json:"reference_name"`
		LeaderboardID string `json:"leaderboard_id"`
		Repeatable    *bool  `json:"repeatable"`
		Archived      *bool  `json:"archived"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ChallengeID == "" {
		return mcp.NewErrorResult("challenge_id is required"), nil
	}

	req := &api.GameCenterChallengeUpdateRequest{
		Data: api.GameCenterChallengeUpdateData{
			Type: "gameCenterChallenges",
			ID:   params.ChallengeID,
			Attributes: api.GameCenterChallengeUpdateAttributes{
				ReferenceName: params.ReferenceName,
				Repeatable:    params.Repeatable,
				Archived:      params.Archived,
			},
		},
	}
	if params.LeaderboardID != "" {
		req.Data.Relationships = &api.GameCenterChallengeUpdateRelationships{
			LeaderboardV2: &api.RelationshipData{
				Data: api.ResourceIdentifier{Type: "gameCenterLeaderboards", ID: params.LeaderboardID},
			},
		}
	}

	resp, err := r.client.UpdateGameCenterChallenge(ctx, params.ChallengeID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update challenge: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Updated challenge: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleDeleteGameCenterChallenge(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ChallengeID string `json:"challenge_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ChallengeID == "" {
		return mcp.NewErrorResult("challenge_id is required"), nil
	}

	if err := r.client.DeleteGameCenterChallenge(ctx, params.ChallengeID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete challenge: %v", err)), nil
	}

	return mcp.NewSuccessResult("Challenge deleted successfully"), nil
}

func (r *Registry) handleListGameCenterChallengeVersions(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ChallengeID string `json:"challenge_id"`
		gcListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ChallengeID == "" {
		return mcp.NewErrorResult("challenge_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.GameCenterChallengeVersionsResponse, error) {
		return r.client.ListGameCenterChallengeVersions(ctx, params.ChallengeID, params.opts())
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list challenge versions: %v", err)), nil
	}

	return newListResult(formatGameCenterChallengeVersions(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetGameCenterChallengeVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	resp, err := r.client.GetGameCenterChallengeVersion(ctx, params.VersionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get challenge version: %v", err)), nil
	}

	return newDataResult(formatGameCenterChallengeVersion(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateGameCenterChallengeVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ChallengeID string `json:"challenge_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ChallengeID == "" {
		return mcp.NewErrorResult("challenge_id is required"), nil
	}

	req := &api.GameCenterChallengeVersionCreateRequest{
		Data: api.GameCenterChallengeVersionCreateData{
			Type: "gameCenterChallengeVersions",
			Relationships: api.GameCenterChallengeVersionCreateRelationships{
				Challenge: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "gameCenterChallenges", ID: params.ChallengeID},
				},
			},
		},
	}

	resp, err := r.client.CreateGameCenterChallengeVersion(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create challenge version: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created challenge version: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleListGameCenterChallengeLocalizations(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.GameCenterChallengeLocalizationsResponse, error) {
		return r.client.ListGameCenterChallengeLocalizations(ctx, params.VersionID, params.opts())
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list challenge localizations: %v", err)), nil
	}

	return newListResult(formatGameCenterChallengeLocalizations(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleCreateGameCenterChallengeLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	req := &api.GameCenterChallengeLocalizationCreateRequest{
		Data: api.GameCenterChallengeLocalizationCreateData{
			Type: "gameCenterChallengeLocalizations",
			Attributes: api.GameCenterChallengeLocalizationCreateAttributes{
				Locale:      params.Locale,
				Name:        params.Name,
				Description: params.Description,
			},
			Relationships: api.GameCenterChallengeLocalizationCreateRelationships{
				Version: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "gameCenterChallengeVersions", ID: params.VersionID},
				},
			},
		},
	}

	resp, err := r.client.CreateGameCenterChallengeLocalization(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create challenge localization: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created challenge localization %s for %s", resp.Data.ID, params.Locale), resp.Data), nil
}

func (r *Registry) handleUpdateGameCenterChallengeLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	req := &api.GameCenterChallengeLocalizationUpdateRequest{
		Data: api.GameCenterChallengeLocalizationUpdateData{
			Type: "gameCenterChallengeLocalizations",
			ID:   params.LocalizationID,
			Attributes: api.GameCenterChallengeLocalizationUpdateAttributes{
				Name:        params.Name,
				Description: params.Description,
			},
		},
	}

	resp, err := r.client.UpdateGameCenterChallengeLocalization(ctx, params.LocalizationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update challenge localization: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Updated challenge localization: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleDeleteGameCenterChallengeLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	if err := r.client.DeleteGameCenterChallengeLocalization(ctx, params.LocalizationID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete challenge localization: %v", err)), nil
	}

	return mcp.NewSuccessResult("Challenge localization deleted successfully"), nil
}

func (r *Registry) handleUploadGameCenterChallengeImage(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := r.client.UploadGameCenterChallengeImage(ctx, params.LocalizationID, params.VersionID, params.FileName, body)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to upload challenge image: %v", err)), nil
	}

	text := fmt.Sprintf("Uploaded challenge image %s (%s, %d bytes).", resp.Data.ID, params.FileName, len(body))
	return newDataResult(text, resp.Data), nil
}

func formatGameCenterChallenges(challenges []api.GameCenterChallenge) string {
	if len(challenges) == 0 {
		return "No Game Center challenges found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d challenges:\n\n", len(challenges)))

	for _, challenge := range challenges {
		sb.WriteString(formatGameCenterChallenge(challenge))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatGameCenterChallenge(challenge api.GameCenterChallenge) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", challenge.ID))
	sb.WriteString(fmt.Sprintf("Reference Name: %s\n", challenge.Attributes.ReferenceName))
	sb.WriteString(fmt.Sprintf("Vendor ID: %s\n", challenge.Attributes.VendorIdentifier))
	if challenge.Attributes.ChallengeType != "" {
		sb.WriteString(fmt.Sprintf("Challenge Type: %s\n", challenge.Attributes.ChallengeType))
	}
	sb.WriteString(fmt.Sprintf("Repeatable: %t\n", challenge.Attributes.Repeatable))
	sb.WriteString(fmt.Sprintf("Archived: %t\n", challenge.Attributes.Archived))
	return sb.String()
}

func formatGameCenterChallengeVersions(versions []api.GameCenterChallengeVersion) string {
	if len(versions) == 0 {
		return "No Game Center challenge versions found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d challenge versions:\n\n", len(versions)))

	for _, version := range versions {
		sb.WriteString(formatGameCenterChallengeVersion(version))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatGameCenterChallengeVersion(version api.GameCenterChallengeVersion) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", version.ID))
	sb.WriteString(fmt.Sprintf("Version: %d\n", version.Attributes.Version))
	sb.WriteString(fmt.Sprintf("State: %s\n", version.Attributes.State))
	return sb.String()
}

func formatGameCenterChallengeLocalizations(localizations []api.GameCenterChallengeLocalization) string {
	if len(localizations) == 0 {
		return "No Game Center challenge localizations found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d challenge localizations:\n\n", len(localizations)))

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
