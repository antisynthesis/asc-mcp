//! Ported from internal/asc/tools/webhooks.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerWebhookTools`.
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

/// Ported from `Registry.handleListWebhooks`.
async fn handle_list_webhooks(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        fields: GoMap<GoSlice<String>> => "fields";
    }
    }
    todo!("port handleListWebhooks from internal/asc/tools/webhooks.go")
}

/// Ported from `Registry.handleGetWebhook`.
async fn handle_get_webhook(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { WebhookID string \"json:\\\"webhook_id\\\"\" }", "") {
        webhook_id: String => "webhook_id";
    }
    }
    todo!("port handleGetWebhook from internal/asc/tools/webhooks.go")
}

/// Ported from `Registry.handleCreateWebhook`.
async fn handle_create_webhook(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Name string \"json:\\\"name\\\"\"; URL string \"json:\\\"url\\\"\"; Secret string \"json:\\\"secret\\\"\"; EventTypes []string \"json:\\\"event_types\\\"\"; Enabled *bool \"json:\\\"enabled\\\"\" }", "") {
        app_id: String => "app_id";
        name: String => "name";
        url: String => "url";
        secret: String => "secret";
        event_types: GoSlice<String> => "event_types";
        enabled: Option<bool> => "enabled";
    }
    }
    todo!("port handleCreateWebhook from internal/asc/tools/webhooks.go")
}

/// Ported from `Registry.handleUpdateWebhook`.
async fn handle_update_webhook(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { WebhookID string \"json:\\\"webhook_id\\\"\"; Name string \"json:\\\"name\\\"\"; URL string \"json:\\\"url\\\"\"; Secret string \"json:\\\"secret\\\"\"; EventTypes []string \"json:\\\"event_types\\\"\"; Enabled *bool \"json:\\\"enabled\\\"\" }", "") {
        webhook_id: String => "webhook_id";
        name: String => "name";
        url: String => "url";
        secret: String => "secret";
        event_types: GoSlice<String> => "event_types";
        enabled: Option<bool> => "enabled";
    }
    }
    todo!("port handleUpdateWebhook from internal/asc/tools/webhooks.go")
}

/// Ported from `Registry.handleDeleteWebhook`.
async fn handle_delete_webhook(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { WebhookID string \"json:\\\"webhook_id\\\"\" }", "") {
        webhook_id: String => "webhook_id";
    }
    }
    todo!("port handleDeleteWebhook from internal/asc/tools/webhooks.go")
}

/// Ported from `Registry.handleListWebhookDeliveries`.
async fn handle_list_webhook_deliveries(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { WebhookID string \"json:\\\"webhook_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        webhook_id: String => "webhook_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListWebhookDeliveries from internal/asc/tools/webhooks.go")
}

/// Ported from `Registry.handlePingWebhook`.
async fn handle_ping_webhook(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { WebhookID string \"json:\\\"webhook_id\\\"\" }", "") {
        webhook_id: String => "webhook_id";
    }
    }
    todo!("port handlePingWebhook from internal/asc/tools/webhooks.go")
}

/// Ported from `Registry.handleRedeliverWebhookDelivery`.
async fn handle_redeliver_webhook_delivery(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { DeliveryID string \"json:\\\"delivery_id\\\"\" }", "") {
        delivery_id: String => "delivery_id";
    }
    }
    todo!("port handleRedeliverWebhookDelivery from internal/asc/tools/webhooks.go")
}

