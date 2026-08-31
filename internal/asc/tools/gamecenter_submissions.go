package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerGameCenterSubmissionTools registers the Game Center player
// submission tools (App Store Connect API 4.3). These endpoints let a
// game's server post achievement progress and leaderboard scores on a
// player's behalf, identified by the scoped player ID the Game Center
// client hands the server. Setting pre_released marks the submission as
// coming from a build that has not shipped to the App Store yet, keeping
// it out of production leaderboards.
func (r *Registry) registerGameCenterSubmissionTools() {
	// Submit player achievement progress
	r.register(mcp.Tool{
		Name:        "submit_game_center_player_achievement",
		Description: "Submit Game Center achievement progress for a player on the game server's behalf. percentage_achieved is 0-100; 100 earns the achievement.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"bundle_id": {
					Type:        "string",
					Description: "The bundle ID of the game the progress belongs to (e.g. com.example.game)",
				},
				"vendor_identifier": {
					Type:        "string",
					Description: "The achievement's vendor identifier",
				},
				"scoped_player_id": {
					Type:        "string",
					Description: "The scoped player ID the Game Center client provided for this player",
				},
				"percentage_achieved": {
					Type:        "integer",
					Description: "Progress toward the achievement, 0-100",
				},
				"challenge_ids": {
					Type:        "array",
					Description: "Challenge IDs this progress counts toward",
					Items:       &mcp.Property{Type: "string"},
				},
				"submitted_date": {
					Type:        "string",
					Description: "ISO 8601 timestamp the progress was earned. Defaults to the time the API receives the request.",
				},
				"pre_released": {
					Type:        "boolean",
					Description: "Mark the progress as earned in a pre-release build so it stays out of production data",
				},
			},
			Required: []string{"bundle_id", "vendor_identifier", "scoped_player_id", "percentage_achieved"},
		},
		Annotations: writeGameCenterTool("Submit Game Center Player Achievement"),
	}, r.handleSubmitGameCenterPlayerAchievement)

	// Submit leaderboard entry
	r.register(mcp.Tool{
		Name:        "submit_game_center_leaderboard_entry",
		Description: "Submit a Game Center leaderboard score for a player on the game server's behalf. score and context are decimal strings so large values keep full precision.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"bundle_id": {
					Type:        "string",
					Description: "The bundle ID of the game the score belongs to (e.g. com.example.game)",
				},
				"vendor_identifier": {
					Type:        "string",
					Description: "The leaderboard's vendor identifier",
				},
				"scoped_player_id": {
					Type:        "string",
					Description: "The scoped player ID the Game Center client provided for this player",
				},
				"score": {
					Type:        "string",
					Description: "The score to record, as a decimal string",
				},
				"context": {
					Type:        "string",
					Description: "Optional game-defined context value stored with the score, as a decimal string",
				},
				"challenge_ids": {
					Type:        "array",
					Description: "Challenge IDs this score counts toward",
					Items:       &mcp.Property{Type: "string"},
				},
				"submitted_date": {
					Type:        "string",
					Description: "ISO 8601 timestamp the score was earned. Defaults to the time the API receives the request.",
				},
				"pre_released": {
					Type:        "boolean",
					Description: "Mark the score as earned in a pre-release build so it stays out of production leaderboards",
				},
			},
			Required: []string{"bundle_id", "vendor_identifier", "scoped_player_id", "score"},
		},
		Annotations: writeGameCenterTool("Submit Game Center Leaderboard Entry"),
	}, r.handleSubmitGameCenterLeaderboardEntry)
}

func (r *Registry) handleSubmitGameCenterPlayerAchievement(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BundleID           string   `json:"bundle_id"`
		VendorIdentifier   string   `json:"vendor_identifier"`
		ScopedPlayerID     string   `json:"scoped_player_id"`
		PercentageAchieved *int     `json:"percentage_achieved"`
		ChallengeIDs       []string `json:"challenge_ids"`
		SubmittedDate      string   `json:"submitted_date"`
		PreReleased        *bool    `json:"pre_released"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BundleID == "" {
		return mcp.NewErrorResult("bundle_id is required"), nil
	}
	if params.VendorIdentifier == "" {
		return mcp.NewErrorResult("vendor_identifier is required"), nil
	}
	if params.ScopedPlayerID == "" {
		return mcp.NewErrorResult("scoped_player_id is required"), nil
	}
	if params.PercentageAchieved == nil {
		return mcp.NewErrorResult("percentage_achieved is required"), nil
	}
	if *params.PercentageAchieved < 0 || *params.PercentageAchieved > 100 {
		return mcp.NewErrorResult("percentage_achieved must be between 0 and 100"), nil
	}

	req := &api.GameCenterPlayerAchievementSubmissionCreateRequest{
		Data: api.GameCenterPlayerAchievementSubmissionCreateData{
			Type: "gameCenterPlayerAchievementSubmissions",
			Attributes: api.GameCenterPlayerAchievementSubmissionCreateAttributes{
				BundleID:           params.BundleID,
				VendorIdentifier:   params.VendorIdentifier,
				ScopedPlayerID:     params.ScopedPlayerID,
				PercentageAchieved: *params.PercentageAchieved,
				ChallengeIDs:       params.ChallengeIDs,
				SubmittedDate:      params.SubmittedDate,
				PreReleased:        params.PreReleased,
			},
		},
	}

	resp, err := r.client.CreateGameCenterPlayerAchievementSubmission(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to submit player achievement: %v", err)), nil
	}

	return newDataResult(formatGameCenterPlayerAchievementSubmission(resp.Data), resp.Data), nil
}

func (r *Registry) handleSubmitGameCenterLeaderboardEntry(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BundleID         string   `json:"bundle_id"`
		VendorIdentifier string   `json:"vendor_identifier"`
		ScopedPlayerID   string   `json:"scoped_player_id"`
		Score            string   `json:"score"`
		Context          string   `json:"context"`
		ChallengeIDs     []string `json:"challenge_ids"`
		SubmittedDate    string   `json:"submitted_date"`
		PreReleased      *bool    `json:"pre_released"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BundleID == "" {
		return mcp.NewErrorResult("bundle_id is required"), nil
	}
	if params.VendorIdentifier == "" {
		return mcp.NewErrorResult("vendor_identifier is required"), nil
	}
	if params.ScopedPlayerID == "" {
		return mcp.NewErrorResult("scoped_player_id is required"), nil
	}
	if params.Score == "" {
		return mcp.NewErrorResult("score is required"), nil
	}

	req := &api.GameCenterLeaderboardEntrySubmissionCreateRequest{
		Data: api.GameCenterLeaderboardEntrySubmissionCreateData{
			Type: "gameCenterLeaderboardEntrySubmissions",
			Attributes: api.GameCenterLeaderboardEntrySubmissionCreateAttributes{
				BundleID:         params.BundleID,
				VendorIdentifier: params.VendorIdentifier,
				ScopedPlayerID:   params.ScopedPlayerID,
				Score:            params.Score,
				Context:          params.Context,
				ChallengeIDs:     params.ChallengeIDs,
				SubmittedDate:    params.SubmittedDate,
				PreReleased:      params.PreReleased,
			},
		},
	}

	resp, err := r.client.CreateGameCenterLeaderboardEntrySubmission(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to submit leaderboard entry: %v", err)), nil
	}

	return newDataResult(formatGameCenterLeaderboardEntrySubmission(resp.Data), resp.Data), nil
}

func formatGameCenterPlayerAchievementSubmission(submission api.GameCenterPlayerAchievementSubmission) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Submitted achievement progress (ID: %s)\n", submission.ID))
	sb.WriteString(fmt.Sprintf("Vendor ID: %s\n", submission.Attributes.VendorIdentifier))
	sb.WriteString(fmt.Sprintf("Player: %s\n", submission.Attributes.ScopedPlayerID))
	sb.WriteString(fmt.Sprintf("Percentage Achieved: %d\n", submission.Attributes.PercentageAchieved))
	sb.WriteString(fmt.Sprintf("Pre-Released: %t\n", submission.Attributes.PreReleased))
	if submission.Attributes.SubmittedDate != "" {
		sb.WriteString(fmt.Sprintf("Submitted: %s\n", submission.Attributes.SubmittedDate))
	}
	if len(submission.Attributes.ChallengeIDs) > 0 {
		sb.WriteString(fmt.Sprintf("Challenges: %s\n", strings.Join(submission.Attributes.ChallengeIDs, ", ")))
	}
	return sb.String()
}

func formatGameCenterLeaderboardEntrySubmission(submission api.GameCenterLeaderboardEntrySubmission) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Submitted leaderboard entry (ID: %s)\n", submission.ID))
	sb.WriteString(fmt.Sprintf("Vendor ID: %s\n", submission.Attributes.VendorIdentifier))
	sb.WriteString(fmt.Sprintf("Player: %s\n", submission.Attributes.ScopedPlayerID))
	sb.WriteString(fmt.Sprintf("Score: %s\n", submission.Attributes.Score))
	if submission.Attributes.Context != "" {
		sb.WriteString(fmt.Sprintf("Context: %s\n", submission.Attributes.Context))
	}
	sb.WriteString(fmt.Sprintf("Pre-Released: %t\n", submission.Attributes.PreReleased))
	if submission.Attributes.SubmittedDate != "" {
		sb.WriteString(fmt.Sprintf("Submitted: %s\n", submission.Attributes.SubmittedDate))
	}
	if len(submission.Attributes.ChallengeIDs) > 0 {
		sb.WriteString(fmt.Sprintf("Challenges: %s\n", strings.Join(submission.Attributes.ChallengeIDs, ", ")))
	}
	return sb.String()
}
