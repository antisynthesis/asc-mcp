package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerBetaFeedbackTools registers TestFlight beta feedback tools
// (App Store Connect API 4.0+): crash and screenshot submissions from
// testers, plus per-build usage metrics.
func (r *Registry) registerBetaFeedbackTools() {
	// feedbackFilterProperty documents the filter map shared by both
	// feedback list endpoints.
	feedbackFilter := mcp.Property{
		Type:        "object",
		Description: "JSON:API filter map. Supported keys: deviceModel, osVersion, appPlatform (IOS, MAC_OS, TV_OS, VISION_OS), devicePlatform, build, build.preReleaseVersion, tester. Values are arrays, e.g. {\"build\": [\"BUILD_ID\"]}.",
	}
	feedbackSort := mcp.Property{
		Type:        "array",
		Description: "Sort order: createdDate or -createdDate",
		Items:       &mcp.Property{Type: "string"},
	}

	// List beta feedback crashes
	r.register(mcp.Tool{
		Name:        "list_beta_feedback_crashes",
		Description: "List crash feedback submissions from TestFlight testers for an app, including device and environment details. Use get_beta_feedback_crash_log to fetch a submission's crash log text.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to list crash feedback for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of submissions to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
				"filter": feedbackFilter,
				"sort":   feedbackSort,
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Beta Feedback Crashes",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListBetaFeedbackCrashes)

	// Get beta feedback crash
	r.register(mcp.Tool{
		Name:        "get_beta_feedback_crash",
		Description: "Get details of a specific crash feedback submission",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"submission_id": {
					Type:        "string",
					Description: "The Beta Feedback Crash Submission ID",
				},
			},
			Required: []string{"submission_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Beta Feedback Crash",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetBetaFeedbackCrash)

	// Get beta feedback crash log
	r.register(mcp.Tool{
		Name:        "get_beta_feedback_crash_log",
		Description: "Get the crash log text attached to a crash feedback submission",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"submission_id": {
					Type:        "string",
					Description: "The Beta Feedback Crash Submission ID",
				},
			},
			Required: []string{"submission_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Beta Feedback Crash Log",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetBetaFeedbackCrashLog)

	// Delete beta feedback crash
	r.register(mcp.Tool{
		Name:        "delete_beta_feedback_crash",
		Description: "Delete a crash feedback submission",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"submission_id": {
					Type:        "string",
					Description: "The Beta Feedback Crash Submission ID",
				},
			},
			Required: []string{"submission_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Beta Feedback Crash",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteBetaFeedbackCrash)

	// List beta feedback screenshots
	r.register(mcp.Tool{
		Name:        "list_beta_feedback_screenshots",
		Description: "List screenshot feedback submissions from TestFlight testers for an app, including tester comments, device details, and time-limited screenshot image URLs",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to list screenshot feedback for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of submissions to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
				"filter": feedbackFilter,
				"sort":   feedbackSort,
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Beta Feedback Screenshots",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListBetaFeedbackScreenshots)

	// Get beta feedback screenshot
	r.register(mcp.Tool{
		Name:        "get_beta_feedback_screenshot",
		Description: "Get details of a specific screenshot feedback submission, including its screenshot image download URLs",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"submission_id": {
					Type:        "string",
					Description: "The Beta Feedback Screenshot Submission ID",
				},
			},
			Required: []string{"submission_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Beta Feedback Screenshot",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetBetaFeedbackScreenshot)

	// Delete beta feedback screenshot
	r.register(mcp.Tool{
		Name:        "delete_beta_feedback_screenshot",
		Description: "Delete a screenshot feedback submission",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"submission_id": {
					Type:        "string",
					Description: "The Beta Feedback Screenshot Submission ID",
				},
			},
			Required: []string{"submission_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Beta Feedback Screenshot",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteBetaFeedbackScreenshot)

	// Beta build usage metrics
	r.register(mcp.Tool{
		Name:        "get_beta_build_usage_metrics",
		Description: "Get TestFlight usage metrics for a build: install, session, crash, feedback, and invite counts",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"build_id": {
					Type:        "string",
					Description: "The Build ID to fetch usage metrics for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of metric groups to return (max 200)",
				},
			},
			Required: []string{"build_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Beta Build Usage Metrics",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetBetaBuildUsageMetrics)
}

func (r *Registry) handleListBetaFeedbackCrashes(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID  string              `json:"app_id"`
		Limit  int                 `json:"limit"`
		Cursor string              `json:"cursor"`
		Filter map[string][]string `json:"filter"`
		Sort   []string            `json:"sort"`
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.BetaFeedbackCrashSubmissionsResponse, error) {
		return r.client.ListBetaFeedbackCrashSubmissions(ctx, params.AppID, listOpts(limit, params.Filter, params.Sort, nil, nil))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list beta feedback crashes: %v", err)), nil
	}

	content := formatBetaFeedbackList("crash feedback submissions", len(resp.Data), func(sb *strings.Builder) {
		for _, submission := range resp.Data {
			sb.WriteString(formatBetaFeedbackSubmission(submission.ID, submission.Attributes))
			sb.WriteString("\n---\n")
		}
	})
	return newListResult(content, resp.Data, resp.Links), nil
}

func (r *Registry) handleGetBetaFeedbackCrash(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubmissionID string `json:"submission_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubmissionID == "" {
		return mcp.NewErrorResult("submission_id is required"), nil
	}

	resp, err := r.client.GetBetaFeedbackCrashSubmission(ctx, params.SubmissionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get beta feedback crash: %v", err)), nil
	}

	return newDataResult(formatBetaFeedbackSubmission(resp.Data.ID, resp.Data.Attributes), resp.Data), nil
}

func (r *Registry) handleGetBetaFeedbackCrashLog(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubmissionID string `json:"submission_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubmissionID == "" {
		return mcp.NewErrorResult("submission_id is required"), nil
	}

	resp, err := r.client.GetBetaFeedbackCrashLog(ctx, params.SubmissionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get crash log: %v", err)), nil
	}

	if resp.Data.Attributes.LogText == "" {
		return newDataResult("Crash log is empty", resp.Data), nil
	}
	return newDataResult(fmt.Sprintf("Crash log for submission %s:\n\n%s",
		params.SubmissionID, resp.Data.Attributes.LogText), resp.Data), nil
}

func (r *Registry) handleDeleteBetaFeedbackCrash(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubmissionID string `json:"submission_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubmissionID == "" {
		return mcp.NewErrorResult("submission_id is required"), nil
	}

	if err := r.client.DeleteBetaFeedbackCrashSubmission(ctx, params.SubmissionID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete beta feedback crash: %v", err)), nil
	}

	return mcp.NewSuccessResult("Beta feedback crash submission deleted successfully"), nil
}

func (r *Registry) handleListBetaFeedbackScreenshots(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID  string              `json:"app_id"`
		Limit  int                 `json:"limit"`
		Cursor string              `json:"cursor"`
		Filter map[string][]string `json:"filter"`
		Sort   []string            `json:"sort"`
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.BetaFeedbackScreenshotSubmissionsResponse, error) {
		return r.client.ListBetaFeedbackScreenshotSubmissions(ctx, params.AppID, listOpts(limit, params.Filter, params.Sort, nil, nil))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list beta feedback screenshots: %v", err)), nil
	}

	content := formatBetaFeedbackList("screenshot feedback submissions", len(resp.Data), func(sb *strings.Builder) {
		for _, submission := range resp.Data {
			sb.WriteString(formatBetaFeedbackSubmission(submission.ID, submission.Attributes))
			sb.WriteString("\n---\n")
		}
	})
	return newListResult(content, resp.Data, resp.Links), nil
}

func (r *Registry) handleGetBetaFeedbackScreenshot(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubmissionID string `json:"submission_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubmissionID == "" {
		return mcp.NewErrorResult("submission_id is required"), nil
	}

	resp, err := r.client.GetBetaFeedbackScreenshotSubmission(ctx, params.SubmissionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get beta feedback screenshot: %v", err)), nil
	}

	return newDataResult(formatBetaFeedbackSubmission(resp.Data.ID, resp.Data.Attributes), resp.Data), nil
}

func (r *Registry) handleDeleteBetaFeedbackScreenshot(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubmissionID string `json:"submission_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubmissionID == "" {
		return mcp.NewErrorResult("submission_id is required"), nil
	}

	if err := r.client.DeleteBetaFeedbackScreenshotSubmission(ctx, params.SubmissionID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete beta feedback screenshot: %v", err)), nil
	}

	return mcp.NewSuccessResult("Beta feedback screenshot submission deleted successfully"), nil
}

func (r *Registry) handleGetBetaBuildUsageMetrics(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BuildID string `json:"build_id"`
		Limit   int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BuildID == "" {
		return mcp.NewErrorResult("build_id is required"), nil
	}

	limit := params.Limit
	if limit > 200 {
		limit = 200
	}

	resp, err := r.client.GetBetaBuildUsageMetrics(ctx, params.BuildID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get beta build usage metrics: %v", err)), nil
	}

	return newListResult(formatBetaBuildUsages(params.BuildID, resp.Data), resp.Data, resp.Links), nil
}

// formatBetaFeedbackList shares the "Found N ..." framing between the
// two feedback list tools.
func formatBetaFeedbackList(noun string, count int, write func(*strings.Builder)) string {
	if count == 0 {
		return fmt.Sprintf("No %s found", noun)
	}
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d %s:\n\n", count, noun))
	write(&sb)
	return sb.String()
}

// formatBetaFeedbackSubmission renders the shared attributes of a crash
// or screenshot feedback submission, including screenshot URLs when
// present.
func formatBetaFeedbackSubmission(id string, attrs api.BetaFeedbackDeviceAttributes) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Submission ID: %s\n", id))
	if attrs.CreatedDate != nil {
		sb.WriteString(fmt.Sprintf("Created: %s\n", attrs.CreatedDate.Format("2006-01-02 15:04:05")))
	}
	if attrs.Comment != "" {
		sb.WriteString(fmt.Sprintf("Comment: %s\n", attrs.Comment))
	}
	if attrs.Email != "" {
		sb.WriteString(fmt.Sprintf("Tester Email: %s\n", attrs.Email))
	}
	if attrs.DeviceModel != "" {
		sb.WriteString(fmt.Sprintf("Device: %s (%s %s)\n", attrs.DeviceModel, attrs.DevicePlatform, attrs.OSVersion))
	}
	if attrs.AppPlatform != "" {
		sb.WriteString(fmt.Sprintf("App Platform: %s\n", attrs.AppPlatform))
	}
	if attrs.Locale != "" {
		sb.WriteString(fmt.Sprintf("Locale: %s\n", attrs.Locale))
	}
	if attrs.Architecture != "" {
		sb.WriteString(fmt.Sprintf("Architecture: %s\n", attrs.Architecture))
	}
	if attrs.ConnectionType != "" {
		sb.WriteString(fmt.Sprintf("Connection: %s\n", attrs.ConnectionType))
	}
	if attrs.BuildBundleID != "" {
		sb.WriteString(fmt.Sprintf("Build Bundle ID: %s\n", attrs.BuildBundleID))
	}
	for i, shot := range attrs.Screenshots {
		sb.WriteString(fmt.Sprintf("Screenshot %d: %s (%dx%d", i+1, shot.URL, shot.Width, shot.Height))
		if shot.ExpirationDate != nil {
			sb.WriteString(fmt.Sprintf(", URL expires %s", shot.ExpirationDate.Format("2006-01-02 15:04:05")))
		}
		sb.WriteString(")\n")
	}
	return sb.String()
}

func formatBetaBuildUsages(buildID string, groups []api.BetaBuildUsageGroup) string {
	var points int
	for _, group := range groups {
		points += len(group.DataPoints)
	}
	if points == 0 {
		return fmt.Sprintf("No usage metrics found for build %s", buildID)
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("TestFlight usage metrics for build %s:\n\n", buildID))
	for _, group := range groups {
		for _, point := range group.DataPoints {
			if point.Start != nil && point.End != nil {
				sb.WriteString(fmt.Sprintf("%s to %s:\n",
					point.Start.Format("2006-01-02"), point.End.Format("2006-01-02")))
			}
			sb.WriteString(fmt.Sprintf("Installs: %d, Sessions: %d, Crashes: %d, Feedback: %d, Invites: %d\n",
				point.Values.InstallCount, point.Values.SessionCount, point.Values.CrashCount,
				point.Values.FeedbackCount, point.Values.InviteCount))
			sb.WriteString("\n")
		}
	}
	return sb.String()
}
