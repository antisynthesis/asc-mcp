package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerCustomerReviewTools registers customer review tools.
func (r *Registry) registerCustomerReviewTools() {
	// List customer reviews
	r.register(mcp.Tool{
		Name:        "list_customer_reviews",
		Description: "List customer reviews for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to list reviews for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of reviews to return (default 50)",
				},
				"cursor": {
					Type:        "string",
					Description: "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page.",
				},
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Customer Reviews",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListCustomerReviews)

	// Get customer review
	r.register(mcp.Tool{
		Name:        "get_customer_review",
		Description: "Get details of a specific customer review",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"review_id": {
					Type:        "string",
					Description: "The customer review ID",
				},
			},
			Required: []string{"review_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Customer Review",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetCustomerReview)

	// Create customer review response
	r.register(mcp.Tool{
		Name:        "create_customer_review_response",
		Description: "Create a response to a customer review",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"review_id": {
					Type:        "string",
					Description: "The customer review ID to respond to",
				},
				"response_body": {
					Type:        "string",
					Description: "The response text",
				},
			},
			Required: []string{"review_id", "response_body"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Customer Review Response",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateCustomerReviewResponse)

	// Delete customer review response
	r.register(mcp.Tool{
		Name:        "delete_customer_review_response",
		Description: "Delete a response to a customer review",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"response_id": {
					Type:        "string",
					Description: "The customer review response ID",
				},
			},
			Required: []string{"response_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Customer Review Response",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteCustomerReviewResponse)
}

func (r *Registry) handleListCustomerReviews(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID  string `json:"app_id"`
		Limit  int    `json:"limit"`
		Cursor string `json:"cursor"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.CustomerReviewsResponse, error) {
		return r.client.ListCustomerReviews(ctx, params.AppID, limit)
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list customer reviews: %v", err)), nil
	}

	return newListResult(formatCustomerReviews(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetCustomerReview(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ReviewID string `json:"review_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ReviewID == "" {
		return nil, fmt.Errorf("review_id is required")
	}

	resp, err := r.client.GetCustomerReview(ctx, params.ReviewID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get customer review: %v", err)), nil
	}

	return newDataResult(formatCustomerReview(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateCustomerReviewResponse(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ReviewID     string `json:"review_id"`
		ResponseBody string `json:"response_body"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ReviewID == "" {
		return nil, fmt.Errorf("review_id is required")
	}
	if params.ResponseBody == "" {
		return nil, fmt.Errorf("response_body is required")
	}

	req := &api.CustomerReviewResponseCreateRequest{
		Data: api.CustomerReviewResponseCreateData{
			Type: "customerReviewResponses",
			Attributes: api.CustomerReviewResponseCreateAttributes{
				ResponseBody: params.ResponseBody,
			},
			Relationships: api.CustomerReviewResponseCreateRelationships{
				Review: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "customerReviews",
						ID:   params.ReviewID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateCustomerReviewResponse(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create review response: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created review response: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleDeleteCustomerReviewResponse(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ResponseID string `json:"response_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ResponseID == "" {
		return nil, fmt.Errorf("response_id is required")
	}

	err := r.client.DeleteCustomerReviewResponse(ctx, params.ResponseID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete review response: %v", err)), nil
	}

	return mcp.NewSuccessResult("Review response deleted successfully"), nil
}

func formatCustomerReviews(reviews []api.CustomerReview) string {
	if len(reviews) == 0 {
		return "No customer reviews found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d customer reviews:\n\n", len(reviews)))

	for _, review := range reviews {
		sb.WriteString(formatCustomerReview(review))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatCustomerReview(review api.CustomerReview) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Review ID: %s\n", review.ID))
	sb.WriteString(fmt.Sprintf("Rating: %d/5\n", review.Attributes.Rating))
	if review.Attributes.Title != "" {
		sb.WriteString(fmt.Sprintf("Title: %s\n", review.Attributes.Title))
	}
	if review.Attributes.Body != "" {
		sb.WriteString(fmt.Sprintf("Body: %s\n", review.Attributes.Body))
	}
	if review.Attributes.ReviewerName != "" {
		sb.WriteString(fmt.Sprintf("Reviewer: %s\n", review.Attributes.ReviewerName))
	}
	if review.Attributes.Territory != "" {
		sb.WriteString(fmt.Sprintf("Territory: %s\n", review.Attributes.Territory))
	}
	if review.Attributes.CreatedDate != nil {
		sb.WriteString(fmt.Sprintf("Created: %s\n", review.Attributes.CreatedDate.Format("2006-01-02")))
	}
	return sb.String()
}
