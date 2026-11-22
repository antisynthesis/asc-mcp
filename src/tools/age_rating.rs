//! Ported from internal/asc/tools/age_rating.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerAgeRatingTools`.
pub(super) fn register_age_rating_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "get_age_rating_declaration",
            "description": "Get the age rating declaration for an app info",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_info_id": {
                        "type": "string",
                        "description": "The app info ID"
                    }
                },
                "required": [
                    "app_info_id"
                ]
            },
            "annotations": {
                "title": "Get Age Rating Declaration",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_age_rating_declaration),
    );
    r.register(
        tool(r##"{
            "name": "update_age_rating_declaration",
            "description": "Update the age rating declaration for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "alcohol_tobacco_or_drug_use_or_references": {
                        "type": "string",
                        "description": "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE"
                    },
                    "contests": {
                        "type": "string",
                        "description": "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE"
                    },
                    "declaration_id": {
                        "type": "string",
                        "description": "The age rating declaration ID"
                    },
                    "gambling": {
                        "type": "boolean",
                        "description": "Whether app contains gambling"
                    },
                    "gambling_simulated": {
                        "type": "string",
                        "description": "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE"
                    },
                    "horror_or_fear_themes": {
                        "type": "string",
                        "description": "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE"
                    },
                    "mature_or_suggestive_themes": {
                        "type": "string",
                        "description": "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE"
                    },
                    "medical_or_treatment_information": {
                        "type": "string",
                        "description": "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE"
                    },
                    "profanity_or_crude_humor": {
                        "type": "string",
                        "description": "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE"
                    },
                    "seventeen_plus": {
                        "type": "boolean",
                        "description": "Whether app is for 17+"
                    },
                    "sexual_content_graphic_and_nudity": {
                        "type": "string",
                        "description": "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE"
                    },
                    "sexual_content_or_nudity": {
                        "type": "string",
                        "description": "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE"
                    },
                    "unrestricted_web_access": {
                        "type": "boolean",
                        "description": "Whether app has unrestricted web access"
                    },
                    "violence_cartoon_or_fantasy": {
                        "type": "string",
                        "description": "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE"
                    },
                    "violence_realistic": {
                        "type": "string",
                        "description": "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE"
                    },
                    "violence_realistic_prolonged_graphic_or_sadistic": {
                        "type": "string",
                        "description": "NONE, INFREQUENT_OR_MILD, FREQUENT_OR_INTENSE"
                    }
                },
                "required": [
                    "declaration_id"
                ]
            },
            "annotations": {
                "title": "Update Age Rating Declaration",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_age_rating_declaration),
    );
    r.register(
        tool(r##"{
            "name": "list_territory_age_ratings",
            "description": "List an app's App Store age rating per territory for an app info",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_info_id": {
                        "type": "string",
                        "description": "The app info ID"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return."
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response (supported: territory).",
                        "items": {
                            "type": "string"
                        }
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of territory age ratings to return (default 50, max 200)"
                    }
                },
                "required": [
                    "app_info_id"
                ]
            },
            "annotations": {
                "title": "List Territory Age Ratings",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_territory_age_ratings),
    );
}

/// Ported from `Registry.handleGetAgeRatingDeclaration`.
async fn handle_get_age_rating_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppInfoID string \"json:\\\"app_info_id\\\"\" }", "") {
        app_info_id: String => "app_info_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_info_id.is_empty() {
        return Ok(error_result("app_info_id is required"));
    }

    let resp = match c.get_age_rating_declaration(&params.app_info_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get age rating declaration: {e}"))),
    };

    Ok(data_result(&format_age_rating_declaration(&resp.data), &resp.data))
}

/// Ported from `Registry.handleUpdateAgeRatingDeclaration`.
async fn handle_update_age_rating_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { DeclarationID string \"json:\\\"declaration_id\\\"\"; AlcoholTobaccoOrDrugUseOrReferences *string \"json:\\\"alcohol_tobacco_or_drug_use_or_references\\\"\"; Contests *string \"json:\\\"contests\\\"\"; GamblingSimulated *string \"json:\\\"gambling_simulated\\\"\"; HorrorOrFearThemes *string \"json:\\\"horror_or_fear_themes\\\"\"; MatureOrSuggestiveThemes *string \"json:\\\"mature_or_suggestive_themes\\\"\"; MedicalOrTreatmentInformation *string \"json:\\\"medical_or_treatment_information\\\"\"; ProfanityOrCrudeHumor *string \"json:\\\"profanity_or_crude_humor\\\"\"; SexualContentGraphicAndNudity *string \"json:\\\"sexual_content_graphic_and_nudity\\\"\"; SexualContentOrNudity *string \"json:\\\"sexual_content_or_nudity\\\"\"; ViolenceCartoonOrFantasy *string \"json:\\\"violence_cartoon_or_fantasy\\\"\"; ViolenceRealistic *string \"json:\\\"violence_realistic\\\"\"; ViolenceRealisticProlongedGraphicOrSadistic *string \"json:\\\"violence_realistic_prolonged_graphic_or_sadistic\\\"\"; Gambling *bool \"json:\\\"gambling\\\"\"; UnrestrictedWebAccess *bool \"json:\\\"unrestricted_web_access\\\"\"; SeventeenPlus *bool \"json:\\\"seventeen_plus\\\"\" }", "") {
        declaration_id: String => "declaration_id";
        alcohol_tobacco_or_drug_use_or_references: Option<String> => "alcohol_tobacco_or_drug_use_or_references";
        contests: Option<String> => "contests";
        gambling_simulated: Option<String> => "gambling_simulated";
        horror_or_fear_themes: Option<String> => "horror_or_fear_themes";
        mature_or_suggestive_themes: Option<String> => "mature_or_suggestive_themes";
        medical_or_treatment_information: Option<String> => "medical_or_treatment_information";
        profanity_or_crude_humor: Option<String> => "profanity_or_crude_humor";
        sexual_content_graphic_and_nudity: Option<String> => "sexual_content_graphic_and_nudity";
        sexual_content_or_nudity: Option<String> => "sexual_content_or_nudity";
        violence_cartoon_or_fantasy: Option<String> => "violence_cartoon_or_fantasy";
        violence_realistic: Option<String> => "violence_realistic";
        violence_realistic_prolonged_graphic_or_sadistic: Option<String> => "violence_realistic_prolonged_graphic_or_sadistic";
        gambling: Option<bool> => "gambling";
        unrestricted_web_access: Option<bool> => "unrestricted_web_access";
        seventeen_plus: Option<bool> => "seventeen_plus";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.declaration_id.is_empty() {
        return Ok(error_result("declaration_id is required"));
    }

    // Note: horror_or_fear_themes is accepted but, as in Go, not sent.
    let req = AgeRatingDeclarationUpdateRequest {
        data: AgeRatingDeclarationUpdateData {
            type_: "ageRatingDeclarations".into(),
            id: params.declaration_id.clone(),
            attributes: AgeRatingDeclarationUpdateAttributes {
                alcohol_tobacco_or_drug_use_or_references: string_value(&params.alcohol_tobacco_or_drug_use_or_references),
                contests: string_value(&params.contests),
                gambling_simulated: string_value(&params.gambling_simulated),
                mature_or_suggestive_themes: string_value(&params.mature_or_suggestive_themes),
                medical_or_treatment_information: string_value(&params.medical_or_treatment_information),
                profanity_or_crude_humor: string_value(&params.profanity_or_crude_humor),
                sexual_content_graphic_and_nudity: string_value(&params.sexual_content_graphic_and_nudity),
                sexual_content_or_nudity: string_value(&params.sexual_content_or_nudity),
                violence_cartoon_or_fantasy: string_value(&params.violence_cartoon_or_fantasy),
                violence_realistic: string_value(&params.violence_realistic),
                violence_realistic_prolonged_graphic_or_sadistic: string_value(
                    &params.violence_realistic_prolonged_graphic_or_sadistic,
                ),
                gambling: params.gambling,
                unrestricted_web_access: params.unrestricted_web_access,
                seventeen_plus: params.seventeen_plus,
                ..Default::default()
            },
        },
    };

    let resp = match c.update_age_rating_declaration(&params.declaration_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update age rating declaration: {e}"))),
    };

    Ok(data_result(&format!("Age rating declaration updated:\n{}", format_age_rating_declaration(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleListTerritoryAgeRatings`.
async fn handle_list_territory_age_ratings(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppInfoID string \"json:\\\"app_info_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_info_id: String => "app_info_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_info_id.is_empty() {
        return Ok(error_result("app_info_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &GoMap::nil(), &GoSlice::nil(), &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_territory_age_ratings(&params.app_info_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list territory age ratings: {e}"))),
    };

    Ok(list_result(&format_territory_age_ratings(&resp.data), &resp.data, &resp.links))
}

/// Ported from `formatTerritoryAgeRatings`.
fn format_territory_age_ratings(ratings: &[TerritoryAgeRating]) -> String {
    if ratings.is_empty() {
        return "No territory age ratings found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} territory age ratings:\n\n", ratings.len());

    for rating in ratings {
        let _ = writeln!(sb, "ID: {}", rating.id);
        if !rating.attributes.app_store_age_rating.is_empty() {
            let _ = writeln!(sb, "App Store Age Rating: {}", rating.attributes.app_store_age_rating);
        }
        sb.push_str("---\n");
    }

    sb
}

/// Ported from `stringValue`.
fn string_value(s: &Option<String>) -> String {
    match s {
        None => String::new(),
        Some(v) => v.clone(),
    }
}

/// Ported from `formatAgeRatingDeclaration`.
fn format_age_rating_declaration(decl: &AgeRatingDeclaration) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", decl.id);

    let attrs = &decl.attributes;
    if !attrs.alcohol_tobacco_or_drug_use_or_references.is_empty() {
        let _ = writeln!(sb, "Alcohol/Tobacco/Drugs: {}", attrs.alcohol_tobacco_or_drug_use_or_references);
    }
    if !attrs.contests.is_empty() {
        let _ = writeln!(sb, "Contests: {}", attrs.contests);
    }
    if !attrs.gambling_simulated.is_empty() {
        let _ = writeln!(sb, "Simulated Gambling: {}", attrs.gambling_simulated);
    }
    if attrs.gambling {
        sb.push_str("Contains Gambling: Yes\n");
    }
    if !attrs.horror_or_fear_themes.is_empty() {
        let _ = writeln!(sb, "Horror/Fear: {}", attrs.horror_or_fear_themes);
    }
    if !attrs.mature_or_suggestive_themes.is_empty() {
        let _ = writeln!(sb, "Mature/Suggestive: {}", attrs.mature_or_suggestive_themes);
    }
    if !attrs.medical_or_treatment_information.is_empty() {
        let _ = writeln!(sb, "Medical Info: {}", attrs.medical_or_treatment_information);
    }
    if !attrs.profanity_or_crude_humor.is_empty() {
        let _ = writeln!(sb, "Profanity/Crude Humor: {}", attrs.profanity_or_crude_humor);
    }
    if !attrs.sexual_content_graphic_and_nudity.is_empty() {
        let _ = writeln!(sb, "Sexual Content (Graphic): {}", attrs.sexual_content_graphic_and_nudity);
    }
    if !attrs.sexual_content_or_nudity.is_empty() {
        let _ = writeln!(sb, "Sexual Content/Nudity: {}", attrs.sexual_content_or_nudity);
    }
    if !attrs.violence_cartoon_or_fantasy.is_empty() {
        let _ = writeln!(sb, "Violence (Cartoon): {}", attrs.violence_cartoon_or_fantasy);
    }
    if !attrs.violence_realistic.is_empty() {
        let _ = writeln!(sb, "Violence (Realistic): {}", attrs.violence_realistic);
    }
    if !attrs.violence_realistic_prolonged_graphic_or_sadistic.is_empty() {
        let _ = writeln!(sb, "Violence (Prolonged/Graphic): {}", attrs.violence_realistic_prolonged_graphic_or_sadistic);
    }
    if attrs.unrestricted_web_access {
        sb.push_str("Unrestricted Web Access: Yes\n");
    }
    if attrs.seventeen_plus {
        sb.push_str("17+ Age Rating: Yes\n");
    }

    sb
}
