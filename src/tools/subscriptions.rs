//! Ported from internal/asc/tools/subscriptions.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerSubscriptionTools`.
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

/// Ported from `Registry.handleListSubscriptionGroups`.
async fn handle_list_subscription_groups(c: Arc<Client>, args: Args) -> HandlerResult {
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
    todo!("port handleListSubscriptionGroups from internal/asc/tools/subscriptions.go")
}

/// Ported from `Registry.handleGetSubscriptionGroup`.
async fn handle_get_subscription_group(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GroupID string \"json:\\\"group_id\\\"\" }", "") {
        group_id: String => "group_id";
    }
    }
    todo!("port handleGetSubscriptionGroup from internal/asc/tools/subscriptions.go")
}

/// Ported from `Registry.handleListSubscriptions`.
async fn handle_list_subscriptions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GroupID string \"json:\\\"group_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        group_id: String => "group_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListSubscriptions from internal/asc/tools/subscriptions.go")
}

/// Ported from `Registry.handleGetSubscription`.
async fn handle_get_subscription(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubscriptionID string \"json:\\\"subscription_id\\\"\" }", "") {
        subscription_id: String => "subscription_id";
    }
    }
    todo!("port handleGetSubscription from internal/asc/tools/subscriptions.go")
}

