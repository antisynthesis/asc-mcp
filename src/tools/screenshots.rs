//! Ported from internal/asc/tools/screenshots.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerScreenshotTools`.
pub(super) fn register_screenshot_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_screenshot_sets",
            "description": "List screenshot sets for a version localization",
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
                        "description": "Maximum number of sets to return (default 50)"
                    },
                    "localization_id": {
                        "type": "string",
                        "description": "The version localization ID"
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
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "List Screenshot Sets",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_screenshot_sets),
    );
    r.register(
        tool(r##"{
            "name": "list_screenshots",
            "description": "List screenshots in a screenshot set",
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
                        "description": "Maximum number of screenshots to return (default 50)"
                    },
                    "screenshot_set_id": {
                        "type": "string",
                        "description": "The screenshot set ID"
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
                    "screenshot_set_id"
                ]
            },
            "annotations": {
                "title": "List Screenshots",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_screenshots),
    );
    r.register(
        tool(r##"{
            "name": "get_screenshot",
            "description": "Get details of a specific screenshot",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "screenshot_id": {
                        "type": "string",
                        "description": "The screenshot ID"
                    }
                },
                "required": [
                    "screenshot_id"
                ]
            },
            "annotations": {
                "title": "Get Screenshot",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_screenshot),
    );
    r.register(
        tool(r##"{
            "name": "delete_screenshot",
            "description": "Delete a screenshot",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "screenshot_id": {
                        "type": "string",
                        "description": "The screenshot ID"
                    }
                },
                "required": [
                    "screenshot_id"
                ]
            },
            "annotations": {
                "title": "Delete Screenshot",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_screenshot),
    );
    r.register(
        tool(r##"{
            "name": "list_preview_sets",
            "description": "List app preview sets for a version localization",
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
                        "description": "Maximum number of sets to return (default 50)"
                    },
                    "localization_id": {
                        "type": "string",
                        "description": "The version localization ID"
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
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "List Preview Sets",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_preview_sets),
    );
    r.register(
        tool(r##"{
            "name": "list_previews",
            "description": "List app previews in a preview set",
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
                        "description": "Maximum number of previews to return (default 50)"
                    },
                    "preview_set_id": {
                        "type": "string",
                        "description": "The preview set ID"
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
                    "preview_set_id"
                ]
            },
            "annotations": {
                "title": "List Previews",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_previews),
    );
    r.register(
        tool(r##"{
            "name": "get_preview",
            "description": "Get details of a specific app preview",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "preview_id": {
                        "type": "string",
                        "description": "The preview ID"
                    }
                },
                "required": [
                    "preview_id"
                ]
            },
            "annotations": {
                "title": "Get Preview",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_preview),
    );
    r.register(
        tool(r##"{
            "name": "delete_preview",
            "description": "Delete an app preview",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "preview_id": {
                        "type": "string",
                        "description": "The preview ID"
                    }
                },
                "required": [
                    "preview_id"
                ]
            },
            "annotations": {
                "title": "Delete Preview",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_preview),
    );
}

/// Ported from `Registry.handleListScreenshotSets`.
async fn handle_list_screenshot_sets(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        localization_id: String => "localization_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListScreenshotSets from internal/asc/tools/screenshots.go")
}

/// Ported from `Registry.handleListScreenshots`.
async fn handle_list_screenshots(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ScreenshotSetID string \"json:\\\"screenshot_set_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        screenshot_set_id: String => "screenshot_set_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListScreenshots from internal/asc/tools/screenshots.go")
}

/// Ported from `Registry.handleGetScreenshot`.
async fn handle_get_screenshot(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ScreenshotID string \"json:\\\"screenshot_id\\\"\" }", "") {
        screenshot_id: String => "screenshot_id";
    }
    }
    todo!("port handleGetScreenshot from internal/asc/tools/screenshots.go")
}

/// Ported from `Registry.handleDeleteScreenshot`.
async fn handle_delete_screenshot(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ScreenshotID string \"json:\\\"screenshot_id\\\"\" }", "") {
        screenshot_id: String => "screenshot_id";
    }
    }
    todo!("port handleDeleteScreenshot from internal/asc/tools/screenshots.go")
}

/// Ported from `Registry.handleListPreviewSets`.
async fn handle_list_preview_sets(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        localization_id: String => "localization_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListPreviewSets from internal/asc/tools/screenshots.go")
}

/// Ported from `Registry.handleListPreviews`.
async fn handle_list_previews(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PreviewSetID string \"json:\\\"preview_set_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        preview_set_id: String => "preview_set_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListPreviews from internal/asc/tools/screenshots.go")
}

/// Ported from `Registry.handleGetPreview`.
async fn handle_get_preview(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PreviewID string \"json:\\\"preview_id\\\"\" }", "") {
        preview_id: String => "preview_id";
    }
    }
    todo!("port handleGetPreview from internal/asc/tools/screenshots.go")
}

/// Ported from `Registry.handleDeletePreview`.
async fn handle_delete_preview(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PreviewID string \"json:\\\"preview_id\\\"\" }", "") {
        preview_id: String => "preview_id";
    }
    }
    todo!("port handleDeletePreview from internal/asc/tools/screenshots.go")
}

