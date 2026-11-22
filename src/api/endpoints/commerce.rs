// App Store Connect API endpoints for the "commerce" area.
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
    /// GetAppPriceSchedule returns the price schedule for an app.
    pub async fn get_app_price_schedule(&self, app_id: &str) -> Result<AppPriceScheduleResponse> {
        let data = self.get(&format!("/v1/apps/{}/appPriceSchedule", url::path_escape(app_id)), None).await?;
        decode(&data)
    }

    /// ListAppPricePoints returns price points for an app.
    pub async fn list_app_price_points(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<AppPricePointsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/apps/{}/appPricePoints", url::path_escape(app_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListSubscriptionPricePoints returns price points for a subscription.
    pub async fn list_subscription_price_points(&self, subscription_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionPricePointsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/subscriptions/{}/pricePoints", url::path_escape(subscription_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListTerritories returns all territories.
    pub async fn list_territories(&self, opts: Option<&ListOptions>) -> Result<TerritoriesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get("/v1/territories", Some(&query)).await?;
        decode(&data)
    }

    /// CreateAppPriceSchedule replaces an app's price schedule.
    pub async fn create_app_price_schedule(&self, req: &AppPriceScheduleCreateRequest) -> Result<AppPriceScheduleResponse> {
        let data = self.post("/v1/appPriceSchedules", Some(req)).await?;
        decode(&data)
    }

    /// CreateInAppPurchaseAvailability sets the territory availability of an
    /// in-app purchase.
    pub async fn create_in_app_purchase_availability(&self, req: &InAppPurchaseAvailabilityCreateRequest) -> Result<InAppPurchaseAvailabilityResponse> {
        let data = self.post("/v1/inAppPurchaseAvailabilities", Some(req)).await?;
        decode(&data)
    }

    /// CreateInAppPurchaseImage reserves an image upload on an in-app
    /// purchase version. The response carries the upload operations to
    /// perform before committing with UpdateInAppPurchaseImage.
    pub async fn create_in_app_purchase_image(&self, req: &InAppPurchaseImageCreateRequest) -> Result<InAppPurchaseImageResponse> {
        let data = self.post("/v2/inAppPurchaseImages", Some(req)).await?;
        decode(&data)
    }

    /// CreateInAppPurchaseLocalization creates a localization on an in-app
    /// purchase version.
    pub async fn create_in_app_purchase_localization(&self, req: &InAppPurchaseLocalizationCreateRequest) -> Result<InAppPurchaseLocalizationResponse> {
        let data = self.post("/v2/inAppPurchaseLocalizations", Some(req)).await?;
        decode(&data)
    }

    /// CreateInAppPurchaseOfferCode creates an in-app purchase offer code.
    pub async fn create_in_app_purchase_offer_code(&self, req: &InAppPurchaseOfferCodeCreateRequest) -> Result<InAppPurchaseOfferCodeResponse> {
        let data = self.post("/v1/inAppPurchaseOfferCodes", Some(req)).await?;
        decode(&data)
    }

    /// CreateInAppPurchaseOfferCodeCustomCode creates a custom code batch.
    pub async fn create_in_app_purchase_offer_code_custom_code(&self, req: &InAppPurchaseOfferCodeCustomCodeCreateRequest) -> Result<InAppPurchaseOfferCodeCustomCodeResponse> {
        let data = self.post("/v1/inAppPurchaseOfferCodeCustomCodes", Some(req)).await?;
        decode(&data)
    }

    /// CreateInAppPurchaseOfferCodeOneTimeUseCode creates a one-time-use code batch.
    pub async fn create_in_app_purchase_offer_code_one_time_use_code(&self, req: &InAppPurchaseOfferCodeOneTimeUseCodeCreateRequest) -> Result<InAppPurchaseOfferCodeOneTimeUseCodeResponse> {
        let data = self.post("/v1/inAppPurchaseOfferCodeOneTimeUseCodes", Some(req)).await?;
        decode(&data)
    }

    /// CreateInAppPurchasePriceSchedule replaces an in-app purchase's price schedule.
    pub async fn create_in_app_purchase_price_schedule(&self, req: &InAppPurchasePriceScheduleCreateRequest) -> Result<InAppPurchasePriceScheduleResponse> {
        let data = self.post("/v1/inAppPurchasePriceSchedules", Some(req)).await?;
        decode(&data)
    }

    /// CreateInAppPurchaseVersion creates a new version of an in-app
    /// purchase, snapshotting its current metadata for editing.
    pub async fn create_in_app_purchase_version(&self, req: &InAppPurchaseVersionCreateRequest) -> Result<InAppPurchaseVersionResponse> {
        let data = self.post("/v1/inAppPurchaseVersions", Some(req)).await?;
        decode(&data)
    }

    /// CreateSubscriptionGroupLocalization creates a localization on a
    /// subscription group version.
    pub async fn create_subscription_group_localization(&self, req: &SubscriptionGroupLocalizationCreateRequest) -> Result<SubscriptionGroupLocalizationResponse> {
        let data = self.post("/v2/subscriptionGroupLocalizations", Some(req)).await?;
        decode(&data)
    }

    /// CreateSubscriptionGroupVersion creates a new version of a subscription group.
    pub async fn create_subscription_group_version(&self, req: &SubscriptionGroupVersionCreateRequest) -> Result<SubscriptionGroupVersionResponse> {
        let data = self.post("/v1/subscriptionGroupVersions", Some(req)).await?;
        decode(&data)
    }

    /// CreateSubscriptionImage reserves an image upload on a subscription version.
    pub async fn create_subscription_image(&self, req: &SubscriptionImageCreateRequest) -> Result<SubscriptionImageResponse> {
        let data = self.post("/v2/subscriptionImages", Some(req)).await?;
        decode(&data)
    }

    /// CreateSubscriptionLocalization creates a localization on a subscription version.
    pub async fn create_subscription_localization(&self, req: &SubscriptionLocalizationCreateRequest) -> Result<SubscriptionLocalizationResponse> {
        let data = self.post("/v2/subscriptionLocalizations", Some(req)).await?;
        decode(&data)
    }

    /// CreateSubscriptionPlanAvailability configures the territories one
    /// billing plan of a subscription is available in.
    pub async fn create_subscription_plan_availability(&self, req: &SubscriptionPlanAvailabilityCreateRequest) -> Result<SubscriptionPlanAvailabilityResponse> {
        let data = self.post("/v1/subscriptionPlanAvailabilities", Some(req)).await?;
        decode(&data)
    }

    /// CreateSubscriptionVersion creates a new version of a subscription.
    pub async fn create_subscription_version(&self, req: &SubscriptionVersionCreateRequest) -> Result<SubscriptionVersionResponse> {
        let data = self.post("/v1/subscriptionVersions", Some(req)).await?;
        decode(&data)
    }

    /// DeleteInAppPurchaseImage deletes an in-app purchase image.
    pub async fn delete_in_app_purchase_image(&self, image_id: &str) -> Result<()> {
        self.delete(&format!("/v2/inAppPurchaseImages/{}", url::path_escape(image_id))).await
    }

    /// DeleteInAppPurchaseLocalization deletes an in-app purchase localization.
    pub async fn delete_in_app_purchase_localization(&self, localization_id: &str) -> Result<()> {
        self.delete(&format!("/v2/inAppPurchaseLocalizations/{}", url::path_escape(localization_id))).await
    }

    /// DeleteSubscriptionGroupLocalization deletes a subscription group localization.
    pub async fn delete_subscription_group_localization(&self, localization_id: &str) -> Result<()> {
        self.delete(&format!("/v2/subscriptionGroupLocalizations/{}", url::path_escape(localization_id))).await
    }

    /// DeleteSubscriptionImage deletes a subscription image.
    pub async fn delete_subscription_image(&self, image_id: &str) -> Result<()> {
        self.delete(&format!("/v2/subscriptionImages/{}", url::path_escape(image_id))).await
    }

    /// DeleteSubscriptionLocalization deletes a subscription localization.
    pub async fn delete_subscription_localization(&self, localization_id: &str) -> Result<()> {
        self.delete(&format!("/v2/subscriptionLocalizations/{}", url::path_escape(localization_id))).await
    }

    /// GetAppPricePoint returns a single app price point.
    pub async fn get_app_price_point(&self, price_point_id: &str) -> Result<AppPricePointResponse> {
        let data = self.get(&format!("/v3/appPricePoints/{}", url::path_escape(price_point_id)), None).await?;
        decode(&data)
    }

    /// GetInAppPurchaseAvailability returns the territory availability of an
    /// in-app purchase.
    pub async fn get_in_app_purchase_availability(&self, iap_id: &str) -> Result<InAppPurchaseAvailabilityResponse> {
        let data = self.get(&format!("/v2/inAppPurchases/{}/inAppPurchaseAvailability", url::path_escape(iap_id)), None).await?;
        decode(&data)
    }

    /// GetInAppPurchaseOfferCode returns a single in-app purchase offer code.
    pub async fn get_in_app_purchase_offer_code(&self, offer_code_id: &str) -> Result<InAppPurchaseOfferCodeResponse> {
        let data = self.get(&format!("/v1/inAppPurchaseOfferCodes/{}", url::path_escape(offer_code_id)), None).await?;
        decode(&data)
    }

    /// GetInAppPurchaseOfferCodeOneTimeUseCodeValues downloads the generated
    /// codes of a one-time-use batch. Apple returns them as CSV, not JSON, so
    /// the raw body is handed back to the caller.
    pub async fn get_in_app_purchase_offer_code_one_time_use_code_values(&self, code_id: &str) -> Result<String> {
        let data = self.get(&format!("/v1/inAppPurchaseOfferCodeOneTimeUseCodes/{}/values", url::path_escape(code_id)), None).await?;
        Ok(String::from_utf8_lossy(&data).into_owned())
    }

    /// GetInAppPurchasePriceSchedule returns the price schedule of an in-app purchase.
    pub async fn get_in_app_purchase_price_schedule(&self, iap_id: &str) -> Result<InAppPurchasePriceScheduleResponse> {
        let data = self.get(&format!("/v2/inAppPurchases/{}/iapPriceSchedule", url::path_escape(iap_id)), None).await?;
        decode(&data)
    }

    /// GetInAppPurchaseVersion returns a single in-app purchase version.
    pub async fn get_in_app_purchase_version(&self, version_id: &str) -> Result<InAppPurchaseVersionResponse> {
        let data = self.get(&format!("/v1/inAppPurchaseVersions/{}", url::path_escape(version_id)), None).await?;
        decode(&data)
    }

    /// GetSubscriptionGroupVersion returns a single subscription group version.
    pub async fn get_subscription_group_version(&self, version_id: &str) -> Result<SubscriptionGroupVersionResponse> {
        let data = self.get(&format!("/v1/subscriptionGroupVersions/{}", url::path_escape(version_id)), None).await?;
        decode(&data)
    }

    /// GetSubscriptionPlanAvailability returns a single subscription plan availability.
    pub async fn get_subscription_plan_availability(&self, availability_id: &str) -> Result<SubscriptionPlanAvailabilityResponse> {
        let data = self.get(&format!("/v1/subscriptionPlanAvailabilities/{}", url::path_escape(availability_id)), None).await?;
        decode(&data)
    }

    /// GetSubscriptionPricePoint returns a single subscription price point.
    pub async fn get_subscription_price_point(&self, price_point_id: &str) -> Result<SubscriptionPricePointResponse> {
        let data = self.get(&format!("/v1/subscriptionPricePoints/{}", url::path_escape(price_point_id)), None).await?;
        decode(&data)
    }

    /// GetSubscriptionVersion returns a single subscription version.
    pub async fn get_subscription_version(&self, version_id: &str) -> Result<SubscriptionVersionResponse> {
        let data = self.get(&format!("/v1/subscriptionVersions/{}", url::path_escape(version_id)), None).await?;
        decode(&data)
    }

    /// ListAppPricePointEqualizations returns the app price points in other
    /// territories that are equivalent to the given one.
    pub async fn list_app_price_point_equalizations(&self, price_point_id: &str, opts: Option<&ListOptions>) -> Result<AppPricePointsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v3/appPricePoints/{}/equalizations", url::path_escape(price_point_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListInAppPurchaseAvailableTerritories returns the territories covered
    /// by an in-app purchase availability.
    pub async fn list_in_app_purchase_available_territories(&self, availability_id: &str, opts: Option<&ListOptions>) -> Result<TerritoriesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/inAppPurchaseAvailabilities/{}/availableTerritories", url::path_escape(availability_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListInAppPurchaseOfferCodeCustomCodes returns the custom code batches
    /// of an in-app purchase offer code.
    pub async fn list_in_app_purchase_offer_code_custom_codes(&self, offer_code_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchaseOfferCodeCustomCodesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/inAppPurchaseOfferCodes/{}/customCodes", url::path_escape(offer_code_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListInAppPurchaseOfferCodeOneTimeUseCodes returns the one-time-use code
    /// batches of an in-app purchase offer code.
    pub async fn list_in_app_purchase_offer_code_one_time_use_codes(&self, offer_code_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchaseOfferCodeOneTimeUseCodesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/inAppPurchaseOfferCodes/{}/oneTimeUseCodes", url::path_escape(offer_code_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListInAppPurchaseOfferCodePrices returns the per-territory prices of an
    /// in-app purchase offer code.
    pub async fn list_in_app_purchase_offer_code_prices(&self, offer_code_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchaseOfferPricesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/inAppPurchaseOfferCodes/{}/prices", url::path_escape(offer_code_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListInAppPurchaseOfferCodes returns the offer codes of an in-app purchase.
    pub async fn list_in_app_purchase_offer_codes(&self, iap_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchaseOfferCodesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v2/inAppPurchases/{}/offerCodes", url::path_escape(iap_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListInAppPurchasePricePoints returns the price points available to an
    /// in-app purchase.
    pub async fn list_in_app_purchase_price_points(&self, iap_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchasePricePointsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v2/inAppPurchases/{}/pricePoints", url::path_escape(iap_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListInAppPurchasePriceSchedulePrices returns the prices of an in-app
    /// purchase price schedule. Manual prices are the ones the developer set;
    /// automatic prices are the ones Apple derived for other territories.
    pub async fn list_in_app_purchase_price_schedule_prices(&self, schedule_id: &str, automatic: bool, opts: Option<&ListOptions>) -> Result<InAppPurchasePricesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let relationship = if automatic { "automaticPrices" } else { "manualPrices" };
        let data = self.get(&format!("/v1/inAppPurchasePriceSchedules/{}/{}", url::path_escape(schedule_id), relationship), Some(&query)).await?;
        decode(&data)
    }

    /// ListInAppPurchaseVersionImages returns the images of an in-app purchase version.
    pub async fn list_in_app_purchase_version_images(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchaseImagesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/inAppPurchaseVersions/{}/images", url::path_escape(version_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListInAppPurchaseVersionLocalizations returns the localizations of an
    /// in-app purchase version.
    pub async fn list_in_app_purchase_version_localizations(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchaseLocalizationsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/inAppPurchaseVersions/{}/localizations", url::path_escape(version_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListInAppPurchaseVersions returns the versions of an in-app purchase.
    pub async fn list_in_app_purchase_versions(&self, iap_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchaseVersionsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v2/inAppPurchases/{}/versions", url::path_escape(iap_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListSubscriptionGroupVersionLocalizations returns the localizations of
    /// a subscription group version.
    pub async fn list_subscription_group_version_localizations(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionGroupLocalizationsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/subscriptionGroupVersions/{}/localizations", url::path_escape(version_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListSubscriptionGroupVersions returns the versions of a subscription group.
    pub async fn list_subscription_group_versions(&self, group_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionGroupVersionsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/subscriptionGroups/{}/versions", url::path_escape(group_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListSubscriptionPlanAvailabilities returns the plan availabilities of a
    /// subscription, one per billing plan.
    pub async fn list_subscription_plan_availabilities(&self, subscription_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionPlanAvailabilitiesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/subscriptions/{}/planAvailabilities", url::path_escape(subscription_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListSubscriptionPlanAvailableTerritories returns the territories
    /// covered by a subscription plan availability.
    pub async fn list_subscription_plan_available_territories(&self, availability_id: &str, opts: Option<&ListOptions>) -> Result<TerritoriesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/subscriptionPlanAvailabilities/{}/availableTerritories", url::path_escape(availability_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListSubscriptionPricePointAdjustedEqualizations returns equalized price
    /// points adjusted for a pre-paid (upfront) plan, so a monthly price point
    /// can be matched against the upfront plan's price point
    /// (App Store Connect API 4.4.1). Pass filter[upfrontPricePointId] and
    /// filter[planType] through opts.
    pub async fn list_subscription_price_point_adjusted_equalizations(&self, price_point_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionPricePointsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/subscriptionPricePoints/{}/adjustedEqualizations", url::path_escape(price_point_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListSubscriptionPricePointEqualizations returns the price points in
    /// other territories that are equivalent to the given one.
    pub async fn list_subscription_price_point_equalizations(&self, price_point_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionPricePointsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/subscriptionPricePoints/{}/equalizations", url::path_escape(price_point_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListSubscriptionVersionImages returns the images of a subscription version.
    pub async fn list_subscription_version_images(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionImagesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/subscriptionVersions/{}/images", url::path_escape(version_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListSubscriptionVersionLocalizations returns the localizations of a
    /// subscription version.
    pub async fn list_subscription_version_localizations(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionLocalizationsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/subscriptionVersions/{}/localizations", url::path_escape(version_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListSubscriptionVersions returns the versions of a subscription.
    pub async fn list_subscription_versions(&self, subscription_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionVersionsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/subscriptions/{}/versions", url::path_escape(subscription_id)), Some(&query)).await?;
        decode(&data)
    }

    /// UpdateInAppPurchaseImage commits an in-app purchase image upload.
    pub async fn update_in_app_purchase_image(&self, image_id: &str, req: &InAppPurchaseImageUpdateRequest) -> Result<InAppPurchaseImageResponse> {
        let data = self.patch(&format!("/v2/inAppPurchaseImages/{}", url::path_escape(image_id)), Some(req)).await?;
        decode(&data)
    }

    /// UpdateInAppPurchaseLocalization updates an in-app purchase localization.
    pub async fn update_in_app_purchase_localization(&self, localization_id: &str, req: &InAppPurchaseLocalizationUpdateRequest) -> Result<InAppPurchaseLocalizationResponse> {
        let data = self.patch(&format!("/v2/inAppPurchaseLocalizations/{}", url::path_escape(localization_id)), Some(req)).await?;
        decode(&data)
    }

    /// UpdateInAppPurchaseOfferCode activates or deactivates an in-app
    /// purchase offer code.
    pub async fn update_in_app_purchase_offer_code(&self, offer_code_id: &str, req: &InAppPurchaseOfferCodeUpdateRequest) -> Result<InAppPurchaseOfferCodeResponse> {
        let data = self.patch(&format!("/v1/inAppPurchaseOfferCodes/{}", url::path_escape(offer_code_id)), Some(req)).await?;
        decode(&data)
    }

    /// UpdateInAppPurchaseOfferCodeCustomCode activates or deactivates a
    /// custom code batch.
    pub async fn update_in_app_purchase_offer_code_custom_code(&self, custom_code_id: &str, req: &InAppPurchaseOfferCodeCustomCodeUpdateRequest) -> Result<InAppPurchaseOfferCodeCustomCodeResponse> {
        let data = self.patch(&format!("/v1/inAppPurchaseOfferCodeCustomCodes/{}", url::path_escape(custom_code_id)), Some(req)).await?;
        decode(&data)
    }

    /// UpdateInAppPurchaseOfferCodeOneTimeUseCode activates or deactivates a
    /// one-time-use code batch.
    pub async fn update_in_app_purchase_offer_code_one_time_use_code(&self, code_id: &str, req: &InAppPurchaseOfferCodeOneTimeUseCodeUpdateRequest) -> Result<InAppPurchaseOfferCodeOneTimeUseCodeResponse> {
        let data = self.patch(&format!("/v1/inAppPurchaseOfferCodeOneTimeUseCodes/{}", url::path_escape(code_id)), Some(req)).await?;
        decode(&data)
    }

    /// UpdateSubscriptionGroupLocalization updates a subscription group localization.
    pub async fn update_subscription_group_localization(&self, localization_id: &str, req: &SubscriptionGroupLocalizationUpdateRequest) -> Result<SubscriptionGroupLocalizationResponse> {
        let data = self.patch(&format!("/v2/subscriptionGroupLocalizations/{}", url::path_escape(localization_id)), Some(req)).await?;
        decode(&data)
    }

    /// UpdateSubscriptionImage commits a subscription image upload.
    pub async fn update_subscription_image(&self, image_id: &str, req: &SubscriptionImageUpdateRequest) -> Result<SubscriptionImageResponse> {
        let data = self.patch(&format!("/v2/subscriptionImages/{}", url::path_escape(image_id)), Some(req)).await?;
        decode(&data)
    }

    /// UpdateSubscriptionLocalization updates a subscription localization.
    pub async fn update_subscription_localization(&self, localization_id: &str, req: &SubscriptionLocalizationUpdateRequest) -> Result<SubscriptionLocalizationResponse> {
        let data = self.patch(&format!("/v2/subscriptionLocalizations/{}", url::path_escape(localization_id)), Some(req)).await?;
        decode(&data)
    }

    /// UpdateSubscriptionPlanAvailability updates a subscription plan availability.
    pub async fn update_subscription_plan_availability(&self, availability_id: &str, req: &SubscriptionPlanAvailabilityUpdateRequest) -> Result<SubscriptionPlanAvailabilityResponse> {
        let data = self.patch(&format!("/v1/subscriptionPlanAvailabilities/{}", url::path_escape(availability_id)), Some(req)).await?;
        decode(&data)
    }

    /// ListAppTagTerritories returns the territories where an app tag applies.
    pub async fn list_app_tag_territories(&self, app_tag_id: &str, opts: Option<&ListOptions>) -> Result<TerritoriesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/appTags/{}/territories", url::path_escape(app_tag_id)), Some(&query)).await?;
        decode(&data)
    }

    /// ListAppTags returns the Apple-created tags applied to an app.
    pub async fn list_app_tags(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<AppTagsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get(&format!("/v1/apps/{}/appTags", url::path_escape(app_id)), Some(&query)).await?;
        decode(&data)
    }

    /// UpdateAppTag updates an app tag's App Store visibility (opt-out).
    pub async fn update_app_tag(&self, app_tag_id: &str, req: &AppTagUpdateRequest) -> Result<AppTagResponse> {
        let data = self.patch(&format!("/v1/appTags/{}", url::path_escape(app_tag_id)), Some(req)).await?;
        decode(&data)
    }
}

// Ported from internal/asc/api/client_commerce_test.go.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::testutil::{MockApi, MockResponse};

    fn rel(t: &str, id: &str) -> RelationshipData {
        RelationshipData { data: ResourceIdentifier { type_: t.into(), id: id.into() } }
    }

    fn ri(t: &str, id: &str) -> ResourceIdentifier {
        ResourceIdentifier { type_: t.into(), id: id.into() }
    }

    /// Exercises every commerce read endpoint, asserting the client hits
    /// the path and method the App Store Connect OpenAPI spec documents.
    #[tokio::test]
    async fn commerce_read_paths() {
        macro_rules! read_case {
            ($name:expr, $path:expr, $list:expr, |$c:ident| $call:expr) => {{
                let body = if $list { r#"{"data":[],"links":{}}"# } else { r#"{"data":{"type":"x","id":"1"}}"# };
                let mock = MockApi::start(move |_| MockResponse::json(200, body)).await;
                let $c = mock.client();
                if let Err(e) = $call.await {
                    panic!("{}: unexpected error: {e}", $name);
                }
                let reqs = mock.requests();
                assert_eq!(reqs.len(), 1, "{}", $name);
                assert_eq!(reqs[0].path, $path, "{}: path", $name);
                assert_eq!(reqs[0].method, "GET", "{}: method", $name);
            }};
        }
        read_case!("get in-app purchase version", "/v1/inAppPurchaseVersions/ver-1", false, |c| c.get_in_app_purchase_version("ver-1"));
        read_case!("list in-app purchase versions", "/v2/inAppPurchases/iap-1/versions", true, |c| c.list_in_app_purchase_versions("iap-1", None));
        read_case!("list in-app purchase version localizations", "/v1/inAppPurchaseVersions/ver-1/localizations", true, |c| c.list_in_app_purchase_version_localizations("ver-1", None));
        read_case!("list in-app purchase version images", "/v1/inAppPurchaseVersions/ver-1/images", true, |c| c.list_in_app_purchase_version_images("ver-1", None));
        read_case!("get subscription version", "/v1/subscriptionVersions/ver-2", false, |c| c.get_subscription_version("ver-2"));
        read_case!("list subscription versions", "/v1/subscriptions/sub-1/versions", true, |c| c.list_subscription_versions("sub-1", None));
        read_case!("list subscription version localizations", "/v1/subscriptionVersions/ver-2/localizations", true, |c| c.list_subscription_version_localizations("ver-2", None));
        read_case!("list subscription version images", "/v1/subscriptionVersions/ver-2/images", true, |c| c.list_subscription_version_images("ver-2", None));
        read_case!("get subscription group version", "/v1/subscriptionGroupVersions/ver-3", false, |c| c.get_subscription_group_version("ver-3"));
        read_case!("list subscription group versions", "/v1/subscriptionGroups/group-1/versions", true, |c| c.list_subscription_group_versions("group-1", None));
        read_case!("list subscription group version localizations", "/v1/subscriptionGroupVersions/ver-3/localizations", true, |c| c.list_subscription_group_version_localizations("ver-3", None));
        read_case!("list in-app purchase offer codes", "/v2/inAppPurchases/iap-1/offerCodes", true, |c| c.list_in_app_purchase_offer_codes("iap-1", None));
        read_case!("get in-app purchase offer code", "/v1/inAppPurchaseOfferCodes/offer-1", false, |c| c.get_in_app_purchase_offer_code("offer-1"));
        read_case!("list in-app purchase offer code prices", "/v1/inAppPurchaseOfferCodes/offer-1/prices", true, |c| c.list_in_app_purchase_offer_code_prices("offer-1", None));
        read_case!("list in-app purchase offer code custom codes", "/v1/inAppPurchaseOfferCodes/offer-1/customCodes", true, |c| c.list_in_app_purchase_offer_code_custom_codes("offer-1", None));
        read_case!("list in-app purchase offer code one-time-use codes", "/v1/inAppPurchaseOfferCodes/offer-1/oneTimeUseCodes", true, |c| c.list_in_app_purchase_offer_code_one_time_use_codes("offer-1", None));
        read_case!("list in-app purchase price points", "/v2/inAppPurchases/iap-1/pricePoints", true, |c| c.list_in_app_purchase_price_points("iap-1", None));
        read_case!("get in-app purchase price schedule", "/v2/inAppPurchases/iap-1/iapPriceSchedule", false, |c| c.get_in_app_purchase_price_schedule("iap-1"));
        read_case!("list in-app purchase price schedule manual prices", "/v1/inAppPurchasePriceSchedules/sched-1/manualPrices", true, |c| c.list_in_app_purchase_price_schedule_prices("sched-1", false, None));
        read_case!("list in-app purchase price schedule automatic prices", "/v1/inAppPurchasePriceSchedules/sched-1/automaticPrices", true, |c| c.list_in_app_purchase_price_schedule_prices("sched-1", true, None));
        read_case!("get in-app purchase availability", "/v2/inAppPurchases/iap-1/inAppPurchaseAvailability", false, |c| c.get_in_app_purchase_availability("iap-1"));
        read_case!("list in-app purchase available territories", "/v1/inAppPurchaseAvailabilities/avail-1/availableTerritories", true, |c| c.list_in_app_purchase_available_territories("avail-1", None));
        read_case!("list subscription plan availabilities", "/v1/subscriptions/sub-1/planAvailabilities", true, |c| c.list_subscription_plan_availabilities("sub-1", None));
        read_case!("get subscription plan availability", "/v1/subscriptionPlanAvailabilities/plan-1", false, |c| c.get_subscription_plan_availability("plan-1"));
        read_case!("list subscription plan available territories", "/v1/subscriptionPlanAvailabilities/plan-1/availableTerritories", true, |c| c.list_subscription_plan_available_territories("plan-1", None));
        read_case!("get subscription price point", "/v1/subscriptionPricePoints/pp-1", false, |c| c.get_subscription_price_point("pp-1"));
        read_case!("list subscription price point equalizations", "/v1/subscriptionPricePoints/pp-1/equalizations", true, |c| c.list_subscription_price_point_equalizations("pp-1", None));
        read_case!("get app price point", "/v3/appPricePoints/pp-2", false, |c| c.get_app_price_point("pp-2"));
        read_case!("list app price point equalizations", "/v3/appPricePoints/pp-2/equalizations", true, |c| c.list_app_price_point_equalizations("pp-2", None));
    }

    #[tokio::test]
    async fn client_create_in_app_purchase_version() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                201,
                r#"{"data":{"type":"inAppPurchaseVersions","id":"ver-1","attributes":{"version":2,"state":"PREPARE_FOR_SUBMISSION"}}}"#,
            )
        })
        .await;
        let req = InAppPurchaseVersionCreateRequest {
            data: InAppPurchaseVersionCreateData {
                type_: "inAppPurchaseVersions".into(),
                relationships: InAppPurchaseVersionCreateRelationships { in_app_purchase: rel("inAppPurchases", "iap-1") },
            },
        };
        let resp = mock.client().create_in_app_purchase_version(&req).await.expect("unexpected error");

        let reqs = mock.requests();
        assert_eq!(reqs[0].path, "/v1/inAppPurchaseVersions");
        assert_eq!(reqs[0].method, "POST");
        let body = reqs[0].json();
        assert_eq!(body["data"]["type"], "inAppPurchaseVersions");
        assert_eq!(body["data"]["relationships"]["inAppPurchase"]["data"]["type"], "inAppPurchases");
        assert_eq!(body["data"]["relationships"]["inAppPurchase"]["data"]["id"], "iap-1");

        assert_eq!(resp.data.id, "ver-1");
        assert_eq!(resp.data.attributes.version, 2);
    }

    #[tokio::test]
    async fn client_create_subscription_version() {
        let mock =
            MockApi::start(|_| MockResponse::json(201, r#"{"data":{"type":"subscriptionVersions","id":"ver-2","attributes":{}}}"#)).await;
        let req = SubscriptionVersionCreateRequest {
            data: SubscriptionVersionCreateData {
                type_: "subscriptionVersions".into(),
                relationships: SubscriptionVersionCreateRelationships { subscription: rel("subscriptions", "sub-1") },
            },
        };
        let resp = mock.client().create_subscription_version(&req).await.expect("unexpected error");

        let reqs = mock.requests();
        assert_eq!(reqs[0].path, "/v1/subscriptionVersions");
        assert_eq!(reqs[0].json()["data"]["relationships"]["subscription"]["data"]["id"], "sub-1");
        assert_eq!(resp.data.id, "ver-2");
    }

    #[tokio::test]
    async fn client_create_subscription_group_version() {
        let mock = MockApi::start(|_| {
            MockResponse::json(201, r#"{"data":{"type":"subscriptionGroupVersions","id":"ver-3","attributes":{}}}"#)
        })
        .await;
        let req = SubscriptionGroupVersionCreateRequest {
            data: SubscriptionGroupVersionCreateData {
                type_: "subscriptionGroupVersions".into(),
                relationships: SubscriptionGroupVersionCreateRelationships { subscription_group: rel("subscriptionGroups", "group-1") },
            },
        };
        let resp = mock.client().create_subscription_group_version(&req).await.expect("unexpected error");

        let reqs = mock.requests();
        assert_eq!(reqs[0].path, "/v1/subscriptionGroupVersions");
        assert_eq!(reqs[0].json()["data"]["relationships"]["subscriptionGroup"]["data"]["type"], "subscriptionGroups");
        assert_eq!(resp.data.id, "ver-3");
    }

    #[tokio::test]
    async fn client_create_in_app_purchase_localization() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                201,
                r#"{"data":{"type":"inAppPurchaseLocalizations","id":"loc-1","attributes":{"name":"Gold Coins","locale":"en-US"}}}"#,
            )
        })
        .await;
        let req = InAppPurchaseLocalizationCreateRequest {
            data: InAppPurchaseLocalizationCreateData {
                type_: "inAppPurchaseLocalizations".into(),
                attributes: InAppPurchaseLocalizationCreateAttributes {
                    name: "Gold Coins".into(),
                    locale: "en-US".into(),
                    ..Default::default()
                },
                relationships: InAppPurchaseLocalizationCreateRelationships { version: rel("inAppPurchaseVersions", "ver-1") },
            },
        };
        let resp = mock.client().create_in_app_purchase_localization(&req).await.expect("unexpected error");

        let reqs = mock.requests();
        assert_eq!(reqs[0].path, "/v2/inAppPurchaseLocalizations");
        let body = reqs[0].json();
        assert_eq!(body["data"]["attributes"]["locale"], "en-US");
        // The v2 collection points at a version, not at the product.
        assert_eq!(body["data"]["relationships"]["version"]["data"]["type"], "inAppPurchaseVersions");
        assert_eq!(resp.data.attributes.name, "Gold Coins");
    }

    #[tokio::test]
    async fn client_update_subscription_image() {
        let mock =
            MockApi::start(|_| MockResponse::json(200, r#"{"data":{"type":"subscriptionImages","id":"img-1","attributes":{}}}"#)).await;
        let req = SubscriptionImageUpdateRequest {
            data: SubscriptionImageUpdateData {
                type_: "subscriptionImages".into(),
                id: "img-1".into(),
                attributes: CommerceImageUpdateAttributes { uploaded: Some(true) },
            },
        };
        mock.client().update_subscription_image("img-1", &req).await.expect("unexpected error");

        let reqs = mock.requests();
        assert_eq!(reqs[0].path, "/v2/subscriptionImages/img-1");
        assert_eq!(reqs[0].method, "PATCH");
        assert_eq!(reqs[0].json()["data"]["attributes"]["uploaded"], true, "expected uploaded=true in the request body");
    }

    #[tokio::test]
    async fn client_create_in_app_purchase_offer_code() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                201,
                r#"{"data":{"type":"inAppPurchaseOfferCodes","id":"offer-1","attributes":{"name":"Launch","active":true}}}"#,
            )
        })
        .await;
        let req = InAppPurchaseOfferCodeCreateRequest {
            data: InAppPurchaseOfferCodeCreateData {
                type_: "inAppPurchaseOfferCodes".into(),
                attributes: InAppPurchaseOfferCodeCreateAttributes {
                    name: "Launch".into(),
                    customer_eligibilities: GoSlice::from(vec!["NEW".to_string()]),
                },
                relationships: InAppPurchaseOfferCodeCreateRelationships {
                    in_app_purchase: rel("inAppPurchases", "iap-1"),
                    prices: RelationshipDataList { data: GoSlice::from(vec![ri("inAppPurchaseOfferPrices", "price-1")]) },
                },
            },
            included: GoSlice::from(vec![InAppPurchaseOfferPriceInlineCreate {
                type_: "inAppPurchaseOfferPrices".into(),
                id: "price-1".into(),
                relationships: Some(InAppPurchaseOfferPriceInlineCreateRelationships {
                    territory: rel("territories", "USA"),
                    price_point: rel("inAppPurchasePricePoints", "pp-1"),
                }),
            }]),
        };
        let resp = mock.client().create_in_app_purchase_offer_code(&req).await.expect("unexpected error");

        let reqs = mock.requests();
        assert_eq!(reqs[0].path, "/v1/inAppPurchaseOfferCodes");
        let body = reqs[0].json();
        assert_eq!(body["data"]["attributes"]["customerEligibilities"].as_array().unwrap().len(), 1);
        let prices = body["data"]["relationships"]["prices"]["data"].as_array().unwrap();
        assert_eq!(prices.len(), 1, "expected 1 price linkage");
        let included = body["included"].as_array().unwrap();
        assert_eq!(included.len(), 1, "expected 1 included price");
        // The linkage and the inline resource must agree on the ID.
        assert_eq!(prices[0]["id"], included[0]["id"]);
        assert_eq!(included[0]["relationships"]["pricePoint"]["data"]["type"], "inAppPurchasePricePoints");
        assert_eq!(resp.data.id, "offer-1");
    }

    #[tokio::test]
    async fn client_create_in_app_purchase_offer_code_one_time_use_code() {
        let mock = MockApi::start(|_| {
            MockResponse::json(201, r#"{"data":{"type":"inAppPurchaseOfferCodeOneTimeUseCodes","id":"batch-1","attributes":{}}}"#)
        })
        .await;
        let req = InAppPurchaseOfferCodeOneTimeUseCodeCreateRequest {
            data: InAppPurchaseOfferCodeOneTimeUseCodeCreateData {
                type_: "inAppPurchaseOfferCodeOneTimeUseCodes".into(),
                attributes: InAppPurchaseOfferCodeOneTimeUseCodeCreateAttributes {
                    number_of_codes: 500,
                    expiration_date: "2026-12-31".into(),
                    ..Default::default()
                },
                relationships: OfferCodeRelationship { offer_code: rel("inAppPurchaseOfferCodes", "offer-1") },
            },
        };
        mock.client().create_in_app_purchase_offer_code_one_time_use_code(&req).await.expect("unexpected error");

        let reqs = mock.requests();
        assert_eq!(reqs[0].path, "/v1/inAppPurchaseOfferCodeOneTimeUseCodes");
        let body = reqs[0].json();
        assert_eq!(body["data"]["attributes"]["numberOfCodes"], 500);
        assert_eq!(body["data"]["attributes"]["expirationDate"], "2026-12-31");
        assert_eq!(body["data"]["relationships"]["offerCode"]["data"]["id"], "offer-1");
    }

    #[tokio::test]
    async fn client_get_in_app_purchase_offer_code_one_time_use_code_values() {
        const CSV: &str = "code\nABC123\nDEF456\n";
        let mock = MockApi::start(|_| MockResponse {
            status: 200,
            content_type: "text/csv".into(),
            body: CSV.as_bytes().to_vec(),
            headers: vec![],
        })
        .await;
        let got = mock.client().get_in_app_purchase_offer_code_one_time_use_code_values("batch-1").await.expect("unexpected error");
        assert_eq!(mock.requests()[0].path, "/v1/inAppPurchaseOfferCodeOneTimeUseCodes/batch-1/values");
        assert_eq!(got, CSV);
    }

    #[tokio::test]
    async fn client_create_in_app_purchase_price_schedule() {
        let mock =
            MockApi::start(|_| MockResponse::json(201, r#"{"data":{"type":"inAppPurchasePriceSchedules","id":"sched-1"}}"#)).await;
        let req = InAppPurchasePriceScheduleCreateRequest {
            data: InAppPurchasePriceScheduleCreateData {
                type_: "inAppPurchasePriceSchedules".into(),
                relationships: InAppPurchasePriceScheduleCreateRelationships {
                    in_app_purchase: rel("inAppPurchases", "iap-1"),
                    base_territory: rel("territories", "USA"),
                    manual_prices: RelationshipDataList { data: GoSlice::from(vec![ri("inAppPurchasePrices", "price-1")]) },
                },
            },
            included: GoSlice::from(vec![InAppPurchasePriceInlineCreate {
                type_: "inAppPurchasePrices".into(),
                id: "price-1".into(),
                attributes: None,
                relationships: Some(InAppPurchasePriceInlineCreateRelationships {
                    in_app_purchase_price_point: rel("inAppPurchasePricePoints", "pp-1"),
                }),
            }]),
        };
        let resp = mock.client().create_in_app_purchase_price_schedule(&req).await.expect("unexpected error");

        let reqs = mock.requests();
        assert_eq!(reqs[0].path, "/v1/inAppPurchasePriceSchedules");
        let body = reqs[0].json();
        assert_eq!(body["data"]["relationships"]["baseTerritory"]["data"]["id"], "USA");
        let included = body["included"].as_array().unwrap();
        assert_eq!(included.len(), 1, "expected 1 inline price");
        assert_eq!(included[0]["relationships"]["inAppPurchasePricePoint"]["data"]["id"], "pp-1");
        assert_eq!(resp.data.id, "sched-1");
    }

    #[tokio::test]
    async fn client_create_app_price_schedule() {
        let mock = MockApi::start(|_| MockResponse::json(201, r#"{"data":{"type":"appPriceSchedules","id":"app-sched-1"}}"#)).await;
        let req = AppPriceScheduleCreateRequest {
            data: AppPriceScheduleCreateData {
                type_: "appPriceSchedules".into(),
                relationships: AppPriceScheduleCreateRelationships {
                    app: rel("apps", "app-1"),
                    base_territory: rel("territories", "USA"),
                    manual_prices: RelationshipDataList { data: GoSlice::from(vec![ri("appPrices", "price-1")]) },
                },
            },
            included: GoSlice::from(vec![AppPriceInlineCreate {
                type_: "appPrices".into(),
                id: "price-1".into(),
                attributes: Some(CommercePriceInlineAttributes { start_date: "2026-01-01".into(), ..Default::default() }),
                relationships: Some(AppPriceInlineCreateRelationships { app_price_point: rel("appPricePoints", "pp-2") }),
            }]),
        };
        let resp = mock.client().create_app_price_schedule(&req).await.expect("unexpected error");

        let reqs = mock.requests();
        assert_eq!(reqs[0].path, "/v1/appPriceSchedules");
        assert_eq!(reqs[0].method, "POST");
        let body = reqs[0].json();
        assert_eq!(body["data"]["relationships"]["app"]["data"]["id"], "app-1");
        assert_eq!(body["data"]["relationships"]["baseTerritory"]["data"]["type"], "territories");
        let included = body["included"].as_array().unwrap();
        assert_eq!(included.len(), 1, "expected 1 inline price");
        assert_eq!(included[0]["attributes"]["startDate"], "2026-01-01", "expected inline price to carry startDate 2026-01-01");
        assert_eq!(resp.data.id, "app-sched-1");
    }

    #[tokio::test]
    async fn client_create_in_app_purchase_availability() {
        let mock = MockApi::start(|_| {
            MockResponse::json(201, r#"{"data":{"type":"inAppPurchaseAvailabilities","id":"avail-1","attributes":{"availableInNewTerritories":false}}}"#)
        })
        .await;
        let req = InAppPurchaseAvailabilityCreateRequest {
            data: InAppPurchaseAvailabilityCreateData {
                type_: "inAppPurchaseAvailabilities".into(),
                attributes: InAppPurchaseAvailabilityAttributes { available_in_new_territories: true },
                relationships: InAppPurchaseAvailabilityCreateRelationships {
                    in_app_purchase: rel("inAppPurchases", "iap-1"),
                    available_territories: RelationshipDataList {
                        data: GoSlice::from(vec![ri("territories", "USA"), ri("territories", "CAN")]),
                    },
                },
            },
        };
        mock.client().create_in_app_purchase_availability(&req).await.expect("unexpected error");

        let reqs = mock.requests();
        assert_eq!(reqs[0].path, "/v1/inAppPurchaseAvailabilities");
        let body = reqs[0].json();
        assert_eq!(body["data"]["attributes"]["availableInNewTerritories"], true, "expected availableInNewTerritories=true");
        assert_eq!(body["data"]["relationships"]["availableTerritories"]["data"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn client_create_subscription_plan_availability() {
        let mock = MockApi::start(|_| {
            MockResponse::json(201, r#"{"data":{"type":"subscriptionPlanAvailabilities","id":"plan-1","attributes":{"planType":"UPFRONT"}}}"#)
        })
        .await;
        let req = SubscriptionPlanAvailabilityCreateRequest {
            data: SubscriptionPlanAvailabilityCreateData {
                type_: "subscriptionPlanAvailabilities".into(),
                attributes: SubscriptionPlanAvailabilityAttributes { plan_type: "UPFRONT".into(), available_in_new_territories: Some(true) },
                relationships: SubscriptionPlanAvailabilityCreateRelationships {
                    subscription: rel("subscriptions", "sub-1"),
                    available_territories: RelationshipDataList { data: GoSlice::from(vec![ri("territories", "USA")]) },
                },
            },
        };
        let resp = mock.client().create_subscription_plan_availability(&req).await.expect("unexpected error");

        let reqs = mock.requests();
        assert_eq!(reqs[0].path, "/v1/subscriptionPlanAvailabilities");
        let body = reqs[0].json();
        assert_eq!(body["data"]["attributes"]["planType"], "UPFRONT");
        assert_eq!(body["data"]["relationships"]["subscription"]["data"]["id"], "sub-1");
        assert_eq!(resp.data.attributes.plan_type, "UPFRONT");
    }

    #[tokio::test]
    async fn client_update_subscription_plan_availability() {
        let mock = MockApi::start(|_| {
            MockResponse::json(200, r#"{"data":{"type":"subscriptionPlanAvailabilities","id":"plan-1","attributes":{}}}"#)
        })
        .await;
        let req = SubscriptionPlanAvailabilityUpdateRequest {
            data: SubscriptionPlanAvailabilityUpdateData {
                type_: "subscriptionPlanAvailabilities".into(),
                id: "plan-1".into(),
                attributes: None,
                relationships: Some(SubscriptionPlanAvailabilityUpdateRelationships {
                    available_territories: Some(RelationshipDataList { data: GoSlice::from(vec![ri("territories", "JPN")]) }),
                }),
            },
        };
        mock.client().update_subscription_plan_availability("plan-1", &req).await.expect("unexpected error");

        let reqs = mock.requests();
        assert_eq!(reqs[0].path, "/v1/subscriptionPlanAvailabilities/plan-1");
        assert_eq!(reqs[0].method, "PATCH");
        let body = reqs[0].json();
        let terr = &body["data"]["relationships"]["availableTerritories"];
        assert!(terr.is_object(), "expected availableTerritories relationship in the request body");
        assert_eq!(terr["data"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn client_list_subscription_price_point_adjusted_equalizations() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                200,
                r#"{"data":[{"type":"subscriptionPricePoints","id":"pp-2","attributes":{"customerPrice":"9.99"}}],"links":{"self":""}}"#,
            )
        })
        .await;
        let mut filter = GoMap::nil();
        filter.insert("upfrontPricePointId", GoSlice::from(vec!["pp-upfront".to_string()]));
        filter.insert("planType", GoSlice::from(vec!["UPFRONT".to_string()]));
        let opts = ListOptions { filter, ..Default::default() };
        let resp = mock.client().list_subscription_price_point_adjusted_equalizations("pp-1", Some(&opts)).await.expect("unexpected error");

        let reqs = mock.requests();
        assert_eq!(reqs[0].path, "/v1/subscriptionPricePoints/pp-1/adjustedEqualizations");
        assert_eq!(reqs[0].query_param("filter[upfrontPricePointId]").as_deref(), Some("pp-upfront"));
        assert_eq!(reqs[0].query_param("filter[planType]").as_deref(), Some("UPFRONT"));
        assert_eq!(resp.data.len(), 1, "expected 1 price point");
        assert_eq!(resp.data[0].attributes.customer_price, "9.99");
    }

    /// The version-scoped v2 delete endpoints target the v2 tree, not the
    /// v1 one.
    #[tokio::test]
    async fn commerce_delete_paths() {
        macro_rules! delete_case {
            ($name:expr, $path:expr, |$c:ident| $call:expr) => {{
                let mock = MockApi::start(|_| MockResponse::empty(204)).await;
                let $c = mock.client();
                if let Err(e) = $call.await {
                    panic!("{}: unexpected error: {e}", $name);
                }
                let reqs = mock.requests();
                assert_eq!(reqs[0].path, $path, "{}: path", $name);
                assert_eq!(reqs[0].method, "DELETE", "{}: method", $name);
            }};
        }
        delete_case!("delete in-app purchase localization", "/v2/inAppPurchaseLocalizations/loc-1", |c| c
            .delete_in_app_purchase_localization("loc-1"));
        delete_case!("delete in-app purchase image", "/v2/inAppPurchaseImages/img-1", |c| c.delete_in_app_purchase_image("img-1"));
        delete_case!("delete subscription localization", "/v2/subscriptionLocalizations/loc-2", |c| c
            .delete_subscription_localization("loc-2"));
        delete_case!("delete subscription image", "/v2/subscriptionImages/img-2", |c| c.delete_subscription_image("img-2"));
        delete_case!("delete subscription group localization", "/v2/subscriptionGroupLocalizations/loc-3", |c| c
            .delete_subscription_group_localization("loc-3"));
    }

    /// The commerce version relationships added in 4.4.1 serialize under the
    /// member names the review submission item endpoint expects.
    #[test]
    fn review_submission_item_commerce_relationships() {
        let req = ReviewSubmissionItemCreateRequest {
            data: ReviewSubmissionItemCreateData {
                type_: "reviewSubmissionItems".into(),
                relationships: ReviewSubmissionItemCreateRelationships {
                    review_submission: rel("reviewSubmissions", "sub-1"),
                    in_app_purchase_version: Some(rel("inAppPurchaseVersions", "ver-1")),
                    subscription_version: Some(rel("subscriptionVersions", "ver-2")),
                    subscription_group_version: Some(rel("subscriptionGroupVersions", "ver-3")),
                    ..Default::default()
                },
            },
        };
        let body = json::to_value(&req);
        let rels = &body["data"]["relationships"];
        for (member, resource_type) in [
            ("inAppPurchaseVersion", "inAppPurchaseVersions"),
            ("subscriptionVersion", "subscriptionVersions"),
            ("subscriptionGroupVersion", "subscriptionGroupVersions"),
        ] {
            assert!(rels.get(member).is_some(), "missing relationship member {member:?}");
            assert_eq!(rels[member]["data"]["type"], resource_type, "{member} type");
        }
    }
}
