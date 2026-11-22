//! Ported from internal/asc/tools/gamecenter_challenges.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerGameCenterChallengeTools`.
pub(super) fn register_game_center_challenge_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_game_center_challenges",
            "description": "List the Game Center challenges for an app",
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
                        "description": "Maximum number of challenges to return (default 50, max 200)"
                    }
                },
                "required": [
                    "game_center_detail_id"
                ]
            },
            "annotations": {
                "title": "List Game Center Challenges",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_game_center_challenges),
    );
    r.register(
        tool(r##"{
            "name": "get_game_center_challenge",
            "description": "Get details of a specific Game Center challenge",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "challenge_id": {
                        "type": "string",
                        "description": "The challenge ID"
                    }
                },
                "required": [
                    "challenge_id"
                ]
            },
            "annotations": {
                "title": "Get Game Center Challenge",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_game_center_challenge),
    );
    r.register(
        tool(r##"{
            "name": "create_game_center_challenge",
            "description": "Create a Game Center challenge along with its initial version. Provide exactly one of game_center_detail_id (app-owned) or game_center_group_id (shared across a group). A challenge is scored by the leaderboard given in leaderboard_id.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "challenge_type": {
                        "type": "string",
                        "description": "Challenge type. Only LEADERBOARD is supported; defaults to LEADERBOARD."
                    },
                    "game_center_detail_id": {
                        "type": "string",
                        "description": "The Game Center detail ID to own the challenge"
                    },
                    "game_center_group_id": {
                        "type": "string",
                        "description": "The Game Center group ID to own the challenge instead of a single app"
                    },
                    "leaderboard_id": {
                        "type": "string",
                        "description": "The Game Center leaderboard ID the challenge scores against"
                    },
                    "reference_name": {
                        "type": "string",
                        "description": "Internal reference name"
                    },
                    "repeatable": {
                        "type": "boolean",
                        "description": "Whether players can take on the challenge more than once"
                    },
                    "vendor_identifier": {
                        "type": "string",
                        "description": "Unique identifier for the challenge"
                    }
                },
                "required": [
                    "reference_name",
                    "vendor_identifier"
                ]
            },
            "annotations": {
                "title": "Create Game Center Challenge",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_game_center_challenge),
    );
    r.register(
        tool(r##"{
            "name": "update_game_center_challenge",
            "description": "Update a Game Center challenge. The vendor identifier and challenge type are immutable.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "archived": {
                        "type": "boolean",
                        "description": "Archive the challenge"
                    },
                    "challenge_id": {
                        "type": "string",
                        "description": "The challenge ID"
                    },
                    "leaderboard_id": {
                        "type": "string",
                        "description": "Point the challenge at a different Game Center leaderboard"
                    },
                    "reference_name": {
                        "type": "string",
                        "description": "Updated reference name"
                    },
                    "repeatable": {
                        "type": "boolean",
                        "description": "Whether players can take on the challenge more than once"
                    }
                },
                "required": [
                    "challenge_id"
                ]
            },
            "annotations": {
                "title": "Update Game Center Challenge",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_game_center_challenge),
    );
    r.register(
        tool(r##"{
            "name": "delete_game_center_challenge",
            "description": "Delete a Game Center challenge",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "challenge_id": {
                        "type": "string",
                        "description": "The challenge ID"
                    }
                },
                "required": [
                    "challenge_id"
                ]
            },
            "annotations": {
                "title": "Delete Game Center Challenge",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_game_center_challenge),
    );
    r.register(
        tool(r##"{
            "name": "list_game_center_challenge_versions",
            "description": "List the versions of a Game Center challenge, including each version's review state",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "challenge_id": {
                        "type": "string",
                        "description": "The challenge ID"
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
                    "challenge_id"
                ]
            },
            "annotations": {
                "title": "List Game Center Challenge Versions",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_game_center_challenge_versions),
    );
    r.register(
        tool(r##"{
            "name": "get_game_center_challenge_version",
            "description": "Get a specific Game Center challenge version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "version_id": {
                        "type": "string",
                        "description": "The challenge version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Get Game Center Challenge Version",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_game_center_challenge_version),
    );
    r.register(
        tool(r##"{
            "name": "create_game_center_challenge_version",
            "description": "Open a new editable version of a Game Center challenge. Attach localizations to the new version, then submit it for review with add_review_submission_item.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "challenge_id": {
                        "type": "string",
                        "description": "The challenge ID to open a version for"
                    }
                },
                "required": [
                    "challenge_id"
                ]
            },
            "annotations": {
                "title": "Create Game Center Challenge Version",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_game_center_challenge_version),
    );
    r.register(
        tool(r##"{
            "name": "list_game_center_challenge_localizations",
            "description": "List the localizations of a Game Center challenge version",
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
                        "description": "The challenge version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "List Game Center Challenge Localizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_game_center_challenge_localizations),
    );
    r.register(
        tool(r##"{
            "name": "create_game_center_challenge_localization",
            "description": "Add a localized name and description for a Game Center challenge version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "description": {
                        "type": "string",
                        "description": "The localized challenge description"
                    },
                    "locale": {
                        "type": "string",
                        "description": "The locale code (e.g. en-US)"
                    },
                    "name": {
                        "type": "string",
                        "description": "The localized challenge name shown to players"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The challenge version ID"
                    }
                },
                "required": [
                    "version_id",
                    "locale",
                    "name"
                ]
            },
            "annotations": {
                "title": "Create Game Center Challenge Localization",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_game_center_challenge_localization),
    );
    r.register(
        tool(r##"{
            "name": "update_game_center_challenge_localization",
            "description": "Update the localized name or description of a Game Center challenge",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "description": {
                        "type": "string",
                        "description": "The updated localized description"
                    },
                    "localization_id": {
                        "type": "string",
                        "description": "The challenge localization ID"
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
                "title": "Update Game Center Challenge Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_game_center_challenge_localization),
    );
    r.register(
        tool(r##"{
            "name": "delete_game_center_challenge_localization",
            "description": "Delete a Game Center challenge localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The challenge localization ID"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Delete Game Center Challenge Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_game_center_challenge_localization),
    );
    r.register(
        tool(r##"{
            "name": "upload_game_center_challenge_image",
            "description": "Reserve, upload, and commit a Game Center challenge image. Provide exactly one of localization_id (that locale's image) or version_id (the version's default image), and exactly one of file_path or file_data_base64.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "file_data_base64": {
                        "type": "string",
                        "description": "Standard base64 (RFC 4648) encoded bytes of the file. Use this when the client must transport the file across a remote transport such as HTTP."
                    },
                    "file_name": {
                        "type": "string",
                        "description": "The original file name (e.g. challenge.png)"
                    },
                    "file_path": {
                        "type": "string",
                        "description": "Absolute path to the file on the server's local filesystem. Use this when the MCP server and the file are on the same host (most stdio scenarios)."
                    },
                    "localization_id": {
                        "type": "string",
                        "description": "The challenge localization ID to attach the image to"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The challenge version ID to attach the image to as its default image"
                    }
                },
                "required": [
                    "file_name"
                ]
            },
            "annotations": {
                "title": "Upload Game Center Challenge Image",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_upload_game_center_challenge_image),
    );
}

/// Ported from `Registry.handleListGameCenterChallenges`.
async fn handle_list_game_center_challenges(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GameCenterDetailID string \"json:\\\"game_center_detail_id\\\"\"; tools.gcListParams }", "") {
        game_center_detail_id: String => "game_center_detail_id";
        gc_list_params: GcListParams => "gcListParams", flatten;
    }
    }
    todo!("port handleListGameCenterChallenges from internal/asc/tools/gamecenter_challenges.go")
}

/// Ported from `Registry.handleGetGameCenterChallenge`.
async fn handle_get_game_center_challenge(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ChallengeID string \"json:\\\"challenge_id\\\"\" }", "") {
        challenge_id: String => "challenge_id";
    }
    }
    todo!("port handleGetGameCenterChallenge from internal/asc/tools/gamecenter_challenges.go")
}

/// Ported from `Registry.handleCreateGameCenterChallenge`.
async fn handle_create_game_center_challenge(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GameCenterDetailID string \"json:\\\"game_center_detail_id\\\"\"; GameCenterGroupID string \"json:\\\"game_center_group_id\\\"\"; ReferenceName string \"json:\\\"reference_name\\\"\"; VendorIdentifier string \"json:\\\"vendor_identifier\\\"\"; ChallengeType string \"json:\\\"challenge_type\\\"\"; LeaderboardID string \"json:\\\"leaderboard_id\\\"\"; Repeatable *bool \"json:\\\"repeatable\\\"\" }", "") {
        game_center_detail_id: String => "game_center_detail_id";
        game_center_group_id: String => "game_center_group_id";
        reference_name: String => "reference_name";
        vendor_identifier: String => "vendor_identifier";
        challenge_type: String => "challenge_type";
        leaderboard_id: String => "leaderboard_id";
        repeatable: Option<bool> => "repeatable";
    }
    }
    todo!("port handleCreateGameCenterChallenge from internal/asc/tools/gamecenter_challenges.go")
}

/// Ported from `Registry.handleUpdateGameCenterChallenge`.
async fn handle_update_game_center_challenge(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ChallengeID string \"json:\\\"challenge_id\\\"\"; ReferenceName string \"json:\\\"reference_name\\\"\"; LeaderboardID string \"json:\\\"leaderboard_id\\\"\"; Repeatable *bool \"json:\\\"repeatable\\\"\"; Archived *bool \"json:\\\"archived\\\"\" }", "") {
        challenge_id: String => "challenge_id";
        reference_name: String => "reference_name";
        leaderboard_id: String => "leaderboard_id";
        repeatable: Option<bool> => "repeatable";
        archived: Option<bool> => "archived";
    }
    }
    todo!("port handleUpdateGameCenterChallenge from internal/asc/tools/gamecenter_challenges.go")
}

/// Ported from `Registry.handleDeleteGameCenterChallenge`.
async fn handle_delete_game_center_challenge(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ChallengeID string \"json:\\\"challenge_id\\\"\" }", "") {
        challenge_id: String => "challenge_id";
    }
    }
    todo!("port handleDeleteGameCenterChallenge from internal/asc/tools/gamecenter_challenges.go")
}

/// Ported from `Registry.handleListGameCenterChallengeVersions`.
async fn handle_list_game_center_challenge_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ChallengeID string \"json:\\\"challenge_id\\\"\"; tools.gcListParams }", "") {
        challenge_id: String => "challenge_id";
        gc_list_params: GcListParams => "gcListParams", flatten;
    }
    }
    todo!("port handleListGameCenterChallengeVersions from internal/asc/tools/gamecenter_challenges.go")
}

/// Ported from `Registry.handleGetGameCenterChallengeVersion`.
async fn handle_get_game_center_challenge_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    todo!("port handleGetGameCenterChallengeVersion from internal/asc/tools/gamecenter_challenges.go")
}

/// Ported from `Registry.handleCreateGameCenterChallengeVersion`.
async fn handle_create_game_center_challenge_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ChallengeID string \"json:\\\"challenge_id\\\"\" }", "") {
        challenge_id: String => "challenge_id";
    }
    }
    todo!("port handleCreateGameCenterChallengeVersion from internal/asc/tools/gamecenter_challenges.go")
}

/// Ported from `Registry.handleListGameCenterChallengeLocalizations`.
async fn handle_list_game_center_challenge_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; tools.gcListParams }", "") {
        version_id: String => "version_id";
        gc_list_params: GcListParams => "gcListParams", flatten;
    }
    }
    todo!("port handleListGameCenterChallengeLocalizations from internal/asc/tools/gamecenter_challenges.go")
}

/// Ported from `Registry.handleCreateGameCenterChallengeLocalization`.
async fn handle_create_game_center_challenge_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; Locale string \"json:\\\"locale\\\"\"; Name string \"json:\\\"name\\\"\"; Description string \"json:\\\"description\\\"\" }", "") {
        version_id: String => "version_id";
        locale: String => "locale";
        name: String => "name";
        description: String => "description";
    }
    }
    todo!("port handleCreateGameCenterChallengeLocalization from internal/asc/tools/gamecenter_challenges.go")
}

/// Ported from `Registry.handleUpdateGameCenterChallengeLocalization`.
async fn handle_update_game_center_challenge_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Name string \"json:\\\"name\\\"\"; Description string \"json:\\\"description\\\"\" }", "") {
        localization_id: String => "localization_id";
        name: String => "name";
        description: String => "description";
    }
    }
    todo!("port handleUpdateGameCenterChallengeLocalization from internal/asc/tools/gamecenter_challenges.go")
}

/// Ported from `Registry.handleDeleteGameCenterChallengeLocalization`.
async fn handle_delete_game_center_challenge_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    todo!("port handleDeleteGameCenterChallengeLocalization from internal/asc/tools/gamecenter_challenges.go")
}

/// Ported from `Registry.handleUploadGameCenterChallengeImage`.
async fn handle_upload_game_center_challenge_image(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; VersionID string \"json:\\\"version_id\\\"\"; FileName string \"json:\\\"file_name\\\"\"; FilePath string \"json:\\\"file_path\\\"\"; FileDataBase64 string \"json:\\\"file_data_base64\\\"\" }", "") {
        localization_id: String => "localization_id";
        version_id: String => "version_id";
        file_name: String => "file_name";
        file_path: String => "file_path";
        file_data_base64: String => "file_data_base64";
    }
    }
    todo!("port handleUploadGameCenterChallengeImage from internal/asc/tools/gamecenter_challenges.go")
}

