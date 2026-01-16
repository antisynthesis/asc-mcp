package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerBetaReviewTools registers beta review and beta agreements tools.
func (r *Registry) registerBetaReviewTools() {
	// List beta app review submissions
	r.register(mcp.Tool{
		Name:        "list_beta_app_review_submissions",
		Description: "List beta app review submissions",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"limit": {
					Type:        "integer",
					Description: "Maximum number of submissions to return (default 50)",
				},
			},
		},
	}, r.handleListBetaAppReviewSubmissions)

	// Get beta app review submission
	r.register(mcp.Tool{
		Name:        "get_beta_app_review_submission",
		Description: "Get details of a beta app review submission",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"submission_id": {
					Type:        "string",
					Description: "The beta app review submission ID",
				},
			},
			Required: []string{"submission_id"},
		},
	}, r.handleGetBetaAppReviewSubmission)

	// Create beta app review submission
	r.register(mcp.Tool{
		Name:        "create_beta_app_review_submission",
		Description: "Submit a build for beta app review",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"build_id": {
					Type:        "string",
					Description: "The build ID to submit for review",
				},
			},
			Required: []string{"build_id"},
		},
	}, r.handleCreateBetaAppReviewSubmission)

	// Get beta license agreement
	r.register(mcp.Tool{
		Name:        "get_beta_license_agreement",
		Description: "Get the beta license agreement for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleGetBetaLicenseAgreement)

	// Update beta license agreement
	r.register(mcp.Tool{
		Name:        "update_beta_license_agreement",
		Description: "Update the beta license agreement for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"agreement_id": {
					Type:        "string",
					Description: "The beta license agreement ID",
				},
				"agreement_text": {
					Type:        "string",
					Description: "The new agreement text",
				},
			},
			Required: []string{"agreement_id", "agreement_text"},
		},
	}, r.handleUpdateBetaLicenseAgreement)

	// List beta app localizations
	r.register(mcp.Tool{
		Name:        "list_beta_app_localizations",
		Description: "List beta app localizations for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of localizations to return (default 50)",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleListBetaAppLocalizations)

	// Get beta app localization
	r.register(mcp.Tool{
		Name:        "get_beta_app_localization",
		Description: "Get a specific beta app localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The beta app localization ID",
				},
			},
			Required: []string{"localization_id"},
		},
	}, r.handleGetBetaAppLocalization)

	// Create beta app localization
	r.register(mcp.Tool{
		Name:        "create_beta_app_localization",
		Description: "Create a beta app localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
				"locale": {
					Type:        "string",
					Description: "Locale code (e.g., en-US, de-DE)",
				},
				"description": {
					Type:        "string",
					Description: "Description for beta testers",
				},
				"feedback_email": {
					Type:        "string",
					Description: "Email address for beta feedback",
				},
				"marketing_url": {
					Type:        "string",
					Description: "Marketing URL",
				},
				"privacy_policy_url": {
					Type:        "string",
					Description: "Privacy policy URL",
				},
			},
			Required: []string{"app_id", "locale"},
		},
	}, r.handleCreateBetaAppLocalization)

	// Update beta app localization
	r.register(mcp.Tool{
		Name:        "update_beta_app_localization",
		Description: "Update a beta app localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The beta app localization ID",
				},
				"description": {
					Type:        "string",
					Description: "Description for beta testers",
				},
				"feedback_email": {
					Type:        "string",
					Description: "Email address for beta feedback",
				},
				"marketing_url": {
					Type:        "string",
					Description: "Marketing URL",
				},
				"privacy_policy_url": {
					Type:        "string",
					Description: "Privacy policy URL",
				},
			},
			Required: []string{"localization_id"},
		},
	}, r.handleUpdateBetaAppLocalization)

	// Delete beta app localization
	r.register(mcp.Tool{
		Name:        "delete_beta_app_localization",
		Description: "Delete a beta app localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The beta app localization ID to delete",
				},
			},
			Required: []string{"localization_id"},
		},
	}, r.handleDeleteBetaAppLocalization)

	// List beta build localizations
	r.register(mcp.Tool{
		Name:        "list_beta_build_localizations",
		Description: "List beta build localizations for a build",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"build_id": {
					Type:        "string",
					Description: "The build ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of localizations to return (default 50)",
				},
			},
			Required: []string{"build_id"},
		},
	}, r.handleListBetaBuildLocalizations)

	// Get beta build localization
	r.register(mcp.Tool{
		Name:        "get_beta_build_localization",
		Description: "Get a specific beta build localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The beta build localization ID",
				},
			},
			Required: []string{"localization_id"},
		},
	}, r.handleGetBetaBuildLocalization)

	// Create beta build localization
	r.register(mcp.Tool{
		Name:        "create_beta_build_localization",
		Description: "Create a beta build localization (what's new)",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"build_id": {
					Type:        "string",
					Description: "The build ID",
				},
				"locale": {
					Type:        "string",
					Description: "Locale code (e.g., en-US, de-DE)",
				},
				"whats_new": {
					Type:        "string",
					Description: "What's new in this build for beta testers",
				},
			},
			Required: []string{"build_id", "locale"},
		},
	}, r.handleCreateBetaBuildLocalization)

	// Update beta build localization
	r.register(mcp.Tool{
		Name:        "update_beta_build_localization",
		Description: "Update a beta build localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The beta build localization ID",
				},
				"whats_new": {
					Type:        "string",
					Description: "What's new in this build for beta testers",
				},
			},
			Required: []string{"localization_id"},
		},
	}, r.handleUpdateBetaBuildLocalization)

	// Delete beta build localization
	r.register(mcp.Tool{
		Name:        "delete_beta_build_localization",
		Description: "Delete a beta build localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The beta build localization ID to delete",
				},
			},
			Required: []string{"localization_id"},
		},
	}, r.handleDeleteBetaBuildLocalization)

	// Get build beta detail
	r.register(mcp.Tool{
		Name:        "get_build_beta_detail",
		Description: "Get build beta testing details",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"build_id": {
					Type:        "string",
					Description: "The build ID",
				},
			},
			Required: []string{"build_id"},
		},
	}, r.handleGetBuildBetaDetail)

	// Update build beta detail
	r.register(mcp.Tool{
		Name:        "update_build_beta_detail",
		Description: "Update build beta testing details",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"detail_id": {
					Type:        "string",
					Description: "The build beta detail ID",
				},
				"auto_notify_enabled": {
					Type:        "boolean",
					Description: "Whether to auto-notify testers",
				},
			},
			Required: []string{"detail_id"},
		},
	}, r.handleUpdateBuildBetaDetail)
}

func (r *Registry) handleListBetaAppReviewSubmissions(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Limit int `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListBetaAppReviewSubmissions(context.Background(), limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list beta app review submissions: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatBetaAppReviewSubmissions(resp.Data)), nil
}

func (r *Registry) handleGetBetaAppReviewSubmission(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubmissionID string `json:"submission_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubmissionID == "" {
		return nil, fmt.Errorf("submission_id is required")
	}

	resp, err := r.client.GetBetaAppReviewSubmission(context.Background(), params.SubmissionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get beta app review submission: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatBetaAppReviewSubmission(resp.Data)), nil
}

func (r *Registry) handleCreateBetaAppReviewSubmission(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BuildID string `json:"build_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BuildID == "" {
		return nil, fmt.Errorf("build_id is required")
	}

	req := &api.BetaAppReviewSubmissionCreateRequest{
		Data: api.BetaAppReviewSubmissionCreateData{
			Type: "betaAppReviewSubmissions",
			Relationships: api.BetaAppReviewSubmissionCreateRelationships{
				Build: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "builds", ID: params.BuildID},
				},
			},
		},
	}

	resp, err := r.client.CreateBetaAppReviewSubmission(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create beta app review submission: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Beta app review submission created:\n%s", formatBetaAppReviewSubmission(resp.Data))), nil
}

func (r *Registry) handleGetBetaLicenseAgreement(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}

	resp, err := r.client.GetBetaLicenseAgreement(context.Background(), params.AppID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get beta license agreement: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatBetaLicenseAgreement(resp.Data)), nil
}

func (r *Registry) handleUpdateBetaLicenseAgreement(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AgreementID   string `json:"agreement_id"`
		AgreementText string `json:"agreement_text"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AgreementID == "" || params.AgreementText == "" {
		return nil, fmt.Errorf("agreement_id and agreement_text are required")
	}

	req := &api.BetaLicenseAgreementUpdateRequest{
		Data: api.BetaLicenseAgreementUpdateData{
			Type: "betaLicenseAgreements",
			ID:   params.AgreementID,
			Attributes: api.BetaLicenseAgreementUpdateAttributes{
				AgreementText: params.AgreementText,
			},
		},
	}

	resp, err := r.client.UpdateBetaLicenseAgreement(context.Background(), params.AgreementID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update beta license agreement: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Beta license agreement updated:\n%s", formatBetaLicenseAgreement(resp.Data))), nil
}

func (r *Registry) handleListBetaAppLocalizations(args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := r.client.ListBetaAppLocalizations(context.Background(), params.AppID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list beta app localizations: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatBetaAppLocalizations(resp.Data)), nil
}

func (r *Registry) handleGetBetaAppLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return nil, fmt.Errorf("localization_id is required")
	}

	resp, err := r.client.GetBetaAppLocalization(context.Background(), params.LocalizationID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get beta app localization: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatBetaAppLocalization(resp.Data)), nil
}

func (r *Registry) handleCreateBetaAppLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID            string `json:"app_id"`
		Locale           string `json:"locale"`
		Description      string `json:"description"`
		FeedbackEmail    string `json:"feedback_email"`
		MarketingURL     string `json:"marketing_url"`
		PrivacyPolicyURL string `json:"privacy_policy_url"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" || params.Locale == "" {
		return nil, fmt.Errorf("app_id and locale are required")
	}

	req := &api.BetaAppLocalizationCreateRequest{
		Data: api.BetaAppLocalizationCreateData{
			Type: "betaAppLocalizations",
			Attributes: api.BetaAppLocalizationCreateAttributes{
				Locale:           params.Locale,
				Description:      params.Description,
				FeedbackEmail:    params.FeedbackEmail,
				MarketingURL:     params.MarketingURL,
				PrivacyPolicyURL: params.PrivacyPolicyURL,
			},
			Relationships: api.BetaAppLocalizationCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "apps", ID: params.AppID},
				},
			},
		},
	}

	resp, err := r.client.CreateBetaAppLocalization(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create beta app localization: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Beta app localization created:\n%s", formatBetaAppLocalization(resp.Data))), nil
}

func (r *Registry) handleUpdateBetaAppLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID   string `json:"localization_id"`
		Description      string `json:"description"`
		FeedbackEmail    string `json:"feedback_email"`
		MarketingURL     string `json:"marketing_url"`
		PrivacyPolicyURL string `json:"privacy_policy_url"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return nil, fmt.Errorf("localization_id is required")
	}

	req := &api.BetaAppLocalizationUpdateRequest{
		Data: api.BetaAppLocalizationUpdateData{
			Type: "betaAppLocalizations",
			ID:   params.LocalizationID,
			Attributes: api.BetaAppLocalizationUpdateAttributes{
				Description:      params.Description,
				FeedbackEmail:    params.FeedbackEmail,
				MarketingURL:     params.MarketingURL,
				PrivacyPolicyURL: params.PrivacyPolicyURL,
			},
		},
	}

	resp, err := r.client.UpdateBetaAppLocalization(context.Background(), params.LocalizationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update beta app localization: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Beta app localization updated:\n%s", formatBetaAppLocalization(resp.Data))), nil
}

func (r *Registry) handleDeleteBetaAppLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return nil, fmt.Errorf("localization_id is required")
	}

	err := r.client.DeleteBetaAppLocalization(context.Background(), params.LocalizationID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete beta app localization: %v", err)), nil
	}

	return mcp.NewSuccessResult("Beta app localization deleted"), nil
}

func (r *Registry) handleListBetaBuildLocalizations(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BuildID string `json:"build_id"`
		Limit   int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BuildID == "" {
		return nil, fmt.Errorf("build_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListBetaBuildLocalizations(context.Background(), params.BuildID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list beta build localizations: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatBetaBuildLocalizations(resp.Data)), nil
}

func (r *Registry) handleGetBetaBuildLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return nil, fmt.Errorf("localization_id is required")
	}

	resp, err := r.client.GetBetaBuildLocalization(context.Background(), params.LocalizationID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get beta build localization: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatBetaBuildLocalization(resp.Data)), nil
}

func (r *Registry) handleCreateBetaBuildLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BuildID  string `json:"build_id"`
		Locale   string `json:"locale"`
		WhatsNew string `json:"whats_new"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BuildID == "" || params.Locale == "" {
		return nil, fmt.Errorf("build_id and locale are required")
	}

	req := &api.BetaBuildLocalizationCreateRequest{
		Data: api.BetaBuildLocalizationCreateData{
			Type: "betaBuildLocalizations",
			Attributes: api.BetaBuildLocalizationCreateAttributes{
				Locale:   params.Locale,
				WhatsNew: params.WhatsNew,
			},
			Relationships: api.BetaBuildLocalizationCreateRelationships{
				Build: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "builds", ID: params.BuildID},
				},
			},
		},
	}

	resp, err := r.client.CreateBetaBuildLocalization(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create beta build localization: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Beta build localization created:\n%s", formatBetaBuildLocalization(resp.Data))), nil
}

func (r *Registry) handleUpdateBetaBuildLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
		WhatsNew       string `json:"whats_new"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return nil, fmt.Errorf("localization_id is required")
	}

	req := &api.BetaBuildLocalizationUpdateRequest{
		Data: api.BetaBuildLocalizationUpdateData{
			Type: "betaBuildLocalizations",
			ID:   params.LocalizationID,
			Attributes: api.BetaBuildLocalizationUpdateAttributes{
				WhatsNew: params.WhatsNew,
			},
		},
	}

	resp, err := r.client.UpdateBetaBuildLocalization(context.Background(), params.LocalizationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update beta build localization: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Beta build localization updated:\n%s", formatBetaBuildLocalization(resp.Data))), nil
}

func (r *Registry) handleDeleteBetaBuildLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return nil, fmt.Errorf("localization_id is required")
	}

	err := r.client.DeleteBetaBuildLocalization(context.Background(), params.LocalizationID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete beta build localization: %v", err)), nil
	}

	return mcp.NewSuccessResult("Beta build localization deleted"), nil
}

func (r *Registry) handleGetBuildBetaDetail(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BuildID string `json:"build_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BuildID == "" {
		return nil, fmt.Errorf("build_id is required")
	}

	resp, err := r.client.GetBuildBetaDetail(context.Background(), params.BuildID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get build beta detail: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatBuildBetaDetail(resp.Data)), nil
}

func (r *Registry) handleUpdateBuildBetaDetail(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DetailID          string `json:"detail_id"`
		AutoNotifyEnabled *bool  `json:"auto_notify_enabled"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DetailID == "" {
		return nil, fmt.Errorf("detail_id is required")
	}

	req := &api.BuildBetaDetailUpdateRequest{
		Data: api.BuildBetaDetailUpdateData{
			Type: "buildBetaDetails",
			ID:   params.DetailID,
			Attributes: api.BuildBetaDetailUpdateAttributes{
				AutoNotifyEnabled: params.AutoNotifyEnabled,
			},
		},
	}

	resp, err := r.client.UpdateBuildBetaDetail(context.Background(), params.DetailID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update build beta detail: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Build beta detail updated:\n%s", formatBuildBetaDetail(resp.Data))), nil
}

func formatBetaAppReviewSubmissions(submissions []api.BetaAppReviewSubmission) string {
	if len(submissions) == 0 {
		return "No beta app review submissions found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d beta app review submissions:\n\n", len(submissions)))

	for _, sub := range submissions {
		sb.WriteString(formatBetaAppReviewSubmission(sub))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatBetaAppReviewSubmission(sub api.BetaAppReviewSubmission) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", sub.ID))
	sb.WriteString(fmt.Sprintf("State: %s\n", sub.Attributes.BetaReviewState))
	if sub.Attributes.SubmittedDate != nil {
		sb.WriteString(fmt.Sprintf("Submitted: %s\n", sub.Attributes.SubmittedDate.Format("2006-01-02 15:04")))
	}
	return sb.String()
}

func formatBetaLicenseAgreement(agreement api.BetaLicenseAgreement) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", agreement.ID))
	if agreement.Attributes.AgreementText != "" {
		text := agreement.Attributes.AgreementText
		if len(text) > 500 {
			text = text[:500] + "..."
		}
		sb.WriteString(fmt.Sprintf("Agreement Text:\n%s\n", text))
	} else {
		sb.WriteString("Agreement Text: (empty)\n")
	}
	return sb.String()
}

func formatBetaAppLocalizations(localizations []api.BetaAppLocalization) string {
	if len(localizations) == 0 {
		return "No beta app localizations found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d beta app localizations:\n\n", len(localizations)))

	for _, loc := range localizations {
		sb.WriteString(formatBetaAppLocalization(loc))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatBetaAppLocalization(loc api.BetaAppLocalization) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", loc.ID))
	sb.WriteString(fmt.Sprintf("Locale: %s\n", loc.Attributes.Locale))
	if loc.Attributes.Description != "" {
		sb.WriteString(fmt.Sprintf("Description: %s\n", loc.Attributes.Description))
	}
	if loc.Attributes.FeedbackEmail != "" {
		sb.WriteString(fmt.Sprintf("Feedback Email: %s\n", loc.Attributes.FeedbackEmail))
	}
	if loc.Attributes.MarketingURL != "" {
		sb.WriteString(fmt.Sprintf("Marketing URL: %s\n", loc.Attributes.MarketingURL))
	}
	if loc.Attributes.PrivacyPolicyURL != "" {
		sb.WriteString(fmt.Sprintf("Privacy Policy URL: %s\n", loc.Attributes.PrivacyPolicyURL))
	}
	return sb.String()
}

func formatBetaBuildLocalizations(localizations []api.BetaBuildLocalization) string {
	if len(localizations) == 0 {
		return "No beta build localizations found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d beta build localizations:\n\n", len(localizations)))

	for _, loc := range localizations {
		sb.WriteString(formatBetaBuildLocalization(loc))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatBetaBuildLocalization(loc api.BetaBuildLocalization) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", loc.ID))
	sb.WriteString(fmt.Sprintf("Locale: %s\n", loc.Attributes.Locale))
	if loc.Attributes.WhatsNew != "" {
		sb.WriteString(fmt.Sprintf("What's New: %s\n", loc.Attributes.WhatsNew))
	}
	return sb.String()
}

func formatBuildBetaDetail(detail api.BuildBetaDetail) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", detail.ID))
	sb.WriteString(fmt.Sprintf("Auto Notify Enabled: %t\n", detail.Attributes.AutoNotifyEnabled))
	sb.WriteString(fmt.Sprintf("Internal Build State: %s\n", detail.Attributes.InternalBuildState))
	sb.WriteString(fmt.Sprintf("External Build State: %s\n", detail.Attributes.ExternalBuildState))
	return sb.String()
}
