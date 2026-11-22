// App Store Connect API endpoints for the "platform" area.

#[allow(unused_imports)]
use crate::api::{Client, Error, ListOptions, Result, decode, model::*};
#[allow(unused_imports)]
use crate::wire::{
    Base64Bytes, JsonMap, JsonSlice, RawMessage, Timestamp, json,
    url::{self, Values},
};
#[allow(unused_imports)]
use serde_json::Value;

impl Client {
    /// Creates an alternative distribution key.
    pub async fn create_alternative_distribution_key(
        &self,
        req: &AlternativeDistributionKeyCreateRequest,
    ) -> Result<AlternativeDistributionKeyResponse> {
        let data = self
            .post("/v1/alternativeDistributionKeys", Some(req))
            .await?;
        decode(&data)
    }

    /// Creates an analytics report request.
    pub async fn create_analytics_report_request(
        &self,
        req: &AnalyticsReportRequestCreateRequest,
    ) -> Result<AnalyticsReportRequestResponse> {
        let data = self.post("/v1/analyticsReportRequests", Some(req)).await?;
        decode(&data)
    }

    /// Creates a review attachment.
    pub async fn create_app_store_review_attachment(
        &self,
        req: &AppStoreReviewAttachmentCreateRequest,
    ) -> Result<AppStoreReviewAttachmentResponse> {
        let data = self
            .post("/v1/appStoreReviewAttachments", Some(req))
            .await?;
        decode(&data)
    }

    /// Creates an EULA.
    pub async fn create_end_user_license_agreement(
        &self,
        req: &EndUserLicenseAgreementCreateRequest,
    ) -> Result<EndUserLicenseAgreementResponse> {
        let data = self.post("/v1/endUserLicenseAgreements", Some(req)).await?;
        decode(&data)
    }

    /// Creates marketplace search details.
    pub async fn create_marketplace_search_detail(
        &self,
        req: &MarketplaceSearchDetailCreateRequest,
    ) -> Result<MarketplaceSearchDetailResponse> {
        let data = self.post("/v1/marketplaceSearchDetails", Some(req)).await?;
        decode(&data)
    }

    /// Creates routing app coverage.
    pub async fn create_routing_app_coverage(
        &self,
        req: &RoutingAppCoverageCreateRequest,
    ) -> Result<RoutingAppCoverageResponse> {
        let data = self.post("/v1/routingAppCoverages", Some(req)).await?;
        decode(&data)
    }

    /// Deletes an alternative distribution key.
    pub async fn delete_alternative_distribution_key(&self, key_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/alternativeDistributionKeys/{}",
            url::path_escape(key_id)
        ))
        .await
    }

    /// Deletes an analytics report request.
    pub async fn delete_analytics_report_request(&self, request_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/analyticsReportRequests/{}",
            url::path_escape(request_id)
        ))
        .await
    }

    /// Deletes a review attachment.
    pub async fn delete_app_store_review_attachment(&self, attachment_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/appStoreReviewAttachments/{}",
            url::path_escape(attachment_id)
        ))
        .await
    }

    /// Deletes an EULA.
    pub async fn delete_end_user_license_agreement(&self, agreement_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/endUserLicenseAgreements/{}",
            url::path_escape(agreement_id)
        ))
        .await
    }

    /// Deletes marketplace search details.
    pub async fn delete_marketplace_search_detail(&self, detail_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/marketplaceSearchDetails/{}",
            url::path_escape(detail_id)
        ))
        .await
    }

    /// Deletes routing app coverage.
    pub async fn delete_routing_app_coverage(&self, coverage_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/routingAppCoverages/{}",
            url::path_escape(coverage_id)
        ))
        .await
    }

    /// Returns a single alternative distribution key.
    pub async fn get_alternative_distribution_key(
        &self,
        key_id: &str,
    ) -> Result<AlternativeDistributionKeyResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/alternativeDistributionKeys/{}",
                    url::path_escape(key_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns the alternative
    /// distribution package for an app store version.
    pub async fn get_alternative_distribution_package_for_version(
        &self,
        version_id: &str,
    ) -> Result<AlternativeDistributionPackageResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/appStoreVersions/{}/alternativeDistributionPackage",
                    url::path_escape(version_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns a single analytics report request.
    pub async fn get_analytics_report_request(
        &self,
        request_id: &str,
    ) -> Result<AnalyticsReportRequestResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/analyticsReportRequests/{}",
                    url::path_escape(request_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns a single app category.
    pub async fn get_app_category(&self, category_id: &str) -> Result<AppCategoryResponse> {
        let data = self
            .get(
                &format!("/v1/appCategories/{}", url::path_escape(category_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns a single review attachment.
    pub async fn get_app_store_review_attachment(
        &self,
        attachment_id: &str,
    ) -> Result<AppStoreReviewAttachmentResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/appStoreReviewAttachments/{}",
                    url::path_escape(attachment_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns an EULA.
    pub async fn get_end_user_license_agreement(
        &self,
        app_id: &str,
    ) -> Result<EndUserLicenseAgreementResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/apps/{}/endUserLicenseAgreement",
                    url::path_escape(app_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns finance reports.
    pub async fn get_finance_report(
        &self,
        vendor_number: &str,
        region_code: &str,
        report_type: &str,
        report_date: &str,
    ) -> Result<Vec<u8>> {
        let mut query = Values::new();
        query.set("filter[vendorNumber]", vendor_number);
        query.set("filter[regionCode]", region_code);
        query.set("filter[reportType]", report_type);
        query.set("filter[reportDate]", report_date);
        self.get("/v1/financeReports", Some(&query)).await
    }

    /// Returns marketplace search details.
    pub async fn get_marketplace_search_detail(
        &self,
        app_id: &str,
    ) -> Result<MarketplaceSearchDetailResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/apps/{}/marketplaceSearchDetail",
                    url::path_escape(app_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns routing app coverage.
    pub async fn get_routing_app_coverage(
        &self,
        version_id: &str,
    ) -> Result<RoutingAppCoverageResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/appStoreVersions/{}/routingAppCoverage",
                    url::path_escape(version_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns sales reports.
    pub async fn get_sales_report(
        &self,
        vendor_number: &str,
        report_type: &str,
        report_sub_type: &str,
        frequency: &str,
        report_date: &str,
    ) -> Result<Vec<u8>> {
        let mut query = Values::new();
        query.set("filter[vendorNumber]", vendor_number);
        query.set("filter[reportType]", report_type);
        query.set("filter[reportSubType]", report_sub_type);
        query.set("filter[frequency]", frequency);
        query.set("filter[reportDate]", report_date);
        self.get("/v1/salesReports", Some(&query)).await
    }

    /// Returns alternative distribution keys.
    pub async fn list_alternative_distribution_keys(
        &self,
        opts: Option<&ListOptions>,
    ) -> Result<AlternativeDistributionKeysResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get("/v1/alternativeDistributionKeys", Some(&query))
            .await?;
        decode(&data)
    }

    /// Returns instances for a report.
    pub async fn list_analytics_report_instances(
        &self,
        report_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AnalyticsReportInstancesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/analyticsReports/{}/instances",
                    url::path_escape(report_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns analytics report requests for an app.
    pub async fn list_analytics_report_requests(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AnalyticsReportRequestsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/apps/{}/analyticsReportRequests",
                    url::path_escape(app_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns segments for a report instance.
    pub async fn list_analytics_report_segments(
        &self,
        instance_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AnalyticsReportSegmentsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/analyticsReportInstances/{}/segments",
                    url::path_escape(instance_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns analytics reports for a request.
    pub async fn list_analytics_reports(
        &self,
        request_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AnalyticsReportsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/analyticsReportRequests/{}/reports",
                    url::path_escape(request_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns all app categories.
    pub async fn list_app_categories(
        &self,
        opts: Option<&ListOptions>,
    ) -> Result<AppCategoriesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get("/v1/appCategories", Some(&query)).await?;
        decode(&data)
    }

    /// Returns review attachments.
    pub async fn list_app_store_review_attachments(
        &self,
        review_detail_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppStoreReviewAttachmentsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/appStoreReviewDetails/{}/appStoreReviewAttachments",
                    url::path_escape(review_detail_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns performance metrics for a build.
    pub async fn list_build_perf_power_metrics(
        &self,
        build_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<PerfPowerMetricsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/builds/{}/perfPowerMetrics", url::path_escape(build_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns diagnostic logs.
    pub async fn list_diagnostic_logs(
        &self,
        signature_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<DiagnosticLogsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/diagnosticSignatures/{}/logs",
                    url::path_escape(signature_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns diagnostic signatures.
    pub async fn list_diagnostic_signatures(
        &self,
        build_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<DiagnosticSignaturesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/builds/{}/diagnosticSignatures",
                    url::path_escape(build_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns performance and power metrics.
    pub async fn list_perf_power_metrics(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<PerfPowerMetricsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/apps/{}/perfPowerMetrics", url::path_escape(app_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Updates an EULA.
    pub async fn update_end_user_license_agreement(
        &self,
        agreement_id: &str,
        req: &EndUserLicenseAgreementUpdateRequest,
    ) -> Result<EndUserLicenseAgreementResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/endUserLicenseAgreements/{}",
                    url::path_escape(agreement_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Updates marketplace search details.
    pub async fn update_marketplace_search_detail(
        &self,
        detail_id: &str,
        req: &MarketplaceSearchDetailUpdateRequest,
    ) -> Result<MarketplaceSearchDetailResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/marketplaceSearchDetails/{}",
                    url::path_escape(detail_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Creates a background asset for an app.
    pub async fn create_background_asset(
        &self,
        req: &BackgroundAssetCreateRequest,
    ) -> Result<BackgroundAssetResponse> {
        let data = self.post("/v1/backgroundAssets", Some(req)).await?;
        decode(&data)
    }

    /// Reserves an upload file slot on a
    /// background asset version. The response carries the upload operations
    /// for the file's bytes.
    pub async fn create_background_asset_upload_file(
        &self,
        req: &BackgroundAssetUploadFileCreateRequest,
    ) -> Result<BackgroundAssetUploadFileResponse> {
        let data = self
            .post("/v1/backgroundAssetUploadFiles", Some(req))
            .await?;
        decode(&data)
    }

    /// Creates a new version on a background
    /// asset. The version number is assigned by Apple.
    pub async fn create_background_asset_version(
        &self,
        req: &BackgroundAssetVersionCreateRequest,
    ) -> Result<BackgroundAssetVersionResponse> {
        let data = self.post("/v1/backgroundAssetVersions", Some(req)).await?;
        decode(&data)
    }

    /// Returns a single background asset.
    pub async fn get_background_asset(
        &self,
        background_asset_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BackgroundAssetResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/backgroundAssets/{}",
                    url::path_escape(background_asset_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns a single background asset version.
    pub async fn get_background_asset_version(
        &self,
        version_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BackgroundAssetVersionResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/backgroundAssetVersions/{}",
                    url::path_escape(version_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns the App Store
    /// release record for a background asset version.
    pub async fn get_background_asset_version_app_store_release(
        &self,
        release_id: &str,
    ) -> Result<BackgroundAssetVersionReleaseResponse> {
        self.get_background_asset_version_release(&format!(
            "/v1/backgroundAssetVersionAppStoreReleases/{}",
            url::path_escape(release_id)
        ))
        .await
    }

    /// Returns the external
    /// beta (TestFlight) release record for a background asset version.
    pub async fn get_background_asset_version_external_beta_release(
        &self,
        release_id: &str,
    ) -> Result<BackgroundAssetVersionReleaseResponse> {
        self.get_background_asset_version_release(&format!(
            "/v1/backgroundAssetVersionExternalBetaReleases/{}",
            url::path_escape(release_id)
        ))
        .await
    }

    /// Returns the internal
    /// beta release record for a background asset version.
    pub async fn get_background_asset_version_internal_beta_release(
        &self,
        release_id: &str,
    ) -> Result<BackgroundAssetVersionReleaseResponse> {
        self.get_background_asset_version_release(&format!(
            "/v1/backgroundAssetVersionInternalBetaReleases/{}",
            url::path_escape(release_id)
        ))
        .await
    }

    /// Returns the upload files (asset pack
    /// and manifest) attached to a background asset version, including their
    /// delivery states.
    pub async fn list_background_asset_upload_files(
        &self,
        version_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BackgroundAssetUploadFilesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/backgroundAssetVersions/{}/backgroundAssetUploadFiles",
                    url::path_escape(version_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns the versions of a background
    /// asset, including their processing states and release states.
    pub async fn list_background_asset_versions(
        &self,
        background_asset_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BackgroundAssetVersionsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/backgroundAssets/{}/versions",
                    url::path_escape(background_asset_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns the background assets for an app.
    /// Supports filter[versions.locale] and filter[versions.platforms] for
    /// the multi-locale read added in API 4.4.
    pub async fn list_background_assets(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<BackgroundAssetsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/apps/{}/backgroundAssets", url::path_escape(app_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Updates a background asset (archives or
    /// unarchives it).
    pub async fn update_background_asset(
        &self,
        background_asset_id: &str,
        req: &BackgroundAssetUpdateRequest,
    ) -> Result<BackgroundAssetResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/backgroundAssets/{}",
                    url::path_escape(background_asset_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Commits (or amends) a background
    /// asset upload file.
    pub async fn update_background_asset_upload_file(
        &self,
        file_id: &str,
        req: &BackgroundAssetUploadFileUpdateRequest,
    ) -> Result<BackgroundAssetUploadFileResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/backgroundAssetUploadFiles/{}",
                    url::path_escape(file_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Reserves, uploads, and commits one file on
    /// an existing background asset version. assetType is ASSET (the asset
    /// pack archive) or MANIFEST (its manifest plist). The commit carries
    /// the SHA-256 of the body so Apple can verify the stored bytes.
    pub async fn upload_background_asset_file(
        &self,
        version_id: &str,
        file_name: &str,
        asset_type: &str,
        body: &[u8],
    ) -> Result<BackgroundAssetUploadFileResponse> {
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
        let create_req = BackgroundAssetUploadFileCreateRequest {
            data: BackgroundAssetUploadFileCreateData {
                type_: "backgroundAssetUploadFiles".into(),
                attributes: BackgroundAssetUploadFileCreateAttributes {
                    asset_type: asset_type.into(),
                    file_name: file_name.into(),
                    file_size: body.len() as i64,
                },
                relationships: BackgroundAssetUploadFileCreateRelationships {
                    background_asset_version: RelationshipData {
                        data: ResourceIdentifier {
                            type_: "backgroundAssetVersions".into(),
                            id: version_id.into(),
                        },
                    },
                },
            },
        };
        let reservation = self
            .create_background_asset_upload_file(&create_req)
            .await
            .map_err(|e| e.wrap("reservation"))?;
        let ops = &reservation.data.attributes.upload_operations;
        self.perform_upload_operations(ops, body)
            .await
            .map_err(|e| e.wrap("upload"))?;
        let commit = BackgroundAssetUploadFileUpdateRequest {
            data: BackgroundAssetUploadFileUpdateData {
                type_: "backgroundAssetUploadFiles".into(),
                id: reservation.data.id.clone(),
                attributes: Some(BackgroundAssetUploadFileUpdateAttributes {
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
        self.update_background_asset_upload_file(&reservation.data.id, &commit)
            .await
    }

    /// getBackgroundAssetVersionRelease fetches one of the three release
    /// resource shapes, which share a single response structure.
    async fn get_background_asset_version_release(
        &self,
        path: &str,
    ) -> Result<BackgroundAssetVersionReleaseResponse> {
        let data = self.get(path, None).await?;
        decode(&data)
    }

    /// Creates an accessibility declaration for an
    /// app and device family.
    pub async fn create_accessibility_declaration(
        &self,
        req: &AccessibilityDeclarationCreateRequest,
    ) -> Result<AccessibilityDeclarationResponse> {
        let data = self
            .post("/v1/accessibilityDeclarations", Some(req))
            .await?;
        decode(&data)
    }

    /// Deletes an accessibility declaration.
    pub async fn delete_accessibility_declaration(&self, declaration_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/accessibilityDeclarations/{}",
            url::path_escape(declaration_id)
        ))
        .await
    }

    /// Returns a single accessibility declaration.
    pub async fn get_accessibility_declaration(
        &self,
        declaration_id: &str,
    ) -> Result<AccessibilityDeclarationResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/accessibilityDeclarations/{}",
                    url::path_escape(declaration_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns the accessibility declarations for an app.
    pub async fn list_accessibility_declarations(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AccessibilityDeclarationsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/apps/{}/accessibilityDeclarations",
                    url::path_escape(app_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Updates an accessibility declaration.
    /// Setting publish=true in the request publishes the draft declaration.
    pub async fn update_accessibility_declaration(
        &self,
        declaration_id: &str,
        req: &AccessibilityDeclarationUpdateRequest,
    ) -> Result<AccessibilityDeclarationResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/accessibilityDeclarations/{}",
                    url::path_escape(declaration_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// Creates a webhook for an app.
    pub async fn create_webhook(&self, req: &WebhookCreateRequest) -> Result<WebhookResponse> {
        let data = self.post("/v1/webhooks", Some(req)).await?;
        decode(&data)
    }

    /// Sends a test ping event to a webhook endpoint.
    pub async fn create_webhook_ping(
        &self,
        req: &WebhookPingCreateRequest,
    ) -> Result<WebhookPingResponse> {
        let data = self.post("/v1/webhookPings", Some(req)).await?;
        decode(&data)
    }

    /// Requests redelivery of a webhook event using a
    /// previous delivery as the template.
    pub async fn create_webhook_redelivery(
        &self,
        req: &WebhookDeliveryCreateRequest,
    ) -> Result<WebhookDeliveryResponse> {
        let data = self.post("/v1/webhookDeliveries", Some(req)).await?;
        decode(&data)
    }

    /// Deletes a webhook.
    pub async fn delete_webhook(&self, webhook_id: &str) -> Result<()> {
        self.delete(&format!("/v1/webhooks/{}", url::path_escape(webhook_id)))
            .await
    }

    /// Returns a single webhook.
    pub async fn get_webhook(&self, webhook_id: &str) -> Result<WebhookResponse> {
        let data = self
            .get(
                &format!("/v1/webhooks/{}", url::path_escape(webhook_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// Returns delivery attempts for a webhook.
    pub async fn list_webhook_deliveries(
        &self,
        webhook_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<WebhookDeliveriesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/webhooks/{}/deliveries", url::path_escape(webhook_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Returns the webhooks configured for an app.
    pub async fn list_webhooks(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<WebhooksResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/apps/{}/webhooks", url::path_escape(app_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// Updates a webhook's configuration.
    pub async fn update_webhook(
        &self,
        webhook_id: &str,
        req: &WebhookUpdateRequest,
    ) -> Result<WebhookResponse> {
        let data = self
            .patch(
                &format!("/v1/webhooks/{}", url::path_escape(webhook_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::api::testutil::{MockApi, MockResponse};

    fn filter(k: &str, v: &str) -> ListOptions {
        let mut f = JsonMap::default();
        f.insert(k, JsonSlice::from(vec![v.to_string()]));
        ListOptions {
            filter: f,
            ..Default::default()
        }
    }

    fn rel(t: &str, id: &str) -> RelationshipData {
        RelationshipData {
            data: ResourceIdentifier {
                type_: t.into(),
                id: id.into(),
            },
        }
    }

    // ---- webhooks ----

    #[tokio::test]
    async fn test_client_list_webhooks() {
        let mock = MockApi::start(|r| {
            assert_eq!(r.path, "/v1/apps/123/webhooks");
            assert_eq!(r.method, "GET");
            MockResponse::json(200, r#"{"data":[{"type":"webhooks","id":"hook-1","attributes":{"enabled":true,"eventTypes":["BUILD_UPLOAD_STATE_UPDATED"],"name":"CI hook","url":"https://example.com/hook"}}],"links":{"self":""}}"#)
        })
        .await;
        let resp = mock.client().list_webhooks("123", None).await.unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].attributes.name, "CI hook");
    }

    #[tokio::test]
    async fn test_client_create_webhook() {
        let mock = MockApi::start(|r| {
            assert_eq!(r.path, "/v1/webhooks");
            assert_eq!(r.method, "POST");
            let req = r.json();
            assert_eq!(req["data"]["type"], "webhooks");
            assert_eq!(req["data"]["attributes"]["secret"], "s3cret");
            assert_eq!(req["data"]["relationships"]["app"]["data"]["id"], "123");
            MockResponse::json(200, r#"{"data":{"type":"webhooks","id":"hook-1","attributes":{"enabled":true,"name":"CI hook","url":"https://example.com/hook"}}}"#)
        })
        .await;
        let req = WebhookCreateRequest {
            data: WebhookCreateData {
                type_: "webhooks".into(),
                attributes: WebhookCreateAttributes {
                    enabled: true,
                    event_types: JsonSlice::from(vec!["BUILD_UPLOAD_STATE_UPDATED".to_string()]),
                    name: "CI hook".into(),
                    secret: "s3cret".into(),
                    url: "https://example.com/hook".into(),
                },
                relationships: WebhookCreateRelationships {
                    app: rel("apps", "123"),
                },
            },
        };
        let resp = mock.client().create_webhook(&req).await.unwrap();
        assert_eq!(resp.data.id, "hook-1");
    }

    #[tokio::test]
    async fn test_client_update_webhook() {
        let mock = MockApi::start(|r| {
            assert_eq!(r.path, "/v1/webhooks/hook-1");
            assert_eq!(r.method, "PATCH");
            let req = r.json();
            assert_eq!(req["data"]["id"], "hook-1");
            assert_eq!(
                req["data"]["attributes"]["enabled"],
                Value::Bool(false),
                "expected enabled=false in update attributes"
            );
            MockResponse::json(
                200,
                r#"{"data":{"type":"webhooks","id":"hook-1","attributes":{}}}"#,
            )
        })
        .await;
        let req = WebhookUpdateRequest {
            data: WebhookUpdateData {
                type_: "webhooks".into(),
                id: "hook-1".into(),
                attributes: Some(WebhookUpdateAttributes {
                    enabled: Some(false),
                    ..Default::default()
                }),
            },
        };
        let resp = mock.client().update_webhook("hook-1", &req).await.unwrap();
        assert!(
            !resp.data.attributes.enabled,
            "expected webhook to be disabled"
        );
    }

    #[tokio::test]
    async fn test_client_delete_webhook() {
        let mock = MockApi::start(|r| {
            assert_eq!(r.path, "/v1/webhooks/hook-1");
            assert_eq!(r.method, "DELETE");
            MockResponse::empty(204)
        })
        .await;
        mock.client().delete_webhook("hook-1").await.unwrap();
    }

    #[tokio::test]
    async fn test_client_list_webhook_deliveries() {
        let mock = MockApi::start(|r| {
            assert_eq!(r.path, "/v1/webhooks/hook-1/deliveries");
            assert_eq!(r.method, "GET");
            assert_eq!(r.query_param("filter[deliveryState]").as_deref(), Some("FAILED"));
            MockResponse::json(200, r#"{"data":[{"type":"webhookDeliveries","id":"delivery-1","attributes":{"deliveryState":"FAILED","errorMessage":"connection refused"}}],"links":{"self":""}}"#)
        })
        .await;
        let opts = filter("deliveryState", "FAILED");
        let resp = mock
            .client()
            .list_webhook_deliveries("hook-1", Some(&opts))
            .await
            .unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].attributes.delivery_state, "FAILED");
    }

    #[tokio::test]
    async fn test_client_create_webhook_redelivery() {
        let mock = MockApi::start(|r| {
            assert_eq!(r.path, "/v1/webhookDeliveries");
            assert_eq!(r.method, "POST");
            assert_eq!(r.json()["data"]["relationships"]["template"]["data"]["id"], "delivery-1");
            MockResponse::json(200, r#"{"data":{"type":"webhookDeliveries","id":"delivery-2","attributes":{"redelivery":true}}}"#)
        })
        .await;
        let req = WebhookDeliveryCreateRequest {
            data: WebhookDeliveryCreateData {
                type_: "webhookDeliveries".into(),
                relationships: WebhookDeliveryCreateRelationships {
                    template: rel("webhookDeliveries", "delivery-1"),
                },
            },
        };
        let resp = mock.client().create_webhook_redelivery(&req).await.unwrap();
        assert!(
            resp.data.attributes.redelivery,
            "expected redelivery to be true"
        );
    }

    #[tokio::test]
    async fn test_client_create_webhook_ping() {
        let mock = MockApi::start(|r| {
            assert_eq!(r.path, "/v1/webhookPings");
            assert_eq!(r.method, "POST");
            assert_eq!(
                r.json()["data"]["relationships"]["webhook"]["data"]["id"],
                "hook-1"
            );
            MockResponse::json(200, r#"{"data":{"type":"webhookPings","id":"ping-1"}}"#)
        })
        .await;
        let req = WebhookPingCreateRequest {
            data: WebhookPingCreateData {
                type_: "webhookPings".into(),
                relationships: WebhookPingCreateRelationships {
                    webhook: rel("webhooks", "hook-1"),
                },
            },
        };
        let resp = mock.client().create_webhook_ping(&req).await.unwrap();
        assert_eq!(resp.data.id, "ping-1");
    }

    // ---- background assets ----

    #[tokio::test]
    async fn test_client_list_background_assets() {
        let mock = MockApi::start(|r| {
            assert_eq!(r.path, "/v1/apps/123/backgroundAssets");
            assert_eq!(r.method, "GET");
            assert_eq!(r.query_param("filter[versions.locale]").as_deref(), Some("en-US"));
            MockResponse::json(200, r#"{"data":[{"type":"backgroundAssets","id":"asset-1","attributes":{"assetPackIdentifier":"com.example.game.levels1","usedBytes":1024}}],"links":{"self":""}}"#)
        })
        .await;
        let opts = filter("versions.locale", "en-US");
        let resp = mock
            .client()
            .list_background_assets("123", Some(&opts))
            .await
            .unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(
            resp.data[0].attributes.asset_pack_identifier,
            "com.example.game.levels1"
        );
        assert_eq!(resp.data[0].attributes.used_bytes, 1024);
    }

    #[tokio::test]
    async fn test_client_create_background_asset() {
        let mock = MockApi::start(|r| {
            assert_eq!(r.path, "/v1/backgroundAssets");
            assert_eq!(r.method, "POST");
            let req = r.json();
            assert_eq!(req["data"]["type"], "backgroundAssets");
            assert_eq!(req["data"]["attributes"]["assetPackIdentifier"], "com.example.game.levels1");
            assert_eq!(req["data"]["relationships"]["app"]["data"]["id"], "123");
            MockResponse::json(200, r#"{"data":{"type":"backgroundAssets","id":"asset-1","attributes":{"assetPackIdentifier":"com.example.game.levels1"}}}"#)
        })
        .await;
        let req = BackgroundAssetCreateRequest {
            data: BackgroundAssetCreateData {
                type_: "backgroundAssets".into(),
                attributes: BackgroundAssetCreateAttributes {
                    asset_pack_identifier: "com.example.game.levels1".into(),
                },
                relationships: BackgroundAssetCreateRelationships {
                    app: rel("apps", "123"),
                },
            },
        };
        let resp = mock.client().create_background_asset(&req).await.unwrap();
        assert_eq!(resp.data.id, "asset-1");
    }

    #[tokio::test]
    async fn test_client_update_background_asset() {
        let mock = MockApi::start(|r| {
            assert_eq!(r.path, "/v1/backgroundAssets/asset-1");
            assert_eq!(r.method, "PATCH");
            assert_eq!(r.json()["data"]["attributes"]["archived"], Value::Bool(true));
            MockResponse::json(200, r#"{"data":{"type":"backgroundAssets","id":"asset-1","attributes":{"archived":true}}}"#)
        })
        .await;
        let req = BackgroundAssetUpdateRequest {
            data: BackgroundAssetUpdateData {
                type_: "backgroundAssets".into(),
                id: "asset-1".into(),
                attributes: Some(BackgroundAssetUpdateAttributes {
                    archived: Some(true),
                }),
            },
        };
        let resp = mock
            .client()
            .update_background_asset("asset-1", &req)
            .await
            .unwrap();
        assert!(resp.data.attributes.archived, "archived = false, want true");
    }

    #[tokio::test]
    async fn test_client_list_background_asset_versions() {
        let mock = MockApi::start(|r| {
            assert_eq!(r.path, "/v1/backgroundAssets/asset-1/versions");
            assert_eq!(r.method, "GET");
            assert_eq!(r.query_param("filter[state]").as_deref(), Some("COMPLETE"));
            MockResponse::json(200, r#"{"data":[{"type":"backgroundAssetVersions","id":"version-1","attributes":{"state":"COMPLETE","version":"3","locale":"en-US"}}],"links":{"self":""}}"#)
        })
        .await;
        let opts = filter("state", "COMPLETE");
        let resp = mock
            .client()
            .list_background_asset_versions("asset-1", Some(&opts))
            .await
            .unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].attributes.state, "COMPLETE");
    }

    #[tokio::test]
    async fn test_client_get_background_asset_version() {
        let mock = MockApi::start(|r| {
            assert_eq!(r.path, "/v1/backgroundAssetVersions/version-1");
            assert_eq!(r.method, "GET");
            MockResponse::json(200, r#"{"data":{"type":"backgroundAssetVersions","id":"version-1","attributes":{"state":"FAILED","stateDetails":{"errors":[{"code":"INVALID_MANIFEST","description":"bad manifest"}]}},"relationships":{"appStoreRelease":{"data":{"type":"backgroundAssetVersionAppStoreReleases","id":"release-9"}}}}}"#)
        })
        .await;
        let resp = mock
            .client()
            .get_background_asset_version("version-1", None)
            .await
            .unwrap();
        assert_eq!(resp.data.attributes.state, "FAILED");
        let details = resp
            .data
            .attributes
            .state_details
            .as_ref()
            .expect("state details");
        assert_eq!(details.errors.len(), 1);
        assert_eq!(details.errors[0].code, "INVALID_MANIFEST");
        let rels = resp.data.relationships.as_ref().unwrap();
        assert_eq!(
            rels.app_store_release.as_ref().unwrap().data.id,
            "release-9"
        );
    }

    #[tokio::test]
    async fn test_client_create_background_asset_version() {
        let mock = MockApi::start(|r| {
            assert_eq!(r.path, "/v1/backgroundAssetVersions");
            assert_eq!(r.method, "POST");
            let req = r.json();
            assert_eq!(req["data"]["type"], "backgroundAssetVersions");
            assert_eq!(req["data"]["relationships"]["backgroundAsset"]["data"]["id"], "asset-1");
            MockResponse::json(200, r#"{"data":{"type":"backgroundAssetVersions","id":"version-1","attributes":{"state":"AWAITING_UPLOAD"}}}"#)
        })
        .await;
        let req = BackgroundAssetVersionCreateRequest {
            data: BackgroundAssetVersionCreateData {
                type_: "backgroundAssetVersions".into(),
                relationships: BackgroundAssetVersionCreateRelationships {
                    background_asset: rel("backgroundAssets", "asset-1"),
                },
            },
        };
        let resp = mock
            .client()
            .create_background_asset_version(&req)
            .await
            .unwrap();
        assert_eq!(resp.data.id, "version-1");
    }

    #[tokio::test]
    async fn test_client_list_background_asset_upload_files() {
        let mock = MockApi::start(|r| {
            assert_eq!(r.path, "/v1/backgroundAssetVersions/version-1/backgroundAssetUploadFiles");
            assert_eq!(r.method, "GET");
            MockResponse::json(200, r#"{"data":[{"type":"backgroundAssetUploadFiles","id":"file-1","attributes":{"fileName":"levels1.aar","fileSize":4,"assetType":"ASSET","assetDeliveryState":{"state":"COMPLETE"}}}],"links":{"self":""}}"#)
        })
        .await;
        let resp = mock
            .client()
            .list_background_asset_upload_files("version-1", None)
            .await
            .unwrap();
        assert_eq!(resp.data.len(), 1);
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
    async fn test_client_get_background_asset_version_releases() {
        for (name, want_path) in [
            (
                "internal beta",
                "/v1/backgroundAssetVersionInternalBetaReleases/release-1",
            ),
            (
                "external beta",
                "/v1/backgroundAssetVersionExternalBetaReleases/release-1",
            ),
            (
                "app store",
                "/v1/backgroundAssetVersionAppStoreReleases/release-1",
            ),
        ] {
            let want = want_path.to_string();
            let mock = MockApi::start(move |r| {
                assert_eq!(r.path, want);
                assert_eq!(r.method, "GET");
                MockResponse::json(200, r#"{"data":{"type":"","id":"release-1","attributes":{"state":"READY_FOR_TESTING"}}}"#)
            })
            .await;
            let c = mock.client();
            let resp = match name {
                "internal beta" => {
                    c.get_background_asset_version_internal_beta_release("release-1")
                        .await
                }
                "external beta" => {
                    c.get_background_asset_version_external_beta_release("release-1")
                        .await
                }
                _ => {
                    c.get_background_asset_version_app_store_release("release-1")
                        .await
                }
            }
            .unwrap();
            assert_eq!(resp.data.attributes.state, "READY_FOR_TESTING", "{name}");
            assert_eq!(mock.requests().len(), 1, "{name}");
        }
    }

    #[tokio::test]
    async fn test_upload_background_asset_file_end_to_end() {
        let uploaded = Arc::new(AtomicBool::new(false));
        let committed = Arc::new(AtomicBool::new(false));
        let storage_url = Arc::new(Mutex::new(String::new()));
        let (u, cm, su) = (uploaded.clone(), committed.clone(), storage_url.clone());
        let mock = MockApi::start(move |r| match (r.method.as_str(), r.path.as_str()) {
            ("POST", "/v1/backgroundAssetUploadFiles") => {
                let req = r.json();
                assert_eq!(req["data"]["attributes"]["assetType"], "MANIFEST");
                assert_eq!(req["data"]["relationships"]["backgroundAssetVersion"]["data"]["id"], "version-1");
                let url = format!("{}/upload", su.lock().unwrap());
                let body = serde_json::json!({"data": {"type": "backgroundAssetUploadFiles", "id": "file-1", "attributes": {
                    "fileName": "Manifest.plist", "fileSize": 4,
                    "uploadOperations": [{"method": "PUT", "url": url, "offset": 0, "length": 4,
                        "requestHeaders": [{"name": "Content-Type", "value": "application/octet-stream"}]}]}}});
                MockResponse::json(201, body.to_string())
            }
            ("PUT", "/upload") => {
                assert_eq!(r.body, b"PACK");
                u.store(true, Ordering::SeqCst);
                MockResponse::empty(200)
            }
            ("PATCH", "/v1/backgroundAssetUploadFiles/file-1") => {
                let commit = r.json();
                let file = &commit["data"]["attributes"]["sourceFileChecksums"]["file"];
                assert!(file.is_object(), "commit checksums missing");
                assert_eq!(file["hash"], crate::api::sha256_hex(b"PACK"));
                assert_eq!(file["algorithm"], "SHA_256");
                assert_eq!(commit["data"]["attributes"]["uploaded"], Value::Bool(true), "commit uploaded flag not true");
                cm.store(true, Ordering::SeqCst);
                MockResponse::json(200, r#"{"data":{"type":"backgroundAssetUploadFiles","id":"file-1","attributes":{}}}"#)
            }
            (m, p) => panic!("unexpected request: {m} {p}"),
        })
        .await;
        *storage_url.lock().unwrap() = mock.base_url.clone();
        let resp = mock
            .client()
            .upload_background_asset_file("version-1", "Manifest.plist", "MANIFEST", b"PACK")
            .await
            .unwrap();
        assert_eq!(resp.data.id, "file-1");
        assert!(uploaded.load(Ordering::SeqCst), "upload step did not run");
        assert!(committed.load(Ordering::SeqCst), "commit step did not run");
    }

    #[tokio::test]
    async fn test_upload_background_asset_file_validation() {
        let mock = MockApi::start(|_| panic!("no request expected")).await;
        let c = mock.client();
        for (name, file_name, body) in [
            ("missing file name", "", &b"PACK"[..]),
            ("empty body", "levels1.aar", &b""[..]),
        ] {
            assert!(
                c.upload_background_asset_file("version-1", file_name, "ASSET", body)
                    .await
                    .is_err(),
                "{name}: expected error"
            );
        }
        assert!(mock.requests().is_empty());
    }
}
