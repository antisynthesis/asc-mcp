//! Ported from internal/asc/tools/availability.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerAvailabilityTools`.
pub(super) fn register_availability_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "get_app_availability",
            "description": "Get the availability settings for an app",
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
                "title": "Get App Availability",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_availability),
    );
    r.register(
        tool(r##"{
            "name": "create_app_availability",
            "description": "Create or update availability settings for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The app ID"
                    },
                    "available_in_new_territories": {
                        "type": "boolean",
                        "description": "Whether app should be available in new territories by default"
                    },
                    "territory_ids": {
                        "type": "array",
                        "description": "List of territory IDs where the app should be available",
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
                "title": "Create App Availability",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_app_availability),
    );
    r.register(
        tool(r##"{
            "name": "list_territory_availabilities",
            "description": "List territory availability settings for an app availability",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "availability_id": {
                        "type": "string",
                        "description": "The app availability ID"
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
                        "description": "Maximum number of results to return (default 100)"
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
                    "availability_id"
                ]
            },
            "annotations": {
                "title": "List Territory Availabilities",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_territory_availabilities),
    );
}

/// Ported from `Registry.handleGetAppAvailability`.
async fn handle_get_app_availability(c: Arc<Client>, args: Args) -> HandlerResult {
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

    let resp = match c.get_app_availability(&params.app_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get app availability: {e}"))),
    };

    Ok(data_result(&format_app_availability(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateAppAvailability`.
async fn handle_create_app_availability(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; AvailableInNewTerritories *bool \"json:\\\"available_in_new_territories\\\"\"; TerritoryIDs []string \"json:\\\"territory_ids\\\"\" }", "") {
        app_id: String => "app_id";
        available_in_new_territories: Option<bool> => "available_in_new_territories";
        territory_ids: GoSlice<String> => "territory_ids";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }

    let avail_in_new = params.available_in_new_territories.unwrap_or(true);

    // The v2 endpoint takes territory availabilities as inline-created
    // resources referenced by temporary IDs.
    let available = true;
    let mut refs: GoSlice<ResourceIdentifier> = GoSlice::nil();
    let mut included: GoSlice<TerritoryAvailabilityInlineCreate> = GoSlice::nil();
    for tid in params.territory_ids.iter() {
        let temp_id = format!("${{{tid}}}");
        refs.push(ResourceIdentifier { type_: "territoryAvailabilities".into(), id: temp_id.clone() });
        included.push(TerritoryAvailabilityInlineCreate {
            type_: "territoryAvailabilities".into(),
            id: temp_id,
            attributes: Some(TerritoryAvailabilityInlineAttributes { available: Some(available), ..Default::default() }),
            relationships: Some(TerritoryAvailabilityInlineCreateRelationships {
                territory: RelationshipData { data: ResourceIdentifier { type_: "territories".into(), id: tid.clone() } },
            }),
        });
    }

    let req = AppAvailabilityCreateRequest {
        data: AppAvailabilityCreateData {
            type_: "appAvailabilities".into(),
            attributes: AppAvailabilityCreateAttributes { available_in_new_territories: avail_in_new },
            relationships: AppAvailabilityCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
                territory_availabilities: RelationshipDataList { data: refs },
            },
        },
        included,
    };

    let resp = match c.create_app_availability(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create app availability: {e}"))),
    };

    Ok(data_result(&format!("App availability created:\n{}", format_app_availability(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleListTerritoryAvailabilities`.
async fn handle_list_territory_availabilities(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AvailabilityID string \"json:\\\"availability_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        availability_id: String => "availability_id";
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

    if params.availability_id.is_empty() {
        return Ok(error_result("availability_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 100;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_territory_availabilities(&params.availability_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list territory availabilities: {e}"))),
    };

    Ok(list_result(&format_territory_availabilities(&resp.data), &resp.data, &resp.links))
}

/// Ported from `formatAppAvailability`.
fn format_app_availability(avail: &AppAvailability) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", avail.id);
    let _ = write!(sb, "Available in New Territories: {}\n", avail.attributes.available_in_new_territories);
    sb
}

/// Ported from `formatTerritoryAvailabilities`.
fn format_territory_availabilities(availabilities: &[TerritoryAvailability]) -> String {
    if availabilities.is_empty() {
        return "No territory availabilities found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} territory availabilities:\n\n", availabilities.len());

    for avail in availabilities {
        let _ = write!(sb, "ID: {}\n", avail.id);
        let _ = write!(sb, "Available: {}\n", avail.attributes.available);
        let _ = write!(sb, "Pre-Order Enabled: {}\n", avail.attributes.pre_order_enabled);
        if let Some(d) = &avail.attributes.release_date {
            let _ = write!(sb, "Release Date: {}\n", d.format("2006-01-02"));
        }
        sb.push_str("\n---\n");
    }

    sb
}
