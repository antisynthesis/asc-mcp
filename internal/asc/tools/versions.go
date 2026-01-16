package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerVersionSubmissionTools registers app store version submission tools.
func (r *Registry) registerVersionSubmissionTools() {
	// List app store versions
	r.register(mcp.Tool{
		Name:        "list_app_store_versions",
		Description: "List App Store versions for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of versions to return (default 50)",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleListAppStoreVersions)

	// Get app store version
	r.register(mcp.Tool{
		Name:        "get_app_store_version",
		Description: "Get details of a specific App Store version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The App Store version ID",
				},
			},
			Required: []string{"version_id"},
		},
	}, r.handleGetAppStoreVersion)

	// Create app store version
	r.register(mcp.Tool{
		Name:        "create_app_store_version",
		Description: "Create a new App Store version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID",
				},
				"version_string": {
					Type:        "string",
					Description: "The version string (e.g., 1.0.0)",
				},
				"platform": {
					Type:        "string",
					Description: "The platform (IOS, MAC_OS, TV_OS, VISION_OS)",
				},
				"copyright": {
					Type:        "string",
					Description: "The copyright text",
				},
				"release_type": {
					Type:        "string",
					Description: "Release type (MANUAL, AFTER_APPROVAL, SCHEDULED)",
				},
			},
			Required: []string{"app_id", "version_string", "platform"},
		},
	}, r.handleCreateAppStoreVersion)

	// Update app store version
	r.register(mcp.Tool{
		Name:        "update_app_store_version",
		Description: "Update an App Store version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The App Store version ID",
				},
				"version_string": {
					Type:        "string",
					Description: "The updated version string",
				},
				"copyright": {
					Type:        "string",
					Description: "The updated copyright text",
				},
				"release_type": {
					Type:        "string",
					Description: "Release type (MANUAL, AFTER_APPROVAL, SCHEDULED)",
				},
			},
			Required: []string{"version_id"},
		},
	}, r.handleUpdateAppStoreVersion)

	// Delete app store version
	r.register(mcp.Tool{
		Name:        "delete_app_store_version",
		Description: "Delete an App Store version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The App Store version ID",
				},
			},
			Required: []string{"version_id"},
		},
	}, r.handleDeleteAppStoreVersion)

	// Submit for review
	r.register(mcp.Tool{
		Name:        "submit_app_for_review",
		Description: "Submit an App Store version for review",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The App Store version ID to submit",
				},
			},
			Required: []string{"version_id"},
		},
	}, r.handleSubmitAppForReview)

	// Get review detail
	r.register(mcp.Tool{
		Name:        "get_app_store_review_detail",
		Description: "Get App Store review details for a version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The App Store version ID",
				},
			},
			Required: []string{"version_id"},
		},
	}, r.handleGetAppStoreReviewDetail)

	// Create review detail
	r.register(mcp.Tool{
		Name:        "create_app_store_review_detail",
		Description: "Create App Store review details for a version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The App Store version ID",
				},
				"contact_first_name": {
					Type:        "string",
					Description: "Contact first name",
				},
				"contact_last_name": {
					Type:        "string",
					Description: "Contact last name",
				},
				"contact_phone": {
					Type:        "string",
					Description: "Contact phone number",
				},
				"contact_email": {
					Type:        "string",
					Description: "Contact email",
				},
				"demo_account_name": {
					Type:        "string",
					Description: "Demo account username",
				},
				"demo_account_password": {
					Type:        "string",
					Description: "Demo account password",
				},
				"demo_account_required": {
					Type:        "boolean",
					Description: "Whether demo account is required",
				},
				"notes": {
					Type:        "string",
					Description: "Notes for the reviewer",
				},
			},
			Required: []string{"version_id"},
		},
	}, r.handleCreateAppStoreReviewDetail)

	// Update review detail
	r.register(mcp.Tool{
		Name:        "update_app_store_review_detail",
		Description: "Update App Store review details",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"detail_id": {
					Type:        "string",
					Description: "The review detail ID",
				},
				"contact_first_name": {
					Type:        "string",
					Description: "Contact first name",
				},
				"contact_last_name": {
					Type:        "string",
					Description: "Contact last name",
				},
				"contact_phone": {
					Type:        "string",
					Description: "Contact phone number",
				},
				"contact_email": {
					Type:        "string",
					Description: "Contact email",
				},
				"demo_account_name": {
					Type:        "string",
					Description: "Demo account username",
				},
				"demo_account_password": {
					Type:        "string",
					Description: "Demo account password",
				},
				"demo_account_required": {
					Type:        "boolean",
					Description: "Whether demo account is required",
				},
				"notes": {
					Type:        "string",
					Description: "Notes for the reviewer",
				},
			},
			Required: []string{"detail_id"},
		},
	}, r.handleUpdateAppStoreReviewDetail)
}

func (r *Registry) handleListAppStoreVersions(args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := r.client.GetAppVersions(context.Background(), params.AppID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list app store versions: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppStoreVersions(resp.Data)), nil
}

func (r *Registry) handleGetAppStoreVersion(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return nil, fmt.Errorf("version_id is required")
	}

	resp, err := r.client.GetAppStoreVersion(context.Background(), params.VersionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app store version: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppStoreVersion(resp.Data)), nil
}

func (r *Registry) handleCreateAppStoreVersion(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID         string `json:"app_id"`
		VersionString string `json:"version_string"`
		Platform      string `json:"platform"`
		Copyright     string `json:"copyright"`
		ReleaseType   string `json:"release_type"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}
	if params.VersionString == "" {
		return nil, fmt.Errorf("version_string is required")
	}
	if params.Platform == "" {
		return nil, fmt.Errorf("platform is required")
	}

	req := &api.AppStoreVersionCreateRequest{
		Data: api.AppStoreVersionCreateData{
			Type: "appStoreVersions",
			Attributes: api.AppStoreVersionCreateAttributes{
				Platform:      params.Platform,
				VersionString: params.VersionString,
				Copyright:     params.Copyright,
				ReleaseType:   params.ReleaseType,
			},
			Relationships: api.AppStoreVersionCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "apps",
						ID:   params.AppID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateAppStoreVersion(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create app store version: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Created app store version: %s (ID: %s)", resp.Data.Attributes.VersionString, resp.Data.ID)), nil
}

func (r *Registry) handleUpdateAppStoreVersion(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID     string `json:"version_id"`
		VersionString string `json:"version_string"`
		Copyright     string `json:"copyright"`
		ReleaseType   string `json:"release_type"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return nil, fmt.Errorf("version_id is required")
	}

	req := &api.AppStoreVersionUpdateRequest{
		Data: api.AppStoreVersionUpdateData{
			Type: "appStoreVersions",
			ID:   params.VersionID,
			Attributes: api.AppStoreVersionUpdateAttributes{
				VersionString: params.VersionString,
				Copyright:     params.Copyright,
				ReleaseType:   params.ReleaseType,
			},
		},
	}

	resp, err := r.client.UpdateAppStoreVersion(context.Background(), params.VersionID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update app store version: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Updated app store version: %s", resp.Data.ID)), nil
}

func (r *Registry) handleDeleteAppStoreVersion(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return nil, fmt.Errorf("version_id is required")
	}

	err := r.client.DeleteAppStoreVersion(context.Background(), params.VersionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete app store version: %v", err)), nil
	}

	return mcp.NewSuccessResult("App store version deleted successfully"), nil
}

func (r *Registry) handleSubmitAppForReview(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return nil, fmt.Errorf("version_id is required")
	}

	req := &api.AppStoreVersionSubmissionCreateRequest{
		Data: api.AppStoreVersionSubmissionCreateData{
			Type: "appStoreVersionSubmissions",
			Relationships: api.AppStoreVersionSubmissionCreateRelationships{
				AppStoreVersion: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "appStoreVersions",
						ID:   params.VersionID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateAppStoreVersionSubmission(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to submit app for review: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("App submitted for review (submission ID: %s)", resp.Data.ID)), nil
}

func (r *Registry) handleGetAppStoreReviewDetail(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return nil, fmt.Errorf("version_id is required")
	}

	resp, err := r.client.GetAppStoreReviewDetail(context.Background(), params.VersionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get review detail: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatReviewDetail(resp.Data)), nil
}

func (r *Registry) handleCreateAppStoreReviewDetail(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID           string `json:"version_id"`
		ContactFirstName    string `json:"contact_first_name"`
		ContactLastName     string `json:"contact_last_name"`
		ContactPhone        string `json:"contact_phone"`
		ContactEmail        string `json:"contact_email"`
		DemoAccountName     string `json:"demo_account_name"`
		DemoAccountPassword string `json:"demo_account_password"`
		DemoAccountRequired *bool  `json:"demo_account_required"`
		Notes               string `json:"notes"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return nil, fmt.Errorf("version_id is required")
	}

	req := &api.AppStoreReviewDetailCreateRequest{
		Data: api.AppStoreReviewDetailCreateData{
			Type: "appStoreReviewDetails",
			Attributes: api.AppStoreReviewDetailCreateAttributes{
				ContactFirstName:    params.ContactFirstName,
				ContactLastName:     params.ContactLastName,
				ContactPhone:        params.ContactPhone,
				ContactEmail:        params.ContactEmail,
				DemoAccountName:     params.DemoAccountName,
				DemoAccountPassword: params.DemoAccountPassword,
				DemoAccountRequired: params.DemoAccountRequired,
				Notes:               params.Notes,
			},
			Relationships: api.AppStoreReviewDetailCreateRelationships{
				AppStoreVersion: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "appStoreVersions",
						ID:   params.VersionID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateAppStoreReviewDetail(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create review detail: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Created review detail: %s", resp.Data.ID)), nil
}

func (r *Registry) handleUpdateAppStoreReviewDetail(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DetailID            string `json:"detail_id"`
		ContactFirstName    string `json:"contact_first_name"`
		ContactLastName     string `json:"contact_last_name"`
		ContactPhone        string `json:"contact_phone"`
		ContactEmail        string `json:"contact_email"`
		DemoAccountName     string `json:"demo_account_name"`
		DemoAccountPassword string `json:"demo_account_password"`
		DemoAccountRequired *bool  `json:"demo_account_required"`
		Notes               string `json:"notes"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DetailID == "" {
		return nil, fmt.Errorf("detail_id is required")
	}

	req := &api.AppStoreReviewDetailUpdateRequest{
		Data: api.AppStoreReviewDetailUpdateData{
			Type: "appStoreReviewDetails",
			ID:   params.DetailID,
			Attributes: api.AppStoreReviewDetailUpdateAttributes{
				ContactFirstName:    params.ContactFirstName,
				ContactLastName:     params.ContactLastName,
				ContactPhone:        params.ContactPhone,
				ContactEmail:        params.ContactEmail,
				DemoAccountName:     params.DemoAccountName,
				DemoAccountPassword: params.DemoAccountPassword,
				DemoAccountRequired: params.DemoAccountRequired,
				Notes:               params.Notes,
			},
		},
	}

	resp, err := r.client.UpdateAppStoreReviewDetail(context.Background(), params.DetailID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update review detail: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Updated review detail: %s", resp.Data.ID)), nil
}

func formatAppStoreVersions(versions []api.AppStoreVersion) string {
	if len(versions) == 0 {
		return "No app store versions found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d app store versions:\n\n", len(versions)))

	for _, version := range versions {
		sb.WriteString(formatAppStoreVersion(version))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAppStoreVersion(version api.AppStoreVersion) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", version.ID))
	sb.WriteString(fmt.Sprintf("Version: %s\n", version.Attributes.VersionString))
	sb.WriteString(fmt.Sprintf("Platform: %s\n", version.Attributes.Platform))
	sb.WriteString(fmt.Sprintf("State: %s\n", version.Attributes.AppStoreState))
	if version.Attributes.Copyright != "" {
		sb.WriteString(fmt.Sprintf("Copyright: %s\n", version.Attributes.Copyright))
	}
	if version.Attributes.ReleaseType != "" {
		sb.WriteString(fmt.Sprintf("Release Type: %s\n", version.Attributes.ReleaseType))
	}
	if version.Attributes.CreatedDate != nil {
		sb.WriteString(fmt.Sprintf("Created: %s\n", version.Attributes.CreatedDate.Format("2006-01-02")))
	}
	return sb.String()
}

func formatReviewDetail(detail api.AppStoreReviewDetail) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", detail.ID))
	if detail.Attributes.ContactFirstName != "" || detail.Attributes.ContactLastName != "" {
		sb.WriteString(fmt.Sprintf("Contact: %s %s\n", detail.Attributes.ContactFirstName, detail.Attributes.ContactLastName))
	}
	if detail.Attributes.ContactEmail != "" {
		sb.WriteString(fmt.Sprintf("Email: %s\n", detail.Attributes.ContactEmail))
	}
	if detail.Attributes.ContactPhone != "" {
		sb.WriteString(fmt.Sprintf("Phone: %s\n", detail.Attributes.ContactPhone))
	}
	sb.WriteString(fmt.Sprintf("Demo Account Required: %t\n", detail.Attributes.DemoAccountRequired))
	if detail.Attributes.DemoAccountName != "" {
		sb.WriteString(fmt.Sprintf("Demo Account: %s\n", detail.Attributes.DemoAccountName))
	}
	if detail.Attributes.Notes != "" {
		sb.WriteString(fmt.Sprintf("Notes: %s\n", detail.Attributes.Notes))
	}
	return sb.String()
}
