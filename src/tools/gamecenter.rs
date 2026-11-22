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
    todo!("port handleGetGameCenterDetail from internal/asc/tools/gamecenter.go")
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
    todo!("port handleListGameCenterAchievements from internal/asc/tools/gamecenter.go")
}

/// Ported from `Registry.handleGetGameCenterAchievement`.
async fn handle_get_game_center_achievement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AchievementID string \"json:\\\"achievement_id\\\"\" }", "") {
        achievement_id: String => "achievement_id";
    }
    }
    todo!("port handleGetGameCenterAchievement from internal/asc/tools/gamecenter.go")
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
    todo!("port handleCreateGameCenterAchievement from internal/asc/tools/gamecenter.go")
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
    todo!("port handleUpdateGameCenterAchievement from internal/asc/tools/gamecenter.go")
}

/// Ported from `Registry.handleDeleteGameCenterAchievement`.
async fn handle_delete_game_center_achievement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AchievementID string \"json:\\\"achievement_id\\\"\" }", "") {
        achievement_id: String => "achievement_id";
    }
    }
    todo!("port handleDeleteGameCenterAchievement from internal/asc/tools/gamecenter.go")
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
    todo!("port handleListGameCenterLeaderboards from internal/asc/tools/gamecenter.go")
}

/// Ported from `Registry.handleGetGameCenterLeaderboard`.
async fn handle_get_game_center_leaderboard(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardID string \"json:\\\"leaderboard_id\\\"\" }", "") {
        leaderboard_id: String => "leaderboard_id";
    }
    }
    todo!("port handleGetGameCenterLeaderboard from internal/asc/tools/gamecenter.go")
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
    todo!("port handleCreateGameCenterLeaderboard from internal/asc/tools/gamecenter.go")
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
    todo!("port handleUpdateGameCenterLeaderboard from internal/asc/tools/gamecenter.go")
}

/// Ported from `Registry.handleDeleteGameCenterLeaderboard`.
async fn handle_delete_game_center_leaderboard(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LeaderboardID string \"json:\\\"leaderboard_id\\\"\" }", "") {
        leaderboard_id: String => "leaderboard_id";
    }
    }
    todo!("port handleDeleteGameCenterLeaderboard from internal/asc/tools/gamecenter.go")
}

