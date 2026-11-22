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
    todo!("port handleListAppClips from internal/asc/tools/appclips.go")
}

/// Ported from `Registry.handleGetAppClip`.
async fn handle_get_app_clip(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppClipID string \"json:\\\"app_clip_id\\\"\" }", "") {
        app_clip_id: String => "app_clip_id";
    }
    }
    todo!("port handleGetAppClip from internal/asc/tools/appclips.go")
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
    todo!("port handleListAppClipDefaultExperiences from internal/asc/tools/appclips.go")
}

/// Ported from `Registry.handleGetAppClipDefaultExperience`.
async fn handle_get_app_clip_default_experience(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ExperienceID string \"json:\\\"experience_id\\\"\" }", "") {
        experience_id: String => "experience_id";
    }
    }
    todo!("port handleGetAppClipDefaultExperience from internal/asc/tools/appclips.go")
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
    todo!("port handleListAppClipAdvancedExperiences from internal/asc/tools/appclips.go")
}

/// Ported from `Registry.handleGetAppClipAdvancedExperience`.
async fn handle_get_app_clip_advanced_experience(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ExperienceID string \"json:\\\"experience_id\\\"\" }", "") {
        experience_id: String => "experience_id";
    }
    }
    todo!("port handleGetAppClipAdvancedExperience from internal/asc/tools/appclips.go")
}

