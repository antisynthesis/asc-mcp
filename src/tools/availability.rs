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
    todo!("port handleGetAppAvailability from internal/asc/tools/availability.go")
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
    todo!("port handleCreateAppAvailability from internal/asc/tools/availability.go")
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
    todo!("port handleListTerritoryAvailabilities from internal/asc/tools/availability.go")
}

