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
    todo!("port handleGetEndUserLicenseAgreement from internal/asc/tools/misc.go")
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
    todo!("port handleCreateEndUserLicenseAgreement from internal/asc/tools/misc.go")
}

/// Ported from `Registry.handleUpdateEndUserLicenseAgreement`.
async fn handle_update_end_user_license_agreement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { EULAID string \"json:\\\"eula_id\\\"\"; AgreementText string \"json:\\\"agreement_text\\\"\" }", "") {
        eula_id: String => "eula_id";
        agreement_text: String => "agreement_text";
    }
    }
    todo!("port handleUpdateEndUserLicenseAgreement from internal/asc/tools/misc.go")
}

/// Ported from `Registry.handleDeleteEndUserLicenseAgreement`.
async fn handle_delete_end_user_license_agreement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { EULAID string \"json:\\\"eula_id\\\"\" }", "") {
        eula_id: String => "eula_id";
    }
    }
    todo!("port handleDeleteEndUserLicenseAgreement from internal/asc/tools/misc.go")
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
    todo!("port handleListAppCategories from internal/asc/tools/misc.go")
}

/// Ported from `Registry.handleGetAppCategory`.
async fn handle_get_app_category(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { CategoryID string \"json:\\\"category_id\\\"\" }", "") {
        category_id: String => "category_id";
    }
    }
    todo!("port handleGetAppCategory from internal/asc/tools/misc.go")
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
    todo!("port handleListAlternativeDistributionKeys from internal/asc/tools/misc.go")
}

/// Ported from `Registry.handleGetAlternativeDistributionKey`.
async fn handle_get_alternative_distribution_key(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { KeyID string \"json:\\\"key_id\\\"\" }", "") {
        key_id: String => "key_id";
    }
    }
    todo!("port handleGetAlternativeDistributionKey from internal/asc/tools/misc.go")
}

/// Ported from `Registry.handleCreateAlternativeDistributionKey`.
async fn handle_create_alternative_distribution_key(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\" }", "") {
        app_id: String => "app_id";
    }
    }
    todo!("port handleCreateAlternativeDistributionKey from internal/asc/tools/misc.go")
}

/// Ported from `Registry.handleDeleteAlternativeDistributionKey`.
async fn handle_delete_alternative_distribution_key(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { KeyID string \"json:\\\"key_id\\\"\" }", "") {
        key_id: String => "key_id";
    }
    }
    todo!("port handleDeleteAlternativeDistributionKey from internal/asc/tools/misc.go")
}

/// Ported from `Registry.handleGetAlternativeDistributionPackage`.
async fn handle_get_alternative_distribution_package(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    todo!("port handleGetAlternativeDistributionPackage from internal/asc/tools/misc.go")
}

/// Ported from `Registry.handleGetMarketplaceSearchDetail`.
async fn handle_get_marketplace_search_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\" }", "") {
        app_id: String => "app_id";
    }
    }
    todo!("port handleGetMarketplaceSearchDetail from internal/asc/tools/misc.go")
}

/// Ported from `Registry.handleCreateMarketplaceSearchDetail`.
async fn handle_create_marketplace_search_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; CatalogURL string \"json:\\\"catalog_url\\\"\" }", "") {
        app_id: String => "app_id";
        catalog_url: String => "catalog_url";
    }
    }
    todo!("port handleCreateMarketplaceSearchDetail from internal/asc/tools/misc.go")
}

/// Ported from `Registry.handleUpdateMarketplaceSearchDetail`.
async fn handle_update_marketplace_search_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { DetailID string \"json:\\\"detail_id\\\"\"; CatalogURL string \"json:\\\"catalog_url\\\"\" }", "") {
        detail_id: String => "detail_id";
        catalog_url: String => "catalog_url";
    }
    }
    todo!("port handleUpdateMarketplaceSearchDetail from internal/asc/tools/misc.go")
}

/// Ported from `Registry.handleDeleteMarketplaceSearchDetail`.
async fn handle_delete_marketplace_search_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { DetailID string \"json:\\\"detail_id\\\"\" }", "") {
        detail_id: String => "detail_id";
    }
    }
    todo!("port handleDeleteMarketplaceSearchDetail from internal/asc/tools/misc.go")
}

