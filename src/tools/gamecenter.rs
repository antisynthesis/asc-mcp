//! Ported from internal/asc/tools/gamecenter.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerGameCenterTools`.
pub(super) fn register_game_center_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "get_game_center_detail",
            "description": "Get Game Center details for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID"
                    }
                },
                "required": [
                    "app_id"
                ]
            },
            "annotations": {
                "title": "Get Game Center Detail",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_game_center_detail),
    );
    r.register(
        tool(r##"{
            "name": "list_game_center_achievements",
            "description": "List Game Center achievements for an app",
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
                        "description": "Maximum number of achievements to return (default 50)"
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
                    "game_center_detail_id"
                ]
            },
            "annotations": {
                "title": "List Game Center Achievements",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_game_center_achievements),
    );
    r.register(
        tool(r##"{
            "name": "get_game_center_achievement",
            "description": "Get details of a specific Game Center achievement",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "achievement_id": {
                        "type": "string",
                        "description": "The achievement ID"
                    }
                },
                "required": [
                    "achievement_id"
                ]
            },
            "annotations": {
                "title": "Get Game Center Achievement",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_game_center_achievement),
    );
    r.register(
        tool(r##"{
            "name": "create_game_center_achievement",
            "description": "Create a new Game Center achievement",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "game_center_detail_id": {
                        "type": "string",
                        "description": "The Game Center detail ID"
                    },
                    "points": {
                        "type": "integer",
                        "description": "Points awarded for the achievement"
                    },
                    "reference_name": {
                        "type": "string",
                        "description": "Internal reference name"
                    },
                    "repeatable": {
                        "type": "boolean",
                        "description": "Achievement can be earned multiple times"
                    },
                    "show_before_earned": {
                        "type": "boolean",
                        "description": "Show achievement before earned"
                    },
                    "vendor_identifier": {
                        "type": "string",
                        "description": "Unique identifier for the achievement"
                    }
                },
                "required": [
                    "game_center_detail_id",
                    "reference_name",
                    "vendor_identifier",
                    "points"
                ]
            },
            "annotations": {
                "title": "Create Game Center Achievement",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_game_center_achievement),
    );
    r.register(
        tool(r##"{
            "name": "update_game_center_achievement",
            "description": "Update a Game Center achievement",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "achievement_id": {
                        "type": "string",
                        "description": "The achievement ID"
                    },
                    "archived": {
                        "type": "boolean",
                        "description": "Archive the achievement"
                    },
                    "points": {
                        "type": "integer",
                        "description": "Updated points"
                    },
                    "reference_name": {
                        "type": "string",
                        "description": "Updated reference name"
                    },
                    "repeatable": {
                        "type": "boolean",
                        "description": "Achievement can be earned multiple times"
                    },
                    "show_before_earned": {
                        "type": "boolean",
                        "description": "Show achievement before earned"
                    }
                },
                "required": [
                    "achievement_id"
                ]
            },
            "annotations": {
                "title": "Update Game Center Achievement",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_game_center_achievement),
    );
    r.register(
        tool(r##"{
            "name": "delete_game_center_achievement",
            "description": "Delete a Game Center achievement",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "achievement_id": {
                        "type": "string",
                        "description": "The achievement ID"
                    }
                },
                "required": [
                    "achievement_id"
                ]
            },
            "annotations": {
                "title": "Delete Game Center Achievement",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_game_center_achievement),
    );
    r.register(
        tool(r##"{
            "name": "list_game_center_leaderboards",
            "description": "List Game Center leaderboards for an app",
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
                        "description": "Maximum number of leaderboards to return (default 50)"
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
                    "game_center_detail_id"
                ]
            },
            "annotations": {
                "title": "List Game Center Leaderboards",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_game_center_leaderboards),
    );
    r.register(
        tool(r##"{
            "name": "get_game_center_leaderboard",
            "description": "Get details of a specific Game Center leaderboard",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "leaderboard_id": {
                        "type": "string",
                        "description": "The leaderboard ID"
                    }
                },
                "required": [
                    "leaderboard_id"
                ]
            },
            "annotations": {
                "title": "Get Game Center Leaderboard",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_game_center_leaderboard),
    );
    r.register(
        tool(r##"{
            "name": "create_game_center_leaderboard",
            "description": "Create a new Game Center leaderboard",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "default_formatter": {
                        "type": "string",
                        "description": "Score display format (e.g. INTEGER, DECIMAL_POINT_1_PLACE, ELAPSED_TIME_SECOND, MONEY_DOLLAR). Defaults to INTEGER."
                    },
                    "game_center_detail_id": {
                        "type": "string",
                        "description": "The Game Center detail ID"
                    },
                    "reference_name": {
                        "type": "string",
                        "description": "Internal reference name"
                    },
                    "score_range_end": {
                        "type": "string",
                        "description": "Maximum valid score"
                    },
                    "score_range_start": {
                        "type": "string",
                        "description": "Minimum valid score"
                    },
                    "score_sort_type": {
                        "type": "string",
                        "description": "How scores are sorted (ASC, DESC)"
                    },
                    "submission_type": {
                        "type": "string",
                        "description": "Score submission type (BEST_SCORE, MOST_RECENT_SCORE)"
                    },
                    "vendor_identifier": {
                        "type": "string",
                        "description": "Unique identifier for the leaderboard"
                    }
                },
                "required": [
                    "game_center_detail_id",
                    "reference_name",
                    "vendor_identifier",
                    "submission_type",
                    "score_sort_type"
                ]
            },
            "annotations": {
                "title": "Create Game Center Leaderboard",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_game_center_leaderboard),
    );
    r.register(
        tool(r##"{
            "name": "update_game_center_leaderboard",
            "description": "Update a Game Center leaderboard",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "archived": {
                        "type": "boolean",
                        "description": "Archive the leaderboard"
                    },
                    "leaderboard_id": {
                        "type": "string",
                        "description": "The leaderboard ID"
                    },
                    "reference_name": {
                        "type": "string",
                        "description": "Updated reference name"
                    },
                    "score_sort_type": {
                        "type": "string",
                        "description": "Updated sort type"
                    },
                    "submission_type": {
                        "type": "string",
                        "description": "Updated submission type"
                    }
                },
                "required": [
                    "leaderboard_id"
                ]
            },
            "annotations": {
                "title": "Update Game Center Leaderboard",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_game_center_leaderboard),
    );
    r.register(
        tool(r##"{
            "name": "delete_game_center_leaderboard",
            "description": "Delete a Game Center leaderboard",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "leaderboard_id": {
                        "type": "string",
                        "description": "The leaderboard ID"
                    }
                },
                "required": [
                    "leaderboard_id"
                ]
            },
            "annotations": {
                "title": "Delete Game Center Leaderboard",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_game_center_leaderboard),
    );
}

/// Ported from `Registry.handleGetGameCenterDetail`.
async fn handle_get_game_center_detail(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
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

    let resp = match c.get_game_center_detail(&params.app_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get Game Center detail: {e}"))),
    };

    Ok(data_result(&format_game_center_detail(&resp.data), &resp.data))
}

/// Ported from `Registry.handleListGameCenterAchievements`.
async fn handle_list_game_center_achievements(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GameCenterDetailID string \"json:\\\"game_center_detail_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        game_center_detail_id: String => "game_center_detail_id";
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

    if params.game_center_detail_id.is_empty() {
        return Ok(error_result("game_center_detail_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_game_center_achievements(&params.game_center_detail_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list achievements: {e}"))),
    };

    Ok(list_result(&format_game_center_achievements(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetGameCenterAchievement`.
async fn handle_get_game_center_achievement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AchievementID string \"json:\\\"achievement_id\\\"\" }", "") {
        achievement_id: String => "achievement_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.achievement_id.is_empty() {
        return Ok(error_result("achievement_id is required"));
    }

    let resp = match c.get_game_center_achievement(&params.achievement_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get achievement: {e}"))),
    };

    Ok(data_result(&format_game_center_achievement(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateGameCenterAchievement`.
async fn handle_create_game_center_achievement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GameCenterDetailID string \"json:\\\"game_center_detail_id\\\"\"; ReferenceName string \"json:\\\"reference_name\\\"\"; VendorIdentifier string \"json:\\\"vendor_identifier\\\"\"; Points int \"json:\\\"points\\\"\"; ShowBeforeEarned bool \"json:\\\"show_before_earned\\\"\"; Repeatable bool \"json:\\\"repeatable\\\"\" }", "") {
        game_center_detail_id: String => "game_center_detail_id";
        reference_name: String => "reference_name";
        vendor_identifier: String => "vendor_identifier";
        points: i64 => "points";
        show_before_earned: bool => "show_before_earned";
        repeatable: bool => "repeatable";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.game_center_detail_id.is_empty() {
        return Ok(error_result("game_center_detail_id is required"));
    }
    if params.reference_name.is_empty() {
        return Ok(error_result("reference_name is required"));
    }
    if params.vendor_identifier.is_empty() {
        return Ok(error_result("vendor_identifier is required"));
    }

    // The v2 API requires an initial achievement version, created inline with a
    // client-chosen temporary ID.
    let req = GameCenterAchievementCreateRequest {
        data: GameCenterAchievementCreateData {
            type_: "gameCenterAchievements".into(),
            attributes: GameCenterAchievementCreateAttributes {
                reference_name: params.reference_name.clone(),
                vendor_identifier: params.vendor_identifier.clone(),
                points: params.points,
                show_before_earned: params.show_before_earned,
                repeatable: params.repeatable,
            },
            relationships: GameCenterAchievementCreateRelationships {
                game_center_detail: RelationshipData {
                    data: ResourceIdentifier { type_: "gameCenterDetails".into(), id: params.game_center_detail_id.clone() },
                },
                versions: RelationshipDataList {
                    data: GoSlice::from(vec![ResourceIdentifier { type_: "gameCenterAchievementVersions".into(), id: "${new-version}".into() }]),
                },
            },
        },
        included: GoSlice::from(vec![GameCenterVersionInlineCreate { type_: "gameCenterAchievementVersions".into(), id: "${new-version}".into() }]),
    };

    let resp = match c.create_game_center_achievement(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create achievement: {e}"))),
    };

    Ok(data_result(&format!("Created achievement: {} (ID: {})", resp.data.attributes.reference_name, resp.data.id), &resp.data))
}

/// Ported from `Registry.handleUpdateGameCenterAchievement`.
async fn handle_update_game_center_achievement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AchievementID string \"json:\\\"achievement_id\\\"\"; ReferenceName string \"json:\\\"reference_name\\\"\"; Points *int \"json:\\\"points\\\"\"; ShowBeforeEarned *bool \"json:\\\"show_before_earned\\\"\"; Repeatable *bool \"json:\\\"repeatable\\\"\"; Archived *bool \"json:\\\"archived\\\"\" }", "") {
        achievement_id: String => "achievement_id";
        reference_name: String => "reference_name";
        points: Option<i64> => "points";
        show_before_earned: Option<bool> => "show_before_earned";
        repeatable: Option<bool> => "repeatable";
        archived: Option<bool> => "archived";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.achievement_id.is_empty() {
        return Ok(error_result("achievement_id is required"));
    }

    let req = GameCenterAchievementUpdateRequest {
        data: GameCenterAchievementUpdateData {
            type_: "gameCenterAchievements".into(),
            id: params.achievement_id.clone(),
            attributes: GameCenterAchievementUpdateAttributes {
                reference_name: params.reference_name.clone(),
                points: params.points,
                show_before_earned: params.show_before_earned,
                repeatable: params.repeatable,
                archived: params.archived,
            },
        },
    };

    let resp = match c.update_game_center_achievement(&params.achievement_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update achievement: {e}"))),
    };

    Ok(data_result(&format!("Updated achievement: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleDeleteGameCenterAchievement`.
async fn handle_delete_game_center_achievement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AchievementID string \"json:\\\"achievement_id\\\"\" }", "") {
        achievement_id: String => "achievement_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.achievement_id.is_empty() {
        return Ok(error_result("achievement_id is required"));
    }

    if let Err(e) = c.delete_game_center_achievement(&params.achievement_id).await {
        return Ok(error_result(format!("Failed to delete achievement: {e}")));
    }

    Ok(CallToolResult::success("Achievement deleted successfully"))
}

/// Ported from `Registry.handleListGameCenterLeaderboards`.
async fn handle_list_game_center_leaderboards(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GameCenterDetailID string \"json:\\\"game_center_detail_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        game_center_detail_id: String => "game_center_detail_id";
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

    if params.game_center_detail_id.is_empty() {
        return Ok(error_result("game_center_detail_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_game_center_leaderboards(&params.game_center_detail_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list leaderboards: {e}"))),
    };

    Ok(list_result(&format_game_center_leaderboards(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetGameCenterLeaderboard`.
async fn handle_get_game_center_leaderboard(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardID string \"json:\\\"leaderboard_id\\\"\" }", "") {
        leaderboard_id: String => "leaderboard_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.leaderboard_id.is_empty() {
        return Ok(error_result("leaderboard_id is required"));
    }

    let resp = match c.get_game_center_leaderboard(&params.leaderboard_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get leaderboard: {e}"))),
    };

    Ok(data_result(&format_game_center_leaderboard(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateGameCenterLeaderboard`.
async fn handle_create_game_center_leaderboard(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GameCenterDetailID string \"json:\\\"game_center_detail_id\\\"\"; ReferenceName string \"json:\\\"reference_name\\\"\"; VendorIdentifier string \"json:\\\"vendor_identifier\\\"\"; SubmissionType string \"json:\\\"submission_type\\\"\"; ScoreSortType string \"json:\\\"score_sort_type\\\"\"; ScoreRangeStart string \"json:\\\"score_range_start\\\"\"; ScoreRangeEnd string \"json:\\\"score_range_end\\\"\"; DefaultFormatter string \"json:\\\"default_formatter\\\"\" }", "") {
        game_center_detail_id: String => "game_center_detail_id";
        reference_name: String => "reference_name";
        vendor_identifier: String => "vendor_identifier";
        submission_type: String => "submission_type";
        score_sort_type: String => "score_sort_type";
        score_range_start: String => "score_range_start";
        score_range_end: String => "score_range_end";
        default_formatter: String => "default_formatter";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.game_center_detail_id.is_empty() {
        return Ok(error_result("game_center_detail_id is required"));
    }
    if params.reference_name.is_empty() {
        return Ok(error_result("reference_name is required"));
    }
    if params.vendor_identifier.is_empty() {
        return Ok(error_result("vendor_identifier is required"));
    }
    if params.submission_type.is_empty() {
        return Ok(error_result("submission_type is required"));
    }
    if params.score_sort_type.is_empty() {
        return Ok(error_result("score_sort_type is required"));
    }

    // The v2 API requires a default formatter for score display.
    let mut formatter = params.default_formatter.clone();
    if formatter.is_empty() {
        formatter = "INTEGER".into();
    }

    // The v2 API requires an initial leaderboard version, created inline with a
    // client-chosen temporary ID.
    let req = GameCenterLeaderboardCreateRequest {
        data: GameCenterLeaderboardCreateData {
            type_: "gameCenterLeaderboards".into(),
            attributes: GameCenterLeaderboardCreateAttributes {
                default_formatter: formatter,
                reference_name: params.reference_name.clone(),
                vendor_identifier: params.vendor_identifier.clone(),
                submission_type: params.submission_type.clone(),
                score_sort_type: params.score_sort_type.clone(),
                score_range_start: params.score_range_start.clone(),
                score_range_end: params.score_range_end.clone(),
                ..Default::default()
            },
            relationships: GameCenterLeaderboardCreateRelationships {
                game_center_detail: RelationshipData {
                    data: ResourceIdentifier { type_: "gameCenterDetails".into(), id: params.game_center_detail_id.clone() },
                },
                versions: RelationshipDataList {
                    data: GoSlice::from(vec![ResourceIdentifier { type_: "gameCenterLeaderboardVersions".into(), id: "${new-version}".into() }]),
                },
            },
        },
        included: GoSlice::from(vec![GameCenterVersionInlineCreate { type_: "gameCenterLeaderboardVersions".into(), id: "${new-version}".into() }]),
    };

    let resp = match c.create_game_center_leaderboard(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create leaderboard: {e}"))),
    };

    Ok(data_result(&format!("Created leaderboard: {} (ID: {})", resp.data.attributes.reference_name, resp.data.id), &resp.data))
}

/// Ported from `Registry.handleUpdateGameCenterLeaderboard`.
async fn handle_update_game_center_leaderboard(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardID string \"json:\\\"leaderboard_id\\\"\"; ReferenceName string \"json:\\\"reference_name\\\"\"; SubmissionType string \"json:\\\"submission_type\\\"\"; ScoreSortType string \"json:\\\"score_sort_type\\\"\"; Archived *bool \"json:\\\"archived\\\"\" }", "") {
        leaderboard_id: String => "leaderboard_id";
        reference_name: String => "reference_name";
        submission_type: String => "submission_type";
        score_sort_type: String => "score_sort_type";
        archived: Option<bool> => "archived";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.leaderboard_id.is_empty() {
        return Ok(error_result("leaderboard_id is required"));
    }

    let req = GameCenterLeaderboardUpdateRequest {
        data: GameCenterLeaderboardUpdateData {
            type_: "gameCenterLeaderboards".into(),
            id: params.leaderboard_id.clone(),
            attributes: GameCenterLeaderboardUpdateAttributes {
                reference_name: params.reference_name.clone(),
                submission_type: params.submission_type.clone(),
                score_sort_type: params.score_sort_type.clone(),
                archived: params.archived,
                ..Default::default()
            },
        },
    };

    let resp = match c.update_game_center_leaderboard(&params.leaderboard_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update leaderboard: {e}"))),
    };

    Ok(data_result(&format!("Updated leaderboard: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleDeleteGameCenterLeaderboard`.
async fn handle_delete_game_center_leaderboard(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardID string \"json:\\\"leaderboard_id\\\"\" }", "") {
        leaderboard_id: String => "leaderboard_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.leaderboard_id.is_empty() {
        return Ok(error_result("leaderboard_id is required"));
    }

    if let Err(e) = c.delete_game_center_leaderboard(&params.leaderboard_id).await {
        return Ok(error_result(format!("Failed to delete leaderboard: {e}")));
    }

    Ok(CallToolResult::success("Leaderboard deleted successfully"))
}

/// Ported from `formatGameCenterDetail`.
fn format_game_center_detail(detail: &GameCenterDetail) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "Game Center Detail ID: {}", detail.id);
    let _ = writeln!(sb, "Arcade Enabled: {}", detail.attributes.arcade_enabled);
    let _ = writeln!(sb, "Challenge Enabled: {}", detail.attributes.challenge_enabled);
    sb
}

/// Ported from `formatGameCenterAchievements`.
fn format_game_center_achievements(achievements: &[GameCenterAchievement]) -> String {
    if achievements.is_empty() {
        return "No Game Center achievements found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} achievements:\n\n", achievements.len());

    for achievement in achievements {
        sb.push_str(&format_game_center_achievement(achievement));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatGameCenterAchievement`.
fn format_game_center_achievement(achievement: &GameCenterAchievement) -> String {
    let a = &achievement.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", achievement.id);
    let _ = writeln!(sb, "Reference Name: {}", a.reference_name);
    let _ = writeln!(sb, "Vendor ID: {}", a.vendor_identifier);
    let _ = writeln!(sb, "Points: {}", a.points);
    let _ = writeln!(sb, "Show Before Earned: {}", a.show_before_earned);
    let _ = writeln!(sb, "Repeatable: {}", a.repeatable);
    let _ = writeln!(sb, "Archived: {}", a.archived);
    sb
}

/// Ported from `formatGameCenterLeaderboards`.
pub(super) fn format_game_center_leaderboards(leaderboards: &[GameCenterLeaderboard]) -> String {
    if leaderboards.is_empty() {
        return "No Game Center leaderboards found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} leaderboards:\n\n", leaderboards.len());

    for leaderboard in leaderboards {
        sb.push_str(&format_game_center_leaderboard(leaderboard));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatGameCenterLeaderboard`.
fn format_game_center_leaderboard(leaderboard: &GameCenterLeaderboard) -> String {
    let a = &leaderboard.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", leaderboard.id);
    let _ = writeln!(sb, "Reference Name: {}", a.reference_name);
    let _ = writeln!(sb, "Vendor ID: {}", a.vendor_identifier);
    let _ = writeln!(sb, "Submission Type: {}", a.submission_type);
    let _ = writeln!(sb, "Score Sort Type: {}", a.score_sort_type);
    if !a.score_range_start.is_empty() {
        let _ = writeln!(sb, "Score Range: {} - {}", a.score_range_start, a.score_range_end);
    }
    let _ = writeln!(sb, "Archived: {}", a.archived);
    sb
}
