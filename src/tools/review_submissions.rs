//! Ported from internal/asc/tools/review_submissions.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerReviewSubmissionTools`.
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

/// Ported from `Registry.handleListReviewSubmissions`.
async fn handle_list_review_submissions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListReviewSubmissions from internal/asc/tools/review_submissions.go")
}

/// Ported from `Registry.handleGetReviewSubmission`.
async fn handle_get_review_submission(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ReviewSubmissionID string \"json:\\\"review_submission_id\\\"\" }", "") {
        review_submission_id: String => "review_submission_id";
    }
    }
    todo!("port handleGetReviewSubmission from internal/asc/tools/review_submissions.go")
}

/// Ported from `Registry.handleCreateReviewSubmission`.
async fn handle_create_review_submission(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Platform string \"json:\\\"platform\\\"\" }", "") {
        app_id: String => "app_id";
        platform: String => "platform";
    }
    }
    todo!("port handleCreateReviewSubmission from internal/asc/tools/review_submissions.go")
}

/// Ported from `Registry.handleAddReviewSubmissionItem`.
async fn handle_add_review_submission_item(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
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
    todo!("port handleAddReviewSubmissionItem from internal/asc/tools/review_submissions.go")
}

/// Ported from `Registry.handleSubmitReviewSubmission`.
async fn handle_submit_review_submission(c: Arc<Client>, args: Args) -> HandlerResult {
    todo!("port handleSubmitReviewSubmission from internal/asc/tools/review_submissions.go")
}

/// Ported from `Registry.handleCancelReviewSubmission`.
async fn handle_cancel_review_submission(c: Arc<Client>, args: Args) -> HandlerResult {
    todo!("port handleCancelReviewSubmission from internal/asc/tools/review_submissions.go")
}

