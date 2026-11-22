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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.game_center_detail_id.is_empty() {
        return Ok(error_result("game_center_detail_id is required"));
    }

    let opts = params.gc_list_params.opts();
    let resp = match paginated_fetch(&c, &params.gc_list_params.cursor, || c.list_game_center_challenges(&params.game_center_detail_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list challenges: {e}"))),
    };

    Ok(list_result(&format_game_center_challenges(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetGameCenterChallenge`.
async fn handle_get_game_center_challenge(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ChallengeID string \"json:\\\"challenge_id\\\"\" }", "") {
        challenge_id: String => "challenge_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.challenge_id.is_empty() {
        return Ok(error_result("challenge_id is required"));
    }

    let resp = match c.get_game_center_challenge(&params.challenge_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get challenge: {e}"))),
    };

    Ok(data_result(&format_game_center_challenge(&resp.data), &resp.data))
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

    // LEADERBOARD is the only challenge type the API accepts today.
    let mut challenge_type = params.challenge_type.clone();
    if challenge_type.is_empty() {
        challenge_type = "LEADERBOARD".into();
    }

    // The initial challenge version is declared inline with a
    // client-chosen temporary ID.
    let mut rels = GameCenterChallengeCreateRelationships {
        versions: Some(RelationshipDataList {
            data: GoSlice::from(vec![ResourceIdentifier { type_: "gameCenterChallengeVersions".into(), id: "${new-version}".into() }]),
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
    if !params.leaderboard_id.is_empty() {
        rels.leaderboard_v2 = Some(RelationshipData {
            data: ResourceIdentifier { type_: "gameCenterLeaderboards".into(), id: params.leaderboard_id.clone() },
        });
    }

    let req = GameCenterChallengeCreateRequest {
        data: GameCenterChallengeCreateData {
            type_: "gameCenterChallenges".into(),
            attributes: GameCenterChallengeCreateAttributes {
                reference_name: params.reference_name.clone(),
                vendor_identifier: params.vendor_identifier.clone(),
                challenge_type,
                repeatable: params.repeatable,
            },
            relationships: rels,
        },
        included: GoSlice::from(vec![GameCenterVersionInlineCreate { type_: "gameCenterChallengeVersions".into(), id: "${new-version}".into() }]),
    };

    let resp = match c.create_game_center_challenge(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create challenge: {e}"))),
    };

    Ok(data_result(&format!("Created challenge: {} (ID: {})", resp.data.attributes.reference_name, resp.data.id), &resp.data))
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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.challenge_id.is_empty() {
        return Ok(error_result("challenge_id is required"));
    }

    let mut req = GameCenterChallengeUpdateRequest {
        data: GameCenterChallengeUpdateData {
            type_: "gameCenterChallenges".into(),
            id: params.challenge_id.clone(),
            attributes: GameCenterChallengeUpdateAttributes {
                reference_name: params.reference_name.clone(),
                repeatable: params.repeatable,
                archived: params.archived,
            },
            ..Default::default()
        },
    };
    if !params.leaderboard_id.is_empty() {
        req.data.relationships = Some(GameCenterChallengeUpdateRelationships {
            leaderboard_v2: Some(RelationshipData {
                data: ResourceIdentifier { type_: "gameCenterLeaderboards".into(), id: params.leaderboard_id.clone() },
            }),
        });
    }

    let resp = match c.update_game_center_challenge(&params.challenge_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update challenge: {e}"))),
    };

    Ok(data_result(&format!("Updated challenge: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleDeleteGameCenterChallenge`.
async fn handle_delete_game_center_challenge(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ChallengeID string \"json:\\\"challenge_id\\\"\" }", "") {
        challenge_id: String => "challenge_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.challenge_id.is_empty() {
        return Ok(error_result("challenge_id is required"));
    }

    if let Err(e) = c.delete_game_center_challenge(&params.challenge_id).await {
        return Ok(error_result(format!("Failed to delete challenge: {e}")));
    }

    Ok(CallToolResult::success("Challenge deleted successfully"))
}

/// Ported from `Registry.handleListGameCenterChallengeVersions`.
async fn handle_list_game_center_challenge_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ChallengeID string \"json:\\\"challenge_id\\\"\"; tools.gcListParams }", "") {
        challenge_id: String => "challenge_id";
        gc_list_params: GcListParams => "gcListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.challenge_id.is_empty() {
        return Ok(error_result("challenge_id is required"));
    }

    let opts = params.gc_list_params.opts();
    let resp = match paginated_fetch(&c, &params.gc_list_params.cursor, || c.list_game_center_challenge_versions(&params.challenge_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list challenge versions: {e}"))),
    };

    Ok(list_result(&format_game_center_challenge_versions(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetGameCenterChallengeVersion`.
async fn handle_get_game_center_challenge_version(c: Arc<Client>, args: Args) -> HandlerResult {
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

    let resp = match c.get_game_center_challenge_version(&params.version_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get challenge version: {e}"))),
    };

    Ok(data_result(&format_game_center_challenge_version(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateGameCenterChallengeVersion`.
async fn handle_create_game_center_challenge_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ChallengeID string \"json:\\\"challenge_id\\\"\" }", "") {
        challenge_id: String => "challenge_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.challenge_id.is_empty() {
        return Ok(error_result("challenge_id is required"));
    }

    let req = GameCenterChallengeVersionCreateRequest {
        data: GameCenterChallengeVersionCreateData {
            type_: "gameCenterChallengeVersions".into(),
            relationships: GameCenterChallengeVersionCreateRelationships {
                challenge: RelationshipData {
                    data: ResourceIdentifier { type_: "gameCenterChallenges".into(), id: params.challenge_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_game_center_challenge_version(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create challenge version: {e}"))),
    };

    Ok(data_result(&format!("Created challenge version: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleListGameCenterChallengeLocalizations`.
async fn handle_list_game_center_challenge_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
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
    let resp = match paginated_fetch(&c, &params.gc_list_params.cursor, || c.list_game_center_challenge_localizations(&params.version_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list challenge localizations: {e}"))),
    };

    Ok(list_result(&format_game_center_challenge_localizations(&resp.data), &resp.data, &resp.links))
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

    let req = GameCenterChallengeLocalizationCreateRequest {
        data: GameCenterChallengeLocalizationCreateData {
            type_: "gameCenterChallengeLocalizations".into(),
            attributes: GameCenterChallengeLocalizationCreateAttributes {
                locale: params.locale.clone(),
                name: params.name.clone(),
                description: params.description.clone(),
            },
            relationships: GameCenterChallengeLocalizationCreateRelationships {
                version: RelationshipData {
                    data: ResourceIdentifier { type_: "gameCenterChallengeVersions".into(), id: params.version_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_game_center_challenge_localization(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create challenge localization: {e}"))),
    };

    Ok(data_result(&format!("Created challenge localization {} for {}", resp.data.id, params.locale), &resp.data))
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

    let req = GameCenterChallengeLocalizationUpdateRequest {
        data: GameCenterChallengeLocalizationUpdateData {
            type_: "gameCenterChallengeLocalizations".into(),
            id: params.localization_id.clone(),
            attributes: GameCenterChallengeLocalizationUpdateAttributes {
                name: params.name.clone(),
                description: params.description.clone(),
            },
        },
    };

    let resp = match c.update_game_center_challenge_localization(&params.localization_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update challenge localization: {e}"))),
    };

    Ok(data_result(&format!("Updated challenge localization: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleDeleteGameCenterChallengeLocalization`.
async fn handle_delete_game_center_challenge_localization(c: Arc<Client>, args: Args) -> HandlerResult {
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

    if let Err(e) = c.delete_game_center_challenge_localization(&params.localization_id).await {
        return Ok(error_result(format!("Failed to delete challenge localization: {e}")));
    }

    Ok(CallToolResult::success("Challenge localization deleted successfully"))
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

    let resp = match c.upload_game_center_challenge_image(&params.localization_id, &params.version_id, &params.file_name, &body).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to upload challenge image: {e}"))),
    };

    let text = format!("Uploaded challenge image {} ({}, {} bytes).", resp.data.id, params.file_name, body.len());
    Ok(data_result(&text, &resp.data))
}

/// Ported from `formatGameCenterChallenges`.
fn format_game_center_challenges(challenges: &[GameCenterChallenge]) -> String {
    if challenges.is_empty() {
        return "No Game Center challenges found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} challenges:\n\n", challenges.len());

    for challenge in challenges {
        sb.push_str(&format_game_center_challenge(challenge));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatGameCenterChallenge`.
fn format_game_center_challenge(challenge: &GameCenterChallenge) -> String {
    let a = &challenge.attributes;
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", challenge.id);
    let _ = write!(sb, "Reference Name: {}\n", a.reference_name);
    let _ = write!(sb, "Vendor ID: {}\n", a.vendor_identifier);
    if !a.challenge_type.is_empty() {
        let _ = write!(sb, "Challenge Type: {}\n", a.challenge_type);
    }
    let _ = write!(sb, "Repeatable: {}\n", a.repeatable);
    let _ = write!(sb, "Archived: {}\n", a.archived);
    sb
}

/// Ported from `formatGameCenterChallengeVersions`.
fn format_game_center_challenge_versions(versions: &[GameCenterChallengeVersion]) -> String {
    if versions.is_empty() {
        return "No Game Center challenge versions found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} challenge versions:\n\n", versions.len());

    for version in versions {
        sb.push_str(&format_game_center_challenge_version(version));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatGameCenterChallengeVersion`.
fn format_game_center_challenge_version(version: &GameCenterChallengeVersion) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", version.id);
    let _ = write!(sb, "Version: {}\n", version.attributes.version);
    let _ = write!(sb, "State: {}\n", version.attributes.state);
    sb
}

/// Ported from `formatGameCenterChallengeLocalizations`.
fn format_game_center_challenge_localizations(localizations: &[GameCenterChallengeLocalization]) -> String {
    if localizations.is_empty() {
        return "No Game Center challenge localizations found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} challenge localizations:\n\n", localizations.len());

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
