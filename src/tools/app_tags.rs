//! App tag tools.

#![allow(unused_imports)]

use super::prelude::*;
use super::pricing::format_territories;

/// Registers this module's tools in definition order.
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

/// Handles `list_app_tags`.
async fn handle_list_app_tags(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_id: String => "app_id";
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
    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }
    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }
    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_tags(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list app tags: {e}"))),
    };
    Ok(list_result(&format_app_tags(&resp.data), &resp.data, &resp.links))
}

/// Handles `update_app_tag`.
async fn handle_update_app_tag(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppTagID string \"json:\\\"app_tag_id\\\"\"; VisibleInAppStore *bool \"json:\\\"visible_in_app_store\\\"\" }", "") {
        app_tag_id: String => "app_tag_id";
        visible_in_app_store: Option<bool> => "visible_in_app_store";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.app_tag_id.is_empty() {
        return Ok(error_result("app_tag_id is required"));
    }
    if params.visible_in_app_store.is_none() {
        return Ok(error_result("visible_in_app_store is required"));
    }
    let req = AppTagUpdateRequest {
        data: AppTagUpdateData {
            type_: "appTags".into(),
            id: params.app_tag_id.clone(),
            attributes: AppTagUpdateAttributes { visible_in_app_store: params.visible_in_app_store },
        },
    };
    let resp = match c.update_app_tag(&params.app_tag_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update app tag: {e}"))),
    };
    Ok(data_result(&format!("App tag updated:\n{}", format_app_tag(&resp.data)), &resp.data))
}

/// Handles `list_app_tag_territories`.
async fn handle_list_app_tag_territories(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppTagID string \"json:\\\"app_tag_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\" }", "") {
        app_tag_id: String => "app_tag_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.app_tag_id.is_empty() {
        return Ok(error_result("app_tag_id is required"));
    }
    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }
    let opts = limit_opts(limit);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_tag_territories(&params.app_tag_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list app tag territories: {e}"))),
    };
    Ok(list_result(&format_territories(&resp.data), &resp.data, &resp.links))
}

fn format_app_tags(tags: &[AppTag]) -> String {
    if tags.is_empty() {
        return "No app tags found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} app tags:\n\n", tags.len());
    for tag in tags {
        sb.push_str(&format_app_tag(tag));
        sb.push_str("\n---\n");
    }
    sb
}

fn format_app_tag(tag: &AppTag) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "App Tag ID: {}", tag.id);
    if !tag.attributes.name.is_empty() {
        let _ = writeln!(sb, "Name: {}", tag.attributes.name);
    }
    if let Some(v) = tag.attributes.visible_in_app_store {
        let _ = writeln!(sb, "Visible In App Store: {}", v);
    }
    sb
}
