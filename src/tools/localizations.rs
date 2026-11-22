//! App info and version localization tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
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

/// Registers this module's tools in definition order.
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

/// Handles `list_app_info_localizations`.
async fn handle_list_app_info_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppInfoID string \"json:\\\"app_info_id\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_info_id: String => "app_info_id";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_info_id.is_empty() {
        return Ok(error_result("app_info_id is required"));
    }

    let opts = list_opts(0, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_info_localizations(&params.app_info_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list app info localizations: {e}"))),
    };

    Ok(list_result(&format_app_info_localizations(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_app_info_localization`.
async fn handle_get_app_info_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
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

    let resp = match c.get_app_info_localization(&params.localization_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get app info localization: {e}"))),
    };

    Ok(data_result(&format_app_info_localization(&resp.data), &resp.data))
}

/// Handles `create_app_info_localization`.
async fn handle_create_app_info_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_info_id.is_empty() || params.locale.is_empty() || params.name.is_empty() {
        return Ok(error_result("app_info_id, locale, and name are required"));
    }

    let req = AppInfoLocalizationCreateRequest {
        data: AppInfoLocalizationCreateData {
            type_: "appInfoLocalizations".into(),
            attributes: AppInfoLocalizationCreateAttributes {
                locale: params.locale.clone(),
                name: params.name.clone(),
                subtitle: params.subtitle.clone(),
                privacy_policy_url: params.privacy_policy_url.clone(),
                privacy_choices_url: params.privacy_choices_url.clone(),
                privacy_policy_text: params.privacy_policy_text.clone(),
            },
            relationships: AppInfoLocalizationCreateRelationships {
                app_info: RelationshipData { data: ResourceIdentifier { type_: "appInfos".into(), id: params.app_info_id.clone() } },
            },
        },
    };

    let resp = match c.create_app_info_localization(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create app info localization: {e}"))),
    };

    let result = format!(
        "Created app info localization for locale '{}'\n\n{}",
        params.locale,
        format_app_info_localization(&resp.data)
    );
    Ok(data_result(&result, &resp.data))
}

/// Handles `update_app_info_localization`.
async fn handle_update_app_info_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Name string \"json:\\\"name\\\"\"; Subtitle string \"json:\\\"subtitle\\\"\"; PrivacyPolicyURL string \"json:\\\"privacy_policy_url\\\"\"; PrivacyChoicesURL string \"json:\\\"privacy_choices_url\\\"\"; PrivacyPolicyText string \"json:\\\"privacy_policy_text\\\"\" }", "") {
        localization_id: String => "localization_id";
        name: String => "name";
        subtitle: String => "subtitle";
        privacy_policy_url: String => "privacy_policy_url";
        privacy_choices_url: String => "privacy_choices_url";
        privacy_policy_text: String => "privacy_policy_text";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }

    let req = AppInfoLocalizationUpdateRequest {
        data: AppInfoLocalizationUpdateData {
            type_: "appInfoLocalizations".into(),
            id: params.localization_id.clone(),
            attributes: AppInfoLocalizationUpdateAttributes {
                name: params.name.clone(),
                subtitle: params.subtitle.clone(),
                privacy_policy_url: params.privacy_policy_url.clone(),
                privacy_choices_url: params.privacy_choices_url.clone(),
                privacy_policy_text: params.privacy_policy_text.clone(),
            },
        },
    };

    let resp = match c.update_app_info_localization(&params.localization_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update app info localization: {e}"))),
    };

    let result = format!("Updated app info localization\n\n{}", format_app_info_localization(&resp.data));
    Ok(data_result(&result, &resp.data))
}

/// Handles `delete_app_info_localization`.
async fn handle_delete_app_info_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
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

    if let Err(e) = c.delete_app_info_localization(&params.localization_id).await {
        return Ok(error_result(format!("Failed to delete app info localization: {e}")));
    }

    Ok(CallToolResult::success("Successfully deleted app info localization"))
}

/// Handles `get_app_infos`.
async fn handle_get_app_infos(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
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

    let resp = match c.get_app_infos(&params.app_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get app infos: {e}"))),
    };

    Ok(data_result(&format_app_infos(&resp.data), &resp.data))
}

/// Handles `list_version_localizations`.
async fn handle_list_version_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        version_id: String => "version_id";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    let opts = list_opts(0, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_store_version_localizations(&params.version_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list version localizations: {e}"))),
    };

    Ok(list_result(&format_version_localizations(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_version_localization`.
async fn handle_get_version_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
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

    let resp = match c.get_app_store_version_localization(&params.localization_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get version localization: {e}"))),
    };

    Ok(data_result(&format_version_localization(&resp.data), &resp.data))
}

/// Handles `create_version_localization`.
async fn handle_create_version_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() || params.locale.is_empty() {
        return Ok(error_result("version_id and locale are required"));
    }

    let req = AppStoreVersionLocalizationCreateRequest {
        data: AppStoreVersionLocalizationCreateData {
            type_: "appStoreVersionLocalizations".into(),
            attributes: AppStoreVersionLocalizationCreateAttributes {
                locale: params.locale.clone(),
                description: params.description.clone(),
                keywords: params.keywords.clone(),
                whats_new: params.whats_new.clone(),
                promotional_text: params.promotional_text.clone(),
                marketing_url: params.marketing_url.clone(),
                support_url: params.support_url.clone(),
            },
            relationships: AppStoreVersionLocalizationCreateRelationships {
                app_store_version: RelationshipData {
                    data: ResourceIdentifier { type_: "appStoreVersions".into(), id: params.version_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_app_store_version_localization(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create version localization: {e}"))),
    };

    let result = format!(
        "Created version localization for locale '{}'\n\n{}",
        params.locale,
        format_version_localization(&resp.data)
    );
    Ok(data_result(&result, &resp.data))
}

/// Handles `update_version_localization`.
async fn handle_update_version_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }

    let req = AppStoreVersionLocalizationUpdateRequest {
        data: AppStoreVersionLocalizationUpdateData {
            type_: "appStoreVersionLocalizations".into(),
            id: params.localization_id.clone(),
            attributes: AppStoreVersionLocalizationUpdateAttributes {
                description: params.description.clone(),
                keywords: params.keywords.clone(),
                whats_new: params.whats_new.clone(),
                promotional_text: params.promotional_text.clone(),
                marketing_url: params.marketing_url.clone(),
                support_url: params.support_url.clone(),
            },
        },
    };

    let resp = match c.update_app_store_version_localization(&params.localization_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update version localization: {e}"))),
    };

    let result = format!("Updated version localization\n\n{}", format_version_localization(&resp.data));
    Ok(data_result(&result, &resp.data))
}

/// Handles `delete_version_localization`.
async fn handle_delete_version_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
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

    if let Err(e) = c.delete_app_store_version_localization(&params.localization_id).await {
        return Ok(error_result(format!("Failed to delete version localization: {e}")));
    }

    Ok(CallToolResult::success("Successfully deleted version localization"))
}

// Formatting helpers

fn format_app_infos(infos: &[AppInfo]) -> String {
    if infos.is_empty() {
        return "No app infos found.".into();
    }

    let mut result = String::new();
    for (i, info) in infos.iter().enumerate() {
        if i > 0 {
            result.push_str("\n---\n");
        }
        let _ = writeln!(result, "App Info ID: {}", info.id);
        let _ = writeln!(result, "State: {}", info.attributes.state);
        if !info.attributes.app_store_state.is_empty() {
            let _ = writeln!(result, "App Store State: {}", info.attributes.app_store_state);
        }
    }
    result
}

fn format_app_info_localizations(localizations: &[AppInfoLocalization]) -> String {
    if localizations.is_empty() {
        return "No localizations found.".into();
    }

    let mut result = String::new();
    for (i, loc) in localizations.iter().enumerate() {
        if i > 0 {
            result.push_str("\n---\n");
        }
        result.push_str(&format_app_info_localization(loc));
    }
    result
}

fn format_app_info_localization(loc: &AppInfoLocalization) -> String {
    let a = &loc.attributes;
    let mut result = format!("ID: {}\n", loc.id);
    let _ = writeln!(result, "Locale: {}", a.locale);
    if !a.name.is_empty() {
        let _ = writeln!(result, "Name: {}", a.name);
    }
    if !a.subtitle.is_empty() {
        let _ = writeln!(result, "Subtitle: {}", a.subtitle);
    }
    if !a.privacy_policy_url.is_empty() {
        let _ = writeln!(result, "Privacy Policy URL: {}", a.privacy_policy_url);
    }
    if !a.privacy_choices_url.is_empty() {
        let _ = writeln!(result, "Privacy Choices URL: {}", a.privacy_choices_url);
    }
    if !a.privacy_policy_text.is_empty() {
        let _ = writeln!(result, "Privacy Policy Text: {}", a.privacy_policy_text);
    }
    result
}

fn format_version_localizations(localizations: &[AppStoreVersionLocalization]) -> String {
    if localizations.is_empty() {
        return "No localizations found.".into();
    }

    let mut result = String::new();
    for (i, loc) in localizations.iter().enumerate() {
        if i > 0 {
            result.push_str("\n---\n");
        }
        result.push_str(&format_version_localization(loc));
    }
    result
}

/// Truncates a string longer than `n` bytes to `n` bytes plus `...`. A cut
/// through a multi-byte character leaves invalid UTF-8 bytes, each of which
/// is emitted on the wire as U+FFFD.
fn truncate_ellipsis(s: &str, n: usize) -> String {
    let mut b = n;
    while !s.is_char_boundary(b) {
        b -= 1;
    }
    let mut out = s[..b].to_string();
    for _ in b..n {
        out.push('\u{FFFD}');
    }
    out.push_str("...");
    out
}

fn format_version_localization(loc: &AppStoreVersionLocalization) -> String {
    let a = &loc.attributes;
    let mut result = format!("ID: {}\n", loc.id);
    let _ = writeln!(result, "Locale: {}", a.locale);
    if !a.description.is_empty() {
        let desc = if a.description.len() > 200 { truncate_ellipsis(&a.description, 200) } else { a.description.clone() };
        let _ = writeln!(result, "Description: {}", desc);
    }
    if !a.keywords.is_empty() {
        let _ = writeln!(result, "Keywords: {}", a.keywords);
    }
    if !a.whats_new.is_empty() {
        let whats_new = if a.whats_new.len() > 200 { truncate_ellipsis(&a.whats_new, 200) } else { a.whats_new.clone() };
        let _ = writeln!(result, "What's New: {}", whats_new);
    }
    if !a.promotional_text.is_empty() {
        let _ = writeln!(result, "Promotional Text: {}", a.promotional_text);
    }
    if !a.marketing_url.is_empty() {
        let _ = writeln!(result, "Marketing URL: {}", a.marketing_url);
    }
    if !a.support_url.is_empty() {
        let _ = writeln!(result, "Support URL: {}", a.support_url);
    }
    result
}
