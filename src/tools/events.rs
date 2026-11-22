//! Ported from internal/asc/tools/events.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerAppEventTools`.
pub(super) fn register_app_event_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_app_events",
            "description": "List promotional app events for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID"
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
                        "description": "Maximum number of events to return (default 50)"
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
                "title": "List App Events",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_app_events),
    );
    r.register(
        tool(r##"{
            "name": "get_app_event",
            "description": "Get details of a specific app event",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "event_id": {
                        "type": "string",
                        "description": "The app event ID"
                    }
                },
                "required": [
                    "event_id"
                ]
            },
            "annotations": {
                "title": "Get App Event",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_event),
    );
    r.register(
        tool(r##"{
            "name": "create_app_event",
            "description": "Create a new promotional app event",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID"
                    },
                    "badge": {
                        "type": "string",
                        "description": "Event badge (LIVE_EVENT, PREMIERE, CHALLENGE, COMPETITION, NEW_SEASON, MAJOR_UPDATE, SPECIAL_EVENT)"
                    },
                    "deep_link": {
                        "type": "string",
                        "description": "Deep link URL"
                    },
                    "primary_locale": {
                        "type": "string",
                        "description": "Primary locale (e.g., en-US)"
                    },
                    "priority": {
                        "type": "string",
                        "description": "Event priority (HIGH, NORMAL)"
                    },
                    "purchase_requirement": {
                        "type": "string",
                        "description": "Purchase requirement (NO_COST_ASSOCIATED, IN_APP_PURCHASE, SUBSCRIPTION, IN_APP_PURCHASE_AND_SUBSCRIPTION, IN_APP_PURCHASE_OR_SUBSCRIPTION)"
                    },
                    "purpose": {
                        "type": "string",
                        "description": "Event purpose (APPROPRIATE_FOR_ALL_USERS, ATTRACT_NEW_USERS, KEEP_ACTIVE_USERS_INFORMED, BRING_BACK_LAPSED_USERS)"
                    },
                    "reference_name": {
                        "type": "string",
                        "description": "Internal reference name"
                    }
                },
                "required": [
                    "app_id",
                    "reference_name"
                ]
            },
            "annotations": {
                "title": "Create App Event",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_app_event),
    );
    r.register(
        tool(r##"{
            "name": "update_app_event",
            "description": "Update an existing app event",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "badge": {
                        "type": "string",
                        "description": "Updated event badge"
                    },
                    "deep_link": {
                        "type": "string",
                        "description": "Updated deep link URL"
                    },
                    "event_id": {
                        "type": "string",
                        "description": "The app event ID"
                    },
                    "priority": {
                        "type": "string",
                        "description": "Updated event priority"
                    },
                    "purchase_requirement": {
                        "type": "string",
                        "description": "Updated purchase requirement"
                    },
                    "purpose": {
                        "type": "string",
                        "description": "Updated event purpose"
                    },
                    "reference_name": {
                        "type": "string",
                        "description": "Updated internal reference name"
                    }
                },
                "required": [
                    "event_id"
                ]
            },
            "annotations": {
                "title": "Update App Event",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_app_event),
    );
    r.register(
        tool(r##"{
            "name": "delete_app_event",
            "description": "Delete an app event",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "event_id": {
                        "type": "string",
                        "description": "The app event ID"
                    }
                },
                "required": [
                    "event_id"
                ]
            },
            "annotations": {
                "title": "Delete App Event",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_app_event),
    );
}

/// Ported from `Registry.handleListAppEvents`.
async fn handle_list_app_events(c: Arc<Client>, args: Args) -> HandlerResult {
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
    todo!("port handleListAppEvents from internal/asc/tools/events.go")
}

/// Ported from `Registry.handleGetAppEvent`.
async fn handle_get_app_event(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { EventID string \"json:\\\"event_id\\\"\" }", "") {
        event_id: String => "event_id";
    }
    }
    todo!("port handleGetAppEvent from internal/asc/tools/events.go")
}

/// Ported from `Registry.handleCreateAppEvent`.
async fn handle_create_app_event(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; ReferenceName string \"json:\\\"reference_name\\\"\"; Badge string \"json:\\\"badge\\\"\"; DeepLink string \"json:\\\"deep_link\\\"\"; PurchaseRequirement string \"json:\\\"purchase_requirement\\\"\"; PrimaryLocale string \"json:\\\"primary_locale\\\"\"; Priority string \"json:\\\"priority\\\"\"; Purpose string \"json:\\\"purpose\\\"\" }", "") {
        app_id: String => "app_id";
        reference_name: String => "reference_name";
        badge: String => "badge";
        deep_link: String => "deep_link";
        purchase_requirement: String => "purchase_requirement";
        primary_locale: String => "primary_locale";
        priority: String => "priority";
        purpose: String => "purpose";
    }
    }
    todo!("port handleCreateAppEvent from internal/asc/tools/events.go")
}

/// Ported from `Registry.handleUpdateAppEvent`.
async fn handle_update_app_event(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { EventID string \"json:\\\"event_id\\\"\"; ReferenceName string \"json:\\\"reference_name\\\"\"; Badge string \"json:\\\"badge\\\"\"; DeepLink string \"json:\\\"deep_link\\\"\"; PurchaseRequirement string \"json:\\\"purchase_requirement\\\"\"; Priority string \"json:\\\"priority\\\"\"; Purpose string \"json:\\\"purpose\\\"\" }", "") {
        event_id: String => "event_id";
        reference_name: String => "reference_name";
        badge: String => "badge";
        deep_link: String => "deep_link";
        purchase_requirement: String => "purchase_requirement";
        priority: String => "priority";
        purpose: String => "purpose";
    }
    }
    todo!("port handleUpdateAppEvent from internal/asc/tools/events.go")
}

/// Ported from `Registry.handleDeleteAppEvent`.
async fn handle_delete_app_event(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { EventID string \"json:\\\"event_id\\\"\" }", "") {
        event_id: String => "event_id";
    }
    }
    todo!("port handleDeleteAppEvent from internal/asc/tools/events.go")
}

