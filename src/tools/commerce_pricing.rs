//! In-app purchase and subscription pricing and availability tools.

#![allow(unused_imports)]

use super::prelude::*;
use super::commerce_versions::rel;
use super::pricing::{format_app_price_point, format_app_price_points, format_subscription_price_point, format_subscription_price_points, format_territories};

/// Registers this module's tools in definition order.
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

json_struct! {
pub(crate) struct InlinePriceParams ("tools.inlinePriceParams", "inlinePriceParams") {
    pub(crate) price_point_id: String => "price_point_id";
    pub(crate) start_date: String => "start_date";
    pub(crate) end_date: String => "end_date";
}
}

/// Handles `list_in_app_purchase_price_points`.
async fn handle_list_in_app_purchase_price_points(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\"; tools.commerceListParams }", "") {
        iap_id: String => "iap_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.iap_id.is_empty() {
        return Ok(error_result("iap_id is required"));
    }
    let opts = params.commerce_list_params.opts(100);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_in_app_purchase_price_points(&params.iap_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list in-app purchase price points: {e}"))),
    };
    Ok(list_result(&format_in_app_purchase_price_points(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_in_app_purchase_price_schedule`.
async fn handle_get_in_app_purchase_price_schedule(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
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
    let resp = match c.get_in_app_purchase_price_schedule(&params.iap_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get in-app purchase price schedule: {e}"))),
    };
    Ok(data_result(&format!("In-App Purchase Price Schedule ID: {}\n", resp.data.id), &resp.data))
}

/// Handles `set_in_app_purchase_price_schedule`.
async fn handle_set_in_app_purchase_price_schedule(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\"; BaseTerritoryID string \"json:\\\"base_territory_id\\\"\"; Prices []tools.inlinePriceParams \"json:\\\"prices\\\"\" }", "") {
        iap_id: String => "iap_id";
        base_territory_id: String => "base_territory_id";
        prices: JsonSlice<InlinePriceParams> => "prices";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.iap_id.is_empty() {
        return Ok(error_result("iap_id is required"));
    }
    if params.base_territory_id.is_empty() {
        return Ok(error_result("base_territory_id is required"));
    }
    if params.prices.is_empty() {
        return Ok(error_result("prices is required"));
    }
    let mut price_refs: Vec<ResourceIdentifier> = Vec::with_capacity(params.prices.len());
    let mut included: Vec<InAppPurchasePriceInlineCreate> = Vec::with_capacity(params.prices.len());
    for (i, price) in params.prices.iter().enumerate() {
        if price.price_point_id.is_empty() {
            return Ok(error_result("each price requires price_point_id"));
        }
        let id = format!("price-{}", i + 1);
        price_refs.push(ResourceIdentifier { type_: "inAppPurchasePrices".into(), id: id.clone() });
        included.push(InAppPurchasePriceInlineCreate {
            type_: "inAppPurchasePrices".into(),
            id,
            attributes: inline_price_attributes(price),
            relationships: Some(InAppPurchasePriceInlineCreateRelationships {
                in_app_purchase_price_point: rel("inAppPurchasePricePoints", &price.price_point_id),
            }),
        });
    }
    let req = InAppPurchasePriceScheduleCreateRequest {
        data: InAppPurchasePriceScheduleCreateData {
            type_: "inAppPurchasePriceSchedules".into(),
            relationships: InAppPurchasePriceScheduleCreateRelationships {
                in_app_purchase: rel("inAppPurchases", &params.iap_id),
                base_territory: rel("territories", &params.base_territory_id),
                manual_prices: RelationshipDataList { data: JsonSlice::from(price_refs) },
            },
        },
        included: JsonSlice::from(included),
    };
    let resp = match c.create_in_app_purchase_price_schedule(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to set in-app purchase price schedule: {e}"))),
    };
    Ok(data_result(&format!("In-app purchase price schedule set: {}", resp.data.id), &resp.data))
}

/// Handles `list_in_app_purchase_price_schedule_prices`.
async fn handle_list_in_app_purchase_price_schedule_prices(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { PriceScheduleID string \"json:\\\"price_schedule_id\\\"\"; PriceType string \"json:\\\"price_type\\\"\"; tools.commerceListParams }", "") {
        price_schedule_id: String => "price_schedule_id";
        price_type: String => "price_type";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.price_schedule_id.is_empty() {
        return Ok(error_result("price_schedule_id is required"));
    }
    // strings.EqualFold against an all-ASCII target with no special fold
    // orbits reduces to an ASCII case-insensitive comparison.
    let automatic = params.price_type.eq_ignore_ascii_case("AUTOMATIC");
    let opts = params.commerce_list_params.opts(100);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_in_app_purchase_price_schedule_prices(&params.price_schedule_id, automatic, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list in-app purchase price schedule prices: {e}"))),
    };
    Ok(list_result(&format_in_app_purchase_prices(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_in_app_purchase_availability`.
async fn handle_get_in_app_purchase_availability(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
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
    let resp = match c.get_in_app_purchase_availability(&params.iap_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get in-app purchase availability: {e}"))),
    };
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", resp.data.id);
    let _ = writeln!(sb, "Available In New Territories: {}", resp.data.attributes.available_in_new_territories);
    // Best-effort: also show the territories the availability covers.
    if let Ok(territories) = c.list_in_app_purchase_available_territories(&resp.data.id, None).await
        && !territories.data.is_empty() {
            let _ = write!(sb, "\n{}", format_territories(&territories.data));
        }
    Ok(data_result(&sb, &resp.data))
}

/// Handles `set_in_app_purchase_availability`.
async fn handle_set_in_app_purchase_availability(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\"; TerritoryIDs []string \"json:\\\"territory_ids\\\"\"; AvailableInNewTerritories bool \"json:\\\"available_in_new_territories\\\"\" }", "") {
        iap_id: String => "iap_id";
        territory_ids: JsonSlice<String> => "territory_ids";
        available_in_new_territories: bool => "available_in_new_territories";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.iap_id.is_empty() {
        return Ok(error_result("iap_id is required"));
    }
    if params.territory_ids.is_empty() {
        return Ok(error_result("territory_ids is required"));
    }
    let req = InAppPurchaseAvailabilityCreateRequest {
        data: InAppPurchaseAvailabilityCreateData {
            type_: "inAppPurchaseAvailabilities".into(),
            attributes: InAppPurchaseAvailabilityAttributes { available_in_new_territories: params.available_in_new_territories },
            relationships: InAppPurchaseAvailabilityCreateRelationships {
                in_app_purchase: rel("inAppPurchases", &params.iap_id),
                available_territories: RelationshipDataList { data: territory_identifiers(&params.territory_ids) },
            },
        },
    };
    let resp = match c.create_in_app_purchase_availability(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to set in-app purchase availability: {e}"))),
    };
    Ok(data_result(&format!("In-app purchase availability set: {} ({} territories)", resp.data.id, params.territory_ids.len()), &resp.data))
}

/// Handles `list_in_app_purchase_available_territories`.
async fn handle_list_in_app_purchase_available_territories(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AvailabilityID string \"json:\\\"availability_id\\\"\"; tools.commerceListParams }", "") {
        availability_id: String => "availability_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.availability_id.is_empty() {
        return Ok(error_result("availability_id is required"));
    }
    let opts = params.commerce_list_params.opts(200);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_in_app_purchase_available_territories(&params.availability_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list in-app purchase available territories: {e}"))),
    };
    Ok(list_result(&format_territories(&resp.data), &resp.data, &resp.links))
}

/// Handles `list_subscription_plan_availabilities`.
async fn handle_list_subscription_plan_availabilities(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { SubscriptionID string \"json:\\\"subscription_id\\\"\"; tools.commerceListParams }", "") {
        subscription_id: String => "subscription_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.subscription_id.is_empty() {
        return Ok(error_result("subscription_id is required"));
    }
    let opts = params.commerce_list_params.opts(50);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_subscription_plan_availabilities(&params.subscription_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list subscription plan availabilities: {e}"))),
    };
    Ok(list_result(&format_subscription_plan_availabilities(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_subscription_plan_availability`.
async fn handle_get_subscription_plan_availability(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AvailabilityID string \"json:\\\"availability_id\\\"\" }", "") {
        availability_id: String => "availability_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.availability_id.is_empty() {
        return Ok(error_result("availability_id is required"));
    }
    let resp = match c.get_subscription_plan_availability(&params.availability_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get subscription plan availability: {e}"))),
    };
    let mut sb = format_subscription_plan_availability(&resp.data);
    // Best-effort: also show the territories the plan is available in.
    if let Ok(territories) = c.list_subscription_plan_available_territories(&resp.data.id, None).await
        && !territories.data.is_empty() {
            let _ = write!(sb, "\n{}", format_territories(&territories.data));
        }
    Ok(data_result(&sb, &resp.data))
}

/// Handles `create_subscription_plan_availability`.
async fn handle_create_subscription_plan_availability(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { SubscriptionID string \"json:\\\"subscription_id\\\"\"; PlanType string \"json:\\\"plan_type\\\"\"; TerritoryIDs []string \"json:\\\"territory_ids\\\"\"; AvailableInNewTerritories *bool \"json:\\\"available_in_new_territories\\\"\" }", "") {
        subscription_id: String => "subscription_id";
        plan_type: String => "plan_type";
        territory_ids: JsonSlice<String> => "territory_ids";
        available_in_new_territories: Option<bool> => "available_in_new_territories";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.subscription_id.is_empty() {
        return Ok(error_result("subscription_id is required"));
    }
    if params.plan_type.is_empty() {
        return Ok(error_result("plan_type is required"));
    }
    if params.territory_ids.is_empty() {
        return Ok(error_result("territory_ids is required"));
    }
    let req = SubscriptionPlanAvailabilityCreateRequest {
        data: SubscriptionPlanAvailabilityCreateData {
            type_: "subscriptionPlanAvailabilities".into(),
            attributes: SubscriptionPlanAvailabilityAttributes {
                plan_type: params.plan_type.clone(),
                available_in_new_territories: params.available_in_new_territories,
            },
            relationships: SubscriptionPlanAvailabilityCreateRelationships {
                subscription: rel("subscriptions", &params.subscription_id),
                available_territories: RelationshipDataList { data: territory_identifiers(&params.territory_ids) },
            },
        },
    };
    let resp = match c.create_subscription_plan_availability(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create subscription plan availability: {e}"))),
    };
    Ok(data_result(&format!("Subscription plan availability created:\n{}", format_subscription_plan_availability(&resp.data)), &resp.data))
}

/// Handles `update_subscription_plan_availability`.
async fn handle_update_subscription_plan_availability(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AvailabilityID string \"json:\\\"availability_id\\\"\"; TerritoryIDs []string \"json:\\\"territory_ids\\\"\"; AvailableInNewTerritories *bool \"json:\\\"available_in_new_territories\\\"\" }", "") {
        availability_id: String => "availability_id";
        territory_ids: JsonSlice<String> => "territory_ids";
        available_in_new_territories: Option<bool> => "available_in_new_territories";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.availability_id.is_empty() {
        return Ok(error_result("availability_id is required"));
    }
    if params.territory_ids.is_empty() && params.available_in_new_territories.is_none() {
        return Ok(error_result("one of territory_ids or available_in_new_territories is required"));
    }
    let mut data = SubscriptionPlanAvailabilityUpdateData {
        type_: "subscriptionPlanAvailabilities".into(),
        id: params.availability_id.clone(),
        ..Default::default()
    };
    if params.available_in_new_territories.is_some() {
        data.attributes = Some(SubscriptionPlanAvailabilityUpdateAttributes {
            available_in_new_territories: params.available_in_new_territories,
        });
    }
    if !params.territory_ids.is_empty() {
        data.relationships = Some(SubscriptionPlanAvailabilityUpdateRelationships {
            available_territories: Some(RelationshipDataList { data: territory_identifiers(&params.territory_ids) }),
        });
    }
    let req = SubscriptionPlanAvailabilityUpdateRequest { data };
    let resp = match c.update_subscription_plan_availability(&params.availability_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update subscription plan availability: {e}"))),
    };
    Ok(data_result(&format!("Subscription plan availability updated:\n{}", format_subscription_plan_availability(&resp.data)), &resp.data))
}

/// Handles `list_subscription_plan_available_territories`.
async fn handle_list_subscription_plan_available_territories(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AvailabilityID string \"json:\\\"availability_id\\\"\"; tools.commerceListParams }", "") {
        availability_id: String => "availability_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.availability_id.is_empty() {
        return Ok(error_result("availability_id is required"));
    }
    let opts = params.commerce_list_params.opts(200);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_subscription_plan_available_territories(&params.availability_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list subscription plan available territories: {e}"))),
    };
    Ok(list_result(&format_territories(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_subscription_price_point`.
async fn handle_get_subscription_price_point(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { PricePointID string \"json:\\\"price_point_id\\\"\" }", "") {
        price_point_id: String => "price_point_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.price_point_id.is_empty() {
        return Ok(error_result("price_point_id is required"));
    }
    let resp = match c.get_subscription_price_point(&params.price_point_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get subscription price point: {e}"))),
    };
    Ok(data_result(&format_subscription_price_point(&resp.data), &resp.data))
}

/// Handles `list_subscription_price_point_equalizations`.
async fn handle_list_subscription_price_point_equalizations(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { PricePointID string \"json:\\\"price_point_id\\\"\"; tools.commerceListParams }", "") {
        price_point_id: String => "price_point_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.price_point_id.is_empty() {
        return Ok(error_result("price_point_id is required"));
    }
    let opts = params.commerce_list_params.opts(100);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_subscription_price_point_equalizations(&params.price_point_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list subscription price point equalizations: {e}"))),
    };
    Ok(list_result(&format_subscription_price_points(&resp.data), &resp.data, &resp.links))
}

/// Handles `list_subscription_price_point_adjusted_equalizations`.
async fn handle_list_subscription_price_point_adjusted_equalizations(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { PricePointID string \"json:\\\"price_point_id\\\"\"; UpfrontPricePointID string \"json:\\\"upfront_price_point_id\\\"\"; PlanType string \"json:\\\"plan_type\\\"\"; tools.commerceListParams }", "") {
        price_point_id: String => "price_point_id";
        upfront_price_point_id: String => "upfront_price_point_id";
        plan_type: String => "plan_type";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.price_point_id.is_empty() {
        return Ok(error_result("price_point_id is required"));
    }
    // upfrontPricePointId and planType are ordinary JSON:API filters; fold
    // the dedicated arguments into whatever filter map the caller supplied.
    let mut filter: JsonMap<JsonSlice<String>> = JsonMap::from_map(params.commerce_list_params.filter.iter().map(|(k, v)| (k.clone(), v.clone())).collect());
    if !params.upfront_price_point_id.is_empty() {
        filter.insert("upfrontPricePointId", JsonSlice::from(vec![params.upfront_price_point_id.clone()]));
    }
    if !params.plan_type.is_empty() {
        filter.insert("planType", JsonSlice::from(vec![params.plan_type.clone()]));
    }
    params.commerce_list_params.filter = filter;
    let opts = params.commerce_list_params.opts(100);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_subscription_price_point_adjusted_equalizations(&params.price_point_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list subscription price point adjusted equalizations: {e}"))),
    };
    Ok(list_result(&format_subscription_price_points(&resp.data), &resp.data, &resp.links))
}

/// Handles `set_app_price_schedule`.
async fn handle_set_app_price_schedule(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; BaseTerritoryID string \"json:\\\"base_territory_id\\\"\"; Prices []tools.inlinePriceParams \"json:\\\"prices\\\"\" }", "") {
        app_id: String => "app_id";
        base_territory_id: String => "base_territory_id";
        prices: JsonSlice<InlinePriceParams> => "prices";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }
    if params.base_territory_id.is_empty() {
        return Ok(error_result("base_territory_id is required"));
    }
    let mut price_refs: Vec<ResourceIdentifier> = Vec::with_capacity(params.prices.len());
    let mut included: Vec<AppPriceInlineCreate> = Vec::with_capacity(params.prices.len());
    for (i, price) in params.prices.iter().enumerate() {
        if price.price_point_id.is_empty() {
            return Ok(error_result("each price requires price_point_id"));
        }
        let id = format!("price-{}", i + 1);
        price_refs.push(ResourceIdentifier { type_: "appPrices".into(), id: id.clone() });
        included.push(AppPriceInlineCreate {
            type_: "appPrices".into(),
            id,
            attributes: inline_price_attributes(price),
            relationships: Some(AppPriceInlineCreateRelationships { app_price_point: rel("appPricePoints", &price.price_point_id) }),
        });
    }
    let req = AppPriceScheduleCreateRequest {
        data: AppPriceScheduleCreateData {
            type_: "appPriceSchedules".into(),
            relationships: AppPriceScheduleCreateRelationships {
                app: rel("apps", &params.app_id),
                base_territory: rel("territories", &params.base_territory_id),
                manual_prices: RelationshipDataList { data: JsonSlice::from(price_refs) },
            },
        },
        included: JsonSlice::from(included),
    };
    let resp = match c.create_app_price_schedule(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to set app price schedule: {e}"))),
    };
    Ok(data_result(&format!("App price schedule set: {}", resp.data.id), &resp.data))
}

/// Handles `get_app_price_point`.
async fn handle_get_app_price_point(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { PricePointID string \"json:\\\"price_point_id\\\"\" }", "") {
        price_point_id: String => "price_point_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.price_point_id.is_empty() {
        return Ok(error_result("price_point_id is required"));
    }
    let resp = match c.get_app_price_point(&params.price_point_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get app price point: {e}"))),
    };
    Ok(data_result(&format_app_price_point(&resp.data), &resp.data))
}

/// Handles `list_app_price_point_equalizations`.
async fn handle_list_app_price_point_equalizations(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { PricePointID string \"json:\\\"price_point_id\\\"\"; tools.commerceListParams }", "") {
        price_point_id: String => "price_point_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.price_point_id.is_empty() {
        return Ok(error_result("price_point_id is required"));
    }
    let opts = params.commerce_list_params.opts(100);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_app_price_point_equalizations(&params.price_point_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list app price point equalizations: {e}"))),
    };
    Ok(list_result(&format_app_price_points(&resp.data), &resp.data, &resp.links))
}

/// The optional scheduling window of an inline
/// price, `None` when the caller gave neither date.
fn inline_price_attributes(price: &InlinePriceParams) -> Option<CommercePriceInlineAttributes> {
    if price.start_date.is_empty() && price.end_date.is_empty() {
        return None;
    }
    Some(CommercePriceInlineAttributes { start_date: price.start_date.clone(), end_date: price.end_date.clone() })
}

fn territory_identifiers(ids: &[String]) -> JsonSlice<ResourceIdentifier> {
    ids.iter().map(|id| ResourceIdentifier { type_: "territories".into(), id: id.clone() }).collect()
}

fn format_in_app_purchase_price_points(price_points: &[InAppPurchasePricePoint]) -> String {
    if price_points.is_empty() {
        return "No in-app purchase price points found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} in-app purchase price points:\n\n", price_points.len());
    for pp in price_points {
        let _ = writeln!(sb, "ID: {}", pp.id);
        let _ = writeln!(sb, "Customer Price: {}", pp.attributes.customer_price);
        let _ = writeln!(sb, "Proceeds: {}", pp.attributes.proceeds);
        sb.push_str("\n---\n");
    }
    sb
}

fn format_in_app_purchase_prices(prices: &[InAppPurchasePrice]) -> String {
    if prices.is_empty() {
        return "No prices found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} prices:\n\n", prices.len());
    for p in prices {
        let _ = writeln!(sb, "ID: {}", p.id);
        if !p.attributes.start_date.is_empty() {
            let _ = writeln!(sb, "Start Date: {}", p.attributes.start_date);
        }
        if !p.attributes.end_date.is_empty() {
            let _ = writeln!(sb, "End Date: {}", p.attributes.end_date);
        }
        let _ = writeln!(sb, "Manual: {}", p.attributes.manual);
        sb.push_str("\n---\n");
    }
    sb
}

fn format_subscription_plan_availabilities(availabilities: &[SubscriptionPlanAvailability]) -> String {
    if availabilities.is_empty() {
        return "No subscription plan availabilities found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} subscription plan availabilities:\n\n", availabilities.len());
    for a in availabilities {
        sb.push_str(&format_subscription_plan_availability(a));
        sb.push_str("\n---\n");
    }
    sb
}

fn format_subscription_plan_availability(a: &SubscriptionPlanAvailability) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", a.id);
    if !a.attributes.plan_type.is_empty() {
        let _ = writeln!(sb, "Plan Type: {}", a.attributes.plan_type);
    }
    if let Some(v) = a.attributes.available_in_new_territories {
        let _ = writeln!(sb, "Available In New Territories: {}", v);
    }
    sb
}
