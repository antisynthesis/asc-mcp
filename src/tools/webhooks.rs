//! Webhook tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
pub(super) fn register_webhook_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_webhooks",
            "description": "List the webhooks configured for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to list webhooks for"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return, e.g. {\"webhooks\": [\"name\", \"url\", \"enabled\"]}."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of webhooks to return (default 50, max 200)"
                    }
                },
                "required": [
                    "app_id"
                ]
            },
            "annotations": {
                "title": "List Webhooks",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_webhooks),
    );
    r.register(
        tool(r##"{
            "name": "get_webhook",
            "description": "Get details of a specific webhook",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "webhook_id": {
                        "type": "string",
                        "description": "The webhook ID"
                    }
                },
                "required": [
                    "webhook_id"
                ]
            },
            "annotations": {
                "title": "Get Webhook",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_webhook),
    );
    r.register(
        tool(r##"{
            "name": "create_webhook",
            "description": "Create a webhook for an app to receive App Store Connect event notifications",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to create the webhook for"
                    },
                    "enabled": {
                        "type": "boolean",
                        "description": "Whether the webhook is enabled on creation (default true)"
                    },
                    "event_types": {
                        "type": "array",
                        "description": "Event types to subscribe to, e.g. APP_STORE_VERSION_APP_VERSION_STATE_UPDATED, BUILD_UPLOAD_STATE_UPDATED, BETA_FEEDBACK_CRASH_SUBMISSION_CREATED",
                        "items": {
                            "type": "string"
                        }
                    },
                    "name": {
                        "type": "string",
                        "description": "A name for the webhook"
                    },
                    "secret": {
                        "type": "string",
                        "description": "Shared secret used to sign webhook payloads"
                    },
                    "url": {
                        "type": "string",
                        "description": "The HTTPS endpoint URL that receives event payloads"
                    }
                },
                "required": [
                    "app_id",
                    "name",
                    "url",
                    "secret",
                    "event_types"
                ]
            },
            "annotations": {
                "title": "Create Webhook",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_webhook),
    );
    r.register(
        tool(r##"{
            "name": "update_webhook",
            "description": "Update a webhook's configuration (name, URL, secret, event types, or enabled state)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "enabled": {
                        "type": "boolean",
                        "description": "Enable or disable the webhook"
                    },
                    "event_types": {
                        "type": "array",
                        "description": "New list of event types to subscribe to (replaces the existing list)",
                        "items": {
                            "type": "string"
                        }
                    },
                    "name": {
                        "type": "string",
                        "description": "New name for the webhook"
                    },
                    "secret": {
                        "type": "string",
                        "description": "New shared secret for signing payloads"
                    },
                    "url": {
                        "type": "string",
                        "description": "New HTTPS endpoint URL"
                    },
                    "webhook_id": {
                        "type": "string",
                        "description": "The webhook ID"
                    }
                },
                "required": [
                    "webhook_id"
                ]
            },
            "annotations": {
                "title": "Update Webhook",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_webhook),
    );
    r.register(
        tool(r##"{
            "name": "delete_webhook",
            "description": "Delete a webhook",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "webhook_id": {
                        "type": "string",
                        "description": "The webhook ID"
                    }
                },
                "required": [
                    "webhook_id"
                ]
            },
            "annotations": {
                "title": "Delete Webhook",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_webhook),
    );
    r.register(
        tool(r##"{
            "name": "list_webhook_deliveries",
            "description": "List delivery attempts for a webhook, including delivery state and endpoint responses",
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
                        "description": "JSON:API filter map. Supported keys: deliveryState (SUCCEEDED, FAILED, PENDING), createdDateGreaterThanOrEqualTo, createdDateLessThan. Values are arrays, e.g. {\"deliveryState\": [\"FAILED\"]}."
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response (supported: event).",
                        "items": {
                            "type": "string"
                        }
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of deliveries to return (default 50, max 200)"
                    },
                    "webhook_id": {
                        "type": "string",
                        "description": "The webhook ID to list deliveries for"
                    }
                },
                "required": [
                    "webhook_id"
                ]
            },
            "annotations": {
                "title": "List Webhook Deliveries",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_webhook_deliveries),
    );
    r.register(
        tool(r##"{
            "name": "ping_webhook",
            "description": "Send a test ping event to a webhook endpoint to verify it is reachable",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "webhook_id": {
                        "type": "string",
                        "description": "The webhook ID to ping"
                    }
                },
                "required": [
                    "webhook_id"
                ]
            },
            "annotations": {
                "title": "Ping Webhook",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_ping_webhook),
    );
    r.register(
        tool(r##"{
            "name": "redeliver_webhook_delivery",
            "description": "Request redelivery of a webhook event using a previous delivery as the template",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "delivery_id": {
                        "type": "string",
                        "description": "The webhook delivery ID to use as the redelivery template"
                    }
                },
                "required": [
                    "delivery_id"
                ]
            },
            "annotations": {
                "title": "Redeliver Webhook Delivery",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_redeliver_webhook_delivery),
    );
}

/// Handles `list_webhooks`.
async fn handle_list_webhooks(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        fields: JsonMap<JsonSlice<String>> => "fields";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }

    let limit = clamp_limit(params.limit);

    let opts = list_opts(limit, &JsonMap::nil(), &JsonSlice::nil(), &params.fields, &JsonSlice::nil());
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_webhooks(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list webhooks: {e}"))),
    };

    Ok(list_result(&format_webhooks(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_webhook`.
async fn handle_get_webhook(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { WebhookID string \"json:\\\"webhook_id\\\"\" }", "") {
        webhook_id: String => "webhook_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.webhook_id.is_empty() {
        return Ok(error_result("webhook_id is required"));
    }

    let resp = match c.get_webhook(&params.webhook_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get webhook: {e}"))),
    };

    Ok(data_result(&format_webhook(&resp.data), &resp.data))
}

/// Handles `create_webhook`.
async fn handle_create_webhook(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Name string \"json:\\\"name\\\"\"; URL string \"json:\\\"url\\\"\"; Secret string \"json:\\\"secret\\\"\"; EventTypes []string \"json:\\\"event_types\\\"\"; Enabled *bool \"json:\\\"enabled\\\"\" }", "") {
        app_id: String => "app_id";
        name: String => "name";
        url: String => "url";
        secret: String => "secret";
        event_types: JsonSlice<String> => "event_types";
        enabled: Option<bool> => "enabled";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }

    if params.name.is_empty() {
        return Ok(error_result("name is required"));
    }

    if params.url.is_empty() {
        return Ok(error_result("url is required"));
    }

    if params.secret.is_empty() {
        return Ok(error_result("secret is required"));
    }

    if params.event_types.is_empty() {
        return Ok(error_result("event_types is required"));
    }

    let mut enabled = true;
    if let Some(v) = params.enabled {
        enabled = v;
    }

    let req = WebhookCreateRequest {
        data: WebhookCreateData {
            type_: "webhooks".into(),
            attributes: WebhookCreateAttributes {
                enabled,
                event_types: params.event_types.clone(),
                name: params.name.clone(),
                secret: params.secret.clone(),
                url: params.url.clone(),
            },
            relationships: WebhookCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
            },
        },
    };

    let resp = match c.create_webhook(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create webhook: {e}"))),
    };

    Ok(data_result(&format!("Created webhook:\n{}", format_webhook(&resp.data)), &resp.data))
}

/// Handles `update_webhook`.
async fn handle_update_webhook(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { WebhookID string \"json:\\\"webhook_id\\\"\"; Name string \"json:\\\"name\\\"\"; URL string \"json:\\\"url\\\"\"; Secret string \"json:\\\"secret\\\"\"; EventTypes []string \"json:\\\"event_types\\\"\"; Enabled *bool \"json:\\\"enabled\\\"\" }", "") {
        webhook_id: String => "webhook_id";
        name: String => "name";
        url: String => "url";
        secret: String => "secret";
        event_types: JsonSlice<String> => "event_types";
        enabled: Option<bool> => "enabled";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.webhook_id.is_empty() {
        return Ok(error_result("webhook_id is required"));
    }

    let req = WebhookUpdateRequest {
        data: WebhookUpdateData {
            type_: "webhooks".into(),
            id: params.webhook_id.clone(),
            attributes: Some(WebhookUpdateAttributes {
                enabled: params.enabled,
                event_types: params.event_types.clone(),
                name: params.name.clone(),
                secret: params.secret.clone(),
                url: params.url.clone(),
            }),
        },
    };

    let resp = match c.update_webhook(&params.webhook_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update webhook: {e}"))),
    };

    Ok(data_result(&format!("Webhook updated:\n{}", format_webhook(&resp.data)), &resp.data))
}

/// Handles `delete_webhook`.
async fn handle_delete_webhook(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { WebhookID string \"json:\\\"webhook_id\\\"\" }", "") {
        webhook_id: String => "webhook_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.webhook_id.is_empty() {
        return Ok(error_result("webhook_id is required"));
    }

    if let Err(e) = c.delete_webhook(&params.webhook_id).await {
        return Ok(error_result(format!("Failed to delete webhook: {e}")));
    }

    Ok(CallToolResult::success("Webhook deleted successfully"))
}

/// Handles `list_webhook_deliveries`.
async fn handle_list_webhook_deliveries(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { WebhookID string \"json:\\\"webhook_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        webhook_id: String => "webhook_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.webhook_id.is_empty() {
        return Ok(error_result("webhook_id is required"));
    }

    let limit = clamp_limit(params.limit);

    let opts = list_opts(limit, &params.filter, &JsonSlice::nil(), &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_webhook_deliveries(&params.webhook_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list webhook deliveries: {e}"))),
    };

    Ok(list_result(&format_webhook_deliveries(&resp.data), &resp.data, &resp.links))
}

/// Handles `ping_webhook`.
async fn handle_ping_webhook(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { WebhookID string \"json:\\\"webhook_id\\\"\" }", "") {
        webhook_id: String => "webhook_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.webhook_id.is_empty() {
        return Ok(error_result("webhook_id is required"));
    }

    let req = WebhookPingCreateRequest {
        data: WebhookPingCreateData {
            type_: "webhookPings".into(),
            relationships: WebhookPingCreateRelationships {
                webhook: RelationshipData { data: ResourceIdentifier { type_: "webhooks".into(), id: params.webhook_id.clone() } },
            },
        },
    };

    let resp = match c.create_webhook_ping(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to ping webhook: {e}"))),
    };

    Ok(data_result(&format!("Ping event sent: {}", resp.data.id), &resp.data))
}

/// Handles `redeliver_webhook_delivery`.
async fn handle_redeliver_webhook_delivery(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { DeliveryID string \"json:\\\"delivery_id\\\"\" }", "") {
        delivery_id: String => "delivery_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.delivery_id.is_empty() {
        return Ok(error_result("delivery_id is required"));
    }

    let req = WebhookDeliveryCreateRequest {
        data: WebhookDeliveryCreateData {
            type_: "webhookDeliveries".into(),
            relationships: WebhookDeliveryCreateRelationships {
                template: RelationshipData {
                    data: ResourceIdentifier { type_: "webhookDeliveries".into(), id: params.delivery_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_webhook_redelivery(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to request redelivery: {e}"))),
    };

    Ok(data_result(&format!("Redelivery requested:\n{}", format_webhook_delivery(&resp.data)), &resp.data))
}

/// Clamps a page size: `limit <= 0 → 50; limit > 200 → 200`.
fn clamp_limit(limit: i64) -> i64 {
    let mut limit = limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }
    limit
}

fn format_webhooks(webhooks: &[Webhook]) -> String {
    if webhooks.is_empty() {
        return "No webhooks found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} webhooks:\n\n", webhooks.len());

    for webhook in webhooks {
        sb.push_str(&format_webhook(webhook));
        sb.push_str("\n---\n");
    }

    sb
}

fn format_webhook(webhook: &Webhook) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "Webhook ID: {}", webhook.id);
    if !webhook.attributes.name.is_empty() {
        let _ = writeln!(sb, "Name: {}", webhook.attributes.name);
    }
    if !webhook.attributes.url.is_empty() {
        let _ = writeln!(sb, "URL: {}", webhook.attributes.url);
    }
    let _ = writeln!(sb, "Enabled: {}", webhook.attributes.enabled);
    if !webhook.attributes.event_types.is_empty() {
        let _ = writeln!(sb, "Event Types: {}", webhook.attributes.event_types.join(", "));
    }
    sb
}

fn format_webhook_deliveries(deliveries: &[WebhookDelivery]) -> String {
    if deliveries.is_empty() {
        return "No webhook deliveries found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} webhook deliveries:\n\n", deliveries.len());

    for delivery in deliveries {
        sb.push_str(&format_webhook_delivery(delivery));
        sb.push_str("\n---\n");
    }

    sb
}

fn format_webhook_delivery(delivery: &WebhookDelivery) -> String {
    let a = &delivery.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "Delivery ID: {}", delivery.id);
    if !a.delivery_state.is_empty() {
        let _ = writeln!(sb, "State: {}", a.delivery_state);
    }
    if a.redelivery {
        sb.push_str("Redelivery: Yes\n");
    }
    if let Some(d) = &a.created_date {
        let _ = writeln!(sb, "Created: {}", d.format("2006-01-02 15:04:05"));
    }
    if let Some(d) = &a.sent_date {
        let _ = writeln!(sb, "Sent: {}", d.format("2006-01-02 15:04:05"));
    }
    if !a.error_message.is_empty() {
        let _ = writeln!(sb, "Error: {}", a.error_message);
    }
    if let Some(r) = &a.response
        && r.http_status_code != 0 {
            let _ = writeln!(sb, "Response Status: {}", r.http_status_code);
        }
    sb
}
