//! Review submission tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
pub(super) fn register_review_submission_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_review_submissions",
            "description": "List review submissions for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The app ID"
                    },
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
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"WAITING_FOR_REVIEW\"]} becomes filter[state]=WAITING_FOR_REVIEW."
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
                        "description": "Maximum number of results to return (default 50)"
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
                    "app_id"
                ]
            },
            "annotations": {
                "title": "List Review Submissions",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_review_submissions),
    );
    r.register(
        tool(r##"{
            "name": "get_review_submission",
            "description": "Get a review submission and its attached items",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "review_submission_id": {
                        "type": "string",
                        "description": "The review submission ID"
                    }
                },
                "required": [
                    "review_submission_id"
                ]
            },
            "annotations": {
                "title": "Get Review Submission",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_review_submission),
    );
    r.register(
        tool(r##"{
            "name": "create_review_submission",
            "description": "Create a draft review submission for an app and platform. Attach items with add_review_submission_item, then submit with submit_review_submission.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The app ID"
                    },
                    "platform": {
                        "type": "string",
                        "description": "Platform: IOS, MAC_OS, TV_OS, VISION_OS (default IOS)"
                    }
                },
                "required": [
                    "app_id"
                ]
            },
            "annotations": {
                "title": "Create Review Submission",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_review_submission),
    );
    r.register(
        tool(r##"{
            "name": "add_review_submission_item",
            "description": "Attach an item to a draft review submission. Provide exactly one of app_store_version_id, app_custom_product_page_version_id, app_store_version_experiment_id, app_event_id, one of the Game Center version IDs, or one of the commerce version IDs. Game Center content and commerce products reach App Review through this tool: attach the achievement, leaderboard, leaderboard set, activity, challenge, in-app purchase, subscription, or subscription group VERSION, not the content resource itself.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_custom_product_page_version_id": {
                        "type": "string",
                        "description": "An app custom product page version ID to attach"
                    },
                    "app_event_id": {
                        "type": "string",
                        "description": "An app event ID to attach"
                    },
                    "app_store_version_experiment_id": {
                        "type": "string",
                        "description": "An App Store version experiment ID to attach"
                    },
                    "app_store_version_id": {
                        "type": "string",
                        "description": "An App Store version ID to attach"
                    },
                    "game_center_achievement_version_id": {
                        "type": "string",
                        "description": "A Game Center achievement version ID to attach"
                    },
                    "game_center_activity_version_id": {
                        "type": "string",
                        "description": "A Game Center activity version ID to attach"
                    },
                    "game_center_challenge_version_id": {
                        "type": "string",
                        "description": "A Game Center challenge version ID to attach"
                    },
                    "game_center_leaderboard_set_version_id": {
                        "type": "string",
                        "description": "A Game Center leaderboard set version ID to attach"
                    },
                    "game_center_leaderboard_version_id": {
                        "type": "string",
                        "description": "A Game Center leaderboard version ID to attach"
                    },
                    "in_app_purchase_version_id": {
                        "type": "string",
                        "description": "An in-app purchase version ID to attach"
                    },
                    "review_submission_id": {
                        "type": "string",
                        "description": "The review submission ID"
                    },
                    "subscription_group_version_id": {
                        "type": "string",
                        "description": "A subscription group version ID to attach"
                    },
                    "subscription_version_id": {
                        "type": "string",
                        "description": "A subscription version ID to attach"
                    }
                },
                "required": [
                    "review_submission_id"
                ]
            },
            "annotations": {
                "title": "Add Review Submission Item",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_add_review_submission_item),
    );
    r.register(
        tool(r##"{
            "name": "submit_review_submission",
            "description": "Submit a draft review submission for App Review",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "review_submission_id": {
                        "type": "string",
                        "description": "The review submission ID to submit"
                    }
                },
                "required": [
                    "review_submission_id"
                ]
            },
            "annotations": {
                "title": "Submit Review Submission",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_submit_review_submission),
    );
    r.register(
        tool(r##"{
            "name": "cancel_review_submission",
            "description": "Cancel a review submission",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "review_submission_id": {
                        "type": "string",
                        "description": "The review submission ID to cancel"
                    }
                },
                "required": [
                    "review_submission_id"
                ]
            },
            "annotations": {
                "title": "Cancel Review Submission",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_cancel_review_submission),
    );
}

/// Handles `list_review_submissions`.
async fn handle_list_review_submissions(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_review_submissions(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list review submissions: {e}"))),
    };

    Ok(list_result(&format_review_submissions(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_review_submission`.
async fn handle_get_review_submission(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { ReviewSubmissionID string \"json:\\\"review_submission_id\\\"\" }", "") {
        review_submission_id: String => "review_submission_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.review_submission_id.is_empty() {
        return Ok(error_result("review_submission_id is required"));
    }

    let resp = match c.get_review_submission(&params.review_submission_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get review submission: {e}"))),
    };

    let mut sb = String::new();
    sb.push_str(&format_review_submission(&resp.data));

    // Best-effort: also show the attached items.
    if let Ok(items) = c.list_review_submission_items(&params.review_submission_id, None).await {
        if !items.data.is_empty() {
            let _ = write!(sb, "\nItems ({}):\n", items.data.len());
            for item in items.data.iter() {
                let _ = writeln!(sb, "- {} (state: {})", item.id, item.attributes.state);
            }
        }
    }

    Ok(data_result(&sb, &resp.data))
}

/// Handles `create_review_submission`.
async fn handle_create_review_submission(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Platform string \"json:\\\"platform\\\"\" }", "") {
        app_id: String => "app_id";
        platform: String => "platform";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }

    let platform = if params.platform.is_empty() { "IOS".to_string() } else { params.platform.clone() };

    let req = ReviewSubmissionCreateRequest {
        data: ReviewSubmissionCreateData {
            type_: "reviewSubmissions".into(),
            attributes: Some(ReviewSubmissionCreateAttributes { platform }),
            relationships: ReviewSubmissionCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
            },
        },
    };

    let resp = match c.create_review_submission(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create review submission: {e}"))),
    };

    Ok(data_result(&format!("Review submission created:\n{}", format_review_submission(&resp.data)), &resp.data))
}

/// Handles `add_review_submission_item`.
async fn handle_add_review_submission_item(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { ReviewSubmissionID string \"json:\\\"review_submission_id\\\"\"; AppStoreVersionID string \"json:\\\"app_store_version_id\\\"\"; AppCustomProductPageVersionID string \"json:\\\"app_custom_product_page_version_id\\\"\"; AppStoreVersionExperimentID string \"json:\\\"app_store_version_experiment_id\\\"\"; AppEventID string \"json:\\\"app_event_id\\\"\"; GameCenterAchievementVersionID string \"json:\\\"game_center_achievement_version_id\\\"\"; GameCenterLeaderboardVersionID string \"json:\\\"game_center_leaderboard_version_id\\\"\"; GameCenterLeaderboardSetVersionID string \"json:\\\"game_center_leaderboard_set_version_id\\\"\"; GameCenterActivityVersionID string \"json:\\\"game_center_activity_version_id\\\"\"; GameCenterChallengeVersionID string \"json:\\\"game_center_challenge_version_id\\\"\"; InAppPurchaseVersionID string \"json:\\\"in_app_purchase_version_id\\\"\"; SubscriptionVersionID string \"json:\\\"subscription_version_id\\\"\"; SubscriptionGroupVersionID string \"json:\\\"subscription_group_version_id\\\"\" }", "") {
        review_submission_id: String => "review_submission_id";
        app_store_version_id: String => "app_store_version_id";
        app_custom_product_page_version_id: String => "app_custom_product_page_version_id";
        app_store_version_experiment_id: String => "app_store_version_experiment_id";
        app_event_id: String => "app_event_id";
        game_center_achievement_version_id: String => "game_center_achievement_version_id";
        game_center_leaderboard_version_id: String => "game_center_leaderboard_version_id";
        game_center_leaderboard_set_version_id: String => "game_center_leaderboard_set_version_id";
        game_center_activity_version_id: String => "game_center_activity_version_id";
        game_center_challenge_version_id: String => "game_center_challenge_version_id";
        in_app_purchase_version_id: String => "in_app_purchase_version_id";
        subscription_version_id: String => "subscription_version_id";
        subscription_group_version_id: String => "subscription_group_version_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.review_submission_id.is_empty() {
        return Ok(error_result("review_submission_id is required"));
    }

    let rel = |t: &str, id: &str| Some(RelationshipData { data: ResourceIdentifier { type_: t.into(), id: id.into() } });

    let mut rels = ReviewSubmissionItemCreateRelationships {
        review_submission: RelationshipData {
            data: ResourceIdentifier { type_: "reviewSubmissions".into(), id: params.review_submission_id.clone() },
        },
        ..Default::default()
    };

    let mut item_count = 0;
    if !params.app_store_version_id.is_empty() {
        rels.app_store_version = rel("appStoreVersions", &params.app_store_version_id);
        item_count += 1;
    }
    if !params.app_custom_product_page_version_id.is_empty() {
        rels.app_custom_product_page_version = rel("appCustomProductPageVersions", &params.app_custom_product_page_version_id);
        item_count += 1;
    }
    if !params.app_store_version_experiment_id.is_empty() {
        rels.app_store_version_experiment_v2 = rel("appStoreVersionExperiments", &params.app_store_version_experiment_id);
        item_count += 1;
    }
    if !params.app_event_id.is_empty() {
        rels.app_event = rel("appEvents", &params.app_event_id);
        item_count += 1;
    }
    // Game Center content is submitted for review by attaching the
    // content's version resource (App Store Connect API 4.2+).
    if !params.game_center_achievement_version_id.is_empty() {
        rels.game_center_achievement_version = rel("gameCenterAchievementVersions", &params.game_center_achievement_version_id);
        item_count += 1;
    }
    if !params.game_center_leaderboard_version_id.is_empty() {
        rels.game_center_leaderboard_version = rel("gameCenterLeaderboardVersions", &params.game_center_leaderboard_version_id);
        item_count += 1;
    }
    if !params.game_center_leaderboard_set_version_id.is_empty() {
        rels.game_center_leaderboard_set_version =
            rel("gameCenterLeaderboardSetVersions", &params.game_center_leaderboard_set_version_id);
        item_count += 1;
    }
    if !params.game_center_activity_version_id.is_empty() {
        rels.game_center_activity_version = rel("gameCenterActivityVersions", &params.game_center_activity_version_id);
        item_count += 1;
    }
    if !params.game_center_challenge_version_id.is_empty() {
        rels.game_center_challenge_version = rel("gameCenterChallengeVersions", &params.game_center_challenge_version_id);
        item_count += 1;
    }

    // In-app purchases, subscriptions and subscription groups are
    // submitted for review by attaching the product's version resource
    // (App Store Connect API 4.4.1).
    if !params.in_app_purchase_version_id.is_empty() {
        rels.in_app_purchase_version = rel("inAppPurchaseVersions", &params.in_app_purchase_version_id);
        item_count += 1;
    }
    if !params.subscription_version_id.is_empty() {
        rels.subscription_version = rel("subscriptionVersions", &params.subscription_version_id);
        item_count += 1;
    }
    if !params.subscription_group_version_id.is_empty() {
        rels.subscription_group_version = rel("subscriptionGroupVersions", &params.subscription_group_version_id);
        item_count += 1;
    }

    if item_count != 1 {
        return Ok(error_result("exactly one of app_store_version_id, app_custom_product_page_version_id, app_store_version_experiment_id, app_event_id, game_center_achievement_version_id, game_center_leaderboard_version_id, game_center_leaderboard_set_version_id, game_center_activity_version_id, game_center_challenge_version_id, in_app_purchase_version_id, subscription_version_id, or subscription_group_version_id is required"));
    }

    let req = ReviewSubmissionItemCreateRequest {
        data: ReviewSubmissionItemCreateData { type_: "reviewSubmissionItems".into(), relationships: rels },
    };

    let resp = match c.create_review_submission_item(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to add review submission item: {e}"))),
    };

    Ok(data_result(&format!("Review submission item added: {}", resp.data.id), &resp.data))
}

/// Handles `submit_review_submission`.
async fn handle_submit_review_submission(c: Arc<Client>, args: Args) -> HandlerResult {
    update_review_submission_state(c, args, "submit").await
}

/// Handles `cancel_review_submission`.
async fn handle_cancel_review_submission(c: Arc<Client>, args: Args) -> HandlerResult {
    update_review_submission_state(c, args, "cancel").await
}

/// Patches a review submission with submitted=true or canceled=true
/// depending on the action.
async fn update_review_submission_state(c: Arc<Client>, args: Args, action: &str) -> HandlerResult {
    json_struct! {
    struct Params ("struct { ReviewSubmissionID string \"json:\\\"review_submission_id\\\"\" }", "") {
        review_submission_id: String => "review_submission_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.review_submission_id.is_empty() {
        return Ok(error_result("review_submission_id is required"));
    }

    let flag = true;
    let mut attrs = ReviewSubmissionUpdateAttributes::default();
    if action == "submit" {
        attrs.submitted = Some(flag);
    } else {
        attrs.canceled = Some(flag);
    }

    let req = ReviewSubmissionUpdateRequest {
        data: ReviewSubmissionUpdateData {
            type_: "reviewSubmissions".into(),
            id: params.review_submission_id.clone(),
            attributes: attrs,
        },
    };

    let resp = match c.update_review_submission(&params.review_submission_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to {action} review submission: {e}"))),
    };

    if action == "submit" {
        return Ok(data_result(&format!("Review submission submitted:\n{}", format_review_submission(&resp.data)), &resp.data));
    }
    Ok(data_result(&format!("Review submission canceled:\n{}", format_review_submission(&resp.data)), &resp.data))
}

fn format_review_submissions(submissions: &[ReviewSubmission]) -> String {
    if submissions.is_empty() {
        return "No review submissions found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} review submissions:\n\n", submissions.len());

    for submission in submissions {
        sb.push_str(&format_review_submission(submission));
        sb.push_str("\n---\n");
    }

    sb
}

fn format_review_submission(submission: &ReviewSubmission) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", submission.id);
    if !submission.attributes.platform.is_empty() {
        let _ = writeln!(sb, "Platform: {}", submission.attributes.platform);
    }
    if !submission.attributes.state.is_empty() {
        let _ = writeln!(sb, "State: {}", submission.attributes.state);
    }
    if let Some(d) = &submission.attributes.submitted_date {
        let _ = writeln!(sb, "Submitted: {}", d.format("2006-01-02 15:04:05"));
    }
    sb
}
