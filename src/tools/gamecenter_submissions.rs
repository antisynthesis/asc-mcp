//! Ported from internal/asc/tools/gamecenter_submissions.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerGameCenterSubmissionTools`.
pub(super) fn register_game_center_submission_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "submit_game_center_player_achievement",
            "description": "Submit Game Center achievement progress for a player on the game server's behalf. percentage_achieved is 0-100; 100 earns the achievement.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "bundle_id": {
                        "type": "string",
                        "description": "The bundle ID of the game the progress belongs to (e.g. com.example.game)"
                    },
                    "challenge_ids": {
                        "type": "array",
                        "description": "Challenge IDs this progress counts toward",
                        "items": {
                            "type": "string"
                        }
                    },
                    "percentage_achieved": {
                        "type": "integer",
                        "description": "Progress toward the achievement, 0-100"
                    },
                    "pre_released": {
                        "type": "boolean",
                        "description": "Mark the progress as earned in a pre-release build so it stays out of production data"
                    },
                    "scoped_player_id": {
                        "type": "string",
                        "description": "The scoped player ID the Game Center client provided for this player"
                    },
                    "submitted_date": {
                        "type": "string",
                        "description": "ISO 8601 timestamp the progress was earned. Defaults to the time the API receives the request."
                    },
                    "vendor_identifier": {
                        "type": "string",
                        "description": "The achievement's vendor identifier"
                    }
                },
                "required": [
                    "bundle_id",
                    "vendor_identifier",
                    "scoped_player_id",
                    "percentage_achieved"
                ]
            },
            "annotations": {
                "title": "Submit Game Center Player Achievement",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_submit_game_center_player_achievement),
    );
    r.register(
        tool(r##"{
            "name": "submit_game_center_leaderboard_entry",
            "description": "Submit a Game Center leaderboard score for a player on the game server's behalf. score and context are decimal strings so large values keep full precision.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "bundle_id": {
                        "type": "string",
                        "description": "The bundle ID of the game the score belongs to (e.g. com.example.game)"
                    },
                    "challenge_ids": {
                        "type": "array",
                        "description": "Challenge IDs this score counts toward",
                        "items": {
                            "type": "string"
                        }
                    },
                    "context": {
                        "type": "string",
                        "description": "Optional game-defined context value stored with the score, as a decimal string"
                    },
                    "pre_released": {
                        "type": "boolean",
                        "description": "Mark the score as earned in a pre-release build so it stays out of production leaderboards"
                    },
                    "scoped_player_id": {
                        "type": "string",
                        "description": "The scoped player ID the Game Center client provided for this player"
                    },
                    "score": {
                        "type": "string",
                        "description": "The score to record, as a decimal string"
                    },
                    "submitted_date": {
                        "type": "string",
                        "description": "ISO 8601 timestamp the score was earned. Defaults to the time the API receives the request."
                    },
                    "vendor_identifier": {
                        "type": "string",
                        "description": "The leaderboard's vendor identifier"
                    }
                },
                "required": [
                    "bundle_id",
                    "vendor_identifier",
                    "scoped_player_id",
                    "score"
                ]
            },
            "annotations": {
                "title": "Submit Game Center Leaderboard Entry",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_submit_game_center_leaderboard_entry),
    );
}

/// Ported from `Registry.handleSubmitGameCenterPlayerAchievement`.
async fn handle_submit_game_center_player_achievement(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BundleID string \"json:\\\"bundle_id\\\"\"; VendorIdentifier string \"json:\\\"vendor_identifier\\\"\"; ScopedPlayerID string \"json:\\\"scoped_player_id\\\"\"; PercentageAchieved *int \"json:\\\"percentage_achieved\\\"\"; ChallengeIDs []string \"json:\\\"challenge_ids\\\"\"; SubmittedDate string \"json:\\\"submitted_date\\\"\"; PreReleased *bool \"json:\\\"pre_released\\\"\" }", "") {
        bundle_id: String => "bundle_id";
        vendor_identifier: String => "vendor_identifier";
        scoped_player_id: String => "scoped_player_id";
        percentage_achieved: Option<i64> => "percentage_achieved";
        challenge_ids: GoSlice<String> => "challenge_ids";
        submitted_date: String => "submitted_date";
        pre_released: Option<bool> => "pre_released";
    }
    }
    todo!("port handleSubmitGameCenterPlayerAchievement from internal/asc/tools/gamecenter_submissions.go")
}

/// Ported from `Registry.handleSubmitGameCenterLeaderboardEntry`.
async fn handle_submit_game_center_leaderboard_entry(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BundleID string \"json:\\\"bundle_id\\\"\"; VendorIdentifier string \"json:\\\"vendor_identifier\\\"\"; ScopedPlayerID string \"json:\\\"scoped_player_id\\\"\"; Score string \"json:\\\"score\\\"\"; Context string \"json:\\\"context\\\"\"; ChallengeIDs []string \"json:\\\"challenge_ids\\\"\"; SubmittedDate string \"json:\\\"submitted_date\\\"\"; PreReleased *bool \"json:\\\"pre_released\\\"\" }", "") {
        bundle_id: String => "bundle_id";
        vendor_identifier: String => "vendor_identifier";
        scoped_player_id: String => "scoped_player_id";
        score: String => "score";
        context: String => "context";
        challenge_ids: GoSlice<String> => "challenge_ids";
        submitted_date: String => "submitted_date";
        pre_released: Option<bool> => "pre_released";
    }
    }
    todo!("port handleSubmitGameCenterLeaderboardEntry from internal/asc/tools/gamecenter_submissions.go")
}

