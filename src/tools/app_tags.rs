//! Ported from internal/asc/tools/app_tags.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerAppTagTools`.
pub(super) fn register_app_tag_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_app_tags",
            "description": "List the Apple-created tags applied to an app on the App Store",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to list tags for"
                    },
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
                        "description": "JSON:API filter map. Supported key: visibleInAppStore. Values are arrays, e.g. {\"visibleInAppStore\": [\"true\"]}."
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response (supported: territories).",
                        "items": {
                            "type": "string"
                        }
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of tags to return (default 50, max 200)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Supported: name, -name.",
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
                "title": "List App Tags",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_app_tags),
    );
    r.register(
        tool(r##"{
            "name": "update_app_tag",
            "description": "Update an Apple-created app tag's App Store visibility; set visible_in_app_store=false to opt the app out of the tag",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_tag_id": {
                        "type": "string",
                        "description": "The app tag ID"
                    },
                    "visible_in_app_store": {
                        "type": "boolean",
                        "description": "Whether the tag should be visible for the app on the App Store"
                    }
                },
                "required": [
                    "app_tag_id",
                    "visible_in_app_store"
                ]
            },
            "annotations": {
                "title": "Update App Tag",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_app_tag),
    );
    r.register(
        tool(r##"{
            "name": "list_app_tag_territories",
            "description": "List the territories where an Apple-created app tag applies",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_tag_id": {
                        "type": "string",
                        "description": "The app tag ID"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of territories to return (default 50, max 200)"
                    }
                },
                "required": [
                    "app_tag_id"
                ]
            },
            "annotations": {
                "title": "List App Tag Territories",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_app_tag_territories),
    );
}

/// Ported from `Registry.handleListAppTags`.
async fn handle_list_app_tags(c: Arc<Client>, args: Args) -> HandlerResult {
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
    todo!("port handleListAppTags from internal/asc/tools/app_tags.go")
}

/// Ported from `Registry.handleUpdateAppTag`.
async fn handle_update_app_tag(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppTagID string \"json:\\\"app_tag_id\\\"\"; VisibleInAppStore *bool \"json:\\\"visible_in_app_store\\\"\" }", "") {
        app_tag_id: String => "app_tag_id";
        visible_in_app_store: Option<bool> => "visible_in_app_store";
    }
    }
    todo!("port handleUpdateAppTag from internal/asc/tools/app_tags.go")
}

/// Ported from `Registry.handleListAppTagTerritories`.
async fn handle_list_app_tag_territories(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppTagID string \"json:\\\"app_tag_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\" }", "") {
        app_tag_id: String => "app_tag_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
    }
    }
    todo!("port handleListAppTagTerritories from internal/asc/tools/app_tags.go")
}

