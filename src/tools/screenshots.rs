//! Screenshot and preview tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
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

/// Handles `list_screenshot_sets`.
async fn handle_list_screenshot_sets(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        localization_id: String => "localization_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_screenshot_sets(&params.localization_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list screenshot sets: {e}"))),
    };

    Ok(list_result(&format_screenshot_sets(&resp.data), &resp.data, &resp.links))
}

/// Handles `list_screenshots`.
async fn handle_list_screenshots(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { ScreenshotSetID string \"json:\\\"screenshot_set_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        screenshot_set_id: String => "screenshot_set_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.screenshot_set_id.is_empty() {
        return Ok(error_result("screenshot_set_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_screenshots(&params.screenshot_set_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list screenshots: {e}"))),
    };

    Ok(list_result(&format_screenshots(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_screenshot`.
async fn handle_get_screenshot(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { ScreenshotID string \"json:\\\"screenshot_id\\\"\" }", "") {
        screenshot_id: String => "screenshot_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.screenshot_id.is_empty() {
        return Ok(error_result("screenshot_id is required"));
    }

    let resp = match c.get_app_screenshot(&params.screenshot_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get screenshot: {e}"))),
    };

    Ok(data_result(&format_screenshot(&resp.data), &resp.data))
}

/// Handles `delete_screenshot`.
async fn handle_delete_screenshot(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { ScreenshotID string \"json:\\\"screenshot_id\\\"\" }", "") {
        screenshot_id: String => "screenshot_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.screenshot_id.is_empty() {
        return Ok(error_result("screenshot_id is required"));
    }

    if let Err(e) = c.delete_app_screenshot(&params.screenshot_id).await {
        return Ok(error_result(format!("Failed to delete screenshot: {e}")));
    }

    Ok(CallToolResult::success("Screenshot deleted successfully"))
}

/// Handles `list_preview_sets`.
async fn handle_list_preview_sets(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        localization_id: String => "localization_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_preview_sets(&params.localization_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list preview sets: {e}"))),
    };

    Ok(list_result(&format_preview_sets(&resp.data), &resp.data, &resp.links))
}

/// Handles `list_previews`.
async fn handle_list_previews(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { PreviewSetID string \"json:\\\"preview_set_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        preview_set_id: String => "preview_set_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.preview_set_id.is_empty() {
        return Ok(error_result("preview_set_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_previews(&params.preview_set_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list previews: {e}"))),
    };

    Ok(list_result(&format_previews(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_preview`.
async fn handle_get_preview(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { PreviewID string \"json:\\\"preview_id\\\"\" }", "") {
        preview_id: String => "preview_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.preview_id.is_empty() {
        return Ok(error_result("preview_id is required"));
    }

    let resp = match c.get_app_preview(&params.preview_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get preview: {e}"))),
    };

    Ok(data_result(&format_preview(&resp.data), &resp.data))
}

/// Handles `delete_preview`.
async fn handle_delete_preview(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { PreviewID string \"json:\\\"preview_id\\\"\" }", "") {
        preview_id: String => "preview_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.preview_id.is_empty() {
        return Ok(error_result("preview_id is required"));
    }

    if let Err(e) = c.delete_app_preview(&params.preview_id).await {
        return Ok(error_result(format!("Failed to delete preview: {e}")));
    }

    Ok(CallToolResult::success("Preview deleted successfully"))
}

fn format_screenshot_sets(sets: &[AppScreenshotSet]) -> String {
    if sets.is_empty() {
        return "No screenshot sets found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} screenshot sets:\n\n", sets.len());

    for set in sets {
        let _ = writeln!(sb, "ID: {}", set.id);
        let _ = writeln!(sb, "Display Type: {}", set.attributes.screenshot_display_type);
        sb.push_str("---\n");
    }

    sb
}

fn format_screenshots(screenshots: &[AppScreenshot]) -> String {
    if screenshots.is_empty() {
        return "No screenshots found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} screenshots:\n\n", screenshots.len());

    for ss in screenshots {
        sb.push_str(&format_screenshot(ss));
        sb.push_str("---\n");
    }

    sb
}

fn format_screenshot(ss: &AppScreenshot) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", ss.id);
    let _ = writeln!(sb, "File Name: {}", ss.attributes.file_name);
    let _ = writeln!(sb, "File Size: {} bytes", ss.attributes.file_size);
    if let Some(img) = &ss.attributes.image_asset {
        let _ = writeln!(sb, "Dimensions: {}x{}", img.width, img.height);
    }
    if let Some(st) = &ss.attributes.asset_delivery_state {
        let _ = writeln!(sb, "State: {}", st.state);
    }
    sb
}

fn format_preview_sets(sets: &[AppPreviewSet]) -> String {
    if sets.is_empty() {
        return "No preview sets found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} preview sets:\n\n", sets.len());

    for set in sets {
        let _ = writeln!(sb, "ID: {}", set.id);
        let _ = writeln!(sb, "Preview Type: {}", set.attributes.preview_type);
        sb.push_str("---\n");
    }

    sb
}

fn format_previews(previews: &[AppPreview]) -> String {
    if previews.is_empty() {
        return "No previews found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} previews:\n\n", previews.len());

    for p in previews {
        sb.push_str(&format_preview(p));
        sb.push_str("---\n");
    }

    sb
}

fn format_preview(p: &AppPreview) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", p.id);
    let _ = writeln!(sb, "File Name: {}", p.attributes.file_name);
    let _ = writeln!(sb, "File Size: {} bytes", p.attributes.file_size);
    if !p.attributes.mime_type.is_empty() {
        let _ = writeln!(sb, "MIME Type: {}", p.attributes.mime_type);
    }
    if !p.attributes.video_url.is_empty() {
        let _ = writeln!(sb, "Video URL: {}", p.attributes.video_url);
    }
    if let Some(st) = &p.attributes.asset_delivery_state {
        let _ = writeln!(sb, "State: {}", st.state);
    }
    sb
}
