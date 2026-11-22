//! Ported from internal/asc/tools/iap_offer_codes.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerInAppPurchaseOfferCodeTools`.
pub(super) fn register_in_app_purchase_offer_code_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_in_app_purchase_offer_codes",
            "description": "List the offer codes of an in-app purchase",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"APPROVED\"]} becomes filter[state]=APPROVED."
                    },
                    "iap_id": {
                        "type": "string",
                        "description": "The in-app purchase ID"
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
                        "description": "Maximum number of results to return (default 50)"
                    }
                },
                "required": [
                    "iap_id"
                ]
            },
            "annotations": {
                "title": "List In App Purchase Offer Codes",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_in_app_purchase_offer_codes),
    );
    r.register(
        tool(r##"{
            "name": "get_in_app_purchase_offer_code",
            "description": "Get an in-app purchase offer code and its redemption counts",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "offer_code_id": {
                        "type": "string",
                        "description": "The in-app purchase offer code ID"
                    }
                },
                "required": [
                    "offer_code_id"
                ]
            },
            "annotations": {
                "title": "Get In App Purchase Offer Code",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_in_app_purchase_offer_code),
    );
    r.register(
        tool(r##"{
            "name": "create_in_app_purchase_offer_code",
            "description": "Create an offer code for an in-app purchase. Supply one price per territory you want the offer available in; use list_in_app_purchase_price_points to find price point IDs.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "customer_eligibilities": {
                        "type": "array",
                        "description": "Customer eligibilities: NEW, EXISTING, EXPIRED",
                        "items": {
                            "type": "string"
                        }
                    },
                    "iap_id": {
                        "type": "string",
                        "description": "The in-app purchase ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "The offer code reference name"
                    },
                    "prices": {
                        "type": "array",
                        "description": "Offer prices, each an object with territory_id and price_point_id, e.g. [{\"territory_id\": \"USA\", \"price_point_id\": \"eyJz...\"}].",
                        "items": {
                            "type": "object"
                        }
                    }
                },
                "required": [
                    "iap_id",
                    "name",
                    "customer_eligibilities",
                    "prices"
                ]
            },
            "annotations": {
                "title": "Create In App Purchase Offer Code",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_in_app_purchase_offer_code),
    );
    r.register(
        tool(r##"{
            "name": "update_in_app_purchase_offer_code",
            "description": "Activate or deactivate an in-app purchase offer code",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "active": {
                        "type": "boolean",
                        "description": "Whether the offer code is active"
                    },
                    "offer_code_id": {
                        "type": "string",
                        "description": "The in-app purchase offer code ID"
                    }
                },
                "required": [
                    "offer_code_id",
                    "active"
                ]
            },
            "annotations": {
                "title": "Update In App Purchase Offer Code",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_in_app_purchase_offer_code),
    );
    r.register(
        tool(r##"{
            "name": "list_in_app_purchase_offer_code_prices",
            "description": "List the per-territory prices of an in-app purchase offer code",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"APPROVED\"]} becomes filter[state]=APPROVED."
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
                        "description": "Maximum number of results to return (default 50)"
                    },
                    "offer_code_id": {
                        "type": "string",
                        "description": "The in-app purchase offer code ID"
                    }
                },
                "required": [
                    "offer_code_id"
                ]
            },
            "annotations": {
                "title": "List In App Purchase Offer Code Prices",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_in_app_purchase_offer_code_prices),
    );
    r.register(
        tool(r##"{
            "name": "list_in_app_purchase_offer_code_custom_codes",
            "description": "List the custom code batches of an in-app purchase offer code",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"APPROVED\"]} becomes filter[state]=APPROVED."
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
                        "description": "Maximum number of results to return (default 50)"
                    },
                    "offer_code_id": {
                        "type": "string",
                        "description": "The in-app purchase offer code ID"
                    }
                },
                "required": [
                    "offer_code_id"
                ]
            },
            "annotations": {
                "title": "List In App Purchase Offer Code Custom Codes",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_in_app_purchase_offer_code_custom_codes),
    );
    r.register(
        tool(r##"{
            "name": "create_in_app_purchase_offer_code_custom_code",
            "description": "Issue a custom (memorable) code batch for an in-app purchase offer code",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "custom_code": {
                        "type": "string",
                        "description": "The code customers redeem, e.g. LAUNCH2026"
                    },
                    "expiration_date": {
                        "type": "string",
                        "description": "Expiration date in YYYY-MM-DD format"
                    },
                    "number_of_codes": {
                        "type": "integer",
                        "description": "How many times the code can be redeemed"
                    },
                    "offer_code_id": {
                        "type": "string",
                        "description": "The in-app purchase offer code ID"
                    }
                },
                "required": [
                    "offer_code_id",
                    "custom_code",
                    "number_of_codes"
                ]
            },
            "annotations": {
                "title": "Create In App Purchase Offer Code Custom Code",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_in_app_purchase_offer_code_custom_code),
    );
    r.register(
        tool(r##"{
            "name": "update_in_app_purchase_offer_code_custom_code",
            "description": "Activate or deactivate a custom code batch",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "active": {
                        "type": "boolean",
                        "description": "Whether the custom code is active"
                    },
                    "custom_code_id": {
                        "type": "string",
                        "description": "The custom code batch ID"
                    }
                },
                "required": [
                    "custom_code_id",
                    "active"
                ]
            },
            "annotations": {
                "title": "Update In App Purchase Offer Code Custom Code",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_in_app_purchase_offer_code_custom_code),
    );
    r.register(
        tool(r##"{
            "name": "list_in_app_purchase_offer_code_one_time_use_codes",
            "description": "List the one-time-use code batches of an in-app purchase offer code",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"APPROVED\"]} becomes filter[state]=APPROVED."
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
                        "description": "Maximum number of results to return (default 50)"
                    },
                    "offer_code_id": {
                        "type": "string",
                        "description": "The in-app purchase offer code ID"
                    }
                },
                "required": [
                    "offer_code_id"
                ]
            },
            "annotations": {
                "title": "List In App Purchase Offer Code One Time Use Codes",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_in_app_purchase_offer_code_one_time_use_codes),
    );
    r.register(
        tool(r##"{
            "name": "create_in_app_purchase_offer_code_one_time_use_code",
            "description": "Generate a batch of one-time-use codes for an in-app purchase offer code. Download the generated codes with get_in_app_purchase_offer_code_one_time_use_code_values.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "environment": {
                        "type": "string",
                        "description": "Environment: PRODUCTION or SANDBOX"
                    },
                    "expiration_date": {
                        "type": "string",
                        "description": "Expiration date in YYYY-MM-DD format"
                    },
                    "number_of_codes": {
                        "type": "integer",
                        "description": "How many unique codes to generate"
                    },
                    "offer_code_id": {
                        "type": "string",
                        "description": "The in-app purchase offer code ID"
                    }
                },
                "required": [
                    "offer_code_id",
                    "number_of_codes",
                    "expiration_date"
                ]
            },
            "annotations": {
                "title": "Create In App Purchase Offer Code One Time Use Code",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_in_app_purchase_offer_code_one_time_use_code),
    );
    r.register(
        tool(r##"{
            "name": "update_in_app_purchase_offer_code_one_time_use_code",
            "description": "Activate or deactivate a one-time-use code batch",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "active": {
                        "type": "boolean",
                        "description": "Whether the code batch is active"
                    },
                    "one_time_use_code_id": {
                        "type": "string",
                        "description": "The one-time-use code batch ID"
                    }
                },
                "required": [
                    "one_time_use_code_id",
                    "active"
                ]
            },
            "annotations": {
                "title": "Update In App Purchase Offer Code One Time Use Code",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_in_app_purchase_offer_code_one_time_use_code),
    );
    r.register(
        tool(r##"{
            "name": "get_in_app_purchase_offer_code_one_time_use_code_values",
            "description": "Download the generated codes of a one-time-use batch. Apple returns them as CSV.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "one_time_use_code_id": {
                        "type": "string",
                        "description": "The one-time-use code batch ID"
                    }
                },
                "required": [
                    "one_time_use_code_id"
                ]
            },
            "annotations": {
                "title": "Get In App Purchase Offer Code One Time Use Code Values",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_in_app_purchase_offer_code_one_time_use_code_values),
    );
}

/// Ported from `Registry.handleListInAppPurchaseOfferCodes`.
async fn handle_list_in_app_purchase_offer_codes(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\"; tools.commerceListParams }", "") {
        iap_id: String => "iap_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListInAppPurchaseOfferCodes from internal/asc/tools/iap_offer_codes.go")
}

/// Ported from `Registry.handleGetInAppPurchaseOfferCode`.
async fn handle_get_in_app_purchase_offer_code(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferCodeID string \"json:\\\"offer_code_id\\\"\" }", "") {
        offer_code_id: String => "offer_code_id";
    }
    }
    todo!("port handleGetInAppPurchaseOfferCode from internal/asc/tools/iap_offer_codes.go")
}

/// Ported from `Registry.handleCreateInAppPurchaseOfferCode`.
async fn handle_create_in_app_purchase_offer_code(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    pub struct ParamsPrices ("struct { TerritoryID string \"json:\\\"territory_id\\\"\"; PricePointID string \"json:\\\"price_point_id\\\"\" }", "") {
        pub territory_id: String => "territory_id";
        pub price_point_id: String => "price_point_id";
    }
    }
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\"; Name string \"json:\\\"name\\\"\"; CustomerEligibilities []string \"json:\\\"customer_eligibilities\\\"\"; Prices []struct { TerritoryID string \"json:\\\"territory_id\\\"\"; PricePointID string \"json:\\\"price_point_id\\\"\" } \"json:\\\"prices\\\"\" }", "") {
        iap_id: String => "iap_id";
        name: String => "name";
        customer_eligibilities: GoSlice<String> => "customer_eligibilities";
        prices: GoSlice<ParamsPrices> => "prices";
    }
    }
    todo!("port handleCreateInAppPurchaseOfferCode from internal/asc/tools/iap_offer_codes.go")
}

/// Ported from `Registry.handleUpdateInAppPurchaseOfferCode`.
async fn handle_update_in_app_purchase_offer_code(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferCodeID string \"json:\\\"offer_code_id\\\"\"; Active *bool \"json:\\\"active\\\"\" }", "") {
        offer_code_id: String => "offer_code_id";
        active: Option<bool> => "active";
    }
    }
    todo!("port handleUpdateInAppPurchaseOfferCode from internal/asc/tools/iap_offer_codes.go")
}

/// Ported from `Registry.handleListInAppPurchaseOfferCodePrices`.
async fn handle_list_in_app_purchase_offer_code_prices(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferCodeID string \"json:\\\"offer_code_id\\\"\"; tools.commerceListParams }", "") {
        offer_code_id: String => "offer_code_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListInAppPurchaseOfferCodePrices from internal/asc/tools/iap_offer_codes.go")
}

/// Ported from `Registry.handleListInAppPurchaseOfferCodeCustomCodes`.
async fn handle_list_in_app_purchase_offer_code_custom_codes(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferCodeID string \"json:\\\"offer_code_id\\\"\"; tools.commerceListParams }", "") {
        offer_code_id: String => "offer_code_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListInAppPurchaseOfferCodeCustomCodes from internal/asc/tools/iap_offer_codes.go")
}

/// Ported from `Registry.handleCreateInAppPurchaseOfferCodeCustomCode`.
async fn handle_create_in_app_purchase_offer_code_custom_code(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferCodeID string \"json:\\\"offer_code_id\\\"\"; CustomCode string \"json:\\\"custom_code\\\"\"; NumberOfCodes int \"json:\\\"number_of_codes\\\"\"; ExpirationDate string \"json:\\\"expiration_date\\\"\" }", "") {
        offer_code_id: String => "offer_code_id";
        custom_code: String => "custom_code";
        number_of_codes: i64 => "number_of_codes";
        expiration_date: String => "expiration_date";
    }
    }
    todo!("port handleCreateInAppPurchaseOfferCodeCustomCode from internal/asc/tools/iap_offer_codes.go")
}

/// Ported from `Registry.handleUpdateInAppPurchaseOfferCodeCustomCode`.
async fn handle_update_in_app_purchase_offer_code_custom_code(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { CustomCodeID string \"json:\\\"custom_code_id\\\"\"; Active *bool \"json:\\\"active\\\"\" }", "") {
        custom_code_id: String => "custom_code_id";
        active: Option<bool> => "active";
    }
    }
    todo!("port handleUpdateInAppPurchaseOfferCodeCustomCode from internal/asc/tools/iap_offer_codes.go")
}

/// Ported from `Registry.handleListInAppPurchaseOfferCodeOneTimeUseCodes`.
async fn handle_list_in_app_purchase_offer_code_one_time_use_codes(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferCodeID string \"json:\\\"offer_code_id\\\"\"; tools.commerceListParams }", "") {
        offer_code_id: String => "offer_code_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListInAppPurchaseOfferCodeOneTimeUseCodes from internal/asc/tools/iap_offer_codes.go")
}

/// Ported from `Registry.handleCreateInAppPurchaseOfferCodeOneTimeUseCode`.
async fn handle_create_in_app_purchase_offer_code_one_time_use_code(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferCodeID string \"json:\\\"offer_code_id\\\"\"; NumberOfCodes int \"json:\\\"number_of_codes\\\"\"; ExpirationDate string \"json:\\\"expiration_date\\\"\"; Environment string \"json:\\\"environment\\\"\" }", "") {
        offer_code_id: String => "offer_code_id";
        number_of_codes: i64 => "number_of_codes";
        expiration_date: String => "expiration_date";
        environment: String => "environment";
    }
    }
    todo!("port handleCreateInAppPurchaseOfferCodeOneTimeUseCode from internal/asc/tools/iap_offer_codes.go")
}

/// Ported from `Registry.handleUpdateInAppPurchaseOfferCodeOneTimeUseCode`.
async fn handle_update_in_app_purchase_offer_code_one_time_use_code(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OneTimeUseCodeID string \"json:\\\"one_time_use_code_id\\\"\"; Active *bool \"json:\\\"active\\\"\" }", "") {
        one_time_use_code_id: String => "one_time_use_code_id";
        active: Option<bool> => "active";
    }
    }
    todo!("port handleUpdateInAppPurchaseOfferCodeOneTimeUseCode from internal/asc/tools/iap_offer_codes.go")
}

/// Ported from `Registry.handleGetInAppPurchaseOfferCodeOneTimeUseCodeValues`.
async fn handle_get_in_app_purchase_offer_code_one_time_use_code_values(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OneTimeUseCodeID string \"json:\\\"one_time_use_code_id\\\"\" }", "") {
        one_time_use_code_id: String => "one_time_use_code_id";
    }
    }
    todo!("port handleGetInAppPurchaseOfferCodeOneTimeUseCodeValues from internal/asc/tools/iap_offer_codes.go")
}

