// Package tools provides MCP tool implementations for App Store Connect.
package tools

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerAppInfoLocalizationTools registers app info localization tools.
func (r *Registry) registerAppInfoLocalizationTools() {
	r.register(mcp.Tool{
		Name:        "list_app_info_localizations",
		Description: "List all localizations for an app's metadata (name, subtitle, privacy URLs). Requires the app_info_id which can be obtained from get_app_infos.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_info_id": {
					Type:        "string",
					Description: "The app info ID",
				},
			},
			Required: []string{"app_info_id"},
		},
	}, r.handleListAppInfoLocalizations)

	r.register(mcp.Tool{
		Name:        "get_app_info_localization",
		Description: "Get a specific app info localization by ID.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The app info localization ID",
				},
			},
			Required: []string{"localization_id"},
		},
	}, r.handleGetAppInfoLocalization)

	r.register(mcp.Tool{
		Name:        "create_app_info_localization",
		Description: "Create a new localization for app metadata. Use this to add support for a new language.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_info_id": {
					Type:        "string",
					Description: "The app info ID",
				},
				"locale": {
					Type:        "string",
					Description: "The locale code (e.g., en-US, de-DE, ja)",
				},
				"name": {
					Type:        "string",
					Description: "The app name for this locale",
				},
				"subtitle": {
					Type:        "string",
					Description: "The app subtitle for this locale (optional)",
				},
				"privacy_policy_url": {
					Type:        "string",
					Description: "Privacy policy URL (optional)",
				},
				"privacy_choices_url": {
					Type:        "string",
					Description: "Privacy choices URL (optional)",
				},
				"privacy_policy_text": {
					Type:        "string",
					Description: "Privacy policy text (optional)",
				},
			},
			Required: []string{"app_info_id", "locale", "name"},
		},
	}, r.handleCreateAppInfoLocalization)

	r.register(mcp.Tool{
		Name:        "update_app_info_localization",
		Description: "Update an existing app info localization. Use this to change the app name, subtitle, or privacy information for a specific locale.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The app info localization ID",
				},
				"name": {
					Type:        "string",
					Description: "The app name (optional)",
				},
				"subtitle": {
					Type:        "string",
					Description: "The app subtitle (optional)",
				},
				"privacy_policy_url": {
					Type:        "string",
					Description: "Privacy policy URL (optional)",
				},
				"privacy_choices_url": {
					Type:        "string",
					Description: "Privacy choices URL (optional)",
				},
				"privacy_policy_text": {
					Type:        "string",
					Description: "Privacy policy text (optional)",
				},
			},
			Required: []string{"localization_id"},
		},
	}, r.handleUpdateAppInfoLocalization)

	r.register(mcp.Tool{
		Name:        "delete_app_info_localization",
		Description: "Delete an app info localization. This removes support for a specific language.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The app info localization ID to delete",
				},
			},
			Required: []string{"localization_id"},
		},
	}, r.handleDeleteAppInfoLocalization)

	r.register(mcp.Tool{
		Name:        "get_app_infos",
		Description: "Get app info resources for an app. Returns app info IDs needed for localization operations.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App Store Connect app ID",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleGetAppInfos)
}

// registerVersionLocalizationTools registers app store version localization tools.
func (r *Registry) registerVersionLocalizationTools() {
	r.register(mcp.Tool{
		Name:        "list_version_localizations",
		Description: "List all localizations for an app store version (descriptions, keywords, what's new). Requires a version_id from get_app_versions.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The app store version ID",
				},
			},
			Required: []string{"version_id"},
		},
	}, r.handleListVersionLocalizations)

	r.register(mcp.Tool{
		Name:        "get_version_localization",
		Description: "Get a specific version localization by ID.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The version localization ID",
				},
			},
			Required: []string{"localization_id"},
		},
	}, r.handleGetVersionLocalization)

	r.register(mcp.Tool{
		Name:        "create_version_localization",
		Description: "Create a new localization for an app store version. Use this to add support for a new language on a specific version.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The app store version ID",
				},
				"locale": {
					Type:        "string",
					Description: "The locale code (e.g., en-US, de-DE, ja)",
				},
				"description": {
					Type:        "string",
					Description: "The full app description for this locale (optional)",
				},
				"keywords": {
					Type:        "string",
					Description: "Comma-separated keywords for App Store search (optional, max 100 chars)",
				},
				"whats_new": {
					Type:        "string",
					Description: "Release notes / what's new text (optional)",
				},
				"promotional_text": {
					Type:        "string",
					Description: "Promotional text that appears above the description (optional)",
				},
				"marketing_url": {
					Type:        "string",
					Description: "Marketing URL (optional)",
				},
				"support_url": {
					Type:        "string",
					Description: "Support URL (optional)",
				},
			},
			Required: []string{"version_id", "locale"},
		},
	}, r.handleCreateVersionLocalization)

	r.register(mcp.Tool{
		Name:        "update_version_localization",
		Description: "Update an existing version localization. Use this to change description, keywords, what's new, and other metadata for a specific locale.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The version localization ID",
				},
				"description": {
					Type:        "string",
					Description: "The full app description (optional)",
				},
				"keywords": {
					Type:        "string",
					Description: "Comma-separated keywords for App Store search (optional, max 100 chars)",
				},
				"whats_new": {
					Type:        "string",
					Description: "Release notes / what's new text (optional)",
				},
				"promotional_text": {
					Type:        "string",
					Description: "Promotional text that appears above the description (optional)",
				},
				"marketing_url": {
					Type:        "string",
					Description: "Marketing URL (optional)",
				},
				"support_url": {
					Type:        "string",
					Description: "Support URL (optional)",
				},
			},
			Required: []string{"localization_id"},
		},
	}, r.handleUpdateVersionLocalization)

	r.register(mcp.Tool{
		Name:        "delete_version_localization",
		Description: "Delete a version localization. This removes support for a specific language from a version.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The version localization ID to delete",
				},
			},
			Required: []string{"localization_id"},
		},
	}, r.handleDeleteVersionLocalization)
}

// App Info Localization handlers

func (r *Registry) handleGetAppInfos(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}

	ctx := context.Background()
	resp, err := r.client.GetAppInfos(ctx, params.AppID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app infos: %v", err)), nil
	}

	result := formatAppInfos(resp.Data)
	return mcp.NewSuccessResult(result), nil
}

func (r *Registry) handleListAppInfoLocalizations(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppInfoID string `json:"app_info_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppInfoID == "" {
		return mcp.NewErrorResult("app_info_id is required"), nil
	}

	ctx := context.Background()
	resp, err := r.client.ListAppInfoLocalizations(ctx, params.AppInfoID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list app info localizations: %v", err)), nil
	}

	result := formatAppInfoLocalizations(resp.Data)
	return mcp.NewSuccessResult(result), nil
}

func (r *Registry) handleGetAppInfoLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	ctx := context.Background()
	resp, err := r.client.GetAppInfoLocalization(ctx, params.LocalizationID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app info localization: %v", err)), nil
	}

	result := formatAppInfoLocalization(&resp.Data)
	return mcp.NewSuccessResult(result), nil
}

func (r *Registry) handleCreateAppInfoLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppInfoID         string `json:"app_info_id"`
		Locale            string `json:"locale"`
		Name              string `json:"name"`
		Subtitle          string `json:"subtitle"`
		PrivacyPolicyURL  string `json:"privacy_policy_url"`
		PrivacyChoicesURL string `json:"privacy_choices_url"`
		PrivacyPolicyText string `json:"privacy_policy_text"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppInfoID == "" || params.Locale == "" || params.Name == "" {
		return mcp.NewErrorResult("app_info_id, locale, and name are required"), nil
	}

	req := &api.AppInfoLocalizationCreateRequest{
		Data: api.AppInfoLocalizationCreateData{
			Type: "appInfoLocalizations",
			Attributes: api.AppInfoLocalizationCreateAttributes{
				Locale:            params.Locale,
				Name:              params.Name,
				Subtitle:          params.Subtitle,
				PrivacyPolicyURL:  params.PrivacyPolicyURL,
				PrivacyChoicesURL: params.PrivacyChoicesURL,
				PrivacyPolicyText: params.PrivacyPolicyText,
			},
			Relationships: api.AppInfoLocalizationCreateRelationships{
				AppInfo: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "appInfos",
						ID:   params.AppInfoID,
					},
				},
			},
		},
	}

	ctx := context.Background()
	resp, err := r.client.CreateAppInfoLocalization(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create app info localization: %v", err)), nil
	}

	result := fmt.Sprintf("Created app info localization for locale '%s'\n\n%s",
		params.Locale, formatAppInfoLocalization(&resp.Data))
	return mcp.NewSuccessResult(result), nil
}

func (r *Registry) handleUpdateAppInfoLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID    string `json:"localization_id"`
		Name              string `json:"name"`
		Subtitle          string `json:"subtitle"`
		PrivacyPolicyURL  string `json:"privacy_policy_url"`
		PrivacyChoicesURL string `json:"privacy_choices_url"`
		PrivacyPolicyText string `json:"privacy_policy_text"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	req := &api.AppInfoLocalizationUpdateRequest{
		Data: api.AppInfoLocalizationUpdateData{
			Type: "appInfoLocalizations",
			ID:   params.LocalizationID,
			Attributes: api.AppInfoLocalizationUpdateAttributes{
				Name:              params.Name,
				Subtitle:          params.Subtitle,
				PrivacyPolicyURL:  params.PrivacyPolicyURL,
				PrivacyChoicesURL: params.PrivacyChoicesURL,
				PrivacyPolicyText: params.PrivacyPolicyText,
			},
		},
	}

	ctx := context.Background()
	resp, err := r.client.UpdateAppInfoLocalization(ctx, params.LocalizationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update app info localization: %v", err)), nil
	}

	result := fmt.Sprintf("Updated app info localization\n\n%s", formatAppInfoLocalization(&resp.Data))
	return mcp.NewSuccessResult(result), nil
}

func (r *Registry) handleDeleteAppInfoLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	ctx := context.Background()
	err := r.client.DeleteAppInfoLocalization(ctx, params.LocalizationID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete app info localization: %v", err)), nil
	}

	return mcp.NewSuccessResult("Successfully deleted app info localization"), nil
}

// Version Localization handlers

func (r *Registry) handleListVersionLocalizations(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	ctx := context.Background()
	resp, err := r.client.ListAppStoreVersionLocalizations(ctx, params.VersionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list version localizations: %v", err)), nil
	}

	result := formatVersionLocalizations(resp.Data)
	return mcp.NewSuccessResult(result), nil
}

func (r *Registry) handleGetVersionLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	ctx := context.Background()
	resp, err := r.client.GetAppStoreVersionLocalization(ctx, params.LocalizationID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get version localization: %v", err)), nil
	}

	result := formatVersionLocalization(&resp.Data)
	return mcp.NewSuccessResult(result), nil
}

func (r *Registry) handleCreateVersionLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID       string `json:"version_id"`
		Locale          string `json:"locale"`
		Description     string `json:"description"`
		Keywords        string `json:"keywords"`
		WhatsNew        string `json:"whats_new"`
		PromotionalText string `json:"promotional_text"`
		MarketingURL    string `json:"marketing_url"`
		SupportURL      string `json:"support_url"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" || params.Locale == "" {
		return mcp.NewErrorResult("version_id and locale are required"), nil
	}

	req := &api.AppStoreVersionLocalizationCreateRequest{
		Data: api.AppStoreVersionLocalizationCreateData{
			Type: "appStoreVersionLocalizations",
			Attributes: api.AppStoreVersionLocalizationCreateAttributes{
				Locale:          params.Locale,
				Description:     params.Description,
				Keywords:        params.Keywords,
				WhatsNew:        params.WhatsNew,
				PromotionalText: params.PromotionalText,
				MarketingURL:    params.MarketingURL,
				SupportURL:      params.SupportURL,
			},
			Relationships: api.AppStoreVersionLocalizationCreateRelationships{
				AppStoreVersion: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "appStoreVersions",
						ID:   params.VersionID,
					},
				},
			},
		},
	}

	ctx := context.Background()
	resp, err := r.client.CreateAppStoreVersionLocalization(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create version localization: %v", err)), nil
	}

	result := fmt.Sprintf("Created version localization for locale '%s'\n\n%s",
		params.Locale, formatVersionLocalization(&resp.Data))
	return mcp.NewSuccessResult(result), nil
}

func (r *Registry) handleUpdateVersionLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID  string `json:"localization_id"`
		Description     string `json:"description"`
		Keywords        string `json:"keywords"`
		WhatsNew        string `json:"whats_new"`
		PromotionalText string `json:"promotional_text"`
		MarketingURL    string `json:"marketing_url"`
		SupportURL      string `json:"support_url"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	req := &api.AppStoreVersionLocalizationUpdateRequest{
		Data: api.AppStoreVersionLocalizationUpdateData{
			Type: "appStoreVersionLocalizations",
			ID:   params.LocalizationID,
			Attributes: api.AppStoreVersionLocalizationUpdateAttributes{
				Description:     params.Description,
				Keywords:        params.Keywords,
				WhatsNew:        params.WhatsNew,
				PromotionalText: params.PromotionalText,
				MarketingURL:    params.MarketingURL,
				SupportURL:      params.SupportURL,
			},
		},
	}

	ctx := context.Background()
	resp, err := r.client.UpdateAppStoreVersionLocalization(ctx, params.LocalizationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update version localization: %v", err)), nil
	}

	result := fmt.Sprintf("Updated version localization\n\n%s", formatVersionLocalization(&resp.Data))
	return mcp.NewSuccessResult(result), nil
}

func (r *Registry) handleDeleteVersionLocalization(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	ctx := context.Background()
	err := r.client.DeleteAppStoreVersionLocalization(ctx, params.LocalizationID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete version localization: %v", err)), nil
	}

	return mcp.NewSuccessResult("Successfully deleted version localization"), nil
}

// Formatting helpers

func formatAppInfos(infos []api.AppInfo) string {
	if len(infos) == 0 {
		return "No app infos found."
	}

	var result string
	for i, info := range infos {
		if i > 0 {
			result += "\n---\n"
		}
		result += fmt.Sprintf("App Info ID: %s\n", info.ID)
		result += fmt.Sprintf("State: %s\n", info.Attributes.State)
		if info.Attributes.AppStoreState != "" {
			result += fmt.Sprintf("App Store State: %s\n", info.Attributes.AppStoreState)
		}
	}
	return result
}

func formatAppInfoLocalizations(localizations []api.AppInfoLocalization) string {
	if len(localizations) == 0 {
		return "No localizations found."
	}

	var result string
	for i, loc := range localizations {
		if i > 0 {
			result += "\n---\n"
		}
		result += formatAppInfoLocalization(&loc)
	}
	return result
}

func formatAppInfoLocalization(loc *api.AppInfoLocalization) string {
	result := fmt.Sprintf("ID: %s\n", loc.ID)
	result += fmt.Sprintf("Locale: %s\n", loc.Attributes.Locale)
	if loc.Attributes.Name != "" {
		result += fmt.Sprintf("Name: %s\n", loc.Attributes.Name)
	}
	if loc.Attributes.Subtitle != "" {
		result += fmt.Sprintf("Subtitle: %s\n", loc.Attributes.Subtitle)
	}
	if loc.Attributes.PrivacyPolicyURL != "" {
		result += fmt.Sprintf("Privacy Policy URL: %s\n", loc.Attributes.PrivacyPolicyURL)
	}
	if loc.Attributes.PrivacyChoicesURL != "" {
		result += fmt.Sprintf("Privacy Choices URL: %s\n", loc.Attributes.PrivacyChoicesURL)
	}
	if loc.Attributes.PrivacyPolicyText != "" {
		result += fmt.Sprintf("Privacy Policy Text: %s\n", loc.Attributes.PrivacyPolicyText)
	}
	return result
}

func formatVersionLocalizations(localizations []api.AppStoreVersionLocalization) string {
	if len(localizations) == 0 {
		return "No localizations found."
	}

	var result string
	for i, loc := range localizations {
		if i > 0 {
			result += "\n---\n"
		}
		result += formatVersionLocalization(&loc)
	}
	return result
}

func formatVersionLocalization(loc *api.AppStoreVersionLocalization) string {
	result := fmt.Sprintf("ID: %s\n", loc.ID)
	result += fmt.Sprintf("Locale: %s\n", loc.Attributes.Locale)
	if loc.Attributes.Description != "" {
		desc := loc.Attributes.Description
		if len(desc) > 200 {
			desc = desc[:200] + "..."
		}
		result += fmt.Sprintf("Description: %s\n", desc)
	}
	if loc.Attributes.Keywords != "" {
		result += fmt.Sprintf("Keywords: %s\n", loc.Attributes.Keywords)
	}
	if loc.Attributes.WhatsNew != "" {
		whatsNew := loc.Attributes.WhatsNew
		if len(whatsNew) > 200 {
			whatsNew = whatsNew[:200] + "..."
		}
		result += fmt.Sprintf("What's New: %s\n", whatsNew)
	}
	if loc.Attributes.PromotionalText != "" {
		result += fmt.Sprintf("Promotional Text: %s\n", loc.Attributes.PromotionalText)
	}
	if loc.Attributes.MarketingURL != "" {
		result += fmt.Sprintf("Marketing URL: %s\n", loc.Attributes.MarketingURL)
	}
	if loc.Attributes.SupportURL != "" {
		result += fmt.Sprintf("Support URL: %s\n", loc.Attributes.SupportURL)
	}
	return result
}
