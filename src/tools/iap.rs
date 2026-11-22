//! Ported from internal/asc/tools/iap.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerInAppPurchaseTools`.
pub(super) fn register_in_app_purchase_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_in_app_purchases",
            "description": "List in-app purchases for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to list in-app purchases for"
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
                        "description": "Maximum number of in-app purchases to return (default 50)"
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
                "title": "List In App Purchases",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_in_app_purchases),
    );
    r.register(
        tool(r##"{
            "name": "get_in_app_purchase",
            "description": "Get details of a specific in-app purchase",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "iap_id": {
                        "type": "string",
                        "description": "The in-app purchase ID"
                    }
                },
                "required": [
                    "iap_id"
                ]
            },
            "annotations": {
                "title": "Get In App Purchase",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_in_app_purchase),
    );
    r.register(
        tool(r##"{
            "name": "create_in_app_purchase",
            "description": "Create a new in-app purchase",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to create the in-app purchase for"
                    },
                    "family_sharable": {
                        "type": "boolean",
                        "description": "Whether the IAP is sharable with family"
                    },
                    "iap_type": {
                        "type": "string",
                        "description": "The type (CONSUMABLE, NON_CONSUMABLE, NON_RENEWING_SUBSCRIPTION)"
                    },
                    "name": {
                        "type": "string",
                        "description": "The name of the in-app purchase"
                    },
                    "product_id": {
                        "type": "string",
                        "description": "The product identifier"
                    },
                    "review_note": {
                        "type": "string",
                        "description": "Notes for App Review"
                    }
                },
                "required": [
                    "app_id",
                    "name",
                    "product_id",
                    "iap_type"
                ]
            },
            "annotations": {
                "title": "Create In App Purchase",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_in_app_purchase),
    );
    r.register(
        tool(r##"{
            "name": "update_in_app_purchase",
            "description": "Update an existing in-app purchase",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "family_sharable": {
                        "type": "boolean",
                        "description": "Whether the IAP is sharable with family"
                    },
                    "iap_id": {
                        "type": "string",
                        "description": "The in-app purchase ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "The updated name"
                    },
                    "review_note": {
                        "type": "string",
                        "description": "Updated notes for App Review"
                    }
                },
                "required": [
                    "iap_id"
                ]
            },
            "annotations": {
                "title": "Update In App Purchase",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_in_app_purchase),
    );
    r.register(
        tool(r##"{
            "name": "delete_in_app_purchase",
            "description": "Delete an in-app purchase",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "iap_id": {
                        "type": "string",
                        "description": "The in-app purchase ID"
                    }
                },
                "required": [
                    "iap_id"
                ]
            },
            "annotations": {
                "title": "Delete In App Purchase",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_in_app_purchase),
    );
}

/// Ported from `Registry.handleListInAppPurchases`.
async fn handle_list_in_app_purchases(c: Arc<Client>, args: Args) -> HandlerResult {
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
    todo!("port handleListInAppPurchases from internal/asc/tools/iap.go")
}

/// Ported from `Registry.handleGetInAppPurchase`.
async fn handle_get_in_app_purchase(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\" }", "") {
        iap_id: String => "iap_id";
    }
    }
    todo!("port handleGetInAppPurchase from internal/asc/tools/iap.go")
}

/// Ported from `Registry.handleCreateInAppPurchase`.
async fn handle_create_in_app_purchase(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Name string \"json:\\\"name\\\"\"; ProductID string \"json:\\\"product_id\\\"\"; IAPType string \"json:\\\"iap_type\\\"\"; ReviewNote string \"json:\\\"review_note\\\"\"; FamilySharable bool \"json:\\\"family_sharable\\\"\" }", "") {
        app_id: String => "app_id";
        name: String => "name";
        product_id: String => "product_id";
        iap_type: String => "iap_type";
        review_note: String => "review_note";
        family_sharable: bool => "family_sharable";
    }
    }
    todo!("port handleCreateInAppPurchase from internal/asc/tools/iap.go")
}

/// Ported from `Registry.handleUpdateInAppPurchase`.
async fn handle_update_in_app_purchase(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\"; Name string \"json:\\\"name\\\"\"; ReviewNote string \"json:\\\"review_note\\\"\"; FamilySharable *bool \"json:\\\"family_sharable\\\"\" }", "") {
        iap_id: String => "iap_id";
        name: String => "name";
        review_note: String => "review_note";
        family_sharable: Option<bool> => "family_sharable";
    }
    }
    todo!("port handleUpdateInAppPurchase from internal/asc/tools/iap.go")
}

/// Ported from `Registry.handleDeleteInAppPurchase`.
async fn handle_delete_in_app_purchase(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\" }", "") {
        iap_id: String => "iap_id";
    }
    }
    todo!("port handleDeleteInAppPurchase from internal/asc/tools/iap.go")
}

