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
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_in_app_purchases(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list in-app purchases: {e}"))),
    };

    Ok(list_result(&format_in_app_purchases(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetInAppPurchase`.
async fn handle_get_in_app_purchase(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\" }", "") {
        iap_id: String => "iap_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.iap_id.is_empty() {
        return Ok(error_result("iap_id is required"));
    }

    let resp = match c.get_in_app_purchase(&params.iap_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get in-app purchase: {e}"))),
    };

    Ok(data_result(&format_in_app_purchase(&resp.data), &resp.data))
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }
    if params.name.is_empty() {
        return Ok(error_result("name is required"));
    }
    if params.product_id.is_empty() {
        return Ok(error_result("product_id is required"));
    }
    if params.iap_type.is_empty() {
        return Ok(error_result("iap_type is required"));
    }

    let req = InAppPurchaseCreateRequest {
        data: InAppPurchaseCreateData {
            type_: "inAppPurchases".into(),
            attributes: InAppPurchaseCreateAttributes {
                name: params.name.clone(),
                product_id: params.product_id.clone(),
                in_app_purchase_type: params.iap_type.clone(),
                review_note: params.review_note.clone(),
                family_sharable: params.family_sharable,
                ..Default::default()
            },
            relationships: InAppPurchaseCreateRelationships {
                app: RelationshipData {
                    data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_in_app_purchase(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create in-app purchase: {e}"))),
    };

    Ok(data_result(
        &format!("Created in-app purchase: {} (ID: {})", resp.data.attributes.name, resp.data.id),
        &resp.data,
    ))
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.iap_id.is_empty() {
        return Ok(error_result("iap_id is required"));
    }

    let req = InAppPurchaseUpdateRequest {
        data: InAppPurchaseUpdateData {
            type_: "inAppPurchases".into(),
            id: params.iap_id.clone(),
            attributes: InAppPurchaseUpdateAttributes {
                name: params.name.clone(),
                review_note: params.review_note.clone(),
                family_sharable: params.family_sharable,
                ..Default::default()
            },
        },
    };

    let resp = match c.update_in_app_purchase(&params.iap_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update in-app purchase: {e}"))),
    };

    Ok(data_result(&format!("Updated in-app purchase: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleDeleteInAppPurchase`.
async fn handle_delete_in_app_purchase(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\" }", "") {
        iap_id: String => "iap_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.iap_id.is_empty() {
        return Ok(error_result("iap_id is required"));
    }

    if let Err(e) = c.delete_in_app_purchase(&params.iap_id).await {
        return Ok(error_result(format!("Failed to delete in-app purchase: {e}")));
    }

    Ok(CallToolResult::success("In-app purchase deleted successfully"))
}

/// Ported from `formatInAppPurchases`.
fn format_in_app_purchases(iaps: &[InAppPurchase]) -> String {
    if iaps.is_empty() {
        return "No in-app purchases found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} in-app purchases:\n\n", iaps.len());

    for iap in iaps {
        sb.push_str(&format_in_app_purchase(iap));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatInAppPurchase`.
fn format_in_app_purchase(iap: &InAppPurchase) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", iap.id);
    let _ = write!(sb, "Name: {}\n", iap.attributes.name);
    let _ = write!(sb, "Product ID: {}\n", iap.attributes.product_id);
    let _ = write!(sb, "Type: {}\n", iap.attributes.in_app_purchase_type);
    let _ = write!(sb, "State: {}\n", iap.attributes.state);
    let _ = write!(sb, "Family Sharable: {}\n", iap.attributes.family_sharable);
    if !iap.attributes.review_note.is_empty() {
        let _ = write!(sb, "Review Note: {}\n", iap.attributes.review_note);
    }
    sb
}
