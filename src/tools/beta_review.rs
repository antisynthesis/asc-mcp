//! Ported from internal/asc/tools/beta_review.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerBetaReviewTools`.
pub(super) fn register_beta_review_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_beta_app_review_submissions",
            "description": "List beta app review submissions",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"platform\": [\"IOS\"]} becomes filter[platform]=IOS."
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response.",
                        "items": {
                            "type": "string"
                        }
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of submissions to return (default 50)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    }
                }
            },
            "annotations": {
                "title": "List Beta App Review Submissions",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_beta_app_review_submissions),
    );
    r.register(
        tool(r##"{
            "name": "get_beta_app_review_submission",
            "description": "Get details of a beta app review submission",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "submission_id": {
                        "type": "string",
                        "description": "The beta app review submission ID"
                    }
                },
                "required": [
                    "submission_id"
                ]
            },
            "annotations": {
                "title": "Get Beta App Review Submission",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_beta_app_review_submission),
    );
    r.register(
        tool(r##"{
            "name": "create_beta_app_review_submission",
            "description": "Submit a build for beta app review",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "build_id": {
                        "type": "string",
                        "description": "The build ID to submit for review"
                    }
                },
                "required": [
                    "build_id"
                ]
            },
            "annotations": {
                "title": "Create Beta App Review Submission",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_beta_app_review_submission),
    );
    r.register(
        tool(r##"{
            "name": "get_beta_license_agreement",
            "description": "Get the beta license agreement for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The app ID"
                    }
                },
                "required": [
                    "app_id"
                ]
            },
            "annotations": {
                "title": "Get Beta License Agreement",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_beta_license_agreement),
    );
    r.register(
        tool(r##"{
            "name": "update_beta_license_agreement",
            "description": "Update the beta license agreement for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "agreement_id": {
                        "type": "string",
                        "description": "The beta license agreement ID"
                    },
                    "agreement_text": {
                        "type": "string",
                        "description": "The new agreement text"
                    }
                },
                "required": [
                    "agreement_id",
                    "agreement_text"
                ]
            },
            "annotations": {
                "title": "Update Beta License Agreement",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_beta_license_agreement),
    );
    r.register(
        tool(r##"{
            "name": "list_beta_app_localizations",
            "description": "List beta app localizations for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The app ID"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"platform\": [\"IOS\"]} becomes filter[platform]=IOS."
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response.",
                        "items": {
                            "type": "string"
                        }
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of localizations to return (default 50)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "app_id"
                ]
            },
            "annotations": {
                "title": "List Beta App Localizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_beta_app_localizations),
    );
    r.register(
        tool(r##"{
            "name": "get_beta_app_localization",
            "description": "Get a specific beta app localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The beta app localization ID"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Get Beta App Localization",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_beta_app_localization),
    );
    r.register(
        tool(r##"{
            "name": "create_beta_app_localization",
            "description": "Create a beta app localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The app ID"
                    },
                    "description": {
                        "type": "string",
                        "description": "Description for beta testers"
                    },
                    "feedback_email": {
                        "type": "string",
                        "description": "Email address for beta feedback"
                    },
                    "locale": {
                        "type": "string",
                        "description": "Locale code (e.g., en-US, de-DE)"
                    },
                    "marketing_url": {
                        "type": "string",
                        "description": "Marketing URL"
                    },
                    "privacy_policy_url": {
                        "type": "string",
                        "description": "Privacy policy URL"
                    }
                },
                "required": [
                    "app_id",
                    "locale"
                ]
            },
            "annotations": {
                "title": "Create Beta App Localization",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_beta_app_localization),
    );
    r.register(
        tool(r##"{
            "name": "update_beta_app_localization",
            "description": "Update a beta app localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "description": {
                        "type": "string",
                        "description": "Description for beta testers"
                    },
                    "feedback_email": {
                        "type": "string",
                        "description": "Email address for beta feedback"
                    },
                    "localization_id": {
                        "type": "string",
                        "description": "The beta app localization ID"
                    },
                    "marketing_url": {
                        "type": "string",
                        "description": "Marketing URL"
                    },
                    "privacy_policy_url": {
                        "type": "string",
                        "description": "Privacy policy URL"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Update Beta App Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_beta_app_localization),
    );
    r.register(
        tool(r##"{
            "name": "delete_beta_app_localization",
            "description": "Delete a beta app localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The beta app localization ID to delete"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Delete Beta App Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_beta_app_localization),
    );
    r.register(
        tool(r##"{
            "name": "list_beta_build_localizations",
            "description": "List beta build localizations for a build",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "build_id": {
                        "type": "string",
                        "description": "The build ID"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"platform\": [\"IOS\"]} becomes filter[platform]=IOS."
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response.",
                        "items": {
                            "type": "string"
                        }
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of localizations to return (default 50)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "build_id"
                ]
            },
            "annotations": {
                "title": "List Beta Build Localizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_beta_build_localizations),
    );
    r.register(
        tool(r##"{
            "name": "get_beta_build_localization",
            "description": "Get a specific beta build localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The beta build localization ID"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Get Beta Build Localization",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_beta_build_localization),
    );
    r.register(
        tool(r##"{
            "name": "create_beta_build_localization",
            "description": "Create a beta build localization (what's new)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "build_id": {
                        "type": "string",
                        "description": "The build ID"
                    },
                    "locale": {
                        "type": "string",
                        "description": "Locale code (e.g., en-US, de-DE)"
                    },
                    "whats_new": {
                        "type": "string",
                        "description": "What's new in this build for beta testers"
                    }
                },
                "required": [
                    "build_id",
                    "locale"
                ]
            },
            "annotations": {
                "title": "Create Beta Build Localization",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_beta_build_localization),
    );
    r.register(
        tool(r##"{
            "name": "update_beta_build_localization",
            "description": "Update a beta build localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The beta build localization ID"
                    },
                    "whats_new": {
                        "type": "string",
                        "description": "What's new in this build for beta testers"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Update Beta Build Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_beta_build_localization),
    );
    r.register(
        tool(r##"{
            "name": "delete_beta_build_localization",
            "description": "Delete a beta build localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The beta build localization ID to delete"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Delete Beta Build Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_beta_build_localization),
    );
    r.register(
        tool(r##"{
            "name": "get_build_beta_detail",
            "description": "Get build beta testing details",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "build_id": {
                        "type": "string",
                        "description": "The build ID"
                    }
                },
                "required": [
                    "build_id"
                ]
            },
            "annotations": {
                "title": "Get Build Beta Detail",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_build_beta_detail),
    );
    r.register(
        tool(r##"{
            "name": "update_build_beta_detail",
            "description": "Update build beta testing details",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "auto_notify_enabled": {
                        "type": "boolean",
                        "description": "Whether to auto-notify testers"
                    },
                    "detail_id": {
                        "type": "string",
                        "description": "The build beta detail ID"
                    }
                },
                "required": [
                    "detail_id"
                ]
            },
            "annotations": {
                "title": "Update Build Beta Detail",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_build_beta_detail),
    );
}

/// Ported from `Registry.handleListBetaAppReviewSubmissions`.
async fn handle_list_beta_app_review_submissions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_beta_app_review_submissions(opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list beta app review submissions: {e}"))),
    };

    Ok(list_result(&format_beta_app_review_submissions(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetBetaAppReviewSubmission`.
async fn handle_get_beta_app_review_submission(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubmissionID string \"json:\\\"submission_id\\\"\" }", "") {
        submission_id: String => "submission_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.submission_id.is_empty() {
        return Ok(error_result("submission_id is required"));
    }

    let resp = match c.get_beta_app_review_submission(&params.submission_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get beta app review submission: {e}"))),
    };

    Ok(data_result(&format_beta_app_review_submission(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateBetaAppReviewSubmission`.
async fn handle_create_beta_app_review_submission(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildID string \"json:\\\"build_id\\\"\" }", "") {
        build_id: String => "build_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.build_id.is_empty() {
        return Ok(error_result("build_id is required"));
    }

    let req = BetaAppReviewSubmissionCreateRequest {
        data: BetaAppReviewSubmissionCreateData {
            type_: "betaAppReviewSubmissions".into(),
            relationships: BetaAppReviewSubmissionCreateRelationships {
                build: RelationshipData {
                    data: ResourceIdentifier { type_: "builds".into(), id: params.build_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_beta_app_review_submission(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create beta app review submission: {e}"))),
    };

    Ok(data_result(
        &format!("Beta app review submission created:\n{}", format_beta_app_review_submission(&resp.data)),
        &resp.data,
    ))
}

/// Ported from `Registry.handleGetBetaLicenseAgreement`.
async fn handle_get_beta_license_agreement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\" }", "") {
        app_id: String => "app_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }

    let resp = match c.get_beta_license_agreement(&params.app_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get beta license agreement: {e}"))),
    };

    Ok(data_result(&format_beta_license_agreement(&resp.data), &resp.data))
}

/// Ported from `Registry.handleUpdateBetaLicenseAgreement`.
async fn handle_update_beta_license_agreement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AgreementID string \"json:\\\"agreement_id\\\"\"; AgreementText string \"json:\\\"agreement_text\\\"\" }", "") {
        agreement_id: String => "agreement_id";
        agreement_text: String => "agreement_text";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.agreement_id.is_empty() || params.agreement_text.is_empty() {
        return Ok(error_result("agreement_id and agreement_text are required"));
    }

    let req = BetaLicenseAgreementUpdateRequest {
        data: BetaLicenseAgreementUpdateData {
            type_: "betaLicenseAgreements".into(),
            id: params.agreement_id.clone(),
            attributes: BetaLicenseAgreementUpdateAttributes { agreement_text: params.agreement_text.clone() },
        },
    };

    let resp = match c.update_beta_license_agreement(&params.agreement_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update beta license agreement: {e}"))),
    };

    Ok(data_result(&format!("Beta license agreement updated:\n{}", format_beta_license_agreement(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleListBetaAppLocalizations`.
async fn handle_list_beta_app_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_beta_app_localizations(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list beta app localizations: {e}"))),
    };

    Ok(list_result(&format_beta_app_localizations(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetBetaAppLocalization`.
async fn handle_get_beta_app_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }

    let resp = match c.get_beta_app_localization(&params.localization_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get beta app localization: {e}"))),
    };

    Ok(data_result(&format_beta_app_localization(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateBetaAppLocalization`.
async fn handle_create_beta_app_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Locale string \"json:\\\"locale\\\"\"; Description string \"json:\\\"description\\\"\"; FeedbackEmail string \"json:\\\"feedback_email\\\"\"; MarketingURL string \"json:\\\"marketing_url\\\"\"; PrivacyPolicyURL string \"json:\\\"privacy_policy_url\\\"\" }", "") {
        app_id: String => "app_id";
        locale: String => "locale";
        description: String => "description";
        feedback_email: String => "feedback_email";
        marketing_url: String => "marketing_url";
        privacy_policy_url: String => "privacy_policy_url";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() || params.locale.is_empty() {
        return Ok(error_result("app_id and locale are required"));
    }

    let req = BetaAppLocalizationCreateRequest {
        data: BetaAppLocalizationCreateData {
            type_: "betaAppLocalizations".into(),
            attributes: BetaAppLocalizationCreateAttributes {
                locale: params.locale.clone(),
                description: params.description.clone(),
                feedback_email: params.feedback_email.clone(),
                marketing_url: params.marketing_url.clone(),
                privacy_policy_url: params.privacy_policy_url.clone(),
                ..Default::default()
            },
            relationships: BetaAppLocalizationCreateRelationships {
                app: RelationshipData {
                    data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_beta_app_localization(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create beta app localization: {e}"))),
    };

    Ok(data_result(&format!("Beta app localization created:\n{}", format_beta_app_localization(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleUpdateBetaAppLocalization`.
async fn handle_update_beta_app_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Description string \"json:\\\"description\\\"\"; FeedbackEmail string \"json:\\\"feedback_email\\\"\"; MarketingURL string \"json:\\\"marketing_url\\\"\"; PrivacyPolicyURL string \"json:\\\"privacy_policy_url\\\"\" }", "") {
        localization_id: String => "localization_id";
        description: String => "description";
        feedback_email: String => "feedback_email";
        marketing_url: String => "marketing_url";
        privacy_policy_url: String => "privacy_policy_url";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }

    let req = BetaAppLocalizationUpdateRequest {
        data: BetaAppLocalizationUpdateData {
            type_: "betaAppLocalizations".into(),
            id: params.localization_id.clone(),
            attributes: BetaAppLocalizationUpdateAttributes {
                description: params.description.clone(),
                feedback_email: params.feedback_email.clone(),
                marketing_url: params.marketing_url.clone(),
                privacy_policy_url: params.privacy_policy_url.clone(),
                ..Default::default()
            },
        },
    };

    let resp = match c.update_beta_app_localization(&params.localization_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update beta app localization: {e}"))),
    };

    Ok(data_result(&format!("Beta app localization updated:\n{}", format_beta_app_localization(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleDeleteBetaAppLocalization`.
async fn handle_delete_beta_app_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }

    if let Err(e) = c.delete_beta_app_localization(&params.localization_id).await {
        return Ok(error_result(format!("Failed to delete beta app localization: {e}")));
    }

    Ok(CallToolResult::success("Beta app localization deleted"))
}

/// Ported from `Registry.handleListBetaBuildLocalizations`.
async fn handle_list_beta_build_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildID string \"json:\\\"build_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        build_id: String => "build_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.build_id.is_empty() {
        return Ok(error_result("build_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_beta_build_localizations(&params.build_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list beta build localizations: {e}"))),
    };

    Ok(list_result(&format_beta_build_localizations(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetBetaBuildLocalization`.
async fn handle_get_beta_build_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }

    let resp = match c.get_beta_build_localization(&params.localization_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get beta build localization: {e}"))),
    };

    Ok(data_result(&format_beta_build_localization(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateBetaBuildLocalization`.
async fn handle_create_beta_build_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildID string \"json:\\\"build_id\\\"\"; Locale string \"json:\\\"locale\\\"\"; WhatsNew string \"json:\\\"whats_new\\\"\" }", "") {
        build_id: String => "build_id";
        locale: String => "locale";
        whats_new: String => "whats_new";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.build_id.is_empty() || params.locale.is_empty() {
        return Ok(error_result("build_id and locale are required"));
    }

    let req = BetaBuildLocalizationCreateRequest {
        data: BetaBuildLocalizationCreateData {
            type_: "betaBuildLocalizations".into(),
            attributes: BetaBuildLocalizationCreateAttributes {
                locale: params.locale.clone(),
                whats_new: params.whats_new.clone(),
            },
            relationships: BetaBuildLocalizationCreateRelationships {
                build: RelationshipData {
                    data: ResourceIdentifier { type_: "builds".into(), id: params.build_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_beta_build_localization(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create beta build localization: {e}"))),
    };

    Ok(data_result(&format!("Beta build localization created:\n{}", format_beta_build_localization(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleUpdateBetaBuildLocalization`.
async fn handle_update_beta_build_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; WhatsNew string \"json:\\\"whats_new\\\"\" }", "") {
        localization_id: String => "localization_id";
        whats_new: String => "whats_new";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }

    let req = BetaBuildLocalizationUpdateRequest {
        data: BetaBuildLocalizationUpdateData {
            type_: "betaBuildLocalizations".into(),
            id: params.localization_id.clone(),
            attributes: BetaBuildLocalizationUpdateAttributes { whats_new: params.whats_new.clone() },
        },
    };

    let resp = match c.update_beta_build_localization(&params.localization_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update beta build localization: {e}"))),
    };

    Ok(data_result(&format!("Beta build localization updated:\n{}", format_beta_build_localization(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleDeleteBetaBuildLocalization`.
async fn handle_delete_beta_build_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }

    if let Err(e) = c.delete_beta_build_localization(&params.localization_id).await {
        return Ok(error_result(format!("Failed to delete beta build localization: {e}")));
    }

    Ok(CallToolResult::success("Beta build localization deleted"))
}

/// Ported from `Registry.handleGetBuildBetaDetail`.
async fn handle_get_build_beta_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildID string \"json:\\\"build_id\\\"\" }", "") {
        build_id: String => "build_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.build_id.is_empty() {
        return Ok(error_result("build_id is required"));
    }

    let resp = match c.get_build_beta_detail(&params.build_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get build beta detail: {e}"))),
    };

    Ok(data_result(&format_build_beta_detail(&resp.data), &resp.data))
}

/// Ported from `Registry.handleUpdateBuildBetaDetail`.
async fn handle_update_build_beta_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { DetailID string \"json:\\\"detail_id\\\"\"; AutoNotifyEnabled *bool \"json:\\\"auto_notify_enabled\\\"\" }", "") {
        detail_id: String => "detail_id";
        auto_notify_enabled: Option<bool> => "auto_notify_enabled";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.detail_id.is_empty() {
        return Ok(error_result("detail_id is required"));
    }

    let req = BuildBetaDetailUpdateRequest {
        data: BuildBetaDetailUpdateData {
            type_: "buildBetaDetails".into(),
            id: params.detail_id.clone(),
            attributes: BuildBetaDetailUpdateAttributes { auto_notify_enabled: params.auto_notify_enabled },
        },
    };

    let resp = match c.update_build_beta_detail(&params.detail_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update build beta detail: {e}"))),
    };

    Ok(data_result(&format!("Build beta detail updated:\n{}", format_build_beta_detail(&resp.data)), &resp.data))
}

/// Ported from `formatBetaAppReviewSubmissions`.
fn format_beta_app_review_submissions(submissions: &[BetaAppReviewSubmission]) -> String {
    if submissions.is_empty() {
        return "No beta app review submissions found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} beta app review submissions:\n\n", submissions.len());
    for sub in submissions {
        sb.push_str(&format_beta_app_review_submission(sub));
        sb.push_str("\n---\n");
    }
    sb
}

/// Ported from `formatBetaAppReviewSubmission`.
fn format_beta_app_review_submission(sub: &BetaAppReviewSubmission) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", sub.id);
    let _ = writeln!(sb, "State: {}", sub.attributes.beta_review_state);
    if let Some(d) = &sub.attributes.submitted_date {
        let _ = writeln!(sb, "Submitted: {}", d.format("2006-01-02 15:04"));
    }
    sb
}

/// Go `s[:n]` on a valid UTF-8 string, rendered the way Go's JSON encoder
/// emits it: a rune split by the cut leaves invalid trailing bytes, each of
/// which becomes U+FFFD.
fn go_byte_prefix(s: &str, n: usize) -> String {
    if s.len() <= n {
        return s.to_string();
    }
    let mut b = n;
    while !s.is_char_boundary(b) {
        b -= 1;
    }
    let mut out = s[..b].to_string();
    for _ in b..n {
        out.push('\u{FFFD}');
    }
    out
}

/// Ported from `formatBetaLicenseAgreement`.
fn format_beta_license_agreement(agreement: &BetaLicenseAgreement) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", agreement.id);
    if !agreement.attributes.agreement_text.is_empty() {
        let mut text = agreement.attributes.agreement_text.clone();
        if text.len() > 500 {
            text = go_byte_prefix(&text, 500) + "...";
        }
        let _ = write!(sb, "Agreement Text:\n{}\n", text);
    } else {
        sb.push_str("Agreement Text: (empty)\n");
    }
    sb
}

/// Ported from `formatBetaAppLocalizations`.
fn format_beta_app_localizations(localizations: &[BetaAppLocalization]) -> String {
    if localizations.is_empty() {
        return "No beta app localizations found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} beta app localizations:\n\n", localizations.len());
    for loc in localizations {
        sb.push_str(&format_beta_app_localization(loc));
        sb.push_str("\n---\n");
    }
    sb
}

/// Ported from `formatBetaAppLocalization`.
fn format_beta_app_localization(loc: &BetaAppLocalization) -> String {
    let a = &loc.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", loc.id);
    let _ = writeln!(sb, "Locale: {}", a.locale);
    if !a.description.is_empty() {
        let _ = writeln!(sb, "Description: {}", a.description);
    }
    if !a.feedback_email.is_empty() {
        let _ = writeln!(sb, "Feedback Email: {}", a.feedback_email);
    }
    if !a.marketing_url.is_empty() {
        let _ = writeln!(sb, "Marketing URL: {}", a.marketing_url);
    }
    if !a.privacy_policy_url.is_empty() {
        let _ = writeln!(sb, "Privacy Policy URL: {}", a.privacy_policy_url);
    }
    sb
}

/// Ported from `formatBetaBuildLocalizations`.
fn format_beta_build_localizations(localizations: &[BetaBuildLocalization]) -> String {
    if localizations.is_empty() {
        return "No beta build localizations found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} beta build localizations:\n\n", localizations.len());
    for loc in localizations {
        sb.push_str(&format_beta_build_localization(loc));
        sb.push_str("\n---\n");
    }
    sb
}

/// Ported from `formatBetaBuildLocalization`.
fn format_beta_build_localization(loc: &BetaBuildLocalization) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", loc.id);
    let _ = writeln!(sb, "Locale: {}", loc.attributes.locale);
    if !loc.attributes.whats_new.is_empty() {
        let _ = writeln!(sb, "What's New: {}", loc.attributes.whats_new);
    }
    sb
}

/// Ported from `formatBuildBetaDetail`.
fn format_build_beta_detail(detail: &BuildBetaDetail) -> String {
    let a = &detail.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", detail.id);
    let _ = writeln!(sb, "Auto Notify Enabled: {}", a.auto_notify_enabled);
    let _ = writeln!(sb, "Internal Build State: {}", a.internal_build_state);
    let _ = writeln!(sb, "External Build State: {}", a.external_build_state);
    sb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn go_byte_prefix_splits_runes_like_go() {
        assert_eq!(go_byte_prefix("abc", 5), "abc");
        // "é" is 2 bytes; cutting after 1 byte leaves one invalid byte.
        assert_eq!(go_byte_prefix("aé", 2), "a\u{FFFD}");
        // "€" is 3 bytes; cutting after 2 leaves two invalid bytes.
        assert_eq!(go_byte_prefix("€x", 2), "\u{FFFD}\u{FFFD}");
        assert_eq!(go_byte_prefix("€x", 3), "€");
    }
}
