//! Ported from internal/asc/tools/preorders.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerPreOrderTools`.
pub(super) fn register_pre_order_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "end_app_availability_pre_order",
            "description": "End pre-order availability for one or more territory availabilities, releasing the app in those territories",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "territory_availability_ids": {
                        "type": "array",
                        "description": "Territory availability IDs to end pre-order for (from list_territory_availabilities)",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "territory_availability_ids"
                ]
            },
            "annotations": {
                "title": "End App Availability Pre Order",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_end_app_availability_pre_order),
    );
}

/// Ported from `Registry.handleEndAppAvailabilityPreOrder`.
async fn handle_end_app_availability_pre_order(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { TerritoryAvailabilityIDs []string \"json:\\\"territory_availability_ids\\\"\" }", "") {
        territory_availability_ids: GoSlice<String> => "territory_availability_ids";
    }
    }
    todo!("port handleEndAppAvailabilityPreOrder from internal/asc/tools/preorders.go")
}

