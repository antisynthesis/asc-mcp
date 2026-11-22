//! In-app event tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
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

/// Handles `list_app_events`.
async fn handle_list_app_events(c: Arc<Client>, args: Args) -> HandlerResult {
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
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_events(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list app events: {e}"))),
    };

    Ok(list_result(&format_app_events(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_app_event`.
async fn handle_get_app_event(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { EventID string \"json:\\\"event_id\\\"\" }", "") {
        event_id: String => "event_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.event_id.is_empty() {
        return Ok(error_result("event_id is required"));
    }

    let resp = match c.get_app_event(&params.event_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get app event: {e}"))),
    };

    Ok(data_result(&format_app_event(&resp.data), &resp.data))
}

/// Handles `create_app_event`.
async fn handle_create_app_event(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }
    if params.reference_name.is_empty() {
        return Ok(error_result("reference_name is required"));
    }

    let req = AppEventCreateRequest {
        data: AppEventCreateData {
            type_: "appEvents".into(),
            attributes: AppEventCreateAttributes {
                reference_name: params.reference_name.clone(),
                badge: params.badge.clone(),
                deep_link: params.deep_link.clone(),
                purchase_requirement: params.purchase_requirement.clone(),
                primary_locale: params.primary_locale.clone(),
                priority: params.priority.clone(),
                purpose: params.purpose.clone(),
                ..Default::default()
            },
            relationships: AppEventCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
            },
        },
    };

    let resp = match c.create_app_event(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create app event: {e}"))),
    };

    Ok(data_result(
        &format!("Created app event: {} (ID: {})", resp.data.attributes.reference_name, resp.data.id),
        &resp.data,
    ))
}

/// Handles `update_app_event`.
async fn handle_update_app_event(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.event_id.is_empty() {
        return Ok(error_result("event_id is required"));
    }

    let req = AppEventUpdateRequest {
        data: AppEventUpdateData {
            type_: "appEvents".into(),
            id: params.event_id.clone(),
            attributes: AppEventUpdateAttributes {
                reference_name: params.reference_name.clone(),
                badge: params.badge.clone(),
                deep_link: params.deep_link.clone(),
                purchase_requirement: params.purchase_requirement.clone(),
                priority: params.priority.clone(),
                purpose: params.purpose.clone(),
                ..Default::default()
            },
        },
    };

    let resp = match c.update_app_event(&params.event_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update app event: {e}"))),
    };

    Ok(data_result(&format!("Updated app event: {}", resp.data.id), &resp.data))
}

/// Handles `delete_app_event`.
async fn handle_delete_app_event(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { EventID string \"json:\\\"event_id\\\"\" }", "") {
        event_id: String => "event_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.event_id.is_empty() {
        return Ok(error_result("event_id is required"));
    }

    if let Err(e) = c.delete_app_event(&params.event_id).await {
        return Ok(error_result(format!("Failed to delete app event: {e}")));
    }

    Ok(CallToolResult::success("App event deleted successfully"))
}

fn format_app_events(events: &[AppEvent]) -> String {
    if events.is_empty() {
        return "No app events found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} app events:\n\n", events.len());

    for event in events {
        sb.push_str(&format_app_event(event));
        sb.push_str("\n---\n");
    }

    sb
}

fn format_app_event(event: &AppEvent) -> String {
    let a = &event.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", event.id);
    let _ = writeln!(sb, "Reference Name: {}", a.reference_name);
    if !a.badge.is_empty() {
        let _ = writeln!(sb, "Badge: {}", a.badge);
    }
    if !a.event_state.is_empty() {
        let _ = writeln!(sb, "State: {}", a.event_state);
    }
    if !a.priority.is_empty() {
        let _ = writeln!(sb, "Priority: {}", a.priority);
    }
    if !a.purpose.is_empty() {
        let _ = writeln!(sb, "Purpose: {}", a.purpose);
    }
    if !a.deep_link.is_empty() {
        let _ = writeln!(sb, "Deep Link: {}", a.deep_link);
    }
    if !a.purchase_requirement.is_empty() {
        let _ = writeln!(sb, "Purchase Requirement: {}", a.purchase_requirement);
    }
    sb
}
