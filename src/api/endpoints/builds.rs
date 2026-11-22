// App Store Connect API endpoints for the "builds" area.
//
// Signatures were generated from the Go declarations; bodies are ported by
// hand from the Go file named in each method's todo!/comment and must
// preserve the exact request path, query and body.

#[allow(unused_imports)]
use crate::api::{Client, Error, ListOptions, Result, decode, model::*};
#[allow(unused_imports)]
use crate::go::{
    GoBytes, GoMap, GoSlice, GoTime, RawMessage, json,
    url::{self, Values},
};
#[allow(unused_imports)]
use serde_json::Value;

impl Client {
    /// AddBetaTesterToGroup adds a beta tester to a group.
    pub async fn add_beta_tester_to_group(
        &self,
        beta_group_id: &str,
        beta_tester_id: &str,
    ) -> Result<()> {
        let body = serde_json::json!({
            "data": [
                {
                    "type": "betaTesters",
                    "id": beta_tester_id,
                },
            ],
        });
        self.post(
            &format!(
                "/v1/betaGroups/{}/relationships/betaTesters",
                url::path_escape(beta_group_id)
            ),
            Some(&body),
        )
        .await?;
        Ok(())
    }

    /// CreateBetaAppLocalization creates a beta app localization.
    pub async fn create_beta_app_localization(
        &self,
        req: &BetaAppLocalizationCreateRequest,
    ) -> Result<BetaAppLocalizationResponse> {
        let data = self.post("/v1/betaAppLocalizations", Some(req)).await?;
        decode(&data)
    }

    /// CreateBetaAppReviewSubmission submits a build for beta app review.
    pub async fn create_beta_app_review_submission(
        &self,
        req: &BetaAppReviewSubmissionCreateRequest,
    ) -> Result<BetaAppReviewSubmissionResponse> {
        let data = self.post("/v1/betaAppReviewSubmissions", Some(req)).await?;
        decode(&data)
    }

    /// CreateBetaBuildLocalization creates a beta build localization.
    pub async fn create_beta_build_localization(
        &self,
        req: &BetaBuildLocalizationCreateRequest,
    ) -> Result<BetaBuildLocalizationResponse> {
        let data = self.post("/v1/betaBuildLocalizations", Some(req)).await?;
        decode(&data)
    }

    /// CreateBetaGroup creates a new beta group.
    pub async fn create_beta_group(
        &self,
        req: &BetaGroupCreateRequest,
    ) -> Result<BetaGroupResponse> {
        let data = self.post("/v1/betaGroups", Some(req)).await?;
        decode(&data)
    }

    /// CreateBetaTester invites a new beta tester.
    pub async fn create_beta_tester(
        &self,
        req: &BetaTesterCreateRequest,
    ) -> Result<BetaTesterResponse> {
        let data = self.post("/v1/betaTesters", Some(req)).await?;
        decode(&data)
    }

    /// CreateCustomerReviewResponse creates a response to a customer review.
    pub async fn create_customer_review_response(
        &self,
        req: &CustomerReviewResponseCreateRequest,
    ) -> Result<CustomerReviewResponseV1Response> {
        let data = self.post("/v1/customerReviewResponses", Some(req)).await?;
        decode(&data)
    }

    /// DeleteBetaAppLocalization deletes a beta app localization.
    pub async fn delete_beta_app_localization(&self, localization_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/betaAppLocalizations/{}",
            url::path_escape(localization_id)
        ))
        .await
    }

    /// DeleteBetaBuildLocalization deletes a beta build localization.
    pub async fn delete_beta_build_localization(&self, localization_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/betaBuildLocalizations/{}",
            url::path_escape(localization_id)
        ))
        .await
    }

    /// DeleteBetaGroup deletes a beta group.
    pub async fn delete_beta_group(&self, beta_group_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/betaGroups/{}",
            url::path_escape(beta_group_id)
        ))
        .await
    }

    /// DeleteBetaTester removes a beta tester.
    pub async fn delete_beta_tester(&self, beta_tester_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/betaTesters/{}",
            url::path_escape(beta_tester_id)
        ))
        .await
    }

    /// DeleteCustomerReviewResponse deletes a customer review response.
    pub async fn delete_customer_review_response(&self, response_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/customerReviewResponses/{}",
            url::path_escape(response_id)
        ))
        .await
    }

    /// GetBetaAppLocalization returns a single beta app localization.
    pub async fn get_beta_app_localization(
        &self,
        localization_id: &str,
    ) -> Result<BetaAppLocalizationResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/betaAppLocalizations/{}",
                    url::path_escape(localization_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetBetaAppReviewSubmission returns a single beta app review submission.
    pub async fn get_beta_app_review_submission(
        &self,
        submission_id: &str,
    ) -> Result<BetaAppReviewSubmissionResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/betaAppReviewSubmissions/{}",
                    url::path_escape(submission_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetBetaBuildLocalization returns a single beta build localization.
    pub async fn get_beta_build_localization(
        &self,
        localization_id: &str,
    ) -> Result<BetaBuildLocalizationResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/betaBuildLocalizations/{}",
                    url::path_escape(localization_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetBetaLicenseAgreement returns a single beta license agreement.
    pub async fn get_beta_license_agreement(
        &self,
        agreement_id: &str,
    ) -> Result<BetaLicenseAgreementResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/betaLicenseAgreements/{}",
                    url::path_escape(agreement_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetBuild returns a single build by ID.
    pub async fn get_build(&self, build_id: &str) -> Result<BuildResponse> {
        let data = self
            .get(&format!("/v1/builds/{}", url::path_escape(build_id)), None)
            .await?;
        decode(&data)
    }

    /// GetBuildBetaDetail returns build beta detail.
    pub async fn get_build_beta_detail(&self, build_id: &str) -> Result<BuildBetaDetailResponse> {
        let data = self
            .get(
                &format!("/v1/builds/{}/buildBetaDetail", url::path_escape(build_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetBundleID returns a single bundle ID by ID.
    pub async fn get_bundle_id(&self, bundle_id_id: &str) -> Result<BundleIDResponse> {
        let data = self
            .get(
                &format!("/v1/bundleIds/{}", url::path_escape(bundle_id_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetCiBuildRun returns a single build run.
    pub async fn get_ci_build_run(&self, build_run_id: &str) -> Result<CiBuildRunResponse> {
        let data = self
            .get(
                &format!("/v1/ciBuildRuns/{}", url::path_escape(build_run_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetCiProduct returns a single Xcode Cloud product.
    pub async fn get_ci_product(&self, product_id: &str) -> Result<CiProductResponse> {
        let data = self
            .get(
                &format!("/v1/ciProducts/{}", url::path_escape(product_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetCiWorkflow returns a single workflow.
    pub async fn get_ci_workflow(&self, workflow_id: &str) -> Result<CiWorkflowResponse> {
        let data = self
            .get(
                &format!("/v1/ciWorkflows/{}", url::path_escape(workflow_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetCustomerReview returns a single customer review by ID.
    pub async fn get_customer_review(&self, review_id: &str) -> Result<CustomerReviewResponse> {
        let data = self
            .get(
                &format!("/v1/customerReviews/{}", url::path_escape(review_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetProfile returns a single profile by ID.
    pub async fn get_profile(&self, profile_id: &str) -> Result<ProfileResponse> {
        let data = self
            .get(
                &format!("/v1/profiles/{}", url::path_escape(profile_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// ListBetaAppLocalizations returns beta app localizations.
    pub async fn list_beta_app_localizations(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BetaAppLocalizationsResponse> {
        let mut query = Values::new();
        query.set("filter[app]", app_id);
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get("/v1/betaAppLocalizations", Some(&query)).await?;
        decode(&data)
    }

    /// ListBetaAppReviewSubmissions returns a list of beta app review submissions.
    pub async fn list_beta_app_review_submissions(
        &self,
        opts: Option<&ListOptions>,
    ) -> Result<BetaAppReviewSubmissionsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get("/v1/betaAppReviewSubmissions", Some(&query))
            .await?;
        decode(&data)
    }

    /// ListBetaBuildLocalizations returns beta build localizations.
    pub async fn list_beta_build_localizations(
        &self,
        build_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BetaBuildLocalizationsResponse> {
        let mut query = Values::new();
        query.set("filter[build]", build_id);
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get("/v1/betaBuildLocalizations", Some(&query)).await?;
        decode(&data)
    }

    /// ListBetaGroups returns a list of beta groups.
    pub async fn list_beta_groups(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BetaGroupsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        if !app_id.is_empty() {
            query.set("filter[app]", app_id);
        }
        let data = self.get("/v1/betaGroups", Some(&query)).await?;
        decode(&data)
    }

    /// ListBetaLicenseAgreements returns a list of beta license agreements.
    pub async fn list_beta_license_agreements(
        &self,
        opts: Option<&ListOptions>,
    ) -> Result<BetaLicenseAgreementsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get("/v1/betaLicenseAgreements", Some(&query)).await?;
        decode(&data)
    }

    /// ListBetaTesters returns a list of beta testers.
    pub async fn list_beta_testers(
        &self,
        beta_group_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BetaTestersResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        if !beta_group_id.is_empty() {
            query.set("filter[betaGroups]", beta_group_id);
        }
        let data = self.get("/v1/betaTesters", Some(&query)).await?;
        decode(&data)
    }

    /// ListBuilds returns a list of builds.
    pub async fn list_builds(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BuildsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        if !app_id.is_empty() {
            query.set("filter[app]", app_id);
        }
        let data = self.get("/v1/builds", Some(&query)).await?;
        decode(&data)
    }

    /// ListBundleIDs returns a list of bundle IDs.
    pub async fn list_bundle_ids(&self, opts: Option<&ListOptions>) -> Result<BundleIDsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get("/v1/bundleIds", Some(&query)).await?;
        decode(&data)
    }

    /// ListCertificates returns a list of certificates.
    pub async fn list_certificates(
        &self,
        opts: Option<&ListOptions>,
    ) -> Result<CertificatesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get("/v1/certificates", Some(&query)).await?;
        decode(&data)
    }

    /// ListCiBuildRuns returns build runs for a workflow.
    pub async fn list_ci_build_runs(
        &self,
        workflow_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<CiBuildRunsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/ciWorkflows/{}/buildRuns",
                    url::path_escape(workflow_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListCiProducts returns Xcode Cloud products for an app.
    pub async fn list_ci_products(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<CiProductsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        if !app_id.is_empty() {
            query.set("filter[app]", app_id);
        }
        let data = self.get("/v1/ciProducts", Some(&query)).await?;
        decode(&data)
    }

    /// ListCiWorkflows returns workflows for a product.
    pub async fn list_ci_workflows(
        &self,
        product_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<CiWorkflowsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/ciProducts/{}/workflows", url::path_escape(product_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListCustomerReviews returns customer reviews for an app.
    pub async fn list_customer_reviews(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<CustomerReviewsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/apps/{}/customerReviews", url::path_escape(app_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListDevices returns a list of devices.
    pub async fn list_devices(&self, opts: Option<&ListOptions>) -> Result<DevicesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get("/v1/devices", Some(&query)).await?;
        decode(&data)
    }

    /// ListProfiles returns a list of provisioning profiles.
    pub async fn list_profiles(&self, opts: Option<&ListOptions>) -> Result<ProfilesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get("/v1/profiles", Some(&query)).await?;
        decode(&data)
    }

    /// RegisterDevice registers a new device.
    pub async fn register_device(&self, req: &DeviceCreateRequest) -> Result<DeviceResponse> {
        let data = self.post("/v1/devices", Some(req)).await?;
        decode(&data)
    }

    /// RemoveBetaTesterFromGroup removes a beta tester from a group.
    pub async fn remove_beta_tester_from_group(
        &self,
        beta_group_id: &str,
        beta_tester_id: &str,
    ) -> Result<()> {
        // This requires a DELETE with a body, which is non-standard
        // For now, we use the delete beta tester endpoint
        let _ = beta_tester_id;
        self.delete(&format!(
            "/v1/betaGroups/{}/relationships/betaTesters",
            url::path_escape(beta_group_id)
        ))
        .await
    }

    /// StartCiBuildRun starts a new build run for a workflow.
    pub async fn start_ci_build_run(&self, workflow_id: &str) -> Result<CiBuildRunResponse> {
        let body = serde_json::json!({
            "data": {
                "type": "ciBuildRuns",
                "relationships": {
                    "workflow": {
                        "data": {
                            "type": "ciWorkflows",
                            "id": workflow_id,
                        },
                    },
                },
            },
        });
        let data = self.post("/v1/ciBuildRuns", Some(&body)).await?;
        decode(&data)
    }

    /// UpdateBetaAppLocalization updates a beta app localization.
    pub async fn update_beta_app_localization(
        &self,
        localization_id: &str,
        req: &BetaAppLocalizationUpdateRequest,
    ) -> Result<BetaAppLocalizationResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/betaAppLocalizations/{}",
                    url::path_escape(localization_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateBetaBuildLocalization updates a beta build localization.
    pub async fn update_beta_build_localization(
        &self,
        localization_id: &str,
        req: &BetaBuildLocalizationUpdateRequest,
    ) -> Result<BetaBuildLocalizationResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/betaBuildLocalizations/{}",
                    url::path_escape(localization_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateBetaLicenseAgreement updates a beta license agreement.
    pub async fn update_beta_license_agreement(
        &self,
        agreement_id: &str,
        req: &BetaLicenseAgreementUpdateRequest,
    ) -> Result<BetaLicenseAgreementResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/betaLicenseAgreements/{}",
                    url::path_escape(agreement_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateBuildBetaDetail updates build beta detail.
    pub async fn update_build_beta_detail(
        &self,
        detail_id: &str,
        req: &BuildBetaDetailUpdateRequest,
    ) -> Result<BuildBetaDetailResponse> {
        let data = self
            .patch(
                &format!("/v1/buildBetaDetails/{}", url::path_escape(detail_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// DeleteBetaFeedbackCrashSubmission deletes a crash feedback submission.
    pub async fn delete_beta_feedback_crash_submission(&self, submission_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/betaFeedbackCrashSubmissions/{}",
            url::path_escape(submission_id)
        ))
        .await
    }

    /// DeleteBetaFeedbackScreenshotSubmission deletes a screenshot feedback
    /// submission.
    pub async fn delete_beta_feedback_screenshot_submission(
        &self,
        submission_id: &str,
    ) -> Result<()> {
        self.delete(&format!(
            "/v1/betaFeedbackScreenshotSubmissions/{}",
            url::path_escape(submission_id)
        ))
        .await
    }

    /// GetBetaBuildUsageMetrics returns TestFlight usage metrics for a build.
    pub async fn get_beta_build_usage_metrics(
        &self,
        build_id: &str,
        limit: i64,
    ) -> Result<BetaBuildUsagesResponse> {
        let mut query = Values::new();
        if limit > 0 {
            query.set("limit", limit.to_string());
        }
        let data = self
            .get(
                &format!(
                    "/v1/builds/{}/metrics/betaBuildUsages",
                    url::path_escape(build_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// GetBetaFeedbackCrashLog returns the crash log attached to a crash
    /// feedback submission.
    pub async fn get_beta_feedback_crash_log(
        &self,
        submission_id: &str,
    ) -> Result<BetaCrashLogResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/betaFeedbackCrashSubmissions/{}/crashLog",
                    url::path_escape(submission_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetBetaFeedbackCrashSubmission returns a single crash feedback
    /// submission.
    pub async fn get_beta_feedback_crash_submission(
        &self,
        submission_id: &str,
    ) -> Result<BetaFeedbackCrashSubmissionResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/betaFeedbackCrashSubmissions/{}",
                    url::path_escape(submission_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetBetaFeedbackScreenshotSubmission returns a single screenshot
    /// feedback submission, including its image download URLs.
    pub async fn get_beta_feedback_screenshot_submission(
        &self,
        submission_id: &str,
    ) -> Result<BetaFeedbackScreenshotSubmissionResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/betaFeedbackScreenshotSubmissions/{}",
                    url::path_escape(submission_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// ListBetaFeedbackCrashSubmissions returns crash feedback submissions
    /// for an app.
    pub async fn list_beta_feedback_crash_submissions(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BetaFeedbackCrashSubmissionsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/apps/{}/betaFeedbackCrashSubmissions",
                    url::path_escape(app_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListBetaFeedbackScreenshotSubmissions returns screenshot feedback
    /// submissions for an app.
    pub async fn list_beta_feedback_screenshot_submissions(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BetaFeedbackScreenshotSubmissionsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/apps/{}/betaFeedbackScreenshotSubmissions",
                    url::path_escape(app_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// CreateBuildUpload starts a new build upload for an app.
    pub async fn create_build_upload(
        &self,
        req: &BuildUploadCreateRequest,
    ) -> Result<BuildUploadResponse> {
        let data = self.post("/v1/buildUploads", Some(req)).await?;
        decode(&data)
    }

    /// CreateBuildUploadFile reserves an asset file slot on a build upload.
    /// The response carries the upload operations for the file's bytes.
    pub async fn create_build_upload_file(
        &self,
        req: &BuildUploadFileCreateRequest,
    ) -> Result<BuildUploadFileResponse> {
        let data = self.post("/v1/buildUploadFiles", Some(req)).await?;
        decode(&data)
    }

    /// DeleteBuildUpload deletes a build upload (e.g. an abandoned or failed
    /// delivery).
    pub async fn delete_build_upload(&self, build_upload_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/buildUploads/{}",
            url::path_escape(build_upload_id)
        ))
        .await
    }

    /// GetBuildUpload returns a single build upload.
    pub async fn get_build_upload(
        &self,
        build_upload_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BuildUploadResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/buildUploads/{}", url::path_escape(build_upload_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListBuildUploadFiles returns the asset files attached to a build
    /// upload, including their delivery states.
    pub async fn list_build_upload_files(
        &self,
        build_upload_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BuildUploadFilesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/buildUploads/{}/buildUploadFiles",
                    url::path_escape(build_upload_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListBuildUploads returns the build uploads for an app.
    pub async fn list_build_uploads(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BuildUploadsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/apps/{}/buildUploads", url::path_escape(app_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// UpdateBuildUploadFile commits (or amends) a build upload file.
    pub async fn update_build_upload_file(
        &self,
        file_id: &str,
        req: &BuildUploadFileUpdateRequest,
    ) -> Result<BuildUploadFileResponse> {
        let data = self
            .patch(
                &format!("/v1/buildUploadFiles/{}", url::path_escape(file_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UploadBuildFile reserves, uploads, and commits one asset file on an
    /// existing build upload. assetType is ASSET, ASSET_DESCRIPTION, or
    /// ASSET_SPI; uti must be one of Apple's accepted identifiers (e.g.
    /// com.apple.ipa). The commit carries the SHA-256 of the body so Apple
    /// can verify the stored bytes.
    pub async fn upload_build_file(
        &self,
        build_upload_id: &str,
        file_name: &str,
        asset_type: &str,
        uti: &str,
        body: &[u8],
    ) -> Result<BuildUploadFileResponse> {
        if file_name.is_empty() {
            return Err(Error::msg("file_name is required"));
        }
        if body.is_empty() {
            return Err(Error::msg("file body is empty"));
        }
        if body.len() > crate::api::MAX_UPLOAD_SIZE {
            return Err(Error::msg(format!(
                "file body exceeds max size of {} bytes",
                crate::api::MAX_UPLOAD_SIZE
            )));
        }
        let create_req = BuildUploadFileCreateRequest {
            data: BuildUploadFileCreateData {
                type_: "buildUploadFiles".into(),
                attributes: BuildUploadFileCreateAttributes {
                    asset_type: asset_type.into(),
                    file_name: file_name.into(),
                    file_size: body.len() as i64,
                    uti: uti.into(),
                },
                relationships: BuildUploadFileCreateRelationships {
                    build_upload: RelationshipData {
                        data: ResourceIdentifier {
                            type_: "buildUploads".into(),
                            id: build_upload_id.into(),
                        },
                    },
                },
            },
        };
        let reservation = self
            .create_build_upload_file(&create_req)
            .await
            .map_err(|e| e.wrap("reservation"))?;
        let ops = &reservation.data.attributes.upload_operations;
        self.perform_upload_operations(ops, body)
            .await
            .map_err(|e| e.wrap("upload"))?;
        let commit = BuildUploadFileUpdateRequest {
            data: BuildUploadFileUpdateData {
                type_: "buildUploadFiles".into(),
                id: reservation.data.id.clone(),
                attributes: Some(BuildUploadFileUpdateAttributes {
                    source_file_checksums: Some(Checksums {
                        file: Some(Checksum {
                            hash: crate::api::sha256_hex(body),
                            algorithm: "SHA_256".into(),
                        }),
                        ..Default::default()
                    }),
                    uploaded: Some(true),
                }),
            },
        };
        self.update_build_upload_file(&reservation.data.id, &commit)
            .await
    }

    /// ListCustomerReviewSummarizations returns Apple-generated summaries of an
    /// app's customer reviews.
    pub async fn list_customer_review_summarizations(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<CustomerReviewSummarizationsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/apps/{}/customerReviewSummarizations",
                    url::path_escape(app_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }
}

#[cfg(test)]
mod tests {
    // Ported from internal/asc/api/client_beta_feedback_test.go and
    // internal/asc/api/client_build_uploads_test.go.

    use super::*;
    use crate::api::testutil::{MockApi, MockResponse, test_token_provider};
    use std::sync::{Arc, Mutex};

    fn filter(k: &str, v: &str) -> ListOptions {
        let mut o = ListOptions::default();
        o.filter.insert(k, GoSlice::from(vec![v.to_string()]));
        o
    }

    #[tokio::test]
    async fn test_client_list_beta_feedback_crash_submissions() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                200,
                r#"{"data":[{"type":"betaFeedbackCrashSubmissions","id":"crash-1","attributes":{"comment":"crashed on launch","deviceModel":"iPhone16,1","osVersion":"18.2"}}],"links":{}}"#,
            )
        })
        .await;
        let resp = mock
            .client()
            .list_beta_feedback_crash_submissions("123", Some(&filter("build", "build-9")))
            .await
            .unwrap();
        let req = &mock.requests()[0];
        assert_eq!(req.path, "/v1/apps/123/betaFeedbackCrashSubmissions");
        assert_eq!(req.method, "GET");
        assert_eq!(req.query_param("filter[build]").as_deref(), Some("build-9"));
        assert_eq!(resp.data.len(), 1, "expected 1 submission");
        assert_eq!(resp.data[0].attributes.device_model, "iPhone16,1");
    }

    #[tokio::test]
    async fn test_client_get_beta_feedback_crash_log() {
        let mock = MockApi::start(|_| {
            MockResponse::json(200, r#"{"data":{"type":"betaCrashLogs","id":"crash-1","attributes":{"logText":"Exception Type: EXC_CRASH"}}}"#)
        })
        .await;
        let resp = mock
            .client()
            .get_beta_feedback_crash_log("crash-1")
            .await
            .unwrap();
        let req = &mock.requests()[0];
        assert_eq!(
            req.path,
            "/v1/betaFeedbackCrashSubmissions/crash-1/crashLog"
        );
        assert_eq!(req.method, "GET");
        assert_eq!(resp.data.attributes.log_text, "Exception Type: EXC_CRASH");
    }

    #[tokio::test]
    async fn test_client_delete_beta_feedback_crash_submission() {
        let mock = MockApi::start(|_| MockResponse::empty(204)).await;
        mock.client()
            .delete_beta_feedback_crash_submission("crash-1")
            .await
            .unwrap();
        let req = &mock.requests()[0];
        assert_eq!(req.path, "/v1/betaFeedbackCrashSubmissions/crash-1");
        assert_eq!(req.method, "DELETE");
    }

    #[tokio::test]
    async fn test_client_list_beta_feedback_screenshot_submissions() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                200,
                r#"{"data":[{"type":"betaFeedbackScreenshotSubmissions","id":"shot-1","attributes":{"comment":"misaligned button","screenshots":[{"url":"https://example.com/img.png","width":1179,"height":2556}]}}],"links":{}}"#,
            )
        })
        .await;
        let resp = mock
            .client()
            .list_beta_feedback_screenshot_submissions("123", None)
            .await
            .unwrap();
        let req = &mock.requests()[0];
        assert_eq!(req.path, "/v1/apps/123/betaFeedbackScreenshotSubmissions");
        assert_eq!(req.method, "GET");
        assert_eq!(resp.data.len(), 1, "expected 1 submission");
        let shots = &resp.data[0].attributes.screenshots;
        assert!(
            shots.len() == 1 && shots[0].url == "https://example.com/img.png",
            "screenshots: want one with URL"
        );
    }

    #[tokio::test]
    async fn test_client_get_beta_feedback_screenshot_submission() {
        let mock = MockApi::start(|_| {
            MockResponse::json(200, r#"{"data":{"type":"betaFeedbackScreenshotSubmissions","id":"shot-1","attributes":{}}}"#)
        })
        .await;
        let resp = mock
            .client()
            .get_beta_feedback_screenshot_submission("shot-1")
            .await
            .unwrap();
        let req = &mock.requests()[0];
        assert_eq!(req.path, "/v1/betaFeedbackScreenshotSubmissions/shot-1");
        assert_eq!(req.method, "GET");
        assert_eq!(resp.data.id, "shot-1");
    }

    #[tokio::test]
    async fn test_client_get_beta_build_usage_metrics() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                200,
                r#"{"data":[{"dataPoints":[{"values":{"crashCount":2,"installCount":12,"sessionCount":40,"feedbackCount":0,"inviteCount":0}}]}],"links":{}}"#,
            )
        })
        .await;
        let resp = mock
            .client()
            .get_beta_build_usage_metrics("build-9", 5)
            .await
            .unwrap();
        let req = &mock.requests()[0];
        assert_eq!(req.path, "/v1/builds/build-9/metrics/betaBuildUsages");
        assert_eq!(req.method, "GET");
        assert_eq!(req.query_param("limit").as_deref(), Some("5"));
        assert!(
            resp.data.len() == 1 && resp.data[0].data_points.len() == 1,
            "unexpected data shape"
        );
        assert_eq!(resp.data[0].data_points[0].values.install_count, 12);
    }

    #[tokio::test]
    async fn test_client_list_build_uploads() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                200,
                r#"{"data":[{"type":"buildUploads","id":"upload-1","attributes":{"cfBundleShortVersionString":"1.2.3","cfBundleVersion":"42","platform":"IOS","state":{"state":"PROCESSING"}}}],"links":{}}"#,
            )
        })
        .await;
        let resp = mock
            .client()
            .list_build_uploads("123", Some(&filter("state", "PROCESSING")))
            .await
            .unwrap();
        let req = &mock.requests()[0];
        assert_eq!(req.path, "/v1/apps/123/buildUploads");
        assert_eq!(req.method, "GET");
        assert_eq!(
            req.query_param("filter[state]").as_deref(),
            Some("PROCESSING")
        );
        assert_eq!(resp.data.len(), 1, "expected 1 build upload");
        assert_eq!(
            resp.data[0].attributes.state.as_ref().unwrap().state,
            "PROCESSING"
        );
    }

    #[tokio::test]
    async fn test_client_create_build_upload() {
        let mock = MockApi::start(|_| {
            MockResponse::json(200, r#"{"data":{"type":"buildUploads","id":"upload-1","attributes":{"state":{"state":"AWAITING_UPLOAD"}}}}"#)
        })
        .await;
        let req = BuildUploadCreateRequest {
            data: BuildUploadCreateData {
                type_: "buildUploads".into(),
                attributes: BuildUploadCreateAttributes {
                    cf_bundle_short_version_string: "1.2.3".into(),
                    cf_bundle_version: "42".into(),
                    platform: "IOS".into(),
                },
                relationships: BuildUploadCreateRelationships {
                    app: RelationshipData {
                        data: ResourceIdentifier {
                            type_: "apps".into(),
                            id: "123".into(),
                        },
                    },
                },
            },
        };
        let resp = mock.client().create_build_upload(&req).await.unwrap();
        let rec = &mock.requests()[0];
        assert_eq!(rec.path, "/v1/buildUploads");
        assert_eq!(rec.method, "POST");
        let body = rec.json();
        assert_eq!(body["data"]["type"], "buildUploads");
        assert_eq!(body["data"]["attributes"]["cfBundleVersion"], "42");
        assert_eq!(body["data"]["relationships"]["app"]["data"]["id"], "123");
        assert_eq!(resp.data.id, "upload-1");
    }

    #[tokio::test]
    async fn test_client_get_build_upload() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                200,
                r#"{"data":{"type":"buildUploads","id":"upload-1","attributes":{"state":{"errors":[{"code":"INVALID_BINARY","description":"bad ipa"}],"state":"FAILED"}},"relationships":{"build":{"data":{"type":"builds","id":"build-9"}}}}}"#,
            )
        })
        .await;
        let resp = mock
            .client()
            .get_build_upload("upload-1", None)
            .await
            .unwrap();
        let req = &mock.requests()[0];
        assert_eq!(req.path, "/v1/buildUploads/upload-1");
        assert_eq!(req.method, "GET");
        let state = resp.data.attributes.state.as_ref().unwrap();
        assert_eq!(state.state, "FAILED");
        assert!(
            state.errors.len() == 1 && state.errors[0].code == "INVALID_BINARY",
            "state errors: want one INVALID_BINARY"
        );
        assert_eq!(
            resp.data
                .relationships
                .as_ref()
                .unwrap()
                .build
                .as_ref()
                .unwrap()
                .data
                .id,
            "build-9"
        );
    }

    #[tokio::test]
    async fn test_client_delete_build_upload() {
        let mock = MockApi::start(|_| MockResponse::empty(204)).await;
        mock.client().delete_build_upload("upload-1").await.unwrap();
        let req = &mock.requests()[0];
        assert_eq!(req.path, "/v1/buildUploads/upload-1");
        assert_eq!(req.method, "DELETE");
    }

    #[tokio::test]
    async fn test_client_list_build_upload_files() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                200,
                r#"{"data":[{"type":"buildUploadFiles","id":"file-1","attributes":{"assetDeliveryState":{"state":"COMPLETE"},"assetType":"ASSET","fileName":"MyApp.ipa","fileSize":4,"uti":"com.apple.ipa"}}],"links":{}}"#,
            )
        })
        .await;
        let resp = mock
            .client()
            .list_build_upload_files("upload-1", None)
            .await
            .unwrap();
        let req = &mock.requests()[0];
        assert_eq!(req.path, "/v1/buildUploads/upload-1/buildUploadFiles");
        assert_eq!(req.method, "GET");
        assert_eq!(resp.data.len(), 1, "expected 1 file");
        assert_eq!(
            resp.data[0]
                .attributes
                .asset_delivery_state
                .as_ref()
                .unwrap()
                .state,
            "COMPLETE"
        );
    }

    #[tokio::test]
    async fn test_upload_build_file_end_to_end() {
        // One server stands in for both the App Store Connect API and the
        // pre-signed storage URL. We route by path.
        let storage_url = Arc::new(Mutex::new(String::new()));
        let su = storage_url.clone();
        let mock = MockApi::start(move |r| match (r.method.as_str(), r.path.as_str()) {
            ("POST", "/v1/buildUploadFiles") => {
                let url = format!("{}/upload", su.lock().unwrap());
                MockResponse::json(
                    201,
                    format!(
                        r#"{{"data":{{"type":"buildUploadFiles","id":"file-1","attributes":{{"fileName":"MyApp.ipa","fileSize":4,"uploadOperations":[{{"method":"PUT","url":"{url}","length":4,"requestHeaders":[{{"name":"Content-Type","value":"application/octet-stream"}}]}}]}}}}}}"#
                    ),
                )
            }
            ("PUT", "/upload") => MockResponse::empty(200),
            ("PATCH", "/v1/buildUploadFiles/file-1") => {
                MockResponse::json(200, r#"{"data":{"type":"buildUploadFiles","id":"file-1","attributes":{}}}"#)
            }
            _ => MockResponse::empty(404),
        })
        .await;
        *storage_url.lock().unwrap() = mock.base_url.clone();

        let c = Client::with_token_provider(test_token_provider(), mock.base_url.clone());
        let resp = c
            .upload_build_file("upload-1", "MyApp.ipa", "ASSET", "com.apple.ipa", b"IPA!")
            .await
            .unwrap();
        assert_eq!(resp.data.id, "file-1", "final ID");

        let reqs = mock.requests();
        assert_eq!(reqs.len(), 3, "unexpected requests");
        let reservation = reqs[0].json();
        assert_eq!(reservation["data"]["attributes"]["assetType"], "ASSET");
        assert_eq!(reservation["data"]["attributes"]["uti"], "com.apple.ipa");
        assert_eq!(
            reservation["data"]["relationships"]["buildUpload"]["data"]["id"],
            "upload-1"
        );

        assert_eq!(
            (reqs[1].method.as_str(), reqs[1].path.as_str()),
            ("PUT", "/upload"),
            "upload step did not run"
        );
        assert_eq!(reqs[1].body, b"IPA!", "uploaded body");

        assert_eq!(
            (reqs[2].method.as_str(), reqs[2].path.as_str()),
            ("PATCH", "/v1/buildUploadFiles/file-1"),
            "commit step did not run"
        );
        let commit = reqs[2].json();
        let checksums = &commit["data"]["attributes"]["sourceFileChecksums"];
        assert!(checksums["file"].is_object(), "commit checksums missing");
        assert_eq!(checksums["file"]["hash"], crate::api::sha256_hex(b"IPA!"));
        assert_eq!(checksums["file"]["algorithm"], "SHA_256");
        assert_eq!(
            commit["data"]["attributes"]["uploaded"], true,
            "commit uploaded flag not true"
        );
    }

    #[tokio::test]
    async fn test_upload_build_file_validation() {
        let c = Client::with_token_provider(test_token_provider(), "http://127.0.0.1:1");
        let tests: [(&str, &str, &[u8]); 2] = [
            ("missing file name", "", b"IPA!"),
            ("empty body", "MyApp.ipa", b""),
        ];
        for (name, file_name, body) in tests {
            assert!(
                c.upload_build_file("upload-1", file_name, "ASSET", "com.apple.ipa", body)
                    .await
                    .is_err(),
                "{name}: expected error"
            );
        }
    }
}
