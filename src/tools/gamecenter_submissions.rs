//! Game Center player submission tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
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

/// Handles `submit_game_center_player_achievement`.
async fn handle_submit_game_center_player_achievement(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { BundleID string \"json:\\\"bundle_id\\\"\"; VendorIdentifier string \"json:\\\"vendor_identifier\\\"\"; ScopedPlayerID string \"json:\\\"scoped_player_id\\\"\"; PercentageAchieved *int \"json:\\\"percentage_achieved\\\"\"; ChallengeIDs []string \"json:\\\"challenge_ids\\\"\"; SubmittedDate string \"json:\\\"submitted_date\\\"\"; PreReleased *bool \"json:\\\"pre_released\\\"\" }", "") {
        bundle_id: String => "bundle_id";
        vendor_identifier: String => "vendor_identifier";
        scoped_player_id: String => "scoped_player_id";
        percentage_achieved: Option<i64> => "percentage_achieved";
        challenge_ids: JsonSlice<String> => "challenge_ids";
        submitted_date: String => "submitted_date";
        pre_released: Option<bool> => "pre_released";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.bundle_id.is_empty() {
        return Ok(error_result("bundle_id is required"));
    }
    if params.vendor_identifier.is_empty() {
        return Ok(error_result("vendor_identifier is required"));
    }
    if params.scoped_player_id.is_empty() {
        return Ok(error_result("scoped_player_id is required"));
    }
    let Some(percentage_achieved) = params.percentage_achieved else {
        return Ok(error_result("percentage_achieved is required"));
    };
    if !(0..=100).contains(&percentage_achieved) {
        return Ok(error_result("percentage_achieved must be between 0 and 100"));
    }

    let req = GameCenterPlayerAchievementSubmissionCreateRequest {
        data: GameCenterPlayerAchievementSubmissionCreateData {
            type_: "gameCenterPlayerAchievementSubmissions".into(),
            attributes: GameCenterPlayerAchievementSubmissionCreateAttributes {
                bundle_id: params.bundle_id.clone(),
                vendor_identifier: params.vendor_identifier.clone(),
                scoped_player_id: params.scoped_player_id.clone(),
                percentage_achieved,
                challenge_ids: params.challenge_ids.clone(),
                submitted_date: params.submitted_date.clone(),
                pre_released: params.pre_released,
            },
        },
    };

    let resp = match c.create_game_center_player_achievement_submission(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to submit player achievement: {e}"))),
    };

    Ok(data_result(&format_game_center_player_achievement_submission(&resp.data), &resp.data))
}

/// Handles `submit_game_center_leaderboard_entry`.
async fn handle_submit_game_center_leaderboard_entry(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { BundleID string \"json:\\\"bundle_id\\\"\"; VendorIdentifier string \"json:\\\"vendor_identifier\\\"\"; ScopedPlayerID string \"json:\\\"scoped_player_id\\\"\"; Score string \"json:\\\"score\\\"\"; Context string \"json:\\\"context\\\"\"; ChallengeIDs []string \"json:\\\"challenge_ids\\\"\"; SubmittedDate string \"json:\\\"submitted_date\\\"\"; PreReleased *bool \"json:\\\"pre_released\\\"\" }", "") {
        bundle_id: String => "bundle_id";
        vendor_identifier: String => "vendor_identifier";
        scoped_player_id: String => "scoped_player_id";
        score: String => "score";
        context: String => "context";
        challenge_ids: JsonSlice<String> => "challenge_ids";
        submitted_date: String => "submitted_date";
        pre_released: Option<bool> => "pre_released";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.bundle_id.is_empty() {
        return Ok(error_result("bundle_id is required"));
    }
    if params.vendor_identifier.is_empty() {
        return Ok(error_result("vendor_identifier is required"));
    }
    if params.scoped_player_id.is_empty() {
        return Ok(error_result("scoped_player_id is required"));
    }
    if params.score.is_empty() {
        return Ok(error_result("score is required"));
    }

    let req = GameCenterLeaderboardEntrySubmissionCreateRequest {
        data: GameCenterLeaderboardEntrySubmissionCreateData {
            type_: "gameCenterLeaderboardEntrySubmissions".into(),
            attributes: GameCenterLeaderboardEntrySubmissionCreateAttributes {
                bundle_id: params.bundle_id.clone(),
                vendor_identifier: params.vendor_identifier.clone(),
                scoped_player_id: params.scoped_player_id.clone(),
                score: params.score.clone(),
                context: params.context.clone(),
                challenge_ids: params.challenge_ids.clone(),
                submitted_date: params.submitted_date.clone(),
                pre_released: params.pre_released,
            },
        },
    };

    let resp = match c.create_game_center_leaderboard_entry_submission(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to submit leaderboard entry: {e}"))),
    };

    Ok(data_result(&format_game_center_leaderboard_entry_submission(&resp.data), &resp.data))
}

fn format_game_center_player_achievement_submission(submission: &GameCenterPlayerAchievementSubmission) -> String {
    let a = &submission.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "Submitted achievement progress (ID: {})", submission.id);
    let _ = writeln!(sb, "Vendor ID: {}", a.vendor_identifier);
    let _ = writeln!(sb, "Player: {}", a.scoped_player_id);
    let _ = writeln!(sb, "Percentage Achieved: {}", a.percentage_achieved);
    let _ = writeln!(sb, "Pre-Released: {}", a.pre_released);
    if !a.submitted_date.is_empty() {
        let _ = writeln!(sb, "Submitted: {}", a.submitted_date);
    }
    if !a.challenge_ids.is_empty() {
        let _ = writeln!(sb, "Challenges: {}", a.challenge_ids.join(", "));
    }
    sb
}

fn format_game_center_leaderboard_entry_submission(submission: &GameCenterLeaderboardEntrySubmission) -> String {
    let a = &submission.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "Submitted leaderboard entry (ID: {})", submission.id);
    let _ = writeln!(sb, "Vendor ID: {}", a.vendor_identifier);
    let _ = writeln!(sb, "Player: {}", a.scoped_player_id);
    let _ = writeln!(sb, "Score: {}", a.score);
    if !a.context.is_empty() {
        let _ = writeln!(sb, "Context: {}", a.context);
    }
    let _ = writeln!(sb, "Pre-Released: {}", a.pre_released);
    if !a.submitted_date.is_empty() {
        let _ = writeln!(sb, "Submitted: {}", a.submitted_date);
    }
    if !a.challenge_ids.is_empty() {
        let _ = writeln!(sb, "Challenges: {}", a.challenge_ids.join(", "));
    }
    sb
}
