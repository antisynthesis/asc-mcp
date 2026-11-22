//! Ported from internal/asc/tools/sandbox.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerSandboxTools`.
pub(super) fn register_sandbox_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_sandbox_testers",
            "description": "List sandbox testers for the account",
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
                        "description": "Maximum number of testers to return (default 50)"
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
                "title": "List Sandbox Testers",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_sandbox_testers),
    );
    r.register(
        tool(r##"{
            "name": "update_sandbox_tester",
            "description": "Update a sandbox tester's territory or interruptable setting",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "interruptable": {
                        "type": "boolean",
                        "description": "Whether purchases can be interrupted for testing"
                    },
                    "subscription_renewal_rate": {
                        "type": "string",
                        "description": "Subscription renewal rate: MONTHLY_RENEWAL_EVERY_ONE_HOUR, etc."
                    },
                    "territory": {
                        "type": "string",
                        "description": "New App Store territory code"
                    },
                    "tester_id": {
                        "type": "string",
                        "description": "The sandbox tester ID"
                    }
                },
                "required": [
                    "tester_id"
                ]
            },
            "annotations": {
                "title": "Update Sandbox Tester",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_sandbox_tester),
    );
    r.register(
        tool(r##"{
            "name": "clear_sandbox_tester_purchase_history",
            "description": "Clear the purchase history for one or more sandbox testers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "tester_ids": {
                        "type": "array",
                        "description": "The sandbox tester IDs whose purchase history should be cleared",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "tester_ids"
                ]
            },
            "annotations": {
                "title": "Clear Sandbox Tester Purchase History",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_clear_sandbox_tester_purchase_history),
    );
}

/// Ported from `Registry.handleListSandboxTesters`.
async fn handle_list_sandbox_testers(c: Arc<Client>, args: Args) -> HandlerResult {
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
    todo!("port handleListSandboxTesters from internal/asc/tools/sandbox.go")
}

/// Ported from `Registry.handleUpdateSandboxTester`.
async fn handle_update_sandbox_tester(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { TesterID string \"json:\\\"tester_id\\\"\"; Territory string \"json:\\\"territory\\\"\"; Interruptable *bool \"json:\\\"interruptable\\\"\"; SubscriptionRenewalRate string \"json:\\\"subscription_renewal_rate\\\"\" }", "") {
        tester_id: String => "tester_id";
        territory: String => "territory";
        interruptable: Option<bool> => "interruptable";
        subscription_renewal_rate: String => "subscription_renewal_rate";
    }
    }
    todo!("port handleUpdateSandboxTester from internal/asc/tools/sandbox.go")
}

/// Ported from `Registry.handleClearSandboxTesterPurchaseHistory`.
async fn handle_clear_sandbox_tester_purchase_history(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { TesterIDs []string \"json:\\\"tester_ids\\\"\" }", "") {
        tester_ids: GoSlice<String> => "tester_ids";
    }
    }
    todo!("port handleClearSandboxTesterPurchaseHistory from internal/asc/tools/sandbox.go")
}

