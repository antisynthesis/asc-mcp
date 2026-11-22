// App Store Connect API endpoints for the "gamecenter" area.
//
// Signatures were generated from the Go declarations; bodies are ported by
// hand from the Go file named in each method's todo!/comment and must
// preserve the exact request path, query and body.

#[allow(unused_imports)]
use crate::api::{Client, Error, ListOptions, MAX_UPLOAD_SIZE, Result, decode, model::*};
#[allow(unused_imports)]
use crate::go::{
    GoBytes, GoMap, GoSlice, GoTime, RawMessage, json,
    url::{self, Values},
};
#[allow(unused_imports)]
use serde_json::Value;

impl Client {
    /// CreateGameCenterAchievement creates a new achievement via the v2 Game Center API.
    pub async fn create_game_center_achievement(
        &self,
        req: &GameCenterAchievementCreateRequest,
    ) -> Result<GameCenterAchievementResponse> {
        let data = self.post("/v2/gameCenterAchievements", Some(req)).await?;
        decode(&data)
    }

    /// CreateGameCenterLeaderboard creates a new leaderboard via the v2 Game Center API.
    pub async fn create_game_center_leaderboard(
        &self,
        req: &GameCenterLeaderboardCreateRequest,
    ) -> Result<GameCenterLeaderboardResponse> {
        let data = self.post("/v2/gameCenterLeaderboards", Some(req)).await?;
        decode(&data)
    }

    /// DeleteGameCenterAchievement deletes an achievement.
    pub async fn delete_game_center_achievement(&self, achievement_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v2/gameCenterAchievements/{}",
            url::path_escape(achievement_id)
        ))
        .await
    }

    /// DeleteGameCenterLeaderboard deletes a leaderboard.
    pub async fn delete_game_center_leaderboard(&self, leaderboard_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v2/gameCenterLeaderboards/{}",
            url::path_escape(leaderboard_id)
        ))
        .await
    }

    /// GetGameCenterAchievement returns a single achievement.
    pub async fn get_game_center_achievement(
        &self,
        achievement_id: &str,
    ) -> Result<GameCenterAchievementResponse> {
        let data = self
            .get(
                &format!(
                    "/v2/gameCenterAchievements/{}",
                    url::path_escape(achievement_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetGameCenterDetail returns game center details for an app.
    pub async fn get_game_center_detail(&self, app_id: &str) -> Result<GameCenterDetailResponse> {
        let data = self
            .get(
                &format!("/v1/apps/{}/gameCenterDetail", url::path_escape(app_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetGameCenterLeaderboard returns a single leaderboard.
    pub async fn get_game_center_leaderboard(
        &self,
        leaderboard_id: &str,
    ) -> Result<GameCenterLeaderboardResponse> {
        let data = self
            .get(
                &format!(
                    "/v2/gameCenterLeaderboards/{}",
                    url::path_escape(leaderboard_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// ListGameCenterAchievements returns achievements for a game center detail via the v2 Game Center API.
    pub async fn list_game_center_achievements(
        &self,
        game_center_detail_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<GameCenterAchievementsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/gameCenterDetails/{}/gameCenterAchievementsV2",
                    url::path_escape(game_center_detail_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListGameCenterLeaderboards returns leaderboards for a game center detail via the v2 Game Center API.
    pub async fn list_game_center_leaderboards(
        &self,
        game_center_detail_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<GameCenterLeaderboardsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/gameCenterDetails/{}/gameCenterLeaderboardsV2",
                    url::path_escape(game_center_detail_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// UpdateGameCenterAchievement updates an achievement.
    pub async fn update_game_center_achievement(
        &self,
        achievement_id: &str,
        req: &GameCenterAchievementUpdateRequest,
    ) -> Result<GameCenterAchievementResponse> {
        let data = self
            .patch(
                &format!(
                    "/v2/gameCenterAchievements/{}",
                    url::path_escape(achievement_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateGameCenterLeaderboard updates a leaderboard.
    pub async fn update_game_center_leaderboard(
        &self,
        leaderboard_id: &str,
        req: &GameCenterLeaderboardUpdateRequest,
    ) -> Result<GameCenterLeaderboardResponse> {
        let data = self
            .patch(
                &format!(
                    "/v2/gameCenterLeaderboards/{}",
                    url::path_escape(leaderboard_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// AddGameCenterLeaderboardSetMembers adds leaderboards to a leaderboard set.
    pub async fn add_game_center_leaderboard_set_members(
        &self,
        leaderboard_set_id: &str,
        req: &RelationshipDataList,
    ) -> Result<()> {
        self.post(
            &format!(
                "/v2/gameCenterLeaderboardSets/{}/relationships/gameCenterLeaderboards",
                url::path_escape(leaderboard_set_id)
            ),
            Some(req),
        )
        .await?;
        Ok(())
    }

    /// CreateGameCenterActivity creates an activity along with its initial version.
    pub async fn create_game_center_activity(
        &self,
        req: &GameCenterActivityCreateRequest,
    ) -> Result<GameCenterActivityResponse> {
        let data = self.post("/v1/gameCenterActivities", Some(req)).await?;
        decode(&data)
    }

    /// CreateGameCenterActivityLocalization creates an activity localization.
    pub async fn create_game_center_activity_localization(
        &self,
        req: &GameCenterActivityLocalizationCreateRequest,
    ) -> Result<GameCenterActivityLocalizationResponse> {
        let data = self
            .post("/v1/gameCenterActivityLocalizations", Some(req))
            .await?;
        decode(&data)
    }

    /// CreateGameCenterActivityVersion opens a new editable version of an activity.
    pub async fn create_game_center_activity_version(
        &self,
        req: &GameCenterActivityVersionCreateRequest,
    ) -> Result<GameCenterActivityVersionResponse> {
        let data = self
            .post("/v1/gameCenterActivityVersions", Some(req))
            .await?;
        decode(&data)
    }

    /// CreateGameCenterChallenge creates a challenge along with its initial version.
    pub async fn create_game_center_challenge(
        &self,
        req: &GameCenterChallengeCreateRequest,
    ) -> Result<GameCenterChallengeResponse> {
        let data = self.post("/v1/gameCenterChallenges", Some(req)).await?;
        decode(&data)
    }

    /// CreateGameCenterChallengeLocalization creates a challenge localization.
    pub async fn create_game_center_challenge_localization(
        &self,
        req: &GameCenterChallengeLocalizationCreateRequest,
    ) -> Result<GameCenterChallengeLocalizationResponse> {
        let data = self
            .post("/v1/gameCenterChallengeLocalizations", Some(req))
            .await?;
        decode(&data)
    }

    /// CreateGameCenterChallengeVersion opens a new editable version of a challenge.
    pub async fn create_game_center_challenge_version(
        &self,
        req: &GameCenterChallengeVersionCreateRequest,
    ) -> Result<GameCenterChallengeVersionResponse> {
        let data = self
            .post("/v1/gameCenterChallengeVersions", Some(req))
            .await?;
        decode(&data)
    }

    /// CreateGameCenterLeaderboardEntrySubmission submits a leaderboard score for
    /// a player on the game server's behalf.
    pub async fn create_game_center_leaderboard_entry_submission(
        &self,
        req: &GameCenterLeaderboardEntrySubmissionCreateRequest,
    ) -> Result<GameCenterLeaderboardEntrySubmissionResponse> {
        let data = self
            .post("/v1/gameCenterLeaderboardEntrySubmissions", Some(req))
            .await?;
        decode(&data)
    }

    /// CreateGameCenterLeaderboardSet creates a leaderboard set via the v2 Game Center API.
    pub async fn create_game_center_leaderboard_set(
        &self,
        req: &GameCenterLeaderboardSetCreateRequest,
    ) -> Result<GameCenterLeaderboardSetResponse> {
        let data = self
            .post("/v2/gameCenterLeaderboardSets", Some(req))
            .await?;
        decode(&data)
    }

    /// CreateGameCenterLeaderboardSetLocalization creates a leaderboard set localization.
    pub async fn create_game_center_leaderboard_set_localization(
        &self,
        req: &GameCenterLeaderboardSetLocalizationCreateRequest,
    ) -> Result<GameCenterLeaderboardSetLocalizationResponse> {
        let data = self
            .post("/v2/gameCenterLeaderboardSetLocalizations", Some(req))
            .await?;
        decode(&data)
    }

    /// CreateGameCenterLeaderboardSetMemberLocalization names a leaderboard
    /// within a leaderboard set for one locale.
    pub async fn create_game_center_leaderboard_set_member_localization(
        &self,
        req: &GameCenterLeaderboardSetMemberLocalizationCreateRequest,
    ) -> Result<GameCenterLeaderboardSetMemberLocalizationResponse> {
        let data = self
            .post("/v1/gameCenterLeaderboardSetMemberLocalizations", Some(req))
            .await?;
        decode(&data)
    }

    /// CreateGameCenterLeaderboardSetVersion opens a new editable version of a leaderboard set.
    pub async fn create_game_center_leaderboard_set_version(
        &self,
        req: &GameCenterLeaderboardSetVersionCreateRequest,
    ) -> Result<GameCenterLeaderboardSetVersionResponse> {
        let data = self
            .post("/v2/gameCenterLeaderboardSetVersions", Some(req))
            .await?;
        decode(&data)
    }

    /// CreateGameCenterPlayerAchievementSubmission submits achievement progress
    /// for a player on the game server's behalf.
    pub async fn create_game_center_player_achievement_submission(
        &self,
        req: &GameCenterPlayerAchievementSubmissionCreateRequest,
    ) -> Result<GameCenterPlayerAchievementSubmissionResponse> {
        let data = self
            .post("/v1/gameCenterPlayerAchievementSubmissions", Some(req))
            .await?;
        decode(&data)
    }

    /// DeleteGameCenterActivity deletes an activity.
    pub async fn delete_game_center_activity(&self, activity_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/gameCenterActivities/{}",
            url::path_escape(activity_id)
        ))
        .await
    }

    /// DeleteGameCenterActivityLocalization deletes an activity localization.
    pub async fn delete_game_center_activity_localization(
        &self,
        localization_id: &str,
    ) -> Result<()> {
        self.delete(&format!(
            "/v1/gameCenterActivityLocalizations/{}",
            url::path_escape(localization_id)
        ))
        .await
    }

    /// DeleteGameCenterChallenge deletes a challenge.
    pub async fn delete_game_center_challenge(&self, challenge_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/gameCenterChallenges/{}",
            url::path_escape(challenge_id)
        ))
        .await
    }

    /// DeleteGameCenterChallengeLocalization deletes a challenge localization.
    pub async fn delete_game_center_challenge_localization(
        &self,
        localization_id: &str,
    ) -> Result<()> {
        self.delete(&format!(
            "/v1/gameCenterChallengeLocalizations/{}",
            url::path_escape(localization_id)
        ))
        .await
    }

    /// DeleteGameCenterLeaderboardSet deletes a leaderboard set.
    pub async fn delete_game_center_leaderboard_set(&self, leaderboard_set_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v2/gameCenterLeaderboardSets/{}",
            url::path_escape(leaderboard_set_id)
        ))
        .await
    }

    /// DeleteGameCenterLeaderboardSetLocalization deletes a leaderboard set localization.
    pub async fn delete_game_center_leaderboard_set_localization(
        &self,
        localization_id: &str,
    ) -> Result<()> {
        self.delete(&format!(
            "/v2/gameCenterLeaderboardSetLocalizations/{}",
            url::path_escape(localization_id)
        ))
        .await
    }

    /// DeleteGameCenterLeaderboardSetMemberLocalization deletes a member localization.
    pub async fn delete_game_center_leaderboard_set_member_localization(
        &self,
        localization_id: &str,
    ) -> Result<()> {
        self.delete(&format!(
            "/v1/gameCenterLeaderboardSetMemberLocalizations/{}",
            url::path_escape(localization_id)
        ))
        .await
    }

    /// GetGameCenterActivity returns a single activity.
    pub async fn get_game_center_activity(
        &self,
        activity_id: &str,
    ) -> Result<GameCenterActivityResponse> {
        let data = self
            .get(
                &format!("/v1/gameCenterActivities/{}", url::path_escape(activity_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetGameCenterActivityVersion returns a single activity version.
    pub async fn get_game_center_activity_version(
        &self,
        version_id: &str,
    ) -> Result<GameCenterActivityVersionResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/gameCenterActivityVersions/{}",
                    url::path_escape(version_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetGameCenterChallenge returns a single challenge.
    pub async fn get_game_center_challenge(
        &self,
        challenge_id: &str,
    ) -> Result<GameCenterChallengeResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/gameCenterChallenges/{}",
                    url::path_escape(challenge_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetGameCenterChallengeVersion returns a single challenge version.
    pub async fn get_game_center_challenge_version(
        &self,
        version_id: &str,
    ) -> Result<GameCenterChallengeVersionResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/gameCenterChallengeVersions/{}",
                    url::path_escape(version_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetGameCenterLeaderboardSet returns a single leaderboard set.
    pub async fn get_game_center_leaderboard_set(
        &self,
        leaderboard_set_id: &str,
    ) -> Result<GameCenterLeaderboardSetResponse> {
        let data = self
            .get(
                &format!(
                    "/v2/gameCenterLeaderboardSets/{}",
                    url::path_escape(leaderboard_set_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetGameCenterLeaderboardSetVersion returns a single leaderboard set version.
    pub async fn get_game_center_leaderboard_set_version(
        &self,
        version_id: &str,
    ) -> Result<GameCenterLeaderboardSetVersionResponse> {
        let data = self
            .get(
                &format!(
                    "/v2/gameCenterLeaderboardSetVersions/{}",
                    url::path_escape(version_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// ListGameCenterActivities returns the activities for a game center detail.
    pub async fn list_game_center_activities(
        &self,
        game_center_detail_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<GameCenterActivitiesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/gameCenterDetails/{}/gameCenterActivities",
                    url::path_escape(game_center_detail_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListGameCenterActivityLocalizations returns the localizations of an activity version.
    pub async fn list_game_center_activity_localizations(
        &self,
        version_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<GameCenterActivityLocalizationsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/gameCenterActivityVersions/{}/localizations",
                    url::path_escape(version_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListGameCenterActivityVersions returns the versions of an activity.
    pub async fn list_game_center_activity_versions(
        &self,
        activity_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<GameCenterActivityVersionsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/gameCenterActivities/{}/versions",
                    url::path_escape(activity_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListGameCenterChallengeLocalizations returns the localizations of a challenge version.
    pub async fn list_game_center_challenge_localizations(
        &self,
        version_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<GameCenterChallengeLocalizationsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/gameCenterChallengeVersions/{}/localizations",
                    url::path_escape(version_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListGameCenterChallengeVersions returns the versions of a challenge.
    pub async fn list_game_center_challenge_versions(
        &self,
        challenge_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<GameCenterChallengeVersionsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/gameCenterChallenges/{}/versions",
                    url::path_escape(challenge_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListGameCenterChallenges returns the challenges for a game center detail.
    pub async fn list_game_center_challenges(
        &self,
        game_center_detail_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<GameCenterChallengesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/gameCenterDetails/{}/gameCenterChallenges",
                    url::path_escape(game_center_detail_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListGameCenterLeaderboardSetLocalizations returns the localizations of a
    /// leaderboard set version.
    pub async fn list_game_center_leaderboard_set_localizations(
        &self,
        version_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<GameCenterLeaderboardSetLocalizationsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v2/gameCenterLeaderboardSetVersions/{}/localizations",
                    url::path_escape(version_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListGameCenterLeaderboardSetMemberLocalizations returns the per-set names
    /// of leaderboards. The API requires at least one of the
    /// filter[gameCenterLeaderboardSet] / filter[gameCenterLeaderboard] filters,
    /// which the caller supplies through opts.
    pub async fn list_game_center_leaderboard_set_member_localizations(
        &self,
        opts: Option<&ListOptions>,
    ) -> Result<GameCenterLeaderboardSetMemberLocalizationsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                "/v1/gameCenterLeaderboardSetMemberLocalizations",
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListGameCenterLeaderboardSetMembers returns the leaderboards that belong
    /// to a leaderboard set.
    pub async fn list_game_center_leaderboard_set_members(
        &self,
        leaderboard_set_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<GameCenterLeaderboardsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v2/gameCenterLeaderboardSets/{}/gameCenterLeaderboards",
                    url::path_escape(leaderboard_set_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListGameCenterLeaderboardSetVersions returns the versions of a leaderboard set.
    pub async fn list_game_center_leaderboard_set_versions(
        &self,
        leaderboard_set_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<GameCenterLeaderboardSetVersionsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v2/gameCenterLeaderboardSets/{}/versions",
                    url::path_escape(leaderboard_set_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListGameCenterLeaderboardSets returns the leaderboard sets for a game
    /// center detail via the v2 Game Center API.
    pub async fn list_game_center_leaderboard_sets(
        &self,
        game_center_detail_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<GameCenterLeaderboardSetsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/gameCenterDetails/{}/gameCenterLeaderboardSetsV2",
                    url::path_escape(game_center_detail_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// RemoveGameCenterLeaderboardSetMembers removes leaderboards from a
    /// leaderboard set. The linkages travel in the DELETE body.
    pub async fn remove_game_center_leaderboard_set_members(
        &self,
        leaderboard_set_id: &str,
        req: &RelationshipDataList,
    ) -> Result<()> {
        self.delete_with_body(
            &format!(
                "/v2/gameCenterLeaderboardSets/{}/relationships/gameCenterLeaderboards",
                url::path_escape(leaderboard_set_id)
            ),
            Some(req),
        )
        .await
    }

    /// UpdateGameCenterActivity updates an activity.
    pub async fn update_game_center_activity(
        &self,
        activity_id: &str,
        req: &GameCenterActivityUpdateRequest,
    ) -> Result<GameCenterActivityResponse> {
        let data = self
            .patch(
                &format!("/v1/gameCenterActivities/{}", url::path_escape(activity_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateGameCenterActivityLocalization updates an activity localization.
    pub async fn update_game_center_activity_localization(
        &self,
        localization_id: &str,
        req: &GameCenterActivityLocalizationUpdateRequest,
    ) -> Result<GameCenterActivityLocalizationResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/gameCenterActivityLocalizations/{}",
                    url::path_escape(localization_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateGameCenterActivityVersion updates an activity version's fallback URL.
    pub async fn update_game_center_activity_version(
        &self,
        version_id: &str,
        req: &GameCenterActivityVersionUpdateRequest,
    ) -> Result<GameCenterActivityVersionResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/gameCenterActivityVersions/{}",
                    url::path_escape(version_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateGameCenterChallenge updates a challenge.
    pub async fn update_game_center_challenge(
        &self,
        challenge_id: &str,
        req: &GameCenterChallengeUpdateRequest,
    ) -> Result<GameCenterChallengeResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/gameCenterChallenges/{}",
                    url::path_escape(challenge_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateGameCenterChallengeLocalization updates a challenge localization.
    pub async fn update_game_center_challenge_localization(
        &self,
        localization_id: &str,
        req: &GameCenterChallengeLocalizationUpdateRequest,
    ) -> Result<GameCenterChallengeLocalizationResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/gameCenterChallengeLocalizations/{}",
                    url::path_escape(localization_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateGameCenterLeaderboardSet updates a leaderboard set.
    pub async fn update_game_center_leaderboard_set(
        &self,
        leaderboard_set_id: &str,
        req: &GameCenterLeaderboardSetUpdateRequest,
    ) -> Result<GameCenterLeaderboardSetResponse> {
        let data = self
            .patch(
                &format!(
                    "/v2/gameCenterLeaderboardSets/{}",
                    url::path_escape(leaderboard_set_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateGameCenterLeaderboardSetLocalization updates a leaderboard set localization.
    pub async fn update_game_center_leaderboard_set_localization(
        &self,
        localization_id: &str,
        req: &GameCenterLeaderboardSetLocalizationUpdateRequest,
    ) -> Result<GameCenterLeaderboardSetLocalizationResponse> {
        let data = self
            .patch(
                &format!(
                    "/v2/gameCenterLeaderboardSetLocalizations/{}",
                    url::path_escape(localization_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateGameCenterLeaderboardSetMemberLocalization updates a member localization.
    pub async fn update_game_center_leaderboard_set_member_localization(
        &self,
        localization_id: &str,
        req: &GameCenterLeaderboardSetMemberLocalizationUpdateRequest,
    ) -> Result<GameCenterLeaderboardSetMemberLocalizationResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/gameCenterLeaderboardSetMemberLocalizations/{}",
                    url::path_escape(localization_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UploadGameCenterActivityImage uploads an activity image. Passing a
    /// localization ID sets that locale's image; passing a version ID instead
    /// sets the version's default image.
    pub async fn upload_game_center_activity_image(
        &self,
        localization_id: &str,
        version_id: &str,
        file_name: &str,
        body: &[u8],
    ) -> Result<GameCenterImageResponse> {
        let mut rels: GoMap<Option<RelationshipData>> = GoMap::from_map(Default::default());
        if !localization_id.is_empty() {
            rels.insert(
                "localization",
                Some(RelationshipData {
                    data: ResourceIdentifier {
                        type_: "gameCenterActivityLocalizations".into(),
                        id: localization_id.to_string(),
                    },
                }),
            );
        }
        if !version_id.is_empty() {
            rels.insert(
                "version",
                Some(RelationshipData {
                    data: ResourceIdentifier {
                        type_: "gameCenterActivityVersions".into(),
                        id: version_id.to_string(),
                    },
                }),
            );
        }
        self.upload_game_center_image(
            "/v1/gameCenterActivityImages",
            "gameCenterActivityImages",
            rels,
            file_name,
            body,
        )
        .await
    }

    /// UploadGameCenterChallengeImage uploads a challenge image. Passing a
    /// localization ID sets that locale's image; passing a version ID instead
    /// sets the version's default image.
    pub async fn upload_game_center_challenge_image(
        &self,
        localization_id: &str,
        version_id: &str,
        file_name: &str,
        body: &[u8],
    ) -> Result<GameCenterImageResponse> {
        let mut rels: GoMap<Option<RelationshipData>> = GoMap::from_map(Default::default());
        if !localization_id.is_empty() {
            rels.insert(
                "localization",
                Some(RelationshipData {
                    data: ResourceIdentifier {
                        type_: "gameCenterChallengeLocalizations".into(),
                        id: localization_id.to_string(),
                    },
                }),
            );
        }
        if !version_id.is_empty() {
            rels.insert(
                "version",
                Some(RelationshipData {
                    data: ResourceIdentifier {
                        type_: "gameCenterChallengeVersions".into(),
                        id: version_id.to_string(),
                    },
                }),
            );
        }
        self.upload_game_center_image(
            "/v1/gameCenterChallengeImages",
            "gameCenterChallengeImages",
            rels,
            file_name,
            body,
        )
        .await
    }

    /// UploadGameCenterLeaderboardSetImage uploads the image shown for a
    /// leaderboard set localization.
    pub async fn upload_game_center_leaderboard_set_image(
        &self,
        localization_id: &str,
        file_name: &str,
        body: &[u8],
    ) -> Result<GameCenterImageResponse> {
        let mut rels: GoMap<Option<RelationshipData>> = GoMap::from_map(Default::default());
        rels.insert(
            "localization",
            Some(RelationshipData {
                data: ResourceIdentifier {
                    type_: "gameCenterLeaderboardSetLocalizations".into(),
                    id: localization_id.to_string(),
                },
            }),
        );
        self.upload_game_center_image(
            "/v2/gameCenterLeaderboardSetImages",
            "gameCenterLeaderboardSetImages",
            rels,
            file_name,
            body,
        )
        .await
    }

    async fn create_game_center_image(
        &self,
        path: &str,
        req: &GameCenterImageCreateRequest,
    ) -> Result<GameCenterImageResponse> {
        let data = self.post(path, Some(req)).await?;
        decode(&data)
    }

    async fn commit_game_center_image(
        &self,
        path: &str,
        req: &GameCenterImageUpdateRequest,
    ) -> Result<GameCenterImageResponse> {
        let data = self.patch(path, Some(req)).await?;
        decode(&data)
    }

    async fn upload_game_center_image(
        &self,
        collection_path: &str,
        resource_type: &str,
        relationships: GoMap<Option<RelationshipData>>,
        file_name: &str,
        body: &[u8],
    ) -> Result<GameCenterImageResponse> {
        if file_name.is_empty() {
            return Err(Error::msg("file_name is required"));
        }
        if body.is_empty() {
            return Err(Error::msg("file body is empty"));
        }
        if body.len() > MAX_UPLOAD_SIZE {
            return Err(Error::msg(format!(
                "file body exceeds max size of {MAX_UPLOAD_SIZE} bytes"
            )));
        }

        let reservation = self
            .create_game_center_image(
                collection_path,
                &GameCenterImageCreateRequest {
                    data: GameCenterImageCreateData {
                        type_: resource_type.to_string(),
                        attributes: GameCenterImageCreateAttrs {
                            file_name: file_name.to_string(),
                            file_size: body.len() as i64,
                        },
                        relationships,
                    },
                },
            )
            .await
            .map_err(|e| e.wrap("reservation"))?;

        self.perform_upload_operations(&reservation.data.attributes.upload_operations, body)
            .await
            .map_err(|e| e.wrap("upload"))?;

        // Game Center images commit with uploaded=true only; unlike App Store
        // screenshots they carry no source file checksum.
        self.commit_game_center_image(
            &format!(
                "{}/{}",
                collection_path,
                url::path_escape(&reservation.data.id)
            ),
            &GameCenterImageUpdateRequest {
                data: GameCenterImageUpdateData {
                    type_: resource_type.to_string(),
                    id: reservation.data.id.clone(),
                    attributes: GameCenterImageUpdateAttrs {
                        uploaded: Some(true),
                    },
                },
            },
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::api::testutil::{MockApi, MockResponse, RecordedRequest};

    fn rid(t: &str, id: &str) -> ResourceIdentifier {
        ResourceIdentifier {
            type_: t.into(),
            id: id.into(),
        }
    }

    fn rel(t: &str, id: &str) -> RelationshipData {
        RelationshipData { data: rid(t, id) }
    }

    #[tokio::test]
    async fn list_game_center_leaderboard_sets() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                200,
                r#"{"data":[{"type":"gameCenterLeaderboardSets","id":"set-1","attributes":{"referenceName":"Season Ladders","vendorIdentifier":"com.example.sets.season"}}],"links":{}}"#,
            )
        })
        .await;

        let opts = ListOptions {
            limit: 25,
            ..Default::default()
        };
        let resp = mock
            .client()
            .list_game_center_leaderboard_sets("gc-1", Some(&opts))
            .await
            .expect("unexpected error");

        let reqs = mock.requests();
        assert_eq!(
            reqs[0].path,
            "/v1/gameCenterDetails/gc-1/gameCenterLeaderboardSetsV2"
        );
        assert_eq!(reqs[0].method, "GET");
        assert_eq!(reqs[0].query_param("limit").as_deref(), Some("25"));

        assert_eq!(resp.data.len(), 1, "expected 1 leaderboard set");
        assert_eq!(resp.data[0].attributes.reference_name, "Season Ladders");
    }

    #[tokio::test]
    async fn create_game_center_leaderboard_set() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                201,
                r#"{"data":{"type":"gameCenterLeaderboardSets","id":"set-1","attributes":{}}}"#,
            )
        })
        .await;

        let req = GameCenterLeaderboardSetCreateRequest {
            data: GameCenterLeaderboardSetCreateData {
                type_: "gameCenterLeaderboardSets".into(),
                attributes: GameCenterLeaderboardSetCreateAttributes {
                    reference_name: "Season Ladders".into(),
                    vendor_identifier: "com.example.sets.season".into(),
                },
                relationships: GameCenterLeaderboardSetCreateRelationships {
                    game_center_detail: Some(rel("gameCenterDetails", "gc-1")),
                    versions: RelationshipDataList {
                        data: GoSlice::from(vec![rid(
                            "gameCenterLeaderboardSetVersions",
                            "${new-version}",
                        )]),
                    },
                    ..Default::default()
                },
            },
            included: GoSlice::from(vec![GameCenterVersionInlineCreate {
                type_: "gameCenterLeaderboardSetVersions".into(),
                id: "${new-version}".into(),
            }]),
        };

        let resp = mock
            .client()
            .create_game_center_leaderboard_set(&req)
            .await
            .expect("unexpected error");
        assert_eq!(resp.data.id, "set-1");

        let r = &mock.requests()[0];
        assert_eq!(r.path, "/v2/gameCenterLeaderboardSets");
        assert_eq!(r.method, "POST");
        let body = r.json();
        assert_eq!(body["data"]["type"], "gameCenterLeaderboardSets");
        assert_eq!(
            body["data"]["relationships"]["gameCenterDetail"]["data"]["id"],
            "gc-1"
        );
        let versions = body["data"]["relationships"]["versions"]["data"]
            .as_array()
            .unwrap();
        assert_eq!(versions.len(), 1, "expected 1 inline version linkage");
        let included = body["included"].as_array().unwrap();
        assert_eq!(included.len(), 1);
        assert_eq!(included[0]["type"], "gameCenterLeaderboardSetVersions");
        assert_eq!(
            included[0]["id"], versions[0]["id"],
            "included version ID must match the versions linkage ID"
        );
    }

    #[tokio::test]
    async fn game_center_leaderboard_set_membership() {
        for (name, want_method) in [("add", "POST"), ("remove", "DELETE")] {
            let mock = MockApi::start(|_| MockResponse::empty(204)).await;
            let linkages = RelationshipDataList {
                data: GoSlice::from(vec![
                    rid("gameCenterLeaderboards", "lb-1"),
                    rid("gameCenterLeaderboards", "lb-2"),
                ]),
            };
            let c = mock.client();
            let res = if name == "add" {
                c.add_game_center_leaderboard_set_members("set-1", &linkages)
                    .await
            } else {
                c.remove_game_center_leaderboard_set_members("set-1", &linkages)
                    .await
            };
            res.unwrap_or_else(|e| panic!("{name}: unexpected error: {e}"));

            let r = &mock.requests()[0];
            assert_eq!(
                r.path, "/v2/gameCenterLeaderboardSets/set-1/relationships/gameCenterLeaderboards",
                "{name}"
            );
            assert_eq!(r.method, want_method, "{name}");
            let body = r.json();
            let data = body["data"].as_array().unwrap();
            assert_eq!(data.len(), 2, "{name}: expected 2 linkages");
            assert_eq!(data[0]["type"], "gameCenterLeaderboards", "{name}");
        }
    }

    #[tokio::test]
    async fn create_game_center_leaderboard_set_localization() {
        let mock = MockApi::start(|_| {
            MockResponse::json(201, r#"{"data":{"type":"gameCenterLeaderboardSetLocalizations","id":"loc-1","attributes":{}}}"#)
        })
        .await;

        let req = GameCenterLeaderboardSetLocalizationCreateRequest {
            data: GameCenterLeaderboardSetLocalizationCreateData {
                type_: "gameCenterLeaderboardSetLocalizations".into(),
                attributes: GameCenterLeaderboardSetLocalizationCreateAttributes {
                    locale: "en-US".into(),
                    name: "Season Ladders".into(),
                },
                relationships: GameCenterLeaderboardSetLocalizationCreateRelationships {
                    version: rel("gameCenterLeaderboardSetVersions", "ver-1"),
                },
            },
        };

        let resp = mock
            .client()
            .create_game_center_leaderboard_set_localization(&req)
            .await
            .expect("unexpected error");
        assert_eq!(resp.data.id, "loc-1");

        let r = &mock.requests()[0];
        assert_eq!(r.path, "/v2/gameCenterLeaderboardSetLocalizations");
        assert_eq!(r.method, "POST");
        let body = r.json();
        assert_eq!(body["data"]["attributes"]["locale"], "en-US");
        assert_eq!(
            body["data"]["relationships"]["version"]["data"]["type"],
            "gameCenterLeaderboardSetVersions"
        );
    }

    #[tokio::test]
    async fn create_game_center_leaderboard_set_member_localization() {
        let mock = MockApi::start(|_| {
            MockResponse::json(201, r#"{"data":{"type":"gameCenterLeaderboardSetMemberLocalizations","id":"mem-1","attributes":{}}}"#)
        })
        .await;

        let req = GameCenterLeaderboardSetMemberLocalizationCreateRequest {
            data: GameCenterLeaderboardSetMemberLocalizationCreateData {
                type_: "gameCenterLeaderboardSetMemberLocalizations".into(),
                attributes: GameCenterLeaderboardSetMemberLocalizationCreateAttributes {
                    locale: "en-US".into(),
                    name: "Weekly".into(),
                },
                relationships: GameCenterLeaderboardSetMemberLocalizationCreateRelationships {
                    game_center_leaderboard_set: rel("gameCenterLeaderboardSets", "set-1"),
                    game_center_leaderboard: rel("gameCenterLeaderboards", "lb-1"),
                },
            },
        };

        let resp = mock
            .client()
            .create_game_center_leaderboard_set_member_localization(&req)
            .await
            .expect("unexpected error");
        assert_eq!(resp.data.id, "mem-1");

        let r = &mock.requests()[0];
        assert_eq!(r.path, "/v1/gameCenterLeaderboardSetMemberLocalizations");
        assert_eq!(r.method, "POST");
        let body = r.json();
        assert_eq!(
            body["data"]["relationships"]["gameCenterLeaderboardSet"]["data"]["id"],
            "set-1"
        );
        assert_eq!(
            body["data"]["relationships"]["gameCenterLeaderboard"]["data"]["id"],
            "lb-1"
        );
    }

    #[tokio::test]
    async fn create_game_center_activity() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                201,
                r#"{"data":{"type":"gameCenterActivities","id":"act-1","attributes":{}}}"#,
            )
        })
        .await;

        let req = GameCenterActivityCreateRequest {
            data: GameCenterActivityCreateData {
                type_: "gameCenterActivities".into(),
                attributes: GameCenterActivityCreateAttributes {
                    reference_name: "Co-op Raid".into(),
                    vendor_identifier: "com.example.activities.raid".into(),
                    play_style: "SYNCHRONOUS".into(),
                    minimum_players_count: Some(2),
                    ..Default::default()
                },
                relationships: GameCenterActivityCreateRelationships {
                    game_center_detail: Some(rel("gameCenterDetails", "gc-1")),
                    versions: Some(RelationshipDataList {
                        data: GoSlice::from(vec![rid(
                            "gameCenterActivityVersions",
                            "${new-version}",
                        )]),
                    }),
                    ..Default::default()
                },
            },
            included: GoSlice::from(vec![GameCenterActivityVersionInlineCreate {
                type_: "gameCenterActivityVersions".into(),
                id: "${new-version}".into(),
                attributes: Some(GameCenterActivityVersionInlineCreateAttributes {
                    fallback_url: "https://example.com/play".into(),
                }),
            }]),
        };

        let resp = mock
            .client()
            .create_game_center_activity(&req)
            .await
            .expect("unexpected error");
        assert_eq!(resp.data.id, "act-1");

        let r = &mock.requests()[0];
        assert_eq!(r.path, "/v1/gameCenterActivities");
        assert_eq!(r.method, "POST");
        let body = r.json();
        assert_eq!(body["data"]["attributes"]["playStyle"], "SYNCHRONOUS");
        assert_eq!(
            body["data"]["relationships"]["versions"]["data"]
                .as_array()
                .map(|a| a.len()),
            Some(1),
            "expected one inline version linkage"
        );
        let included = body["included"].as_array().unwrap();
        assert_eq!(included.len(), 1, "expected 1 included version");
        assert_eq!(
            included[0]["attributes"]["fallbackUrl"], "https://example.com/play",
            "expected fallbackUrl on the inline initial version"
        );
    }

    #[tokio::test]
    async fn update_game_center_activity_version() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                200,
                r#"{"data":{"type":"gameCenterActivityVersions","id":"ver-1","attributes":{}}}"#,
            )
        })
        .await;

        let req = GameCenterActivityVersionUpdateRequest {
            data: GameCenterActivityVersionUpdateData {
                type_: "gameCenterActivityVersions".into(),
                id: "ver-1".into(),
                attributes: GameCenterActivityVersionUpdateAttributes {
                    fallback_url: "https://example.com/next".into(),
                },
            },
        };

        let resp = mock
            .client()
            .update_game_center_activity_version("ver-1", &req)
            .await
            .expect("unexpected error");
        assert_eq!(resp.data.id, "ver-1");

        let r = &mock.requests()[0];
        assert_eq!(r.path, "/v1/gameCenterActivityVersions/ver-1");
        assert_eq!(r.method, "PATCH");
        assert_eq!(
            r.json()["data"]["attributes"]["fallbackUrl"],
            "https://example.com/next"
        );
    }

    #[tokio::test]
    async fn create_game_center_challenge() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                201,
                r#"{"data":{"type":"gameCenterChallenges","id":"chl-1","attributes":{}}}"#,
            )
        })
        .await;

        let req = GameCenterChallengeCreateRequest {
            data: GameCenterChallengeCreateData {
                type_: "gameCenterChallenges".into(),
                attributes: GameCenterChallengeCreateAttributes {
                    reference_name: "Weekend Sprint".into(),
                    vendor_identifier: "com.example.challenges.sprint".into(),
                    challenge_type: "LEADERBOARD".into(),
                    repeatable: Some(true),
                },
                relationships: GameCenterChallengeCreateRelationships {
                    game_center_detail: Some(rel("gameCenterDetails", "gc-1")),
                    leaderboard_v2: Some(rel("gameCenterLeaderboards", "lb-1")),
                    versions: Some(RelationshipDataList {
                        data: GoSlice::from(vec![rid(
                            "gameCenterChallengeVersions",
                            "${new-version}",
                        )]),
                    }),
                    ..Default::default()
                },
            },
            included: GoSlice::from(vec![GameCenterVersionInlineCreate {
                type_: "gameCenterChallengeVersions".into(),
                id: "${new-version}".into(),
            }]),
        };

        let resp = mock
            .client()
            .create_game_center_challenge(&req)
            .await
            .expect("unexpected error");
        assert_eq!(resp.data.id, "chl-1");

        let r = &mock.requests()[0];
        assert_eq!(r.path, "/v1/gameCenterChallenges");
        assert_eq!(r.method, "POST");
        let body = r.json();
        // allowedDurations was removed from the API in 4.1; make sure the
        // request never reintroduces it.
        assert!(
            body["data"]["attributes"].get("allowedDurations").is_none(),
            "request must not contain allowedDurations"
        );
        assert_eq!(body["data"]["attributes"]["challengeType"], "LEADERBOARD");
        assert_eq!(
            body["data"]["relationships"]["leaderboardV2"]["data"]["id"], "lb-1",
            "expected leaderboardV2 relationship with id lb-1"
        );
        let included = body["included"].as_array().unwrap();
        assert_eq!(included.len(), 1);
        assert_eq!(included[0]["type"], "gameCenterChallengeVersions");
    }

    #[tokio::test]
    async fn create_game_center_player_achievement_submission() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                201,
                r#"{"data":{"type":"gameCenterPlayerAchievementSubmissions","id":"sub-1","attributes":{"percentageAchieved":100,"preReleased":true}}}"#,
            )
        })
        .await;

        let req = GameCenterPlayerAchievementSubmissionCreateRequest {
            data: GameCenterPlayerAchievementSubmissionCreateData {
                type_: "gameCenterPlayerAchievementSubmissions".into(),
                attributes: GameCenterPlayerAchievementSubmissionCreateAttributes {
                    bundle_id: "com.example.game".into(),
                    vendor_identifier: "com.example.achievements.first".into(),
                    scoped_player_id: "player-1".into(),
                    percentage_achieved: 100,
                    pre_released: Some(true),
                    ..Default::default()
                },
            },
        };

        let resp = mock
            .client()
            .create_game_center_player_achievement_submission(&req)
            .await
            .expect("unexpected error");
        assert!(
            resp.data.attributes.pre_released,
            "expected preReleased submission in response"
        );

        let r = &mock.requests()[0];
        assert_eq!(r.path, "/v1/gameCenterPlayerAchievementSubmissions");
        assert_eq!(r.method, "POST");
        let body = r.json();
        assert_eq!(body["data"]["attributes"]["percentageAchieved"], 100);
        assert_eq!(
            body["data"]["attributes"]["preReleased"], true,
            "expected preReleased=true in request body"
        );
    }

    #[tokio::test]
    async fn create_game_center_leaderboard_entry_submission() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                201,
                r#"{"data":{"type":"gameCenterLeaderboardEntrySubmissions","id":"entry-1","attributes":{"score":"12345"}}}"#,
            )
        })
        .await;

        let req = GameCenterLeaderboardEntrySubmissionCreateRequest {
            data: GameCenterLeaderboardEntrySubmissionCreateData {
                type_: "gameCenterLeaderboardEntrySubmissions".into(),
                attributes: GameCenterLeaderboardEntrySubmissionCreateAttributes {
                    bundle_id: "com.example.game".into(),
                    vendor_identifier: "com.example.leaderboards.high".into(),
                    scoped_player_id: "player-1".into(),
                    score: "12345".into(),
                    pre_released: Some(false),
                    ..Default::default()
                },
            },
        };

        let resp = mock
            .client()
            .create_game_center_leaderboard_entry_submission(&req)
            .await
            .expect("unexpected error");
        assert_eq!(resp.data.id, "entry-1");

        let r = &mock.requests()[0];
        assert_eq!(r.path, "/v1/gameCenterLeaderboardEntrySubmissions");
        assert_eq!(r.method, "POST");
        let body = r.json();
        assert_eq!(body["data"]["attributes"]["score"], "12345");
        assert_eq!(body["data"]["attributes"]["scopedPlayerId"], "player-1");
        assert_eq!(
            body["data"]["attributes"]["preReleased"], false,
            "expected preReleased=false in request body"
        );
    }

    #[tokio::test]
    async fn upload_game_center_images() {
        let cases: [(&str, &str, &str); 3] = [
            (
                "leaderboard set",
                "/v2/gameCenterLeaderboardSetImages",
                "gameCenterLeaderboardSetImages",
            ),
            (
                "activity",
                "/v1/gameCenterActivityImages",
                "gameCenterActivityImages",
            ),
            (
                "challenge",
                "/v1/gameCenterChallengeImages",
                "gameCenterChallengeImages",
            ),
        ];
        for (name, collection, resource_type) in cases {
            let payload: &'static [u8] = b"image-bytes";

            // The upload URL is filled in once the mock hands us its
            // address; the reservation response points the upload step back
            // at the same server.
            let upload_url = Arc::new(Mutex::new(String::new()));
            let url_for_handler = upload_url.clone();
            let collection_s = collection.to_string();
            let resource_s = resource_type.to_string();
            let mock = MockApi::start(move |r: &RecordedRequest| {
                if r.path == collection_s && r.method == "POST" {
                    let url = url_for_handler.lock().unwrap().clone();
                    let body = serde_json::json!({
                        "data": {
                            "type": resource_s,
                            "id": "img-1",
                            "attributes": {"uploadOperations": [{
                                "method": "PUT",
                                "url": url,
                                "offset": 0,
                                "length": payload.len(),
                            }]},
                        }
                    });
                    MockResponse::json(201, body.to_string())
                } else if r.path == "/upload" {
                    MockResponse::empty(200)
                } else if r.path == format!("{collection_s}/img-1") && r.method == "PATCH" {
                    MockResponse::json(
                        200,
                        format!(
                            r#"{{"data":{{"type":"{resource_s}","id":"img-1","attributes":{{}}}}}}"#
                        ),
                    )
                } else {
                    MockResponse::empty(404)
                }
            })
            .await;
            *upload_url.lock().unwrap() = format!("{}/upload", mock.base_url);

            let c = mock.client();
            let resp = match name {
                "leaderboard set" => {
                    c.upload_game_center_leaderboard_set_image("loc-1", "set.png", payload)
                        .await
                }
                "activity" => {
                    c.upload_game_center_activity_image("loc-1", "", "activity.png", payload)
                        .await
                }
                _ => {
                    c.upload_game_center_challenge_image("", "ver-1", "challenge.png", payload)
                        .await
                }
            }
            .unwrap_or_else(|e| panic!("{name}: unexpected error: {e}"));
            assert_eq!(resp.data.id, "img-1", "{name}");

            let reqs = mock.requests();
            assert_eq!(reqs.len(), 3, "{name}: unexpected requests {reqs:?}");
            let reservation = reqs[0].json();
            assert_eq!(reservation["data"]["type"], resource_type, "{name}");
            assert_eq!(
                reservation["data"]["attributes"]["fileSize"],
                payload.len(),
                "{name}"
            );
            assert_eq!(
                reservation["data"]["relationships"]
                    .as_object()
                    .map(|m| m.len()),
                Some(1),
                "{name}: expected exactly 1 relationship"
            );
            assert_eq!(reqs[1].path, "/upload", "{name}");
            assert_eq!(reqs[1].body, payload, "{name}: uploaded chunk");
            assert_eq!(
                reqs[2].method, "PATCH",
                "{name}: expected the commit PATCH to be issued"
            );
            assert_eq!(
                reqs[2].json()["data"]["attributes"]["uploaded"],
                true,
                "{name}: expected uploaded=true on commit"
            );
        }
    }

    #[tokio::test]
    async fn delete_game_center_content() {
        let cases: [(&str, &str); 7] = [
            ("leaderboard set", "/v2/gameCenterLeaderboardSets/set-1"),
            (
                "leaderboard set localization",
                "/v2/gameCenterLeaderboardSetLocalizations/loc-1",
            ),
            (
                "member localization",
                "/v1/gameCenterLeaderboardSetMemberLocalizations/mem-1",
            ),
            ("activity", "/v1/gameCenterActivities/act-1"),
            (
                "activity localization",
                "/v1/gameCenterActivityLocalizations/loc-1",
            ),
            ("challenge", "/v1/gameCenterChallenges/chl-1"),
            (
                "challenge localization",
                "/v1/gameCenterChallengeLocalizations/loc-1",
            ),
        ];
        for (name, want_path) in cases {
            let mock = MockApi::start(|_| MockResponse::empty(204)).await;
            let c = mock.client();
            let res = match name {
                "leaderboard set" => c.delete_game_center_leaderboard_set("set-1").await,
                "leaderboard set localization" => {
                    c.delete_game_center_leaderboard_set_localization("loc-1")
                        .await
                }
                "member localization" => {
                    c.delete_game_center_leaderboard_set_member_localization("mem-1")
                        .await
                }
                "activity" => c.delete_game_center_activity("act-1").await,
                "activity localization" => {
                    c.delete_game_center_activity_localization("loc-1").await
                }
                "challenge" => c.delete_game_center_challenge("chl-1").await,
                _ => c.delete_game_center_challenge_localization("loc-1").await,
            };
            res.unwrap_or_else(|e| panic!("{name}: unexpected error: {e}"));
            let r = &mock.requests()[0];
            assert_eq!(r.path, want_path, "{name}");
            assert_eq!(r.method, "DELETE", "{name}");
        }
    }

    #[tokio::test]
    async fn game_center_version_collections() {
        let cases: [(&str, &str); 10] = [
            (
                "leaderboard set versions",
                "/v2/gameCenterLeaderboardSets/set-1/versions",
            ),
            (
                "leaderboard set localizations",
                "/v2/gameCenterLeaderboardSetVersions/ver-1/localizations",
            ),
            (
                "leaderboard set members",
                "/v2/gameCenterLeaderboardSets/set-1/gameCenterLeaderboards",
            ),
            (
                "activities",
                "/v1/gameCenterDetails/gc-1/gameCenterActivities",
            ),
            (
                "activity versions",
                "/v1/gameCenterActivities/act-1/versions",
            ),
            (
                "activity localizations",
                "/v1/gameCenterActivityVersions/ver-1/localizations",
            ),
            (
                "challenges",
                "/v1/gameCenterDetails/gc-1/gameCenterChallenges",
            ),
            (
                "challenge versions",
                "/v1/gameCenterChallenges/chl-1/versions",
            ),
            (
                "challenge localizations",
                "/v1/gameCenterChallengeVersions/ver-1/localizations",
            ),
            (
                "member localizations",
                "/v1/gameCenterLeaderboardSetMemberLocalizations",
            ),
        ];
        for (name, want_path) in cases {
            let mock = MockApi::start(|_| MockResponse::json(200, r#"{"data":[]}"#)).await;
            let c = mock.client();
            let res: Result<()> = match name {
                "leaderboard set versions" => c
                    .list_game_center_leaderboard_set_versions("set-1", None)
                    .await
                    .map(|_| ()),
                "leaderboard set localizations" => c
                    .list_game_center_leaderboard_set_localizations("ver-1", None)
                    .await
                    .map(|_| ()),
                "leaderboard set members" => c
                    .list_game_center_leaderboard_set_members("set-1", None)
                    .await
                    .map(|_| ()),
                "activities" => c
                    .list_game_center_activities("gc-1", None)
                    .await
                    .map(|_| ()),
                "activity versions" => c
                    .list_game_center_activity_versions("act-1", None)
                    .await
                    .map(|_| ()),
                "activity localizations" => c
                    .list_game_center_activity_localizations("ver-1", None)
                    .await
                    .map(|_| ()),
                "challenges" => c
                    .list_game_center_challenges("gc-1", None)
                    .await
                    .map(|_| ()),
                "challenge versions" => c
                    .list_game_center_challenge_versions("chl-1", None)
                    .await
                    .map(|_| ()),
                "challenge localizations" => c
                    .list_game_center_challenge_localizations("ver-1", None)
                    .await
                    .map(|_| ()),
                _ => {
                    let mut filter = GoMap::nil();
                    filter.insert(
                        "gameCenterLeaderboardSet",
                        GoSlice::from(vec!["set-1".to_string()]),
                    );
                    let opts = ListOptions {
                        filter,
                        ..Default::default()
                    };
                    c.list_game_center_leaderboard_set_member_localizations(Some(&opts))
                        .await
                        .map(|_| ())
                }
            };
            res.unwrap_or_else(|e| panic!("{name}: unexpected error: {e}"));
            let r = &mock.requests()[0];
            assert_eq!(r.path, want_path, "{name}");
            assert_eq!(r.method, "GET", "{name}");
        }
    }
}
