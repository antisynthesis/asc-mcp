//! Ported from internal/asc/tools/misc.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerMiscTools`.
pub(super) fn register_misc_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "get_end_user_license_agreement",
            "description": "Get the End User License Agreement for an app",
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
                "title": "Get End User License Agreement",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_end_user_license_agreement),
    );
    r.register(
        tool(r##"{
            "name": "create_end_user_license_agreement",
            "description": "Create an End User License Agreement for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "agreement_text": {
                        "type": "string",
                        "description": "The EULA text"
                    },
                    "app_id": {
                        "type": "string",
                        "description": "The app ID"
                    },
                    "territory_ids": {
                        "type": "array",
                        "description": "List of territory IDs where this EULA applies",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "app_id",
                    "agreement_text"
                ]
            },
            "annotations": {
                "title": "Create End User License Agreement",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_end_user_license_agreement),
    );
    r.register(
        tool(r##"{
            "name": "update_end_user_license_agreement",
            "description": "Update an End User License Agreement",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "agreement_text": {
                        "type": "string",
                        "description": "The updated EULA text"
                    },
                    "eula_id": {
                        "type": "string",
                        "description": "The EULA ID"
                    }
                },
                "required": [
                    "eula_id",
                    "agreement_text"
                ]
            },
            "annotations": {
                "title": "Update End User License Agreement",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_end_user_license_agreement),
    );
    r.register(
        tool(r##"{
            "name": "delete_end_user_license_agreement",
            "description": "Delete an End User License Agreement (reverts to standard Apple EULA)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "eula_id": {
                        "type": "string",
                        "description": "The EULA ID to delete"
                    }
                },
                "required": [
                    "eula_id"
                ]
            },
            "annotations": {
                "title": "Delete End User License Agreement",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_end_user_license_agreement),
    );
    r.register(
        tool(r##"{
            "name": "list_app_categories",
            "description": "List available App Store categories",
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
                        "description": "Maximum number of categories to return (default 100)"
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
                "title": "List App Categories",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_app_categories),
    );
    r.register(
        tool(r##"{
            "name": "get_app_category",
            "description": "Get details of a specific App Store category",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "category_id": {
                        "type": "string",
                        "description": "The category ID"
                    }
                },
                "required": [
                    "category_id"
                ]
            },
            "annotations": {
                "title": "Get App Category",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_category),
    );
    r.register(
        tool(r##"{
            "name": "list_alternative_distribution_keys",
            "description": "List alternative distribution keys for EU marketplace distribution",
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
                        "description": "Maximum number of keys to return (default 50)"
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
                "title": "List Alternative Distribution Keys",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_alternative_distribution_keys),
    );
    r.register(
        tool(r##"{
            "name": "get_alternative_distribution_key",
            "description": "Get a specific alternative distribution key",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "key_id": {
                        "type": "string",
                        "description": "The alternative distribution key ID"
                    }
                },
                "required": [
                    "key_id"
                ]
            },
            "annotations": {
                "title": "Get Alternative Distribution Key",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_alternative_distribution_key),
    );
    r.register(
        tool(r##"{
            "name": "create_alternative_distribution_key",
            "description": "Create a new alternative distribution key for EU marketplace",
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
                "title": "Create Alternative Distribution Key",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_alternative_distribution_key),
    );
    r.register(
        tool(r##"{
            "name": "delete_alternative_distribution_key",
            "description": "Delete an alternative distribution key",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "key_id": {
                        "type": "string",
                        "description": "The alternative distribution key ID to delete"
                    }
                },
                "required": [
                    "key_id"
                ]
            },
            "annotations": {
                "title": "Delete Alternative Distribution Key",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_alternative_distribution_key),
    );
    r.register(
        tool(r##"{
            "name": "get_alternative_distribution_package",
            "description": "Get the alternative distribution package for an App Store version (EU marketplace distribution)",
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
                "title": "Get Alternative Distribution Package",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_alternative_distribution_package),
    );
    r.register(
        tool(r##"{
            "name": "get_marketplace_search_detail",
            "description": "Get marketplace search details for an app (EU alternative marketplaces)",
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
                "title": "Get Marketplace Search Detail",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_marketplace_search_detail),
    );
    r.register(
        tool(r##"{
            "name": "create_marketplace_search_detail",
            "description": "Create marketplace search detail for EU distribution",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The app ID"
                    },
                    "catalog_url": {
                        "type": "string",
                        "description": "URL for the marketplace catalog"
                    }
                },
                "required": [
                    "app_id",
                    "catalog_url"
                ]
            },
            "annotations": {
                "title": "Create Marketplace Search Detail",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_marketplace_search_detail),
    );
    r.register(
        tool(r##"{
            "name": "update_marketplace_search_detail",
            "description": "Update marketplace search detail",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "catalog_url": {
                        "type": "string",
                        "description": "New URL for the marketplace catalog"
                    },
                    "detail_id": {
                        "type": "string",
                        "description": "The marketplace search detail ID"
                    }
                },
                "required": [
                    "detail_id",
                    "catalog_url"
                ]
            },
            "annotations": {
                "title": "Update Marketplace Search Detail",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_marketplace_search_detail),
    );
    r.register(
        tool(r##"{
            "name": "delete_marketplace_search_detail",
            "description": "Delete marketplace search detail",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "detail_id": {
                        "type": "string",
                        "description": "The marketplace search detail ID to delete"
                    }
                },
                "required": [
                    "detail_id"
                ]
            },
            "annotations": {
                "title": "Delete Marketplace Search Detail",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_marketplace_search_detail),
    );
}

/// Ported from `Registry.handleGetEndUserLicenseAgreement`.
async fn handle_get_end_user_license_agreement(c: Arc<Client>, args: Args) -> HandlerResult {
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

    let resp = match c.get_end_user_license_agreement(&params.app_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get EULA: {e}"))),
    };

    Ok(data_result(&format_end_user_license_agreement(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateEndUserLicenseAgreement`.
async fn handle_create_end_user_license_agreement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; AgreementText string \"json:\\\"agreement_text\\\"\"; TerritoryIDs []string \"json:\\\"territory_ids\\\"\" }", "") {
        app_id: String => "app_id";
        agreement_text: String => "agreement_text";
        territory_ids: GoSlice<String> => "territory_ids";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() || params.agreement_text.is_empty() {
        return Ok(error_result("app_id and agreement_text are required"));
    }

    let mut territories: GoSlice<ResourceIdentifier> = GoSlice::nil();
    for tid in params.territory_ids.iter() {
        territories.push(ResourceIdentifier { type_: "territories".into(), id: tid.clone() });
    }

    let req = EndUserLicenseAgreementCreateRequest {
        data: EndUserLicenseAgreementCreateData {
            type_: "endUserLicenseAgreements".into(),
            attributes: EndUserLicenseAgreementCreateAttributes { agreement_text: params.agreement_text.clone() },
            relationships: EndUserLicenseAgreementCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
                territories: RelationshipDataList { data: territories },
            },
        },
    };

    let resp = match c.create_end_user_license_agreement(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create EULA: {e}"))),
    };

    Ok(data_result(&format!("EULA created:\n{}", format_end_user_license_agreement(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleUpdateEndUserLicenseAgreement`.
async fn handle_update_end_user_license_agreement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { EULAID string \"json:\\\"eula_id\\\"\"; AgreementText string \"json:\\\"agreement_text\\\"\" }", "") {
        eula_id: String => "eula_id";
        agreement_text: String => "agreement_text";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.eula_id.is_empty() || params.agreement_text.is_empty() {
        return Ok(error_result("eula_id and agreement_text are required"));
    }

    let req = EndUserLicenseAgreementUpdateRequest {
        data: EndUserLicenseAgreementUpdateData {
            type_: "endUserLicenseAgreements".into(),
            id: params.eula_id.clone(),
            attributes: EndUserLicenseAgreementUpdateAttributes { agreement_text: params.agreement_text.clone() },
            relationships: None,
        },
    };

    let resp = match c.update_end_user_license_agreement(&params.eula_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update EULA: {e}"))),
    };

    Ok(data_result(&format!("EULA updated:\n{}", format_end_user_license_agreement(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleDeleteEndUserLicenseAgreement`.
async fn handle_delete_end_user_license_agreement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { EULAID string \"json:\\\"eula_id\\\"\" }", "") {
        eula_id: String => "eula_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.eula_id.is_empty() {
        return Ok(error_result("eula_id is required"));
    }

    if let Err(e) = c.delete_end_user_license_agreement(&params.eula_id).await {
        return Ok(error_result(format!("Failed to delete EULA: {e}")));
    }

    Ok(CallToolResult::success("EULA deleted (reverted to standard Apple EULA)"))
}

/// Ported from `Registry.handleListAppCategories`.
async fn handle_list_app_categories(c: Arc<Client>, args: Args) -> HandlerResult {
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
        limit = 100;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_categories(opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list app categories: {e}"))),
    };

    Ok(list_result(&format_app_categories(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetAppCategory`.
async fn handle_get_app_category(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { CategoryID string \"json:\\\"category_id\\\"\" }", "") {
        category_id: String => "category_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.category_id.is_empty() {
        return Ok(error_result("category_id is required"));
    }

    let resp = match c.get_app_category(&params.category_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get app category: {e}"))),
    };

    Ok(data_result(&format_app_category(&resp.data), &resp.data))
}

/// Ported from `Registry.handleListAlternativeDistributionKeys`.
async fn handle_list_alternative_distribution_keys(c: Arc<Client>, args: Args) -> HandlerResult {
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

    let limit = clamp_limit(params.limit);

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_alternative_distribution_keys(opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list alternative distribution keys: {e}"))),
    };

    Ok(list_result(&format_alternative_distribution_keys(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetAlternativeDistributionKey`.
async fn handle_get_alternative_distribution_key(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { KeyID string \"json:\\\"key_id\\\"\" }", "") {
        key_id: String => "key_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.key_id.is_empty() {
        return Ok(error_result("key_id is required"));
    }

    let resp = match c.get_alternative_distribution_key(&params.key_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get alternative distribution key: {e}"))),
    };

    Ok(data_result(&format_alternative_distribution_key(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateAlternativeDistributionKey`.
async fn handle_create_alternative_distribution_key(c: Arc<Client>, args: Args) -> HandlerResult {
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

    let req = AlternativeDistributionKeyCreateRequest {
        data: AlternativeDistributionKeyCreateData {
            type_: "alternativeDistributionKeys".into(),
            attributes: AlternativeDistributionKeyCreateAttributes::default(),
            relationships: AlternativeDistributionKeyCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
            },
        },
    };

    let resp = match c.create_alternative_distribution_key(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create alternative distribution key: {e}"))),
    };

    Ok(data_result(&format!("Alternative distribution key created:\n{}", format_alternative_distribution_key(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleDeleteAlternativeDistributionKey`.
async fn handle_delete_alternative_distribution_key(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { KeyID string \"json:\\\"key_id\\\"\" }", "") {
        key_id: String => "key_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.key_id.is_empty() {
        return Ok(error_result("key_id is required"));
    }

    if let Err(e) = c.delete_alternative_distribution_key(&params.key_id).await {
        return Ok(error_result(format!("Failed to delete alternative distribution key: {e}")));
    }

    Ok(CallToolResult::success("Alternative distribution key deleted"))
}

/// Ported from `Registry.handleGetAlternativeDistributionPackage`.
async fn handle_get_alternative_distribution_package(c: Arc<Client>, args: Args) -> HandlerResult {
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

    let resp = match c.get_alternative_distribution_package_for_version(&params.version_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get alternative distribution package: {e}"))),
    };

    Ok(data_result(&format!("Alternative Distribution Package ID: {}\n", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleGetMarketplaceSearchDetail`.
async fn handle_get_marketplace_search_detail(c: Arc<Client>, args: Args) -> HandlerResult {
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

    let resp = match c.get_marketplace_search_detail(&params.app_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get marketplace search detail: {e}"))),
    };

    Ok(data_result(&format_marketplace_search_detail(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateMarketplaceSearchDetail`.
async fn handle_create_marketplace_search_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; CatalogURL string \"json:\\\"catalog_url\\\"\" }", "") {
        app_id: String => "app_id";
        catalog_url: String => "catalog_url";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() || params.catalog_url.is_empty() {
        return Ok(error_result("app_id and catalog_url are required"));
    }

    let req = MarketplaceSearchDetailCreateRequest {
        data: MarketplaceSearchDetailCreateData {
            type_: "marketplaceSearchDetails".into(),
            attributes: MarketplaceSearchDetailCreateAttributes { catalog_url: params.catalog_url.clone() },
            relationships: MarketplaceSearchDetailCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
            },
        },
    };

    let resp = match c.create_marketplace_search_detail(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create marketplace search detail: {e}"))),
    };

    Ok(data_result(&format!("Marketplace search detail created:\n{}", format_marketplace_search_detail(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleUpdateMarketplaceSearchDetail`.
async fn handle_update_marketplace_search_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { DetailID string \"json:\\\"detail_id\\\"\"; CatalogURL string \"json:\\\"catalog_url\\\"\" }", "") {
        detail_id: String => "detail_id";
        catalog_url: String => "catalog_url";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.detail_id.is_empty() || params.catalog_url.is_empty() {
        return Ok(error_result("detail_id and catalog_url are required"));
    }

    let req = MarketplaceSearchDetailUpdateRequest {
        data: MarketplaceSearchDetailUpdateData {
            type_: "marketplaceSearchDetails".into(),
            id: params.detail_id.clone(),
            attributes: MarketplaceSearchDetailUpdateAttributes { catalog_url: params.catalog_url.clone() },
        },
    };

    let resp = match c.update_marketplace_search_detail(&params.detail_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update marketplace search detail: {e}"))),
    };

    Ok(data_result(&format!("Marketplace search detail updated:\n{}", format_marketplace_search_detail(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleDeleteMarketplaceSearchDetail`.
async fn handle_delete_marketplace_search_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { DetailID string \"json:\\\"detail_id\\\"\" }", "") {
        detail_id: String => "detail_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.detail_id.is_empty() {
        return Ok(error_result("detail_id is required"));
    }

    if let Err(e) = c.delete_marketplace_search_detail(&params.detail_id).await {
        return Ok(error_result(format!("Failed to delete marketplace search detail: {e}")));
    }

    Ok(CallToolResult::success("Marketplace search detail deleted"))
}

/// The `limit <= 0 → 50; limit > 200 → 200` clamp the Go handlers inline.
fn clamp_limit(limit: i64) -> i64 {
    let mut limit = limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }
    limit
}

/// Go `s[:n] + "..."` on a string: slices bytes (possibly mid-rune) and
/// renders the result the way Go's JSON encoder would.
fn go_truncate(s: &str, n: usize) -> String {
    let mut b = s.as_bytes()[..n].to_vec();
    b.extend_from_slice(b"...");
    super::reports::go_string(&b)
}

/// Ported from `formatEndUserLicenseAgreement`.
fn format_end_user_license_agreement(eula: &EndUserLicenseAgreement) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", eula.id);
    let mut text = eula.attributes.agreement_text.clone();
    if text.len() > 500 {
        text = go_truncate(&text, 500);
    }
    let _ = write!(sb, "Agreement Text:\n{}\n", text);
    sb
}

/// Ported from `formatAppCategories`.
fn format_app_categories(categories: &[AppCategory]) -> String {
    if categories.is_empty() {
        return "No app categories found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} app categories:\n\n", categories.len());

    for c in categories {
        sb.push_str(&format_app_category(c));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatAppCategory`.
fn format_app_category(c: &AppCategory) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", c.id);
    if !c.attributes.platforms.is_empty() {
        let _ = write!(sb, "Platforms: {}\n", c.attributes.platforms.join(", "));
    }
    sb
}

/// Ported from `formatAlternativeDistributionKeys`.
fn format_alternative_distribution_keys(keys: &[AlternativeDistributionKey]) -> String {
    if keys.is_empty() {
        return "No alternative distribution keys found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} alternative distribution keys:\n\n", keys.len());

    for k in keys {
        sb.push_str(&format_alternative_distribution_key(k));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatAlternativeDistributionKey`.
fn format_alternative_distribution_key(k: &AlternativeDistributionKey) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", k.id);
    if !k.attributes.public_key.is_empty() {
        // Show truncated public key
        let mut pk = k.attributes.public_key.clone();
        if pk.len() > 100 {
            pk = go_truncate(&pk, 100);
        }
        let _ = write!(sb, "Public Key: {}\n", pk);
    }
    sb
}

/// Ported from `formatMarketplaceSearchDetail`.
fn format_marketplace_search_detail(d: &MarketplaceSearchDetail) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", d.id);
    let _ = write!(sb, "Catalog URL: {}\n", d.attributes.catalog_url);
    sb
}
