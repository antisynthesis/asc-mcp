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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_sandbox_testers(opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list sandbox testers: {e}"))),
    };

    Ok(list_result(&format_sandbox_testers(&resp.data), &resp.data, &resp.links))
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.tester_id.is_empty() {
        return Ok(error_result("tester_id is required"));
    }

    let req = SandboxTesterUpdateRequest {
        data: SandboxTesterUpdateData {
            type_: "sandboxTesters".into(),
            id: params.tester_id.clone(),
            attributes: SandboxTesterUpdateAttributes {
                territory: params.territory.clone(),
                interruptable: params.interruptable,
                subscription_renewal_rate: params.subscription_renewal_rate.clone(),
            },
        },
    };

    let resp = match c.update_sandbox_tester(&params.tester_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update sandbox tester: {e}"))),
    };

    Ok(data_result(&format!("Sandbox tester updated:\n{}", format_sandbox_tester(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleClearSandboxTesterPurchaseHistory`.
async fn handle_clear_sandbox_tester_purchase_history(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { TesterIDs []string \"json:\\\"tester_ids\\\"\" }", "") {
        tester_ids: GoSlice<String> => "tester_ids";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.tester_ids.is_empty() {
        return Ok(error_result("tester_ids is required"));
    }

    let mut testers: GoSlice<ResourceIdentifier> = GoSlice::nil();
    for id in params.tester_ids.iter() {
        testers.push(ResourceIdentifier { type_: "sandboxTesters".into(), id: id.clone() });
    }

    let req = SandboxTesterClearPurchaseHistoryRequest {
        data: SandboxTesterClearPurchaseHistoryData {
            type_: "sandboxTestersClearPurchaseHistoryRequest".into(),
            relationships: SandboxTesterClearPurchaseHistoryRelationships {
                sandbox_testers: RelationshipDataList { data: testers },
            },
        },
    };

    let resp = match c.clear_sandbox_tester_purchase_history(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to clear sandbox tester purchase history: {e}"))),
    };

    Ok(data_result(
        &format!(
            "Purchase history cleared for {} sandbox testers (request ID: {})",
            params.tester_ids.len(),
            resp.data.id
        ),
        &resp.data,
    ))
}

/// Ported from `formatSandboxTesters`.
fn format_sandbox_testers(testers: &[SandboxTester]) -> String {
    if testers.is_empty() {
        return "No sandbox testers found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} sandbox testers:\n\n", testers.len());

    for tester in testers {
        sb.push_str(&format_sandbox_tester(tester));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatSandboxTester`.
fn format_sandbox_tester(tester: &SandboxTester) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", tester.id);
    let _ = writeln!(sb, "Email: {}", tester.attributes.email);
    let _ = writeln!(sb, "Name: {} {}", tester.attributes.first_name, tester.attributes.last_name);
    if !tester.attributes.app_store_territory.is_empty() {
        let _ = writeln!(sb, "Territory: {}", tester.attributes.app_store_territory);
    }
    let _ = writeln!(sb, "Interruptable: {}", tester.attributes.interruptable);
    sb
}
