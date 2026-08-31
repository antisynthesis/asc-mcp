package api

import (
	"context"
	"encoding/json"
	"fmt"
	"net/url"
)

// Review Submission API methods

// ListReviewSubmissions returns review submissions for an app.
func (c *Client) ListReviewSubmissions(ctx context.Context, appID string, opts *ListOptions) (*ReviewSubmissionsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/apps/"+url.PathEscape(appID)+"/reviewSubmissions", query)
	if err != nil {
		return nil, err
	}

	var resp ReviewSubmissionsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetReviewSubmission returns a single review submission.
func (c *Client) GetReviewSubmission(ctx context.Context, submissionID string) (*ReviewSubmissionResponse, error) {
	data, err := c.Get(ctx, "/v1/reviewSubmissions/"+url.PathEscape(submissionID), nil)
	if err != nil {
		return nil, err
	}

	var resp ReviewSubmissionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListReviewSubmissionItems returns the items attached to a review submission.
func (c *Client) ListReviewSubmissionItems(ctx context.Context, submissionID string, opts *ListOptions) (*ReviewSubmissionItemsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/reviewSubmissions/"+url.PathEscape(submissionID)+"/items", query)
	if err != nil {
		return nil, err
	}

	var resp ReviewSubmissionItemsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateReviewSubmission creates a review submission for an app.
func (c *Client) CreateReviewSubmission(ctx context.Context, req *ReviewSubmissionCreateRequest) (*ReviewSubmissionResponse, error) {
	data, err := c.Post(ctx, "/v1/reviewSubmissions", req)
	if err != nil {
		return nil, err
	}

	var resp ReviewSubmissionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateReviewSubmission updates a review submission. Setting submitted=true
// submits it for review; setting canceled=true cancels it.
func (c *Client) UpdateReviewSubmission(ctx context.Context, submissionID string, req *ReviewSubmissionUpdateRequest) (*ReviewSubmissionResponse, error) {
	data, err := c.Patch(ctx, "/v1/reviewSubmissions/"+url.PathEscape(submissionID), req)
	if err != nil {
		return nil, err
	}

	var resp ReviewSubmissionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateReviewSubmissionItem attaches an item to a review submission.
func (c *Client) CreateReviewSubmissionItem(ctx context.Context, req *ReviewSubmissionItemCreateRequest) (*ReviewSubmissionItemResponse, error) {
	data, err := c.Post(ctx, "/v1/reviewSubmissionItems", req)
	if err != nil {
		return nil, err
	}

	var resp ReviewSubmissionItemResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteReviewSubmissionItem removes an item from a review submission.
func (c *Client) DeleteReviewSubmissionItem(ctx context.Context, itemID string) error {
	return c.Delete(ctx, "/v1/reviewSubmissionItems/"+url.PathEscape(itemID))
}
