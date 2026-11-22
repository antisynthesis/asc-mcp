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
    todo!("port handleListBetaAppReviewSubmissions from internal/asc/tools/beta_review.go")
}

/// Ported from `Registry.handleGetBetaAppReviewSubmission`.
async fn handle_get_beta_app_review_submission(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubmissionID string \"json:\\\"submission_id\\\"\" }", "") {
        submission_id: String => "submission_id";
    }
    }
    todo!("port handleGetBetaAppReviewSubmission from internal/asc/tools/beta_review.go")
}

/// Ported from `Registry.handleCreateBetaAppReviewSubmission`.
async fn handle_create_beta_app_review_submission(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildID string \"json:\\\"build_id\\\"\" }", "") {
        build_id: String => "build_id";
    }
    }
    todo!("port handleCreateBetaAppReviewSubmission from internal/asc/tools/beta_review.go")
}

/// Ported from `Registry.handleGetBetaLicenseAgreement`.
async fn handle_get_beta_license_agreement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\" }", "") {
        app_id: String => "app_id";
    }
    }
    todo!("port handleGetBetaLicenseAgreement from internal/asc/tools/beta_review.go")
}

/// Ported from `Registry.handleUpdateBetaLicenseAgreement`.
async fn handle_update_beta_license_agreement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AgreementID string \"json:\\\"agreement_id\\\"\"; AgreementText string \"json:\\\"agreement_text\\\"\" }", "") {
        agreement_id: String => "agreement_id";
        agreement_text: String => "agreement_text";
    }
    }
    todo!("port handleUpdateBetaLicenseAgreement from internal/asc/tools/beta_review.go")
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
    todo!("port handleListBetaAppLocalizations from internal/asc/tools/beta_review.go")
}

/// Ported from `Registry.handleGetBetaAppLocalization`.
async fn handle_get_beta_app_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    todo!("port handleGetBetaAppLocalization from internal/asc/tools/beta_review.go")
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
    todo!("port handleCreateBetaAppLocalization from internal/asc/tools/beta_review.go")
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
    todo!("port handleUpdateBetaAppLocalization from internal/asc/tools/beta_review.go")
}

/// Ported from `Registry.handleDeleteBetaAppLocalization`.
async fn handle_delete_beta_app_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    todo!("port handleDeleteBetaAppLocalization from internal/asc/tools/beta_review.go")
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
    todo!("port handleListBetaBuildLocalizations from internal/asc/tools/beta_review.go")
}

/// Ported from `Registry.handleGetBetaBuildLocalization`.
async fn handle_get_beta_build_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    todo!("port handleGetBetaBuildLocalization from internal/asc/tools/beta_review.go")
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
    todo!("port handleCreateBetaBuildLocalization from internal/asc/tools/beta_review.go")
}

/// Ported from `Registry.handleUpdateBetaBuildLocalization`.
async fn handle_update_beta_build_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; WhatsNew string \"json:\\\"whats_new\\\"\" }", "") {
        localization_id: String => "localization_id";
        whats_new: String => "whats_new";
    }
    }
    todo!("port handleUpdateBetaBuildLocalization from internal/asc/tools/beta_review.go")
}

/// Ported from `Registry.handleDeleteBetaBuildLocalization`.
async fn handle_delete_beta_build_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    todo!("port handleDeleteBetaBuildLocalization from internal/asc/tools/beta_review.go")
}

/// Ported from `Registry.handleGetBuildBetaDetail`.
async fn handle_get_build_beta_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildID string \"json:\\\"build_id\\\"\" }", "") {
        build_id: String => "build_id";
    }
    }
    todo!("port handleGetBuildBetaDetail from internal/asc/tools/beta_review.go")
}

/// Ported from `Registry.handleUpdateBuildBetaDetail`.
async fn handle_update_build_beta_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { DetailID string \"json:\\\"detail_id\\\"\"; AutoNotifyEnabled *bool \"json:\\\"auto_notify_enabled\\\"\" }", "") {
        detail_id: String => "detail_id";
        auto_notify_enabled: Option<bool> => "auto_notify_enabled";
    }
    }
    todo!("port handleUpdateBuildBetaDetail from internal/asc/tools/beta_review.go")
}

