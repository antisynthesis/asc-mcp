//! Shared Game Center content tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
pub(super) fn register_game_center_content_tools(r: &mut Registry) {
    super::gamecenter_leaderboard_sets::register_game_center_leaderboard_set_tools(r);
    super::gamecenter_activities::register_game_center_activity_tools(r);
    super::gamecenter_challenges::register_game_center_challenge_tools(r);
    super::gamecenter_submissions::register_game_center_submission_tools(r);
}

json_struct! {
pub(crate) struct GcListParams ("tools.gcListParams", "gcListParams") {
    pub(crate) limit: i64 => "limit";
    pub(crate) cursor: String => "cursor";
    pub(crate) filter: JsonMap<JsonSlice<String>> => "filter";
    pub(crate) fields: JsonMap<JsonSlice<String>> => "fields";
    pub(crate) include: JsonSlice<String> => "include";
}
}


/// Clamps a caller-supplied page size to the range the App
/// Store Connect API accepts, defaulting to 50.
pub(crate) fn gc_limit(limit: i64) -> i64 {
    if limit <= 0 {
        return 50;
    }
    if limit > 200 {
        return 200;
    }
    limit
}

impl GcListParams {
    /// List options with the limit already clamped.
    pub(crate) fn opts(&self) -> Option<ListOptions> {
        list_opts(gc_limit(self.limit), &self.filter, &JsonSlice::nil(), &self.fields, &self.include)
    }
}
