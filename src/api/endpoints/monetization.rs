// App Store Connect API endpoints for the "monetization" area.
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
    /// AssignBuildToEncryptionDeclaration assigns a build to an encryption declaration
    /// by setting the build's appEncryptionDeclaration relationship. This replaces the
    /// deprecated POST /v1/appEncryptionDeclarations/{id}/relationships/builds endpoint.
    pub async fn assign_build_to_encryption_declaration(&self, declaration_id: &str, build_id: &str) -> Result<()> {
        todo!("port Client.AssignBuildToEncryptionDeclaration from internal/asc/api/client.go")
    }

    /// ClearSandboxTesterPurchaseHistory clears the purchase history for the
    /// given sandbox testers.
    pub async fn clear_sandbox_tester_purchase_history(&self, req: &SandboxTesterClearPurchaseHistoryRequest) -> Result<SandboxTesterClearPurchaseHistoryResponse> {
        todo!("port Client.ClearSandboxTesterPurchaseHistory from internal/asc/api/client.go")
    }

    /// CreateAppAvailability sets app availability.
    pub async fn create_app_availability(&self, req: &AppAvailabilityCreateRequest) -> Result<AppAvailabilityResponse> {
        todo!("port Client.CreateAppAvailability from internal/asc/api/client.go")
    }

    /// CreateAppEncryptionDeclaration creates an encryption declaration.
    pub async fn create_app_encryption_declaration(&self, req: &AppEncryptionDeclarationCreateRequest) -> Result<AppEncryptionDeclarationResponse> {
        todo!("port Client.CreateAppEncryptionDeclaration from internal/asc/api/client.go")
    }

    /// CreateInAppPurchase creates a new in-app purchase.
    pub async fn create_in_app_purchase(&self, req: &InAppPurchaseCreateRequest) -> Result<InAppPurchaseResponse> {
        todo!("port Client.CreateInAppPurchase from internal/asc/api/client.go")
    }

    /// CreatePromotedPurchase creates a promoted purchase.
    pub async fn create_promoted_purchase(&self, req: &PromotedPurchaseCreateRequest) -> Result<PromotedPurchaseResponse> {
        todo!("port Client.CreatePromotedPurchase from internal/asc/api/client.go")
    }

    /// CreateSubscriptionOfferCode creates an offer code.
    pub async fn create_subscription_offer_code(&self, req: &SubscriptionOfferCodeCreateRequest) -> Result<SubscriptionOfferCodeResponse> {
        todo!("port Client.CreateSubscriptionOfferCode from internal/asc/api/client.go")
    }

    /// CreateUserInvitation invites a new user.
    pub async fn create_user_invitation(&self, req: &UserInvitationCreateRequest) -> Result<UserInvitationResponse> {
        todo!("port Client.CreateUserInvitation from internal/asc/api/client.go")
    }

    /// CreateWinBackOffer creates a win-back offer.
    pub async fn create_win_back_offer(&self, req: &WinBackOfferCreateRequest) -> Result<WinBackOfferResponse> {
        todo!("port Client.CreateWinBackOffer from internal/asc/api/client.go")
    }

    /// DeleteInAppPurchase deletes an in-app purchase.
    pub async fn delete_in_app_purchase(&self, iap_id: &str) -> Result<()> {
        todo!("port Client.DeleteInAppPurchase from internal/asc/api/client.go")
    }

    /// DeletePromotedPurchase deletes a promoted purchase.
    pub async fn delete_promoted_purchase(&self, promoted_purchase_id: &str) -> Result<()> {
        todo!("port Client.DeletePromotedPurchase from internal/asc/api/client.go")
    }

    /// DeleteUser removes a user from the team.
    pub async fn delete_user(&self, user_id: &str) -> Result<()> {
        todo!("port Client.DeleteUser from internal/asc/api/client.go")
    }

    /// DeleteUserInvitation cancels a user invitation.
    pub async fn delete_user_invitation(&self, invitation_id: &str) -> Result<()> {
        todo!("port Client.DeleteUserInvitation from internal/asc/api/client.go")
    }

    /// DeleteWinBackOffer deletes a win-back offer.
    pub async fn delete_win_back_offer(&self, offer_id: &str) -> Result<()> {
        todo!("port Client.DeleteWinBackOffer from internal/asc/api/client.go")
    }

    /// EndAppAvailabilityPreOrder ends pre-order availability for the given
    /// territory availabilities.
    pub async fn end_app_availability_pre_order(&self, req: &EndAppAvailabilityPreOrderCreateRequest) -> Result<EndAppAvailabilityPreOrderResponse> {
        todo!("port Client.EndAppAvailabilityPreOrder from internal/asc/api/client.go")
    }

    /// GetAgeRatingDeclaration returns an age rating declaration.
    pub async fn get_age_rating_declaration(&self, app_info_id: &str) -> Result<AgeRatingDeclarationResponse> {
        todo!("port Client.GetAgeRatingDeclaration from internal/asc/api/client.go")
    }

    /// GetAppAvailability returns app availability.
    pub async fn get_app_availability(&self, app_id: &str) -> Result<AppAvailabilityResponse> {
        todo!("port Client.GetAppAvailability from internal/asc/api/client.go")
    }

    /// GetAppEncryptionDeclaration returns a single encryption declaration.
    pub async fn get_app_encryption_declaration(&self, declaration_id: &str) -> Result<AppEncryptionDeclarationResponse> {
        todo!("port Client.GetAppEncryptionDeclaration from internal/asc/api/client.go")
    }

    /// GetInAppPurchase returns a single in-app purchase by ID.
    pub async fn get_in_app_purchase(&self, iap_id: &str) -> Result<InAppPurchaseResponse> {
        todo!("port Client.GetInAppPurchase from internal/asc/api/client.go")
    }

    /// GetPromotedPurchase returns a single promoted purchase.
    pub async fn get_promoted_purchase(&self, promoted_purchase_id: &str) -> Result<PromotedPurchaseResponse> {
        todo!("port Client.GetPromotedPurchase from internal/asc/api/client.go")
    }

    /// GetSubscription returns a single subscription by ID.
    pub async fn get_subscription(&self, subscription_id: &str) -> Result<SubscriptionResponse> {
        todo!("port Client.GetSubscription from internal/asc/api/client.go")
    }

    /// GetSubscriptionGroup returns a single subscription group by ID.
    pub async fn get_subscription_group(&self, group_id: &str) -> Result<SubscriptionGroupResponse> {
        todo!("port Client.GetSubscriptionGroup from internal/asc/api/client.go")
    }

    /// GetSubscriptionOfferCode returns a single offer code.
    pub async fn get_subscription_offer_code(&self, offer_code_id: &str) -> Result<SubscriptionOfferCodeResponse> {
        todo!("port Client.GetSubscriptionOfferCode from internal/asc/api/client.go")
    }

    /// GetUser returns a single user.
    pub async fn get_user(&self, user_id: &str) -> Result<UserResponse> {
        todo!("port Client.GetUser from internal/asc/api/client.go")
    }

    /// GetUserInvitation returns a single user invitation.
    pub async fn get_user_invitation(&self, invitation_id: &str) -> Result<UserInvitationResponse> {
        todo!("port Client.GetUserInvitation from internal/asc/api/client.go")
    }

    /// GetWinBackOffer returns a single win-back offer.
    pub async fn get_win_back_offer(&self, offer_id: &str) -> Result<WinBackOfferResponse> {
        todo!("port Client.GetWinBackOffer from internal/asc/api/client.go")
    }

    /// ListAppEncryptionDeclarations returns encryption declarations for an app.
    pub async fn list_app_encryption_declarations(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<AppEncryptionDeclarationsResponse> {
        todo!("port Client.ListAppEncryptionDeclarations from internal/asc/api/client.go")
    }

    /// ListInAppPurchases returns in-app purchases for an app.
    pub async fn list_in_app_purchases(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<InAppPurchasesResponse> {
        todo!("port Client.ListInAppPurchases from internal/asc/api/client.go")
    }

    /// ListPromotedPurchases returns promoted purchases for an app.
    pub async fn list_promoted_purchases(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<PromotedPurchasesResponse> {
        todo!("port Client.ListPromotedPurchases from internal/asc/api/client.go")
    }

    /// ListSandboxTesters returns a list of sandbox testers.
    pub async fn list_sandbox_testers(&self, opts: Option<&ListOptions>) -> Result<SandboxTestersResponse> {
        todo!("port Client.ListSandboxTesters from internal/asc/api/client.go")
    }

    /// ListSubscriptionGroups returns subscription groups for an app.
    pub async fn list_subscription_groups(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionGroupsResponse> {
        todo!("port Client.ListSubscriptionGroups from internal/asc/api/client.go")
    }

    /// ListSubscriptionOfferCodes returns offer codes for a subscription.
    pub async fn list_subscription_offer_codes(&self, subscription_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionOfferCodesResponse> {
        todo!("port Client.ListSubscriptionOfferCodes from internal/asc/api/client.go")
    }

    /// ListSubscriptions returns subscriptions for a subscription group.
    pub async fn list_subscriptions(&self, group_id: &str, opts: Option<&ListOptions>) -> Result<SubscriptionsResponse> {
        todo!("port Client.ListSubscriptions from internal/asc/api/client.go")
    }

    /// ListTerritoryAvailabilities returns territory availabilities.
    pub async fn list_territory_availabilities(&self, app_availability_id: &str, opts: Option<&ListOptions>) -> Result<TerritoryAvailabilitiesResponse> {
        todo!("port Client.ListTerritoryAvailabilities from internal/asc/api/client.go")
    }

    /// ListUserInvitations returns a list of user invitations.
    pub async fn list_user_invitations(&self, opts: Option<&ListOptions>) -> Result<UserInvitationsResponse> {
        todo!("port Client.ListUserInvitations from internal/asc/api/client.go")
    }

    /// ListUsers returns a list of users.
    pub async fn list_users(&self, opts: Option<&ListOptions>) -> Result<UsersResponse> {
        todo!("port Client.ListUsers from internal/asc/api/client.go")
    }

    /// ListWinBackOffers returns win-back offers for a subscription.
    pub async fn list_win_back_offers(&self, subscription_id: &str, opts: Option<&ListOptions>) -> Result<WinBackOffersResponse> {
        todo!("port Client.ListWinBackOffers from internal/asc/api/client.go")
    }

    /// UpdateAgeRatingDeclaration updates an age rating declaration.
    pub async fn update_age_rating_declaration(&self, declaration_id: &str, req: &AgeRatingDeclarationUpdateRequest) -> Result<AgeRatingDeclarationResponse> {
        todo!("port Client.UpdateAgeRatingDeclaration from internal/asc/api/client.go")
    }

    /// UpdateInAppPurchase updates an in-app purchase.
    pub async fn update_in_app_purchase(&self, iap_id: &str, req: &InAppPurchaseUpdateRequest) -> Result<InAppPurchaseResponse> {
        todo!("port Client.UpdateInAppPurchase from internal/asc/api/client.go")
    }

    /// UpdatePromotedPurchase updates a promoted purchase.
    pub async fn update_promoted_purchase(&self, promoted_purchase_id: &str, req: &PromotedPurchaseUpdateRequest) -> Result<PromotedPurchaseResponse> {
        todo!("port Client.UpdatePromotedPurchase from internal/asc/api/client.go")
    }

    /// UpdateSandboxTester updates a sandbox tester.
    pub async fn update_sandbox_tester(&self, tester_id: &str, req: &SandboxTesterUpdateRequest) -> Result<SandboxTesterResponse> {
        todo!("port Client.UpdateSandboxTester from internal/asc/api/client.go")
    }

    /// UpdateSubscriptionOfferCode updates an offer code.
    pub async fn update_subscription_offer_code(&self, offer_code_id: &str, req: &SubscriptionOfferCodeUpdateRequest) -> Result<SubscriptionOfferCodeResponse> {
        todo!("port Client.UpdateSubscriptionOfferCode from internal/asc/api/client.go")
    }

    /// UpdateUser updates a user.
    pub async fn update_user(&self, user_id: &str, req: &UserUpdateRequest) -> Result<UserResponse> {
        todo!("port Client.UpdateUser from internal/asc/api/client.go")
    }

    /// UpdateWinBackOffer updates a win-back offer.
    pub async fn update_win_back_offer(&self, offer_id: &str, req: &WinBackOfferUpdateRequest) -> Result<WinBackOfferResponse> {
        todo!("port Client.UpdateWinBackOffer from internal/asc/api/client.go")
    }

    /// CreateAndroidToIosAppMappingDetail creates an Android-to-iOS app mapping.
    pub async fn create_android_to_ios_app_mapping_detail(&self, req: &AndroidToIosAppMappingDetailCreateRequest) -> Result<AndroidToIosAppMappingDetailResponse> {
        todo!("port Client.CreateAndroidToIosAppMappingDetail from internal/asc/api/client_metadata_extras.go")
    }

    /// DeleteAndroidToIosAppMappingDetail deletes an Android-to-iOS app mapping.
    pub async fn delete_android_to_ios_app_mapping_detail(&self, mapping_id: &str) -> Result<()> {
        todo!("port Client.DeleteAndroidToIosAppMappingDetail from internal/asc/api/client_metadata_extras.go")
    }

    /// GetAndroidToIosAppMappingDetail returns a single Android-to-iOS app mapping.
    pub async fn get_android_to_ios_app_mapping_detail(&self, mapping_id: &str) -> Result<AndroidToIosAppMappingDetailResponse> {
        todo!("port Client.GetAndroidToIosAppMappingDetail from internal/asc/api/client_metadata_extras.go")
    }

    /// ListAndroidToIosAppMappingDetails returns the Android-to-iOS app mappings
    /// for an app.
    pub async fn list_android_to_ios_app_mapping_details(&self, app_id: &str, opts: Option<&ListOptions>) -> Result<AndroidToIosAppMappingDetailsResponse> {
        todo!("port Client.ListAndroidToIosAppMappingDetails from internal/asc/api/client_metadata_extras.go")
    }

    /// ListTerritoryAgeRatings returns an app's age ratings per territory for an
    /// app info.
    pub async fn list_territory_age_ratings(&self, app_info_id: &str, opts: Option<&ListOptions>) -> Result<TerritoryAgeRatingsResponse> {
        todo!("port Client.ListTerritoryAgeRatings from internal/asc/api/client_metadata_extras.go")
    }

    /// UpdateAndroidToIosAppMappingDetail updates an Android-to-iOS app mapping.
    pub async fn update_android_to_ios_app_mapping_detail(&self, mapping_id: &str, req: &AndroidToIosAppMappingDetailUpdateRequest) -> Result<AndroidToIosAppMappingDetailResponse> {
        todo!("port Client.UpdateAndroidToIosAppMappingDetail from internal/asc/api/client_metadata_extras.go")
    }
}
