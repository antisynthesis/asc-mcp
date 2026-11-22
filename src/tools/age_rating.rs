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
    todo!("port handleGetAgeRatingDeclaration from internal/asc/tools/age_rating.go")
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
    todo!("port handleUpdateAgeRatingDeclaration from internal/asc/tools/age_rating.go")
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
    todo!("port handleListTerritoryAgeRatings from internal/asc/tools/age_rating.go")
}

