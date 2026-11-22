// App Store Connect API endpoints for the "builds" area.
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
    /// AddBetaTesterToGroup adds a beta tester to a group.
    pub async fn add_beta_tester_to_group(&self, beta_group_id: &str, beta_tester_id: &str) -> Result<()> {
        todo!("port Client.AddBetaTesterToGroup from internal/asc/api/client.go")
    }

    /// CreateBetaAppLocalization creates a beta app localization.
    pub async fn create_beta_app_localization(&self, req: &BetaAppLocalizationCreateRequest) -> Result<BetaAppLocalizationResponse> {
        todo!("port Client.CreateBetaAppLocalization from internal/asc/api/client.go")
    }

    /// CreateBetaAppReviewSubmission submits a build for beta app review.
    pub async fn create_beta_app_review_submission(&self, req: &BetaAppReviewSubmissionCreateRequest) -> Result<BetaAppReviewSubmissionResponse> {
        todo!("port Client.CreateBetaAppReviewSubmission from internal/asc/api/client.go")
    }

    /// CreateBetaBuildLocalization creates a beta build localization.
    pub async fn create_beta_build_localization(&self, req: &BetaBuildLocalizationCreateRequest) -> Result<BetaBuildLocalizationResponse> {
        todo!("port Client.CreateBetaBuildLocalization from internal/asc/api/client.go")
    }

    /// CreateBetaGroup creates a new beta group.
    pub async fn create_beta_group(&self, req: &BetaGroupCreateRequest) -> Result<BetaGroupResponse> {
        todo!("port Client.CreateBetaGroup from internal/asc/api/client.go")
    }

    /// CreateBetaTester invites a new beta tester.
    pub async fn create_beta_tester(&self, req: &BetaTesterCreateRequest) -> Result<BetaTesterResponse> {
        todo!("port Client.CreateBetaTester from internal/asc/api/client.go")
    }

    /// CreateCustomerReviewResponse creates a response to a customer review.
    pub async fn create_customer_review_response(&self, req: &CustomerReviewResponseCreateRequest) -> Result<CustomerReviewResponseV1Response> {
        todo!("port Client.CreateCustomerReviewResponse from internal/asc/api/client.go")
    }

    /// DeleteBetaAppLocalization deletes a beta app localization.
    pub async fn delete_beta_app_localization(&self, localization_id: &str) -> Result<()> {
        todo!("port Client.DeleteBetaAppLocalization from internal/asc/api/client.go")
    }

    /// DeleteBetaBuildLocalization deletes a beta build localization.
    pub async fn delete_beta_build_localization(&self, localization_id: &str) -> Result<()> {
        todo!("port Client.DeleteBetaBuildLocalization from internal/asc/api/client.go")
    }

    /// DeleteBetaGroup deletes a beta group.
    pub async fn delete_beta_group(&self, beta_group_id: &str) -> Result<()> {
        todo!("port Client.DeleteBetaGroup from internal/asc/api/client.go")
    }

    /// DeleteBetaTester removes a beta tester.
    pub async fn delete_beta_tester(&self, beta_tester_id: &str) -> Result<()> {
        todo!("port Client.DeleteBetaTester from internal/asc/api/client.go")
    }

    /// DeleteCustomerReviewResponse deletes a customer review response.
    pub async fn delete_customer_review_response(&self, response_id: &str) -> Result<()> {
        todo!("port Client.DeleteCustomerReviewResponse from internal/asc/api/client.go")
    }

    /// GetBetaAppLocalization returns a single beta app localization.
    pub async fn get_beta_app_localization(&self, localization_id: &str) -> Result<BetaAppLocalizationResponse> {
        todo!("port Client.GetBetaAppLocalization from internal/asc/api/client.go")
    }

    /// GetBetaAppReviewSubmission returns a single beta app review submission.
    pub async fn get_beta_app_review_submission(&self, submission_id: &str) -> Result<BetaAppReviewSubmissionResponse> {
        todo!("port Client.GetBetaAppReviewSubmission from internal/asc/api/client.go")
    }

    /// GetBetaBuildLocalization returns a single beta build localization.
    pub async fn get_beta_build_localization(&self, localization_id: &str) -> Result<BetaBuildLocalizationResponse> {
        todo!("port Client.GetBetaBuildLocalization from internal/asc/api/client.go")
    }

    /// GetBetaLicenseAgreement returns a single beta license agreement.
    pub async fn get_beta_license_agreement(&self, agreement_id: &str) -> Result<BetaLicenseAgreementResponse> {
        todo!("port Client.GetBetaLicenseAgreement from internal/asc/api/client.go")
    }

    /// GetBuild returns a single build by ID.
    pub async fn get_build(&self, build_id: &str) -> Result<BuildResponse> {
        todo!("port Client.GetBuild from internal/asc/api/client.go")
    }

    /// GetBuildBetaDetail returns build beta detail.
    pub async fn get_build_beta_detail(&self, build_id: &str) -> Result<BuildBetaDetailResponse> {
        todo!("port Client.GetBuildBetaDetail from internal/asc/api/client.go")
    }

    /// GetBundleID returns a single bundle ID by ID.
    pub async fn get_bundle_id(&self, bundle_id_id: &str) -> Result<BundleIDResponse> {
        todo!("port Client.GetBundleID from internal/asc/api/client.go")
    }

    /// GetCiBuildRun returns a single build run.
    pub async fn get_ci_build_run(&self, build_run_id: &str) -> Result<CiBuildRunResponse> {
        todo!("port Client.GetCiBuildRun from internal/asc/api/client.go")
    }

    /// GetCiProduct returns a single Xcode Cloud product.
    pub async fn get_ci_product(&self, product_id: &str) -> Result<CiProductResponse> {
        todo!("port Client.GetCiProduct from internal/asc/api/client.go")
    }

    /// GetCiWorkflow returns a single workflow.
    pub async fn get_ci_workflow(&self, workflow_id: &str) -> Result<CiWorkflowResponse> {
        todo!("port Client.GetCiWorkflow from internal/asc/api/client.go")
    }

    /// GetCustomerReview returns a single customer review by ID.
    pub async fn get_customer_review(&self, review_id: &str) -> Result<CustomerReviewResponse> {
        todo!("port Client.GetCustomerReview from internal/asc/api/client.go")
    }

    /// GetProfile returns a single profile by ID.
    pub async fn get_profile(&self, profile_id: &str) -> Result<ProfileResponse> {
        todo!("port Client.GetProfile from internal/asc/api/client.go")
    }

    /// ListBetaAppLocalizations returns beta app localizations.
    pub async fn list_beta_app_localizations(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<BetaAppLocalizationsResponse> {
        todo!("port Client.ListBetaAppLocalizations from internal/asc/api/client.go")
    }

    /// ListBetaAppReviewSubmissions returns a list of beta app review submissions.
    pub async fn list_beta_app_review_submissions(&self, opts: Option<&ListOptions>) -> Result<BetaAppReviewSubmissionsResponse> {
        todo!("port Client.ListBetaAppReviewSubmissions from internal/asc/api/client.go")
    }

    /// ListBetaBuildLocalizations returns beta build localizations.
    pub async fn list_beta_build_localizations(&self, build_id: &str, opts: Option<&ListOptions>) -> Result<BetaBuildLocalizationsResponse> {
        todo!("port Client.ListBetaBuildLocalizations from internal/asc/api/client.go")
    }

    /// ListBetaGroups returns a list of beta groups.
    pub async fn list_beta_groups(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<BetaGroupsResponse> {
        todo!("port Client.ListBetaGroups from internal/asc/api/client.go")
    }

    /// ListBetaLicenseAgreements returns a list of beta license agreements.
    pub async fn list_beta_license_agreements(&self, opts: Option<&ListOptions>) -> Result<BetaLicenseAgreementsResponse> {
        todo!("port Client.ListBetaLicenseAgreements from internal/asc/api/client.go")
    }

    /// ListBetaTesters returns a list of beta testers.
    pub async fn list_beta_testers(&self, beta_group_id: &str, opts: Option<&ListOptions>) -> Result<BetaTestersResponse> {
        todo!("port Client.ListBetaTesters from internal/asc/api/client.go")
    }

    /// ListBuilds returns a list of builds.
    pub async fn list_builds(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<BuildsResponse> {
        todo!("port Client.ListBuilds from internal/asc/api/client.go")
    }

    /// ListBundleIDs returns a list of bundle IDs.
    pub async fn list_bundle_ids(&self, opts: Option<&ListOptions>) -> Result<BundleIDsResponse> {
        todo!("port Client.ListBundleIDs from internal/asc/api/client.go")
    }

    /// ListCertificates returns a list of certificates.
    pub async fn list_certificates(&self, opts: Option<&ListOptions>) -> Result<CertificatesResponse> {
        todo!("port Client.ListCertificates from internal/asc/api/client.go")
    }

    /// ListCiBuildRuns returns build runs for a workflow.
    pub async fn list_ci_build_runs(&self, workflow_id: &str, opts: Option<&ListOptions>) -> Result<CiBuildRunsResponse> {
        todo!("port Client.ListCiBuildRuns from internal/asc/api/client.go")
    }

    /// ListCiProducts returns Xcode Cloud products for an app.
    pub async fn list_ci_products(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<CiProductsResponse> {
        todo!("port Client.ListCiProducts from internal/asc/api/client.go")
    }

    /// ListCiWorkflows returns workflows for a product.
    pub async fn list_ci_workflows(&self, product_id: &str, opts: Option<&ListOptions>) -> Result<CiWorkflowsResponse> {
        todo!("port Client.ListCiWorkflows from internal/asc/api/client.go")
    }

    /// ListCustomerReviews returns customer reviews for an app.
    pub async fn list_customer_reviews(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<CustomerReviewsResponse> {
        todo!("port Client.ListCustomerReviews from internal/asc/api/client.go")
    }

    /// ListDevices returns a list of devices.
    pub async fn list_devices(&self, opts: Option<&ListOptions>) -> Result<DevicesResponse> {
        todo!("port Client.ListDevices from internal/asc/api/client.go")
    }

    /// ListProfiles returns a list of provisioning profiles.
    pub async fn list_profiles(&self, opts: Option<&ListOptions>) -> Result<ProfilesResponse> {
        todo!("port Client.ListProfiles from internal/asc/api/client.go")
    }

    /// RegisterDevice registers a new device.
    pub async fn register_device(&self, req: &DeviceCreateRequest) -> Result<DeviceResponse> {
        todo!("port Client.RegisterDevice from internal/asc/api/client.go")
    }

    /// RemoveBetaTesterFromGroup removes a beta tester from a group.
    pub async fn remove_beta_tester_from_group(&self, beta_group_id: &str, beta_tester_id: &str) -> Result<()> {
        todo!("port Client.RemoveBetaTesterFromGroup from internal/asc/api/client.go")
    }

    /// StartCiBuildRun starts a new build run for a workflow.
    pub async fn start_ci_build_run(&self, workflow_id: &str) -> Result<CiBuildRunResponse> {
        todo!("port Client.StartCiBuildRun from internal/asc/api/client.go")
    }

    /// UpdateBetaAppLocalization updates a beta app localization.
    pub async fn update_beta_app_localization(&self, localization_id: &str, req: &BetaAppLocalizationUpdateRequest) -> Result<BetaAppLocalizationResponse> {
        todo!("port Client.UpdateBetaAppLocalization from internal/asc/api/client.go")
    }

    /// UpdateBetaBuildLocalization updates a beta build localization.
    pub async fn update_beta_build_localization(&self, localization_id: &str, req: &BetaBuildLocalizationUpdateRequest) -> Result<BetaBuildLocalizationResponse> {
        todo!("port Client.UpdateBetaBuildLocalization from internal/asc/api/client.go")
    }

    /// UpdateBetaLicenseAgreement updates a beta license agreement.
    pub async fn update_beta_license_agreement(&self, agreement_id: &str, req: &BetaLicenseAgreementUpdateRequest) -> Result<BetaLicenseAgreementResponse> {
        todo!("port Client.UpdateBetaLicenseAgreement from internal/asc/api/client.go")
    }

    /// UpdateBuildBetaDetail updates build beta detail.
    pub async fn update_build_beta_detail(&self, detail_id: &str, req: &BuildBetaDetailUpdateRequest) -> Result<BuildBetaDetailResponse> {
        todo!("port Client.UpdateBuildBetaDetail from internal/asc/api/client.go")
    }

    /// DeleteBetaFeedbackCrashSubmission deletes a crash feedback submission.
    pub async fn delete_beta_feedback_crash_submission(&self, submission_id: &str) -> Result<()> {
        todo!("port Client.DeleteBetaFeedbackCrashSubmission from internal/asc/api/client_beta_feedback.go")
    }

    /// DeleteBetaFeedbackScreenshotSubmission deletes a screenshot feedback
    /// submission.
    pub async fn delete_beta_feedback_screenshot_submission(&self, submission_id: &str) -> Result<()> {
        todo!("port Client.DeleteBetaFeedbackScreenshotSubmission from internal/asc/api/client_beta_feedback.go")
    }

    /// GetBetaBuildUsageMetrics returns TestFlight usage metrics for a build.
    pub async fn get_beta_build_usage_metrics(&self, build_id: &str, limit: i64) -> Result<BetaBuildUsagesResponse> {
        todo!("port Client.GetBetaBuildUsageMetrics from internal/asc/api/client_beta_feedback.go")
    }

    /// GetBetaFeedbackCrashLog returns the crash log attached to a crash
    /// feedback submission.
    pub async fn get_beta_feedback_crash_log(&self, submission_id: &str) -> Result<BetaCrashLogResponse> {
        todo!("port Client.GetBetaFeedbackCrashLog from internal/asc/api/client_beta_feedback.go")
    }

    /// GetBetaFeedbackCrashSubmission returns a single crash feedback
    /// submission.
    pub async fn get_beta_feedback_crash_submission(&self, submission_id: &str) -> Result<BetaFeedbackCrashSubmissionResponse> {
        todo!("port Client.GetBetaFeedbackCrashSubmission from internal/asc/api/client_beta_feedback.go")
    }

    /// GetBetaFeedbackScreenshotSubmission returns a single screenshot
    /// feedback submission, including its image download URLs.
    pub async fn get_beta_feedback_screenshot_submission(&self, submission_id: &str) -> Result<BetaFeedbackScreenshotSubmissionResponse> {
        todo!("port Client.GetBetaFeedbackScreenshotSubmission from internal/asc/api/client_beta_feedback.go")
    }

    /// ListBetaFeedbackCrashSubmissions returns crash feedback submissions
    /// for an app.
    pub async fn list_beta_feedback_crash_submissions(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<BetaFeedbackCrashSubmissionsResponse> {
        todo!("port Client.ListBetaFeedbackCrashSubmissions from internal/asc/api/client_beta_feedback.go")
    }

    /// ListBetaFeedbackScreenshotSubmissions returns screenshot feedback
    /// submissions for an app.
    pub async fn list_beta_feedback_screenshot_submissions(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<BetaFeedbackScreenshotSubmissionsResponse> {
        todo!("port Client.ListBetaFeedbackScreenshotSubmissions from internal/asc/api/client_beta_feedback.go")
    }

    /// CreateBuildUpload starts a new build upload for an app.
    pub async fn create_build_upload(&self, req: &BuildUploadCreateRequest) -> Result<BuildUploadResponse> {
        todo!("port Client.CreateBuildUpload from internal/asc/api/client_build_uploads.go")
    }

    /// CreateBuildUploadFile reserves an asset file slot on a build upload.
    /// The response carries the upload operations for the file's bytes.
    pub async fn create_build_upload_file(&self, req: &BuildUploadFileCreateRequest) -> Result<BuildUploadFileResponse> {
        todo!("port Client.CreateBuildUploadFile from internal/asc/api/client_build_uploads.go")
    }

    /// DeleteBuildUpload deletes a build upload (e.g. an abandoned or failed
    /// delivery).
    pub async fn delete_build_upload(&self, build_upload_id: &str) -> Result<()> {
        todo!("port Client.DeleteBuildUpload from internal/asc/api/client_build_uploads.go")
    }

    /// GetBuildUpload returns a single build upload.
    pub async fn get_build_upload(&self, build_upload_id: &str, opts: Option<&ListOptions>) -> Result<BuildUploadResponse> {
        todo!("port Client.GetBuildUpload from internal/asc/api/client_build_uploads.go")
    }

    /// ListBuildUploadFiles returns the asset files attached to a build
    /// upload, including their delivery states.
    pub async fn list_build_upload_files(&self, build_upload_id: &str, opts: Option<&ListOptions>) -> Result<BuildUploadFilesResponse> {
        todo!("port Client.ListBuildUploadFiles from internal/asc/api/client_build_uploads.go")
    }

    /// ListBuildUploads returns the build uploads for an app.
    pub async fn list_build_uploads(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<BuildUploadsResponse> {
        todo!("port Client.ListBuildUploads from internal/asc/api/client_build_uploads.go")
    }

    /// UpdateBuildUploadFile commits (or amends) a build upload file.
    pub async fn update_build_upload_file(&self, file_id: &str, req: &BuildUploadFileUpdateRequest) -> Result<BuildUploadFileResponse> {
        todo!("port Client.UpdateBuildUploadFile from internal/asc/api/client_build_uploads.go")
    }

    /// UploadBuildFile reserves, uploads, and commits one asset file on an
    /// existing build upload. assetType is ASSET, ASSET_DESCRIPTION, or
    /// ASSET_SPI; uti must be one of Apple's accepted identifiers (e.g.
    /// com.apple.ipa). The commit carries the SHA-256 of the body so Apple
    /// can verify the stored bytes.
    pub async fn upload_build_file(&self, build_upload_id: &str, file_name: &str, asset_type: &str, uti: &str, body: &[u8]) -> Result<BuildUploadFileResponse> {
        todo!("port Client.UploadBuildFile from internal/asc/api/client_build_uploads.go")
    }

    /// ListCustomerReviewSummarizations returns Apple-generated summaries of an
    /// app's customer reviews.
    pub async fn list_customer_review_summarizations(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<CustomerReviewSummarizationsResponse> {
        todo!("port Client.ListCustomerReviewSummarizations from internal/asc/api/client_metadata_extras.go")
    }
}
