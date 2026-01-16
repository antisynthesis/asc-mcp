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
			},
			Required: []string{"app_id"},
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
	}, r.handleDeleteCustomerReviewResponse)
}

func (r *Registry) handleListCustomerReviews(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
		Limit int    `json:"limit"`
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

	resp, err := r.client.ListCustomerReviews(context.Background(), params.AppID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list customer reviews: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatCustomerReviews(resp.Data)), nil
}

func (r *Registry) handleGetCustomerReview(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ReviewID string `json:"review_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ReviewID == "" {
		return nil, fmt.Errorf("review_id is required")
	}

	resp, err := r.client.GetCustomerReview(context.Background(), params.ReviewID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get customer review: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatCustomerReview(resp.Data)), nil
}

func (r *Registry) handleCreateCustomerReviewResponse(args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := r.client.CreateCustomerReviewResponse(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create review response: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Created review response: %s", resp.Data.ID)), nil
}

func (r *Registry) handleDeleteCustomerReviewResponse(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ResponseID string `json:"response_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ResponseID == "" {
		return nil, fmt.Errorf("response_id is required")
	}

	err := r.client.DeleteCustomerReviewResponse(context.Background(), params.ResponseID)
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
