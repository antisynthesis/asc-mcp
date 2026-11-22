//! Ported from internal/asc/tools/appclips.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerAppClipTools`.
pub(super) fn register_app_clip_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_app_clips",
            "description": "List app clips for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID"
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
                        "description": "Maximum number of app clips to return (default 50)"
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
                    "app_id"
                ]
            },
            "annotations": {
                "title": "List App Clips",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_app_clips),
    );
    r.register(
        tool(r##"{
            "name": "get_app_clip",
            "description": "Get details of a specific app clip",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_clip_id": {
                        "type": "string",
                        "description": "The app clip ID"
                    }
                },
                "required": [
                    "app_clip_id"
                ]
            },
            "annotations": {
                "title": "Get App Clip",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_clip),
    );
    r.register(
        tool(r##"{
            "name": "list_app_clip_default_experiences",
            "description": "List default experiences for an app clip",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_clip_id": {
                        "type": "string",
                        "description": "The app clip ID"
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
                        "description": "Maximum number of experiences to return (default 50)"
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
                    "app_clip_id"
                ]
            },
            "annotations": {
                "title": "List App Clip Default Experiences",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_app_clip_default_experiences),
    );
    r.register(
        tool(r##"{
            "name": "get_app_clip_default_experience",
            "description": "Get details of a specific app clip default experience",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "experience_id": {
                        "type": "string",
                        "description": "The default experience ID"
                    }
                },
                "required": [
                    "experience_id"
                ]
            },
            "annotations": {
                "title": "Get App Clip Default Experience",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_clip_default_experience),
    );
    r.register(
        tool(r##"{
            "name": "list_app_clip_advanced_experiences",
            "description": "List advanced experiences for an app clip",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_clip_id": {
                        "type": "string",
                        "description": "The app clip ID"
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
                        "description": "Maximum number of experiences to return (default 50)"
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
                    "app_clip_id"
                ]
            },
            "annotations": {
                "title": "List App Clip Advanced Experiences",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_app_clip_advanced_experiences),
    );
    r.register(
        tool(r##"{
            "name": "get_app_clip_advanced_experience",
            "description": "Get details of a specific app clip advanced experience",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "experience_id": {
                        "type": "string",
                        "description": "The advanced experience ID"
                    }
                },
                "required": [
                    "experience_id"
                ]
            },
            "annotations": {
                "title": "Get App Clip Advanced Experience",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_clip_advanced_experience),
    );
}

/// Ported from `Registry.handleListAppClips`.
async fn handle_list_app_clips(c: Arc<Client>, args: Args) -> HandlerResult {
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
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_clips(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list app clips: {e}"))),
    };

    Ok(list_result(&format_app_clips(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetAppClip`.
async fn handle_get_app_clip(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppClipID string \"json:\\\"app_clip_id\\\"\" }", "") {
        app_clip_id: String => "app_clip_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_clip_id.is_empty() {
        return Ok(error_result("app_clip_id is required"));
    }

    let resp = match c.get_app_clip(&params.app_clip_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get app clip: {e}"))),
    };

    Ok(data_result(&format_app_clip(&resp.data), &resp.data))
}

/// Ported from `Registry.handleListAppClipDefaultExperiences`.
async fn handle_list_app_clip_default_experiences(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppClipID string \"json:\\\"app_clip_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_clip_id: String => "app_clip_id";
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

    if params.app_clip_id.is_empty() {
        return Ok(error_result("app_clip_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_clip_default_experiences(&params.app_clip_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list app clip default experiences: {e}"))),
    };

    Ok(list_result(&format_app_clip_default_experiences(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetAppClipDefaultExperience`.
async fn handle_get_app_clip_default_experience(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ExperienceID string \"json:\\\"experience_id\\\"\" }", "") {
        experience_id: String => "experience_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.experience_id.is_empty() {
        return Ok(error_result("experience_id is required"));
    }

    let resp = match c.get_app_clip_default_experience(&params.experience_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get app clip default experience: {e}"))),
    };

    Ok(data_result(&format_app_clip_default_experience(&resp.data), &resp.data))
}

/// Ported from `Registry.handleListAppClipAdvancedExperiences`.
async fn handle_list_app_clip_advanced_experiences(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppClipID string \"json:\\\"app_clip_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_clip_id: String => "app_clip_id";
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

    if params.app_clip_id.is_empty() {
        return Ok(error_result("app_clip_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_clip_advanced_experiences(&params.app_clip_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list app clip advanced experiences: {e}"))),
    };

    Ok(list_result(&format_app_clip_advanced_experiences(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetAppClipAdvancedExperience`.
async fn handle_get_app_clip_advanced_experience(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ExperienceID string \"json:\\\"experience_id\\\"\" }", "") {
        experience_id: String => "experience_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.experience_id.is_empty() {
        return Ok(error_result("experience_id is required"));
    }

    let resp = match c.get_app_clip_advanced_experience(&params.experience_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get app clip advanced experience: {e}"))),
    };

    Ok(data_result(&format_app_clip_advanced_experience(&resp.data), &resp.data))
}

/// Ported from `formatAppClips`.
fn format_app_clips(clips: &[AppClip]) -> String {
    if clips.is_empty() {
        return "No app clips found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} app clips:\n\n", clips.len());

    for clip in clips {
        sb.push_str(&format_app_clip(clip));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatAppClip`.
fn format_app_clip(clip: &AppClip) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", clip.id);
    let _ = writeln!(sb, "Bundle ID: {}", clip.attributes.bundle_id);
    sb
}

/// Ported from `formatAppClipDefaultExperiences`.
fn format_app_clip_default_experiences(experiences: &[AppClipDefaultExperience]) -> String {
    if experiences.is_empty() {
        return "No app clip default experiences found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} default experiences:\n\n", experiences.len());

    for exp in experiences {
        sb.push_str(&format_app_clip_default_experience(exp));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatAppClipDefaultExperience`.
fn format_app_clip_default_experience(exp: &AppClipDefaultExperience) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", exp.id);
    if !exp.attributes.action.is_empty() {
        let _ = writeln!(sb, "Action: {}", exp.attributes.action);
    }
    sb
}

/// Ported from `formatAppClipAdvancedExperiences`.
fn format_app_clip_advanced_experiences(experiences: &[AppClipAdvancedExperience]) -> String {
    if experiences.is_empty() {
        return "No app clip advanced experiences found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} advanced experiences:\n\n", experiences.len());

    for exp in experiences {
        sb.push_str(&format_app_clip_advanced_experience(exp));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatAppClipAdvancedExperience`.
fn format_app_clip_advanced_experience(exp: &AppClipAdvancedExperience) -> String {
    let a = &exp.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", exp.id);
    if !a.action.is_empty() {
        let _ = writeln!(sb, "Action: {}", a.action);
    }
    if !a.link.is_empty() {
        let _ = writeln!(sb, "Link: {}", a.link);
    }
    if !a.status.is_empty() {
        let _ = writeln!(sb, "Status: {}", a.status);
    }
    if !a.business_category.is_empty() {
        let _ = writeln!(sb, "Business Category: {}", a.business_category);
    }
    if !a.default_language.is_empty() {
        let _ = writeln!(sb, "Default Language: {}", a.default_language);
    }
    let _ = writeln!(sb, "Is Powered By: {}", bool_v(a.is_powered_by));
    sb
}
