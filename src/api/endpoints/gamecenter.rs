// App Store Connect API endpoints for the "gamecenter" area.
//
// Signatures were generated from the Go declarations; bodies are ported by
// hand from the Go file named in each method's todo!/comment and must
// preserve the exact request path, query and body.

#[allow(unused_imports)]
use crate::api::{decode, model::*, Client, Error, ListOptions, Result};
#[allow(unused_imports)]
use crate::go::{json, url::{self, Values}, GoBytes, GoMap, GoSlice, GoTime, RawMessage};
#[allow(unused_imports)]
use serde_json::Value;

impl Client {
    /// CreateGameCenterAchievement creates a new achievement via the v2 Game Center API.
    pub async fn create_game_center_achievement(&self, req: &GameCenterAchievementCreateRequest) -> Result<GameCenterAchievementResponse> {
        todo!("port Client.CreateGameCenterAchievement from internal/asc/api/client.go")
    }

    /// CreateGameCenterLeaderboard creates a new leaderboard via the v2 Game Center API.
    pub async fn create_game_center_leaderboard(&self, req: &GameCenterLeaderboardCreateRequest) -> Result<GameCenterLeaderboardResponse> {
        todo!("port Client.CreateGameCenterLeaderboard from internal/asc/api/client.go")
    }

    /// DeleteGameCenterAchievement deletes an achievement.
    pub async fn delete_game_center_achievement(&self, achievement_id: &str) -> Result<()> {
        todo!("port Client.DeleteGameCenterAchievement from internal/asc/api/client.go")
    }

    /// DeleteGameCenterLeaderboard deletes a leaderboard.
    pub async fn delete_game_center_leaderboard(&self, leaderboard_id: &str) -> Result<()> {
        todo!("port Client.DeleteGameCenterLeaderboard from internal/asc/api/client.go")
    }

    /// GetGameCenterAchievement returns a single achievement.
    pub async fn get_game_center_achievement(&self, achievement_id: &str) -> Result<GameCenterAchievementResponse> {
        todo!("port Client.GetGameCenterAchievement from internal/asc/api/client.go")
    }

    /// GetGameCenterDetail returns game center details for an app.
    pub async fn get_game_center_detail(&self, app_id: &str) -> Result<GameCenterDetailResponse> {
        todo!("port Client.GetGameCenterDetail from internal/asc/api/client.go")
    }

    /// GetGameCenterLeaderboard returns a single leaderboard.
    pub async fn get_game_center_leaderboard(&self, leaderboard_id: &str) -> Result<GameCenterLeaderboardResponse> {
        todo!("port Client.GetGameCenterLeaderboard from internal/asc/api/client.go")
    }

    /// ListGameCenterAchievements returns achievements for a game center detail via the v2 Game Center API.
    pub async fn list_game_center_achievements(&self, game_center_detail_id: &str, opts: Option<&ListOptions>) -> Result<GameCenterAchievementsResponse> {
        todo!("port Client.ListGameCenterAchievements from internal/asc/api/client.go")
    }

    /// ListGameCenterLeaderboards returns leaderboards for a game center detail via the v2 Game Center API.
    pub async fn list_game_center_leaderboards(&self, game_center_detail_id: &str, opts: Option<&ListOptions>) -> Result<GameCenterLeaderboardsResponse> {
        todo!("port Client.ListGameCenterLeaderboards from internal/asc/api/client.go")
    }

    /// UpdateGameCenterAchievement updates an achievement.
    pub async fn update_game_center_achievement(&self, achievement_id: &str, req: &GameCenterAchievementUpdateRequest) -> Result<GameCenterAchievementResponse> {
        todo!("port Client.UpdateGameCenterAchievement from internal/asc/api/client.go")
    }

    /// UpdateGameCenterLeaderboard updates a leaderboard.
    pub async fn update_game_center_leaderboard(&self, leaderboard_id: &str, req: &GameCenterLeaderboardUpdateRequest) -> Result<GameCenterLeaderboardResponse> {
        todo!("port Client.UpdateGameCenterLeaderboard from internal/asc/api/client.go")
    }

    /// AddGameCenterLeaderboardSetMembers adds leaderboards to a leaderboard set.
    pub async fn add_game_center_leaderboard_set_members(&self, leaderboard_set_id: &str, req: &RelationshipDataList) -> Result<()> {
        todo!("port Client.AddGameCenterLeaderboardSetMembers from internal/asc/api/client_gamecenter.go")
    }

    /// CreateGameCenterActivity creates an activity along with its initial version.
    pub async fn create_game_center_activity(&self, req: &GameCenterActivityCreateRequest) -> Result<GameCenterActivityResponse> {
        todo!("port Client.CreateGameCenterActivity from internal/asc/api/client_gamecenter.go")
    }

    /// CreateGameCenterActivityLocalization creates an activity localization.
    pub async fn create_game_center_activity_localization(&self, req: &GameCenterActivityLocalizationCreateRequest) -> Result<GameCenterActivityLocalizationResponse> {
        todo!("port Client.CreateGameCenterActivityLocalization from internal/asc/api/client_gamecenter.go")
    }

    /// CreateGameCenterActivityVersion opens a new editable version of an activity.
    pub async fn create_game_center_activity_version(&self, req: &GameCenterActivityVersionCreateRequest) -> Result<GameCenterActivityVersionResponse> {
        todo!("port Client.CreateGameCenterActivityVersion from internal/asc/api/client_gamecenter.go")
    }

    /// CreateGameCenterChallenge creates a challenge along with its initial version.
    pub async fn create_game_center_challenge(&self, req: &GameCenterChallengeCreateRequest) -> Result<GameCenterChallengeResponse> {
        todo!("port Client.CreateGameCenterChallenge from internal/asc/api/client_gamecenter.go")
    }

    /// CreateGameCenterChallengeLocalization creates a challenge localization.
    pub async fn create_game_center_challenge_localization(&self, req: &GameCenterChallengeLocalizationCreateRequest) -> Result<GameCenterChallengeLocalizationResponse> {
        todo!("port Client.CreateGameCenterChallengeLocalization from internal/asc/api/client_gamecenter.go")
    }

    /// CreateGameCenterChallengeVersion opens a new editable version of a challenge.
    pub async fn create_game_center_challenge_version(&self, req: &GameCenterChallengeVersionCreateRequest) -> Result<GameCenterChallengeVersionResponse> {
        todo!("port Client.CreateGameCenterChallengeVersion from internal/asc/api/client_gamecenter.go")
    }

    /// CreateGameCenterLeaderboardEntrySubmission submits a leaderboard score for
    /// a player on the game server's behalf.
    pub async fn create_game_center_leaderboard_entry_submission(&self, req: &GameCenterLeaderboardEntrySubmissionCreateRequest) -> Result<GameCenterLeaderboardEntrySubmissionResponse> {
        todo!("port Client.CreateGameCenterLeaderboardEntrySubmission from internal/asc/api/client_gamecenter.go")
    }

    /// CreateGameCenterLeaderboardSet creates a leaderboard set via the v2 Game Center API.
    pub async fn create_game_center_leaderboard_set(&self, req: &GameCenterLeaderboardSetCreateRequest) -> Result<GameCenterLeaderboardSetResponse> {
        todo!("port Client.CreateGameCenterLeaderboardSet from internal/asc/api/client_gamecenter.go")
    }

    /// CreateGameCenterLeaderboardSetLocalization creates a leaderboard set localization.
    pub async fn create_game_center_leaderboard_set_localization(&self, req: &GameCenterLeaderboardSetLocalizationCreateRequest) -> Result<GameCenterLeaderboardSetLocalizationResponse> {
        todo!("port Client.CreateGameCenterLeaderboardSetLocalization from internal/asc/api/client_gamecenter.go")
    }

    /// CreateGameCenterLeaderboardSetMemberLocalization names a leaderboard
    /// within a leaderboard set for one locale.
    pub async fn create_game_center_leaderboard_set_member_localization(&self, req: &GameCenterLeaderboardSetMemberLocalizationCreateRequest) -> Result<GameCenterLeaderboardSetMemberLocalizationResponse> {
        todo!("port Client.CreateGameCenterLeaderboardSetMemberLocalization from internal/asc/api/client_gamecenter.go")
    }

    /// CreateGameCenterLeaderboardSetVersion opens a new editable version of a leaderboard set.
    pub async fn create_game_center_leaderboard_set_version(&self, req: &GameCenterLeaderboardSetVersionCreateRequest) -> Result<GameCenterLeaderboardSetVersionResponse> {
        todo!("port Client.CreateGameCenterLeaderboardSetVersion from internal/asc/api/client_gamecenter.go")
    }

    /// CreateGameCenterPlayerAchievementSubmission submits achievement progress
    /// for a player on the game server's behalf.
    pub async fn create_game_center_player_achievement_submission(&self, req: &GameCenterPlayerAchievementSubmissionCreateRequest) -> Result<GameCenterPlayerAchievementSubmissionResponse> {
        todo!("port Client.CreateGameCenterPlayerAchievementSubmission from internal/asc/api/client_gamecenter.go")
    }

    /// DeleteGameCenterActivity deletes an activity.
    pub async fn delete_game_center_activity(&self, activity_id: &str) -> Result<()> {
        todo!("port Client.DeleteGameCenterActivity from internal/asc/api/client_gamecenter.go")
    }

    /// DeleteGameCenterActivityLocalization deletes an activity localization.
    pub async fn delete_game_center_activity_localization(&self, localization_id: &str) -> Result<()> {
        todo!("port Client.DeleteGameCenterActivityLocalization from internal/asc/api/client_gamecenter.go")
    }

    /// DeleteGameCenterChallenge deletes a challenge.
    pub async fn delete_game_center_challenge(&self, challenge_id: &str) -> Result<()> {
        todo!("port Client.DeleteGameCenterChallenge from internal/asc/api/client_gamecenter.go")
    }

    /// DeleteGameCenterChallengeLocalization deletes a challenge localization.
    pub async fn delete_game_center_challenge_localization(&self, localization_id: &str) -> Result<()> {
        todo!("port Client.DeleteGameCenterChallengeLocalization from internal/asc/api/client_gamecenter.go")
    }

    /// DeleteGameCenterLeaderboardSet deletes a leaderboard set.
    pub async fn delete_game_center_leaderboard_set(&self, leaderboard_set_id: &str) -> Result<()> {
        todo!("port Client.DeleteGameCenterLeaderboardSet from internal/asc/api/client_gamecenter.go")
    }

    /// DeleteGameCenterLeaderboardSetLocalization deletes a leaderboard set localization.
    pub async fn delete_game_center_leaderboard_set_localization(&self, localization_id: &str) -> Result<()> {
        todo!("port Client.DeleteGameCenterLeaderboardSetLocalization from internal/asc/api/client_gamecenter.go")
    }

    /// DeleteGameCenterLeaderboardSetMemberLocalization deletes a member localization.
    pub async fn delete_game_center_leaderboard_set_member_localization(&self, localization_id: &str) -> Result<()> {
        todo!("port Client.DeleteGameCenterLeaderboardSetMemberLocalization from internal/asc/api/client_gamecenter.go")
    }

    /// GetGameCenterActivity returns a single activity.
    pub async fn get_game_center_activity(&self, activity_id: &str) -> Result<GameCenterActivityResponse> {
        todo!("port Client.GetGameCenterActivity from internal/asc/api/client_gamecenter.go")
    }

    /// GetGameCenterActivityVersion returns a single activity version.
    pub async fn get_game_center_activity_version(&self, version_id: &str) -> Result<GameCenterActivityVersionResponse> {
        todo!("port Client.GetGameCenterActivityVersion from internal/asc/api/client_gamecenter.go")
    }

    /// GetGameCenterChallenge returns a single challenge.
    pub async fn get_game_center_challenge(&self, challenge_id: &str) -> Result<GameCenterChallengeResponse> {
        todo!("port Client.GetGameCenterChallenge from internal/asc/api/client_gamecenter.go")
    }

    /// GetGameCenterChallengeVersion returns a single challenge version.
    pub async fn get_game_center_challenge_version(&self, version_id: &str) -> Result<GameCenterChallengeVersionResponse> {
        todo!("port Client.GetGameCenterChallengeVersion from internal/asc/api/client_gamecenter.go")
    }

    /// GetGameCenterLeaderboardSet returns a single leaderboard set.
    pub async fn get_game_center_leaderboard_set(&self, leaderboard_set_id: &str) -> Result<GameCenterLeaderboardSetResponse> {
        todo!("port Client.GetGameCenterLeaderboardSet from internal/asc/api/client_gamecenter.go")
    }

    /// GetGameCenterLeaderboardSetVersion returns a single leaderboard set version.
    pub async fn get_game_center_leaderboard_set_version(&self, version_id: &str) -> Result<GameCenterLeaderboardSetVersionResponse> {
        todo!("port Client.GetGameCenterLeaderboardSetVersion from internal/asc/api/client_gamecenter.go")
    }

    /// ListGameCenterActivities returns the activities for a game center detail.
    pub async fn list_game_center_activities(&self, game_center_detail_id: &str, opts: Option<&ListOptions>) -> Result<GameCenterActivitiesResponse> {
        todo!("port Client.ListGameCenterActivities from internal/asc/api/client_gamecenter.go")
    }

    /// ListGameCenterActivityLocalizations returns the localizations of an activity version.
    pub async fn list_game_center_activity_localizations(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<GameCenterActivityLocalizationsResponse> {
        todo!("port Client.ListGameCenterActivityLocalizations from internal/asc/api/client_gamecenter.go")
    }

    /// ListGameCenterActivityVersions returns the versions of an activity.
    pub async fn list_game_center_activity_versions(&self, activity_id: &str, opts: Option<&ListOptions>) -> Result<GameCenterActivityVersionsResponse> {
        todo!("port Client.ListGameCenterActivityVersions from internal/asc/api/client_gamecenter.go")
    }

    /// ListGameCenterChallengeLocalizations returns the localizations of a challenge version.
    pub async fn list_game_center_challenge_localizations(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<GameCenterChallengeLocalizationsResponse> {
        todo!("port Client.ListGameCenterChallengeLocalizations from internal/asc/api/client_gamecenter.go")
    }

    /// ListGameCenterChallengeVersions returns the versions of a challenge.
    pub async fn list_game_center_challenge_versions(&self, challenge_id: &str, opts: Option<&ListOptions>) -> Result<GameCenterChallengeVersionsResponse> {
        todo!("port Client.ListGameCenterChallengeVersions from internal/asc/api/client_gamecenter.go")
    }

    /// ListGameCenterChallenges returns the challenges for a game center detail.
    pub async fn list_game_center_challenges(&self, game_center_detail_id: &str, opts: Option<&ListOptions>) -> Result<GameCenterChallengesResponse> {
        todo!("port Client.ListGameCenterChallenges from internal/asc/api/client_gamecenter.go")
    }

    /// ListGameCenterLeaderboardSetLocalizations returns the localizations of a
    /// leaderboard set version.
    pub async fn list_game_center_leaderboard_set_localizations(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<GameCenterLeaderboardSetLocalizationsResponse> {
        todo!("port Client.ListGameCenterLeaderboardSetLocalizations from internal/asc/api/client_gamecenter.go")
    }

    /// ListGameCenterLeaderboardSetMemberLocalizations returns the per-set names
    /// of leaderboards. The API requires at least one of the
    /// filter[gameCenterLeaderboardSet] / filter[gameCenterLeaderboard] filters,
    /// which the caller supplies through opts.
    pub async fn list_game_center_leaderboard_set_member_localizations(&self, opts: Option<&ListOptions>) -> Result<GameCenterLeaderboardSetMemberLocalizationsResponse> {
        todo!("port Client.ListGameCenterLeaderboardSetMemberLocalizations from internal/asc/api/client_gamecenter.go")
    }

    /// ListGameCenterLeaderboardSetMembers returns the leaderboards that belong
    /// to a leaderboard set.
    pub async fn list_game_center_leaderboard_set_members(&self, leaderboard_set_id: &str, opts: Option<&ListOptions>) -> Result<GameCenterLeaderboardsResponse> {
        todo!("port Client.ListGameCenterLeaderboardSetMembers from internal/asc/api/client_gamecenter.go")
    }

    /// ListGameCenterLeaderboardSetVersions returns the versions of a leaderboard set.
    pub async fn list_game_center_leaderboard_set_versions(&self, leaderboard_set_id: &str, opts: Option<&ListOptions>) -> Result<GameCenterLeaderboardSetVersionsResponse> {
        todo!("port Client.ListGameCenterLeaderboardSetVersions from internal/asc/api/client_gamecenter.go")
    }

    /// ListGameCenterLeaderboardSets returns the leaderboard sets for a game
    /// center detail via the v2 Game Center API.
    pub async fn list_game_center_leaderboard_sets(&self, game_center_detail_id: &str, opts: Option<&ListOptions>) -> Result<GameCenterLeaderboardSetsResponse> {
        todo!("port Client.ListGameCenterLeaderboardSets from internal/asc/api/client_gamecenter.go")
    }

    /// RemoveGameCenterLeaderboardSetMembers removes leaderboards from a
    /// leaderboard set. The linkages travel in the DELETE body.
    pub async fn remove_game_center_leaderboard_set_members(&self, leaderboard_set_id: &str, req: &RelationshipDataList) -> Result<()> {
        todo!("port Client.RemoveGameCenterLeaderboardSetMembers from internal/asc/api/client_gamecenter.go")
    }

    /// UpdateGameCenterActivity updates an activity.
    pub async fn update_game_center_activity(&self, activity_id: &str, req: &GameCenterActivityUpdateRequest) -> Result<GameCenterActivityResponse> {
        todo!("port Client.UpdateGameCenterActivity from internal/asc/api/client_gamecenter.go")
    }

    /// UpdateGameCenterActivityLocalization updates an activity localization.
    pub async fn update_game_center_activity_localization(&self, localization_id: &str, req: &GameCenterActivityLocalizationUpdateRequest) -> Result<GameCenterActivityLocalizationResponse> {
        todo!("port Client.UpdateGameCenterActivityLocalization from internal/asc/api/client_gamecenter.go")
    }

    /// UpdateGameCenterActivityVersion updates an activity version's fallback URL.
    pub async fn update_game_center_activity_version(&self, version_id: &str, req: &GameCenterActivityVersionUpdateRequest) -> Result<GameCenterActivityVersionResponse> {
        todo!("port Client.UpdateGameCenterActivityVersion from internal/asc/api/client_gamecenter.go")
    }

    /// UpdateGameCenterChallenge updates a challenge.
    pub async fn update_game_center_challenge(&self, challenge_id: &str, req: &GameCenterChallengeUpdateRequest) -> Result<GameCenterChallengeResponse> {
        todo!("port Client.UpdateGameCenterChallenge from internal/asc/api/client_gamecenter.go")
    }

    /// UpdateGameCenterChallengeLocalization updates a challenge localization.
    pub async fn update_game_center_challenge_localization(&self, localization_id: &str, req: &GameCenterChallengeLocalizationUpdateRequest) -> Result<GameCenterChallengeLocalizationResponse> {
        todo!("port Client.UpdateGameCenterChallengeLocalization from internal/asc/api/client_gamecenter.go")
    }

    /// UpdateGameCenterLeaderboardSet updates a leaderboard set.
    pub async fn update_game_center_leaderboard_set(&self, leaderboard_set_id: &str, req: &GameCenterLeaderboardSetUpdateRequest) -> Result<GameCenterLeaderboardSetResponse> {
        todo!("port Client.UpdateGameCenterLeaderboardSet from internal/asc/api/client_gamecenter.go")
    }

    /// UpdateGameCenterLeaderboardSetLocalization updates a leaderboard set localization.
    pub async fn update_game_center_leaderboard_set_localization(&self, localization_id: &str, req: &GameCenterLeaderboardSetLocalizationUpdateRequest) -> Result<GameCenterLeaderboardSetLocalizationResponse> {
        todo!("port Client.UpdateGameCenterLeaderboardSetLocalization from internal/asc/api/client_gamecenter.go")
    }

    /// UpdateGameCenterLeaderboardSetMemberLocalization updates a member localization.
    pub async fn update_game_center_leaderboard_set_member_localization(&self, localization_id: &str, req: &GameCenterLeaderboardSetMemberLocalizationUpdateRequest) -> Result<GameCenterLeaderboardSetMemberLocalizationResponse> {
        todo!("port Client.UpdateGameCenterLeaderboardSetMemberLocalization from internal/asc/api/client_gamecenter.go")
    }

    /// UploadGameCenterActivityImage uploads an activity image. Passing a
    /// localization ID sets that locale's image; passing a version ID instead
    /// sets the version's default image.
    pub async fn upload_game_center_activity_image(&self, localization_id: &str, version_id: &str, file_name: &str, body: &[u8]) -> Result<GameCenterImageResponse> {
        todo!("port Client.UploadGameCenterActivityImage from internal/asc/api/client_gamecenter.go")
    }

    /// UploadGameCenterChallengeImage uploads a challenge image. Passing a
    /// localization ID sets that locale's image; passing a version ID instead
    /// sets the version's default image.
    pub async fn upload_game_center_challenge_image(&self, localization_id: &str, version_id: &str, file_name: &str, body: &[u8]) -> Result<GameCenterImageResponse> {
        todo!("port Client.UploadGameCenterChallengeImage from internal/asc/api/client_gamecenter.go")
    }

    /// UploadGameCenterLeaderboardSetImage uploads the image shown for a
    /// leaderboard set localization.
    pub async fn upload_game_center_leaderboard_set_image(&self, localization_id: &str, file_name: &str, body: &[u8]) -> Result<GameCenterImageResponse> {
        todo!("port Client.UploadGameCenterLeaderboardSetImage from internal/asc/api/client_gamecenter.go")
    }
}
