package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerReviewSubmissionTools registers review submission tools.
func (r *Registry) registerReviewSubmissionTools() {
	// List review submissions
	r.register(mcp.Tool{
		Name:        "list_review_submissions",
		Description: "List review submissions for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of results to return (default 50)",
				},
				"cursor": {
					Type:        "string",
					Description: "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page.",
				},
				"filter": {
					Type:        "object",
					Description: "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"WAITING_FOR_REVIEW\"]} becomes filter[state]=WAITING_FOR_REVIEW.",
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
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Review Submissions",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListReviewSubmissions)

	// Get review submission
	r.register(mcp.Tool{
		Name:        "get_review_submission",
		Description: "Get a review submission and its attached items",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"review_submission_id": {
					Type:        "string",
					Description: "The review submission ID",
				},
			},
			Required: []string{"review_submission_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Review Submission",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetReviewSubmission)

	// Create review submission
	r.register(mcp.Tool{
		Name:        "create_review_submission",
		Description: "Create a draft review submission for an app and platform. Attach items with add_review_submission_item, then submit with submit_review_submission.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
				"platform": {
					Type:        "string",
					Description: "Platform: IOS, MAC_OS, TV_OS, VISION_OS (default IOS)",
				},
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Review Submission",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateReviewSubmission)

	// Add review submission item
	r.register(mcp.Tool{
		Name:        "add_review_submission_item",
		Description: "Attach an item to a draft review submission. Provide exactly one of app_store_version_id, app_custom_product_page_version_id, app_store_version_experiment_id, app_event_id, one of the Game Center version IDs, or one of the commerce version IDs. Game Center content and commerce products reach App Review through this tool: attach the achievement, leaderboard, leaderboard set, activity, challenge, in-app purchase, subscription, or subscription group VERSION, not the content resource itself.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"review_submission_id": {
					Type:        "string",
					Description: "The review submission ID",
				},
				"app_store_version_id": {
					Type:        "string",
					Description: "An App Store version ID to attach",
				},
				"app_custom_product_page_version_id": {
					Type:        "string",
					Description: "An app custom product page version ID to attach",
				},
				"app_store_version_experiment_id": {
					Type:        "string",
					Description: "An App Store version experiment ID to attach",
				},
				"app_event_id": {
					Type:        "string",
					Description: "An app event ID to attach",
				},
				"game_center_achievement_version_id": {
					Type:        "string",
					Description: "A Game Center achievement version ID to attach",
				},
				"game_center_leaderboard_version_id": {
					Type:        "string",
					Description: "A Game Center leaderboard version ID to attach",
				},
				"game_center_leaderboard_set_version_id": {
					Type:        "string",
					Description: "A Game Center leaderboard set version ID to attach",
				},
				"game_center_activity_version_id": {
					Type:        "string",
					Description: "A Game Center activity version ID to attach",
				},
				"game_center_challenge_version_id": {
					Type:        "string",
					Description: "A Game Center challenge version ID to attach",
				},
				"in_app_purchase_version_id": {
					Type:        "string",
					Description: "An in-app purchase version ID to attach",
				},
				"subscription_version_id": {
					Type:        "string",
					Description: "A subscription version ID to attach",
				},
				"subscription_group_version_id": {
					Type:        "string",
					Description: "A subscription group version ID to attach",
				},
			},
			Required: []string{"review_submission_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Add Review Submission Item",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleAddReviewSubmissionItem)

	// Submit review submission
	r.register(mcp.Tool{
		Name:        "submit_review_submission",
		Description: "Submit a draft review submission for App Review",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"review_submission_id": {
					Type:        "string",
					Description: "The review submission ID to submit",
				},
			},
			Required: []string{"review_submission_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Submit Review Submission",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleSubmitReviewSubmission)

	// Cancel review submission
	r.register(mcp.Tool{
		Name:        "cancel_review_submission",
		Description: "Cancel a review submission",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"review_submission_id": {
					Type:        "string",
					Description: "The review submission ID to cancel",
				},
			},
			Required: []string{"review_submission_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Cancel Review Submission",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCancelReviewSubmission)
}

func (r *Registry) handleListReviewSubmissions(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID   string              `json:"app_id"`
		Limit   int                 `json:"limit"`
		Cursor  string              `json:"cursor"`
		Filter  map[string][]string `json:"filter"`
		Sort    []string            `json:"sort"`
		Fields  map[string][]string `json:"fields"`
		Include []string            `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.ReviewSubmissionsResponse, error) {
		return r.client.ListReviewSubmissions(ctx, params.AppID, listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list review submissions: %v", err)), nil
	}

	return newListResult(formatReviewSubmissions(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetReviewSubmission(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ReviewSubmissionID string `json:"review_submission_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ReviewSubmissionID == "" {
		return mcp.NewErrorResult("review_submission_id is required"), nil
	}

	resp, err := r.client.GetReviewSubmission(ctx, params.ReviewSubmissionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get review submission: %v", err)), nil
	}

	var sb strings.Builder
	sb.WriteString(formatReviewSubmission(resp.Data))

	// Best-effort: also show the attached items.
	if items, err := r.client.ListReviewSubmissionItems(ctx, params.ReviewSubmissionID, nil); err == nil && len(items.Data) > 0 {
		sb.WriteString(fmt.Sprintf("\nItems (%d):\n", len(items.Data)))
		for _, item := range items.Data {
			sb.WriteString(fmt.Sprintf("- %s (state: %s)\n", item.ID, item.Attributes.State))
		}
	}

	return newDataResult(sb.String(), resp.Data), nil
}

func (r *Registry) handleCreateReviewSubmission(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID    string `json:"app_id"`
		Platform string `json:"platform"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}

	platform := params.Platform
	if platform == "" {
		platform = "IOS"
	}

	req := &api.ReviewSubmissionCreateRequest{
		Data: api.ReviewSubmissionCreateData{
			Type: "reviewSubmissions",
			Attributes: &api.ReviewSubmissionCreateAttributes{
				Platform: platform,
			},
			Relationships: api.ReviewSubmissionCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "apps", ID: params.AppID},
				},
			},
		},
	}

	resp, err := r.client.CreateReviewSubmission(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create review submission: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Review submission created:\n%s", formatReviewSubmission(resp.Data)), resp.Data), nil
}

func (r *Registry) handleAddReviewSubmissionItem(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ReviewSubmissionID                string `json:"review_submission_id"`
		AppStoreVersionID                 string `json:"app_store_version_id"`
		AppCustomProductPageVersionID     string `json:"app_custom_product_page_version_id"`
		AppStoreVersionExperimentID       string `json:"app_store_version_experiment_id"`
		AppEventID                        string `json:"app_event_id"`
		GameCenterAchievementVersionID    string `json:"game_center_achievement_version_id"`
		GameCenterLeaderboardVersionID    string `json:"game_center_leaderboard_version_id"`
		GameCenterLeaderboardSetVersionID string `json:"game_center_leaderboard_set_version_id"`
		GameCenterActivityVersionID       string `json:"game_center_activity_version_id"`
		GameCenterChallengeVersionID      string `json:"game_center_challenge_version_id"`
		InAppPurchaseVersionID            string `json:"in_app_purchase_version_id"`
		SubscriptionVersionID             string `json:"subscription_version_id"`
		SubscriptionGroupVersionID        string `json:"subscription_group_version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ReviewSubmissionID == "" {
		return mcp.NewErrorResult("review_submission_id is required"), nil
	}

	rels := api.ReviewSubmissionItemCreateRelationships{
		ReviewSubmission: api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "reviewSubmissions", ID: params.ReviewSubmissionID},
		},
	}

	itemCount := 0
	if params.AppStoreVersionID != "" {
		rels.AppStoreVersion = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "appStoreVersions", ID: params.AppStoreVersionID},
		}
		itemCount++
	}
	if params.AppCustomProductPageVersionID != "" {
		rels.AppCustomProductPageVersion = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "appCustomProductPageVersions", ID: params.AppCustomProductPageVersionID},
		}
		itemCount++
	}
	if params.AppStoreVersionExperimentID != "" {
		rels.AppStoreVersionExperimentV2 = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "appStoreVersionExperiments", ID: params.AppStoreVersionExperimentID},
		}
		itemCount++
	}
	if params.AppEventID != "" {
		rels.AppEvent = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "appEvents", ID: params.AppEventID},
		}
		itemCount++
	}
	// Game Center content is submitted for review by attaching the
	// content's version resource (App Store Connect API 4.2+).
	if params.GameCenterAchievementVersionID != "" {
		rels.GameCenterAchievementVersion = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "gameCenterAchievementVersions", ID: params.GameCenterAchievementVersionID},
		}
		itemCount++
	}
	if params.GameCenterLeaderboardVersionID != "" {
		rels.GameCenterLeaderboardVersion = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "gameCenterLeaderboardVersions", ID: params.GameCenterLeaderboardVersionID},
		}
		itemCount++
	}
	if params.GameCenterLeaderboardSetVersionID != "" {
		rels.GameCenterLeaderboardSetVersion = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "gameCenterLeaderboardSetVersions", ID: params.GameCenterLeaderboardSetVersionID},
		}
		itemCount++
	}
	if params.GameCenterActivityVersionID != "" {
		rels.GameCenterActivityVersion = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "gameCenterActivityVersions", ID: params.GameCenterActivityVersionID},
		}
		itemCount++
	}
	if params.GameCenterChallengeVersionID != "" {
		rels.GameCenterChallengeVersion = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "gameCenterChallengeVersions", ID: params.GameCenterChallengeVersionID},
		}
		itemCount++
	}

	// In-app purchases, subscriptions and subscription groups are
	// submitted for review by attaching the product's version resource
	// (App Store Connect API 4.4.1).
	if params.InAppPurchaseVersionID != "" {
		rels.InAppPurchaseVersion = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "inAppPurchaseVersions", ID: params.InAppPurchaseVersionID},
		}
		itemCount++
	}
	if params.SubscriptionVersionID != "" {
		rels.SubscriptionVersion = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "subscriptionVersions", ID: params.SubscriptionVersionID},
		}
		itemCount++
	}
	if params.SubscriptionGroupVersionID != "" {
		rels.SubscriptionGroupVersion = &api.RelationshipData{
			Data: api.ResourceIdentifier{Type: "subscriptionGroupVersions", ID: params.SubscriptionGroupVersionID},
		}
		itemCount++
	}

	if itemCount != 1 {
		return mcp.NewErrorResult("exactly one of app_store_version_id, app_custom_product_page_version_id, app_store_version_experiment_id, app_event_id, game_center_achievement_version_id, game_center_leaderboard_version_id, game_center_leaderboard_set_version_id, game_center_activity_version_id, game_center_challenge_version_id, in_app_purchase_version_id, subscription_version_id, or subscription_group_version_id is required"), nil
	}

	req := &api.ReviewSubmissionItemCreateRequest{
		Data: api.ReviewSubmissionItemCreateData{
			Type:          "reviewSubmissionItems",
			Relationships: rels,
		},
	}

	resp, err := r.client.CreateReviewSubmissionItem(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to add review submission item: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Review submission item added: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleSubmitReviewSubmission(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	return r.updateReviewSubmissionState(ctx, args, "submit")
}

func (r *Registry) handleCancelReviewSubmission(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	return r.updateReviewSubmissionState(ctx, args, "cancel")
}

// updateReviewSubmissionState patches a review submission with
// submitted=true or canceled=true depending on the action.
func (r *Registry) updateReviewSubmissionState(ctx context.Context, args json.RawMessage, action string) (*mcp.ToolsCallResult, error) {
	var params struct {
		ReviewSubmissionID string `json:"review_submission_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ReviewSubmissionID == "" {
		return mcp.NewErrorResult("review_submission_id is required"), nil
	}

	flag := true
	attrs := api.ReviewSubmissionUpdateAttributes{}
	if action == "submit" {
		attrs.Submitted = &flag
	} else {
		attrs.Canceled = &flag
	}

	req := &api.ReviewSubmissionUpdateRequest{
		Data: api.ReviewSubmissionUpdateData{
			Type:       "reviewSubmissions",
			ID:         params.ReviewSubmissionID,
			Attributes: attrs,
		},
	}

	resp, err := r.client.UpdateReviewSubmission(ctx, params.ReviewSubmissionID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to %s review submission: %v", action, err)), nil
	}

	if action == "submit" {
		return newDataResult(fmt.Sprintf("Review submission submitted:\n%s", formatReviewSubmission(resp.Data)), resp.Data), nil
	}
	return newDataResult(fmt.Sprintf("Review submission canceled:\n%s", formatReviewSubmission(resp.Data)), resp.Data), nil
}

func formatReviewSubmissions(submissions []api.ReviewSubmission) string {
	if len(submissions) == 0 {
		return "No review submissions found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d review submissions:\n\n", len(submissions)))

	for _, submission := range submissions {
		sb.WriteString(formatReviewSubmission(submission))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatReviewSubmission(submission api.ReviewSubmission) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", submission.ID))
	if submission.Attributes.Platform != "" {
		sb.WriteString(fmt.Sprintf("Platform: %s\n", submission.Attributes.Platform))
	}
	if submission.Attributes.State != "" {
		sb.WriteString(fmt.Sprintf("State: %s\n", submission.Attributes.State))
	}
	if submission.Attributes.SubmittedDate != nil {
		sb.WriteString(fmt.Sprintf("Submitted: %s\n", submission.Attributes.SubmittedDate.Format("2006-01-02 15:04:05")))
	}
	return sb.String()
}
