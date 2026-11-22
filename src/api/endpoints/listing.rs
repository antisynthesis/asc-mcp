// App Store Connect API endpoints for the "listing" area.
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
    /// CreateAppCustomProductPage creates a custom product page.
    pub async fn create_app_custom_product_page(&self, req: &AppCustomProductPageCreateRequest) -> Result<AppCustomProductPageResponse> {
        todo!("port Client.CreateAppCustomProductPage from internal/asc/api/client.go")
    }

    /// CreateAppEvent creates a new app event.
    pub async fn create_app_event(&self, req: &AppEventCreateRequest) -> Result<AppEventResponse> {
        todo!("port Client.CreateAppEvent from internal/asc/api/client.go")
    }

    /// CreateAppInfoLocalization creates a new app info localization.
    pub async fn create_app_info_localization(&self, req: &AppInfoLocalizationCreateRequest) -> Result<AppInfoLocalizationResponse> {
        todo!("port Client.CreateAppInfoLocalization from internal/asc/api/client.go")
    }

    /// CreateAppPreview creates a new preview.
    pub async fn create_app_preview(&self, req: &AppPreviewCreateRequest) -> Result<AppPreviewResponse> {
        todo!("port Client.CreateAppPreview from internal/asc/api/client.go")
    }

    /// CreateAppScreenshot creates a new screenshot.
    pub async fn create_app_screenshot(&self, req: &AppScreenshotCreateRequest) -> Result<AppScreenshotResponse> {
        todo!("port Client.CreateAppScreenshot from internal/asc/api/client.go")
    }

    /// CreateAppStoreReviewDetail creates review details for a version.
    pub async fn create_app_store_review_detail(&self, req: &AppStoreReviewDetailCreateRequest) -> Result<AppStoreReviewDetailResponse> {
        todo!("port Client.CreateAppStoreReviewDetail from internal/asc/api/client.go")
    }

    /// CreateAppStoreVersion creates a new app store version.
    pub async fn create_app_store_version(&self, req: &AppStoreVersionCreateRequest) -> Result<AppStoreVersionResponse> {
        todo!("port Client.CreateAppStoreVersion from internal/asc/api/client.go")
    }

    /// CreateAppStoreVersionExperiment creates a v2 experiment attached to an app.
    pub async fn create_app_store_version_experiment(&self, req: &AppStoreVersionExperimentCreateRequest) -> Result<AppStoreVersionExperimentResponse> {
        todo!("port Client.CreateAppStoreVersionExperiment from internal/asc/api/client.go")
    }

    /// CreateAppStoreVersionLocalization creates a new version localization.
    pub async fn create_app_store_version_localization(&self, req: &AppStoreVersionLocalizationCreateRequest) -> Result<AppStoreVersionLocalizationResponse> {
        todo!("port Client.CreateAppStoreVersionLocalization from internal/asc/api/client.go")
    }

    /// CreateAppStoreVersionPhasedRelease creates a phased release.
    pub async fn create_app_store_version_phased_release(&self, req: &AppStoreVersionPhasedReleaseCreateRequest) -> Result<AppStoreVersionPhasedReleaseResponse> {
        todo!("port Client.CreateAppStoreVersionPhasedRelease from internal/asc/api/client.go")
    }

    /// DeleteAppCustomProductPage deletes a custom product page.
    pub async fn delete_app_custom_product_page(&self, page_id: &str) -> Result<()> {
        todo!("port Client.DeleteAppCustomProductPage from internal/asc/api/client.go")
    }

    /// DeleteAppEvent deletes an app event.
    pub async fn delete_app_event(&self, event_id: &str) -> Result<()> {
        todo!("port Client.DeleteAppEvent from internal/asc/api/client.go")
    }

    /// DeleteAppInfoLocalization deletes an app info localization.
    pub async fn delete_app_info_localization(&self, localization_id: &str) -> Result<()> {
        todo!("port Client.DeleteAppInfoLocalization from internal/asc/api/client.go")
    }

    /// DeleteAppPreview deletes a preview.
    pub async fn delete_app_preview(&self, preview_id: &str) -> Result<()> {
        todo!("port Client.DeleteAppPreview from internal/asc/api/client.go")
    }

    /// DeleteAppScreenshot deletes a screenshot.
    pub async fn delete_app_screenshot(&self, screenshot_id: &str) -> Result<()> {
        todo!("port Client.DeleteAppScreenshot from internal/asc/api/client.go")
    }

    /// DeleteAppStoreVersion deletes an app store version.
    pub async fn delete_app_store_version(&self, version_id: &str) -> Result<()> {
        todo!("port Client.DeleteAppStoreVersion from internal/asc/api/client.go")
    }

    /// DeleteAppStoreVersionExperiment deletes a v2 experiment.
    pub async fn delete_app_store_version_experiment(&self, experiment_id: &str) -> Result<()> {
        todo!("port Client.DeleteAppStoreVersionExperiment from internal/asc/api/client.go")
    }

    /// DeleteAppStoreVersionLocalization deletes a version localization.
    pub async fn delete_app_store_version_localization(&self, localization_id: &str) -> Result<()> {
        todo!("port Client.DeleteAppStoreVersionLocalization from internal/asc/api/client.go")
    }

    /// DeleteAppStoreVersionPhasedRelease deletes a phased release.
    pub async fn delete_app_store_version_phased_release(&self, phased_release_id: &str) -> Result<()> {
        todo!("port Client.DeleteAppStoreVersionPhasedRelease from internal/asc/api/client.go")
    }

    /// GetAppClip returns a single app clip by ID.
    pub async fn get_app_clip(&self, app_clip_id: &str) -> Result<AppClipResponse> {
        todo!("port Client.GetAppClip from internal/asc/api/client.go")
    }

    /// GetAppClipAdvancedExperience returns a single advanced experience.
    pub async fn get_app_clip_advanced_experience(&self, experience_id: &str) -> Result<AppClipAdvancedExperienceResponse> {
        todo!("port Client.GetAppClipAdvancedExperience from internal/asc/api/client.go")
    }

    /// GetAppClipDefaultExperience returns a single default experience.
    pub async fn get_app_clip_default_experience(&self, experience_id: &str) -> Result<AppClipDefaultExperienceResponse> {
        todo!("port Client.GetAppClipDefaultExperience from internal/asc/api/client.go")
    }

    /// GetAppCustomProductPage returns a single custom product page.
    pub async fn get_app_custom_product_page(&self, page_id: &str) -> Result<AppCustomProductPageResponse> {
        todo!("port Client.GetAppCustomProductPage from internal/asc/api/client.go")
    }

    /// GetAppEvent returns a single app event by ID.
    pub async fn get_app_event(&self, event_id: &str) -> Result<AppEventResponse> {
        todo!("port Client.GetAppEvent from internal/asc/api/client.go")
    }

    /// GetAppInfoLocalization returns a single app info localization by ID.
    pub async fn get_app_info_localization(&self, localization_id: &str) -> Result<AppInfoLocalizationResponse> {
        todo!("port Client.GetAppInfoLocalization from internal/asc/api/client.go")
    }

    /// GetAppInfos returns app infos for an app.
    pub async fn get_app_infos(&self, app_id: &str) -> Result<AppInfosResponse> {
        todo!("port Client.GetAppInfos from internal/asc/api/client.go")
    }

    /// GetAppPreview returns a single preview by ID.
    pub async fn get_app_preview(&self, preview_id: &str) -> Result<AppPreviewResponse> {
        todo!("port Client.GetAppPreview from internal/asc/api/client.go")
    }

    /// GetAppScreenshot returns a single screenshot by ID.
    pub async fn get_app_screenshot(&self, screenshot_id: &str) -> Result<AppScreenshotResponse> {
        todo!("port Client.GetAppScreenshot from internal/asc/api/client.go")
    }

    /// GetAppStoreReviewDetail returns review details for a version.
    pub async fn get_app_store_review_detail(&self, version_id: &str) -> Result<AppStoreReviewDetailResponse> {
        todo!("port Client.GetAppStoreReviewDetail from internal/asc/api/client.go")
    }

    /// GetAppStoreVersion returns a single app store version by ID.
    pub async fn get_app_store_version(&self, version_id: &str) -> Result<AppStoreVersionResponse> {
        todo!("port Client.GetAppStoreVersion from internal/asc/api/client.go")
    }

    /// GetAppStoreVersionExperiment returns a single v2 experiment.
    pub async fn get_app_store_version_experiment(&self, experiment_id: &str) -> Result<AppStoreVersionExperimentResponse> {
        todo!("port Client.GetAppStoreVersionExperiment from internal/asc/api/client.go")
    }

    /// GetAppStoreVersionLocalization returns a single version localization by ID.
    pub async fn get_app_store_version_localization(&self, localization_id: &str) -> Result<AppStoreVersionLocalizationResponse> {
        todo!("port Client.GetAppStoreVersionLocalization from internal/asc/api/client.go")
    }

    /// GetAppStoreVersionPhasedRelease returns phased release for a version.
    pub async fn get_app_store_version_phased_release(&self, version_id: &str) -> Result<AppStoreVersionPhasedReleaseResponse> {
        todo!("port Client.GetAppStoreVersionPhasedRelease from internal/asc/api/client.go")
    }

    /// ListAppClipAdvancedExperiences returns advanced experiences for an app clip.
    pub async fn list_app_clip_advanced_experiences(&self, app_clip_id: &str, opts: Option<&ListOptions>) -> Result<AppClipAdvancedExperiencesResponse> {
        todo!("port Client.ListAppClipAdvancedExperiences from internal/asc/api/client.go")
    }

    /// ListAppClipDefaultExperiences returns default experiences for an app clip.
    pub async fn list_app_clip_default_experiences(&self, app_clip_id: &str, opts: Option<&ListOptions>) -> Result<AppClipDefaultExperiencesResponse> {
        todo!("port Client.ListAppClipDefaultExperiences from internal/asc/api/client.go")
    }

    /// ListAppClips returns app clips for an app.
    pub async fn list_app_clips(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<AppClipsResponse> {
        todo!("port Client.ListAppClips from internal/asc/api/client.go")
    }

    /// ListAppCustomProductPages returns custom product pages for an app.
    pub async fn list_app_custom_product_pages(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<AppCustomProductPagesResponse> {
        todo!("port Client.ListAppCustomProductPages from internal/asc/api/client.go")
    }

    /// ListAppEvents returns app events for an app.
    pub async fn list_app_events(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<AppEventsResponse> {
        todo!("port Client.ListAppEvents from internal/asc/api/client.go")
    }

    /// ListAppInfoLocalizations returns localizations for an app info.
    pub async fn list_app_info_localizations(&self, app_info_id: &str, opts: Option<&ListOptions>) -> Result<AppInfoLocalizationsResponse> {
        todo!("port Client.ListAppInfoLocalizations from internal/asc/api/client.go")
    }

    /// ListAppPreviewSets returns preview sets for a version localization.
    pub async fn list_app_preview_sets(&self, localization_id: &str, opts: Option<&ListOptions>) -> Result<AppPreviewSetsResponse> {
        todo!("port Client.ListAppPreviewSets from internal/asc/api/client.go")
    }

    /// ListAppPreviews returns previews for a preview set.
    pub async fn list_app_previews(&self, preview_set_id: &str, opts: Option<&ListOptions>) -> Result<AppPreviewsResponse> {
        todo!("port Client.ListAppPreviews from internal/asc/api/client.go")
    }

    /// ListAppScreenshotSets returns screenshot sets for a version localization.
    pub async fn list_app_screenshot_sets(&self, localization_id: &str, opts: Option<&ListOptions>) -> Result<AppScreenshotSetsResponse> {
        todo!("port Client.ListAppScreenshotSets from internal/asc/api/client.go")
    }

    /// ListAppScreenshots returns screenshots for a screenshot set.
    pub async fn list_app_screenshots(&self, screenshot_set_id: &str, opts: Option<&ListOptions>) -> Result<AppScreenshotsResponse> {
        todo!("port Client.ListAppScreenshots from internal/asc/api/client.go")
    }

    /// ListAppStoreVersionExperiments returns v2 experiments for a version.
    pub async fn list_app_store_version_experiments(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<AppStoreVersionExperimentsResponse> {
        todo!("port Client.ListAppStoreVersionExperiments from internal/asc/api/client.go")
    }

    /// ListAppStoreVersionExperimentsForApp returns v2 experiments for an app.
    pub async fn list_app_store_version_experiments_for_app(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<AppStoreVersionExperimentsResponse> {
        todo!("port Client.ListAppStoreVersionExperimentsForApp from internal/asc/api/client.go")
    }

    /// ListAppStoreVersionLocalizations returns localizations for a version.
    pub async fn list_app_store_version_localizations(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<AppStoreVersionLocalizationsResponse> {
        todo!("port Client.ListAppStoreVersionLocalizations from internal/asc/api/client.go")
    }

    /// UpdateAppCustomProductPage updates a custom product page.
    pub async fn update_app_custom_product_page(&self, page_id: &str, req: &AppCustomProductPageUpdateRequest) -> Result<AppCustomProductPageResponse> {
        todo!("port Client.UpdateAppCustomProductPage from internal/asc/api/client.go")
    }

    /// UpdateAppEvent updates an app event.
    pub async fn update_app_event(&self, event_id: &str, req: &AppEventUpdateRequest) -> Result<AppEventResponse> {
        todo!("port Client.UpdateAppEvent from internal/asc/api/client.go")
    }

    /// UpdateAppInfoLocalization updates an app info localization.
    pub async fn update_app_info_localization(&self, localization_id: &str, req: &AppInfoLocalizationUpdateRequest) -> Result<AppInfoLocalizationResponse> {
        todo!("port Client.UpdateAppInfoLocalization from internal/asc/api/client.go")
    }

    /// UpdateAppPreview commits an uploaded preview by sending its SHA-256
    /// checksum and uploaded=true. Also used to update the preview frame
    /// timecode after upload.
    pub async fn update_app_preview(&self, preview_id: &str, req: &AppPreviewUpdateRequest) -> Result<AppPreviewResponse> {
        todo!("port Client.UpdateAppPreview from internal/asc/api/client.go")
    }

    /// UpdateAppScreenshot updates a screenshot.
    pub async fn update_app_screenshot(&self, screenshot_id: &str, req: &AppScreenshotUpdateRequest) -> Result<AppScreenshotResponse> {
        todo!("port Client.UpdateAppScreenshot from internal/asc/api/client.go")
    }

    /// UpdateAppStoreReviewAttachment updates a review attachment.
    pub async fn update_app_store_review_attachment(&self, attachment_id: &str, req: &AppStoreReviewAttachmentUpdateRequest) -> Result<AppStoreReviewAttachmentResponse> {
        todo!("port Client.UpdateAppStoreReviewAttachment from internal/asc/api/client.go")
    }

    /// UpdateAppStoreReviewDetail updates review details.
    pub async fn update_app_store_review_detail(&self, detail_id: &str, req: &AppStoreReviewDetailUpdateRequest) -> Result<AppStoreReviewDetailResponse> {
        todo!("port Client.UpdateAppStoreReviewDetail from internal/asc/api/client.go")
    }

    /// UpdateAppStoreVersion updates an app store version.
    pub async fn update_app_store_version(&self, version_id: &str, req: &AppStoreVersionUpdateRequest) -> Result<AppStoreVersionResponse> {
        todo!("port Client.UpdateAppStoreVersion from internal/asc/api/client.go")
    }

    /// UpdateAppStoreVersionExperiment updates a v2 experiment.
    pub async fn update_app_store_version_experiment(&self, experiment_id: &str, req: &AppStoreVersionExperimentUpdateRequest) -> Result<AppStoreVersionExperimentResponse> {
        todo!("port Client.UpdateAppStoreVersionExperiment from internal/asc/api/client.go")
    }

    /// UpdateAppStoreVersionLocalization updates a version localization.
    pub async fn update_app_store_version_localization(&self, localization_id: &str, req: &AppStoreVersionLocalizationUpdateRequest) -> Result<AppStoreVersionLocalizationResponse> {
        todo!("port Client.UpdateAppStoreVersionLocalization from internal/asc/api/client.go")
    }

    /// UpdateAppStoreVersionPhasedRelease updates a phased release.
    pub async fn update_app_store_version_phased_release(&self, phased_release_id: &str, req: &AppStoreVersionPhasedReleaseUpdateRequest) -> Result<AppStoreVersionPhasedReleaseResponse> {
        todo!("port Client.UpdateAppStoreVersionPhasedRelease from internal/asc/api/client.go")
    }

    /// UpdateRoutingAppCoverage updates routing app coverage.
    pub async fn update_routing_app_coverage(&self, coverage_id: &str, req: &RoutingAppCoverageUpdateRequest) -> Result<RoutingAppCoverageResponse> {
        todo!("port Client.UpdateRoutingAppCoverage from internal/asc/api/client.go")
    }

    /// CreateReviewSubmission creates a review submission for an app.
    pub async fn create_review_submission(&self, req: &ReviewSubmissionCreateRequest) -> Result<ReviewSubmissionResponse> {
        todo!("port Client.CreateReviewSubmission from internal/asc/api/client_review_submissions.go")
    }

    /// CreateReviewSubmissionItem attaches an item to a review submission.
    pub async fn create_review_submission_item(&self, req: &ReviewSubmissionItemCreateRequest) -> Result<ReviewSubmissionItemResponse> {
        todo!("port Client.CreateReviewSubmissionItem from internal/asc/api/client_review_submissions.go")
    }

    /// DeleteReviewSubmissionItem removes an item from a review submission.
    pub async fn delete_review_submission_item(&self, item_id: &str) -> Result<()> {
        todo!("port Client.DeleteReviewSubmissionItem from internal/asc/api/client_review_submissions.go")
    }

    /// GetReviewSubmission returns a single review submission.
    pub async fn get_review_submission(&self, submission_id: &str) -> Result<ReviewSubmissionResponse> {
        todo!("port Client.GetReviewSubmission from internal/asc/api/client_review_submissions.go")
    }

    /// ListReviewSubmissionItems returns the items attached to a review submission.
    pub async fn list_review_submission_items(&self, submission_id: &str, opts: Option<&ListOptions>) -> Result<ReviewSubmissionItemsResponse> {
        todo!("port Client.ListReviewSubmissionItems from internal/asc/api/client_review_submissions.go")
    }

    /// ListReviewSubmissions returns review submissions for an app.
    pub async fn list_review_submissions(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<ReviewSubmissionsResponse> {
        todo!("port Client.ListReviewSubmissions from internal/asc/api/client_review_submissions.go")
    }

    /// UpdateReviewSubmission updates a review submission. Setting submitted=true
    /// submits it for review; setting canceled=true cancels it.
    pub async fn update_review_submission(&self, submission_id: &str, req: &ReviewSubmissionUpdateRequest) -> Result<ReviewSubmissionResponse> {
        todo!("port Client.UpdateReviewSubmission from internal/asc/api/client_review_submissions.go")
    }

    /// UploadAppPreview reserves, uploads, and commits an app preview video.
    /// The MIME type defaults to video/mp4 if not provided.
    pub async fn upload_app_preview(&self, preview_set_id: &str, file_name: &str, mime_type: &str, body: &[u8]) -> Result<AppPreviewResponse> {
        todo!("port Client.UploadAppPreview from internal/asc/api/upload.go")
    }

    /// UploadAppScreenshot reserves, uploads, and commits a screenshot in
    /// one call. The reservation is created against the supplied screenshot
    /// set; the response includes the final screenshot record after the
    /// commit PATCH has been accepted.
    pub async fn upload_app_screenshot(&self, screenshot_set_id: &str, file_name: &str, body: &[u8]) -> Result<AppScreenshotResponse> {
        todo!("port Client.UploadAppScreenshot from internal/asc/api/upload.go")
    }

    /// UploadAppStoreReviewAttachment reserves, uploads, and commits a
    /// review attachment (typically a PDF or screenshot) bound to an App
    /// Store review detail.
    pub async fn upload_app_store_review_attachment(&self, review_detail_id: &str, file_name: &str, body: &[u8]) -> Result<AppStoreReviewAttachmentResponse> {
        todo!("port Client.UploadAppStoreReviewAttachment from internal/asc/api/upload.go")
    }
}
