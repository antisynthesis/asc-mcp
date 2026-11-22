//! Ported from internal/asc/tools/commerce_pricing.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerCommercePricingTools`.
pub(super) fn register_commerce_pricing_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_in_app_purchase_price_points",
            "description": "List the price points available to an in-app purchase. Filter by territory, e.g. {\"territory\": [\"USA\"]}.",
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
                "title": "List In App Purchase Price Points",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_in_app_purchase_price_points),
    );
    r.register(
        tool(r##"{
            "name": "get_in_app_purchase_price_schedule",
            "description": "Get the price schedule of an in-app purchase",
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
                "title": "Get In App Purchase Price Schedule",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_in_app_purchase_price_schedule),
    );
    r.register(
        tool(r##"{
            "name": "set_in_app_purchase_price_schedule",
            "description": "Replace an in-app purchase's price schedule. Prices in territories you do not list are derived automatically from the base territory.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "base_territory_id": {
                        "type": "string",
                        "description": "The territory the automatic prices are derived from, e.g. USA"
                    },
                    "iap_id": {
                        "type": "string",
                        "description": "The in-app purchase ID"
                    },
                    "prices": {
                        "type": "array",
                        "description": "Manual prices, each an object with price_point_id and optional start_date and end_date (YYYY-MM-DD), e.g. [{\"price_point_id\": \"eyJz...\"}].",
                        "items": {
                            "type": "object"
                        }
                    }
                },
                "required": [
                    "iap_id",
                    "base_territory_id",
                    "prices"
                ]
            },
            "annotations": {
                "title": "Set In App Purchase Price Schedule",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_set_in_app_purchase_price_schedule),
    );
    r.register(
        tool(r##"{
            "name": "list_in_app_purchase_price_schedule_prices",
            "description": "List the prices of an in-app purchase price schedule. Manual prices are the ones set explicitly; automatic prices are the ones Apple derived for other territories.",
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
                    "price_schedule_id": {
                        "type": "string",
                        "description": "The in-app purchase price schedule ID"
                    },
                    "price_type": {
                        "type": "string",
                        "description": "Which prices to list: MANUAL (default) or AUTOMATIC"
                    }
                },
                "required": [
                    "price_schedule_id"
                ]
            },
            "annotations": {
                "title": "List In App Purchase Price Schedule Prices",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_in_app_purchase_price_schedule_prices),
    );
    r.register(
        tool(r##"{
            "name": "get_in_app_purchase_availability",
            "description": "Get the territory availability of an in-app purchase",
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
                "title": "Get In App Purchase Availability",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_in_app_purchase_availability),
    );
    r.register(
        tool(r##"{
            "name": "set_in_app_purchase_availability",
            "description": "Set the territories an in-app purchase is available in",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "available_in_new_territories": {
                        "type": "boolean",
                        "description": "Whether the in-app purchase becomes available automatically in territories Apple adds later"
                    },
                    "iap_id": {
                        "type": "string",
                        "description": "The in-app purchase ID"
                    },
                    "territory_ids": {
                        "type": "array",
                        "description": "Territory IDs the in-app purchase is available in, e.g. [\"USA\", \"CAN\"]",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "iap_id",
                    "territory_ids"
                ]
            },
            "annotations": {
                "title": "Set In App Purchase Availability",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_set_in_app_purchase_availability),
    );
    r.register(
        tool(r##"{
            "name": "list_in_app_purchase_available_territories",
            "description": "List the territories covered by an in-app purchase availability",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "availability_id": {
                        "type": "string",
                        "description": "The in-app purchase availability ID"
                    },
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
                    }
                },
                "required": [
                    "availability_id"
                ]
            },
            "annotations": {
                "title": "List In App Purchase Available Territories",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_in_app_purchase_available_territories),
    );
    r.register(
        tool(r##"{
            "name": "list_subscription_plan_availabilities",
            "description": "List the plan availabilities of a subscription, one per billing plan. Replaces the deprecated subscription availability resource.",
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
                "title": "List Subscription Plan Availabilities",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_subscription_plan_availabilities),
    );
    r.register(
        tool(r##"{
            "name": "get_subscription_plan_availability",
            "description": "Get a subscription plan availability",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "availability_id": {
                        "type": "string",
                        "description": "The subscription plan availability ID"
                    }
                },
                "required": [
                    "availability_id"
                ]
            },
            "annotations": {
                "title": "Get Subscription Plan Availability",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_subscription_plan_availability),
    );
    r.register(
        tool(r##"{
            "name": "create_subscription_plan_availability",
            "description": "Configure the territories one billing plan of a subscription is available in",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "available_in_new_territories": {
                        "type": "boolean",
                        "description": "Whether the plan becomes available automatically in territories Apple adds later"
                    },
                    "plan_type": {
                        "type": "string",
                        "description": "The billing plan: MONTHLY or UPFRONT"
                    },
                    "subscription_id": {
                        "type": "string",
                        "description": "The subscription ID"
                    },
                    "territory_ids": {
                        "type": "array",
                        "description": "Territory IDs the plan is available in, e.g. [\"USA\", \"CAN\"]",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "subscription_id",
                    "plan_type",
                    "territory_ids"
                ]
            },
            "annotations": {
                "title": "Create Subscription Plan Availability",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_subscription_plan_availability),
    );
    r.register(
        tool(r##"{
            "name": "update_subscription_plan_availability",
            "description": "Update a subscription plan's territory availability",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "availability_id": {
                        "type": "string",
                        "description": "The subscription plan availability ID"
                    },
                    "available_in_new_territories": {
                        "type": "boolean",
                        "description": "Whether the plan becomes available automatically in territories Apple adds later"
                    },
                    "territory_ids": {
                        "type": "array",
                        "description": "Replacement territory IDs the plan is available in",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "availability_id"
                ]
            },
            "annotations": {
                "title": "Update Subscription Plan Availability",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_subscription_plan_availability),
    );
    r.register(
        tool(r##"{
            "name": "list_subscription_plan_available_territories",
            "description": "List the territories covered by a subscription plan availability",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "availability_id": {
                        "type": "string",
                        "description": "The subscription plan availability ID"
                    },
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
                    }
                },
                "required": [
                    "availability_id"
                ]
            },
            "annotations": {
                "title": "List Subscription Plan Available Territories",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_subscription_plan_available_territories),
    );
    r.register(
        tool(r##"{
            "name": "get_subscription_price_point",
            "description": "Get a subscription price point",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "price_point_id": {
                        "type": "string",
                        "description": "The subscription price point ID"
                    }
                },
                "required": [
                    "price_point_id"
                ]
            },
            "annotations": {
                "title": "Get Subscription Price Point",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_subscription_price_point),
    );
    r.register(
        tool(r##"{
            "name": "list_subscription_price_point_equalizations",
            "description": "List the price points in other territories equivalent to a subscription price point",
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
                    "price_point_id": {
                        "type": "string",
                        "description": "The subscription price point ID"
                    }
                },
                "required": [
                    "price_point_id"
                ]
            },
            "annotations": {
                "title": "List Subscription Price Point Equalizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_subscription_price_point_equalizations),
    );
    r.register(
        tool(r##"{
            "name": "list_subscription_price_point_adjusted_equalizations",
            "description": "List equalized subscription price points adjusted for a pre-paid plan, so a monthly price point can be matched against the upfront plan's price point",
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
                        "description": "Additional JSON:API filters, e.g. {\"territory\": [\"USA\"]} becomes filter[territory]=USA."
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
                        "description": "Maximum number of results to return (default 100)"
                    },
                    "plan_type": {
                        "type": "string",
                        "description": "The billing plan the adjusted prices apply to: MONTHLY or UPFRONT"
                    },
                    "price_point_id": {
                        "type": "string",
                        "description": "The subscription price point ID to equalize from"
                    },
                    "upfront_price_point_id": {
                        "type": "string",
                        "description": "The upfront (pre-paid) price point ID the equalization is adjusted against"
                    }
                },
                "required": [
                    "price_point_id"
                ]
            },
            "annotations": {
                "title": "List Subscription Price Point Adjusted Equalizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_subscription_price_point_adjusted_equalizations),
    );
    r.register(
        tool(r##"{
            "name": "set_app_price_schedule",
            "description": "Replace an app's price schedule. Prices in territories you do not list are derived automatically from the base territory; pass an empty prices array to keep the app free.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The app ID"
                    },
                    "base_territory_id": {
                        "type": "string",
                        "description": "The territory the automatic prices are derived from, e.g. USA"
                    },
                    "prices": {
                        "type": "array",
                        "description": "Manual prices, each an object with price_point_id and optional start_date and end_date (YYYY-MM-DD), e.g. [{\"price_point_id\": \"eyJz...\", \"start_date\": \"2026-01-01\"}].",
                        "items": {
                            "type": "object"
                        }
                    }
                },
                "required": [
                    "app_id",
                    "base_territory_id"
                ]
            },
            "annotations": {
                "title": "Set App Price Schedule",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_set_app_price_schedule),
    );
    r.register(
        tool(r##"{
            "name": "get_app_price_point",
            "description": "Get an app price point",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "price_point_id": {
                        "type": "string",
                        "description": "The app price point ID"
                    }
                },
                "required": [
                    "price_point_id"
                ]
            },
            "annotations": {
                "title": "Get App Price Point",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_price_point),
    );
    r.register(
        tool(r##"{
            "name": "list_app_price_point_equalizations",
            "description": "List the app price points in other territories equivalent to a given price point",
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
                    "price_point_id": {
                        "type": "string",
                        "description": "The app price point ID"
                    }
                },
                "required": [
                    "price_point_id"
                ]
            },
            "annotations": {
                "title": "List App Price Point Equalizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_app_price_point_equalizations),
    );
}

go_struct! {
pub(crate) struct InlinePriceParams ("tools.inlinePriceParams", "inlinePriceParams") {
    pub(crate) price_point_id: String => "price_point_id";
    pub(crate) start_date: String => "start_date";
    pub(crate) end_date: String => "end_date";
}
}

/// Ported from `Registry.handleListInAppPurchasePricePoints`.
async fn handle_list_in_app_purchase_price_points(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\"; tools.commerceListParams }", "") {
        iap_id: String => "iap_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListInAppPurchasePricePoints from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleGetInAppPurchasePriceSchedule`.
async fn handle_get_in_app_purchase_price_schedule(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\" }", "") {
        iap_id: String => "iap_id";
    }
    }
    todo!("port handleGetInAppPurchasePriceSchedule from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleSetInAppPurchasePriceSchedule`.
async fn handle_set_in_app_purchase_price_schedule(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\"; BaseTerritoryID string \"json:\\\"base_territory_id\\\"\"; Prices []tools.inlinePriceParams \"json:\\\"prices\\\"\" }", "") {
        iap_id: String => "iap_id";
        base_territory_id: String => "base_territory_id";
        prices: GoSlice<InlinePriceParams> => "prices";
    }
    }
    todo!("port handleSetInAppPurchasePriceSchedule from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleListInAppPurchasePriceSchedulePrices`.
async fn handle_list_in_app_purchase_price_schedule_prices(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PriceScheduleID string \"json:\\\"price_schedule_id\\\"\"; PriceType string \"json:\\\"price_type\\\"\"; tools.commerceListParams }", "") {
        price_schedule_id: String => "price_schedule_id";
        price_type: String => "price_type";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListInAppPurchasePriceSchedulePrices from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleGetInAppPurchaseAvailability`.
async fn handle_get_in_app_purchase_availability(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\" }", "") {
        iap_id: String => "iap_id";
    }
    }
    todo!("port handleGetInAppPurchaseAvailability from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleSetInAppPurchaseAvailability`.
async fn handle_set_in_app_purchase_availability(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\"; TerritoryIDs []string \"json:\\\"territory_ids\\\"\"; AvailableInNewTerritories bool \"json:\\\"available_in_new_territories\\\"\" }", "") {
        iap_id: String => "iap_id";
        territory_ids: GoSlice<String> => "territory_ids";
        available_in_new_territories: bool => "available_in_new_territories";
    }
    }
    todo!("port handleSetInAppPurchaseAvailability from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleListInAppPurchaseAvailableTerritories`.
async fn handle_list_in_app_purchase_available_territories(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AvailabilityID string \"json:\\\"availability_id\\\"\"; tools.commerceListParams }", "") {
        availability_id: String => "availability_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListInAppPurchaseAvailableTerritories from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleListSubscriptionPlanAvailabilities`.
async fn handle_list_subscription_plan_availabilities(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubscriptionID string \"json:\\\"subscription_id\\\"\"; tools.commerceListParams }", "") {
        subscription_id: String => "subscription_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListSubscriptionPlanAvailabilities from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleGetSubscriptionPlanAvailability`.
async fn handle_get_subscription_plan_availability(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AvailabilityID string \"json:\\\"availability_id\\\"\" }", "") {
        availability_id: String => "availability_id";
    }
    }
    todo!("port handleGetSubscriptionPlanAvailability from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleCreateSubscriptionPlanAvailability`.
async fn handle_create_subscription_plan_availability(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubscriptionID string \"json:\\\"subscription_id\\\"\"; PlanType string \"json:\\\"plan_type\\\"\"; TerritoryIDs []string \"json:\\\"territory_ids\\\"\"; AvailableInNewTerritories *bool \"json:\\\"available_in_new_territories\\\"\" }", "") {
        subscription_id: String => "subscription_id";
        plan_type: String => "plan_type";
        territory_ids: GoSlice<String> => "territory_ids";
        available_in_new_territories: Option<bool> => "available_in_new_territories";
    }
    }
    todo!("port handleCreateSubscriptionPlanAvailability from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleUpdateSubscriptionPlanAvailability`.
async fn handle_update_subscription_plan_availability(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AvailabilityID string \"json:\\\"availability_id\\\"\"; TerritoryIDs []string \"json:\\\"territory_ids\\\"\"; AvailableInNewTerritories *bool \"json:\\\"available_in_new_territories\\\"\" }", "") {
        availability_id: String => "availability_id";
        territory_ids: GoSlice<String> => "territory_ids";
        available_in_new_territories: Option<bool> => "available_in_new_territories";
    }
    }
    todo!("port handleUpdateSubscriptionPlanAvailability from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleListSubscriptionPlanAvailableTerritories`.
async fn handle_list_subscription_plan_available_territories(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AvailabilityID string \"json:\\\"availability_id\\\"\"; tools.commerceListParams }", "") {
        availability_id: String => "availability_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListSubscriptionPlanAvailableTerritories from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleGetSubscriptionPricePoint`.
async fn handle_get_subscription_price_point(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PricePointID string \"json:\\\"price_point_id\\\"\" }", "") {
        price_point_id: String => "price_point_id";
    }
    }
    todo!("port handleGetSubscriptionPricePoint from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleListSubscriptionPricePointEqualizations`.
async fn handle_list_subscription_price_point_equalizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PricePointID string \"json:\\\"price_point_id\\\"\"; tools.commerceListParams }", "") {
        price_point_id: String => "price_point_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListSubscriptionPricePointEqualizations from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleListSubscriptionPricePointAdjustedEqualizations`.
async fn handle_list_subscription_price_point_adjusted_equalizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PricePointID string \"json:\\\"price_point_id\\\"\"; UpfrontPricePointID string \"json:\\\"upfront_price_point_id\\\"\"; PlanType string \"json:\\\"plan_type\\\"\"; tools.commerceListParams }", "") {
        price_point_id: String => "price_point_id";
        upfront_price_point_id: String => "upfront_price_point_id";
        plan_type: String => "plan_type";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListSubscriptionPricePointAdjustedEqualizations from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleSetAppPriceSchedule`.
async fn handle_set_app_price_schedule(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; BaseTerritoryID string \"json:\\\"base_territory_id\\\"\"; Prices []tools.inlinePriceParams \"json:\\\"prices\\\"\" }", "") {
        app_id: String => "app_id";
        base_territory_id: String => "base_territory_id";
        prices: GoSlice<InlinePriceParams> => "prices";
    }
    }
    todo!("port handleSetAppPriceSchedule from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleGetAppPricePoint`.
async fn handle_get_app_price_point(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PricePointID string \"json:\\\"price_point_id\\\"\" }", "") {
        price_point_id: String => "price_point_id";
    }
    }
    todo!("port handleGetAppPricePoint from internal/asc/tools/commerce_pricing.go")
}

/// Ported from `Registry.handleListAppPricePointEqualizations`.
async fn handle_list_app_price_point_equalizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PricePointID string \"json:\\\"price_point_id\\\"\"; tools.commerceListParams }", "") {
        price_point_id: String => "price_point_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListAppPricePointEqualizations from internal/asc/tools/commerce_pricing.go")
}

