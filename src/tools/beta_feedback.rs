//! Ported from internal/asc/tools/beta_feedback.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerBetaFeedbackTools`.
pub(super) fn register_beta_feedback_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_beta_feedback_crashes",
            "description": "List crash feedback submissions from TestFlight testers for an app, including device and environment details. Use get_beta_feedback_crash_log to fetch a submission's crash log text.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to list crash feedback for"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Supported keys: deviceModel, osVersion, appPlatform (IOS, MAC_OS, TV_OS, VISION_OS), devicePlatform, build, build.preReleaseVersion, tester. Values are arrays, e.g. {\"build\": [\"BUILD_ID\"]}."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of submissions to return (default 50, max 200)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort order: createdDate or -createdDate",
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
                "title": "List Beta Feedback Crashes",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_beta_feedback_crashes),
    );
    r.register(
        tool(r##"{
            "name": "get_beta_feedback_crash",
            "description": "Get details of a specific crash feedback submission",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "submission_id": {
                        "type": "string",
                        "description": "The Beta Feedback Crash Submission ID"
                    }
                },
                "required": [
                    "submission_id"
                ]
            },
            "annotations": {
                "title": "Get Beta Feedback Crash",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_beta_feedback_crash),
    );
    r.register(
        tool(r##"{
            "name": "get_beta_feedback_crash_log",
            "description": "Get the crash log text attached to a crash feedback submission",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "submission_id": {
                        "type": "string",
                        "description": "The Beta Feedback Crash Submission ID"
                    }
                },
                "required": [
                    "submission_id"
                ]
            },
            "annotations": {
                "title": "Get Beta Feedback Crash Log",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_beta_feedback_crash_log),
    );
    r.register(
        tool(r##"{
            "name": "delete_beta_feedback_crash",
            "description": "Delete a crash feedback submission",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "submission_id": {
                        "type": "string",
                        "description": "The Beta Feedback Crash Submission ID"
                    }
                },
                "required": [
                    "submission_id"
                ]
            },
            "annotations": {
                "title": "Delete Beta Feedback Crash",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_beta_feedback_crash),
    );
    r.register(
        tool(r##"{
            "name": "list_beta_feedback_screenshots",
            "description": "List screenshot feedback submissions from TestFlight testers for an app, including tester comments, device details, and time-limited screenshot image URLs",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to list screenshot feedback for"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Supported keys: deviceModel, osVersion, appPlatform (IOS, MAC_OS, TV_OS, VISION_OS), devicePlatform, build, build.preReleaseVersion, tester. Values are arrays, e.g. {\"build\": [\"BUILD_ID\"]}."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of submissions to return (default 50, max 200)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort order: createdDate or -createdDate",
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
                "title": "List Beta Feedback Screenshots",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_beta_feedback_screenshots),
    );
    r.register(
        tool(r##"{
            "name": "get_beta_feedback_screenshot",
            "description": "Get details of a specific screenshot feedback submission, including its screenshot image download URLs",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "submission_id": {
                        "type": "string",
                        "description": "The Beta Feedback Screenshot Submission ID"
                    }
                },
                "required": [
                    "submission_id"
                ]
            },
            "annotations": {
                "title": "Get Beta Feedback Screenshot",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_beta_feedback_screenshot),
    );
    r.register(
        tool(r##"{
            "name": "delete_beta_feedback_screenshot",
            "description": "Delete a screenshot feedback submission",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "submission_id": {
                        "type": "string",
                        "description": "The Beta Feedback Screenshot Submission ID"
                    }
                },
                "required": [
                    "submission_id"
                ]
            },
            "annotations": {
                "title": "Delete Beta Feedback Screenshot",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_beta_feedback_screenshot),
    );
    r.register(
        tool(r##"{
            "name": "get_beta_build_usage_metrics",
            "description": "Get TestFlight usage metrics for a build: install, session, crash, feedback, and invite counts",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "build_id": {
                        "type": "string",
                        "description": "The Build ID to fetch usage metrics for"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of metric groups to return (max 200)"
                    }
                },
                "required": [
                    "build_id"
                ]
            },
            "annotations": {
                "title": "Get Beta Build Usage Metrics",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_beta_build_usage_metrics),
    );
}

/// Ported from `Registry.handleListBetaFeedbackCrashes`.
async fn handle_list_beta_feedback_crashes(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
    }
    }
    todo!("port handleListBetaFeedbackCrashes from internal/asc/tools/beta_feedback.go")
}

/// Ported from `Registry.handleGetBetaFeedbackCrash`.
async fn handle_get_beta_feedback_crash(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubmissionID string \"json:\\\"submission_id\\\"\" }", "") {
        submission_id: String => "submission_id";
    }
    }
    todo!("port handleGetBetaFeedbackCrash from internal/asc/tools/beta_feedback.go")
}

/// Ported from `Registry.handleGetBetaFeedbackCrashLog`.
async fn handle_get_beta_feedback_crash_log(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubmissionID string \"json:\\\"submission_id\\\"\" }", "") {
        submission_id: String => "submission_id";
    }
    }
    todo!("port handleGetBetaFeedbackCrashLog from internal/asc/tools/beta_feedback.go")
}

/// Ported from `Registry.handleDeleteBetaFeedbackCrash`.
async fn handle_delete_beta_feedback_crash(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubmissionID string \"json:\\\"submission_id\\\"\" }", "") {
        submission_id: String => "submission_id";
    }
    }
    todo!("port handleDeleteBetaFeedbackCrash from internal/asc/tools/beta_feedback.go")
}

/// Ported from `Registry.handleListBetaFeedbackScreenshots`.
async fn handle_list_beta_feedback_screenshots(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
    }
    }
    todo!("port handleListBetaFeedbackScreenshots from internal/asc/tools/beta_feedback.go")
}

/// Ported from `Registry.handleGetBetaFeedbackScreenshot`.
async fn handle_get_beta_feedback_screenshot(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubmissionID string \"json:\\\"submission_id\\\"\" }", "") {
        submission_id: String => "submission_id";
    }
    }
    todo!("port handleGetBetaFeedbackScreenshot from internal/asc/tools/beta_feedback.go")
}

/// Ported from `Registry.handleDeleteBetaFeedbackScreenshot`.
async fn handle_delete_beta_feedback_screenshot(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubmissionID string \"json:\\\"submission_id\\\"\" }", "") {
        submission_id: String => "submission_id";
    }
    }
    todo!("port handleDeleteBetaFeedbackScreenshot from internal/asc/tools/beta_feedback.go")
}

/// Ported from `Registry.handleGetBetaBuildUsageMetrics`.
async fn handle_get_beta_build_usage_metrics(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildID string \"json:\\\"build_id\\\"\"; Limit int \"json:\\\"limit\\\"\" }", "") {
        build_id: String => "build_id";
        limit: i64 => "limit";
    }
    }
    todo!("port handleGetBetaBuildUsageMetrics from internal/asc/tools/beta_feedback.go")
}

