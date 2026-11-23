// @generated registration order.

use super::Registry;

/// Registers every built-in tool in the canonical order (the order
/// `tools/list` reports).
pub(super) fn register_all(r: &mut Registry) {
    super::apps::register_app_tools(r);
    super::builds::register_build_tools(r);
    super::testflight::register_test_flight_tools(r);
    super::provisioning::register_provisioning_tools(r);
    super::localizations::register_app_info_localization_tools(r);
    super::localizations::register_version_localization_tools(r);
    super::reviews::register_customer_review_tools(r);
    super::iap::register_in_app_purchase_tools(r);
    super::subscriptions::register_subscription_tools(r);
    super::commerce_versions::register_commerce_version_tools(r);
    super::iap_offer_codes::register_in_app_purchase_offer_code_tools(r);
    super::commerce_pricing::register_commerce_pricing_tools(r);
    super::versions::register_version_submission_tools(r);
    super::review_submissions::register_review_submission_tools(r);
    super::phased_release::register_phased_release_tools(r);
    super::screenshots::register_screenshot_tools(r);
    super::preorders::register_pre_order_tools(r);
    super::events::register_app_event_tools(r);
    super::analytics::register_analytics_tools(r);
    super::appclips::register_app_clip_tools(r);
    super::gamecenter::register_game_center_tools(r);
    super::gamecenter_content::register_game_center_content_tools(r);
    super::xcode_cloud::register_xcode_cloud_tools(r);
    super::reports::register_reports_tools(r);
    super::encryption::register_encryption_tools(r);
    super::users::register_user_tools(r);
    super::pricing::register_pricing_tools(r);
    super::availability::register_availability_tools(r);
    super::age_rating::register_age_rating_tools(r);
    super::beta_review::register_beta_review_tools(r);
    super::sandbox::register_sandbox_tools(r);
    super::promoted_purchases::register_promoted_purchases_tools(r);
    super::product_pages::register_product_pages_tools(r);
    super::diagnostics::register_diagnostics_tools(r);
    super::misc::register_misc_tools(r);
    super::webhooks::register_webhook_tools(r);
    super::accessibility::register_accessibility_tools(r);
    super::app_tags::register_app_tag_tools(r);
    super::android_mapping::register_android_mapping_tools(r);
    super::uploads::register_upload_tools(r);
    super::build_uploads::register_build_upload_tools(r);
    super::beta_feedback::register_beta_feedback_tools(r);
    super::background_assets::register_background_asset_tools(r);
}
