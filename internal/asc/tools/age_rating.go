package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerAgeRatingTools registers age rating declaration tools.
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
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Age Rating Declaration",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
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
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update Age Rating Declaration",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateAgeRatingDeclaration)

	// List territory age ratings
	r.register(mcp.Tool{
		Name:        "list_territory_age_ratings",
		Description: "List an app's App Store age rating per territory for an app info",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_info_id": {
					Type:        "string",
					Description: "The app info ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of territory age ratings to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
				"fields": {
					Type:        "object",
					Description: "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return.",
				},
				"include": {
					Type:        "array",
					Description: "Related resource names to include in the response (supported: territory).",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"app_info_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Territory Age Ratings",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListTerritoryAgeRatings)
}

func (r *Registry) handleGetAgeRatingDeclaration(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppInfoID string `json:"app_info_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppInfoID == "" {
		return mcp.NewErrorResult("app_info_id is required"), nil
	}

	resp, err := r.client.GetAgeRatingDeclaration(ctx, params.AppInfoID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get age rating declaration: %v", err)), nil
	}

	return newDataResult(formatAgeRatingDeclaration(resp.Data), resp.Data), nil
}

func (r *Registry) handleUpdateAgeRatingDeclaration(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DeclarationID                               string  `json:"declaration_id"`
		AlcoholTobaccoOrDrugUseOrReferences         *string `json:"alcohol_tobacco_or_drug_use_or_references"`
		Contests                                    *string `json:"contests"`
		GamblingSimulated                           *string `json:"gambling_simulated"`
		HorrorOrFearThemes                          *string `json:"horror_or_fear_themes"`
		MatureOrSuggestiveThemes                    *string `json:"mature_or_suggestive_themes"`
		MedicalOrTreatmentInformation               *string `json:"medical_or_treatment_information"`
		ProfanityOrCrudeHumor                       *string `json:"profanity_or_crude_humor"`
		SexualContentGraphicAndNudity               *string `json:"sexual_content_graphic_and_nudity"`
		SexualContentOrNudity                       *string `json:"sexual_content_or_nudity"`
		ViolenceCartoonOrFantasy                    *string `json:"violence_cartoon_or_fantasy"`
		ViolenceRealistic                           *string `json:"violence_realistic"`
		ViolenceRealisticProlongedGraphicOrSadistic *string `json:"violence_realistic_prolonged_graphic_or_sadistic"`
		Gambling                                    *bool   `json:"gambling"`
		UnrestrictedWebAccess                       *bool   `json:"unrestricted_web_access"`
		SeventeenPlus                               *bool   `json:"seventeen_plus"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DeclarationID == "" {
		return mcp.NewErrorResult("declaration_id is required"), nil
	}

	req := &api.AgeRatingDeclarationUpdateRequest{
		Data: api.AgeRatingDeclarationUpdateData{
			Type: "ageRatingDeclarations",
			ID:   params.DeclarationID,
			Attributes: api.AgeRatingDeclarationUpdateAttributes{
				AlcoholTobaccoOrDrugUseOrReferences:         stringValue(params.AlcoholTobaccoOrDrugUseOrReferences),
				Contests:                                    stringValue(params.Contests),
				GamblingSimulated:                           stringValue(params.GamblingSimulated),
				MatureOrSuggestiveThemes:                    stringValue(params.MatureOrSuggestiveThemes),
				MedicalOrTreatmentInformation:               stringValue(params.MedicalOrTreatmentInformation),
				ProfanityOrCrudeHumor:                       stringValue(params.ProfanityOrCrudeHumor),
				SexualContentGraphicAndNudity:               stringValue(params.SexualContentGraphicAndNudity),
				SexualContentOrNudity:                       stringValue(params.SexualContentOrNudity),
				ViolenceCartoonOrFantasy:                    stringValue(params.ViolenceCartoonOrFantasy),
				ViolenceRealistic:                           stringValue(params.ViolenceRealistic),
				ViolenceRealisticProlongedGraphicOrSadistic: stringValue(params.ViolenceRealisticProlongedGraphicOrSadistic),
				Gambling:              params.Gambling,
				UnrestrictedWebAccess: params.UnrestrictedWebAccess,
				SeventeenPlus:         params.SeventeenPlus,
			},
		},
	}

	resp, err := r.client.UpdateAgeRatingDeclaration(ctx, params.DeclarationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update age rating declaration: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Age rating declaration updated:\n%s", formatAgeRatingDeclaration(resp.Data)), resp.Data), nil
}

func (r *Registry) handleListTerritoryAgeRatings(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppInfoID string              `json:"app_info_id"`
		Limit     int                 `json:"limit"`
		Cursor    string              `json:"cursor"`
		Fields    map[string][]string `json:"fields"`
		Include   []string            `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppInfoID == "" {
		return mcp.NewErrorResult("app_info_id is required"), nil
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.TerritoryAgeRatingsResponse, error) {
		return r.client.ListTerritoryAgeRatings(ctx, params.AppInfoID, listOpts(limit, nil, nil, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list territory age ratings: %v", err)), nil
	}

	return newListResult(formatTerritoryAgeRatings(resp.Data), resp.Data, resp.Links), nil
}

func formatTerritoryAgeRatings(ratings []api.TerritoryAgeRating) string {
	if len(ratings) == 0 {
		return "No territory age ratings found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d territory age ratings:\n\n", len(ratings)))

	for _, rating := range ratings {
		sb.WriteString(fmt.Sprintf("ID: %s\n", rating.ID))
		if rating.Attributes.AppStoreAgeRating != "" {
			sb.WriteString(fmt.Sprintf("App Store Age Rating: %s\n", rating.Attributes.AppStoreAgeRating))
		}
		sb.WriteString("---\n")
	}

	return sb.String()
}

func stringValue(s *string) string {
	if s == nil {
		return ""
	}
	return *s
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
