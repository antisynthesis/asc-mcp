//! Ported from internal/asc/tools/pricing.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerPricingTools`.
pub(super) fn register_pricing_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "get_app_price_schedule",
            "description": "Get the price schedule for an app",
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
                "title": "Get App Price Schedule",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_price_schedule),
    );
    r.register(
        tool(r##"{
            "name": "list_app_price_points",
            "description": "List available price points for an app",
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
                        "description": "Maximum number of price points to return (default 100)"
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
                "title": "List App Price Points",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_app_price_points),
    );
    r.register(
        tool(r##"{
            "name": "list_territories",
            "description": "List all available App Store territories",
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
                        "description": "Maximum number of territories to return (default 200)"
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
                "title": "List Territories",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_territories),
    );
    r.register(
        tool(r##"{
            "name": "list_subscription_price_points",
            "description": "List price points for a subscription",
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
                        "description": "Maximum number of price points to return (default 100)"
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
                "title": "List Subscription Price Points",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_subscription_price_points),
    );
}

/// Ported from `Registry.handleGetAppPriceSchedule`.
async fn handle_get_app_price_schedule(c: Arc<Client>, args: Args) -> HandlerResult {
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
    let resp = match c.get_app_price_schedule(&params.app_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get app price schedule: {e}"))),
    };
    Ok(data_result(&format_app_price_schedule(&resp.data), &resp.data))
}

/// Ported from `Registry.handleListAppPricePoints`.
async fn handle_list_app_price_points(c: Arc<Client>, args: Args) -> HandlerResult {
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
        limit = 100;
    }
    if limit > 200 {
        limit = 200;
    }
    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_price_points(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list app price points: {e}"))),
    };
    Ok(list_result(&format_app_price_points(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleListTerritories`.
async fn handle_list_territories(c: Arc<Client>, args: Args) -> HandlerResult {
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
        limit = 200;
    }
    if limit > 200 {
        limit = 200;
    }
    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_territories(opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list territories: {e}"))),
    };
    Ok(list_result(&format_territories(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleListSubscriptionPricePoints`.
async fn handle_list_subscription_price_points(c: Arc<Client>, args: Args) -> HandlerResult {
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
        limit = 100;
    }
    if limit > 200 {
        limit = 200;
    }
    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_subscription_price_points(&params.subscription_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list subscription price points: {e}"))),
    };
    Ok(list_result(&format_subscription_price_points(&resp.data), &resp.data, &resp.links))
}

/// Go `formatAppPriceSchedule`.
fn format_app_price_schedule(schedule: &AppPriceSchedule) -> String {
    format!("App Price Schedule ID: {}\n", schedule.id)
}

/// Go `formatAppPricePoints`.
pub(crate) fn format_app_price_points(price_points: &[AppPricePoint]) -> String {
    if price_points.is_empty() {
        return "No price points found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} price points:\n\n", price_points.len());
    for pp in price_points {
        sb.push_str(&format_app_price_point(pp));
        sb.push_str("\n---\n");
    }
    sb
}

/// Go `formatAppPricePoint`.
pub(crate) fn format_app_price_point(pp: &AppPricePoint) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", pp.id);
    let _ = write!(sb, "Customer Price: {}\n", pp.attributes.customer_price);
    let _ = write!(sb, "Proceeds: {}\n", pp.attributes.proceeds);
    sb
}

/// Go `formatTerritories`.
pub(crate) fn format_territories(territories: &[Territory]) -> String {
    if territories.is_empty() {
        return "No territories found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} territories:\n\n", territories.len());
    for t in territories {
        let _ = write!(sb, "ID: {} - Currency: {}\n", t.id, t.attributes.currency);
    }
    sb
}

/// Go `formatSubscriptionPricePoints`.
pub(crate) fn format_subscription_price_points(price_points: &[SubscriptionPricePoint]) -> String {
    if price_points.is_empty() {
        return "No subscription price points found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} subscription price points:\n\n", price_points.len());
    for pp in price_points {
        sb.push_str(&format_subscription_price_point(pp));
        sb.push_str("\n---\n");
    }
    sb
}

/// Go `formatSubscriptionPricePoint`.
pub(crate) fn format_subscription_price_point(pp: &SubscriptionPricePoint) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", pp.id);
    let _ = write!(sb, "Customer Price: {}\n", pp.attributes.customer_price);
    let _ = write!(sb, "Proceeds: {}\n", pp.attributes.proceeds);
    let _ = write!(sb, "Proceeds Year 2: {}\n", pp.attributes.proceeds_year2);
    sb
}
