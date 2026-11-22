//! Ported from internal/asc/tools/versions.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerVersionSubmissionTools`.
pub(super) fn register_version_submission_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_app_store_versions",
            "description": "List App Store versions for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID"
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
                        "description": "Maximum number of versions to return (default 50)"
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
                "title": "List App Store Versions",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_app_store_versions),
    );
    r.register(
        tool(r##"{
            "name": "get_app_store_version",
            "description": "Get details of a specific App Store version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "version_id": {
                        "type": "string",
                        "description": "The App Store version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Get App Store Version",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_store_version),
    );
    r.register(
        tool(r##"{
            "name": "create_app_store_version",
            "description": "Create a new App Store version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID"
                    },
                    "copyright": {
                        "type": "string",
                        "description": "The copyright text"
                    },
                    "platform": {
                        "type": "string",
                        "description": "The platform (IOS, MAC_OS, TV_OS, VISION_OS)"
                    },
                    "release_type": {
                        "type": "string",
                        "description": "Release type (MANUAL, AFTER_APPROVAL, SCHEDULED)"
                    },
                    "version_string": {
                        "type": "string",
                        "description": "The version string (e.g., 1.0.0)"
                    }
                },
                "required": [
                    "app_id",
                    "version_string",
                    "platform"
                ]
            },
            "annotations": {
                "title": "Create App Store Version",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_app_store_version),
    );
    r.register(
        tool(r##"{
            "name": "update_app_store_version",
            "description": "Update an App Store version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "copyright": {
                        "type": "string",
                        "description": "The updated copyright text"
                    },
                    "release_type": {
                        "type": "string",
                        "description": "Release type (MANUAL, AFTER_APPROVAL, SCHEDULED)"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The App Store version ID"
                    },
                    "version_string": {
                        "type": "string",
                        "description": "The updated version string"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Update App Store Version",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_app_store_version),
    );
    r.register(
        tool(r##"{
            "name": "delete_app_store_version",
            "description": "Delete an App Store version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "version_id": {
                        "type": "string",
                        "description": "The App Store version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Delete App Store Version",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_app_store_version),
    );
    r.register(
        tool(r##"{
            "name": "submit_app_for_review",
            "description": "Submit an App Store version for review. Creates a review submission for the app, attaches the version, and submits it.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The app ID"
                    },
                    "platform": {
                        "type": "string",
                        "description": "Platform: IOS, MAC_OS, TV_OS, VISION_OS (default IOS)"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The App Store version ID to submit"
                    }
                },
                "required": [
                    "app_id",
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Submit App For Review",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_submit_app_for_review),
    );
    r.register(
        tool(r##"{
            "name": "get_app_store_review_detail",
            "description": "Get App Store review details for a version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "version_id": {
                        "type": "string",
                        "description": "The App Store version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Get App Store Review Detail",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_store_review_detail),
    );
    r.register(
        tool(r##"{
            "name": "create_app_store_review_detail",
            "description": "Create App Store review details for a version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "contact_email": {
                        "type": "string",
                        "description": "Contact email"
                    },
                    "contact_first_name": {
                        "type": "string",
                        "description": "Contact first name"
                    },
                    "contact_last_name": {
                        "type": "string",
                        "description": "Contact last name"
                    },
                    "contact_phone": {
                        "type": "string",
                        "description": "Contact phone number"
                    },
                    "demo_account_name": {
                        "type": "string",
                        "description": "Demo account username"
                    },
                    "demo_account_password": {
                        "type": "string",
                        "description": "Demo account password"
                    },
                    "demo_account_required": {
                        "type": "boolean",
                        "description": "Whether demo account is required"
                    },
                    "notes": {
                        "type": "string",
                        "description": "Notes for the reviewer"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The App Store version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Create App Store Review Detail",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_app_store_review_detail),
    );
    r.register(
        tool(r##"{
            "name": "update_app_store_review_detail",
            "description": "Update App Store review details",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "contact_email": {
                        "type": "string",
                        "description": "Contact email"
                    },
                    "contact_first_name": {
                        "type": "string",
                        "description": "Contact first name"
                    },
                    "contact_last_name": {
                        "type": "string",
                        "description": "Contact last name"
                    },
                    "contact_phone": {
                        "type": "string",
                        "description": "Contact phone number"
                    },
                    "demo_account_name": {
                        "type": "string",
                        "description": "Demo account username"
                    },
                    "demo_account_password": {
                        "type": "string",
                        "description": "Demo account password"
                    },
                    "demo_account_required": {
                        "type": "boolean",
                        "description": "Whether demo account is required"
                    },
                    "detail_id": {
                        "type": "string",
                        "description": "The review detail ID"
                    },
                    "notes": {
                        "type": "string",
                        "description": "Notes for the reviewer"
                    }
                },
                "required": [
                    "detail_id"
                ]
            },
            "annotations": {
                "title": "Update App Store Review Detail",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_app_store_review_detail),
    );
}

/// Ported from `Registry.handleListAppStoreVersions`.
async fn handle_list_app_store_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
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

    let resp = match c.get_app_versions(&params.app_id, limit_opts(limit).as_ref()).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list app store versions: {e}"))),
    };

    Ok(data_result(&format_app_store_versions(&resp.data), &resp.data))
}

/// Ported from `Registry.handleGetAppStoreVersion`.
async fn handle_get_app_store_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    let resp = match c.get_app_store_version(&params.version_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get app store version: {e}"))),
    };

    Ok(data_result(&format_app_store_version(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateAppStoreVersion`.
async fn handle_create_app_store_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; VersionString string \"json:\\\"version_string\\\"\"; Platform string \"json:\\\"platform\\\"\"; Copyright string \"json:\\\"copyright\\\"\"; ReleaseType string \"json:\\\"release_type\\\"\" }", "") {
        app_id: String => "app_id";
        version_string: String => "version_string";
        platform: String => "platform";
        copyright: String => "copyright";
        release_type: String => "release_type";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }
    if params.version_string.is_empty() {
        return Ok(error_result("version_string is required"));
    }
    if params.platform.is_empty() {
        return Ok(error_result("platform is required"));
    }

    let req = AppStoreVersionCreateRequest {
        data: AppStoreVersionCreateData {
            type_: "appStoreVersions".into(),
            attributes: AppStoreVersionCreateAttributes {
                platform: params.platform.clone(),
                version_string: params.version_string.clone(),
                copyright: params.copyright.clone(),
                release_type: params.release_type.clone(),
                ..Default::default()
            },
            relationships: AppStoreVersionCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
                ..Default::default()
            },
        },
    };

    let resp = match c.create_app_store_version(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create app store version: {e}"))),
    };

    Ok(data_result(
        &format!("Created app store version: {} (ID: {})", resp.data.attributes.version_string, resp.data.id),
        &resp.data,
    ))
}

/// Ported from `Registry.handleUpdateAppStoreVersion`.
async fn handle_update_app_store_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; VersionString string \"json:\\\"version_string\\\"\"; Copyright string \"json:\\\"copyright\\\"\"; ReleaseType string \"json:\\\"release_type\\\"\" }", "") {
        version_id: String => "version_id";
        version_string: String => "version_string";
        copyright: String => "copyright";
        release_type: String => "release_type";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    let req = AppStoreVersionUpdateRequest {
        data: AppStoreVersionUpdateData {
            type_: "appStoreVersions".into(),
            id: params.version_id.clone(),
            attributes: AppStoreVersionUpdateAttributes {
                version_string: params.version_string.clone(),
                copyright: params.copyright.clone(),
                release_type: params.release_type.clone(),
                ..Default::default()
            },
        },
    };

    let resp = match c.update_app_store_version(&params.version_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update app store version: {e}"))),
    };

    Ok(data_result(&format!("Updated app store version: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleDeleteAppStoreVersion`.
async fn handle_delete_app_store_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    if let Err(e) = c.delete_app_store_version(&params.version_id).await {
        return Ok(error_result(format!("Failed to delete app store version: {e}")));
    }

    Ok(CallToolResult::success("App store version deleted successfully"))
}

/// Ported from `Registry.handleSubmitAppForReview`.
async fn handle_submit_app_for_review(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; VersionID string \"json:\\\"version_id\\\"\"; Platform string \"json:\\\"platform\\\"\" }", "") {
        app_id: String => "app_id";
        version_id: String => "version_id";
        platform: String => "platform";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }
    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    let platform = if params.platform.is_empty() { "IOS".to_string() } else { params.platform.clone() };

    // Create a review submission for the app.
    let create_req = ReviewSubmissionCreateRequest {
        data: ReviewSubmissionCreateData {
            type_: "reviewSubmissions".into(),
            attributes: Some(ReviewSubmissionCreateAttributes { platform }),
            relationships: ReviewSubmissionCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
            },
        },
    };

    let submission = match c.create_review_submission(&create_req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create review submission: {e}"))),
    };

    // Attach the app store version to the submission.
    let item_req = ReviewSubmissionItemCreateRequest {
        data: ReviewSubmissionItemCreateData {
            type_: "reviewSubmissionItems".into(),
            relationships: ReviewSubmissionItemCreateRelationships {
                review_submission: RelationshipData {
                    data: ResourceIdentifier { type_: "reviewSubmissions".into(), id: submission.data.id.clone() },
                },
                app_store_version: Some(RelationshipData {
                    data: ResourceIdentifier { type_: "appStoreVersions".into(), id: params.version_id.clone() },
                }),
                ..Default::default()
            },
        },
    };

    if let Err(e) = c.create_review_submission_item(&item_req).await {
        return Ok(error_result(format!(
            "Failed to attach version to review submission {}: {e}",
            submission.data.id
        )));
    }

    // Submit the review submission.
    let update_req = ReviewSubmissionUpdateRequest {
        data: ReviewSubmissionUpdateData {
            type_: "reviewSubmissions".into(),
            id: submission.data.id.clone(),
            attributes: ReviewSubmissionUpdateAttributes { submitted: Some(true), ..Default::default() },
        },
    };

    let resp = match c.update_review_submission(&submission.data.id, &update_req).await {
        Ok(r) => r,
        Err(e) => {
            return Ok(error_result(format!("Failed to submit review submission {}: {e}", submission.data.id)))
        }
    };

    Ok(data_result(&format!("App submitted for review (review submission ID: {})", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleGetAppStoreReviewDetail`.
async fn handle_get_app_store_review_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    let resp = match c.get_app_store_review_detail(&params.version_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get review detail: {e}"))),
    };

    Ok(data_result(&format_review_detail(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateAppStoreReviewDetail`.
async fn handle_create_app_store_review_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; ContactFirstName string \"json:\\\"contact_first_name\\\"\"; ContactLastName string \"json:\\\"contact_last_name\\\"\"; ContactPhone string \"json:\\\"contact_phone\\\"\"; ContactEmail string \"json:\\\"contact_email\\\"\"; DemoAccountName string \"json:\\\"demo_account_name\\\"\"; DemoAccountPassword string \"json:\\\"demo_account_password\\\"\"; DemoAccountRequired *bool \"json:\\\"demo_account_required\\\"\"; Notes string \"json:\\\"notes\\\"\" }", "") {
        version_id: String => "version_id";
        contact_first_name: String => "contact_first_name";
        contact_last_name: String => "contact_last_name";
        contact_phone: String => "contact_phone";
        contact_email: String => "contact_email";
        demo_account_name: String => "demo_account_name";
        demo_account_password: String => "demo_account_password";
        demo_account_required: Option<bool> => "demo_account_required";
        notes: String => "notes";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    let req = AppStoreReviewDetailCreateRequest {
        data: AppStoreReviewDetailCreateData {
            type_: "appStoreReviewDetails".into(),
            attributes: AppStoreReviewDetailCreateAttributes {
                contact_first_name: params.contact_first_name.clone(),
                contact_last_name: params.contact_last_name.clone(),
                contact_phone: params.contact_phone.clone(),
                contact_email: params.contact_email.clone(),
                demo_account_name: params.demo_account_name.clone(),
                demo_account_password: params.demo_account_password.clone(),
                demo_account_required: params.demo_account_required,
                notes: params.notes.clone(),
            },
            relationships: AppStoreReviewDetailCreateRelationships {
                app_store_version: RelationshipData {
                    data: ResourceIdentifier { type_: "appStoreVersions".into(), id: params.version_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_app_store_review_detail(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create review detail: {e}"))),
    };

    Ok(data_result(&format!("Created review detail: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleUpdateAppStoreReviewDetail`.
async fn handle_update_app_store_review_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { DetailID string \"json:\\\"detail_id\\\"\"; ContactFirstName string \"json:\\\"contact_first_name\\\"\"; ContactLastName string \"json:\\\"contact_last_name\\\"\"; ContactPhone string \"json:\\\"contact_phone\\\"\"; ContactEmail string \"json:\\\"contact_email\\\"\"; DemoAccountName string \"json:\\\"demo_account_name\\\"\"; DemoAccountPassword string \"json:\\\"demo_account_password\\\"\"; DemoAccountRequired *bool \"json:\\\"demo_account_required\\\"\"; Notes string \"json:\\\"notes\\\"\" }", "") {
        detail_id: String => "detail_id";
        contact_first_name: String => "contact_first_name";
        contact_last_name: String => "contact_last_name";
        contact_phone: String => "contact_phone";
        contact_email: String => "contact_email";
        demo_account_name: String => "demo_account_name";
        demo_account_password: String => "demo_account_password";
        demo_account_required: Option<bool> => "demo_account_required";
        notes: String => "notes";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.detail_id.is_empty() {
        return Ok(error_result("detail_id is required"));
    }

    let req = AppStoreReviewDetailUpdateRequest {
        data: AppStoreReviewDetailUpdateData {
            type_: "appStoreReviewDetails".into(),
            id: params.detail_id.clone(),
            attributes: AppStoreReviewDetailUpdateAttributes {
                contact_first_name: params.contact_first_name.clone(),
                contact_last_name: params.contact_last_name.clone(),
                contact_phone: params.contact_phone.clone(),
                contact_email: params.contact_email.clone(),
                demo_account_name: params.demo_account_name.clone(),
                demo_account_password: params.demo_account_password.clone(),
                demo_account_required: params.demo_account_required,
                notes: params.notes.clone(),
            },
        },
    };

    let resp = match c.update_app_store_review_detail(&params.detail_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update review detail: {e}"))),
    };

    Ok(data_result(&format!("Updated review detail: {}", resp.data.id), &resp.data))
}

/// Ported from `formatAppStoreVersions`.
fn format_app_store_versions(versions: &[AppStoreVersion]) -> String {
    if versions.is_empty() {
        return "No app store versions found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} app store versions:\n\n", versions.len());

    for version in versions {
        sb.push_str(&format_app_store_version(version));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatAppStoreVersion`.
fn format_app_store_version(version: &AppStoreVersion) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", version.id);
    let _ = writeln!(sb, "Version: {}", version.attributes.version_string);
    let _ = writeln!(sb, "Platform: {}", version.attributes.platform);
    let _ = writeln!(sb, "State: {}", version.attributes.app_store_state);
    if !version.attributes.copyright.is_empty() {
        let _ = writeln!(sb, "Copyright: {}", version.attributes.copyright);
    }
    if !version.attributes.release_type.is_empty() {
        let _ = writeln!(sb, "Release Type: {}", version.attributes.release_type);
    }
    if let Some(created) = &version.attributes.created_date {
        let _ = writeln!(sb, "Created: {}", created.format("2006-01-02"));
    }
    sb
}

/// Ported from `formatReviewDetail`.
fn format_review_detail(detail: &AppStoreReviewDetail) -> String {
    let a = &detail.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", detail.id);
    if !a.contact_first_name.is_empty() || !a.contact_last_name.is_empty() {
        let _ = writeln!(sb, "Contact: {} {}", a.contact_first_name, a.contact_last_name);
    }
    if !a.contact_email.is_empty() {
        let _ = writeln!(sb, "Email: {}", a.contact_email);
    }
    if !a.contact_phone.is_empty() {
        let _ = writeln!(sb, "Phone: {}", a.contact_phone);
    }
    let _ = writeln!(sb, "Demo Account Required: {}", bool_v(a.demo_account_required));
    if !a.demo_account_name.is_empty() {
        let _ = writeln!(sb, "Demo Account: {}", a.demo_account_name);
    }
    if !a.notes.is_empty() {
        let _ = writeln!(sb, "Notes: {}", a.notes);
    }
    sb
}
