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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.game_center_detail_id.is_empty() {
        return Ok(error_result("game_center_detail_id is required"));
    }

    let opts = params.gc_list_params.opts();
    let resp = match paginated_fetch(&c, &params.gc_list_params.cursor, || c.list_game_center_activities(&params.game_center_detail_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list activities: {e}"))),
    };

    Ok(list_result(&format_game_center_activities(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetGameCenterActivity`.
async fn handle_get_game_center_activity(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ActivityID string \"json:\\\"activity_id\\\"\" }", "") {
        activity_id: String => "activity_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.activity_id.is_empty() {
        return Ok(error_result("activity_id is required"));
    }

    let resp = match c.get_game_center_activity(&params.activity_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get activity: {e}"))),
    };

    Ok(data_result(&format_game_center_activity(&resp.data), &resp.data))
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.reference_name.is_empty() {
        return Ok(error_result("reference_name is required"));
    }
    if params.vendor_identifier.is_empty() {
        return Ok(error_result("vendor_identifier is required"));
    }
    if params.game_center_detail_id.is_empty() == params.game_center_group_id.is_empty() {
        return Ok(error_result("exactly one of game_center_detail_id or game_center_group_id is required"));
    }

    // The initial activity version is declared inline with a
    // client-chosen temporary ID; it carries the fallback URL.
    let mut rels = GameCenterActivityCreateRelationships {
        versions: Some(RelationshipDataList {
            data: GoSlice::from(vec![ResourceIdentifier { type_: "gameCenterActivityVersions".into(), id: "${new-version}".into() }]),
        }),
        ..Default::default()
    };
    if !params.game_center_detail_id.is_empty() {
        rels.game_center_detail = Some(RelationshipData {
            data: ResourceIdentifier { type_: "gameCenterDetails".into(), id: params.game_center_detail_id.clone() },
        });
    } else {
        rels.game_center_group = Some(RelationshipData {
            data: ResourceIdentifier { type_: "gameCenterGroups".into(), id: params.game_center_group_id.clone() },
        });
    }

    let mut inline = GameCenterActivityVersionInlineCreate {
        type_: "gameCenterActivityVersions".into(),
        id: "${new-version}".into(),
        ..Default::default()
    };
    if !params.fallback_url.is_empty() {
        inline.attributes = Some(GameCenterActivityVersionInlineCreateAttributes { fallback_url: params.fallback_url.clone() });
    }

    let req = GameCenterActivityCreateRequest {
        data: GameCenterActivityCreateData {
            type_: "gameCenterActivities".into(),
            attributes: GameCenterActivityCreateAttributes {
                reference_name: params.reference_name.clone(),
                vendor_identifier: params.vendor_identifier.clone(),
                play_style: params.play_style.clone(),
                minimum_players_count: params.minimum_players_count,
                maximum_players_count: params.maximum_players_count,
                supports_party_code: params.supports_party_code,
                properties: params.properties.clone(),
            },
            relationships: rels,
        },
        included: GoSlice::from(vec![inline]),
    };

    let resp = match c.create_game_center_activity(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create activity: {e}"))),
    };

    Ok(data_result(&format!("Created activity: {} (ID: {})", resp.data.attributes.reference_name, resp.data.id), &resp.data))
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.activity_id.is_empty() {
        return Ok(error_result("activity_id is required"));
    }

    let req = GameCenterActivityUpdateRequest {
        data: GameCenterActivityUpdateData {
            type_: "gameCenterActivities".into(),
            id: params.activity_id.clone(),
            attributes: GameCenterActivityUpdateAttributes {
                reference_name: params.reference_name.clone(),
                play_style: params.play_style.clone(),
                minimum_players_count: params.minimum_players_count,
                maximum_players_count: params.maximum_players_count,
                supports_party_code: params.supports_party_code,
                properties: params.properties.clone(),
                archived: params.archived,
            },
        },
    };

    let resp = match c.update_game_center_activity(&params.activity_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update activity: {e}"))),
    };

    Ok(data_result(&format!("Updated activity: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleDeleteGameCenterActivity`.
async fn handle_delete_game_center_activity(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ActivityID string \"json:\\\"activity_id\\\"\" }", "") {
        activity_id: String => "activity_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.activity_id.is_empty() {
        return Ok(error_result("activity_id is required"));
    }

    if let Err(e) = c.delete_game_center_activity(&params.activity_id).await {
        return Ok(error_result(format!("Failed to delete activity: {e}")));
    }

    Ok(CallToolResult::success("Activity deleted successfully"))
}

/// Ported from `Registry.handleListGameCenterActivityVersions`.
async fn handle_list_game_center_activity_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ActivityID string \"json:\\\"activity_id\\\"\"; tools.gcListParams }", "") {
        activity_id: String => "activity_id";
        gc_list_params: GcListParams => "gcListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.activity_id.is_empty() {
        return Ok(error_result("activity_id is required"));
    }

    let opts = params.gc_list_params.opts();
    let resp = match paginated_fetch(&c, &params.gc_list_params.cursor, || c.list_game_center_activity_versions(&params.activity_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list activity versions: {e}"))),
    };

    Ok(list_result(&format_game_center_activity_versions(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetGameCenterActivityVersion`.
async fn handle_get_game_center_activity_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    let resp = match c.get_game_center_activity_version(&params.version_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get activity version: {e}"))),
    };

    Ok(data_result(&format_game_center_activity_version(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateGameCenterActivityVersion`.
async fn handle_create_game_center_activity_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ActivityID string \"json:\\\"activity_id\\\"\"; FallbackURL string \"json:\\\"fallback_url\\\"\" }", "") {
        activity_id: String => "activity_id";
        fallback_url: String => "fallback_url";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.activity_id.is_empty() {
        return Ok(error_result("activity_id is required"));
    }

    let mut req = GameCenterActivityVersionCreateRequest {
        data: GameCenterActivityVersionCreateData {
            type_: "gameCenterActivityVersions".into(),
            relationships: GameCenterActivityVersionCreateRelationships {
                activity: RelationshipData {
                    data: ResourceIdentifier { type_: "gameCenterActivities".into(), id: params.activity_id.clone() },
                },
            },
            ..Default::default()
        },
    };
    if !params.fallback_url.is_empty() {
        req.data.attributes = Some(GameCenterActivityVersionCreateAttributes { fallback_url: params.fallback_url.clone() });
    }

    let resp = match c.create_game_center_activity_version(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create activity version: {e}"))),
    };

    Ok(data_result(&format!("Created activity version: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleUpdateGameCenterActivityVersion`.
async fn handle_update_game_center_activity_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; FallbackURL string \"json:\\\"fallback_url\\\"\" }", "") {
        version_id: String => "version_id";
        fallback_url: String => "fallback_url";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }
    if params.fallback_url.is_empty() {
        return Ok(error_result("fallback_url is required"));
    }

    let req = GameCenterActivityVersionUpdateRequest {
        data: GameCenterActivityVersionUpdateData {
            type_: "gameCenterActivityVersions".into(),
            id: params.version_id.clone(),
            attributes: GameCenterActivityVersionUpdateAttributes { fallback_url: params.fallback_url.clone() },
        },
    };

    let resp = match c.update_game_center_activity_version(&params.version_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update activity version: {e}"))),
    };

    Ok(data_result(&format!("Updated activity version: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleListGameCenterActivityLocalizations`.
async fn handle_list_game_center_activity_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; tools.gcListParams }", "") {
        version_id: String => "version_id";
        gc_list_params: GcListParams => "gcListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    let opts = params.gc_list_params.opts();
    let resp = match paginated_fetch(&c, &params.gc_list_params.cursor, || c.list_game_center_activity_localizations(&params.version_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list activity localizations: {e}"))),
    };

    Ok(list_result(&format_game_center_activity_localizations(&resp.data), &resp.data, &resp.links))
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }
    if params.locale.is_empty() {
        return Ok(error_result("locale is required"));
    }
    if params.name.is_empty() {
        return Ok(error_result("name is required"));
    }

    let req = GameCenterActivityLocalizationCreateRequest {
        data: GameCenterActivityLocalizationCreateData {
            type_: "gameCenterActivityLocalizations".into(),
            attributes: GameCenterActivityLocalizationCreateAttributes {
                locale: params.locale.clone(),
                name: params.name.clone(),
                description: params.description.clone(),
            },
            relationships: GameCenterActivityLocalizationCreateRelationships {
                version: RelationshipData {
                    data: ResourceIdentifier { type_: "gameCenterActivityVersions".into(), id: params.version_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_game_center_activity_localization(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create activity localization: {e}"))),
    };

    Ok(data_result(&format!("Created activity localization {} for {}", resp.data.id, params.locale), &resp.data))
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }
    if params.name.is_empty() && params.description.is_empty() {
        return Ok(error_result("at least one of name or description is required"));
    }

    let req = GameCenterActivityLocalizationUpdateRequest {
        data: GameCenterActivityLocalizationUpdateData {
            type_: "gameCenterActivityLocalizations".into(),
            id: params.localization_id.clone(),
            attributes: GameCenterActivityLocalizationUpdateAttributes {
                name: params.name.clone(),
                description: params.description.clone(),
            },
        },
    };

    let resp = match c.update_game_center_activity_localization(&params.localization_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update activity localization: {e}"))),
    };

    Ok(data_result(&format!("Updated activity localization: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleDeleteGameCenterActivityLocalization`.
async fn handle_delete_game_center_activity_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }

    if let Err(e) = c.delete_game_center_activity_localization(&params.localization_id).await {
        return Ok(error_result(format!("Failed to delete activity localization: {e}")));
    }

    Ok(CallToolResult::success("Activity localization deleted successfully"))
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.file_name.is_empty() {
        return Ok(error_result("file_name is required"));
    }
    if params.localization_id.is_empty() == params.version_id.is_empty() {
        return Ok(error_result("exactly one of localization_id or version_id is required"));
    }

    let body = match super::uploads::load_upload_body(&params.file_path, &params.file_data_base64) {
        Ok(b) => b,
        Err(e) => return Ok(error_result(e)),
    };

    let resp = match c.upload_game_center_activity_image(&params.localization_id, &params.version_id, &params.file_name, &body).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to upload activity image: {e}"))),
    };

    let text = format!("Uploaded activity image {} ({}, {} bytes).", resp.data.id, params.file_name, body.len());
    Ok(data_result(&text, &resp.data))
}

/// Ported from `formatGameCenterActivities`.
fn format_game_center_activities(activities: &[GameCenterActivity]) -> String {
    if activities.is_empty() {
        return "No Game Center activities found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} activities:\n\n", activities.len());

    for activity in activities {
        sb.push_str(&format_game_center_activity(activity));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatGameCenterActivity`.
fn format_game_center_activity(activity: &GameCenterActivity) -> String {
    let a = &activity.attributes;
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", activity.id);
    let _ = write!(sb, "Reference Name: {}\n", a.reference_name);
    let _ = write!(sb, "Vendor ID: {}\n", a.vendor_identifier);
    if !a.play_style.is_empty() {
        let _ = write!(sb, "Play Style: {}\n", a.play_style);
    }
    if a.minimum_players_count > 0 || a.maximum_players_count > 0 {
        let _ = write!(sb, "Players: {} - {}\n", a.minimum_players_count, a.maximum_players_count);
    }
    let _ = write!(sb, "Supports Party Code: {}\n", a.supports_party_code);
    let _ = write!(sb, "Archived: {}\n", a.archived);
    sb
}

/// Ported from `formatGameCenterActivityVersions`.
fn format_game_center_activity_versions(versions: &[GameCenterActivityVersion]) -> String {
    if versions.is_empty() {
        return "No Game Center activity versions found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} activity versions:\n\n", versions.len());

    for version in versions {
        sb.push_str(&format_game_center_activity_version(version));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatGameCenterActivityVersion`.
fn format_game_center_activity_version(version: &GameCenterActivityVersion) -> String {
    let a = &version.attributes;
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", version.id);
    let _ = write!(sb, "Version: {}\n", a.version);
    let _ = write!(sb, "State: {}\n", a.state);
    if !a.fallback_url.is_empty() {
        let _ = write!(sb, "Fallback URL: {}\n", a.fallback_url);
    }
    sb
}

/// Ported from `formatGameCenterActivityLocalizations`.
fn format_game_center_activity_localizations(localizations: &[GameCenterActivityLocalization]) -> String {
    if localizations.is_empty() {
        return "No Game Center activity localizations found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} activity localizations:\n\n", localizations.len());

    for localization in localizations {
        let a = &localization.attributes;
        let _ = write!(sb, "ID: {}\n", localization.id);
        let _ = write!(sb, "Locale: {}\n", a.locale);
        let _ = write!(sb, "Name: {}\n", a.name);
        if !a.description.is_empty() {
            let _ = write!(sb, "Description: {}\n", a.description);
        }
        sb.push_str("\n---\n");
    }

    sb
}
