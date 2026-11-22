//! Ported from internal/asc/tools/localizations.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerAppInfoLocalizationTools`.
pub(super) fn register_app_info_localization_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_app_info_localizations",
            "description": "List all localizations for an app's metadata (name, subtitle, privacy URLs). Requires the app_info_id which can be obtained from get_app_infos.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_info_id": {
                        "type": "string",
                        "description": "The app info ID"
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
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "app_info_id"
                ]
            },
            "annotations": {
                "title": "List App Info Localizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_app_info_localizations),
    );
    r.register(
        tool(r##"{
            "name": "get_app_info_localization",
            "description": "Get a specific app info localization by ID.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The app info localization ID"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Get App Info Localization",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_info_localization),
    );
    r.register(
        tool(r##"{
            "name": "create_app_info_localization",
            "description": "Create a new localization for app metadata. Use this to add support for a new language.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_info_id": {
                        "type": "string",
                        "description": "The app info ID"
                    },
                    "locale": {
                        "type": "string",
                        "description": "The locale code (e.g., en-US, de-DE, ja)"
                    },
                    "name": {
                        "type": "string",
                        "description": "The app name for this locale"
                    },
                    "privacy_choices_url": {
                        "type": "string",
                        "description": "Privacy choices URL (optional)"
                    },
                    "privacy_policy_text": {
                        "type": "string",
                        "description": "Privacy policy text (optional)"
                    },
                    "privacy_policy_url": {
                        "type": "string",
                        "description": "Privacy policy URL (optional)"
                    },
                    "subtitle": {
                        "type": "string",
                        "description": "The app subtitle for this locale (optional)"
                    }
                },
                "required": [
                    "app_info_id",
                    "locale",
                    "name"
                ]
            },
            "annotations": {
                "title": "Create App Info Localization",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_app_info_localization),
    );
    r.register(
        tool(r##"{
            "name": "update_app_info_localization",
            "description": "Update an existing app info localization. Use this to change the app name, subtitle, or privacy information for a specific locale.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The app info localization ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "The app name (optional)"
                    },
                    "privacy_choices_url": {
                        "type": "string",
                        "description": "Privacy choices URL (optional)"
                    },
                    "privacy_policy_text": {
                        "type": "string",
                        "description": "Privacy policy text (optional)"
                    },
                    "privacy_policy_url": {
                        "type": "string",
                        "description": "Privacy policy URL (optional)"
                    },
                    "subtitle": {
                        "type": "string",
                        "description": "The app subtitle (optional)"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Update App Info Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_app_info_localization),
    );
    r.register(
        tool(r##"{
            "name": "delete_app_info_localization",
            "description": "Delete an app info localization. This removes support for a specific language.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The app info localization ID to delete"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Delete App Info Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_app_info_localization),
    );
    r.register(
        tool(r##"{
            "name": "get_app_infos",
            "description": "Get app info resources for an app. Returns app info IDs needed for localization operations.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App Store Connect app ID"
                    }
                },
                "required": [
                    "app_id"
                ]
            },
            "annotations": {
                "title": "Get App Infos",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_infos),
    );
}

/// Ported from `Registry.registerVersionLocalizationTools`.
pub(super) fn register_version_localization_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_version_localizations",
            "description": "List all localizations for an app store version (descriptions, keywords, what's new). Requires a version_id from get_app_versions.",
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
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The app store version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "List Version Localizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_version_localizations),
    );
    r.register(
        tool(r##"{
            "name": "get_version_localization",
            "description": "Get a specific version localization by ID.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The version localization ID"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Get Version Localization",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_version_localization),
    );
    r.register(
        tool(r##"{
            "name": "create_version_localization",
            "description": "Create a new localization for an app store version. Use this to add support for a new language on a specific version.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "description": {
                        "type": "string",
                        "description": "The full app description for this locale (optional)"
                    },
                    "keywords": {
                        "type": "string",
                        "description": "Comma-separated keywords for App Store search (optional, max 100 chars)"
                    },
                    "locale": {
                        "type": "string",
                        "description": "The locale code (e.g., en-US, de-DE, ja)"
                    },
                    "marketing_url": {
                        "type": "string",
                        "description": "Marketing URL (optional)"
                    },
                    "promotional_text": {
                        "type": "string",
                        "description": "Promotional text that appears above the description (optional)"
                    },
                    "support_url": {
                        "type": "string",
                        "description": "Support URL (optional)"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The app store version ID"
                    },
                    "whats_new": {
                        "type": "string",
                        "description": "Release notes / what's new text (optional)"
                    }
                },
                "required": [
                    "version_id",
                    "locale"
                ]
            },
            "annotations": {
                "title": "Create Version Localization",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_version_localization),
    );
    r.register(
        tool(r##"{
            "name": "update_version_localization",
            "description": "Update an existing version localization. Use this to change description, keywords, what's new, and other metadata for a specific locale.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "description": {
                        "type": "string",
                        "description": "The full app description (optional)"
                    },
                    "keywords": {
                        "type": "string",
                        "description": "Comma-separated keywords for App Store search (optional, max 100 chars)"
                    },
                    "localization_id": {
                        "type": "string",
                        "description": "The version localization ID"
                    },
                    "marketing_url": {
                        "type": "string",
                        "description": "Marketing URL (optional)"
                    },
                    "promotional_text": {
                        "type": "string",
                        "description": "Promotional text that appears above the description (optional)"
                    },
                    "support_url": {
                        "type": "string",
                        "description": "Support URL (optional)"
                    },
                    "whats_new": {
                        "type": "string",
                        "description": "Release notes / what's new text (optional)"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Update Version Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_version_localization),
    );
    r.register(
        tool(r##"{
            "name": "delete_version_localization",
            "description": "Delete a version localization. This removes support for a specific language from a version.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The version localization ID to delete"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Delete Version Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_version_localization),
    );
}

/// Ported from `Registry.handleListAppInfoLocalizations`.
async fn handle_list_app_info_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppInfoID string \"json:\\\"app_info_id\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_info_id: String => "app_info_id";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListAppInfoLocalizations from internal/asc/tools/localizations.go")
}

/// Ported from `Registry.handleGetAppInfoLocalization`.
async fn handle_get_app_info_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    todo!("port handleGetAppInfoLocalization from internal/asc/tools/localizations.go")
}

/// Ported from `Registry.handleCreateAppInfoLocalization`.
async fn handle_create_app_info_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppInfoID string \"json:\\\"app_info_id\\\"\"; Locale string \"json:\\\"locale\\\"\"; Name string \"json:\\\"name\\\"\"; Subtitle string \"json:\\\"subtitle\\\"\"; PrivacyPolicyURL string \"json:\\\"privacy_policy_url\\\"\"; PrivacyChoicesURL string \"json:\\\"privacy_choices_url\\\"\"; PrivacyPolicyText string \"json:\\\"privacy_policy_text\\\"\" }", "") {
        app_info_id: String => "app_info_id";
        locale: String => "locale";
        name: String => "name";
        subtitle: String => "subtitle";
        privacy_policy_url: String => "privacy_policy_url";
        privacy_choices_url: String => "privacy_choices_url";
        privacy_policy_text: String => "privacy_policy_text";
    }
    }
    todo!("port handleCreateAppInfoLocalization from internal/asc/tools/localizations.go")
}

/// Ported from `Registry.handleUpdateAppInfoLocalization`.
async fn handle_update_app_info_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Name string \"json:\\\"name\\\"\"; Subtitle string \"json:\\\"subtitle\\\"\"; PrivacyPolicyURL string \"json:\\\"privacy_policy_url\\\"\"; PrivacyChoicesURL string \"json:\\\"privacy_choices_url\\\"\"; PrivacyPolicyText string \"json:\\\"privacy_policy_text\\\"\" }", "") {
        localization_id: String => "localization_id";
        name: String => "name";
        subtitle: String => "subtitle";
        privacy_policy_url: String => "privacy_policy_url";
        privacy_choices_url: String => "privacy_choices_url";
        privacy_policy_text: String => "privacy_policy_text";
    }
    }
    todo!("port handleUpdateAppInfoLocalization from internal/asc/tools/localizations.go")
}

/// Ported from `Registry.handleDeleteAppInfoLocalization`.
async fn handle_delete_app_info_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    todo!("port handleDeleteAppInfoLocalization from internal/asc/tools/localizations.go")
}

/// Ported from `Registry.handleGetAppInfos`.
async fn handle_get_app_infos(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\" }", "") {
        app_id: String => "app_id";
    }
    }
    todo!("port handleGetAppInfos from internal/asc/tools/localizations.go")
}

/// Ported from `Registry.handleListVersionLocalizations`.
async fn handle_list_version_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        version_id: String => "version_id";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListVersionLocalizations from internal/asc/tools/localizations.go")
}

/// Ported from `Registry.handleGetVersionLocalization`.
async fn handle_get_version_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    todo!("port handleGetVersionLocalization from internal/asc/tools/localizations.go")
}

/// Ported from `Registry.handleCreateVersionLocalization`.
async fn handle_create_version_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; Locale string \"json:\\\"locale\\\"\"; Description string \"json:\\\"description\\\"\"; Keywords string \"json:\\\"keywords\\\"\"; WhatsNew string \"json:\\\"whats_new\\\"\"; PromotionalText string \"json:\\\"promotional_text\\\"\"; MarketingURL string \"json:\\\"marketing_url\\\"\"; SupportURL string \"json:\\\"support_url\\\"\" }", "") {
        version_id: String => "version_id";
        locale: String => "locale";
        description: String => "description";
        keywords: String => "keywords";
        whats_new: String => "whats_new";
        promotional_text: String => "promotional_text";
        marketing_url: String => "marketing_url";
        support_url: String => "support_url";
    }
    }
    todo!("port handleCreateVersionLocalization from internal/asc/tools/localizations.go")
}

/// Ported from `Registry.handleUpdateVersionLocalization`.
async fn handle_update_version_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Description string \"json:\\\"description\\\"\"; Keywords string \"json:\\\"keywords\\\"\"; WhatsNew string \"json:\\\"whats_new\\\"\"; PromotionalText string \"json:\\\"promotional_text\\\"\"; MarketingURL string \"json:\\\"marketing_url\\\"\"; SupportURL string \"json:\\\"support_url\\\"\" }", "") {
        localization_id: String => "localization_id";
        description: String => "description";
        keywords: String => "keywords";
        whats_new: String => "whats_new";
        promotional_text: String => "promotional_text";
        marketing_url: String => "marketing_url";
        support_url: String => "support_url";
    }
    }
    todo!("port handleUpdateVersionLocalization from internal/asc/tools/localizations.go")
}

/// Ported from `Registry.handleDeleteVersionLocalization`.
async fn handle_delete_version_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    todo!("port handleDeleteVersionLocalization from internal/asc/tools/localizations.go")
}

