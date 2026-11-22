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
        todo!("port Client.GetAppPriceSchedule from internal/asc/api/client.go")
    }

    /// ListAppPricePoints returns price points for an app.
    pub async fn list_app_price_points(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<AppPricePointsResponse> {
        todo!("port Client.ListAppPricePoints from internal/asc/api/client.go")
    }

    /// ListSubscriptionPricePoints returns price points for a subscription.
    pub async fn list_subscription_price_points(&self, subscription_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionPricePointsResponse> {
        todo!("port Client.ListSubscriptionPricePoints from internal/asc/api/client.go")
    }

    /// ListTerritories returns all territories.
    pub async fn list_territories(&self, opts: Option<&ListOptions>) -> Result<TerritoriesResponse> {
        todo!("port Client.ListTerritories from internal/asc/api/client.go")
    }

    /// CreateAppPriceSchedule replaces an app's price schedule.
    pub async fn create_app_price_schedule(&self, req: &AppPriceScheduleCreateRequest) -> Result<AppPriceScheduleResponse> {
        todo!("port Client.CreateAppPriceSchedule from internal/asc/api/client_commerce.go")
    }

    /// CreateInAppPurchaseAvailability sets the territory availability of an
    /// in-app purchase.
    pub async fn create_in_app_purchase_availability(&self, req: &InAppPurchaseAvailabilityCreateRequest) -> Result<InAppPurchaseAvailabilityResponse> {
        todo!("port Client.CreateInAppPurchaseAvailability from internal/asc/api/client_commerce.go")
    }

    /// CreateInAppPurchaseImage reserves an image upload on an in-app
    /// purchase version. The response carries the upload operations to
    /// perform before committing with UpdateInAppPurchaseImage.
    pub async fn create_in_app_purchase_image(&self, req: &InAppPurchaseImageCreateRequest) -> Result<InAppPurchaseImageResponse> {
        todo!("port Client.CreateInAppPurchaseImage from internal/asc/api/client_commerce.go")
    }

    /// CreateInAppPurchaseLocalization creates a localization on an in-app
    /// purchase version.
    pub async fn create_in_app_purchase_localization(&self, req: &InAppPurchaseLocalizationCreateRequest) -> Result<InAppPurchaseLocalizationResponse> {
        todo!("port Client.CreateInAppPurchaseLocalization from internal/asc/api/client_commerce.go")
    }

    /// CreateInAppPurchaseOfferCode creates an in-app purchase offer code.
    pub async fn create_in_app_purchase_offer_code(&self, req: &InAppPurchaseOfferCodeCreateRequest) -> Result<InAppPurchaseOfferCodeResponse> {
        todo!("port Client.CreateInAppPurchaseOfferCode from internal/asc/api/client_commerce.go")
    }

    /// CreateInAppPurchaseOfferCodeCustomCode creates a custom code batch.
    pub async fn create_in_app_purchase_offer_code_custom_code(&self, req: &InAppPurchaseOfferCodeCustomCodeCreateRequest) -> Result<InAppPurchaseOfferCodeCustomCodeResponse> {
        todo!("port Client.CreateInAppPurchaseOfferCodeCustomCode from internal/asc/api/client_commerce.go")
    }

    /// CreateInAppPurchaseOfferCodeOneTimeUseCode creates a one-time-use code batch.
    pub async fn create_in_app_purchase_offer_code_one_time_use_code(&self, req: &InAppPurchaseOfferCodeOneTimeUseCodeCreateRequest) -> Result<InAppPurchaseOfferCodeOneTimeUseCodeResponse> {
        todo!("port Client.CreateInAppPurchaseOfferCodeOneTimeUseCode from internal/asc/api/client_commerce.go")
    }

    /// CreateInAppPurchasePriceSchedule replaces an in-app purchase's price schedule.
    pub async fn create_in_app_purchase_price_schedule(&self, req: &InAppPurchasePriceScheduleCreateRequest) -> Result<InAppPurchasePriceScheduleResponse> {
        todo!("port Client.CreateInAppPurchasePriceSchedule from internal/asc/api/client_commerce.go")
    }

    /// CreateInAppPurchaseVersion creates a new version of an in-app
    /// purchase, snapshotting its current metadata for editing.
    pub async fn create_in_app_purchase_version(&self, req: &InAppPurchaseVersionCreateRequest) -> Result<InAppPurchaseVersionResponse> {
        todo!("port Client.CreateInAppPurchaseVersion from internal/asc/api/client_commerce.go")
    }

    /// CreateSubscriptionGroupLocalization creates a localization on a
    /// subscription group version.
    pub async fn create_subscription_group_localization(&self, req: &SubscriptionGroupLocalizationCreateRequest) -> Result<SubscriptionGroupLocalizationResponse> {
        todo!("port Client.CreateSubscriptionGroupLocalization from internal/asc/api/client_commerce.go")
    }

    /// CreateSubscriptionGroupVersion creates a new version of a subscription group.
    pub async fn create_subscription_group_version(&self, req: &SubscriptionGroupVersionCreateRequest) -> Result<SubscriptionGroupVersionResponse> {
        todo!("port Client.CreateSubscriptionGroupVersion from internal/asc/api/client_commerce.go")
    }

    /// CreateSubscriptionImage reserves an image upload on a subscription version.
    pub async fn create_subscription_image(&self, req: &SubscriptionImageCreateRequest) -> Result<SubscriptionImageResponse> {
        todo!("port Client.CreateSubscriptionImage from internal/asc/api/client_commerce.go")
    }

    /// CreateSubscriptionLocalization creates a localization on a subscription version.
    pub async fn create_subscription_localization(&self, req: &SubscriptionLocalizationCreateRequest) -> Result<SubscriptionLocalizationResponse> {
        todo!("port Client.CreateSubscriptionLocalization from internal/asc/api/client_commerce.go")
    }

    /// CreateSubscriptionPlanAvailability configures the territories one
    /// billing plan of a subscription is available in.
    pub async fn create_subscription_plan_availability(&self, req: &SubscriptionPlanAvailabilityCreateRequest) -> Result<SubscriptionPlanAvailabilityResponse> {
        todo!("port Client.CreateSubscriptionPlanAvailability from internal/asc/api/client_commerce.go")
    }

    /// CreateSubscriptionVersion creates a new version of a subscription.
    pub async fn create_subscription_version(&self, req: &SubscriptionVersionCreateRequest) -> Result<SubscriptionVersionResponse> {
        todo!("port Client.CreateSubscriptionVersion from internal/asc/api/client_commerce.go")
    }

    /// DeleteInAppPurchaseImage deletes an in-app purchase image.
    pub async fn delete_in_app_purchase_image(&self, image_id: &str) -> Result<()> {
        todo!("port Client.DeleteInAppPurchaseImage from internal/asc/api/client_commerce.go")
    }

    /// DeleteInAppPurchaseLocalization deletes an in-app purchase localization.
    pub async fn delete_in_app_purchase_localization(&self, localization_id: &str) -> Result<()> {
        todo!("port Client.DeleteInAppPurchaseLocalization from internal/asc/api/client_commerce.go")
    }

    /// DeleteSubscriptionGroupLocalization deletes a subscription group localization.
    pub async fn delete_subscription_group_localization(&self, localization_id: &str) -> Result<()> {
        todo!("port Client.DeleteSubscriptionGroupLocalization from internal/asc/api/client_commerce.go")
    }

    /// DeleteSubscriptionImage deletes a subscription image.
    pub async fn delete_subscription_image(&self, image_id: &str) -> Result<()> {
        todo!("port Client.DeleteSubscriptionImage from internal/asc/api/client_commerce.go")
    }

    /// DeleteSubscriptionLocalization deletes a subscription localization.
    pub async fn delete_subscription_localization(&self, localization_id: &str) -> Result<()> {
        todo!("port Client.DeleteSubscriptionLocalization from internal/asc/api/client_commerce.go")
    }

    /// GetAppPricePoint returns a single app price point.
    pub async fn get_app_price_point(&self, price_point_id: &str) -> Result<AppPricePointResponse> {
        todo!("port Client.GetAppPricePoint from internal/asc/api/client_commerce.go")
    }

    /// GetInAppPurchaseAvailability returns the territory availability of an
    /// in-app purchase.
    pub async fn get_in_app_purchase_availability(&self, iap_id: &str) -> Result<InAppPurchaseAvailabilityResponse> {
        todo!("port Client.GetInAppPurchaseAvailability from internal/asc/api/client_commerce.go")
    }

    /// GetInAppPurchaseOfferCode returns a single in-app purchase offer code.
    pub async fn get_in_app_purchase_offer_code(&self, offer_code_id: &str) -> Result<InAppPurchaseOfferCodeResponse> {
        todo!("port Client.GetInAppPurchaseOfferCode from internal/asc/api/client_commerce.go")
    }

    /// GetInAppPurchaseOfferCodeOneTimeUseCodeValues downloads the generated
    /// codes of a one-time-use batch. Apple returns them as CSV, not JSON, so
    /// the raw body is handed back to the caller.
    pub async fn get_in_app_purchase_offer_code_one_time_use_code_values(&self, code_id: &str) -> Result<String> {
        todo!("port Client.GetInAppPurchaseOfferCodeOneTimeUseCodeValues from internal/asc/api/client_commerce.go")
    }

    /// GetInAppPurchasePriceSchedule returns the price schedule of an in-app purchase.
    pub async fn get_in_app_purchase_price_schedule(&self, iap_id: &str) -> Result<InAppPurchasePriceScheduleResponse> {
        todo!("port Client.GetInAppPurchasePriceSchedule from internal/asc/api/client_commerce.go")
    }

    /// GetInAppPurchaseVersion returns a single in-app purchase version.
    pub async fn get_in_app_purchase_version(&self, version_id: &str) -> Result<InAppPurchaseVersionResponse> {
        todo!("port Client.GetInAppPurchaseVersion from internal/asc/api/client_commerce.go")
    }

    /// GetSubscriptionGroupVersion returns a single subscription group version.
    pub async fn get_subscription_group_version(&self, version_id: &str) -> Result<SubscriptionGroupVersionResponse> {
        todo!("port Client.GetSubscriptionGroupVersion from internal/asc/api/client_commerce.go")
    }

    /// GetSubscriptionPlanAvailability returns a single subscription plan availability.
    pub async fn get_subscription_plan_availability(&self, availability_id: &str) -> Result<SubscriptionPlanAvailabilityResponse> {
        todo!("port Client.GetSubscriptionPlanAvailability from internal/asc/api/client_commerce.go")
    }

    /// GetSubscriptionPricePoint returns a single subscription price point.
    pub async fn get_subscription_price_point(&self, price_point_id: &str) -> Result<SubscriptionPricePointResponse> {
        todo!("port Client.GetSubscriptionPricePoint from internal/asc/api/client_commerce.go")
    }

    /// GetSubscriptionVersion returns a single subscription version.
    pub async fn get_subscription_version(&self, version_id: &str) -> Result<SubscriptionVersionResponse> {
        todo!("port Client.GetSubscriptionVersion from internal/asc/api/client_commerce.go")
    }

    /// ListAppPricePointEqualizations returns the app price points in other
    /// territories that are equivalent to the given one.
    pub async fn list_app_price_point_equalizations(&self, price_point_id: &str, opts: Option<&ListOptions>) -> Result<AppPricePointsResponse> {
        todo!("port Client.ListAppPricePointEqualizations from internal/asc/api/client_commerce.go")
    }

    /// ListInAppPurchaseAvailableTerritories returns the territories covered
    /// by an in-app purchase availability.
    pub async fn list_in_app_purchase_available_territories(&self, availability_id: &str, opts: Option<&ListOptions>) -> Result<TerritoriesResponse> {
        todo!("port Client.ListInAppPurchaseAvailableTerritories from internal/asc/api/client_commerce.go")
    }

    /// ListInAppPurchaseOfferCodeCustomCodes returns the custom code batches
    /// of an in-app purchase offer code.
    pub async fn list_in_app_purchase_offer_code_custom_codes(&self, offer_code_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchaseOfferCodeCustomCodesResponse> {
        todo!("port Client.ListInAppPurchaseOfferCodeCustomCodes from internal/asc/api/client_commerce.go")
    }

    /// ListInAppPurchaseOfferCodeOneTimeUseCodes returns the one-time-use code
    /// batches of an in-app purchase offer code.
    pub async fn list_in_app_purchase_offer_code_one_time_use_codes(&self, offer_code_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchaseOfferCodeOneTimeUseCodesResponse> {
        todo!("port Client.ListInAppPurchaseOfferCodeOneTimeUseCodes from internal/asc/api/client_commerce.go")
    }

    /// ListInAppPurchaseOfferCodePrices returns the per-territory prices of an
    /// in-app purchase offer code.
    pub async fn list_in_app_purchase_offer_code_prices(&self, offer_code_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchaseOfferPricesResponse> {
        todo!("port Client.ListInAppPurchaseOfferCodePrices from internal/asc/api/client_commerce.go")
    }

    /// ListInAppPurchaseOfferCodes returns the offer codes of an in-app purchase.
    pub async fn list_in_app_purchase_offer_codes(&self, iap_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchaseOfferCodesResponse> {
        todo!("port Client.ListInAppPurchaseOfferCodes from internal/asc/api/client_commerce.go")
    }

    /// ListInAppPurchasePricePoints returns the price points available to an
    /// in-app purchase.
    pub async fn list_in_app_purchase_price_points(&self, iap_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchasePricePointsResponse> {
        todo!("port Client.ListInAppPurchasePricePoints from internal/asc/api/client_commerce.go")
    }

    /// ListInAppPurchasePriceSchedulePrices returns the prices of an in-app
    /// purchase price schedule. Manual prices are the ones the developer set;
    /// automatic prices are the ones Apple derived for other territories.
    pub async fn list_in_app_purchase_price_schedule_prices(&self, schedule_id: &str, automatic: bool, opts: Option<&ListOptions>) -> Result<InAppPurchasePricesResponse> {
        todo!("port Client.ListInAppPurchasePriceSchedulePrices from internal/asc/api/client_commerce.go")
    }

    /// ListInAppPurchaseVersionImages returns the images of an in-app purchase version.
    pub async fn list_in_app_purchase_version_images(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchaseImagesResponse> {
        todo!("port Client.ListInAppPurchaseVersionImages from internal/asc/api/client_commerce.go")
    }

    /// ListInAppPurchaseVersionLocalizations returns the localizations of an
    /// in-app purchase version.
    pub async fn list_in_app_purchase_version_localizations(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchaseLocalizationsResponse> {
        todo!("port Client.ListInAppPurchaseVersionLocalizations from internal/asc/api/client_commerce.go")
    }

    /// ListInAppPurchaseVersions returns the versions of an in-app purchase.
    pub async fn list_in_app_purchase_versions(&self, iap_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchaseVersionsResponse> {
        todo!("port Client.ListInAppPurchaseVersions from internal/asc/api/client_commerce.go")
    }

    /// ListSubscriptionGroupVersionLocalizations returns the localizations of
    /// a subscription group version.
    pub async fn list_subscription_group_version_localizations(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionGroupLocalizationsResponse> {
        todo!("port Client.ListSubscriptionGroupVersionLocalizations from internal/asc/api/client_commerce.go")
    }

    /// ListSubscriptionGroupVersions returns the versions of a subscription group.
    pub async fn list_subscription_group_versions(&self, group_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionGroupVersionsResponse> {
        todo!("port Client.ListSubscriptionGroupVersions from internal/asc/api/client_commerce.go")
    }

    /// ListSubscriptionPlanAvailabilities returns the plan availabilities of a
    /// subscription, one per billing plan.
    pub async fn list_subscription_plan_availabilities(&self, subscription_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionPlanAvailabilitiesResponse> {
        todo!("port Client.ListSubscriptionPlanAvailabilities from internal/asc/api/client_commerce.go")
    }

    /// ListSubscriptionPlanAvailableTerritories returns the territories
    /// covered by a subscription plan availability.
    pub async fn list_subscription_plan_available_territories(&self, availability_id: &str, opts: Option<&ListOptions>) -> Result<TerritoriesResponse> {
        todo!("port Client.ListSubscriptionPlanAvailableTerritories from internal/asc/api/client_commerce.go")
    }

    /// ListSubscriptionPricePointAdjustedEqualizations returns equalized price
    /// points adjusted for a pre-paid (upfront) plan, so a monthly price point
    /// can be matched against the upfront plan's price point
    /// (App Store Connect API 4.4.1). Pass filter[upfrontPricePointId] and
    /// filter[planType] through opts.
    pub async fn list_subscription_price_point_adjusted_equalizations(&self, price_point_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionPricePointsResponse> {
        todo!("port Client.ListSubscriptionPricePointAdjustedEqualizations from internal/asc/api/client_commerce.go")
    }

    /// ListSubscriptionPricePointEqualizations returns the price points in
    /// other territories that are equivalent to the given one.
    pub async fn list_subscription_price_point_equalizations(&self, price_point_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionPricePointsResponse> {
        todo!("port Client.ListSubscriptionPricePointEqualizations from internal/asc/api/client_commerce.go")
    }

    /// ListSubscriptionVersionImages returns the images of a subscription version.
    pub async fn list_subscription_version_images(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionImagesResponse> {
        todo!("port Client.ListSubscriptionVersionImages from internal/asc/api/client_commerce.go")
    }

    /// ListSubscriptionVersionLocalizations returns the localizations of a
    /// subscription version.
    pub async fn list_subscription_version_localizations(&self, version_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionLocalizationsResponse> {
        todo!("port Client.ListSubscriptionVersionLocalizations from internal/asc/api/client_commerce.go")
    }

    /// ListSubscriptionVersions returns the versions of a subscription.
    pub async fn list_subscription_versions(&self, subscription_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionVersionsResponse> {
        todo!("port Client.ListSubscriptionVersions from internal/asc/api/client_commerce.go")
    }

    /// UpdateInAppPurchaseImage commits an in-app purchase image upload.
    pub async fn update_in_app_purchase_image(&self, image_id: &str, req: &InAppPurchaseImageUpdateRequest) -> Result<InAppPurchaseImageResponse> {
        todo!("port Client.UpdateInAppPurchaseImage from internal/asc/api/client_commerce.go")
    }

    /// UpdateInAppPurchaseLocalization updates an in-app purchase localization.
    pub async fn update_in_app_purchase_localization(&self, localization_id: &str, req: &InAppPurchaseLocalizationUpdateRequest) -> Result<InAppPurchaseLocalizationResponse> {
        todo!("port Client.UpdateInAppPurchaseLocalization from internal/asc/api/client_commerce.go")
    }

    /// UpdateInAppPurchaseOfferCode activates or deactivates an in-app
    /// purchase offer code.
    pub async fn update_in_app_purchase_offer_code(&self, offer_code_id: &str, req: &InAppPurchaseOfferCodeUpdateRequest) -> Result<InAppPurchaseOfferCodeResponse> {
        todo!("port Client.UpdateInAppPurchaseOfferCode from internal/asc/api/client_commerce.go")
    }

    /// UpdateInAppPurchaseOfferCodeCustomCode activates or deactivates a
    /// custom code batch.
    pub async fn update_in_app_purchase_offer_code_custom_code(&self, custom_code_id: &str, req: &InAppPurchaseOfferCodeCustomCodeUpdateRequest) -> Result<InAppPurchaseOfferCodeCustomCodeResponse> {
        todo!("port Client.UpdateInAppPurchaseOfferCodeCustomCode from internal/asc/api/client_commerce.go")
    }

    /// UpdateInAppPurchaseOfferCodeOneTimeUseCode activates or deactivates a
    /// one-time-use code batch.
    pub async fn update_in_app_purchase_offer_code_one_time_use_code(&self, code_id: &str, req: &InAppPurchaseOfferCodeOneTimeUseCodeUpdateRequest) -> Result<InAppPurchaseOfferCodeOneTimeUseCodeResponse> {
        todo!("port Client.UpdateInAppPurchaseOfferCodeOneTimeUseCode from internal/asc/api/client_commerce.go")
    }

    /// UpdateSubscriptionGroupLocalization updates a subscription group localization.
    pub async fn update_subscription_group_localization(&self, localization_id: &str, req: &SubscriptionGroupLocalizationUpdateRequest) -> Result<SubscriptionGroupLocalizationResponse> {
        todo!("port Client.UpdateSubscriptionGroupLocalization from internal/asc/api/client_commerce.go")
    }

    /// UpdateSubscriptionImage commits a subscription image upload.
    pub async fn update_subscription_image(&self, image_id: &str, req: &SubscriptionImageUpdateRequest) -> Result<SubscriptionImageResponse> {
        todo!("port Client.UpdateSubscriptionImage from internal/asc/api/client_commerce.go")
    }

    /// UpdateSubscriptionLocalization updates a subscription localization.
    pub async fn update_subscription_localization(&self, localization_id: &str, req: &SubscriptionLocalizationUpdateRequest) -> Result<SubscriptionLocalizationResponse> {
        todo!("port Client.UpdateSubscriptionLocalization from internal/asc/api/client_commerce.go")
    }

    /// UpdateSubscriptionPlanAvailability updates a subscription plan availability.
    pub async fn update_subscription_plan_availability(&self, availability_id: &str, req: &SubscriptionPlanAvailabilityUpdateRequest) -> Result<SubscriptionPlanAvailabilityResponse> {
        todo!("port Client.UpdateSubscriptionPlanAvailability from internal/asc/api/client_commerce.go")
    }

    /// ListAppTagTerritories returns the territories where an app tag applies.
    pub async fn list_app_tag_territories(&self, app_tag_id: &str, opts: Option<&ListOptions>) -> Result<TerritoriesResponse> {
        todo!("port Client.ListAppTagTerritories from internal/asc/api/client_metadata_extras.go")
    }

    /// ListAppTags returns the Apple-created tags applied to an app.
    pub async fn list_app_tags(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<AppTagsResponse> {
        todo!("port Client.ListAppTags from internal/asc/api/client_metadata_extras.go")
    }

    /// UpdateAppTag updates an app tag's App Store visibility (opt-out).
    pub async fn update_app_tag(&self, app_tag_id: &str, req: &AppTagUpdateRequest) -> Result<AppTagResponse> {
        todo!("port Client.UpdateAppTag from internal/asc/api/client_metadata_extras.go")
    }
}
