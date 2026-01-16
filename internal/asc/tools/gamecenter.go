package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerGameCenterTools registers Game Center tools.
func (r *Registry) registerGameCenterTools() {
	// Get Game Center detail
	r.register(mcp.Tool{
		Name:        "get_game_center_detail",
		Description: "Get Game Center details for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleGetGameCenterDetail)

	// List Game Center achievements
	r.register(mcp.Tool{
		Name:        "list_game_center_achievements",
		Description: "List Game Center achievements for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"game_center_detail_id": {
					Type:        "string",
					Description: "The Game Center detail ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of achievements to return (default 50)",
				},
			},
			Required: []string{"game_center_detail_id"},
		},
	}, r.handleListGameCenterAchievements)

	// Get Game Center achievement
	r.register(mcp.Tool{
		Name:        "get_game_center_achievement",
		Description: "Get details of a specific Game Center achievement",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"achievement_id": {
					Type:        "string",
					Description: "The achievement ID",
				},
			},
			Required: []string{"achievement_id"},
		},
	}, r.handleGetGameCenterAchievement)

	// Create Game Center achievement
	r.register(mcp.Tool{
		Name:        "create_game_center_achievement",
		Description: "Create a new Game Center achievement",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"game_center_detail_id": {
					Type:        "string",
					Description: "The Game Center detail ID",
				},
				"reference_name": {
					Type:        "string",
					Description: "Internal reference name",
				},
				"vendor_identifier": {
					Type:        "string",
					Description: "Unique identifier for the achievement",
				},
				"points": {
					Type:        "integer",
					Description: "Points awarded for the achievement",
				},
				"show_before_earned": {
					Type:        "boolean",
					Description: "Show achievement before earned",
				},
				"repeatable": {
					Type:        "boolean",
					Description: "Achievement can be earned multiple times",
				},
			},
			Required: []string{"game_center_detail_id", "reference_name", "vendor_identifier", "points"},
		},
	}, r.handleCreateGameCenterAchievement)

	// Update Game Center achievement
	r.register(mcp.Tool{
		Name:        "update_game_center_achievement",
		Description: "Update a Game Center achievement",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"achievement_id": {
					Type:        "string",
					Description: "The achievement ID",
				},
				"reference_name": {
					Type:        "string",
					Description: "Updated reference name",
				},
				"points": {
					Type:        "integer",
					Description: "Updated points",
				},
				"show_before_earned": {
					Type:        "boolean",
					Description: "Show achievement before earned",
				},
				"repeatable": {
					Type:        "boolean",
					Description: "Achievement can be earned multiple times",
				},
				"archived": {
					Type:        "boolean",
					Description: "Archive the achievement",
				},
			},
			Required: []string{"achievement_id"},
		},
	}, r.handleUpdateGameCenterAchievement)

	// Delete Game Center achievement
	r.register(mcp.Tool{
		Name:        "delete_game_center_achievement",
		Description: "Delete a Game Center achievement",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"achievement_id": {
					Type:        "string",
					Description: "The achievement ID",
				},
			},
			Required: []string{"achievement_id"},
		},
	}, r.handleDeleteGameCenterAchievement)

	// List Game Center leaderboards
	r.register(mcp.Tool{
		Name:        "list_game_center_leaderboards",
		Description: "List Game Center leaderboards for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"game_center_detail_id": {
					Type:        "string",
					Description: "The Game Center detail ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of leaderboards to return (default 50)",
				},
			},
			Required: []string{"game_center_detail_id"},
		},
	}, r.handleListGameCenterLeaderboards)

	// Get Game Center leaderboard
	r.register(mcp.Tool{
		Name:        "get_game_center_leaderboard",
		Description: "Get details of a specific Game Center leaderboard",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"leaderboard_id": {
					Type:        "string",
					Description: "The leaderboard ID",
				},
			},
			Required: []string{"leaderboard_id"},
		},
	}, r.handleGetGameCenterLeaderboard)

	// Create Game Center leaderboard
	r.register(mcp.Tool{
		Name:        "create_game_center_leaderboard",
		Description: "Create a new Game Center leaderboard",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"game_center_detail_id": {
					Type:        "string",
					Description: "The Game Center detail ID",
				},
				"reference_name": {
					Type:        "string",
					Description: "Internal reference name",
				},
				"vendor_identifier": {
					Type:        "string",
					Description: "Unique identifier for the leaderboard",
				},
				"submission_type": {
					Type:        "string",
					Description: "Score submission type (BEST_SCORE, MOST_RECENT_SCORE)",
				},
				"score_sort_type": {
					Type:        "string",
					Description: "How scores are sorted (ASC, DESC)",
				},
				"score_range_start": {
					Type:        "string",
					Description: "Minimum valid score",
				},
				"score_range_end": {
					Type:        "string",
					Description: "Maximum valid score",
				},
			},
			Required: []string{"game_center_detail_id", "reference_name", "vendor_identifier", "submission_type", "score_sort_type"},
		},
	}, r.handleCreateGameCenterLeaderboard)

	// Update Game Center leaderboard
	r.register(mcp.Tool{
		Name:        "update_game_center_leaderboard",
		Description: "Update a Game Center leaderboard",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"leaderboard_id": {
					Type:        "string",
					Description: "The leaderboard ID",
				},
				"reference_name": {
					Type:        "string",
					Description: "Updated reference name",
				},
				"submission_type": {
					Type:        "string",
					Description: "Updated submission type",
				},
				"score_sort_type": {
					Type:        "string",
					Description: "Updated sort type",
				},
				"archived": {
					Type:        "boolean",
					Description: "Archive the leaderboard",
				},
			},
			Required: []string{"leaderboard_id"},
		},
	}, r.handleUpdateGameCenterLeaderboard)

	// Delete Game Center leaderboard
	r.register(mcp.Tool{
		Name:        "delete_game_center_leaderboard",
		Description: "Delete a Game Center leaderboard",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"leaderboard_id": {
					Type:        "string",
					Description: "The leaderboard ID",
				},
			},
			Required: []string{"leaderboard_id"},
		},
	}, r.handleDeleteGameCenterLeaderboard)
}

func (r *Registry) handleGetGameCenterDetail(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}

	resp, err := r.client.GetGameCenterDetail(context.Background(), params.AppID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get Game Center detail: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatGameCenterDetail(resp.Data)), nil
}

func (r *Registry) handleListGameCenterAchievements(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		GameCenterDetailID string `json:"game_center_detail_id"`
		Limit              int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.GameCenterDetailID == "" {
		return nil, fmt.Errorf("game_center_detail_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListGameCenterAchievements(context.Background(), params.GameCenterDetailID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list achievements: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatGameCenterAchievements(resp.Data)), nil
}

func (r *Registry) handleGetGameCenterAchievement(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AchievementID string `json:"achievement_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AchievementID == "" {
		return nil, fmt.Errorf("achievement_id is required")
	}

	resp, err := r.client.GetGameCenterAchievement(context.Background(), params.AchievementID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get achievement: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatGameCenterAchievement(resp.Data)), nil
}

func (r *Registry) handleCreateGameCenterAchievement(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		GameCenterDetailID string `json:"game_center_detail_id"`
		ReferenceName      string `json:"reference_name"`
		VendorIdentifier   string `json:"vendor_identifier"`
		Points             int    `json:"points"`
		ShowBeforeEarned   bool   `json:"show_before_earned"`
		Repeatable         bool   `json:"repeatable"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.GameCenterDetailID == "" {
		return nil, fmt.Errorf("game_center_detail_id is required")
	}
	if params.ReferenceName == "" {
		return nil, fmt.Errorf("reference_name is required")
	}
	if params.VendorIdentifier == "" {
		return nil, fmt.Errorf("vendor_identifier is required")
	}

	req := &api.GameCenterAchievementCreateRequest{
		Data: api.GameCenterAchievementCreateData{
			Type: "gameCenterAchievements",
			Attributes: api.GameCenterAchievementCreateAttributes{
				ReferenceName:    params.ReferenceName,
				VendorIdentifier: params.VendorIdentifier,
				Points:           params.Points,
				ShowBeforeEarned: params.ShowBeforeEarned,
				Repeatable:       params.Repeatable,
			},
			Relationships: api.GameCenterAchievementCreateRelationships{
				GameCenterDetail: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "gameCenterDetails",
						ID:   params.GameCenterDetailID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateGameCenterAchievement(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create achievement: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Created achievement: %s (ID: %s)", resp.Data.Attributes.ReferenceName, resp.Data.ID)), nil
}

func (r *Registry) handleUpdateGameCenterAchievement(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AchievementID    string `json:"achievement_id"`
		ReferenceName    string `json:"reference_name"`
		Points           *int   `json:"points"`
		ShowBeforeEarned *bool  `json:"show_before_earned"`
		Repeatable       *bool  `json:"repeatable"`
		Archived         *bool  `json:"archived"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AchievementID == "" {
		return nil, fmt.Errorf("achievement_id is required")
	}

	req := &api.GameCenterAchievementUpdateRequest{
		Data: api.GameCenterAchievementUpdateData{
			Type: "gameCenterAchievements",
			ID:   params.AchievementID,
			Attributes: api.GameCenterAchievementUpdateAttributes{
				ReferenceName:    params.ReferenceName,
				Points:           params.Points,
				ShowBeforeEarned: params.ShowBeforeEarned,
				Repeatable:       params.Repeatable,
				Archived:         params.Archived,
			},
		},
	}

	resp, err := r.client.UpdateGameCenterAchievement(context.Background(), params.AchievementID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update achievement: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Updated achievement: %s", resp.Data.ID)), nil
}

func (r *Registry) handleDeleteGameCenterAchievement(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AchievementID string `json:"achievement_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AchievementID == "" {
		return nil, fmt.Errorf("achievement_id is required")
	}

	err := r.client.DeleteGameCenterAchievement(context.Background(), params.AchievementID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete achievement: %v", err)), nil
	}

	return mcp.NewSuccessResult("Achievement deleted successfully"), nil
}

func (r *Registry) handleListGameCenterLeaderboards(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		GameCenterDetailID string `json:"game_center_detail_id"`
		Limit              int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.GameCenterDetailID == "" {
		return nil, fmt.Errorf("game_center_detail_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListGameCenterLeaderboards(context.Background(), params.GameCenterDetailID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list leaderboards: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatGameCenterLeaderboards(resp.Data)), nil
}

func (r *Registry) handleGetGameCenterLeaderboard(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LeaderboardID string `json:"leaderboard_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LeaderboardID == "" {
		return nil, fmt.Errorf("leaderboard_id is required")
	}

	resp, err := r.client.GetGameCenterLeaderboard(context.Background(), params.LeaderboardID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get leaderboard: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatGameCenterLeaderboard(resp.Data)), nil
}

func (r *Registry) handleCreateGameCenterLeaderboard(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		GameCenterDetailID string `json:"game_center_detail_id"`
		ReferenceName      string `json:"reference_name"`
		VendorIdentifier   string `json:"vendor_identifier"`
		SubmissionType     string `json:"submission_type"`
		ScoreSortType      string `json:"score_sort_type"`
		ScoreRangeStart    string `json:"score_range_start"`
		ScoreRangeEnd      string `json:"score_range_end"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.GameCenterDetailID == "" {
		return nil, fmt.Errorf("game_center_detail_id is required")
	}
	if params.ReferenceName == "" {
		return nil, fmt.Errorf("reference_name is required")
	}
	if params.VendorIdentifier == "" {
		return nil, fmt.Errorf("vendor_identifier is required")
	}
	if params.SubmissionType == "" {
		return nil, fmt.Errorf("submission_type is required")
	}
	if params.ScoreSortType == "" {
		return nil, fmt.Errorf("score_sort_type is required")
	}

	req := &api.GameCenterLeaderboardCreateRequest{
		Data: api.GameCenterLeaderboardCreateData{
			Type: "gameCenterLeaderboards",
			Attributes: api.GameCenterLeaderboardCreateAttributes{
				ReferenceName:    params.ReferenceName,
				VendorIdentifier: params.VendorIdentifier,
				SubmissionType:   params.SubmissionType,
				ScoreSortType:    params.ScoreSortType,
				ScoreRangeStart:  params.ScoreRangeStart,
				ScoreRangeEnd:    params.ScoreRangeEnd,
			},
			Relationships: api.GameCenterLeaderboardCreateRelationships{
				GameCenterDetail: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "gameCenterDetails",
						ID:   params.GameCenterDetailID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateGameCenterLeaderboard(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create leaderboard: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Created leaderboard: %s (ID: %s)", resp.Data.Attributes.ReferenceName, resp.Data.ID)), nil
}

func (r *Registry) handleUpdateGameCenterLeaderboard(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LeaderboardID  string `json:"leaderboard_id"`
		ReferenceName  string `json:"reference_name"`
		SubmissionType string `json:"submission_type"`
		ScoreSortType  string `json:"score_sort_type"`
		Archived       *bool  `json:"archived"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LeaderboardID == "" {
		return nil, fmt.Errorf("leaderboard_id is required")
	}

	req := &api.GameCenterLeaderboardUpdateRequest{
		Data: api.GameCenterLeaderboardUpdateData{
			Type: "gameCenterLeaderboards",
			ID:   params.LeaderboardID,
			Attributes: api.GameCenterLeaderboardUpdateAttributes{
				ReferenceName:  params.ReferenceName,
				SubmissionType: params.SubmissionType,
				ScoreSortType:  params.ScoreSortType,
				Archived:       params.Archived,
			},
		},
	}

	resp, err := r.client.UpdateGameCenterLeaderboard(context.Background(), params.LeaderboardID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update leaderboard: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Updated leaderboard: %s", resp.Data.ID)), nil
}

func (r *Registry) handleDeleteGameCenterLeaderboard(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LeaderboardID string `json:"leaderboard_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LeaderboardID == "" {
		return nil, fmt.Errorf("leaderboard_id is required")
	}

	err := r.client.DeleteGameCenterLeaderboard(context.Background(), params.LeaderboardID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete leaderboard: %v", err)), nil
	}

	return mcp.NewSuccessResult("Leaderboard deleted successfully"), nil
}

func formatGameCenterDetail(detail api.GameCenterDetail) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Game Center Detail ID: %s\n", detail.ID))
	sb.WriteString(fmt.Sprintf("Arcade Enabled: %t\n", detail.Attributes.ArcadeEnabled))
	sb.WriteString(fmt.Sprintf("Challenge Enabled: %t\n", detail.Attributes.ChallengeEnabled))
	return sb.String()
}

func formatGameCenterAchievements(achievements []api.GameCenterAchievement) string {
	if len(achievements) == 0 {
		return "No Game Center achievements found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d achievements:\n\n", len(achievements)))

	for _, achievement := range achievements {
		sb.WriteString(formatGameCenterAchievement(achievement))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatGameCenterAchievement(achievement api.GameCenterAchievement) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", achievement.ID))
	sb.WriteString(fmt.Sprintf("Reference Name: %s\n", achievement.Attributes.ReferenceName))
	sb.WriteString(fmt.Sprintf("Vendor ID: %s\n", achievement.Attributes.VendorIdentifier))
	sb.WriteString(fmt.Sprintf("Points: %d\n", achievement.Attributes.Points))
	sb.WriteString(fmt.Sprintf("Show Before Earned: %t\n", achievement.Attributes.ShowBeforeEarned))
	sb.WriteString(fmt.Sprintf("Repeatable: %t\n", achievement.Attributes.Repeatable))
	sb.WriteString(fmt.Sprintf("Archived: %t\n", achievement.Attributes.Archived))
	return sb.String()
}

func formatGameCenterLeaderboards(leaderboards []api.GameCenterLeaderboard) string {
	if len(leaderboards) == 0 {
		return "No Game Center leaderboards found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d leaderboards:\n\n", len(leaderboards)))

	for _, leaderboard := range leaderboards {
		sb.WriteString(formatGameCenterLeaderboard(leaderboard))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatGameCenterLeaderboard(leaderboard api.GameCenterLeaderboard) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", leaderboard.ID))
	sb.WriteString(fmt.Sprintf("Reference Name: %s\n", leaderboard.Attributes.ReferenceName))
	sb.WriteString(fmt.Sprintf("Vendor ID: %s\n", leaderboard.Attributes.VendorIdentifier))
	sb.WriteString(fmt.Sprintf("Submission Type: %s\n", leaderboard.Attributes.SubmissionType))
	sb.WriteString(fmt.Sprintf("Score Sort Type: %s\n", leaderboard.Attributes.ScoreSortType))
	if leaderboard.Attributes.ScoreRangeStart != "" {
		sb.WriteString(fmt.Sprintf("Score Range: %s - %s\n", leaderboard.Attributes.ScoreRangeStart, leaderboard.Attributes.ScoreRangeEnd))
	}
	sb.WriteString(fmt.Sprintf("Archived: %t\n", leaderboard.Attributes.Archived))
	return sb.String()
}
