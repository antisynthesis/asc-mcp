package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerAgeRatingTools registers age rating and IDFA declaration tools.
func (r *Registry) registerAgeRatingTools() {
	// Get age rating declaration
	r.register(mcp.Tool{
		Name:        "get_age_rating_declaration",
		Description: "Get the age rating declaration for an app info",
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
	}, r.handleGetAgeRatingDeclaration)

	// Update age rating declaration
	r.register(mcp.Tool{
		Name:        "update_age_rating_declaration",
		Description: "Update the age rating declaration for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"declaration_id": {
					Type:        "string",
					Description: "The age rating declaration ID",
				},
				"alcohol_tobacco_or_drug_use_or_references": {
					Type:        "string",
					Description: "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE",
				},
				"contests": {
					Type:        "string",
					Description: "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE",
				},
				"gambling_simulated": {
					Type:        "string",
					Description: "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE",
				},
				"horror_or_fear_themes": {
					Type:        "string",
					Description: "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE",
				},
				"mature_or_suggestive_themes": {
					Type:        "string",
					Description: "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE",
				},
				"medical_or_treatment_information": {
					Type:        "string",
					Description: "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE",
				},
				"profanity_or_crude_humor": {
					Type:        "string",
					Description: "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE",
				},
				"sexual_content_graphic_and_nudity": {
					Type:        "string",
					Description: "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE",
				},
				"sexual_content_or_nudity": {
					Type:        "string",
					Description: "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE",
				},
				"violence_cartoon_or_fantasy": {
					Type:        "string",
					Description: "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE",
				},
				"violence_realistic": {
					Type:        "string",
					Description: "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE",
				},
				"violence_realistic_prolonged_graphic_or_sadistic": {
					Type:        "string",
					Description: "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE",
				},
				"gambling": {
					Type:        "boolean",
					Description: "Whether app contains gambling",
				},
				"unrestricted_web_access": {
					Type:        "boolean",
					Description: "Whether app has unrestricted web access",
				},
				"seventeen_plus": {
					Type:        "boolean",
					Description: "Whether app is for 17+",
				},
			},
			Required: []string{"declaration_id"},
		},
	}, r.handleUpdateAgeRatingDeclaration)

	// Get IDFA declaration
	r.register(mcp.Tool{
		Name:        "get_idfa_declaration",
		Description: "Get the IDFA declaration for an app store version",
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
	}, r.handleGetIdfaDeclaration)

	// Create IDFA declaration
	r.register(mcp.Tool{
		Name:        "create_idfa_declaration",
		Description: "Create an IDFA declaration for an app store version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The app store version ID",
				},
				"serves_ads": {
					Type:        "boolean",
					Description: "Whether the app serves ads",
				},
				"attributes_app_installation_to_previous_ad": {
					Type:        "boolean",
					Description: "Whether app attributes installation to ads",
				},
				"attributes_action_with_previous_ad": {
					Type:        "boolean",
					Description: "Whether app attributes actions to ads",
				},
				"honors_limited_ad_tracking": {
					Type:        "boolean",
					Description: "Whether app honors limited ad tracking",
				},
			},
			Required: []string{"version_id", "serves_ads", "attributes_app_installation_to_previous_ad", "attributes_action_with_previous_ad", "honors_limited_ad_tracking"},
		},
	}, r.handleCreateIdfaDeclaration)

	// Update IDFA declaration
	r.register(mcp.Tool{
		Name:        "update_idfa_declaration",
		Description: "Update an IDFA declaration",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"declaration_id": {
					Type:        "string",
					Description: "The IDFA declaration ID",
				},
				"serves_ads": {
					Type:        "boolean",
					Description: "Whether the app serves ads",
				},
				"attributes_app_installation_to_previous_ad": {
					Type:        "boolean",
					Description: "Whether app attributes installation to ads",
				},
				"attributes_action_with_previous_ad": {
					Type:        "boolean",
					Description: "Whether app attributes actions to ads",
				},
				"honors_limited_ad_tracking": {
					Type:        "boolean",
					Description: "Whether app honors limited ad tracking",
				},
			},
			Required: []string{"declaration_id"},
		},
	}, r.handleUpdateIdfaDeclaration)

	// Delete IDFA declaration
	r.register(mcp.Tool{
		Name:        "delete_idfa_declaration",
		Description: "Delete an IDFA declaration",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"declaration_id": {
					Type:        "string",
					Description: "The IDFA declaration ID to delete",
				},
			},
			Required: []string{"declaration_id"},
		},
	}, r.handleDeleteIdfaDeclaration)
}

func (r *Registry) handleGetAgeRatingDeclaration(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppInfoID string `json:"app_info_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppInfoID == "" {
		return nil, fmt.Errorf("app_info_id is required")
	}

	resp, err := r.client.GetAgeRatingDeclaration(context.Background(), params.AppInfoID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get age rating declaration: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAgeRatingDeclaration(resp.Data)), nil
}

func (r *Registry) handleUpdateAgeRatingDeclaration(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DeclarationID                                  string  `json:"declaration_id"`
		AlcoholTobaccoOrDrugUseOrReferences            *string `json:"alcohol_tobacco_or_drug_use_or_references"`
		Contests                                       *string `json:"contests"`
		GamblingSimulated                              *string `json:"gambling_simulated"`
		HorrorOrFearThemes                             *string `json:"horror_or_fear_themes"`
		MatureOrSuggestiveThemes                       *string `json:"mature_or_suggestive_themes"`
		MedicalOrTreatmentInformation                  *string `json:"medical_or_treatment_information"`
		ProfanityOrCrudeHumor                          *string `json:"profanity_or_crude_humor"`
		SexualContentGraphicAndNudity                  *string `json:"sexual_content_graphic_and_nudity"`
		SexualContentOrNudity                          *string `json:"sexual_content_or_nudity"`
		ViolenceCartoonOrFantasy                       *string `json:"violence_cartoon_or_fantasy"`
		ViolenceRealistic                              *string `json:"violence_realistic"`
		ViolenceRealisticProlongedGraphicOrSadistic    *string `json:"violence_realistic_prolonged_graphic_or_sadistic"`
		Gambling                                       *bool   `json:"gambling"`
		UnrestrictedWebAccess                          *bool   `json:"unrestricted_web_access"`
		SeventeenPlus                                  *bool   `json:"seventeen_plus"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DeclarationID == "" {
		return nil, fmt.Errorf("declaration_id is required")
	}

	req := &api.AgeRatingDeclarationUpdateRequest{
		Data: api.AgeRatingDeclarationUpdateData{
			Type: "ageRatingDeclarations",
			ID:   params.DeclarationID,
			Attributes: api.AgeRatingDeclarationUpdateAttributes{
				AlcoholTobaccoOrDrugUseOrReferences: stringValue(params.AlcoholTobaccoOrDrugUseOrReferences),
				Contests:                            stringValue(params.Contests),
				GamblingSimulated:                   stringValue(params.GamblingSimulated),
				MatureOrSuggestiveThemes:            stringValue(params.MatureOrSuggestiveThemes),
				MedicalOrTreatmentInformation:       stringValue(params.MedicalOrTreatmentInformation),
				ProfanityOrCrudeHumor:               stringValue(params.ProfanityOrCrudeHumor),
				SexualContentGraphicAndNudity:       stringValue(params.SexualContentGraphicAndNudity),
				SexualContentOrNudity:               stringValue(params.SexualContentOrNudity),
				ViolenceCartoonOrFantasy:            stringValue(params.ViolenceCartoonOrFantasy),
				ViolenceRealistic:                   stringValue(params.ViolenceRealistic),
				ViolenceRealisticProlongedGraphicOrSadistic: stringValue(params.ViolenceRealisticProlongedGraphicOrSadistic),
				Gambling:              params.Gambling,
				UnrestrictedWebAccess: params.UnrestrictedWebAccess,
				SeventeenPlus:         params.SeventeenPlus,
			},
		},
	}

	resp, err := r.client.UpdateAgeRatingDeclaration(context.Background(), params.DeclarationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update age rating declaration: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Age rating declaration updated:\n%s", formatAgeRatingDeclaration(resp.Data))), nil
}

func (r *Registry) handleGetIdfaDeclaration(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return nil, fmt.Errorf("version_id is required")
	}

	resp, err := r.client.GetIdfaDeclaration(context.Background(), params.VersionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get IDFA declaration: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatIdfaDeclaration(resp.Data)), nil
}

func (r *Registry) handleCreateIdfaDeclaration(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID                            string `json:"version_id"`
		ServesAds                            bool   `json:"serves_ads"`
		AttributesAppInstallationToPreviousAd bool   `json:"attributes_app_installation_to_previous_ad"`
		AttributesActionWithPreviousAd       bool   `json:"attributes_action_with_previous_ad"`
		HonorsLimitedAdTracking              bool   `json:"honors_limited_ad_tracking"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return nil, fmt.Errorf("version_id is required")
	}

	req := &api.IdfaDeclarationCreateRequest{
		Data: api.IdfaDeclarationCreateData{
			Type: "idfaDeclarations",
			Attributes: api.IdfaDeclarationCreateAttributes{
				ServesAds:                             params.ServesAds,
				AttributesAppInstallationToPreviousAd: params.AttributesAppInstallationToPreviousAd,
				AttributesActionWithPreviousAd:        params.AttributesActionWithPreviousAd,
				HonorsLimitedAdTracking:               params.HonorsLimitedAdTracking,
			},
			Relationships: api.IdfaDeclarationCreateRelationships{
				AppStoreVersion: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "appStoreVersions", ID: params.VersionID},
				},
			},
		},
	}

	resp, err := r.client.CreateIdfaDeclaration(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create IDFA declaration: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("IDFA declaration created:\n%s", formatIdfaDeclaration(resp.Data))), nil
}

func (r *Registry) handleUpdateIdfaDeclaration(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DeclarationID                        string `json:"declaration_id"`
		ServesAds                            *bool  `json:"serves_ads"`
		AttributesAppInstallationToPreviousAd *bool  `json:"attributes_app_installation_to_previous_ad"`
		AttributesActionWithPreviousAd       *bool  `json:"attributes_action_with_previous_ad"`
		HonorsLimitedAdTracking              *bool  `json:"honors_limited_ad_tracking"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DeclarationID == "" {
		return nil, fmt.Errorf("declaration_id is required")
	}

	req := &api.IdfaDeclarationUpdateRequest{
		Data: api.IdfaDeclarationUpdateData{
			Type: "idfaDeclarations",
			ID:   params.DeclarationID,
			Attributes: api.IdfaDeclarationUpdateAttributes{
				ServesAds:                             params.ServesAds,
				AttributesAppInstallationToPreviousAd: params.AttributesAppInstallationToPreviousAd,
				AttributesActionWithPreviousAd:        params.AttributesActionWithPreviousAd,
				HonorsLimitedAdTracking:               params.HonorsLimitedAdTracking,
			},
		},
	}

	resp, err := r.client.UpdateIdfaDeclaration(context.Background(), params.DeclarationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update IDFA declaration: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("IDFA declaration updated:\n%s", formatIdfaDeclaration(resp.Data))), nil
}

func (r *Registry) handleDeleteIdfaDeclaration(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DeclarationID string `json:"declaration_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DeclarationID == "" {
		return nil, fmt.Errorf("declaration_id is required")
	}

	err := r.client.DeleteIdfaDeclaration(context.Background(), params.DeclarationID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete IDFA declaration: %v", err)), nil
	}

	return mcp.NewSuccessResult("IDFA declaration deleted"), nil
}

func stringValue(s *string) string {
	if s == nil {
		return ""
	}
	return *s
}

func boolValue(b *bool) bool {
	if b == nil {
		return false
	}
	return *b
}

func formatAgeRatingDeclaration(decl api.AgeRatingDeclaration) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", decl.ID))

	attrs := decl.Attributes
	if attrs.AlcoholTobaccoOrDrugUseOrReferences != "" {
		sb.WriteString(fmt.Sprintf("Alcohol/Tobacco/Drugs: %s\n", attrs.AlcoholTobaccoOrDrugUseOrReferences))
	}
	if attrs.Contests != "" {
		sb.WriteString(fmt.Sprintf("Contests: %s\n", attrs.Contests))
	}
	if attrs.GamblingSimulated != "" {
		sb.WriteString(fmt.Sprintf("Simulated Gambling: %s\n", attrs.GamblingSimulated))
	}
	if attrs.Gambling {
		sb.WriteString("Contains Gambling: Yes\n")
	}
	if attrs.HorrorOrFearThemes != "" {
		sb.WriteString(fmt.Sprintf("Horror/Fear: %s\n", attrs.HorrorOrFearThemes))
	}
	if attrs.MatureOrSuggestiveThemes != "" {
		sb.WriteString(fmt.Sprintf("Mature/Suggestive: %s\n", attrs.MatureOrSuggestiveThemes))
	}
	if attrs.MedicalOrTreatmentInformation != "" {
		sb.WriteString(fmt.Sprintf("Medical Info: %s\n", attrs.MedicalOrTreatmentInformation))
	}
	if attrs.ProfanityOrCrudeHumor != "" {
		sb.WriteString(fmt.Sprintf("Profanity/Crude Humor: %s\n", attrs.ProfanityOrCrudeHumor))
	}
	if attrs.SexualContentGraphicAndNudity != "" {
		sb.WriteString(fmt.Sprintf("Sexual Content (Graphic): %s\n", attrs.SexualContentGraphicAndNudity))
	}
	if attrs.SexualContentOrNudity != "" {
		sb.WriteString(fmt.Sprintf("Sexual Content/Nudity: %s\n", attrs.SexualContentOrNudity))
	}
	if attrs.ViolenceCartoonOrFantasy != "" {
		sb.WriteString(fmt.Sprintf("Violence (Cartoon): %s\n", attrs.ViolenceCartoonOrFantasy))
	}
	if attrs.ViolenceRealistic != "" {
		sb.WriteString(fmt.Sprintf("Violence (Realistic): %s\n", attrs.ViolenceRealistic))
	}
	if attrs.ViolenceRealisticProlongedGraphicOrSadistic != "" {
		sb.WriteString(fmt.Sprintf("Violence (Prolonged/Graphic): %s\n", attrs.ViolenceRealisticProlongedGraphicOrSadistic))
	}
	if attrs.UnrestrictedWebAccess {
		sb.WriteString("Unrestricted Web Access: Yes\n")
	}
	if attrs.SeventeenPlus {
		sb.WriteString("17+ Age Rating: Yes\n")
	}

	return sb.String()
}

func formatIdfaDeclaration(decl api.IdfaDeclaration) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", decl.ID))
	sb.WriteString(fmt.Sprintf("Serves Ads: %t\n", decl.Attributes.ServesAds))
	sb.WriteString(fmt.Sprintf("Attributes Installation to Previous Ad: %t\n", decl.Attributes.AttributesAppInstallationToPreviousAd))
	sb.WriteString(fmt.Sprintf("Attributes Action with Previous Ad: %t\n", decl.Attributes.AttributesActionWithPreviousAd))
	sb.WriteString(fmt.Sprintf("Honors Limited Ad Tracking: %t\n", decl.Attributes.HonorsLimitedAdTracking))
	return sb.String()
}
