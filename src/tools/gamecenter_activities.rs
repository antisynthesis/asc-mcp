//! Ported from internal/asc/tools/gamecenter_activities.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerGameCenterActivityTools`.
pub(super) fn register_game_center_activity_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_game_center_activities",
            "description": "List the Game Center activities for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"referenceName\": [\"Weekly Ladder\"]} becomes filter[referenceName]=Weekly%20Ladder."
                    },
                    "game_center_detail_id": {
                        "type": "string",
                        "description": "The Game Center detail ID"
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
                        "description": "Maximum number of activities to return (default 50, max 200)"
                    }
                },
                "required": [
                    "game_center_detail_id"
                ]
            },
            "annotations": {
                "title": "List Game Center Activities",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_game_center_activities),
    );
    r.register(
        tool(r##"{
            "name": "get_game_center_activity",
            "description": "Get details of a specific Game Center activity",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "activity_id": {
                        "type": "string",
                        "description": "The activity ID"
                    }
                },
                "required": [
                    "activity_id"
                ]
            },
            "annotations": {
                "title": "Get Game Center Activity",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_game_center_activity),
    );
    r.register(
        tool(r##"{
            "name": "create_game_center_activity",
            "description": "Create a Game Center activity along with its initial version. Provide exactly one of game_center_detail_id (app-owned) or game_center_group_id (shared across a group). fallback_url is set on the inline initial version and is opened when a player's device cannot launch the activity.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "fallback_url": {
                        "type": "string",
                        "description": "URL opened when the player's device cannot launch the activity. Set on the inline initial version."
                    },
                    "game_center_detail_id": {
                        "type": "string",
                        "description": "The Game Center detail ID to own the activity"
                    },
                    "game_center_group_id": {
                        "type": "string",
                        "description": "The Game Center group ID to own the activity instead of a single app"
                    },
                    "maximum_players_count": {
                        "type": "integer",
                        "description": "Maximum number of players the activity supports"
                    },
                    "minimum_players_count": {
                        "type": "integer",
                        "description": "Minimum number of players the activity supports"
                    },
                    "play_style": {
                        "type": "string",
                        "description": "Play style: ASYNCHRONOUS or SYNCHRONOUS"
                    },
                    "properties": {
                        "type": "object",
                        "description": "Arbitrary string key/value pairs handed to the game when the activity launches"
                    },
                    "reference_name": {
                        "type": "string",
                        "description": "Internal reference name"
                    },
                    "supports_party_code": {
                        "type": "boolean",
                        "description": "Whether players can join the activity with a party code"
                    },
                    "vendor_identifier": {
                        "type": "string",
                        "description": "Unique identifier for the activity"
                    }
                },
                "required": [
                    "reference_name",
                    "vendor_identifier"
                ]
            },
            "annotations": {
                "title": "Create Game Center Activity",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_game_center_activity),
    );
    r.register(
        tool(r##"{
            "name": "update_game_center_activity",
            "description": "Update a Game Center activity. The vendor identifier is immutable; use update_game_center_activity_version to change the fallback URL.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "activity_id": {
                        "type": "string",
                        "description": "The activity ID"
                    },
                    "archived": {
                        "type": "boolean",
                        "description": "Archive the activity"
                    },
                    "maximum_players_count": {
                        "type": "integer",
                        "description": "Updated maximum number of players"
                    },
                    "minimum_players_count": {
                        "type": "integer",
                        "description": "Updated minimum number of players"
                    },
                    "play_style": {
                        "type": "string",
                        "description": "Updated play style: ASYNCHRONOUS or SYNCHRONOUS"
                    },
                    "properties": {
                        "type": "object",
                        "description": "Replacement string key/value pairs handed to the game when the activity launches"
                    },
                    "reference_name": {
                        "type": "string",
                        "description": "Updated reference name"
                    },
                    "supports_party_code": {
                        "type": "boolean",
                        "description": "Whether players can join the activity with a party code"
                    }
                },
                "required": [
                    "activity_id"
                ]
            },
            "annotations": {
                "title": "Update Game Center Activity",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_game_center_activity),
    );
    r.register(
        tool(r##"{
            "name": "delete_game_center_activity",
            "description": "Delete a Game Center activity",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "activity_id": {
                        "type": "string",
                        "description": "The activity ID"
                    }
                },
                "required": [
                    "activity_id"
                ]
            },
            "annotations": {
                "title": "Delete Game Center Activity",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_game_center_activity),
    );
    r.register(
        tool(r##"{
            "name": "list_game_center_activity_versions",
            "description": "List the versions of a Game Center activity, including each version's review state and fallback URL",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "activity_id": {
                        "type": "string",
                        "description": "The activity ID"
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
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"referenceName\": [\"Weekly Ladder\"]} becomes filter[referenceName]=Weekly%20Ladder."
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
                        "description": "Maximum number of versions to return (default 50, max 200)"
                    }
                },
                "required": [
                    "activity_id"
                ]
            },
            "annotations": {
                "title": "List Game Center Activity Versions",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_game_center_activity_versions),
    );
    r.register(
        tool(r##"{
            "name": "get_game_center_activity_version",
            "description": "Get a specific Game Center activity version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "version_id": {
                        "type": "string",
                        "description": "The activity version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Get Game Center Activity Version",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_game_center_activity_version),
    );
    r.register(
        tool(r##"{
            "name": "create_game_center_activity_version",
            "description": "Open a new editable version of a Game Center activity. Attach localizations to the new version, then submit it for review with add_review_submission_item.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "activity_id": {
                        "type": "string",
                        "description": "The activity ID to open a version for"
                    },
                    "fallback_url": {
                        "type": "string",
                        "description": "URL opened when the player's device cannot launch the activity"
                    }
                },
                "required": [
                    "activity_id"
                ]
            },
            "annotations": {
                "title": "Create Game Center Activity Version",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_game_center_activity_version),
    );
    r.register(
        tool(r##"{
            "name": "update_game_center_activity_version",
            "description": "Update the fallback URL of a Game Center activity version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "fallback_url": {
                        "type": "string",
                        "description": "URL opened when the player's device cannot launch the activity"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The activity version ID"
                    }
                },
                "required": [
                    "version_id",
                    "fallback_url"
                ]
            },
            "annotations": {
                "title": "Update Game Center Activity Version",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_game_center_activity_version),
    );
    r.register(
        tool(r##"{
            "name": "list_game_center_activity_localizations",
            "description": "List the localizations of a Game Center activity version",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"referenceName\": [\"Weekly Ladder\"]} becomes filter[referenceName]=Weekly%20Ladder."
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
                        "description": "Maximum number of localizations to return (default 50, max 200)"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The activity version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "List Game Center Activity Localizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_game_center_activity_localizations),
    );
    r.register(
        tool(r##"{
            "name": "create_game_center_activity_localization",
            "description": "Add a localized name and description for a Game Center activity version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "description": {
                        "type": "string",
                        "description": "The localized activity description"
                    },
                    "locale": {
                        "type": "string",
                        "description": "The locale code (e.g. en-US)"
                    },
                    "name": {
                        "type": "string",
                        "description": "The localized activity name shown to players"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The activity version ID"
                    }
                },
                "required": [
                    "version_id",
                    "locale",
                    "name"
                ]
            },
            "annotations": {
                "title": "Create Game Center Activity Localization",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_game_center_activity_localization),
    );
    r.register(
        tool(r##"{
            "name": "update_game_center_activity_localization",
            "description": "Update the localized name or description of a Game Center activity",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "description": {
                        "type": "string",
                        "description": "The updated localized description"
                    },
                    "localization_id": {
                        "type": "string",
                        "description": "The activity localization ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "The updated localized name"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Update Game Center Activity Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_game_center_activity_localization),
    );
    r.register(
        tool(r##"{
            "name": "delete_game_center_activity_localization",
            "description": "Delete a Game Center activity localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The activity localization ID"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Delete Game Center Activity Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_game_center_activity_localization),
    );
    r.register(
        tool(r##"{
            "name": "upload_game_center_activity_image",
            "description": "Reserve, upload, and commit a Game Center activity image. Provide exactly one of localization_id (that locale's image) or version_id (the version's default image), and exactly one of file_path or file_data_base64.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "file_data_base64": {
                        "type": "string",
                        "description": "Standard base64 (RFC 4648) encoded bytes of the file. Use this when the client must transport the file across a remote transport such as HTTP."
                    },
                    "file_name": {
                        "type": "string",
                        "description": "The original file name (e.g. activity.png)"
                    },
                    "file_path": {
                        "type": "string",
                        "description": "Absolute path to the file on the server's local filesystem. Use this when the MCP server and the file are on the same host (most stdio scenarios)."
                    },
                    "localization_id": {
                        "type": "string",
                        "description": "The activity localization ID to attach the image to"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The activity version ID to attach the image to as its default image"
                    }
                },
                "required": [
                    "file_name"
                ]
            },
            "annotations": {
                "title": "Upload Game Center Activity Image",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_upload_game_center_activity_image),
    );
}

/// Ported from `Registry.handleListGameCenterActivities`.
async fn handle_list_game_center_activities(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GameCenterDetailID string \"json:\\\"game_center_detail_id\\\"\"; tools.gcListParams }", "") {
        game_center_detail_id: String => "game_center_detail_id";
        gc_list_params: GcListParams => "gcListParams", flatten;
    }
    }
    todo!("port handleListGameCenterActivities from internal/asc/tools/gamecenter_activities.go")
}

/// Ported from `Registry.handleGetGameCenterActivity`.
async fn handle_get_game_center_activity(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ActivityID string \"json:\\\"activity_id\\\"\" }", "") {
        activity_id: String => "activity_id";
    }
    }
    todo!("port handleGetGameCenterActivity from internal/asc/tools/gamecenter_activities.go")
}

/// Ported from `Registry.handleCreateGameCenterActivity`.
async fn handle_create_game_center_activity(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GameCenterDetailID string \"json:\\\"game_center_detail_id\\\"\"; GameCenterGroupID string \"json:\\\"game_center_group_id\\\"\"; ReferenceName string \"json:\\\"reference_name\\\"\"; VendorIdentifier string \"json:\\\"vendor_identifier\\\"\"; PlayStyle string \"json:\\\"play_style\\\"\"; MinimumPlayersCount *int \"json:\\\"minimum_players_count\\\"\"; MaximumPlayersCount *int \"json:\\\"maximum_players_count\\\"\"; SupportsPartyCode *bool \"json:\\\"supports_party_code\\\"\"; Properties map[string]string \"json:\\\"properties\\\"\"; FallbackURL string \"json:\\\"fallback_url\\\"\" }", "") {
        game_center_detail_id: String => "game_center_detail_id";
        game_center_group_id: String => "game_center_group_id";
        reference_name: String => "reference_name";
        vendor_identifier: String => "vendor_identifier";
        play_style: String => "play_style";
        minimum_players_count: Option<i64> => "minimum_players_count";
        maximum_players_count: Option<i64> => "maximum_players_count";
        supports_party_code: Option<bool> => "supports_party_code";
        properties: GoMap<String> => "properties";
        fallback_url: String => "fallback_url";
    }
    }
    todo!("port handleCreateGameCenterActivity from internal/asc/tools/gamecenter_activities.go")
}

/// Ported from `Registry.handleUpdateGameCenterActivity`.
async fn handle_update_game_center_activity(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ActivityID string \"json:\\\"activity_id\\\"\"; ReferenceName string \"json:\\\"reference_name\\\"\"; PlayStyle string \"json:\\\"play_style\\\"\"; MinimumPlayersCount *int \"json:\\\"minimum_players_count\\\"\"; MaximumPlayersCount *int \"json:\\\"maximum_players_count\\\"\"; SupportsPartyCode *bool \"json:\\\"supports_party_code\\\"\"; Properties map[string]string \"json:\\\"properties\\\"\"; Archived *bool \"json:\\\"archived\\\"\" }", "") {
        activity_id: String => "activity_id";
        reference_name: String => "reference_name";
        play_style: String => "play_style";
        minimum_players_count: Option<i64> => "minimum_players_count";
        maximum_players_count: Option<i64> => "maximum_players_count";
        supports_party_code: Option<bool> => "supports_party_code";
        properties: GoMap<String> => "properties";
        archived: Option<bool> => "archived";
    }
    }
    todo!("port handleUpdateGameCenterActivity from internal/asc/tools/gamecenter_activities.go")
}

/// Ported from `Registry.handleDeleteGameCenterActivity`.
async fn handle_delete_game_center_activity(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ActivityID string \"json:\\\"activity_id\\\"\" }", "") {
        activity_id: String => "activity_id";
    }
    }
    todo!("port handleDeleteGameCenterActivity from internal/asc/tools/gamecenter_activities.go")
}

/// Ported from `Registry.handleListGameCenterActivityVersions`.
async fn handle_list_game_center_activity_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ActivityID string \"json:\\\"activity_id\\\"\"; tools.gcListParams }", "") {
        activity_id: String => "activity_id";
        gc_list_params: GcListParams => "gcListParams", flatten;
    }
    }
    todo!("port handleListGameCenterActivityVersions from internal/asc/tools/gamecenter_activities.go")
}

/// Ported from `Registry.handleGetGameCenterActivityVersion`.
async fn handle_get_game_center_activity_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    todo!("port handleGetGameCenterActivityVersion from internal/asc/tools/gamecenter_activities.go")
}

/// Ported from `Registry.handleCreateGameCenterActivityVersion`.
async fn handle_create_game_center_activity_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ActivityID string \"json:\\\"activity_id\\\"\"; FallbackURL string \"json:\\\"fallback_url\\\"\" }", "") {
        activity_id: String => "activity_id";
        fallback_url: String => "fallback_url";
    }
    }
    todo!("port handleCreateGameCenterActivityVersion from internal/asc/tools/gamecenter_activities.go")
}

/// Ported from `Registry.handleUpdateGameCenterActivityVersion`.
async fn handle_update_game_center_activity_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; FallbackURL string \"json:\\\"fallback_url\\\"\" }", "") {
        version_id: String => "version_id";
        fallback_url: String => "fallback_url";
    }
    }
    todo!("port handleUpdateGameCenterActivityVersion from internal/asc/tools/gamecenter_activities.go")
}

/// Ported from `Registry.handleListGameCenterActivityLocalizations`.
async fn handle_list_game_center_activity_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; tools.gcListParams }", "") {
        version_id: String => "version_id";
        gc_list_params: GcListParams => "gcListParams", flatten;
    }
    }
    todo!("port handleListGameCenterActivityLocalizations from internal/asc/tools/gamecenter_activities.go")
}

/// Ported from `Registry.handleCreateGameCenterActivityLocalization`.
async fn handle_create_game_center_activity_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; Locale string \"json:\\\"locale\\\"\"; Name string \"json:\\\"name\\\"\"; Description string \"json:\\\"description\\\"\" }", "") {
        version_id: String => "version_id";
        locale: String => "locale";
        name: String => "name";
        description: String => "description";
    }
    }
    todo!("port handleCreateGameCenterActivityLocalization from internal/asc/tools/gamecenter_activities.go")
}

/// Ported from `Registry.handleUpdateGameCenterActivityLocalization`.
async fn handle_update_game_center_activity_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Name string \"json:\\\"name\\\"\"; Description string \"json:\\\"description\\\"\" }", "") {
        localization_id: String => "localization_id";
        name: String => "name";
        description: String => "description";
    }
    }
    todo!("port handleUpdateGameCenterActivityLocalization from internal/asc/tools/gamecenter_activities.go")
}

/// Ported from `Registry.handleDeleteGameCenterActivityLocalization`.
async fn handle_delete_game_center_activity_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    todo!("port handleDeleteGameCenterActivityLocalization from internal/asc/tools/gamecenter_activities.go")
}

/// Ported from `Registry.handleUploadGameCenterActivityImage`.
async fn handle_upload_game_center_activity_image(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; VersionID string \"json:\\\"version_id\\\"\"; FileName string \"json:\\\"file_name\\\"\"; FilePath string \"json:\\\"file_path\\\"\"; FileDataBase64 string \"json:\\\"file_data_base64\\\"\" }", "") {
        localization_id: String => "localization_id";
        version_id: String => "version_id";
        file_name: String => "file_name";
        file_path: String => "file_path";
        file_data_base64: String => "file_data_base64";
    }
    }
    todo!("port handleUploadGameCenterActivityImage from internal/asc/tools/gamecenter_activities.go")
}

