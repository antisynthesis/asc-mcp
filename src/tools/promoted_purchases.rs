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
    todo!("port handleListPromotedPurchases from internal/asc/tools/promoted_purchases.go")
}

/// Ported from `Registry.handleGetPromotedPurchase`.
async fn handle_get_promoted_purchase(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PromotedPurchaseID string \"json:\\\"promoted_purchase_id\\\"\" }", "") {
        promoted_purchase_id: String => "promoted_purchase_id";
    }
    }
    todo!("port handleGetPromotedPurchase from internal/asc/tools/promoted_purchases.go")
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
    todo!("port handleCreatePromotedPurchase from internal/asc/tools/promoted_purchases.go")
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
    todo!("port handleUpdatePromotedPurchase from internal/asc/tools/promoted_purchases.go")
}

/// Ported from `Registry.handleDeletePromotedPurchase`.
async fn handle_delete_promoted_purchase(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PromotedPurchaseID string \"json:\\\"promoted_purchase_id\\\"\" }", "") {
        promoted_purchase_id: String => "promoted_purchase_id";
    }
    }
    todo!("port handleDeletePromotedPurchase from internal/asc/tools/promoted_purchases.go")
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
    todo!("port handleListSubscriptionOfferCodes from internal/asc/tools/promoted_purchases.go")
}

/// Ported from `Registry.handleGetSubscriptionOfferCode`.
async fn handle_get_subscription_offer_code(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferCodeID string \"json:\\\"offer_code_id\\\"\" }", "") {
        offer_code_id: String => "offer_code_id";
    }
    }
    todo!("port handleGetSubscriptionOfferCode from internal/asc/tools/promoted_purchases.go")
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
    todo!("port handleCreateSubscriptionOfferCode from internal/asc/tools/promoted_purchases.go")
}

/// Ported from `Registry.handleUpdateSubscriptionOfferCode`.
async fn handle_update_subscription_offer_code(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferCodeID string \"json:\\\"offer_code_id\\\"\"; Active *bool \"json:\\\"active\\\"\" }", "") {
        offer_code_id: String => "offer_code_id";
        active: Option<bool> => "active";
    }
    }
    todo!("port handleUpdateSubscriptionOfferCode from internal/asc/tools/promoted_purchases.go")
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
    todo!("port handleListWinBackOffers from internal/asc/tools/promoted_purchases.go")
}

/// Ported from `Registry.handleGetWinBackOffer`.
async fn handle_get_win_back_offer(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferID string \"json:\\\"offer_id\\\"\" }", "") {
        offer_id: String => "offer_id";
    }
    }
    todo!("port handleGetWinBackOffer from internal/asc/tools/promoted_purchases.go")
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
    todo!("port handleCreateWinBackOffer from internal/asc/tools/promoted_purchases.go")
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
    todo!("port handleUpdateWinBackOffer from internal/asc/tools/promoted_purchases.go")
}

/// Ported from `Registry.handleDeleteWinBackOffer`.
async fn handle_delete_win_back_offer(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferID string \"json:\\\"offer_id\\\"\" }", "") {
        offer_id: String => "offer_id";
    }
    }
    todo!("port handleDeleteWinBackOffer from internal/asc/tools/promoted_purchases.go")
}

