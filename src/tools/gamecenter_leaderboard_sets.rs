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
    todo!("port handleListGameCenterLeaderboardSets from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

/// Ported from `Registry.handleGetGameCenterLeaderboardSet`.
async fn handle_get_game_center_leaderboard_set(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\" }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
    }
    }
    todo!("port handleGetGameCenterLeaderboardSet from internal/asc/tools/gamecenter_leaderboard_sets.go")
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
    todo!("port handleCreateGameCenterLeaderboardSet from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

/// Ported from `Registry.handleUpdateGameCenterLeaderboardSet`.
async fn handle_update_game_center_leaderboard_set(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\"; ReferenceName string \"json:\\\"reference_name\\\"\" }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
        reference_name: String => "reference_name";
    }
    }
    todo!("port handleUpdateGameCenterLeaderboardSet from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

/// Ported from `Registry.handleDeleteGameCenterLeaderboardSet`.
async fn handle_delete_game_center_leaderboard_set(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\" }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
    }
    }
    todo!("port handleDeleteGameCenterLeaderboardSet from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

/// Ported from `Registry.handleListGameCenterLeaderboardSetMembers`.
async fn handle_list_game_center_leaderboard_set_members(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\"; tools.gcListParams }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
        gc_list_params: GcListParams => "gcListParams", flatten;
    }
    }
    todo!("port handleListGameCenterLeaderboardSetMembers from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

/// Ported from `Registry.handleAddGameCenterLeaderboardSetMembers`.
async fn handle_add_game_center_leaderboard_set_members(c: Arc<Client>, args: Args) -> HandlerResult {
    todo!("port handleAddGameCenterLeaderboardSetMembers from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

/// Ported from `Registry.handleRemoveGameCenterLeaderboardSetMembers`.
async fn handle_remove_game_center_leaderboard_set_members(c: Arc<Client>, args: Args) -> HandlerResult {
    todo!("port handleRemoveGameCenterLeaderboardSetMembers from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

/// Ported from `Registry.handleListGameCenterLeaderboardSetVersions`.
async fn handle_list_game_center_leaderboard_set_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\"; tools.gcListParams }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
        gc_list_params: GcListParams => "gcListParams", flatten;
    }
    }
    todo!("port handleListGameCenterLeaderboardSetVersions from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

/// Ported from `Registry.handleGetGameCenterLeaderboardSetVersion`.
async fn handle_get_game_center_leaderboard_set_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    todo!("port handleGetGameCenterLeaderboardSetVersion from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

/// Ported from `Registry.handleCreateGameCenterLeaderboardSetVersion`.
async fn handle_create_game_center_leaderboard_set_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardSetID string \"json:\\\"leaderboard_set_id\\\"\" }", "") {
        leaderboard_set_id: String => "leaderboard_set_id";
    }
    }
    todo!("port handleCreateGameCenterLeaderboardSetVersion from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

/// Ported from `Registry.handleListGameCenterLeaderboardSetLocalizations`.
async fn handle_list_game_center_leaderboard_set_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; tools.gcListParams }", "") {
        version_id: String => "version_id";
        gc_list_params: GcListParams => "gcListParams", flatten;
    }
    }
    todo!("port handleListGameCenterLeaderboardSetLocalizations from internal/asc/tools/gamecenter_leaderboard_sets.go")
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
    todo!("port handleCreateGameCenterLeaderboardSetLocalization from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

/// Ported from `Registry.handleUpdateGameCenterLeaderboardSetLocalization`.
async fn handle_update_game_center_leaderboard_set_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Name string \"json:\\\"name\\\"\" }", "") {
        localization_id: String => "localization_id";
        name: String => "name";
    }
    }
    todo!("port handleUpdateGameCenterLeaderboardSetLocalization from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

/// Ported from `Registry.handleDeleteGameCenterLeaderboardSetLocalization`.
async fn handle_delete_game_center_leaderboard_set_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    todo!("port handleDeleteGameCenterLeaderboardSetLocalization from internal/asc/tools/gamecenter_leaderboard_sets.go")
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
    todo!("port handleUploadGameCenterLeaderboardSetImage from internal/asc/tools/gamecenter_leaderboard_sets.go")
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
    todo!("port handleListGameCenterLeaderboardSetMemberLocalizations from internal/asc/tools/gamecenter_leaderboard_sets.go")
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
    todo!("port handleCreateGameCenterLeaderboardSetMemberLocalization from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

/// Ported from `Registry.handleUpdateGameCenterLeaderboardSetMemberLocalization`.
async fn handle_update_game_center_leaderboard_set_member_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { MemberLocalizationID string \"json:\\\"member_localization_id\\\"\"; Name string \"json:\\\"name\\\"\" }", "") {
        member_localization_id: String => "member_localization_id";
        name: String => "name";
    }
    }
    todo!("port handleUpdateGameCenterLeaderboardSetMemberLocalization from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

/// Ported from `Registry.handleDeleteGameCenterLeaderboardSetMemberLocalization`.
async fn handle_delete_game_center_leaderboard_set_member_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { MemberLocalizationID string \"json:\\\"member_localization_id\\\"\" }", "") {
        member_localization_id: String => "member_localization_id";
    }
    }
    todo!("port handleDeleteGameCenterLeaderboardSetMemberLocalization from internal/asc/tools/gamecenter_leaderboard_sets.go")
}

