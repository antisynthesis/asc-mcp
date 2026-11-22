// App Store Connect API endpoints for the "listing" area.

#[allow(unused_imports)]
use crate::api::{
    Client, Error, ListOptions, MAX_UPLOAD_SIZE, Result, decode, model::*, sha256_hex,
};
#[allow(unused_imports)]
use crate::wire::{
    Base64Bytes, JsonMap, JsonSlice, RawMessage, Timestamp, json,
    url::{self, Values},
};
#[allow(unused_imports)]
use serde_json::Value;

impl Client {
    /// Creates a custom product page.
    pub async fn create_app_custom_product_page(
        &self,
        req: &AppCustomProductPageCreateRequest,
    ) -> Result<AppCustomProductPageResponse> {
        let data = self.post("/v1/appCustomProductPages", Some(req)).await?;
        decode(&data)
    }

    /// Creates a new app event.
    pub async fn create_app_event(&self, req: &AppEventCreateRequest) -> Result<AppEventResponse> {
        let data = self.post("/v1/appEvents", Some(req)).await?;
        decode(&data)
    }

    /// Creates a new app info localization.
    pub async fn create_app_info_localization(
        &self,
        req: &AppInfoLocalizationCreateRequest,
    ) -> Result<AppInfoLocalizationResponse> {
        let data = self.post("/v1/appInfoLocalizations", Some(req)).await?;
        decode(&data)
    }

    /// Creates a new preview.
    pub async fn create_app_preview(
        &self,
        req: &AppPreviewCreateRequest,
    ) -> Result<AppPreviewResponse> {
        let data = self.post("/v1/appPreviews", Some(req)).await?;
        decode(&data)
    }

    /// Creates a new screenshot.
    pub async fn create_app_screenshot(
        &self,
        req: &AppScreenshotCreateRequest,
    ) -> Result<AppScreenshotResponse> {
        let data = self.post("/v1/appScreenshots", Some(req)).await?;
        decode(&data)
    }

    /// Creates review details for a version.
    pub async fn create_app_store_review_detail(
        &self,
        req: &AppStoreReviewDetailCreateRequest,
    ) -> Result<AppStoreReviewDetailResponse> {
        let data = self.post("/v1/appStoreReviewDetails", Some(req)).await?;
        decode(&data)
    }

    /// Creates a new app store version.
    pub async fn create_app_store_version(
        &self,
        req: &AppStoreVersionCreateRequest,
    ) -> Result<AppStoreVersionResponse> {
        let data = self.post("/v1/appStoreVersions", Some(req)).await?;
        decode(&data)
    }

    /// Creates a v2 experiment attached to an app.
    pub async fn create_app_store_version_experiment(
        &self,
        req: &AppStoreVersionExperimentCreateRequest,
    ) -> Result<AppStoreVersionExperimentResponse> {
        let data = self
            .post("/v2/appStoreVersionExperiments", Some(req))
            .await?;
        decode(&data)
    }

    /// Creates a new version localization.
    pub async fn create_app_store_version_localization(
        &self,
        req: &AppStoreVersionLocalizationCreateRequest,
    ) -> Result<AppStoreVersionLocalizationResponse> {
        let data = self
            .post("/v1/appStoreVersionLocalizations", Some(req))
            .await?;
        decode(&data)
    }

    /// Creates a phased release.
    pub async fn create_app_store_version_phased_release(
        &self,
        req: &AppStoreVersionPhasedReleaseCreateRequest,
    ) -> Result<AppStoreVersionPhasedReleaseResponse> {
        let data = self
            .post("/v1/appStoreVersionPhasedReleases", Some(req))
            .await?;
        decode(&data)
    }

    /// Deletes a custom product page.
    pub async fn delete_app_custom_product_page(&self, page_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/appCustomProductPages/{}",
            url::path_escape(page_id)
        ))
        .await
    }

    /// Deletes an app event.
    pub async fn delete_app_event(&self, event_id: &str) -> Result<()> {
        self.delete(&format!("/v1/appEvents/{}", url::path_escape(event_id)))
            .await
    }

    /// Deletes an app info localization.
    pub async fn delete_app_info_localization(&self, localization_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/appInfoLocalizations/{}",
            url::path_escape(localization_id)
        ))
        .await
    }

    /// Deletes a preview.
    pub async fn delete_app_preview(&self, preview_id: &str) -> Result<()> {
        self.delete(&format!("/v1/appPreviews/{}", url::path_escape(preview_id)))
            .await
    }

    /// Deletes a screenshot.
    pub async fn delete_app_screenshot(&self, screenshot_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/appScreenshots/{}",
            url::path_escape(screenshot_id)
        ))
        .await
    }

    /// Deletes an app store version.
    pub async fn delete_app_store_version(&self, version_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/appStoreVersions/{}",
            url::path_escape(version_id)
        ))
        .await
    }

    /// Deletes a v2 experiment.
    pub async fn delete_app_store_version_experiment(&self, experiment_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v2/appStoreVersionExperiments/{}",
            url::path_escape(experiment_id)
        ))
        .await
    }

    /// Deletes a version localization.
    pub async fn delete_app_store_version_localization(&self, localization_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/appStoreVersionLocalizations/{}",
            url::path_escape(localization_id)
        ))
        .await
    }

    /// Deletes a phased release.
    pub async fn delete_app_store_version_phased_release(
        &self,
        phased_release_id: &str,
    ) -> Result<()> {
        self.delete(&format!(
            "/v1/appStoreVersionPhasedReleases/{}",
            url::path_escape(phased_release_id)
        ))
        .await
    }

    /// Returns a single app clip by ID.
    pub async fn get_app_clip(&self, app_clip_id: &str) -> Result<AppClipResponse> {
        let data = self
            .get(
                &format!("/v1/appClips/{}", url::path_escape(app_clip_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns a single advanced experience.
    pub async fn get_app_clip_advanced_experience(
        &self,
        experience_id: &str,
    ) -> Result<AppClipAdvancedExperienceResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/appClipAdvancedExperiences/{}",
                    url::path_escape(experience_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns a single default experience.
    pub async fn get_app_clip_default_experience(
        &self,
        experience_id: &str,
    ) -> Result<AppClipDefaultExperienceResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/appClipDefaultExperiences/{}",
                    url::path_escape(experience_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns a single custom product page.
    pub async fn get_app_custom_product_page(
        &self,
        page_id: &str,
    ) -> Result<AppCustomProductPageResponse> {
        let data = self
            .get(
                &format!("/v1/appCustomProductPages/{}", url::path_escape(page_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns a single app event by ID.
    pub async fn get_app_event(&self, event_id: &str) -> Result<AppEventResponse> {
        let data = self
            .get(
                &format!("/v1/appEvents/{}", url::path_escape(event_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns a single app info localization by ID.
    pub async fn get_app_info_localization(
        &self,
        localization_id: &str,
    ) -> Result<AppInfoLocalizationResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/appInfoLocalizations/{}",
                    url::path_escape(localization_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns app infos for an app.
    pub async fn get_app_infos(&self, app_id: &str) -> Result<AppInfosResponse> {
        let data = self
            .get(
                &format!("/v1/apps/{}/appInfos", url::path_escape(app_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns a single preview by ID.
    pub async fn get_app_preview(&self, preview_id: &str) -> Result<AppPreviewResponse> {
        let data = self
            .get(
                &format!("/v1/appPreviews/{}", url::path_escape(preview_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns a single screenshot by ID.
    pub async fn get_app_screenshot(&self, screenshot_id: &str) -> Result<AppScreenshotResponse> {
        let data = self
            .get(
                &format!("/v1/appScreenshots/{}", url::path_escape(screenshot_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns review details for a version.
    pub async fn get_app_store_review_detail(
        &self,
        version_id: &str,
    ) -> Result<AppStoreReviewDetailResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/appStoreVersions/{}/appStoreReviewDetail",
                    url::path_escape(version_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns a single app store version by ID.
    pub async fn get_app_store_version(&self, version_id: &str) -> Result<AppStoreVersionResponse> {
        let data = self
            .get(
                &format!("/v1/appStoreVersions/{}", url::path_escape(version_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns a single v2 experiment.
    pub async fn get_app_store_version_experiment(
        &self,
        experiment_id: &str,
    ) -> Result<AppStoreVersionExperimentResponse> {
        let data = self
            .get(
                &format!(
                    "/v2/appStoreVersionExperiments/{}",
                    url::path_escape(experiment_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns a single version localization by ID.
    pub async fn get_app_store_version_localization(
        &self,
        localization_id: &str,
    ) -> Result<AppStoreVersionLocalizationResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/appStoreVersionLocalizations/{}",
                    url::path_escape(localization_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns phased release for a version.
    pub async fn get_app_store_version_phased_release(
        &self,
        version_id: &str,
    ) -> Result<AppStoreVersionPhasedReleaseResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/appStoreVersions/{}/appStoreVersionPhasedRelease",
                    url::path_escape(version_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns advanced experiences for an app clip.
    pub async fn list_app_clip_advanced_experiences(
        &self,
        app_clip_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppClipAdvancedExperiencesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/appClips/{}/appClipAdvancedExperiences",
                    url::path_escape(app_clip_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns default experiences for an app clip.
    pub async fn list_app_clip_default_experiences(
        &self,
        app_clip_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppClipDefaultExperiencesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/appClips/{}/appClipDefaultExperiences",
                    url::path_escape(app_clip_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns app clips for an app.
    pub async fn list_app_clips(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppClipsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/apps/{}/appClips", url::path_escape(app_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns custom product pages for an app.
    pub async fn list_app_custom_product_pages(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppCustomProductPagesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/apps/{}/appCustomProductPages",
                    url::path_escape(app_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns app events for an app.
    pub async fn list_app_events(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppEventsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/apps/{}/appEvents", url::path_escape(app_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns localizations for an app info.
    pub async fn list_app_info_localizations(
        &self,
        app_info_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppInfoLocalizationsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/appInfos/{}/appInfoLocalizations",
                    url::path_escape(app_info_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns preview sets for a version localization.
    pub async fn list_app_preview_sets(
        &self,
        localization_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppPreviewSetsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/appStoreVersionLocalizations/{}/appPreviewSets",
                    url::path_escape(localization_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns previews for a preview set.
    pub async fn list_app_previews(
        &self,
        preview_set_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppPreviewsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/appPreviewSets/{}/appPreviews",
                    url::path_escape(preview_set_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns screenshot sets for a version localization.
    pub async fn list_app_screenshot_sets(
        &self,
        localization_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppScreenshotSetsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/appStoreVersionLocalizations/{}/appScreenshotSets",
                    url::path_escape(localization_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns screenshots for a screenshot set.
    pub async fn list_app_screenshots(
        &self,
        screenshot_set_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppScreenshotsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/appScreenshotSets/{}/appScreenshots",
                    url::path_escape(screenshot_set_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns v2 experiments for a version.
    pub async fn list_app_store_version_experiments(
        &self,
        version_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppStoreVersionExperimentsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/appStoreVersions/{}/appStoreVersionExperimentsV2",
                    url::path_escape(version_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns v2 experiments for an app.
    pub async fn list_app_store_version_experiments_for_app(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppStoreVersionExperimentsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/apps/{}/appStoreVersionExperimentsV2",
                    url::path_escape(app_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns localizations for a version.
    pub async fn list_app_store_version_localizations(
        &self,
        version_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppStoreVersionLocalizationsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/appStoreVersions/{}/appStoreVersionLocalizations",
                    url::path_escape(version_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Updates a custom product page.
    pub async fn update_app_custom_product_page(
        &self,
        page_id: &str,
        req: &AppCustomProductPageUpdateRequest,
    ) -> Result<AppCustomProductPageResponse> {
        let data = self
            .patch(
                &format!("/v1/appCustomProductPages/{}", url::path_escape(page_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Updates an app event.
    pub async fn update_app_event(
        &self,
        event_id: &str,
        req: &AppEventUpdateRequest,
    ) -> Result<AppEventResponse> {
        let data = self
            .patch(
                &format!("/v1/appEvents/{}", url::path_escape(event_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Updates an app info localization.
    pub async fn update_app_info_localization(
        &self,
        localization_id: &str,
        req: &AppInfoLocalizationUpdateRequest,
    ) -> Result<AppInfoLocalizationResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/appInfoLocalizations/{}",
                    url::path_escape(localization_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Commits an uploaded preview by sending its SHA-256
    /// checksum and uploaded=true. Also used to update the preview frame
    /// timecode after upload.
    pub async fn update_app_preview(
        &self,
        preview_id: &str,
        req: &AppPreviewUpdateRequest,
    ) -> Result<AppPreviewResponse> {
        let data = self
            .patch(
                &format!("/v1/appPreviews/{}", url::path_escape(preview_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Updates a screenshot.
    pub async fn update_app_screenshot(
        &self,
        screenshot_id: &str,
        req: &AppScreenshotUpdateRequest,
    ) -> Result<AppScreenshotResponse> {
        let data = self
            .patch(
                &format!("/v1/appScreenshots/{}", url::path_escape(screenshot_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Updates a review attachment.
    pub async fn update_app_store_review_attachment(
        &self,
        attachment_id: &str,
        req: &AppStoreReviewAttachmentUpdateRequest,
    ) -> Result<AppStoreReviewAttachmentResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/appStoreReviewAttachments/{}",
                    url::path_escape(attachment_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Updates review details.
    pub async fn update_app_store_review_detail(
        &self,
        detail_id: &str,
        req: &AppStoreReviewDetailUpdateRequest,
    ) -> Result<AppStoreReviewDetailResponse> {
        let data = self
            .patch(
                &format!("/v1/appStoreReviewDetails/{}", url::path_escape(detail_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Updates an app store version.
    pub async fn update_app_store_version(
        &self,
        version_id: &str,
        req: &AppStoreVersionUpdateRequest,
    ) -> Result<AppStoreVersionResponse> {
        let data = self
            .patch(
                &format!("/v1/appStoreVersions/{}", url::path_escape(version_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Updates a v2 experiment.
    pub async fn update_app_store_version_experiment(
        &self,
        experiment_id: &str,
        req: &AppStoreVersionExperimentUpdateRequest,
    ) -> Result<AppStoreVersionExperimentResponse> {
        let data = self
            .patch(
                &format!(
                    "/v2/appStoreVersionExperiments/{}",
                    url::path_escape(experiment_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Updates a version localization.
    pub async fn update_app_store_version_localization(
        &self,
        localization_id: &str,
        req: &AppStoreVersionLocalizationUpdateRequest,
    ) -> Result<AppStoreVersionLocalizationResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/appStoreVersionLocalizations/{}",
                    url::path_escape(localization_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Updates a phased release.
    pub async fn update_app_store_version_phased_release(
        &self,
        phased_release_id: &str,
        req: &AppStoreVersionPhasedReleaseUpdateRequest,
    ) -> Result<AppStoreVersionPhasedReleaseResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/appStoreVersionPhasedReleases/{}",
                    url::path_escape(phased_release_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Updates routing app coverage.
    pub async fn update_routing_app_coverage(
        &self,
        coverage_id: &str,
        req: &RoutingAppCoverageUpdateRequest,
    ) -> Result<RoutingAppCoverageResponse> {
        let data = self
            .patch(
                &format!("/v1/routingAppCoverages/{}", url::path_escape(coverage_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Creates a review submission for an app.
    pub async fn create_review_submission(
        &self,
        req: &ReviewSubmissionCreateRequest,
    ) -> Result<ReviewSubmissionResponse> {
        let data = self.post("/v1/reviewSubmissions", Some(req)).await?;
        decode(&data)
    }

    /// Attaches an item to a review submission.
    pub async fn create_review_submission_item(
        &self,
        req: &ReviewSubmissionItemCreateRequest,
    ) -> Result<ReviewSubmissionItemResponse> {
        let data = self.post("/v1/reviewSubmissionItems", Some(req)).await?;
        decode(&data)
    }

    /// Removes an item from a review submission.
    pub async fn delete_review_submission_item(&self, item_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/reviewSubmissionItems/{}",
            url::path_escape(item_id)
        ))
        .await
    }

    /// Returns a single review submission.
    pub async fn get_review_submission(
        &self,
        submission_id: &str,
    ) -> Result<ReviewSubmissionResponse> {
        let data = self
            .get(
                &format!("/v1/reviewSubmissions/{}", url::path_escape(submission_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns the items attached to a review submission.
    pub async fn list_review_submission_items(
        &self,
        submission_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<ReviewSubmissionItemsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/reviewSubmissions/{}/items",
                    url::path_escape(submission_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns review submissions for an app.
    pub async fn list_review_submissions(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<ReviewSubmissionsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/apps/{}/reviewSubmissions", url::path_escape(app_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Updates a review submission. Setting submitted=true
    /// submits it for review; setting canceled=true cancels it.
    pub async fn update_review_submission(
        &self,
        submission_id: &str,
        req: &ReviewSubmissionUpdateRequest,
    ) -> Result<ReviewSubmissionResponse> {
        let data = self
            .patch(
                &format!("/v1/reviewSubmissions/{}", url::path_escape(submission_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Reserves, uploads, and commits an app preview video.
    /// The MIME type defaults to video/mp4 if not provided.
    pub async fn upload_app_preview(
        &self,
        preview_set_id: &str,
        file_name: &str,
        mime_type: &str,
        body: &[u8],
    ) -> Result<AppPreviewResponse> {
        if file_name.is_empty() {
            return Err(Error::msg("file_name is required"));
        }
        if body.is_empty() {
            return Err(Error::msg("file body is empty"));
        }
        if body.len() > MAX_UPLOAD_SIZE {
            return Err(Error::msg(format!(
                "file body exceeds max size of {MAX_UPLOAD_SIZE} bytes"
            )));
        }
        let mime_type = if mime_type.is_empty() {
            "video/mp4"
        } else {
            mime_type
        };
        let create_req = AppPreviewCreateRequest {
            data: AppPreviewCreateData {
                type_: "appPreviews".into(),
                attributes: AppPreviewCreateAttributes {
                    file_name: file_name.into(),
                    file_size: body.len() as i64,
                    mime_type: mime_type.into(),
                    preview_frame_time_code: String::new(),
                },
                relationships: AppPreviewCreateRelationships {
                    app_preview_set: RelationshipData {
                        data: ResourceIdentifier {
                            type_: "appPreviewSets".into(),
                            id: preview_set_id.into(),
                        },
                    },
                },
            },
        };
        let reservation = self
            .create_app_preview(&create_req)
            .await
            .map_err(|e| e.wrap("reservation"))?;
        let ops = &reservation.data.attributes.upload_operations;
        if let Err(e) = self.perform_upload_operations(ops, body).await {
            return Err(e.wrap("upload"));
        }
        let uploaded = true;
        let commit = AppPreviewUpdateRequest {
            data: AppPreviewUpdateData {
                type_: "appPreviews".into(),
                id: reservation.data.id.clone(),
                attributes: AppPreviewUpdateAttributes {
                    source_file_checksum: sha256_hex(body),
                    uploaded: Some(uploaded),
                    ..Default::default()
                },
            },
        };
        self.update_app_preview(&reservation.data.id, &commit).await
    }

    /// Reserves, uploads, and commits a screenshot in
    /// one call. The reservation is created against the supplied screenshot
    /// set; the response includes the final screenshot record after the
    /// commit PATCH has been accepted.
    pub async fn upload_app_screenshot(
        &self,
        screenshot_set_id: &str,
        file_name: &str,
        body: &[u8],
    ) -> Result<AppScreenshotResponse> {
        if file_name.is_empty() {
            return Err(Error::msg("file_name is required"));
        }
        if body.is_empty() {
            return Err(Error::msg("file body is empty"));
        }
        if body.len() > MAX_UPLOAD_SIZE {
            return Err(Error::msg(format!(
                "file body exceeds max size of {MAX_UPLOAD_SIZE} bytes"
            )));
        }
        let create_req = AppScreenshotCreateRequest {
            data: AppScreenshotCreateData {
                type_: "appScreenshots".into(),
                attributes: AppScreenshotCreateAttributes {
                    file_name: file_name.into(),
                    file_size: body.len() as i64,
                },
                relationships: AppScreenshotCreateRelationships {
                    app_screenshot_set: RelationshipData {
                        data: ResourceIdentifier {
                            type_: "appScreenshotSets".into(),
                            id: screenshot_set_id.into(),
                        },
                    },
                },
            },
        };
        let reservation = self
            .create_app_screenshot(&create_req)
            .await
            .map_err(|e| e.wrap("reservation"))?;
        let ops = &reservation.data.attributes.upload_operations;
        if let Err(e) = self.perform_upload_operations(ops, body).await {
            return Err(e.wrap("upload"));
        }
        let uploaded = true;
        let commit = AppScreenshotUpdateRequest {
            data: AppScreenshotUpdateData {
                type_: "appScreenshots".into(),
                id: reservation.data.id.clone(),
                attributes: AppScreenshotUpdateAttributes {
                    source_file_checksum: sha256_hex(body),
                    uploaded: Some(uploaded),
                },
            },
        };
        self.update_app_screenshot(&reservation.data.id, &commit)
            .await
    }

    /// Reserves, uploads, and commits a
    /// review attachment (typically a PDF or screenshot) bound to an App
    /// Store review detail.
    pub async fn upload_app_store_review_attachment(
        &self,
        review_detail_id: &str,
        file_name: &str,
        body: &[u8],
    ) -> Result<AppStoreReviewAttachmentResponse> {
        if file_name.is_empty() {
            return Err(Error::msg("file_name is required"));
        }
        if body.is_empty() {
            return Err(Error::msg("file body is empty"));
        }
        if body.len() > MAX_UPLOAD_SIZE {
            return Err(Error::msg(format!(
                "file body exceeds max size of {MAX_UPLOAD_SIZE} bytes"
            )));
        }
        let create_req = AppStoreReviewAttachmentCreateRequest {
            data: AppStoreReviewAttachmentCreateData {
                type_: "appStoreReviewAttachments".into(),
                attributes: AppStoreReviewAttachmentCreateAttributes {
                    file_name: file_name.into(),
                    file_size: body.len() as i64,
                },
                relationships: AppStoreReviewAttachmentCreateRelationships {
                    app_store_review_detail: RelationshipData {
                        data: ResourceIdentifier {
                            type_: "appStoreReviewDetails".into(),
                            id: review_detail_id.into(),
                        },
                    },
                },
            },
        };
        let reservation = self
            .create_app_store_review_attachment(&create_req)
            .await
            .map_err(|e| e.wrap("reservation"))?;
        let ops = &reservation.data.attributes.upload_operations;
        if let Err(e) = self.perform_upload_operations(ops, body).await {
            return Err(e.wrap("upload"));
        }
        let uploaded = true;
        let commit = AppStoreReviewAttachmentUpdateRequest {
            data: AppStoreReviewAttachmentUpdateData {
                type_: "appStoreReviewAttachments".into(),
                id: reservation.data.id.clone(),
                attributes: AppStoreReviewAttachmentUpdateAttributes {
                    source_file_checksum: sha256_hex(body),
                    uploaded: Some(uploaded),
                },
            },
        };
        self.update_app_store_review_attachment(&reservation.data.id, &commit)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};

    use crate::api::model::*;
    use crate::api::sha256_hex;
    use crate::api::testutil::{MockApi, MockResponse};

    #[tokio::test]
    async fn client_list_review_submissions() {
        let mock = MockApi::start(|req| {
            assert_eq!(req.path, "/v1/apps/123/reviewSubmissions", "path");
            assert_eq!(req.method, "GET", "method");
            MockResponse::json(
                200,
                r#"{"data":[{"type":"reviewSubmissions","id":"sub-1","attributes":{"platform":"IOS","state":"READY_FOR_REVIEW"}}],"links":{}}"#,
            )
        })
        .await;

        let resp = mock
            .client()
            .list_review_submissions("123", None)
            .await
            .expect("unexpected error");
        assert_eq!(resp.data.len(), 1, "expected 1 review submission");
        assert_eq!(resp.data[0].attributes.state, "READY_FOR_REVIEW");
    }

    #[tokio::test]
    async fn client_create_review_submission() {
        let mock = MockApi::start(|req| {
            assert_eq!(req.path, "/v1/reviewSubmissions", "path");
            assert_eq!(req.method, "POST", "method");
            let body = req.json();
            assert_eq!(body["data"]["type"], "reviewSubmissions");
            assert_eq!(body["data"]["relationships"]["app"]["data"]["id"], "123");
            MockResponse::json(
                201,
                r#"{"data":{"type":"reviewSubmissions","id":"sub-1","attributes":{"platform":"IOS","state":"READY_FOR_REVIEW"}}}"#,
            )
        })
        .await;

        let req = ReviewSubmissionCreateRequest {
            data: ReviewSubmissionCreateData {
                type_: "reviewSubmissions".into(),
                attributes: Some(ReviewSubmissionCreateAttributes {
                    platform: "IOS".into(),
                }),
                relationships: ReviewSubmissionCreateRelationships {
                    app: RelationshipData {
                        data: ResourceIdentifier {
                            type_: "apps".into(),
                            id: "123".into(),
                        },
                    },
                },
            },
        };
        let resp = mock
            .client()
            .create_review_submission(&req)
            .await
            .expect("unexpected error");
        assert_eq!(resp.data.id, "sub-1");
    }

    #[tokio::test]
    async fn client_update_review_submission() {
        let tests: [(&str, Option<bool>, Option<bool>); 2] =
            [("submit", Some(true), None), ("cancel", None, Some(true))];

        for (name, submitted, canceled) in tests {
            let mock = MockApi::start(move |req| {
                assert_eq!(req.path, "/v1/reviewSubmissions/sub-1", "{name}: path");
                assert_eq!(req.method, "PATCH", "{name}: method");
                let body = req.json();
                let attrs = &body["data"]["attributes"];
                if submitted.is_some() {
                    assert_eq!(
                        attrs["submitted"], true,
                        "{name}: expected submitted=true in request body"
                    );
                }
                if canceled.is_some() {
                    assert_eq!(
                        attrs["canceled"], true,
                        "{name}: expected canceled=true in request body"
                    );
                }
                MockResponse::json(200, r#"{"data":{"type":"reviewSubmissions","id":"sub-1"}}"#)
            })
            .await;

            let req = ReviewSubmissionUpdateRequest {
                data: ReviewSubmissionUpdateData {
                    type_: "reviewSubmissions".into(),
                    id: "sub-1".into(),
                    attributes: ReviewSubmissionUpdateAttributes {
                        submitted,
                        canceled,
                    },
                },
            };
            let resp = mock
                .client()
                .update_review_submission("sub-1", &req)
                .await
                .expect("unexpected error");
            assert_eq!(resp.data.id, "sub-1", "{name}");
            assert_eq!(mock.requests().len(), 1, "{name}");
        }
    }

    #[tokio::test]
    async fn client_create_review_submission_item() {
        let mock = MockApi::start(|req| {
            assert_eq!(req.path, "/v1/reviewSubmissionItems", "path");
            assert_eq!(req.method, "POST", "method");
            let body = req.json();
            let rels = &body["data"]["relationships"];
            assert_eq!(rels["reviewSubmission"]["data"]["id"], "sub-1");
            assert_eq!(
                rels["appStoreVersion"]["data"]["id"], "ver-1",
                "expected appStoreVersion relationship with id ver-1"
            );
            MockResponse::json(
                201,
                r#"{"data":{"type":"reviewSubmissionItems","id":"item-1"}}"#,
            )
        })
        .await;

        let req = ReviewSubmissionItemCreateRequest {
            data: ReviewSubmissionItemCreateData {
                type_: "reviewSubmissionItems".into(),
                relationships: ReviewSubmissionItemCreateRelationships {
                    review_submission: RelationshipData {
                        data: ResourceIdentifier {
                            type_: "reviewSubmissions".into(),
                            id: "sub-1".into(),
                        },
                    },
                    app_store_version: Some(RelationshipData {
                        data: ResourceIdentifier {
                            type_: "appStoreVersions".into(),
                            id: "ver-1".into(),
                        },
                    }),
                    ..Default::default()
                },
            },
        };
        let resp = mock
            .client()
            .create_review_submission_item(&req)
            .await
            .expect("unexpected error");
        assert_eq!(resp.data.id, "item-1");
        assert_eq!(mock.requests().len(), 1);
    }

    #[tokio::test]
    async fn upload_app_screenshot_end_to_end() {
        // One server stands in for both the App Store Connect API and the
        // pre-signed storage URL. We route by path.
        let uploaded = Arc::new(AtomicBool::new(false));
        let committed = Arc::new(AtomicBool::new(false));
        let storage_url = Arc::new(Mutex::new(String::new()));

        let (u, cm, su) = (uploaded.clone(), committed.clone(), storage_url.clone());
        let mock = MockApi::start(move |req| match (req.method.as_str(), req.path.as_str()) {
            ("POST", "/v1/appScreenshots") => {
                // Reservation: respond with an upload operation pointing at /upload.
                let url = format!("{}/upload", su.lock().unwrap());
                MockResponse::json(
                    201,
                    serde_json::json!({"data": {"type": "appScreenshots", "id": "screen-1", "attributes": {
                        "fileName": "shot.png", "fileSize": 4,
                        "uploadOperations": [{"method": "PUT", "url": url, "offset": 0, "length": 4,
                            "requestHeaders": [{"name": "Content-Type", "value": "image/png"}]}]}}})
                    .to_string(),
                )
            }
            ("PUT", "/upload") => {
                assert_eq!(req.body, b"PNG!", "uploaded body");
                u.store(true, Ordering::SeqCst);
                MockResponse::empty(200)
            }
            ("PATCH", "/v1/appScreenshots/screen-1") => {
                let body = req.json();
                let attrs = &body["data"]["attributes"];
                assert_eq!(attrs["sourceFileChecksum"], sha256_hex(b"PNG!"), "commit checksum");
                assert_eq!(attrs["uploaded"], true, "commit uploaded flag not true");
                cm.store(true, Ordering::SeqCst);
                MockResponse::json(200, r#"{"data":{"type":"appScreenshots","id":"screen-1"}}"#)
            }
            (m, p) => panic!("unexpected request: {m} {p}"),
        })
        .await;
        *storage_url.lock().unwrap() = mock.base_url.clone();

        let resp = mock
            .client()
            .upload_app_screenshot("set-1", "shot.png", b"PNG!")
            .await
            .expect("unexpected error");
        assert_eq!(resp.data.id, "screen-1", "final ID");
        assert!(uploaded.load(Ordering::SeqCst), "upload step did not run");
        assert!(committed.load(Ordering::SeqCst), "commit step did not run");
    }
}
