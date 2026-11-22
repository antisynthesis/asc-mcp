//! Subscription tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
pub(super) fn register_subscription_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_subscription_groups",
            "description": "List subscription groups for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to list subscription groups for"
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
                        "description": "Maximum number of groups to return (default 50)"
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
                "title": "List Subscription Groups",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_subscription_groups),
    );
    r.register(
        tool(r##"{
            "name": "get_subscription_group",
            "description": "Get details of a specific subscription group",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "group_id": {
                        "type": "string",
                        "description": "The subscription group ID"
                    }
                },
                "required": [
                    "group_id"
                ]
            },
            "annotations": {
                "title": "Get Subscription Group",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_subscription_group),
    );
    r.register(
        tool(r##"{
            "name": "list_subscriptions",
            "description": "List subscriptions for a subscription group",
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
                    "group_id": {
                        "type": "string",
                        "description": "The subscription group ID"
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
                        "description": "Maximum number of subscriptions to return (default 50)"
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
                    "group_id"
                ]
            },
            "annotations": {
                "title": "List Subscriptions",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_subscriptions),
    );
    r.register(
        tool(r##"{
            "name": "get_subscription",
            "description": "Get details of a specific subscription",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                "title": "Get Subscription",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_subscription),
    );
}

/// Handles `list_subscription_groups`.
async fn handle_list_subscription_groups(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
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
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_subscription_groups(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list subscription groups: {e}"))),
    };

    Ok(list_result(&format_subscription_groups(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_subscription_group`.
async fn handle_get_subscription_group(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { GroupID string \"json:\\\"group_id\\\"\" }", "") {
        group_id: String => "group_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.group_id.is_empty() {
        return Ok(error_result("group_id is required"));
    }

    let resp = match c.get_subscription_group(&params.group_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get subscription group: {e}"))),
    };

    Ok(data_result(&format_subscription_group(&resp.data), &resp.data))
}

/// Handles `list_subscriptions`.
async fn handle_list_subscriptions(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { GroupID string \"json:\\\"group_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        group_id: String => "group_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.group_id.is_empty() {
        return Ok(error_result("group_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_subscriptions(&params.group_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list subscriptions: {e}"))),
    };

    Ok(list_result(&format_subscriptions(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_subscription`.
async fn handle_get_subscription(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { SubscriptionID string \"json:\\\"subscription_id\\\"\" }", "") {
        subscription_id: String => "subscription_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.subscription_id.is_empty() {
        return Ok(error_result("subscription_id is required"));
    }

    let resp = match c.get_subscription(&params.subscription_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get subscription: {e}"))),
    };

    Ok(data_result(&format_subscription(&resp.data), &resp.data))
}

fn format_subscription_groups(groups: &[SubscriptionGroup]) -> String {
    if groups.is_empty() {
        return "No subscription groups found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} subscription groups:\n\n", groups.len());

    for group in groups {
        sb.push_str(&format_subscription_group(group));
        sb.push_str("\n---\n");
    }

    sb
}

fn format_subscription_group(group: &SubscriptionGroup) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "Group ID: {}", group.id);
    let _ = writeln!(sb, "Reference Name: {}", group.attributes.reference_name);
    sb
}

fn format_subscriptions(subs: &[Subscription]) -> String {
    if subs.is_empty() {
        return "No subscriptions found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} subscriptions:\n\n", subs.len());

    for sub in subs {
        sb.push_str(&format_subscription(sub));
        sb.push_str("\n---\n");
    }

    sb
}

fn format_subscription(sub: &Subscription) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", sub.id);
    let _ = writeln!(sb, "Name: {}", sub.attributes.name);
    let _ = writeln!(sb, "Product ID: {}", sub.attributes.product_id);
    let _ = writeln!(sb, "State: {}", sub.attributes.state);
    let _ = writeln!(sb, "Period: {}", sub.attributes.subscription_period);
    let _ = writeln!(sb, "Family Sharable: {}", sub.attributes.family_sharable);
    let _ = writeln!(sb, "Group Level: {}", sub.attributes.group_level);
    if !sub.attributes.review_note.is_empty() {
        let _ = writeln!(sb, "Review Note: {}", sub.attributes.review_note);
    }
    sb
}
