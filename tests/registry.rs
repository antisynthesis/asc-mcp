//! Tool registry tests, ported from internal/asc/tools/registry_test.go.

mod common;

use std::collections::HashSet;
use std::sync::Arc;

use asc_mcp::api::testutil::{MockApi, MockResponse};
use asc_mcp::api::Client;
use asc_mcp::mcp::{CallToolResult, Tool};
use asc_mcp::tools::{CallError, Handler, HandlerFuture, Registry};
use common::*;
use serde_json::json;

/// Every tool the Go registry advertised (Go: TestRegistry_ListTools).
const EXPECTED_TOOLS: &[&str] = &[
    // App tools
    "list_apps",
    "get_app",
    "get_app_versions",
    // Build tools
    "list_builds",
    "get_build",
    // TestFlight tools
    "list_beta_groups",
    "create_beta_group",
    "delete_beta_group",
    "list_beta_testers",
    "invite_beta_tester",
    "remove_beta_tester",
    "add_tester_to_group",
    // Provisioning tools
    "list_bundle_ids",
    "get_bundle_id",
    "list_certificates",
    "list_profiles",
    "list_devices",
    "register_device",
    // App Info Localization tools
    "get_app_infos",
    "list_app_info_localizations",
    "get_app_info_localization",
    "create_app_info_localization",
    "update_app_info_localization",
    "delete_app_info_localization",
    // Version Localization tools
    "list_version_localizations",
    "get_version_localization",
    "create_version_localization",
    "update_version_localization",
    "delete_version_localization",
    // Customer Reviews tools
    "list_customer_reviews",
    "get_customer_review",
    "create_customer_review_response",
    "delete_customer_review_response",
    // In-App Purchase tools
    "list_in_app_purchases",
    "get_in_app_purchase",
    "create_in_app_purchase",
    "update_in_app_purchase",
    "delete_in_app_purchase",
    // Subscription tools
    "list_subscription_groups",
    "get_subscription_group",
    "list_subscriptions",
    "get_subscription",
    // App Store Version tools
    "list_app_store_versions",
    "get_app_store_version",
    "create_app_store_version",
    "update_app_store_version",
    "delete_app_store_version",
    "submit_app_for_review",
    "list_review_submissions",
    "get_review_submission",
    "create_review_submission",
    "add_review_submission_item",
    "submit_review_submission",
    "cancel_review_submission",
    "get_app_store_review_detail",
    "create_app_store_review_detail",
    "update_app_store_review_detail",
    // Phased Release tools
    "get_phased_release",
    "create_phased_release",
    "update_phased_release",
    "delete_phased_release",
    // Screenshot tools
    "list_screenshot_sets",
    "list_screenshots",
    "get_screenshot",
    "delete_screenshot",
    "list_preview_sets",
    "list_previews",
    "get_preview",
    "delete_preview",
    // Pre-Order tools
    "end_app_availability_pre_order",
    // App Event tools
    "list_app_events",
    "get_app_event",
    "create_app_event",
    "update_app_event",
    "delete_app_event",
    // Analytics tools
    "list_analytics_report_requests",
    "get_analytics_report_request",
    "create_analytics_report_request",
    "delete_analytics_report_request",
    "list_analytics_reports",
    "list_analytics_report_instances",
    "list_analytics_report_segments",
    // App Clip tools
    "list_app_clips",
    "get_app_clip",
    "list_app_clip_default_experiences",
    "get_app_clip_default_experience",
    "list_app_clip_advanced_experiences",
    "get_app_clip_advanced_experience",
    // Game Center tools
    "get_game_center_detail",
    "list_game_center_achievements",
    "get_game_center_achievement",
    "create_game_center_achievement",
    "update_game_center_achievement",
    "delete_game_center_achievement",
    "list_game_center_leaderboards",
    "get_game_center_leaderboard",
    "create_game_center_leaderboard",
    "update_game_center_leaderboard",
    "delete_game_center_leaderboard",
    // Game Center leaderboard set tools
    "list_game_center_leaderboard_sets",
    "get_game_center_leaderboard_set",
    "create_game_center_leaderboard_set",
    "update_game_center_leaderboard_set",
    "delete_game_center_leaderboard_set",
    "list_game_center_leaderboard_set_members",
    "add_game_center_leaderboard_set_members",
    "remove_game_center_leaderboard_set_members",
    "list_game_center_leaderboard_set_versions",
    "get_game_center_leaderboard_set_version",
    "create_game_center_leaderboard_set_version",
    "list_game_center_leaderboard_set_localizations",
    "create_game_center_leaderboard_set_localization",
    "update_game_center_leaderboard_set_localization",
    "delete_game_center_leaderboard_set_localization",
    "upload_game_center_leaderboard_set_image",
    "list_game_center_leaderboard_set_member_localizations",
    "create_game_center_leaderboard_set_member_localization",
    "update_game_center_leaderboard_set_member_localization",
    "delete_game_center_leaderboard_set_member_localization",
    // Game Center activity tools
    "list_game_center_activities",
    "get_game_center_activity",
    "create_game_center_activity",
    "update_game_center_activity",
    "delete_game_center_activity",
    "list_game_center_activity_versions",
    "get_game_center_activity_version",
    "create_game_center_activity_version",
    "update_game_center_activity_version",
    "list_game_center_activity_localizations",
    "create_game_center_activity_localization",
    "update_game_center_activity_localization",
    "delete_game_center_activity_localization",
    "upload_game_center_activity_image",
    // Game Center challenge tools
    "list_game_center_challenges",
    "get_game_center_challenge",
    "create_game_center_challenge",
    "update_game_center_challenge",
    "delete_game_center_challenge",
    "list_game_center_challenge_versions",
    "get_game_center_challenge_version",
    "create_game_center_challenge_version",
    "list_game_center_challenge_localizations",
    "create_game_center_challenge_localization",
    "update_game_center_challenge_localization",
    "delete_game_center_challenge_localization",
    "upload_game_center_challenge_image",
    // Game Center player submission tools
    "submit_game_center_player_achievement",
    "submit_game_center_leaderboard_entry",
    // Xcode Cloud tools
    "list_ci_products",
    "get_ci_product",
    "list_ci_workflows",
    "get_ci_workflow",
    "list_ci_build_runs",
    "get_ci_build_run",
    "start_ci_build_run",
    // Reports tools
    "get_sales_report",
    "get_finance_report",
    // Encryption tools
    "list_encryption_declarations",
    "get_encryption_declaration",
    "create_encryption_declaration",
    "assign_build_to_encryption_declaration",
    // User tools
    "list_users",
    "get_user",
    "update_user",
    "delete_user",
    "list_user_invitations",
    "get_user_invitation",
    "create_user_invitation",
    "delete_user_invitation",
    // Pricing tools
    "get_app_price_schedule",
    "list_app_price_points",
    "list_territories",
    "list_subscription_price_points",
    // Availability tools
    "get_app_availability",
    "create_app_availability",
    "list_territory_availabilities",
    // Age Rating tools
    "get_age_rating_declaration",
    "update_age_rating_declaration",
    // Beta Review and Agreements tools
    "list_beta_app_review_submissions",
    "get_beta_app_review_submission",
    "create_beta_app_review_submission",
    "get_beta_license_agreement",
    "update_beta_license_agreement",
    "list_beta_app_localizations",
    "get_beta_app_localization",
    "create_beta_app_localization",
    "update_beta_app_localization",
    "delete_beta_app_localization",
    "list_beta_build_localizations",
    "get_beta_build_localization",
    "create_beta_build_localization",
    "update_beta_build_localization",
    "delete_beta_build_localization",
    "get_build_beta_detail",
    "update_build_beta_detail",
    // Sandbox Testers tools
    "list_sandbox_testers",
    "update_sandbox_tester",
    "clear_sandbox_tester_purchase_history",
    // Promoted Purchases tools
    "list_promoted_purchases",
    "get_promoted_purchase",
    "create_promoted_purchase",
    "update_promoted_purchase",
    "delete_promoted_purchase",
    "list_subscription_offer_codes",
    "get_subscription_offer_code",
    "create_subscription_offer_code",
    "update_subscription_offer_code",
    "list_win_back_offers",
    "get_win_back_offer",
    "create_win_back_offer",
    "update_win_back_offer",
    "delete_win_back_offer",
    // Product Pages tools
    "list_app_custom_product_pages",
    "get_app_custom_product_page",
    "create_app_custom_product_page",
    "update_app_custom_product_page",
    "delete_app_custom_product_page",
    "list_app_store_version_experiments",
    "get_app_store_version_experiment",
    "create_app_store_version_experiment",
    "update_app_store_version_experiment",
    "delete_app_store_version_experiment",
    // Diagnostics and Metrics tools
    "list_perf_power_metrics",
    "list_diagnostic_signatures",
    "list_diagnostic_logs",
    "list_app_store_review_attachments",
    "get_app_store_review_attachment",
    "create_app_store_review_attachment",
    "delete_app_store_review_attachment",
    "get_routing_app_coverage",
    "create_routing_app_coverage",
    "delete_routing_app_coverage",
    // EULA tools
    "get_end_user_license_agreement",
    "create_end_user_license_agreement",
    "update_end_user_license_agreement",
    "delete_end_user_license_agreement",
    // App Categories tools
    "list_app_categories",
    "get_app_category",
    // Alternative Distribution tools
    "list_alternative_distribution_keys",
    "get_alternative_distribution_key",
    "create_alternative_distribution_key",
    "delete_alternative_distribution_key",
    "get_alternative_distribution_package",
    // Marketplace Search tools
    "get_marketplace_search_detail",
    "create_marketplace_search_detail",
    "update_marketplace_search_detail",
    "delete_marketplace_search_detail",
    // Asset upload tools
    "upload_app_screenshot",
    "upload_app_preview",
    "upload_review_attachment",
    // Webhook tools
    "list_webhooks",
    "get_webhook",
    "create_webhook",
    "update_webhook",
    "delete_webhook",
    "list_webhook_deliveries",
    "ping_webhook",
    "redeliver_webhook_delivery",
    // Accessibility declaration tools
    "list_accessibility_declarations",
    "get_accessibility_declaration",
    "create_accessibility_declaration",
    "update_accessibility_declaration",
    "delete_accessibility_declaration",
    // Customer review summarization tools
    "list_customer_review_summarizations",
    // App tag tools
    "list_app_tags",
    "update_app_tag",
    "list_app_tag_territories",
    // Territory age rating tools
    "list_territory_age_ratings",
    // Android-to-iOS app mapping tools
    "list_android_to_ios_app_mappings",
    "get_android_to_ios_app_mapping",
    "create_android_to_ios_app_mapping",
    "update_android_to_ios_app_mapping",
    "delete_android_to_ios_app_mapping",
    // Build upload tools
    "start_build_upload",
    "upload_build_file",
    "get_build_upload",
    "list_build_uploads",
    "list_build_upload_files",
    "delete_build_upload",
    // Beta feedback tools
    "list_beta_feedback_crashes",
    "get_beta_feedback_crash",
    "get_beta_feedback_crash_log",
    "delete_beta_feedback_crash",
    "list_beta_feedback_screenshots",
    "get_beta_feedback_screenshot",
    "delete_beta_feedback_screenshot",
    "get_beta_build_usage_metrics",
    // Background asset tools
    "list_background_assets",
    "get_background_asset",
    "create_background_asset",
    "update_background_asset",
    "list_background_asset_versions",
    "get_background_asset_version",
    "create_background_asset_version",
    "upload_background_asset_file",
    "list_background_asset_upload_files",
    "get_background_asset_version_release",
    // Commerce version tools
    "create_in_app_purchase_version",
    "get_in_app_purchase_version",
    "list_in_app_purchase_versions",
    "list_in_app_purchase_version_localizations",
    "create_in_app_purchase_localization",
    "update_in_app_purchase_localization",
    "delete_in_app_purchase_localization",
    "list_in_app_purchase_version_images",
    "create_in_app_purchase_image",
    "update_in_app_purchase_image",
    "delete_in_app_purchase_image",
    "create_subscription_version",
    "get_subscription_version",
    "list_subscription_versions",
    "list_subscription_version_localizations",
    "create_subscription_localization",
    "update_subscription_localization",
    "delete_subscription_localization",
    "list_subscription_version_images",
    "create_subscription_image",
    "update_subscription_image",
    "delete_subscription_image",
    "create_subscription_group_version",
    "get_subscription_group_version",
    "list_subscription_group_versions",
    "list_subscription_group_version_localizations",
    "create_subscription_group_localization",
    "update_subscription_group_localization",
    "delete_subscription_group_localization",
    // In-app purchase offer code tools
    "list_in_app_purchase_offer_codes",
    "get_in_app_purchase_offer_code",
    "create_in_app_purchase_offer_code",
    "update_in_app_purchase_offer_code",
    "list_in_app_purchase_offer_code_prices",
    "list_in_app_purchase_offer_code_custom_codes",
    "create_in_app_purchase_offer_code_custom_code",
    "update_in_app_purchase_offer_code_custom_code",
    "list_in_app_purchase_offer_code_one_time_use_codes",
    "create_in_app_purchase_offer_code_one_time_use_code",
    "update_in_app_purchase_offer_code_one_time_use_code",
    "get_in_app_purchase_offer_code_one_time_use_code_values",
    // Commerce pricing and availability tools
    "list_in_app_purchase_price_points",
    "get_in_app_purchase_price_schedule",
    "set_in_app_purchase_price_schedule",
    "list_in_app_purchase_price_schedule_prices",
    "get_in_app_purchase_availability",
    "set_in_app_purchase_availability",
    "list_in_app_purchase_available_territories",
    "list_subscription_plan_availabilities",
    "get_subscription_plan_availability",
    "create_subscription_plan_availability",
    "update_subscription_plan_availability",
    "list_subscription_plan_available_territories",
    "get_subscription_price_point",
    "list_subscription_price_point_equalizations",
    "list_subscription_price_point_adjusted_equalizations",
    "set_app_price_schedule",
    "get_app_price_point",
    "list_app_price_point_equalizations",
];

fn key_client() -> (tempfile::TempDir, Arc<Client>) {
    let (dir, key) = write_test_key();
    let client = Client::new("test-issuer", "TESTKEY123", &key.to_string_lossy()).expect("client");
    (dir, Arc::new(client))
}

// Go: TestNewRegistry
#[test]
fn new_registry() {
    let (_dir, client) = key_client();
    let r = Registry::new(Some(client));
    assert!(!r.list_tools().is_empty(), "expected tools to be registered");
    assert!(r.get("list_apps").is_some());
    assert!(r.get("no_such_tool").is_none());
}

// Go: TestRegistry_ListTools
#[test]
fn list_tools() {
    let r = Registry::new(None);
    let tools = r.list_tools();
    // 221 base + 3 upload + 6 build upload + 8 beta feedback + 10
    // background asset + 49 Game Center content + 59 commerce tools.
    assert_eq!(tools.len(), 356);
    assert_eq!(EXPECTED_TOOLS.len(), 356);
    let expected: HashSet<&str> = EXPECTED_TOOLS.iter().copied().collect();
    let mut seen = HashSet::new();
    for t in tools {
        assert!(expected.contains(t.name.as_str()), "unexpected tool: {}", t.name);
        assert!(seen.insert(t.name.as_str()), "duplicate tool: {}", t.name);
        assert!(!t.name.is_empty(), "tool has empty name");
        assert!(!t.description().is_empty(), "tool {} has empty description", t.name);
        assert_eq!(t.input_schema()["type"], "object", "tool {} has invalid input schema type", t.name);
        assert_eq!(t.def["name"], t.name.as_str());
    }
    for name in EXPECTED_TOOLS {
        assert!(seen.contains(name), "missing expected tool: {name}");
    }
}

/// Registration order is deterministic (Go's NewRegistry order).
#[test]
fn list_tools_order_is_stable() {
    let a: Vec<String> = Registry::new(None).list_tools().iter().map(|t| t.name.clone()).collect();
    let b: Vec<String> = Registry::new(None).list_tools().iter().map(|t| t.name.clone()).collect();
    assert_eq!(a, b);
    assert_eq!(&a[..3], ["list_apps", "get_app", "get_app_versions"]);
}

// Go: TestRegistry_CallTool_UnknownTool
#[tokio::test]
async fn call_tool_unknown_tool() {
    let (_dir, client) = key_client();
    let r = Registry::new(Some(client));
    match r.call_tool("unknown_tool", Some(json!({}))).await {
        Err(CallError::UnknownTool(msg)) => assert!(msg.contains("unknown tool"), "error {msg:?}"),
        other => panic!("expected UnknownTool, got {other:?}"),
    }
}

/// Without a client, known tools fail as tool errors (the `tools`
/// subcommand builds a client-less registry only to list).
#[tokio::test]
async fn call_tool_without_client() {
    let r = Registry::new(None);
    assert_eq!(
        r.call_tool("list_apps", None).await,
        Err(CallError::Failed("no App Store Connect client configured".into()))
    );
}

// Go: TestRegistry_CallTool_MissingArgumentIsToolError. These handlers
// belong to the tool ports outside the protocol layer; run with
// `--ignored` once they are ported.
#[tokio::test]
#[ignore = "exercises tool handlers outside the protocol layer (other ports)"]
async fn call_tool_missing_argument_is_tool_error() {
    let (_dir, client) = key_client();
    let r = Registry::new(Some(client));
    for (tool, args, want) in [
        ("list_analytics_report_requests", json!({}), "app_id is required"),
        ("get_analytics_report_request", json!({}), "request_id is required"),
        ("create_analytics_report_request", json!({"app_id":"123"}), "access_type is required"),
        ("get_app_clip", json!({}), "app_clip_id is required"),
        ("get_age_rating_declaration", json!({}), "app_info_id is required"),
        ("start_build_upload", json!({"app_id":"123"}), "cf_bundle_short_version_string is required"),
        ("upload_build_file", json!({}), "build_upload_id is required"),
        ("get_beta_feedback_crash_log", json!({}), "submission_id is required"),
        ("get_beta_build_usage_metrics", json!({}), "build_id is required"),
        ("create_background_asset", json!({"app_id":"123"}), "asset_pack_identifier is required"),
        ("upload_background_asset_file", json!({}), "version_id is required"),
        ("update_background_asset", json!({"background_asset_id":"123"}), "archived is required"),
    ] {
        let res = r
            .call_tool(tool, Some(args))
            .await
            .unwrap_or_else(|e| panic!("{tool}: CallTool returned {e:?}, want isError result"));
        assert!(res.is_error, "{tool}: want isError result, got {res:?}");
        assert_eq!(res.text(), want, "{tool}");
    }
}

/// The same convention, pinned now on the already-ported app tools.
#[tokio::test]
async fn call_tool_missing_argument_is_tool_error_app_tools() {
    let mock = MockApi::start(|req| panic!("no API call expected, got {}", req.path)).await;
    let r = Registry::new(Some(Arc::new(mock.client())));
    for tool in ["get_app", "get_app_versions"] {
        let res = r.call_tool(tool, Some(json!({}))).await.unwrap_or_else(|e| panic!("{tool}: {e:?}"));
        assert!(res.is_error, "{tool}: want isError result, got {res:?}");
        assert_eq!(res.text(), "app_id is required", "{tool}");
    }
    assert!(mock.requests().is_empty());
}

/// A real handler end to end against the mock API (Go's skipped
/// TestHandleListApps_Integration).
#[tokio::test]
async fn call_tool_list_apps_against_mock() {
    let mock = MockApi::start(|req| {
        assert_eq!(req.path, "/v1/apps");
        MockResponse::json(
            200,
            r#"{"data":[{"type":"apps","id":"1","attributes":{"name":"Demo","bundleId":"com.example.demo"}}],"links":{}}"#,
        )
    })
    .await;
    let r = Registry::new(Some(Arc::new(mock.client())));
    let res = r.call_tool("list_apps", Some(json!({"limit": 5}))).await.expect("call");
    assert!(!res.is_error, "{res:?}");
    assert!(res.text().contains("Demo"), "{}", res.text());
    assert_eq!(mock.requests()[0].query_param("limit").as_deref(), Some("5"));
}

// Go: TestRegistry_Register
#[tokio::test]
async fn register() {
    let mut r = Registry::empty(Some(dummy_client()));
    let tool = Tool::from_json(r#"{"name":"custom_tool","description":"A custom test tool","inputSchema":{"type":"object"}}"#);
    let handler: Handler =
        Arc::new(|_c, _a| Box::pin(async { Ok(CallToolResult::success("custom result")) }) as HandlerFuture);
    r.register(tool, handler);
    assert_eq!(r.list_tools().len(), 1);
    assert_eq!(r.list_tools()[0].name, "custom_tool");
    assert!(r.get("custom_tool").is_some(), "handler not registered");
    let res = r.call_tool("custom_tool", Some(json!({}))).await.expect("call");
    assert_eq!(res.text(), "custom result");
}

// Go: TestToolInputSchemas
#[test]
fn tool_input_schemas() {
    let r = Registry::new(None);
    let cases: &[(&str, &[&str])] = &[
        ("list_apps", &[]),
        ("get_app", &["app_id"]),
        ("get_app_versions", &["app_id"]),
        ("list_builds", &[]),
        ("get_build", &["build_id"]),
        ("create_beta_group", &["app_id", "name"]),
        ("delete_beta_group", &["beta_group_id"]),
        ("invite_beta_tester", &["email"]),
        ("remove_beta_tester", &["beta_tester_id"]),
        ("add_tester_to_group", &["beta_tester_id", "beta_group_id"]),
        ("get_bundle_id", &["bundle_id_id"]),
        ("register_device", &["name", "udid", "platform"]),
    ];
    for (name, required) in cases {
        let t = r.get(name).unwrap_or_else(|| panic!("missing tool {name}"));
        if required.is_empty() {
            continue;
        }
        let have: Vec<&str> = t.input_schema()["required"]
            .as_array()
            .unwrap_or_else(|| panic!("tool {name} should have required fields"))
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        for key in *required {
            assert!(have.contains(key), "tool {name} missing required field: {key}");
        }
    }
}
