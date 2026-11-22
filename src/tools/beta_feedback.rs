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

    let opts = list_opts(limit, &params.filter, &params.sort, &GoMap::nil(), &GoSlice::nil());
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_beta_feedback_crash_submissions(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list beta feedback crashes: {e}"))),
    };

    let content = format_beta_feedback_list("crash feedback submissions", resp.data.len(), |sb| {
        for submission in resp.data.iter() {
            sb.push_str(&format_beta_feedback_submission(&submission.id, &submission.attributes));
            sb.push_str("\n---\n");
        }
    });
    Ok(list_result(&content, &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetBetaFeedbackCrash`.
async fn handle_get_beta_feedback_crash(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubmissionID string \"json:\\\"submission_id\\\"\" }", "") {
        submission_id: String => "submission_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.submission_id.is_empty() {
        return Ok(error_result("submission_id is required"));
    }

    let resp = match c.get_beta_feedback_crash_submission(&params.submission_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get beta feedback crash: {e}"))),
    };

    Ok(data_result(&format_beta_feedback_submission(&resp.data.id, &resp.data.attributes), &resp.data))
}

/// Ported from `Registry.handleGetBetaFeedbackCrashLog`.
async fn handle_get_beta_feedback_crash_log(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubmissionID string \"json:\\\"submission_id\\\"\" }", "") {
        submission_id: String => "submission_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.submission_id.is_empty() {
        return Ok(error_result("submission_id is required"));
    }

    let resp = match c.get_beta_feedback_crash_log(&params.submission_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get crash log: {e}"))),
    };

    if resp.data.attributes.log_text.is_empty() {
        return Ok(data_result("Crash log is empty", &resp.data));
    }
    Ok(data_result(
        &format!("Crash log for submission {}:\n\n{}", params.submission_id, resp.data.attributes.log_text),
        &resp.data,
    ))
}

/// Ported from `Registry.handleDeleteBetaFeedbackCrash`.
async fn handle_delete_beta_feedback_crash(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubmissionID string \"json:\\\"submission_id\\\"\" }", "") {
        submission_id: String => "submission_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.submission_id.is_empty() {
        return Ok(error_result("submission_id is required"));
    }

    if let Err(e) = c.delete_beta_feedback_crash_submission(&params.submission_id).await {
        return Ok(error_result(format!("Failed to delete beta feedback crash: {e}")));
    }

    Ok(CallToolResult::success("Beta feedback crash submission deleted successfully"))
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

    let opts = list_opts(limit, &params.filter, &params.sort, &GoMap::nil(), &GoSlice::nil());
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_beta_feedback_screenshot_submissions(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list beta feedback screenshots: {e}"))),
    };

    let content = format_beta_feedback_list("screenshot feedback submissions", resp.data.len(), |sb| {
        for submission in resp.data.iter() {
            sb.push_str(&format_beta_feedback_submission(&submission.id, &submission.attributes));
            sb.push_str("\n---\n");
        }
    });
    Ok(list_result(&content, &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetBetaFeedbackScreenshot`.
async fn handle_get_beta_feedback_screenshot(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubmissionID string \"json:\\\"submission_id\\\"\" }", "") {
        submission_id: String => "submission_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.submission_id.is_empty() {
        return Ok(error_result("submission_id is required"));
    }

    let resp = match c.get_beta_feedback_screenshot_submission(&params.submission_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get beta feedback screenshot: {e}"))),
    };

    Ok(data_result(&format_beta_feedback_submission(&resp.data.id, &resp.data.attributes), &resp.data))
}

/// Ported from `Registry.handleDeleteBetaFeedbackScreenshot`.
async fn handle_delete_beta_feedback_screenshot(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubmissionID string \"json:\\\"submission_id\\\"\" }", "") {
        submission_id: String => "submission_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.submission_id.is_empty() {
        return Ok(error_result("submission_id is required"));
    }

    if let Err(e) = c.delete_beta_feedback_screenshot_submission(&params.submission_id).await {
        return Ok(error_result(format!("Failed to delete beta feedback screenshot: {e}")));
    }

    Ok(CallToolResult::success("Beta feedback screenshot submission deleted successfully"))
}

/// Ported from `Registry.handleGetBetaBuildUsageMetrics`.
async fn handle_get_beta_build_usage_metrics(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildID string \"json:\\\"build_id\\\"\"; Limit int \"json:\\\"limit\\\"\" }", "") {
        build_id: String => "build_id";
        limit: i64 => "limit";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.build_id.is_empty() {
        return Ok(error_result("build_id is required"));
    }

    let mut limit = params.limit;
    if limit > 200 {
        limit = 200;
    }

    let resp = match c.get_beta_build_usage_metrics(&params.build_id, limit).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get beta build usage metrics: {e}"))),
    };

    Ok(list_result(&format_beta_build_usages(&params.build_id, &resp.data), &resp.data, &resp.links))
}

/// Ported from `formatBetaFeedbackList`: shares the "Found N ..." framing
/// between the two feedback list tools.
fn format_beta_feedback_list(noun: &str, count: usize, write: impl FnOnce(&mut String)) -> String {
    if count == 0 {
        return format!("No {noun} found");
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} {}:\n\n", count, noun);
    write(&mut sb);
    sb
}

/// Ported from `formatBetaFeedbackSubmission`.
fn format_beta_feedback_submission(id: &str, attrs: &BetaFeedbackDeviceAttributes) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "Submission ID: {}", id);
    if let Some(d) = &attrs.created_date {
        let _ = writeln!(sb, "Created: {}", d.format("2006-01-02 15:04:05"));
    }
    if !attrs.comment.is_empty() {
        let _ = writeln!(sb, "Comment: {}", attrs.comment);
    }
    if !attrs.email.is_empty() {
        let _ = writeln!(sb, "Tester Email: {}", attrs.email);
    }
    if !attrs.device_model.is_empty() {
        let _ = writeln!(sb, "Device: {} ({} {})", attrs.device_model, attrs.device_platform, attrs.os_version);
    }
    if !attrs.app_platform.is_empty() {
        let _ = writeln!(sb, "App Platform: {}", attrs.app_platform);
    }
    if !attrs.locale.is_empty() {
        let _ = writeln!(sb, "Locale: {}", attrs.locale);
    }
    if !attrs.architecture.is_empty() {
        let _ = writeln!(sb, "Architecture: {}", attrs.architecture);
    }
    if !attrs.connection_type.is_empty() {
        let _ = writeln!(sb, "Connection: {}", attrs.connection_type);
    }
    if !attrs.build_bundle_id.is_empty() {
        let _ = writeln!(sb, "Build Bundle ID: {}", attrs.build_bundle_id);
    }
    for (i, shot) in attrs.screenshots.iter().enumerate() {
        let _ = write!(sb, "Screenshot {}: {} ({}x{}", i + 1, shot.url, shot.width, shot.height);
        if let Some(d) = &shot.expiration_date {
            let _ = write!(sb, ", URL expires {}", d.format("2006-01-02 15:04:05"));
        }
        sb.push_str(")\n");
    }
    sb
}

/// Ported from `formatBetaBuildUsages`.
fn format_beta_build_usages(build_id: &str, groups: &[BetaBuildUsageGroup]) -> String {
    let mut points = 0usize;
    for group in groups {
        points += group.data_points.len();
    }
    if points == 0 {
        return format!("No usage metrics found for build {build_id}");
    }

    let mut sb = String::new();
    let _ = write!(sb, "TestFlight usage metrics for build {}:\n\n", build_id);
    for group in groups {
        for point in group.data_points.iter() {
            if let (Some(start), Some(end)) = (&point.start, &point.end) {
                let _ = writeln!(sb, "{} to {}:", start.format("2006-01-02"), end.format("2006-01-02"));
            }
            let v = &point.values;
            let _ = writeln!(
                sb,
                "Installs: {}, Sessions: {}, Crashes: {}, Feedback: {}, Invites: {}",
                v.install_count, v.session_count, v.crash_count, v.feedback_count, v.invite_count
            );
            sb.push('\n');
        }
    }
    sb
}
