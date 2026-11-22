//! Ported from internal/asc/tools/builds.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerBuildTools`.
pub(super) fn register_build_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_builds",
            "description": "List builds for your apps. Can filter by app ID. Returns version, processing state, upload date, and expiration information.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "Optional: Filter builds by app ID"
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
                        "description": "Maximum number of builds to return (default: 20, max: 200)",
                        "default": 20
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
                "title": "List Builds",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_builds),
    );
    r.register(
        tool(r##"{
            "name": "get_build",
            "description": "Get detailed information about a specific build by its ID, including version, processing state, and TestFlight information.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "build_id": {
                        "type": "string",
                        "description": "The App Store Connect ID of the build"
                    }
                },
                "required": [
                    "build_id"
                ]
            },
            "annotations": {
                "title": "Get Build",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_build),
    );
}

/// Ported from `Registry.handleListBuilds`.
async fn handle_list_builds(c: Arc<Client>, args: Args) -> HandlerResult {
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
    todo!("port handleListBuilds from internal/asc/tools/builds.go")
}

/// Ported from `Registry.handleGetBuild`.
async fn handle_get_build(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildID string \"json:\\\"build_id\\\"\" }", "") {
        build_id: String => "build_id";
    }
    }
    todo!("port handleGetBuild from internal/asc/tools/builds.go")
}

