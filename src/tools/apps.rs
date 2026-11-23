//! App tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
pub(super) fn register_app_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_apps",
            "description": "List all apps in your App Store Connect account. Returns app name, bundle ID, SKU, and primary locale for each app.",
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
                        "description": "Maximum number of apps to return (default: 50, max: 200)",
                        "default": 50
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
                "title": "List Apps",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_apps),
    );
    r.register(
        tool(r##"{
            "name": "get_app",
            "description": "Get detailed information about a specific app by its App Store Connect ID.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App Store Connect ID of the app"
                    }
                },
                "required": [
                    "app_id"
                ]
            },
            "annotations": {
                "title": "Get App",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app),
    );
    r.register(
        tool(r##"{
            "name": "get_app_versions",
            "description": "Get all App Store versions for a specific app, including version string, platform, state, and release information.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App Store Connect ID of the app"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of versions to return (default: 20)",
                        "default": 20
                    }
                },
                "required": [
                    "app_id"
                ]
            },
            "annotations": {
                "title": "Get App Versions",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_versions),
    );
}

/// Handles `list_apps`.
async fn handle_list_apps(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params { limit: 50, ..Default::default() };
    if args.is_some()
        && let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
            return Err(format!("invalid arguments: {e}"));
        }
    if params.limit <= 0 {
        params.limit = 50;
    }
    if params.limit > 200 {
        params.limit = 200;
    }

    let opts = list_opts(params.limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_apps(opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list apps: {e}"))),
    };

    if resp.data.is_empty() {
        return Ok(list_result("No apps found in your App Store Connect account.", &resp.data, &resp.links));
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} apps:\n\n", resp.data.len());
    for app in resp.data.iter() {
        let _ = writeln!(sb, "**{}**", app.attributes.name);
        let _ = writeln!(sb, "  - ID: {}", app.id);
        let _ = writeln!(sb, "  - Bundle ID: {}", app.attributes.bundle_id);
        let _ = writeln!(sb, "  - SKU: {}", app.attributes.sku);
        let _ = writeln!(sb, "  - Primary Locale: {}", app.attributes.primary_locale);
        sb.push('\n');
    }

    Ok(list_result(&sb, &resp.data, &resp.links))
}

/// Handles `get_app`.
async fn handle_get_app(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
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

    let resp = match c.get_app(&params.app_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get app: {e}"))),
    };

    let app = &resp.data;
    let mut sb = String::new();
    let _ = write!(sb, "**{}**\n\n", app.attributes.name);
    let _ = writeln!(sb, "- ID: {}", app.id);
    let _ = writeln!(sb, "- Bundle ID: {}", app.attributes.bundle_id);
    let _ = writeln!(sb, "- SKU: {}", app.attributes.sku);
    let _ = writeln!(sb, "- Primary Locale: {}", app.attributes.primary_locale);
    let _ = writeln!(sb, "- Made for Kids: {}", bool_v(app.attributes.is_or_ever_was_made_for_kids));

    if !app.attributes.content_rights_declaration.is_empty() {
        let _ = writeln!(sb, "- Content Rights: {}", app.attributes.content_rights_declaration);
    }

    Ok(data_result(&sb, &resp.data))
}

/// Handles `get_app_versions`.
async fn handle_get_app_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
    }
    }
    let mut params = Params { limit: 20, ..Default::default() };
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }

    let resp = match c.get_app_versions(&params.app_id, limit_opts(params.limit).as_ref()).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get app versions: {e}"))),
    };

    if resp.data.is_empty() {
        return Ok(data_result("No versions found for this app.", &resp.data));
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} versions:\n\n", resp.data.len());
    for version in resp.data.iter() {
        let a = &version.attributes;
        let _ = writeln!(sb, "**Version {}** ({})", a.version_string, a.platform);
        let _ = writeln!(sb, "  - ID: {}", version.id);
        let _ = writeln!(sb, "  - State: {}", a.app_store_state);
        let _ = writeln!(sb, "  - Release Type: {}", a.release_type);
        let _ = writeln!(sb, "  - Downloadable: {}", bool_v(a.downloadable));
        if let Some(created) = &a.created_date {
            let _ = writeln!(sb, "  - Created: {}", created.format("2006-01-02 15:04"));
        }
        sb.push('\n');
    }

    Ok(data_result(&sb, &resp.data))
}

