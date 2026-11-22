// App Store Connect API endpoints for the "platform" area.
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
    /// CreateAlternativeDistributionKey creates an alternative distribution key.
    pub async fn create_alternative_distribution_key(&self, req: &AlternativeDistributionKeyCreateRequest) -> Result<AlternativeDistributionKeyResponse> {
        todo!("port Client.CreateAlternativeDistributionKey from internal/asc/api/client.go")
    }

    /// CreateAnalyticsReportRequest creates an analytics report request.
    pub async fn create_analytics_report_request(&self, req: &AnalyticsReportRequestCreateRequest) -> Result<AnalyticsReportRequestResponse> {
        todo!("port Client.CreateAnalyticsReportRequest from internal/asc/api/client.go")
    }

    /// CreateAppStoreReviewAttachment creates a review attachment.
    pub async fn create_app_store_review_attachment(&self, req: &AppStoreReviewAttachmentCreateRequest) -> Result<AppStoreReviewAttachmentResponse> {
        todo!("port Client.CreateAppStoreReviewAttachment from internal/asc/api/client.go")
    }

    /// CreateEndUserLicenseAgreement creates an EULA.
    pub async fn create_end_user_license_agreement(&self, req: &EndUserLicenseAgreementCreateRequest) -> Result<EndUserLicenseAgreementResponse> {
        todo!("port Client.CreateEndUserLicenseAgreement from internal/asc/api/client.go")
    }

    /// CreateMarketplaceSearchDetail creates marketplace search details.
    pub async fn create_marketplace_search_detail(&self, req: &MarketplaceSearchDetailCreateRequest) -> Result<MarketplaceSearchDetailResponse> {
        todo!("port Client.CreateMarketplaceSearchDetail from internal/asc/api/client.go")
    }

    /// CreateRoutingAppCoverage creates routing app coverage.
    pub async fn create_routing_app_coverage(&self, req: &RoutingAppCoverageCreateRequest) -> Result<RoutingAppCoverageResponse> {
        todo!("port Client.CreateRoutingAppCoverage from internal/asc/api/client.go")
    }

    /// DeleteAlternativeDistributionKey deletes an alternative distribution key.
    pub async fn delete_alternative_distribution_key(&self, key_id: &str) -> Result<()> {
        todo!("port Client.DeleteAlternativeDistributionKey from internal/asc/api/client.go")
    }

    /// DeleteAnalyticsReportRequest deletes an analytics report request.
    pub async fn delete_analytics_report_request(&self, request_id: &str) -> Result<()> {
        todo!("port Client.DeleteAnalyticsReportRequest from internal/asc/api/client.go")
    }

    /// DeleteAppStoreReviewAttachment deletes a review attachment.
    pub async fn delete_app_store_review_attachment(&self, attachment_id: &str) -> Result<()> {
        todo!("port Client.DeleteAppStoreReviewAttachment from internal/asc/api/client.go")
    }

    /// DeleteEndUserLicenseAgreement deletes an EULA.
    pub async fn delete_end_user_license_agreement(&self, agreement_id: &str) -> Result<()> {
        todo!("port Client.DeleteEndUserLicenseAgreement from internal/asc/api/client.go")
    }

    /// DeleteMarketplaceSearchDetail deletes marketplace search details.
    pub async fn delete_marketplace_search_detail(&self, detail_id: &str) -> Result<()> {
        todo!("port Client.DeleteMarketplaceSearchDetail from internal/asc/api/client.go")
    }

    /// DeleteRoutingAppCoverage deletes routing app coverage.
    pub async fn delete_routing_app_coverage(&self, coverage_id: &str) -> Result<()> {
        todo!("port Client.DeleteRoutingAppCoverage from internal/asc/api/client.go")
    }

    /// GetAlternativeDistributionKey returns a single alternative distribution key.
    pub async fn get_alternative_distribution_key(&self, key_id: &str) -> Result<AlternativeDistributionKeyResponse> {
        todo!("port Client.GetAlternativeDistributionKey from internal/asc/api/client.go")
    }

    /// GetAlternativeDistributionPackageForVersion returns the alternative
    /// distribution package for an app store version.
    pub async fn get_alternative_distribution_package_for_version(&self, version_id: &str) -> Result<AlternativeDistributionPackageResponse> {
        todo!("port Client.GetAlternativeDistributionPackageForVersion from internal/asc/api/client.go")
    }

    /// GetAnalyticsReportRequest returns a single analytics report request.
    pub async fn get_analytics_report_request(&self, request_id: &str) -> Result<AnalyticsReportRequestResponse> {
        todo!("port Client.GetAnalyticsReportRequest from internal/asc/api/client.go")
    }

    /// GetAppCategory returns a single app category.
    pub async fn get_app_category(&self, category_id: &str) -> Result<AppCategoryResponse> {
        todo!("port Client.GetAppCategory from internal/asc/api/client.go")
    }

    /// GetAppStoreReviewAttachment returns a single review attachment.
    pub async fn get_app_store_review_attachment(&self, attachment_id: &str) -> Result<AppStoreReviewAttachmentResponse> {
        todo!("port Client.GetAppStoreReviewAttachment from internal/asc/api/client.go")
    }

    /// GetEndUserLicenseAgreement returns an EULA.
    pub async fn get_end_user_license_agreement(&self, app_id: &str) -> Result<EndUserLicenseAgreementResponse> {
        todo!("port Client.GetEndUserLicenseAgreement from internal/asc/api/client.go")
    }

    /// GetFinanceReport returns finance reports.
    pub async fn get_finance_report(&self, vendor_number: &str, region_code: &str, report_type: &str, report_date: &str) -> Result<Vec<u8>> {
        todo!("port Client.GetFinanceReport from internal/asc/api/client.go")
    }

    /// GetMarketplaceSearchDetail returns marketplace search details.
    pub async fn get_marketplace_search_detail(&self, app_id: &str) -> Result<MarketplaceSearchDetailResponse> {
        todo!("port Client.GetMarketplaceSearchDetail from internal/asc/api/client.go")
    }

    /// GetRoutingAppCoverage returns routing app coverage.
    pub async fn get_routing_app_coverage(&self, version_id: &str) -> Result<RoutingAppCoverageResponse> {
        todo!("port Client.GetRoutingAppCoverage from internal/asc/api/client.go")
    }

    /// GetSalesReport returns sales reports.
    pub async fn get_sales_report(&self, vendor_number: &str, report_type: &str, report_sub_type: &str, frequency: &str, report_date: &str) -> Result<Vec<u8>> {
        todo!("port Client.GetSalesReport from internal/asc/api/client.go")
    }

    /// ListAlternativeDistributionKeys returns alternative distribution keys.
    pub async fn list_alternative_distribution_keys(&self, opts: Option<&ListOptions>) -> Result<AlternativeDistributionKeysResponse> {
        todo!("port Client.ListAlternativeDistributionKeys from internal/asc/api/client.go")
    }

    /// ListAnalyticsReportInstances returns instances for a report.
    pub async fn list_analytics_report_instances(&self, report_id: &str, opts: Option<&ListOptions>) -> Result<AnalyticsReportInstancesResponse> {
        todo!("port Client.ListAnalyticsReportInstances from internal/asc/api/client.go")
    }

    /// ListAnalyticsReportRequests returns analytics report requests for an app.
    pub async fn list_analytics_report_requests(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<AnalyticsReportRequestsResponse> {
        todo!("port Client.ListAnalyticsReportRequests from internal/asc/api/client.go")
    }

    /// ListAnalyticsReportSegments returns segments for a report instance.
    pub async fn list_analytics_report_segments(&self, instance_id: &str, opts: Option<&ListOptions>) -> Result<AnalyticsReportSegmentsResponse> {
        todo!("port Client.ListAnalyticsReportSegments from internal/asc/api/client.go")
    }

    /// ListAnalyticsReports returns analytics reports for a request.
    pub async fn list_analytics_reports(&self, request_id: &str, opts: Option<&ListOptions>) -> Result<AnalyticsReportsResponse> {
        todo!("port Client.ListAnalyticsReports from internal/asc/api/client.go")
    }

    /// ListAppCategories returns all app categories.
    pub async fn list_app_categories(&self, opts: Option<&ListOptions>) -> Result<AppCategoriesResponse> {
        todo!("port Client.ListAppCategories from internal/asc/api/client.go")
    }

    /// ListAppStoreReviewAttachments returns review attachments.
    pub async fn list_app_store_review_attachments(&self, review_detail_id: &str, opts: Option<&ListOptions>) -> Result<AppStoreReviewAttachmentsResponse> {
        todo!("port Client.ListAppStoreReviewAttachments from internal/asc/api/client.go")
    }

    /// ListBuildPerfPowerMetrics returns performance metrics for a build.
    pub async fn list_build_perf_power_metrics(&self, build_id: &str, opts: Option<&ListOptions>) -> Result<PerfPowerMetricsResponse> {
        todo!("port Client.ListBuildPerfPowerMetrics from internal/asc/api/client.go")
    }

    /// ListDiagnosticLogs returns diagnostic logs.
    pub async fn list_diagnostic_logs(&self, signature_id: &str, opts: Option<&ListOptions>) -> Result<DiagnosticLogsResponse> {
        todo!("port Client.ListDiagnosticLogs from internal/asc/api/client.go")
    }

    /// ListDiagnosticSignatures returns diagnostic signatures.
    pub async fn list_diagnostic_signatures(&self, build_id: &str, opts: Option<&ListOptions>) -> Result<DiagnosticSignaturesResponse> {
        todo!("port Client.ListDiagnosticSignatures from internal/asc/api/client.go")
    }

    /// ListPerfPowerMetrics returns performance and power metrics.
    pub async fn list_perf_power_metrics(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<PerfPowerMetricsResponse> {
        todo!("port Client.ListPerfPowerMetrics from internal/asc/api/client.go")
    }

    /// UpdateEndUserLicenseAgreement updates an EULA.
    pub async fn update_end_user_license_agreement(&self, agreement_id: &str, req: &EndUserLicenseAgreementUpdateRequest) -> Result<EndUserLicenseAgreementResponse> {
        todo!("port Client.UpdateEndUserLicenseAgreement from internal/asc/api/client.go")
    }

    /// UpdateMarketplaceSearchDetail updates marketplace search details.
    pub async fn update_marketplace_search_detail(&self, detail_id: &str, req: &MarketplaceSearchDetailUpdateRequest) -> Result<MarketplaceSearchDetailResponse> {
        todo!("port Client.UpdateMarketplaceSearchDetail from internal/asc/api/client.go")
    }

    /// CreateBackgroundAsset creates a background asset for an app.
    pub async fn create_background_asset(&self, req: &BackgroundAssetCreateRequest) -> Result<BackgroundAssetResponse> {
        todo!("port Client.CreateBackgroundAsset from internal/asc/api/client_background_assets.go")
    }

    /// CreateBackgroundAssetUploadFile reserves an upload file slot on a
    /// background asset version. The response carries the upload operations
    /// for the file's bytes.
    pub async fn create_background_asset_upload_file(&self, req: &BackgroundAssetUploadFileCreateRequest) -> Result<BackgroundAssetUploadFileResponse> {
        todo!("port Client.CreateBackgroundAssetUploadFile from internal/asc/api/client_background_assets.go")
    }

    /// CreateBackgroundAssetVersion creates a new version on a background
    /// asset. The version number is assigned by Apple.
    pub async fn create_background_asset_version(&self, req: &BackgroundAssetVersionCreateRequest) -> Result<BackgroundAssetVersionResponse> {
        todo!("port Client.CreateBackgroundAssetVersion from internal/asc/api/client_background_assets.go")
    }

    /// GetBackgroundAsset returns a single background asset.
    pub async fn get_background_asset(&self, background_asset_id: &str, opts: Option<&ListOptions>) -> Result<BackgroundAssetResponse> {
        todo!("port Client.GetBackgroundAsset from internal/asc/api/client_background_assets.go")
    }

    /// GetBackgroundAssetVersion returns a single background asset version.
    pub async fn get_background_asset_version(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<BackgroundAssetVersionResponse> {
        todo!("port Client.GetBackgroundAssetVersion from internal/asc/api/client_background_assets.go")
    }

    /// GetBackgroundAssetVersionAppStoreRelease returns the App Store
    /// release record for a background asset version.
    pub async fn get_background_asset_version_app_store_release(&self, release_id: &str) -> Result<BackgroundAssetVersionReleaseResponse> {
        todo!("port Client.GetBackgroundAssetVersionAppStoreRelease from internal/asc/api/client_background_assets.go")
    }

    /// GetBackgroundAssetVersionExternalBetaRelease returns the external
    /// beta (TestFlight) release record for a background asset version.
    pub async fn get_background_asset_version_external_beta_release(&self, release_id: &str) -> Result<BackgroundAssetVersionReleaseResponse> {
        todo!("port Client.GetBackgroundAssetVersionExternalBetaRelease from internal/asc/api/client_background_assets.go")
    }

    /// GetBackgroundAssetVersionInternalBetaRelease returns the internal
    /// beta release record for a background asset version.
    pub async fn get_background_asset_version_internal_beta_release(&self, release_id: &str) -> Result<BackgroundAssetVersionReleaseResponse> {
        todo!("port Client.GetBackgroundAssetVersionInternalBetaRelease from internal/asc/api/client_background_assets.go")
    }

    /// ListBackgroundAssetUploadFiles returns the upload files (asset pack
    /// and manifest) attached to a background asset version, including their
    /// delivery states.
    pub async fn list_background_asset_upload_files(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<BackgroundAssetUploadFilesResponse> {
        todo!("port Client.ListBackgroundAssetUploadFiles from internal/asc/api/client_background_assets.go")
    }

    /// ListBackgroundAssetVersions returns the versions of a background
    /// asset, including their processing states and release states.
    pub async fn list_background_asset_versions(&self, background_asset_id: &str, opts: Option<&ListOptions>) -> Result<BackgroundAssetVersionsResponse> {
        todo!("port Client.ListBackgroundAssetVersions from internal/asc/api/client_background_assets.go")
    }

    /// ListBackgroundAssets returns the background assets for an app.
    /// Supports filter[versions.locale] and filter[versions.platforms] for
    /// the multi-locale read added in API 4.4.
    pub async fn list_background_assets(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<BackgroundAssetsResponse> {
        todo!("port Client.ListBackgroundAssets from internal/asc/api/client_background_assets.go")
    }

    /// UpdateBackgroundAsset updates a background asset (archives or
    /// unarchives it).
    pub async fn update_background_asset(&self, background_asset_id: &str, req: &BackgroundAssetUpdateRequest) -> Result<BackgroundAssetResponse> {
        todo!("port Client.UpdateBackgroundAsset from internal/asc/api/client_background_assets.go")
    }

    /// UpdateBackgroundAssetUploadFile commits (or amends) a background
    /// asset upload file.
    pub async fn update_background_asset_upload_file(&self, file_id: &str, req: &BackgroundAssetUploadFileUpdateRequest) -> Result<BackgroundAssetUploadFileResponse> {
        todo!("port Client.UpdateBackgroundAssetUploadFile from internal/asc/api/client_background_assets.go")
    }

    /// UploadBackgroundAssetFile reserves, uploads, and commits one file on
    /// an existing background asset version. assetType is ASSET (the asset
    /// pack archive) or MANIFEST (its manifest plist). The commit carries
    /// the SHA-256 of the body so Apple can verify the stored bytes.
    pub async fn upload_background_asset_file(&self, version_id: &str, file_name: &str, asset_type: &str, body: &[u8]) -> Result<BackgroundAssetUploadFileResponse> {
        todo!("port Client.UploadBackgroundAssetFile from internal/asc/api/client_background_assets.go")
    }

    /// CreateAccessibilityDeclaration creates an accessibility declaration for an
    /// app and device family.
    pub async fn create_accessibility_declaration(&self, req: &AccessibilityDeclarationCreateRequest) -> Result<AccessibilityDeclarationResponse> {
        todo!("port Client.CreateAccessibilityDeclaration from internal/asc/api/client_metadata_extras.go")
    }

    /// DeleteAccessibilityDeclaration deletes an accessibility declaration.
    pub async fn delete_accessibility_declaration(&self, declaration_id: &str) -> Result<()> {
        todo!("port Client.DeleteAccessibilityDeclaration from internal/asc/api/client_metadata_extras.go")
    }

    /// GetAccessibilityDeclaration returns a single accessibility declaration.
    pub async fn get_accessibility_declaration(&self, declaration_id: &str) -> Result<AccessibilityDeclarationResponse> {
        todo!("port Client.GetAccessibilityDeclaration from internal/asc/api/client_metadata_extras.go")
    }

    /// ListAccessibilityDeclarations returns the accessibility declarations for an app.
    pub async fn list_accessibility_declarations(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<AccessibilityDeclarationsResponse> {
        todo!("port Client.ListAccessibilityDeclarations from internal/asc/api/client_metadata_extras.go")
    }

    /// UpdateAccessibilityDeclaration updates an accessibility declaration.
    /// Setting publish=true in the request publishes the draft declaration.
    pub async fn update_accessibility_declaration(&self, declaration_id: &str, req: &AccessibilityDeclarationUpdateRequest) -> Result<AccessibilityDeclarationResponse> {
        todo!("port Client.UpdateAccessibilityDeclaration from internal/asc/api/client_metadata_extras.go")
    }

    /// CreateWebhook creates a webhook for an app.
    pub async fn create_webhook(&self, req: &WebhookCreateRequest) -> Result<WebhookResponse> {
        todo!("port Client.CreateWebhook from internal/asc/api/client_webhooks.go")
    }

    /// CreateWebhookPing sends a test ping event to a webhook endpoint.
    pub async fn create_webhook_ping(&self, req: &WebhookPingCreateRequest) -> Result<WebhookPingResponse> {
        todo!("port Client.CreateWebhookPing from internal/asc/api/client_webhooks.go")
    }

    /// CreateWebhookRedelivery requests redelivery of a webhook event using a
    /// previous delivery as the template.
    pub async fn create_webhook_redelivery(&self, req: &WebhookDeliveryCreateRequest) -> Result<WebhookDeliveryResponse> {
        todo!("port Client.CreateWebhookRedelivery from internal/asc/api/client_webhooks.go")
    }

    /// DeleteWebhook deletes a webhook.
    pub async fn delete_webhook(&self, webhook_id: &str) -> Result<()> {
        todo!("port Client.DeleteWebhook from internal/asc/api/client_webhooks.go")
    }

    /// GetWebhook returns a single webhook.
    pub async fn get_webhook(&self, webhook_id: &str) -> Result<WebhookResponse> {
        todo!("port Client.GetWebhook from internal/asc/api/client_webhooks.go")
    }

    /// ListWebhookDeliveries returns delivery attempts for a webhook.
    pub async fn list_webhook_deliveries(&self, webhook_id: &str, opts: Option<&ListOptions>) -> Result<WebhookDeliveriesResponse> {
        todo!("port Client.ListWebhookDeliveries from internal/asc/api/client_webhooks.go")
    }

    /// ListWebhooks returns the webhooks configured for an app.
    pub async fn list_webhooks(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<WebhooksResponse> {
        todo!("port Client.ListWebhooks from internal/asc/api/client_webhooks.go")
    }

    /// UpdateWebhook updates a webhook's configuration.
    pub async fn update_webhook(&self, webhook_id: &str, req: &WebhookUpdateRequest) -> Result<WebhookResponse> {
        todo!("port Client.UpdateWebhook from internal/asc/api/client_webhooks.go")
    }
}
