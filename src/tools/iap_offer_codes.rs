//! Ported from internal/asc/tools/iap_offer_codes.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;
use super::commerce_versions::rel;

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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.iap_id.is_empty() {
        return Ok(error_result("iap_id is required"));
    }
    let opts = params.commerce_list_params.opts(50);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_in_app_purchase_offer_codes(&params.iap_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list in-app purchase offer codes: {e}"))),
    };
    Ok(list_result(&format_in_app_purchase_offer_codes(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetInAppPurchaseOfferCode`.
async fn handle_get_in_app_purchase_offer_code(c: Arc<Client>, args: Args) -> HandlerResult {
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
    let resp = match c.get_in_app_purchase_offer_code(&params.offer_code_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get in-app purchase offer code: {e}"))),
    };
    Ok(data_result(&format_in_app_purchase_offer_code(&resp.data), &resp.data))
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.iap_id.is_empty() {
        return Ok(error_result("iap_id is required"));
    }
    if params.name.is_empty() {
        return Ok(error_result("name is required"));
    }
    if params.customer_eligibilities.is_empty() {
        return Ok(error_result("customer_eligibilities is required"));
    }
    if params.prices.is_empty() {
        return Ok(error_result("prices is required"));
    }
    // Each inline price gets a synthetic client ID that the prices
    // relationship references.
    let mut price_refs: Vec<ResourceIdentifier> = Vec::with_capacity(params.prices.len());
    let mut included: Vec<InAppPurchaseOfferPriceInlineCreate> = Vec::with_capacity(params.prices.len());
    for (i, price) in params.prices.iter().enumerate() {
        if price.territory_id.is_empty() {
            return Ok(error_result("each price requires territory_id"));
        }
        if price.price_point_id.is_empty() {
            return Ok(error_result("each price requires price_point_id"));
        }
        let id = format!("price-{}", i + 1);
        price_refs.push(ResourceIdentifier { type_: "inAppPurchaseOfferPrices".into(), id: id.clone() });
        included.push(InAppPurchaseOfferPriceInlineCreate {
            type_: "inAppPurchaseOfferPrices".into(),
            id,
            relationships: Some(InAppPurchaseOfferPriceInlineCreateRelationships {
                territory: rel("territories", &price.territory_id),
                price_point: rel("inAppPurchasePricePoints", &price.price_point_id),
            }),
        });
    }
    let req = InAppPurchaseOfferCodeCreateRequest {
        data: InAppPurchaseOfferCodeCreateData {
            type_: "inAppPurchaseOfferCodes".into(),
            attributes: InAppPurchaseOfferCodeCreateAttributes {
                name: params.name.clone(),
                customer_eligibilities: params.customer_eligibilities.clone(),
            },
            relationships: InAppPurchaseOfferCodeCreateRelationships {
                in_app_purchase: rel("inAppPurchases", &params.iap_id),
                prices: RelationshipDataList { data: GoSlice::from(price_refs) },
            },
        },
        included: GoSlice::from(included),
    };
    let resp = match c.create_in_app_purchase_offer_code(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create in-app purchase offer code: {e}"))),
    };
    Ok(data_result(&format!("In-app purchase offer code created:\n{}", format_in_app_purchase_offer_code(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleUpdateInAppPurchaseOfferCode`.
async fn handle_update_in_app_purchase_offer_code(c: Arc<Client>, args: Args) -> HandlerResult {
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
    if params.active.is_none() {
        return Ok(error_result("active is required"));
    }
    let req = InAppPurchaseOfferCodeUpdateRequest {
        data: InAppPurchaseOfferCodeUpdateData {
            type_: "inAppPurchaseOfferCodes".into(),
            id: params.offer_code_id.clone(),
            attributes: OfferCodeActiveAttributes { active: params.active },
        },
    };
    let resp = match c.update_in_app_purchase_offer_code(&params.offer_code_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update in-app purchase offer code: {e}"))),
    };
    Ok(data_result(&format!("In-app purchase offer code updated:\n{}", format_in_app_purchase_offer_code(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleListInAppPurchaseOfferCodePrices`.
async fn handle_list_in_app_purchase_offer_code_prices(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferCodeID string \"json:\\\"offer_code_id\\\"\"; tools.commerceListParams }", "") {
        offer_code_id: String => "offer_code_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.offer_code_id.is_empty() {
        return Ok(error_result("offer_code_id is required"));
    }
    let opts = params.commerce_list_params.opts(50);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_in_app_purchase_offer_code_prices(&params.offer_code_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list in-app purchase offer code prices: {e}"))),
    };
    Ok(list_result(&format_in_app_purchase_offer_prices(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleListInAppPurchaseOfferCodeCustomCodes`.
async fn handle_list_in_app_purchase_offer_code_custom_codes(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferCodeID string \"json:\\\"offer_code_id\\\"\"; tools.commerceListParams }", "") {
        offer_code_id: String => "offer_code_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.offer_code_id.is_empty() {
        return Ok(error_result("offer_code_id is required"));
    }
    let opts = params.commerce_list_params.opts(50);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_in_app_purchase_offer_code_custom_codes(&params.offer_code_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list in-app purchase offer code custom codes: {e}"))),
    };
    Ok(list_result(&format_in_app_purchase_offer_code_custom_codes(&resp.data), &resp.data, &resp.links))
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.offer_code_id.is_empty() {
        return Ok(error_result("offer_code_id is required"));
    }
    if params.custom_code.is_empty() {
        return Ok(error_result("custom_code is required"));
    }
    if params.number_of_codes <= 0 {
        return Ok(error_result("number_of_codes is required"));
    }
    let req = InAppPurchaseOfferCodeCustomCodeCreateRequest {
        data: InAppPurchaseOfferCodeCustomCodeCreateData {
            type_: "inAppPurchaseOfferCodeCustomCodes".into(),
            attributes: InAppPurchaseOfferCodeCustomCodeCreateAttributes {
                custom_code: params.custom_code.clone(),
                number_of_codes: params.number_of_codes,
                expiration_date: params.expiration_date.clone(),
            },
            relationships: OfferCodeRelationship { offer_code: rel("inAppPurchaseOfferCodes", &params.offer_code_id) },
        },
    };
    let resp = match c.create_in_app_purchase_offer_code_custom_code(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create in-app purchase offer code custom code: {e}"))),
    };
    Ok(data_result(&format!("Custom code created:\n{}", format_in_app_purchase_offer_code_custom_code(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleUpdateInAppPurchaseOfferCodeCustomCode`.
async fn handle_update_in_app_purchase_offer_code_custom_code(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { CustomCodeID string \"json:\\\"custom_code_id\\\"\"; Active *bool \"json:\\\"active\\\"\" }", "") {
        custom_code_id: String => "custom_code_id";
        active: Option<bool> => "active";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.custom_code_id.is_empty() {
        return Ok(error_result("custom_code_id is required"));
    }
    if params.active.is_none() {
        return Ok(error_result("active is required"));
    }
    let req = InAppPurchaseOfferCodeCustomCodeUpdateRequest {
        data: InAppPurchaseOfferCodeCustomCodeUpdateData {
            type_: "inAppPurchaseOfferCodeCustomCodes".into(),
            id: params.custom_code_id.clone(),
            attributes: OfferCodeActiveAttributes { active: params.active },
        },
    };
    let resp = match c.update_in_app_purchase_offer_code_custom_code(&params.custom_code_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update in-app purchase offer code custom code: {e}"))),
    };
    Ok(data_result(&format!("Custom code updated:\n{}", format_in_app_purchase_offer_code_custom_code(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleListInAppPurchaseOfferCodeOneTimeUseCodes`.
async fn handle_list_in_app_purchase_offer_code_one_time_use_codes(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OfferCodeID string \"json:\\\"offer_code_id\\\"\"; tools.commerceListParams }", "") {
        offer_code_id: String => "offer_code_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.offer_code_id.is_empty() {
        return Ok(error_result("offer_code_id is required"));
    }
    let opts = params.commerce_list_params.opts(50);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_in_app_purchase_offer_code_one_time_use_codes(&params.offer_code_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list in-app purchase offer code one-time-use codes: {e}"))),
    };
    Ok(list_result(&format_in_app_purchase_offer_code_one_time_use_codes(&resp.data), &resp.data, &resp.links))
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.offer_code_id.is_empty() {
        return Ok(error_result("offer_code_id is required"));
    }
    if params.number_of_codes <= 0 {
        return Ok(error_result("number_of_codes is required"));
    }
    if params.expiration_date.is_empty() {
        return Ok(error_result("expiration_date is required"));
    }
    let req = InAppPurchaseOfferCodeOneTimeUseCodeCreateRequest {
        data: InAppPurchaseOfferCodeOneTimeUseCodeCreateData {
            type_: "inAppPurchaseOfferCodeOneTimeUseCodes".into(),
            attributes: InAppPurchaseOfferCodeOneTimeUseCodeCreateAttributes {
                number_of_codes: params.number_of_codes,
                expiration_date: params.expiration_date.clone(),
                environment: params.environment.clone(),
            },
            relationships: OfferCodeRelationship { offer_code: rel("inAppPurchaseOfferCodes", &params.offer_code_id) },
        },
    };
    let resp = match c.create_in_app_purchase_offer_code_one_time_use_code(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create in-app purchase offer code one-time-use code: {e}"))),
    };
    Ok(data_result(&format!("One-time-use code batch created:\n{}", format_in_app_purchase_offer_code_one_time_use_code(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleUpdateInAppPurchaseOfferCodeOneTimeUseCode`.
async fn handle_update_in_app_purchase_offer_code_one_time_use_code(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OneTimeUseCodeID string \"json:\\\"one_time_use_code_id\\\"\"; Active *bool \"json:\\\"active\\\"\" }", "") {
        one_time_use_code_id: String => "one_time_use_code_id";
        active: Option<bool> => "active";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.one_time_use_code_id.is_empty() {
        return Ok(error_result("one_time_use_code_id is required"));
    }
    if params.active.is_none() {
        return Ok(error_result("active is required"));
    }
    let req = InAppPurchaseOfferCodeOneTimeUseCodeUpdateRequest {
        data: InAppPurchaseOfferCodeOneTimeUseCodeUpdateData {
            type_: "inAppPurchaseOfferCodeOneTimeUseCodes".into(),
            id: params.one_time_use_code_id.clone(),
            attributes: OfferCodeActiveAttributes { active: params.active },
        },
    };
    let resp = match c.update_in_app_purchase_offer_code_one_time_use_code(&params.one_time_use_code_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update in-app purchase offer code one-time-use code: {e}"))),
    };
    Ok(data_result(&format!("One-time-use code batch updated:\n{}", format_in_app_purchase_offer_code_one_time_use_code(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleGetInAppPurchaseOfferCodeOneTimeUseCodeValues`.
async fn handle_get_in_app_purchase_offer_code_one_time_use_code_values(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { OneTimeUseCodeID string \"json:\\\"one_time_use_code_id\\\"\" }", "") {
        one_time_use_code_id: String => "one_time_use_code_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.one_time_use_code_id.is_empty() {
        return Ok(error_result("one_time_use_code_id is required"));
    }
    let csv = match c.get_in_app_purchase_offer_code_one_time_use_code_values(&params.one_time_use_code_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get one-time-use code values: {e}"))),
    };
    if csv.trim().is_empty() {
        return Ok(CallToolResult::success("No codes available yet. Apple generates the batch asynchronously; retry shortly."));
    }
    Ok(data_result(&csv, &json!({ "csv": csv })))
}

/// Go `formatInAppPurchaseOfferCodes`.
fn format_in_app_purchase_offer_codes(codes: &[InAppPurchaseOfferCode]) -> String {
    if codes.is_empty() {
        return "No in-app purchase offer codes found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} in-app purchase offer codes:\n\n", codes.len());
    for c in codes {
        sb.push_str(&format_in_app_purchase_offer_code(c));
        sb.push_str("\n---\n");
    }
    sb
}

/// Go `formatInAppPurchaseOfferCode`.
fn format_in_app_purchase_offer_code(c: &InAppPurchaseOfferCode) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", c.id);
    let _ = write!(sb, "Name: {}\n", c.attributes.name);
    if !c.attributes.customer_eligibilities.is_empty() {
        let _ = write!(sb, "Customer Eligibilities: {}\n", c.attributes.customer_eligibilities.join(", "));
    }
    let _ = write!(sb, "Active: {}\n", c.attributes.active);
    let _ = write!(sb, "Production Codes: {}\n", c.attributes.production_code_count);
    let _ = write!(sb, "Sandbox Codes: {}\n", c.attributes.sandbox_code_count);
    sb
}

/// Go `formatInAppPurchaseOfferPrices`.
fn format_in_app_purchase_offer_prices(prices: &[InAppPurchaseOfferPrice]) -> String {
    if prices.is_empty() {
        return "No offer prices found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} offer prices:\n\n", prices.len());
    for p in prices {
        let _ = write!(sb, "ID: {}\n", p.id);
        if let Some(t) = &p.relationships.territory {
            let _ = write!(sb, "Territory: {}\n", t.data.id);
        }
        if let Some(pp) = &p.relationships.price_point {
            let _ = write!(sb, "Price Point: {}\n", pp.data.id);
        }
        sb.push_str("\n---\n");
    }
    sb
}

/// Go `formatInAppPurchaseOfferCodeCustomCodes`.
fn format_in_app_purchase_offer_code_custom_codes(codes: &[InAppPurchaseOfferCodeCustomCode]) -> String {
    if codes.is_empty() {
        return "No custom codes found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} custom codes:\n\n", codes.len());
    for c in codes {
        sb.push_str(&format_in_app_purchase_offer_code_custom_code(c));
        sb.push_str("\n---\n");
    }
    sb
}

/// Go `formatInAppPurchaseOfferCodeCustomCode`.
fn format_in_app_purchase_offer_code_custom_code(c: &InAppPurchaseOfferCodeCustomCode) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", c.id);
    let _ = write!(sb, "Code: {}\n", c.attributes.custom_code);
    let _ = write!(sb, "Number of Codes: {}\n", c.attributes.number_of_codes);
    if !c.attributes.expiration_date.is_empty() {
        let _ = write!(sb, "Expires: {}\n", c.attributes.expiration_date);
    }
    let _ = write!(sb, "Active: {}\n", c.attributes.active);
    sb
}

/// Go `formatInAppPurchaseOfferCodeOneTimeUseCodes`.
fn format_in_app_purchase_offer_code_one_time_use_codes(codes: &[InAppPurchaseOfferCodeOneTimeUseCode]) -> String {
    if codes.is_empty() {
        return "No one-time-use code batches found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} one-time-use code batches:\n\n", codes.len());
    for c in codes {
        sb.push_str(&format_in_app_purchase_offer_code_one_time_use_code(c));
        sb.push_str("\n---\n");
    }
    sb
}

/// Go `formatInAppPurchaseOfferCodeOneTimeUseCode`.
fn format_in_app_purchase_offer_code_one_time_use_code(c: &InAppPurchaseOfferCodeOneTimeUseCode) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", c.id);
    let _ = write!(sb, "Number of Codes: {}\n", c.attributes.number_of_codes);
    if !c.attributes.expiration_date.is_empty() {
        let _ = write!(sb, "Expires: {}\n", c.attributes.expiration_date);
    }
    if !c.attributes.environment.is_empty() {
        let _ = write!(sb, "Environment: {}\n", c.attributes.environment);
    }
    let _ = write!(sb, "Active: {}\n", c.attributes.active);
    sb
}
