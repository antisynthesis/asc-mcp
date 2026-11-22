//! Ported from internal/asc/tools/gamecenter_leaderboard_sets.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerGameCenterLeaderboardSetTools`.
pub(super) fn register_game_center_leaderboard_set_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_game_center_leaderboard_sets",
            "description": "List the Game Center leaderboard sets for an app via the v2 Game Center API",
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
                        "description": "Maximum number of leaderboard sets to return (default 50, max 200)"
                    }
                },
                "required": [
                    "game_center_detail_id"
                ]
            },
            "annotations": {
                "title": "List Game Center Leaderboard Sets",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_game_center_leaderboard_sets),
    );
    r.register(
        tool(r##"{
            "name": "get_game_center_leaderboard_set",
            "description": "Get details of a specific Game Center leaderboard set",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "leaderboard_set_id": {
                        "type": "string",
                        "description": "The leaderboard set ID"
                    }
                },
                "required": [
                    "leaderboard_set_id"
                ]
            },
            "annotations": {
                "title": "Get Game Center Leaderboard Set",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_game_center_leaderboard_set),
    );
    r.register(
        tool(r##"{
            "name": "create_game_center_leaderboard_set",
            "description": "Create a Game Center leaderboard set via the v2 Game Center API. Provide exactly one of game_center_detail_id (app-owned) or game_center_group_id (shared across a group). An initial version is created with the set; add leaderboards with add_game_center_leaderboard_set_members.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "game_center_detail_id": {
                        "type": "string",
                        "description": "The Game Center detail ID to own the set"
                    },
                    "game_center_group_id": {
                        "type": "string",
                        "description": "The Game Center group ID to own the set instead of a single app"
                    },
                    "reference_name": {
                        "type": "string",
                        "description": "Internal reference name"
                    },
                    "vendor_identifier": {
                        "type": "string",
                        "description": "Unique identifier for the leaderboard set"
                    }
                },
                "required": [
                    "reference_name",
                    "vendor_identifier"
                ]
            },
            "annotations": {
                "title": "Create Game Center Leaderboard Set",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_game_center_leaderboard_set),
    );
    r.register(
        tool(r##"{
            "name": "update_game_center_leaderboard_set",
            "description": "Update a Game Center leaderboard set's reference name. The vendor identifier is immutable once the set exists.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "leaderboard_set_id": {
                        "type": "string",
                        "description": "The leaderboard set ID"
                    },
                    "reference_name": {
                        "type": "string",
                        "description": "Updated reference name"
                    }
                },
                "required": [
                    "leaderboard_set_id",
                    "reference_name"
                ]
            },
            "annotations": {
                "title": "Update Game Center Leaderboard Set",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_game_center_leaderboard_set),
    );
    r.register(
        tool(r##"{
            "name": "delete_game_center_leaderboard_set",
            "description": "Delete a Game Center leaderboard set",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "leaderboard_set_id": {
                        "type": "string",
                        "description": "The leaderboard set ID"
                    }
                },
                "required": [
                    "leaderboard_set_id"
                ]
            },
            "annotations": {
                "title": "Delete Game Center Leaderboard Set",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_game_center_leaderboard_set),
    );
    r.register(
        tool(r##"{
            "name": "list_game_center_leaderboard_set_members",
            "description": "List the leaderboards that belong to a Game Center leaderboard set",
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
                    "leaderboard_set_id": {
                        "type": "string",
                        "description": "The leaderboard set ID"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of member leaderboards to return (default 50, max 200)"
                    }
                },
                "required": [
                    "leaderboard_set_id"
                ]
            },
            "annotations": {
                "title": "List Game Center Leaderboard Set Members",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_game_center_leaderboard_set_members),
    );
    r.register(
        tool(r##"{
            "name": "add_game_center_leaderboard_set_members",
            "description": "Add leaderboards to a Game Center leaderboard set",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "leaderboard_ids": {
                        "type": "array",
                        "description": "Leaderboard IDs to add to the set",
                        "items": {
                            "type": "string"
                        }
                    },
                    "leaderboard_set_id": {
                        "type": "string",
                        "description": "The leaderboard set ID"
                    }
                },
                "required": [
                    "leaderboard_set_id",
                    "leaderboard_ids"
                ]
            },
            "annotations": {
                "title": "Add Game Center Leaderboard Set Members",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_add_game_center_leaderboard_set_members),
    );
    r.register(
        tool(r##"{
            "name": "remove_game_center_leaderboard_set_members",
            "description": "Remove leaderboards from a Game Center leaderboard set. The leaderboards themselves are not deleted.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "leaderboard_ids": {
                        "type": "array",
                        "description": "Leaderboard IDs to remove from the set",
                        "items": {
                            "type": "string"
                        }
                    },
                    "leaderboard_set_id": {
                        "type": "string",
                        "description": "The leaderboard set ID"
                    }
                },
                "required": [
                    "leaderboard_set_id",
                    "leaderboard_ids"
                ]
            },
            "annotations": {
                "title": "Remove Game Center Leaderboard Set Members",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_remove_game_center_leaderboard_set_members),
    );
    r.register(
        tool(r##"{
            "name": "list_game_center_leaderboard_set_versions",
            "description": "List the versions of a Game Center leaderboard set, including each version's review state",
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
                    "leaderboard_set_id": {
                        "type": "string",
                        "description": "The leaderboard set ID"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of versions to return (default 50, max 200)"
                    }
                },
                "required": [
                    "leaderboard_set_id"
                ]
            },
            "annotations": {
                "title": "List Game Center Leaderboard Set Versions",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_game_center_leaderboard_set_versions),
    );
    r.register(
        tool(r##"{
            "name": "get_game_center_leaderboard_set_version",
            "description": "Get a specific Game Center leaderboard set version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "version_id": {
                        "type": "string",
                        "description": "The leaderboard set version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Get Game Center Leaderboard Set Version",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_game_center_leaderboard_set_version),
    );
    r.register(
        tool(r##"{
            "name": "create_game_center_leaderboard_set_version",
            "description": "Open a new editable version of a Game Center leaderboard set. Attach localizations to the new version, then submit it for review with add_review_submission_item.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "leaderboard_set_id": {
                        "type": "string",
                        "description": "The leaderboard set ID to open a version for"
                    }
                },
                "required": [
                    "leaderboard_set_id"
                ]
            },
            "annotations": {
                "title": "Create Game Center Leaderboard Set Version",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_game_center_leaderboard_set_version),
    );
    r.register(
        tool(r##"{
            "name": "list_game_center_leaderboard_set_localizations",
            "description": "List the localizations of a Game Center leaderboard set version",
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
                        "description": "The leaderboard set version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "List Game Center Leaderboard Set Localizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_game_center_leaderboard_set_localizations),
    );
    r.register(
        tool(r##"{
            "name": "create_game_center_leaderboard_set_localization",
            "description": "Add a localized name for a Game Center leaderboard set version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "locale": {
                        "type": "string",
                        "description": "The locale code (e.g. en-US)"
                    },
                    "name": {
                        "type": "string",
                        "description": "The localized leaderboard set name shown to players"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The leaderboard set version ID"
                    }
                },
                "required": [
                    "version_id",
                    "locale",
                    "name"
                ]
            },
            "annotations": {
                "title": "Create Game Center Leaderboard Set Localization",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_game_center_leaderboard_set_localization),
    );
    r.register(
        tool(r##"{
            "name": "update_game_center_leaderboard_set_localization",
            "description": "Update the localized name of a Game Center leaderboard set",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The leaderboard set localization ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "The updated localized name"
                    }
                },
                "required": [
                    "localization_id",
                    "name"
                ]
            },
            "annotations": {
                "title": "Update Game Center Leaderboard Set Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_game_center_leaderboard_set_localization),
    );
    r.register(
        tool(r##"{
            "name": "delete_game_center_leaderboard_set_localization",
            "description": "Delete a Game Center leaderboard set localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The leaderboard set localization ID"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Delete Game Center Leaderboard Set Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_game_center_leaderboard_set_localization),
    );
    r.register(
        tool(r##"{
            "name": "upload_game_center_leaderboard_set_image",
            "description": "Reserve, upload, and commit the image for a Game Center leaderboard set localization. Provide exactly one of file_path or file_data_base64.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "file_data_base64": {
                        "type": "string",
                        "description": "Standard base64 (RFC 4648) encoded bytes of the file. Use this when the client must transport the file across a remote transport such as HTTP."
                    },
                    "file_name": {
                        "type": "string",
                        "description": "The original file name (e.g. leaderboard-set.png)"
                    },
                    "file_path": {
                        "type": "string",
                        "description": "Absolute path to the file on the server's local filesystem. Use this when the MCP server and the file are on the same host (most stdio scenarios)."
                    },
                    "localization_id": {
                        "type": "string",
                        "description": "The leaderboard set localization ID the image belongs to"
                    }
                },
                "required": [
                    "localization_id",
                    "file_name"
                ]
            },
            "annotations": {
                "title": "Upload Game Center Leaderboard Set Image",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_upload_game_center_leaderboard_set_image),
    );
    r.register(
        tool(r##"{
            "name": "list_game_center_leaderboard_set_member_localizations",
            "description": "List the per-set localized names of a leaderboard within a leaderboard set. Both leaderboard_set_id and leaderboard_id are required.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response (supported: gameCenterLeaderboard, gameCenterLeaderboardSet).",
                        "items": {
                            "type": "string"
                        }
                    },
                    "leaderboard_id": {
                        "type": "string",
                        "description": "The leaderboard within that set"
                    },
                    "leaderboard_set_id": {
                        "type": "string",
                        "description": "The leaderboard set the member localizations belong to"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of member localizations to return (default 50, max 200)"
                    }
                },
                "required": [
                    "leaderboard_set_id",
                    "leaderboard_id"
                ]
            },
            "annotations": {
                "title": "List Game Center Leaderboard Set Member Localizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_game_center_leaderboard_set_member_localizations),
    );
    r.register(
        tool(r##"{
            "name": "create_game_center_leaderboard_set_member_localization",
            "description": "Name a leaderboard as it appears inside a leaderboard set for one locale. This overrides the leaderboard's own localized name within that set.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "leaderboard_id": {
                        "type": "string",
                        "description": "The member leaderboard ID"
                    },
                    "leaderboard_set_id": {
                        "type": "string",
                        "description": "The leaderboard set ID"
                    },
                    "locale": {
                        "type": "string",
                        "description": "The locale code (e.g. en-US)"
                    },
                    "name": {
                        "type": "string",
                        "description": "The localized name for this leaderboard within the set"
                    }
                },
                "required": [
                    "leaderboard_set_id",
                    "leaderboard_id",
                    "locale",
                    "name"
                ]
            },
            "annotations": {
                "title": "Create Game Center Leaderboard Set Member Localization",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_game_center_leaderboard_set_member_localization),
    );
    r.register(
        tool(r##"{
            "name": "update_game_center_leaderboard_set_member_localization",
            "description": "Update the per-set localized name of a leaderboard",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "member_localization_id": {
                        "type": "string",
                        "description": "The member localization ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "The updated localized name"
                    }
                },
                "required": [
                    "member_localization_id",
                    "name"
                ]
            },
            "annotations": {
                "title": "Update Game Center Leaderboard Set Member Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_game_center_leaderboard_set_member_localization),
    );
    r.register(
        tool(r##"{
            "name": "delete_game_center_leaderboard_set_member_localization",
            "description": "Delete a per-set localized leaderboard name",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "member_localization_id": {
                        "type": "string",
                        "description": "The member localization ID"
                    }
                },
                "required": [
                    "member_localization_id"
                ]
            },
            "annotations": {
                "title": "Delete Game Center Leaderboard Set Member Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_game_center_leaderboard_set_member_localization),
    );
}

/// Ported from `Registry.handleListGameCenterLeaderboardSets`.
async fn handle_list_game_center_leaderboard_sets(c: Arc<Client>, args: Args) -> HandlerResult {
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
    let resp = match paginated_fetch(&c, &params.gc_list_params.cursor, || c.list_game_center_leaderboard_sets(&params.game_center_detail_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list leaderboard sets: {e}"))),
    };

    Ok(list_result(&format_game_center_leaderboard_sets(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetGameCenterLeaderboardSet`.
async fn handle_get_game_center_leaderboard_set(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\" }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.leaderboard_set_id.is_empty() {
        return Ok(error_result("leaderboard_set_id is required"));
    }

    let resp = match c.get_game_center_leaderboard_set(&params.leaderboard_set_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get leaderboard set: {e}"))),
    };

    Ok(data_result(&format_game_center_leaderboard_set(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateGameCenterLeaderboardSet`.
async fn handle_create_game_center_leaderboard_set(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GameCenterDetailID string \"json:\\\"game_center_detail_id\\\"\"; GameCenterGroupID string \"json:\\\"game_center_group_id\\\"\"; ReferenceName string \"json:\\\"reference_name\\\"\"; VendorIdentifier string \"json:\\\"vendor_identifier\\\"\" }", "") {
        game_center_detail_id: String => "game_center_detail_id";
        game_center_group_id: String => "game_center_group_id";
        reference_name: String => "reference_name";
        vendor_identifier: String => "vendor_identifier";
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

    let mut rels = GameCenterLeaderboardSetCreateRelationships {
        // The v2 API requires an initial leaderboard set version, created
        // inline with a client-chosen temporary ID.
        versions: RelationshipDataList {
            data: GoSlice::from(vec![ResourceIdentifier { type_: "gameCenterLeaderboardSetVersions".into(), id: "${new-version}".into() }]),
        },
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

    let req = GameCenterLeaderboardSetCreateRequest {
        data: GameCenterLeaderboardSetCreateData {
            type_: "gameCenterLeaderboardSets".into(),
            attributes: GameCenterLeaderboardSetCreateAttributes {
                reference_name: params.reference_name.clone(),
                vendor_identifier: params.vendor_identifier.clone(),
            },
            relationships: rels,
        },
        included: GoSlice::from(vec![GameCenterVersionInlineCreate { type_: "gameCenterLeaderboardSetVersions".into(), id: "${new-version}".into() }]),
    };

    let resp = match c.create_game_center_leaderboard_set(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create leaderboard set: {e}"))),
    };

    Ok(data_result(&format!("Created leaderboard set: {} (ID: {})", resp.data.attributes.reference_name, resp.data.id), &resp.data))
}

/// Ported from `Registry.handleUpdateGameCenterLeaderboardSet`.
async fn handle_update_game_center_leaderboard_set(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\"; ReferenceName string \"json:\\\"reference_name\\\"\" }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
        reference_name: String => "reference_name";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.leaderboard_set_id.is_empty() {
        return Ok(error_result("leaderboard_set_id is required"));
    }
    if params.reference_name.is_empty() {
        return Ok(error_result("reference_name is required"));
    }

    let req = GameCenterLeaderboardSetUpdateRequest {
        data: GameCenterLeaderboardSetUpdateData {
            type_: "gameCenterLeaderboardSets".into(),
            id: params.leaderboard_set_id.clone(),
            attributes: GameCenterLeaderboardSetUpdateAttributes { reference_name: params.reference_name.clone() },
        },
    };

    let resp = match c.update_game_center_leaderboard_set(&params.leaderboard_set_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update leaderboard set: {e}"))),
    };

    Ok(data_result(&format!("Updated leaderboard set: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleDeleteGameCenterLeaderboardSet`.
async fn handle_delete_game_center_leaderboard_set(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\" }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.leaderboard_set_id.is_empty() {
        return Ok(error_result("leaderboard_set_id is required"));
    }

    if let Err(e) = c.delete_game_center_leaderboard_set(&params.leaderboard_set_id).await {
        return Ok(error_result(format!("Failed to delete leaderboard set: {e}")));
    }

    Ok(CallToolResult::success("Leaderboard set deleted successfully"))
}

/// Ported from `Registry.handleListGameCenterLeaderboardSetMembers`.
async fn handle_list_game_center_leaderboard_set_members(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\"; tools.gcListParams }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
        gc_list_params: GcListParams => "gcListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.leaderboard_set_id.is_empty() {
        return Ok(error_result("leaderboard_set_id is required"));
    }

    let opts = params.gc_list_params.opts();
    let resp = match paginated_fetch(&c, &params.gc_list_params.cursor, || c.list_game_center_leaderboard_set_members(&params.leaderboard_set_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list leaderboard set members: {e}"))),
    };

    Ok(list_result(&super::gamecenter::format_game_center_leaderboards(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleAddGameCenterLeaderboardSetMembers`.
async fn handle_add_game_center_leaderboard_set_members(c: Arc<Client>, args: Args) -> HandlerResult {
    let (set_id, ids) = match parse_leaderboard_set_membership(&args)? {
        Ok(v) => v,
        Err(err_result) => return Ok(err_result),
    };

    if let Err(e) = c.add_game_center_leaderboard_set_members(&set_id, &ids).await {
        return Ok(error_result(format!("Failed to add leaderboard set members: {e}")));
    }

    Ok(CallToolResult::success(format!("Added {} leaderboard(s) to set {}", ids.data.len(), set_id)))
}

/// Ported from `Registry.handleRemoveGameCenterLeaderboardSetMembers`.
async fn handle_remove_game_center_leaderboard_set_members(c: Arc<Client>, args: Args) -> HandlerResult {
    let (set_id, ids) = match parse_leaderboard_set_membership(&args)? {
        Ok(v) => v,
        Err(err_result) => return Ok(err_result),
    };

    if let Err(e) = c.remove_game_center_leaderboard_set_members(&set_id, &ids).await {
        return Ok(error_result(format!("Failed to remove leaderboard set members: {e}")));
    }

    Ok(CallToolResult::success(format!("Removed {} leaderboard(s) from set {}", ids.data.len(), set_id)))
}

/// Ported from `Registry.handleListGameCenterLeaderboardSetVersions`.
async fn handle_list_game_center_leaderboard_set_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\"; tools.gcListParams }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
        gc_list_params: GcListParams => "gcListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.leaderboard_set_id.is_empty() {
        return Ok(error_result("leaderboard_set_id is required"));
    }

    let opts = params.gc_list_params.opts();
    let resp = match paginated_fetch(&c, &params.gc_list_params.cursor, || c.list_game_center_leaderboard_set_versions(&params.leaderboard_set_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list leaderboard set versions: {e}"))),
    };

    Ok(list_result(&format_game_center_leaderboard_set_versions(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetGameCenterLeaderboardSetVersion`.
async fn handle_get_game_center_leaderboard_set_version(c: Arc<Client>, args: Args) -> HandlerResult {
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

    let resp = match c.get_game_center_leaderboard_set_version(&params.version_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get leaderboard set version: {e}"))),
    };

    Ok(data_result(&format_game_center_leaderboard_set_version(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateGameCenterLeaderboardSetVersion`.
async fn handle_create_game_center_leaderboard_set_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\" }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.leaderboard_set_id.is_empty() {
        return Ok(error_result("leaderboard_set_id is required"));
    }

    let req = GameCenterLeaderboardSetVersionCreateRequest {
        data: GameCenterLeaderboardSetVersionCreateData {
            type_: "gameCenterLeaderboardSetVersions".into(),
            relationships: GameCenterLeaderboardSetVersionCreateRelationships {
                leaderboard_set: RelationshipData {
                    data: ResourceIdentifier { type_: "gameCenterLeaderboardSets".into(), id: params.leaderboard_set_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_game_center_leaderboard_set_version(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create leaderboard set version: {e}"))),
    };

    Ok(data_result(&format!("Created leaderboard set version: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleListGameCenterLeaderboardSetLocalizations`.
async fn handle_list_game_center_leaderboard_set_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
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
    let resp = match paginated_fetch(&c, &params.gc_list_params.cursor, || c.list_game_center_leaderboard_set_localizations(&params.version_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list leaderboard set localizations: {e}"))),
    };

    Ok(list_result(&format_game_center_leaderboard_set_localizations(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleCreateGameCenterLeaderboardSetLocalization`.
async fn handle_create_game_center_leaderboard_set_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; Locale string \"json:\\\"locale\\\"\"; Name string \"json:\\\"name\\\"\" }", "") {
        version_id: String => "version_id";
        locale: String => "locale";
        name: String => "name";
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

    let req = GameCenterLeaderboardSetLocalizationCreateRequest {
        data: GameCenterLeaderboardSetLocalizationCreateData {
            type_: "gameCenterLeaderboardSetLocalizations".into(),
            attributes: GameCenterLeaderboardSetLocalizationCreateAttributes {
                locale: params.locale.clone(),
                name: params.name.clone(),
            },
            relationships: GameCenterLeaderboardSetLocalizationCreateRelationships {
                version: RelationshipData {
                    data: ResourceIdentifier { type_: "gameCenterLeaderboardSetVersions".into(), id: params.version_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_game_center_leaderboard_set_localization(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create leaderboard set localization: {e}"))),
    };

    Ok(data_result(&format!("Created leaderboard set localization {} for {}", resp.data.id, params.locale), &resp.data))
}

/// Ported from `Registry.handleUpdateGameCenterLeaderboardSetLocalization`.
async fn handle_update_game_center_leaderboard_set_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Name string \"json:\\\"name\\\"\" }", "") {
        localization_id: String => "localization_id";
        name: String => "name";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }
    if params.name.is_empty() {
        return Ok(error_result("name is required"));
    }

    let req = GameCenterLeaderboardSetLocalizationUpdateRequest {
        data: GameCenterLeaderboardSetLocalizationUpdateData {
            type_: "gameCenterLeaderboardSetLocalizations".into(),
            id: params.localization_id.clone(),
            attributes: GameCenterLeaderboardSetLocalizationUpdateAttributes { name: params.name.clone() },
        },
    };

    let resp = match c.update_game_center_leaderboard_set_localization(&params.localization_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update leaderboard set localization: {e}"))),
    };

    Ok(data_result(&format!("Updated leaderboard set localization: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleDeleteGameCenterLeaderboardSetLocalization`.
async fn handle_delete_game_center_leaderboard_set_localization(c: Arc<Client>, args: Args) -> HandlerResult {
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

    if let Err(e) = c.delete_game_center_leaderboard_set_localization(&params.localization_id).await {
        return Ok(error_result(format!("Failed to delete leaderboard set localization: {e}")));
    }

    Ok(CallToolResult::success("Leaderboard set localization deleted successfully"))
}

/// Ported from `Registry.handleUploadGameCenterLeaderboardSetImage`.
async fn handle_upload_game_center_leaderboard_set_image(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; FileName string \"json:\\\"file_name\\\"\"; FilePath string \"json:\\\"file_path\\\"\"; FileDataBase64 string \"json:\\\"file_data_base64\\\"\" }", "") {
        localization_id: String => "localization_id";
        file_name: String => "file_name";
        file_path: String => "file_path";
        file_data_base64: String => "file_data_base64";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }
    if params.file_name.is_empty() {
        return Ok(error_result("file_name is required"));
    }

    let body = match super::uploads::load_upload_body(&params.file_path, &params.file_data_base64) {
        Ok(b) => b,
        Err(e) => return Ok(error_result(e)),
    };

    let resp = match c.upload_game_center_leaderboard_set_image(&params.localization_id, &params.file_name, &body).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to upload leaderboard set image: {e}"))),
    };

    let text = format!(
        "Uploaded leaderboard set image {} ({}, {} bytes) to localization {}.",
        resp.data.id,
        params.file_name,
        body.len(),
        params.localization_id
    );
    Ok(data_result(&text, &resp.data))
}

/// Ported from `Registry.handleListGameCenterLeaderboardSetMemberLocalizations`.
async fn handle_list_game_center_leaderboard_set_member_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\"; LeaderboardID string \"json:\\\"leaderboard_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
        leaderboard_id: String => "leaderboard_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        include: GoSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    // The collection endpoint is addressable only through both of its
    // relationship filters; the API requires each one.
    if params.leaderboard_set_id.is_empty() {
        return Ok(error_result("leaderboard_set_id is required"));
    }
    if params.leaderboard_id.is_empty() {
        return Ok(error_result("leaderboard_id is required"));
    }
    let mut filter: GoMap<GoSlice<String>> = GoMap::from_map(Default::default());
    filter.insert("gameCenterLeaderboardSet", GoSlice::from(vec![params.leaderboard_set_id.clone()]));
    filter.insert("gameCenterLeaderboard", GoSlice::from(vec![params.leaderboard_id.clone()]));

    let opts = list_opts(super::gamecenter_content::gc_limit(params.limit), &filter, &GoSlice::nil(), &GoMap::nil(), &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_game_center_leaderboard_set_member_localizations(opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list member localizations: {e}"))),
    };

    Ok(list_result(&format_game_center_leaderboard_set_member_localizations(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleCreateGameCenterLeaderboardSetMemberLocalization`.
async fn handle_create_game_center_leaderboard_set_member_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\"; LeaderboardID string \"json:\\\"leaderboard_id\\\"\"; Locale string \"json:\\\"locale\\\"\"; Name string \"json:\\\"name\\\"\" }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
        leaderboard_id: String => "leaderboard_id";
        locale: String => "locale";
        name: String => "name";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.leaderboard_set_id.is_empty() {
        return Ok(error_result("leaderboard_set_id is required"));
    }
    if params.leaderboard_id.is_empty() {
        return Ok(error_result("leaderboard_id is required"));
    }
    if params.locale.is_empty() {
        return Ok(error_result("locale is required"));
    }
    if params.name.is_empty() {
        return Ok(error_result("name is required"));
    }

    let req = GameCenterLeaderboardSetMemberLocalizationCreateRequest {
        data: GameCenterLeaderboardSetMemberLocalizationCreateData {
            type_: "gameCenterLeaderboardSetMemberLocalizations".into(),
            attributes: GameCenterLeaderboardSetMemberLocalizationCreateAttributes {
                locale: params.locale.clone(),
                name: params.name.clone(),
            },
            relationships: GameCenterLeaderboardSetMemberLocalizationCreateRelationships {
                game_center_leaderboard_set: RelationshipData {
                    data: ResourceIdentifier { type_: "gameCenterLeaderboardSets".into(), id: params.leaderboard_set_id.clone() },
                },
                game_center_leaderboard: RelationshipData {
                    data: ResourceIdentifier { type_: "gameCenterLeaderboards".into(), id: params.leaderboard_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_game_center_leaderboard_set_member_localization(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create member localization: {e}"))),
    };

    Ok(data_result(&format!("Created member localization {} for {}", resp.data.id, params.locale), &resp.data))
}

/// Ported from `Registry.handleUpdateGameCenterLeaderboardSetMemberLocalization`.
async fn handle_update_game_center_leaderboard_set_member_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { MemberLocalizationID string \"json:\\\"member_localization_id\\\"\"; Name string \"json:\\\"name\\\"\" }", "") {
        member_localization_id: String => "member_localization_id";
        name: String => "name";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.member_localization_id.is_empty() {
        return Ok(error_result("member_localization_id is required"));
    }
    if params.name.is_empty() {
        return Ok(error_result("name is required"));
    }

    let req = GameCenterLeaderboardSetMemberLocalizationUpdateRequest {
        data: GameCenterLeaderboardSetMemberLocalizationUpdateData {
            type_: "gameCenterLeaderboardSetMemberLocalizations".into(),
            id: params.member_localization_id.clone(),
            attributes: GameCenterLeaderboardSetMemberLocalizationUpdateAttributes { name: params.name.clone() },
        },
    };

    let resp = match c.update_game_center_leaderboard_set_member_localization(&params.member_localization_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update member localization: {e}"))),
    };

    Ok(data_result(&format!("Updated member localization: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleDeleteGameCenterLeaderboardSetMemberLocalization`.
async fn handle_delete_game_center_leaderboard_set_member_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { MemberLocalizationID string \"json:\\\"member_localization_id\\\"\" }", "") {
        member_localization_id: String => "member_localization_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.member_localization_id.is_empty() {
        return Ok(error_result("member_localization_id is required"));
    }

    if let Err(e) = c.delete_game_center_leaderboard_set_member_localization(&params.member_localization_id).await {
        return Ok(error_result(format!("Failed to delete member localization: {e}")));
    }

    Ok(CallToolResult::success("Member localization deleted successfully"))
}

/// Ported from `parseLeaderboardSetMembership`: decodes the arguments
/// shared by the add and remove member tools into a linkages document.
/// The inner `Err` carries the tool result when validation failed.
fn parse_leaderboard_set_membership(args: &Args) -> Result<Result<(String, RelationshipDataList), CallToolResult>, String> {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\"; LeaderboardIDs []string \"json:\\\"leaderboard_ids\\\"\" }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
        leaderboard_ids: GoSlice<String> => "leaderboard_ids";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.leaderboard_set_id.is_empty() {
        return Ok(Err(error_result("leaderboard_set_id is required")));
    }
    if params.leaderboard_ids.is_empty() {
        return Ok(Err(error_result("leaderboard_ids is required")));
    }

    let mut linkages = RelationshipDataList { data: GoSlice::from(Vec::with_capacity(params.leaderboard_ids.len())) };
    for id in params.leaderboard_ids.iter() {
        if id.is_empty() {
            continue;
        }
        linkages.data.push(ResourceIdentifier { type_: "gameCenterLeaderboards".into(), id: id.clone() });
    }
    if linkages.data.is_empty() {
        return Ok(Err(error_result("leaderboard_ids is required")));
    }

    Ok(Ok((params.leaderboard_set_id, linkages)))
}

/// Ported from `formatGameCenterLeaderboardSets`.
fn format_game_center_leaderboard_sets(sets: &[GameCenterLeaderboardSet]) -> String {
    if sets.is_empty() {
        return "No Game Center leaderboard sets found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} leaderboard sets:\n\n", sets.len());

    for set in sets {
        sb.push_str(&format_game_center_leaderboard_set(set));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatGameCenterLeaderboardSet`.
fn format_game_center_leaderboard_set(set: &GameCenterLeaderboardSet) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", set.id);
    let _ = write!(sb, "Reference Name: {}\n", set.attributes.reference_name);
    let _ = write!(sb, "Vendor ID: {}\n", set.attributes.vendor_identifier);
    sb
}

/// Ported from `formatGameCenterLeaderboardSetVersions`.
fn format_game_center_leaderboard_set_versions(versions: &[GameCenterLeaderboardSetVersion]) -> String {
    if versions.is_empty() {
        return "No Game Center leaderboard set versions found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} leaderboard set versions:\n\n", versions.len());

    for version in versions {
        sb.push_str(&format_game_center_leaderboard_set_version(version));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatGameCenterLeaderboardSetVersion`.
fn format_game_center_leaderboard_set_version(version: &GameCenterLeaderboardSetVersion) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", version.id);
    let _ = write!(sb, "Version: {}\n", version.attributes.version);
    let _ = write!(sb, "State: {}\n", version.attributes.state);
    sb
}

/// Ported from `formatGameCenterLeaderboardSetLocalizations`.
fn format_game_center_leaderboard_set_localizations(localizations: &[GameCenterLeaderboardSetLocalization]) -> String {
    if localizations.is_empty() {
        return "No Game Center leaderboard set localizations found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} leaderboard set localizations:\n\n", localizations.len());

    for localization in localizations {
        let _ = write!(sb, "ID: {}\n", localization.id);
        let _ = write!(sb, "Locale: {}\n", localization.attributes.locale);
        let _ = write!(sb, "Name: {}\n", localization.attributes.name);
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatGameCenterLeaderboardSetMemberLocalizations`.
fn format_game_center_leaderboard_set_member_localizations(localizations: &[GameCenterLeaderboardSetMemberLocalization]) -> String {
    if localizations.is_empty() {
        return "No Game Center leaderboard set member localizations found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} member localizations:\n\n", localizations.len());

    for localization in localizations {
        let _ = write!(sb, "ID: {}\n", localization.id);
        let _ = write!(sb, "Locale: {}\n", localization.attributes.locale);
        let _ = write!(sb, "Name: {}\n", localization.attributes.name);
        sb.push_str("\n---\n");
    }

    sb
}
