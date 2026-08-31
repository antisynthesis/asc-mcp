package api

import (
	"context"
	"encoding/json"
	"fmt"
	"net/url"
	"strconv"
	"time"
)

// TestFlight beta feedback API methods (App Store Connect API 4.0+).
// Testers submit crash reports and annotated screenshots from the
// TestFlight app; these endpoints surface those submissions per app.

// Beta feedback types

// BetaFeedbackCrashSubmissionsResponse represents a list of crash
// feedback submissions.
type BetaFeedbackCrashSubmissionsResponse struct {
	Data     []BetaFeedbackCrashSubmission `json:"data"`
	Links    PagedDocumentLinks            `json:"links"`
	Meta     *PagingInformation            `json:"meta,omitempty"`
	Included []any                         `json:"included,omitempty"`
}

// BetaFeedbackCrashSubmissionResponse represents a single crash
// feedback submission.
type BetaFeedbackCrashSubmissionResponse struct {
	Data     BetaFeedbackCrashSubmission `json:"data"`
	Included []any                       `json:"included,omitempty"`
}

// BetaFeedbackCrashSubmission represents one crash report submitted by
// a TestFlight tester.
type BetaFeedbackCrashSubmission struct {
	Type          string                         `json:"type"`
	ID            string                         `json:"id"`
	Attributes    BetaFeedbackDeviceAttributes   `json:"attributes"`
	Relationships *BetaFeedbackSubmissionLinkage `json:"relationships,omitempty"`
}

// BetaFeedbackScreenshotSubmissionsResponse represents a list of
// screenshot feedback submissions.
type BetaFeedbackScreenshotSubmissionsResponse struct {
	Data     []BetaFeedbackScreenshotSubmission `json:"data"`
	Links    PagedDocumentLinks                 `json:"links"`
	Meta     *PagingInformation                 `json:"meta,omitempty"`
	Included []any                              `json:"included,omitempty"`
}

// BetaFeedbackScreenshotSubmissionResponse represents a single
// screenshot feedback submission.
type BetaFeedbackScreenshotSubmissionResponse struct {
	Data     BetaFeedbackScreenshotSubmission `json:"data"`
	Included []any                            `json:"included,omitempty"`
}

// BetaFeedbackScreenshotSubmission represents one screenshot feedback
// item submitted by a TestFlight tester. The Screenshots slice carries
// time-limited download URLs for the annotated images.
type BetaFeedbackScreenshotSubmission struct {
	Type          string                         `json:"type"`
	ID            string                         `json:"id"`
	Attributes    BetaFeedbackDeviceAttributes   `json:"attributes"`
	Relationships *BetaFeedbackSubmissionLinkage `json:"relationships,omitempty"`
}

// BetaFeedbackDeviceAttributes contains the tester comment plus the
// device and environment snapshot Apple captures with each feedback
// submission. Crash and screenshot submissions share this shape; only
// screenshot submissions populate Screenshots.
type BetaFeedbackDeviceAttributes struct {
	CreatedDate             *time.Time                    `json:"createdDate,omitempty"`
	Comment                 string                        `json:"comment,omitempty"`
	Email                   string                        `json:"email,omitempty"`
	DeviceModel             string                        `json:"deviceModel,omitempty"`
	OSVersion               string                        `json:"osVersion,omitempty"`
	Locale                  string                        `json:"locale,omitempty"`
	TimeZone                string                        `json:"timeZone,omitempty"`
	Architecture            string                        `json:"architecture,omitempty"`
	ConnectionType          string                        `json:"connectionType,omitempty"`
	PairedAppleWatch        string                        `json:"pairedAppleWatch,omitempty"`
	AppUptimeInMilliseconds int64                         `json:"appUptimeInMilliseconds,omitempty"`
	DiskBytesAvailable      int64                         `json:"diskBytesAvailable,omitempty"`
	DiskBytesTotal          int64                         `json:"diskBytesTotal,omitempty"`
	BatteryPercentage       int                           `json:"batteryPercentage,omitempty"`
	ScreenWidthInPoints     int                           `json:"screenWidthInPoints,omitempty"`
	ScreenHeightInPoints    int                           `json:"screenHeightInPoints,omitempty"`
	AppPlatform             string                        `json:"appPlatform,omitempty"`
	DevicePlatform          string                        `json:"devicePlatform,omitempty"`
	DeviceFamily            string                        `json:"deviceFamily,omitempty"`
	BuildBundleID           string                        `json:"buildBundleId,omitempty"`
	Screenshots             []BetaFeedbackScreenshotImage `json:"screenshots,omitempty"`
}

// BetaFeedbackScreenshotImage is one downloadable screenshot image with
// its dimensions and URL expiration.
type BetaFeedbackScreenshotImage struct {
	URL            string     `json:"url,omitempty"`
	Width          int        `json:"width,omitempty"`
	Height         int        `json:"height,omitempty"`
	ExpirationDate *time.Time `json:"expirationDate,omitempty"`
}

// BetaFeedbackSubmissionLinkage exposes the build and tester a feedback
// submission belongs to.
type BetaFeedbackSubmissionLinkage struct {
	Build  *RelationshipData `json:"build,omitempty"`
	Tester *RelationshipData `json:"tester,omitempty"`
}

// BetaCrashLogResponse represents the crash log attached to a crash
// feedback submission.
type BetaCrashLogResponse struct {
	Data BetaCrashLog `json:"data"`
}

// BetaCrashLog carries the symbolicated crash log text.
type BetaCrashLog struct {
	Type       string                 `json:"type"`
	ID         string                 `json:"id"`
	Attributes BetaCrashLogAttributes `json:"attributes"`
}

// BetaCrashLogAttributes contains the crash log text.
type BetaCrashLogAttributes struct {
	LogText string `json:"logText,omitempty"`
}

// Beta build usage metric types

// BetaBuildUsagesResponse represents TestFlight usage metrics for one
// build (install, session, crash, feedback, and invite counts).
type BetaBuildUsagesResponse struct {
	Data  []BetaBuildUsageGroup `json:"data"`
	Links PagedDocumentLinks    `json:"links"`
	Meta  *PagingInformation    `json:"meta,omitempty"`
}

// BetaBuildUsageGroup is one group of usage data points.
type BetaBuildUsageGroup struct {
	DataPoints []BetaBuildUsageDataPoint `json:"dataPoints,omitempty"`
}

// BetaBuildUsageDataPoint is one measured interval of build usage.
type BetaBuildUsageDataPoint struct {
	Start  *time.Time           `json:"start,omitempty"`
	End    *time.Time           `json:"end,omitempty"`
	Values BetaBuildUsageValues `json:"values"`
}

// BetaBuildUsageValues carries the usage counters for a data point.
type BetaBuildUsageValues struct {
	CrashCount    int `json:"crashCount"`
	InstallCount  int `json:"installCount"`
	SessionCount  int `json:"sessionCount"`
	FeedbackCount int `json:"feedbackCount"`
	InviteCount   int `json:"inviteCount"`
}

// ListBetaFeedbackCrashSubmissions returns crash feedback submissions
// for an app.
func (c *Client) ListBetaFeedbackCrashSubmissions(ctx context.Context, appID string, opts *ListOptions) (*BetaFeedbackCrashSubmissionsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/apps/"+url.PathEscape(appID)+"/betaFeedbackCrashSubmissions", query)
	if err != nil {
		return nil, err
	}

	var resp BetaFeedbackCrashSubmissionsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetBetaFeedbackCrashSubmission returns a single crash feedback
// submission.
func (c *Client) GetBetaFeedbackCrashSubmission(ctx context.Context, submissionID string) (*BetaFeedbackCrashSubmissionResponse, error) {
	data, err := c.Get(ctx, "/v1/betaFeedbackCrashSubmissions/"+url.PathEscape(submissionID), nil)
	if err != nil {
		return nil, err
	}

	var resp BetaFeedbackCrashSubmissionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteBetaFeedbackCrashSubmission deletes a crash feedback submission.
func (c *Client) DeleteBetaFeedbackCrashSubmission(ctx context.Context, submissionID string) error {
	return c.Delete(ctx, "/v1/betaFeedbackCrashSubmissions/"+url.PathEscape(submissionID))
}

// GetBetaFeedbackCrashLog returns the crash log attached to a crash
// feedback submission.
func (c *Client) GetBetaFeedbackCrashLog(ctx context.Context, submissionID string) (*BetaCrashLogResponse, error) {
	data, err := c.Get(ctx, "/v1/betaFeedbackCrashSubmissions/"+url.PathEscape(submissionID)+"/crashLog", nil)
	if err != nil {
		return nil, err
	}

	var resp BetaCrashLogResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListBetaFeedbackScreenshotSubmissions returns screenshot feedback
// submissions for an app.
func (c *Client) ListBetaFeedbackScreenshotSubmissions(ctx context.Context, appID string, opts *ListOptions) (*BetaFeedbackScreenshotSubmissionsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/apps/"+url.PathEscape(appID)+"/betaFeedbackScreenshotSubmissions", query)
	if err != nil {
		return nil, err
	}

	var resp BetaFeedbackScreenshotSubmissionsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetBetaFeedbackScreenshotSubmission returns a single screenshot
// feedback submission, including its image download URLs.
func (c *Client) GetBetaFeedbackScreenshotSubmission(ctx context.Context, submissionID string) (*BetaFeedbackScreenshotSubmissionResponse, error) {
	data, err := c.Get(ctx, "/v1/betaFeedbackScreenshotSubmissions/"+url.PathEscape(submissionID), nil)
	if err != nil {
		return nil, err
	}

	var resp BetaFeedbackScreenshotSubmissionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteBetaFeedbackScreenshotSubmission deletes a screenshot feedback
// submission.
func (c *Client) DeleteBetaFeedbackScreenshotSubmission(ctx context.Context, submissionID string) error {
	return c.Delete(ctx, "/v1/betaFeedbackScreenshotSubmissions/"+url.PathEscape(submissionID))
}

// GetBetaBuildUsageMetrics returns TestFlight usage metrics for a build.
func (c *Client) GetBetaBuildUsageMetrics(ctx context.Context, buildID string, limit int) (*BetaBuildUsagesResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", strconv.Itoa(limit))
	}

	data, err := c.Get(ctx, "/v1/builds/"+url.PathEscape(buildID)+"/metrics/betaBuildUsages", query)
	if err != nil {
		return nil, err
	}

	var resp BetaBuildUsagesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}
