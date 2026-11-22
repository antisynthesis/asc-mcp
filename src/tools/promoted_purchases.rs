//! Ported from internal/asc/tools/promoted_purchases.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerPromotedPurchasesTools`.
pub(super) fn register_promoted_purchases_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_promoted_purchases",
            "description": "List promoted in-app purchases for an app",
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
                        "description": "Maximum number of promoted purchases to return (default 50)"
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
                "title": "List Promoted Purchases",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_promoted_purchases),
    );
    r.register(
        tool(r##"{
            "name": "get_promoted_purchase",
            "description": "Get details of a specific promoted purchase",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "promoted_purchase_id": {
                        "type": "string",
                        "description": "The promoted purchase ID"
                    }
                },
                "required": [
                    "promoted_purchase_id"
                ]
            },
            "annotations": {
                "title": "Get Promoted Purchase",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_promoted_purchase),
    );
    r.register(
        tool(r##"{
            "name": "create_promoted_purchase",
            "description": "Create a promoted in-app purchase",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The app ID"
                    },
                    "enabled": {
                        "type": "boolean",
                        "description": "Whether promotion is enabled (default true)"
                    },
                    "in_app_purchase_id": {
                        "type": "string",
                        "description": "The in-app purchase ID to promote"
                    },
                    "visibility_type": {
                        "type": "string",
                        "description": "Visibility type: SHOW_FOR_ALL_USERS, APP_STORE_CONNECT_ONLY"
                    }
                },
                "required": [
                    "app_id",
                    "in_app_purchase_id"
                ]
            },
            "annotations": {
                "title": "Create Promoted Purchase",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_promoted_purchase),
    );
    r.register(
        tool(r##"{
            "name": "update_promoted_purchase",
            "description": "Update a promoted purchase's settings",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "enabled": {
                        "type": "boolean",
                        "description": "Whether promotion is enabled"
                    },
                    "promoted_purchase_id": {
                        "type": "string",
                        "description": "The promoted purchase ID"
                    },
                    "visibility_type": {
                        "type": "string",
                        "description": "Visibility type: SHOW_FOR_ALL_USERS, APP_STORE_CONNECT_ONLY"
                    }
                },
                "required": [
                    "promoted_purchase_id"
                ]
            },
            "annotations": {
                "title": "Update Promoted Purchase",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_promoted_purchase),
    );
    r.register(
        tool(r##"{
            "name": "delete_promoted_purchase",
            "description": "Delete a promoted purchase",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "promoted_purchase_id": {
                        "type": "string",
                        "description": "The promoted purchase ID to delete"
                    }
                },
                "required": [
                    "promoted_purchase_id"
                ]
            },
            "annotations": {
                "title": "Delete Promoted Purchase",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_promoted_purchase),
    );
    r.register(
        tool(r##"{
            "name": "list_subscription_offer_codes",
            "description": "List offer codes for a subscription",
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
                        "description": "Maximum number of offer codes to return (default 50)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    },
                    "subscription_id": {
                        "type": "string",
                        "description": "The subscription ID"
                    }
                },
                "required": [
                    "subscription_id"
                ]
            },
            "annotations": {
                "title": "List Subscription Offer Codes",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_subscription_offer_codes),
    );
    r.register(
        tool(r##"{
            "name": "get_subscription_offer_code",
            "description": "Get details of a specific subscription offer code",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "offer_code_id": {
                        "type": "string",
                        "description": "The offer code ID"
                    }
                },
                "required": [
                    "offer_code_id"
                ]
            },
            "annotations": {
                "title": "Get Subscription Offer Code",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_subscription_offer_code),
    );
    r.register(
        tool(r##"{
            "name": "create_subscription_offer_code",
            "description": "Create a new subscription offer code",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "customer_eligibility": {
                        "type": "string",
                        "description": "Customer eligibility: NEW, EXISTING, EXPIRED"
                    },
                    "duration_type": {
                        "type": "string",
                        "description": "Duration type: PAY_AS_YOU_GO, PAY_UP_FRONT, FREE"
                    },
                    "is_active": {
                        "type": "boolean",
                        "description": "Whether the offer code is active"
                    },
                    "name": {
                        "type": "string",
                        "description": "Name of the offer code"
                    },
                    "number_of_periods": {
                        "type": "integer",
                        "description": "Number of periods for the offer"
                    },
                    "offer_eligibility_type": {
                        "type": "string",
                        "description": "Eligibility type: STACK_WITH_INTRO_OFFERS, REPLACE_INTRO_OFFERS"
                    },
                    "offer_mode": {
                        "type": "string",
                        "description": "Offer mode: PAY_AS_YOU_GO, PAY_UP_FRONT, FREE"
                    },
                    "subscription_id": {
                        "type": "string",
                        "description": "The subscription ID"
                    },
                    "total_number_of_codes": {
                        "type": "integer",
                        "description": "Total number of codes to generate"
                    }
                },
                "required": [
                    "subscription_id",
                    "name",
                    "customer_eligibility"
                ]
            },
            "annotations": {
                "title": "Create Subscription Offer Code",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_subscription_offer_code),
    );
    r.register(
        tool(r##"{
            "name": "update_subscription_offer_code",
            "description": "Update a subscription offer code",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "is_active": {
                        "type": "boolean",
                        "description": "Whether the offer code is active"
                    },
                    "offer_code_id": {
                        "type": "string",
                        "description": "The offer code ID"
                    }
                },
                "required": [
                    "offer_code_id"
                ]
            },
            "annotations": {
                "title": "Update Subscription Offer Code",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_subscription_offer_code),
    );
    r.register(
        tool(r##"{
            "name": "list_win_back_offers",
            "description": "List win-back offers for a subscription",
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
                        "description": "Maximum number of offers to return (default 50)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    },
                    "subscription_id": {
                        "type": "string",
                        "description": "The subscription ID"
                    }
                },
                "required": [
                    "subscription_id"
                ]
            },
            "annotations": {
                "title": "List Win Back Offers",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_win_back_offers),
    );
    r.register(
        tool(r##"{
            "name": "get_win_back_offer",
            "description": "Get details of a specific win-back offer",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "offer_id": {
                        "type": "string",
                        "description": "The win-back offer ID"
                    }
                },
                "required": [
                    "offer_id"
                ]
            },
            "annotations": {
                "title": "Get Win Back Offer",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_win_back_offer),
    );
    r.register(
        tool(r##"{
            "name": "create_win_back_offer",
            "description": "Create a new win-back offer for a subscription",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "duration": {
                        "type": "string",
                        "description": "Duration of the offer"
                    },
                    "offer_id": {
                        "type": "string",
                        "description": "Unique identifier for the offer"
                    },
                    "offer_mode": {
                        "type": "string",
                        "description": "Offer mode: PAY_AS_YOU_GO, PAY_UP_FRONT, FREE"
                    },
                    "period_count": {
                        "type": "integer",
                        "description": "Number of periods"
                    },
                    "priority": {
                        "type": "string",
                        "description": "Priority: HIGH, NORMAL"
                    },
                    "promotion_intent": {
                        "type": "string",
                        "description": "Promotion intent: NOT_PROMOTED, USE_CUSTOM_PRODUCT_PAGE"
                    },
                    "reference_name": {
                        "type": "string",
                        "description": "Reference name for the offer"
                    },
                    "subscription_id": {
                        "type": "string",
                        "description": "The subscription ID"
                    }
                },
                "required": [
                    "subscription_id",
                    "reference_name",
                    "offer_id"
                ]
            },
            "annotations": {
                "title": "Create Win Back Offer",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_win_back_offer),
    );
    r.register(
        tool(r##"{
            "name": "update_win_back_offer",
            "description": "Update a win-back offer",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "offer_id": {
                        "type": "string",
                        "description": "The win-back offer ID"
                    },
                    "priority": {
                        "type": "string",
                        "description": "Priority: HIGH, NORMAL"
                    },
                    "promotion_intent": {
                        "type": "string",
                        "description": "Promotion intent: NOT_PROMOTED, USE_CUSTOM_PRODUCT_PAGE"
                    }
                },
                "required": [
                    "offer_id"
                ]
            },
            "annotations": {
                "title": "Update Win Back Offer",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_win_back_offer),
    );
    r.register(
        tool(r##"{
            "name": "delete_win_back_offer",
            "description": "Delete a win-back offer",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "offer_id": {
                        "type": "string",
                        "description": "The win-back offer ID to delete"
                    }
                },
                "required": [
                    "offer_id"
                ]
            },
            "annotations": {
                "title": "Delete Win Back Offer",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_win_back_offer),
    );
}

/// Ported from `Registry.handleListPromotedPurchases`.
async fn handle_list_promoted_purchases(c: Arc<Client>, args: Args) -> HandlerResult {
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
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_promoted_purchases(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list promoted purchases: {e}"))),
    };

    Ok(list_result(&format_promoted_purchases(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetPromotedPurchase`.
async fn handle_get_promoted_purchase(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PromotedPurchaseID string \"json:\\\"promoted_purchase_id\\\"\" }", "") {
        promoted_purchase_id: String => "promoted_purchase_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.promoted_purchase_id.is_empty() {
        return Ok(error_result("promoted_purchase_id is required"));
    }

    let resp = match c.get_promoted_purchase(&params.promoted_purchase_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get promoted purchase: {e}"))),
    };

    Ok(data_result(&format_promoted_purchase(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreatePromotedPurchase`.
async fn handle_create_promoted_purchase(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; InAppPurchaseID string \"json:\\\"in_app_purchase_id\\\"\"; Enabled *bool \"json:\\\"enabled\\\"\"; VisibleForAllUsers *bool \"json:\\\"visible_for_all_users\\\"\" }", "") {
        app_id: String => "app_id";
        in_app_purchase_id: String => "in_app_purchase_id";
        enabled: Option<bool> => "enabled";
        visible_for_all_users: Option<bool> => "visible_for_all_users";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() || params.in_app_purchase_id.is_empty() {
        return Ok(error_result("app_id and in_app_purchase_id are required"));
    }

    let enabled = params.enabled.unwrap_or(true);
    let visible_for_all = params.visible_for_all_users.unwrap_or(true);

    let req = PromotedPurchaseCreateRequest {
        data: PromotedPurchaseCreateData {
            type_: "promotedPurchases".into(),
            attributes: PromotedPurchaseCreateAttributes { enabled, visible_for_all_users: visible_for_all },
            relationships: PromotedPurchaseCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
                in_app_purchase: RelationshipData {
                    data: ResourceIdentifier { type_: "inAppPurchases".into(), id: params.in_app_purchase_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_promoted_purchase(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create promoted purchase: {e}"))),
    };

    Ok(data_result(&format!("Promoted purchase created:\n{}", format_promoted_purchase(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleUpdatePromotedPurchase`.
async fn handle_update_promoted_purchase(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PromotedPurchaseID string \"json:\\\"promoted_purchase_id\\\"\"; Enabled *bool \"json:\\\"enabled\\\"\"; VisibleForAllUsers *bool \"json:\\\"visible_for_all_users\\\"\" }", "") {
        promoted_purchase_id: String => "promoted_purchase_id";
        enabled: Option<bool> => "enabled";
        visible_for_all_users: Option<bool> => "visible_for_all_users";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.promoted_purchase_id.is_empty() {
        return Ok(error_result("promoted_purchase_id is required"));
    }

    let req = PromotedPurchaseUpdateRequest {
        data: PromotedPurchaseUpdateData {
            type_: "promotedPurchases".into(),
            id: params.promoted_purchase_id.clone(),
            attributes: PromotedPurchaseUpdateAttributes {
                enabled: params.enabled,
                visible_for_all_users: params.visible_for_all_users,
            },
        },
    };

    let resp = match c.update_promoted_purchase(&params.promoted_purchase_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update promoted purchase: {e}"))),
    };

    Ok(data_result(&format!("Promoted purchase updated:\n{}", format_promoted_purchase(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleDeletePromotedPurchase`.
async fn handle_delete_promoted_purchase(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PromotedPurchaseID string \"json:\\\"promoted_purchase_id\\\"\" }", "") {
        promoted_purchase_id: String => "promoted_purchase_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.promoted_purchase_id.is_empty() {
        return Ok(error_result("promoted_purchase_id is required"));
    }

    if let Err(e) = c.delete_promoted_purchase(&params.promoted_purchase_id).await {
        return Ok(error_result(format!("Failed to delete promoted purchase: {e}")));
    }

    Ok(CallToolResult::success("Promoted purchase deleted"))
}

/// Ported from `Registry.handleListSubscriptionOfferCodes`.
async fn handle_list_subscription_offer_codes(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubscriptionID string \"json:\\\"subscription_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        subscription_id: String => "subscription_id";
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

    if params.subscription_id.is_empty() {
        return Ok(error_result("subscription_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_subscription_offer_codes(&params.subscription_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list subscription offer codes: {e}"))),
    };

    Ok(list_result(&format_subscription_offer_codes(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetSubscriptionOfferCode`.
async fn handle_get_subscription_offer_code(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferCodeID string \"json:\\\"offer_code_id\\\"\" }", "") {
        offer_code_id: String => "offer_code_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.offer_code_id.is_empty() {
        return Ok(error_result("offer_code_id is required"));
    }

    let resp = match c.get_subscription_offer_code(&params.offer_code_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get subscription offer code: {e}"))),
    };

    Ok(data_result(&format_subscription_offer_code(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateSubscriptionOfferCode`.
async fn handle_create_subscription_offer_code(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubscriptionID string \"json:\\\"subscription_id\\\"\"; Name string \"json:\\\"name\\\"\"; CustomerEligibilities []string \"json:\\\"customer_eligibilities\\\"\"; OfferEligibility string \"json:\\\"offer_eligibility\\\"\"; Duration string \"json:\\\"duration\\\"\"; OfferMode string \"json:\\\"offer_mode\\\"\"; NumberOfPeriods int \"json:\\\"number_of_periods\\\"\" }", "") {
        subscription_id: String => "subscription_id";
        name: String => "name";
        customer_eligibilities: GoSlice<String> => "customer_eligibilities";
        offer_eligibility: String => "offer_eligibility";
        duration: String => "duration";
        offer_mode: String => "offer_mode";
        number_of_periods: i64 => "number_of_periods";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.subscription_id.is_empty() || params.name.is_empty() {
        return Ok(error_result("subscription_id and name are required"));
    }

    let req = SubscriptionOfferCodeCreateRequest {
        data: SubscriptionOfferCodeCreateData {
            type_: "subscriptionOfferCodes".into(),
            attributes: SubscriptionOfferCodeCreateAttributes {
                name: params.name.clone(),
                customer_eligibilities: params.customer_eligibilities.clone(),
                offer_eligibility: params.offer_eligibility.clone(),
                duration: params.duration.clone(),
                offer_mode: params.offer_mode.clone(),
                number_of_periods: params.number_of_periods,
            },
            relationships: SubscriptionOfferCodeCreateRelationships {
                subscription: RelationshipData {
                    data: ResourceIdentifier { type_: "subscriptions".into(), id: params.subscription_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_subscription_offer_code(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create subscription offer code: {e}"))),
    };

    Ok(data_result(&format!("Subscription offer code created:\n{}", format_subscription_offer_code(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleUpdateSubscriptionOfferCode`.
async fn handle_update_subscription_offer_code(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferCodeID string \"json:\\\"offer_code_id\\\"\"; Active *bool \"json:\\\"active\\\"\" }", "") {
        offer_code_id: String => "offer_code_id";
        active: Option<bool> => "active";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.offer_code_id.is_empty() {
        return Ok(error_result("offer_code_id is required"));
    }

    let req = SubscriptionOfferCodeUpdateRequest {
        data: SubscriptionOfferCodeUpdateData {
            type_: "subscriptionOfferCodes".into(),
            id: params.offer_code_id.clone(),
            attributes: SubscriptionOfferCodeUpdateAttributes { active: params.active },
        },
    };

    let resp = match c.update_subscription_offer_code(&params.offer_code_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update subscription offer code: {e}"))),
    };

    Ok(data_result(&format!("Subscription offer code updated:\n{}", format_subscription_offer_code(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleListWinBackOffers`.
async fn handle_list_win_back_offers(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubscriptionID string \"json:\\\"subscription_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        subscription_id: String => "subscription_id";
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

    if params.subscription_id.is_empty() {
        return Ok(error_result("subscription_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_win_back_offers(&params.subscription_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list win-back offers: {e}"))),
    };

    Ok(list_result(&format_win_back_offers(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetWinBackOffer`.
async fn handle_get_win_back_offer(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferID string \"json:\\\"offer_id\\\"\" }", "") {
        offer_id: String => "offer_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.offer_id.is_empty() {
        return Ok(error_result("offer_id is required"));
    }

    let resp = match c.get_win_back_offer(&params.offer_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get win-back offer: {e}"))),
    };

    Ok(data_result(&format_win_back_offer(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateWinBackOffer`.
async fn handle_create_win_back_offer(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubscriptionID string \"json:\\\"subscription_id\\\"\"; ReferenceName string \"json:\\\"reference_name\\\"\"; OfferID string \"json:\\\"offer_id\\\"\"; Duration string \"json:\\\"duration\\\"\"; OfferMode string \"json:\\\"offer_mode\\\"\"; PeriodCount int \"json:\\\"period_count\\\"\"; Priority string \"json:\\\"priority\\\"\"; PromotionIntent string \"json:\\\"promotion_intent\\\"\"; PriceIDs []string \"json:\\\"price_ids\\\"\" }", "") {
        subscription_id: String => "subscription_id";
        reference_name: String => "reference_name";
        offer_id: String => "offer_id";
        duration: String => "duration";
        offer_mode: String => "offer_mode";
        period_count: i64 => "period_count";
        priority: String => "priority";
        promotion_intent: String => "promotion_intent";
        price_ids: GoSlice<String> => "price_ids";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.subscription_id.is_empty() || params.reference_name.is_empty() || params.offer_id.is_empty() {
        return Ok(error_result("subscription_id, reference_name, and offer_id are required"));
    }

    let mut prices: GoSlice<ResourceIdentifier> = GoSlice::nil();
    for pid in params.price_ids.iter() {
        prices.push(ResourceIdentifier { type_: "winBackOfferPrices".into(), id: pid.clone() });
    }

    let req = WinBackOfferCreateRequest {
        data: WinBackOfferCreateData {
            type_: "winBackOffers".into(),
            attributes: WinBackOfferCreateAttributes {
                reference_name: params.reference_name.clone(),
                offer_id: params.offer_id.clone(),
                duration: params.duration.clone(),
                offer_mode: params.offer_mode.clone(),
                period_count: params.period_count,
                priority: params.priority.clone(),
                promotion_intent: params.promotion_intent.clone(),
                ..Default::default()
            },
            relationships: WinBackOfferCreateRelationships {
                subscription: RelationshipData {
                    data: ResourceIdentifier { type_: "subscriptions".into(), id: params.subscription_id.clone() },
                },
                prices: RelationshipDataList { data: prices },
            },
        },
    };

    let resp = match c.create_win_back_offer(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create win-back offer: {e}"))),
    };

    Ok(data_result(&format!("Win-back offer created:\n{}", format_win_back_offer(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleUpdateWinBackOffer`.
async fn handle_update_win_back_offer(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferID string \"json:\\\"offer_id\\\"\"; Priority string \"json:\\\"priority\\\"\"; PromotionIntent string \"json:\\\"promotion_intent\\\"\" }", "") {
        offer_id: String => "offer_id";
        priority: String => "priority";
        promotion_intent: String => "promotion_intent";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.offer_id.is_empty() {
        return Ok(error_result("offer_id is required"));
    }

    let req = WinBackOfferUpdateRequest {
        data: WinBackOfferUpdateData {
            type_: "winBackOffers".into(),
            id: params.offer_id.clone(),
            attributes: WinBackOfferUpdateAttributes {
                priority: params.priority.clone(),
                promotion_intent: params.promotion_intent.clone(),
                ..Default::default()
            },
        },
    };

    let resp = match c.update_win_back_offer(&params.offer_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update win-back offer: {e}"))),
    };

    Ok(data_result(&format!("Win-back offer updated:\n{}", format_win_back_offer(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleDeleteWinBackOffer`.
async fn handle_delete_win_back_offer(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferID string \"json:\\\"offer_id\\\"\" }", "") {
        offer_id: String => "offer_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.offer_id.is_empty() {
        return Ok(error_result("offer_id is required"));
    }

    if let Err(e) = c.delete_win_back_offer(&params.offer_id).await {
        return Ok(error_result(format!("Failed to delete win-back offer: {e}")));
    }

    Ok(CallToolResult::success("Win-back offer deleted"))
}

/// Ported from `formatPromotedPurchases`.
fn format_promoted_purchases(purchases: &[PromotedPurchase]) -> String {
    if purchases.is_empty() {
        return "No promoted purchases found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} promoted purchases:\n\n", purchases.len());

    for p in purchases {
        sb.push_str(&format_promoted_purchase(p));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatPromotedPurchase`.
fn format_promoted_purchase(p: &PromotedPurchase) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", p.id);
    let _ = write!(sb, "Enabled: {}\n", p.attributes.enabled);
    let _ = write!(sb, "Visible For All Users: {}\n", p.attributes.visible_for_all_users);
    if !p.attributes.state.is_empty() {
        let _ = write!(sb, "State: {}\n", p.attributes.state);
    }
    sb
}

/// Ported from `formatSubscriptionOfferCodes`.
fn format_subscription_offer_codes(codes: &[SubscriptionOfferCode]) -> String {
    if codes.is_empty() {
        return "No subscription offer codes found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} subscription offer codes:\n\n", codes.len());

    for c in codes {
        sb.push_str(&format_subscription_offer_code(c));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatSubscriptionOfferCode`.
fn format_subscription_offer_code(c: &SubscriptionOfferCode) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", c.id);
    let _ = write!(sb, "Name: {}\n", c.attributes.name);
    let _ = write!(sb, "Active: {}\n", c.attributes.active);
    if !c.attributes.customer_eligibilities.is_empty() {
        let _ = write!(sb, "Customer Eligibilities: {}\n", c.attributes.customer_eligibilities.join(", "));
    }
    if !c.attributes.offer_mode.is_empty() {
        let _ = write!(sb, "Offer Mode: {}\n", c.attributes.offer_mode);
    }
    if c.attributes.number_of_periods > 0 {
        let _ = write!(sb, "Periods: {}\n", c.attributes.number_of_periods);
    }
    if c.attributes.total_number_of_codes > 0 {
        let _ = write!(sb, "Total Codes: {}\n", c.attributes.total_number_of_codes);
    }
    sb
}

/// Ported from `formatWinBackOffers`.
fn format_win_back_offers(offers: &[WinBackOffer]) -> String {
    if offers.is_empty() {
        return "No win-back offers found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} win-back offers:\n\n", offers.len());

    for o in offers {
        sb.push_str(&format_win_back_offer(o));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatWinBackOffer`.
fn format_win_back_offer(o: &WinBackOffer) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", o.id);
    let _ = write!(sb, "Reference Name: {}\n", o.attributes.reference_name);
    let _ = write!(sb, "Offer ID: {}\n", o.attributes.offer_id);
    if !o.attributes.offer_mode.is_empty() {
        let _ = write!(sb, "Offer Mode: {}\n", o.attributes.offer_mode);
    }
    if !o.attributes.duration.is_empty() {
        let _ = write!(sb, "Duration: {}\n", o.attributes.duration);
    }
    if o.attributes.period_count > 0 {
        let _ = write!(sb, "Period Count: {}\n", o.attributes.period_count);
    }
    if !o.attributes.priority.is_empty() {
        let _ = write!(sb, "Priority: {}\n", o.attributes.priority);
    }
    if !o.attributes.promotion_intent.is_empty() {
        let _ = write!(sb, "Promotion Intent: {}\n", o.attributes.promotion_intent);
    }
    sb
}
