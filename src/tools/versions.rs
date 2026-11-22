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
    todo!("port handleListAppStoreVersions from internal/asc/tools/versions.go")
}

/// Ported from `Registry.handleGetAppStoreVersion`.
async fn handle_get_app_store_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    todo!("port handleGetAppStoreVersion from internal/asc/tools/versions.go")
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
    todo!("port handleCreateAppStoreVersion from internal/asc/tools/versions.go")
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
    todo!("port handleUpdateAppStoreVersion from internal/asc/tools/versions.go")
}

/// Ported from `Registry.handleDeleteAppStoreVersion`.
async fn handle_delete_app_store_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    todo!("port handleDeleteAppStoreVersion from internal/asc/tools/versions.go")
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
    todo!("port handleSubmitAppForReview from internal/asc/tools/versions.go")
}

/// Ported from `Registry.handleGetAppStoreReviewDetail`.
async fn handle_get_app_store_review_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    todo!("port handleGetAppStoreReviewDetail from internal/asc/tools/versions.go")
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
    todo!("port handleCreateAppStoreReviewDetail from internal/asc/tools/versions.go")
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
    todo!("port handleUpdateAppStoreReviewDetail from internal/asc/tools/versions.go")
}

