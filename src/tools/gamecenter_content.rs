//! Ported from internal/asc/tools/gamecenter_content.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerGameCenterContentTools`.
pub(super) fn register_game_center_content_tools(r: &mut Registry) {
    super::gamecenter_leaderboard_sets::register_game_center_leaderboard_set_tools(r);
    super::gamecenter_activities::register_game_center_activity_tools(r);
    super::gamecenter_challenges::register_game_center_challenge_tools(r);
    super::gamecenter_submissions::register_game_center_submission_tools(r);
}

go_struct! {
pub(crate) struct GcListParams ("tools.gcListParams", "gcListParams") {
    pub(crate) limit: i64 => "limit";
    pub(crate) cursor: String => "cursor";
    pub(crate) filter: GoMap<GoSlice<String>> => "filter";
    pub(crate) fields: GoMap<GoSlice<String>> => "fields";
    pub(crate) include: GoSlice<String> => "include";
}
}


/// Go `gcLimit`: clamps a caller-supplied page size to the range the App
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
    /// Go `gcListParams.opts`: list options with the limit already clamped.
    pub(crate) fn opts(&self) -> Option<ListOptions> {
        list_opts(gc_limit(self.limit), &self.filter, &GoSlice::nil(), &self.fields, &self.include)
    }
}
