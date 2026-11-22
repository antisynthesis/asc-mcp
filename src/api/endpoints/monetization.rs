// App Store Connect API endpoints for the "monetization" area.
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
    /// AssignBuildToEncryptionDeclaration assigns a build to an encryption declaration
    /// by setting the build's appEncryptionDeclaration relationship. This replaces the
    /// deprecated POST /v1/appEncryptionDeclarations/{id}/relationships/builds endpoint.
    pub async fn assign_build_to_encryption_declaration(
        &self,
        declaration_id: &str,
        build_id: &str,
    ) -> Result<()> {
        let body = serde_json::json!({
            "data": {
                "type": "appEncryptionDeclarations",
                "id": declaration_id,
            },
        });

        self.patch(
            &format!(
                "/v1/builds/{}/relationships/appEncryptionDeclaration",
                url::path_escape(build_id)
            ),
            Some(&body),
        )
        .await?;
        Ok(())
    }

    /// ClearSandboxTesterPurchaseHistory clears the purchase history for the
    /// given sandbox testers.
    pub async fn clear_sandbox_tester_purchase_history(
        &self,
        req: &SandboxTesterClearPurchaseHistoryRequest,
    ) -> Result<SandboxTesterClearPurchaseHistoryResponse> {
        let data = self
            .post("/v2/sandboxTestersClearPurchaseHistoryRequest", Some(req))
            .await?;
        decode(&data)
    }

    /// CreateAppAvailability sets app availability.
    pub async fn create_app_availability(
        &self,
        req: &AppAvailabilityCreateRequest,
    ) -> Result<AppAvailabilityResponse> {
        let data = self.post("/v2/appAvailabilities", Some(req)).await?;
        decode(&data)
    }

    /// CreateAppEncryptionDeclaration creates an encryption declaration.
    pub async fn create_app_encryption_declaration(
        &self,
        req: &AppEncryptionDeclarationCreateRequest,
    ) -> Result<AppEncryptionDeclarationResponse> {
        let data = self
            .post("/v1/appEncryptionDeclarations", Some(req))
            .await?;
        decode(&data)
    }

    /// CreateInAppPurchase creates a new in-app purchase.
    pub async fn create_in_app_purchase(
        &self,
        req: &InAppPurchaseCreateRequest,
    ) -> Result<InAppPurchaseResponse> {
        let data = self.post("/v2/inAppPurchases", Some(req)).await?;
        decode(&data)
    }

    /// CreatePromotedPurchase creates a promoted purchase.
    pub async fn create_promoted_purchase(
        &self,
        req: &PromotedPurchaseCreateRequest,
    ) -> Result<PromotedPurchaseResponse> {
        let data = self.post("/v1/promotedPurchases", Some(req)).await?;
        decode(&data)
    }

    /// CreateSubscriptionOfferCode creates an offer code.
    pub async fn create_subscription_offer_code(
        &self,
        req: &SubscriptionOfferCodeCreateRequest,
    ) -> Result<SubscriptionOfferCodeResponse> {
        let data = self.post("/v1/subscriptionOfferCodes", Some(req)).await?;
        decode(&data)
    }

    /// CreateUserInvitation invites a new user.
    pub async fn create_user_invitation(
        &self,
        req: &UserInvitationCreateRequest,
    ) -> Result<UserInvitationResponse> {
        let data = self.post("/v1/userInvitations", Some(req)).await?;
        decode(&data)
    }

    /// CreateWinBackOffer creates a win-back offer.
    pub async fn create_win_back_offer(
        &self,
        req: &WinBackOfferCreateRequest,
    ) -> Result<WinBackOfferResponse> {
        let data = self.post("/v1/winBackOffers", Some(req)).await?;
        decode(&data)
    }

    /// DeleteInAppPurchase deletes an in-app purchase.
    pub async fn delete_in_app_purchase(&self, iap_id: &str) -> Result<()> {
        self.delete(&format!("/v2/inAppPurchases/{}", url::path_escape(iap_id)))
            .await
    }

    /// DeletePromotedPurchase deletes a promoted purchase.
    pub async fn delete_promoted_purchase(&self, promoted_purchase_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/promotedPurchases/{}",
            url::path_escape(promoted_purchase_id)
        ))
        .await
    }

    /// DeleteUser removes a user from the team.
    pub async fn delete_user(&self, user_id: &str) -> Result<()> {
        self.delete(&format!("/v1/users/{}", url::path_escape(user_id)))
            .await
    }

    /// DeleteUserInvitation cancels a user invitation.
    pub async fn delete_user_invitation(&self, invitation_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/userInvitations/{}",
            url::path_escape(invitation_id)
        ))
        .await
    }

    /// DeleteWinBackOffer deletes a win-back offer.
    pub async fn delete_win_back_offer(&self, offer_id: &str) -> Result<()> {
        self.delete(&format!("/v1/winBackOffers/{}", url::path_escape(offer_id)))
            .await
    }

    /// EndAppAvailabilityPreOrder ends pre-order availability for the given
    /// territory availabilities.
    pub async fn end_app_availability_pre_order(
        &self,
        req: &EndAppAvailabilityPreOrderCreateRequest,
    ) -> Result<EndAppAvailabilityPreOrderResponse> {
        let data = self
            .post("/v1/endAppAvailabilityPreOrders", Some(req))
            .await?;
        decode(&data)
    }

    /// GetAgeRatingDeclaration returns an age rating declaration.
    pub async fn get_age_rating_declaration(
        &self,
        app_info_id: &str,
    ) -> Result<AgeRatingDeclarationResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/appInfos/{}/ageRatingDeclaration",
                    url::path_escape(app_info_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetAppAvailability returns app availability.
    pub async fn get_app_availability(&self, app_id: &str) -> Result<AppAvailabilityResponse> {
        let data = self
            .get(
                &format!("/v1/apps/{}/appAvailabilityV2", url::path_escape(app_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetAppEncryptionDeclaration returns a single encryption declaration.
    pub async fn get_app_encryption_declaration(
        &self,
        declaration_id: &str,
    ) -> Result<AppEncryptionDeclarationResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/appEncryptionDeclarations/{}",
                    url::path_escape(declaration_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetInAppPurchase returns a single in-app purchase by ID.
    pub async fn get_in_app_purchase(&self, iap_id: &str) -> Result<InAppPurchaseResponse> {
        let data = self
            .get(
                &format!("/v2/inAppPurchases/{}", url::path_escape(iap_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetPromotedPurchase returns a single promoted purchase.
    pub async fn get_promoted_purchase(
        &self,
        promoted_purchase_id: &str,
    ) -> Result<PromotedPurchaseResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/promotedPurchases/{}",
                    url::path_escape(promoted_purchase_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetSubscription returns a single subscription by ID.
    pub async fn get_subscription(&self, subscription_id: &str) -> Result<SubscriptionResponse> {
        let data = self
            .get(
                &format!("/v1/subscriptions/{}", url::path_escape(subscription_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetSubscriptionGroup returns a single subscription group by ID.
    pub async fn get_subscription_group(
        &self,
        group_id: &str,
    ) -> Result<SubscriptionGroupResponse> {
        let data = self
            .get(
                &format!("/v1/subscriptionGroups/{}", url::path_escape(group_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetSubscriptionOfferCode returns a single offer code.
    pub async fn get_subscription_offer_code(
        &self,
        offer_code_id: &str,
    ) -> Result<SubscriptionOfferCodeResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/subscriptionOfferCodes/{}",
                    url::path_escape(offer_code_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetUser returns a single user.
    pub async fn get_user(&self, user_id: &str) -> Result<UserResponse> {
        let data = self
            .get(&format!("/v1/users/{}", url::path_escape(user_id)), None)
            .await?;
        decode(&data)
    }

    /// GetUserInvitation returns a single user invitation.
    pub async fn get_user_invitation(&self, invitation_id: &str) -> Result<UserInvitationResponse> {
        let data = self
            .get(
                &format!("/v1/userInvitations/{}", url::path_escape(invitation_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// GetWinBackOffer returns a single win-back offer.
    pub async fn get_win_back_offer(&self, offer_id: &str) -> Result<WinBackOfferResponse> {
        let data = self
            .get(
                &format!("/v1/winBackOffers/{}", url::path_escape(offer_id)),
                None,
            )
            .await?;
        decode(&data)
    }

    /// ListAppEncryptionDeclarations returns encryption declarations for an app.
    pub async fn list_app_encryption_declarations(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AppEncryptionDeclarationsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        if !app_id.is_empty() {
            query.set("filter[app]", app_id);
        }
        let data = self
            .get("/v1/appEncryptionDeclarations", Some(&query))
            .await?;
        decode(&data)
    }

    /// ListInAppPurchases returns in-app purchases for an app.
    pub async fn list_in_app_purchases(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<InAppPurchasesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/apps/{}/inAppPurchasesV2", url::path_escape(app_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListPromotedPurchases returns promoted purchases for an app.
    pub async fn list_promoted_purchases(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<PromotedPurchasesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/apps/{}/promotedPurchases", url::path_escape(app_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListSandboxTesters returns a list of sandbox testers.
    pub async fn list_sandbox_testers(
        &self,
        opts: Option<&ListOptions>,
    ) -> Result<SandboxTestersResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get("/v2/sandboxTesters", Some(&query)).await?;
        decode(&data)
    }

    /// ListSubscriptionGroups returns subscription groups for an app.
    pub async fn list_subscription_groups(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<SubscriptionGroupsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!("/v1/apps/{}/subscriptionGroups", url::path_escape(app_id)),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListSubscriptionOfferCodes returns offer codes for a subscription.
    pub async fn list_subscription_offer_codes(
        &self,
        subscription_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<SubscriptionOfferCodesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/subscriptions/{}/offerCodes",
                    url::path_escape(subscription_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListSubscriptions returns subscriptions for a subscription group.
    pub async fn list_subscriptions(
        &self,
        group_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<SubscriptionsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/subscriptionGroups/{}/subscriptions",
                    url::path_escape(group_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListTerritoryAvailabilities returns territory availabilities.
    pub async fn list_territory_availabilities(
        &self,
        app_availability_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<TerritoryAvailabilitiesResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v2/appAvailabilities/{}/territoryAvailabilities",
                    url::path_escape(app_availability_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListUserInvitations returns a list of user invitations.
    pub async fn list_user_invitations(
        &self,
        opts: Option<&ListOptions>,
    ) -> Result<UserInvitationsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get("/v1/userInvitations", Some(&query)).await?;
        decode(&data)
    }

    /// ListUsers returns a list of users.
    pub async fn list_users(&self, opts: Option<&ListOptions>) -> Result<UsersResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self.get("/v1/users", Some(&query)).await?;
        decode(&data)
    }

    /// ListWinBackOffers returns win-back offers for a subscription.
    pub async fn list_win_back_offers(
        &self,
        subscription_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<WinBackOffersResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/subscriptions/{}/winBackOffers",
                    url::path_escape(subscription_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// UpdateAgeRatingDeclaration updates an age rating declaration.
    pub async fn update_age_rating_declaration(
        &self,
        declaration_id: &str,
        req: &AgeRatingDeclarationUpdateRequest,
    ) -> Result<AgeRatingDeclarationResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/ageRatingDeclarations/{}",
                    url::path_escape(declaration_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateInAppPurchase updates an in-app purchase.
    pub async fn update_in_app_purchase(
        &self,
        iap_id: &str,
        req: &InAppPurchaseUpdateRequest,
    ) -> Result<InAppPurchaseResponse> {
        let data = self
            .patch(
                &format!("/v2/inAppPurchases/{}", url::path_escape(iap_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdatePromotedPurchase updates a promoted purchase.
    pub async fn update_promoted_purchase(
        &self,
        promoted_purchase_id: &str,
        req: &PromotedPurchaseUpdateRequest,
    ) -> Result<PromotedPurchaseResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/promotedPurchases/{}",
                    url::path_escape(promoted_purchase_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateSandboxTester updates a sandbox tester.
    pub async fn update_sandbox_tester(
        &self,
        tester_id: &str,
        req: &SandboxTesterUpdateRequest,
    ) -> Result<SandboxTesterResponse> {
        let data = self
            .patch(
                &format!("/v2/sandboxTesters/{}", url::path_escape(tester_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateSubscriptionOfferCode updates an offer code.
    pub async fn update_subscription_offer_code(
        &self,
        offer_code_id: &str,
        req: &SubscriptionOfferCodeUpdateRequest,
    ) -> Result<SubscriptionOfferCodeResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/subscriptionOfferCodes/{}",
                    url::path_escape(offer_code_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateUser updates a user.
    pub async fn update_user(
        &self,
        user_id: &str,
        req: &UserUpdateRequest,
    ) -> Result<UserResponse> {
        let data = self
            .patch(
                &format!("/v1/users/{}", url::path_escape(user_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// UpdateWinBackOffer updates a win-back offer.
    pub async fn update_win_back_offer(
        &self,
        offer_id: &str,
        req: &WinBackOfferUpdateRequest,
    ) -> Result<WinBackOfferResponse> {
        let data = self
            .patch(
                &format!("/v1/winBackOffers/{}", url::path_escape(offer_id)),
                Some(req),
            )
            .await?;
        decode(&data)
    }

    /// CreateAndroidToIosAppMappingDetail creates an Android-to-iOS app mapping.
    pub async fn create_android_to_ios_app_mapping_detail(
        &self,
        req: &AndroidToIosAppMappingDetailCreateRequest,
    ) -> Result<AndroidToIosAppMappingDetailResponse> {
        let data = self
            .post("/v1/androidToIosAppMappingDetails", Some(req))
            .await?;
        decode(&data)
    }

    /// DeleteAndroidToIosAppMappingDetail deletes an Android-to-iOS app mapping.
    pub async fn delete_android_to_ios_app_mapping_detail(&self, mapping_id: &str) -> Result<()> {
        self.delete(&format!(
            "/v1/androidToIosAppMappingDetails/{}",
            url::path_escape(mapping_id)
        ))
        .await
    }

    /// GetAndroidToIosAppMappingDetail returns a single Android-to-iOS app mapping.
    pub async fn get_android_to_ios_app_mapping_detail(
        &self,
        mapping_id: &str,
    ) -> Result<AndroidToIosAppMappingDetailResponse> {
        let data = self
            .get(
                &format!(
                    "/v1/androidToIosAppMappingDetails/{}",
                    url::path_escape(mapping_id)
                ),
                None,
            )
            .await?;
        decode(&data)
    }

    /// ListAndroidToIosAppMappingDetails returns the Android-to-iOS app mappings
    /// for an app.
    pub async fn list_android_to_ios_app_mapping_details(
        &self,
        app_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<AndroidToIosAppMappingDetailsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/apps/{}/androidToIosAppMappingDetails",
                    url::path_escape(app_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// ListTerritoryAgeRatings returns an app's age ratings per territory for an
    /// app info.
    pub async fn list_territory_age_ratings(
        &self,
        app_info_id: &str,
        opts: Option<&ListOptions>,
    ) -> Result<TerritoryAgeRatingsResponse> {
        let mut query = Values::new();
        if let Some(o) = opts {
            o.apply(&mut query);
        }
        let data = self
            .get(
                &format!(
                    "/v1/appInfos/{}/territoryAgeRatings",
                    url::path_escape(app_info_id)
                ),
                Some(&query),
            )
            .await?;
        decode(&data)
    }

    /// UpdateAndroidToIosAppMappingDetail updates an Android-to-iOS app mapping.
    pub async fn update_android_to_ios_app_mapping_detail(
        &self,
        mapping_id: &str,
        req: &AndroidToIosAppMappingDetailUpdateRequest,
    ) -> Result<AndroidToIosAppMappingDetailResponse> {
        let data = self
            .patch(
                &format!(
                    "/v1/androidToIosAppMappingDetails/{}",
                    url::path_escape(mapping_id)
                ),
                Some(req),
            )
            .await?;
        decode(&data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::testutil::{MockApi, MockResponse};

    #[tokio::test]
    async fn list_in_app_purchases_with_options() {
        let mock = MockApi::start(|_| {
            MockResponse::json(200, r#"{"data":[{"type":"inAppPurchases","id":"i1","attributes":{"name":"Coins"}}],"links":{"next":"n"}}"#)
        })
        .await;
        let mut opts = ListOptions {
            limit: 10,
            ..Default::default()
        };
        opts.filter
            .insert("state", GoSlice::from(vec!["APPROVED".to_string()]));
        let resp = mock
            .client()
            .list_in_app_purchases("a/1", Some(&opts))
            .await
            .unwrap();
        assert_eq!(resp.data[0].attributes.name, "Coins");
        assert_eq!(resp.links.next, "n");
        let reqs = mock.requests();
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0].method, "GET");
        assert_eq!(reqs[0].path, "/v1/apps/a%2F1/inAppPurchasesV2");
        assert_eq!(reqs[0].query, "filter%5Bstate%5D=APPROVED&limit=10");
    }

    #[tokio::test]
    async fn list_users_without_options() {
        let mock = MockApi::start(|_| MockResponse::json(200, r#"{"data":[],"links":{}}"#)).await;
        let resp = mock.client().list_users(None).await.unwrap();
        assert!(resp.data.is_empty());
        let reqs = mock.requests();
        assert_eq!(reqs[0].method, "GET");
        assert_eq!(reqs[0].path, "/v1/users");
        assert_eq!(reqs[0].query, "");
    }

    #[tokio::test]
    async fn list_app_encryption_declarations_sets_app_filter() {
        let mock = MockApi::start(|_| MockResponse::json(200, r#"{"data":[],"links":{}}"#)).await;
        let mut opts = ListOptions {
            limit: 5,
            ..Default::default()
        };
        opts.filter
            .insert("app", GoSlice::from(vec!["other".to_string()]));
        mock.client()
            .list_app_encryption_declarations("app1", Some(&opts))
            .await
            .unwrap();
        mock.client()
            .list_app_encryption_declarations("", None)
            .await
            .unwrap();
        let reqs = mock.requests();
        assert_eq!(reqs[0].path, "/v1/appEncryptionDeclarations");
        assert_eq!(reqs[0].query_param("filter[app]").as_deref(), Some("app1"));
        assert_eq!(reqs[0].query_param("limit").as_deref(), Some("5"));
        assert_eq!(reqs[1].query, "");
    }

    #[tokio::test]
    async fn get_subscription_group() {
        let mock = MockApi::start(|_| {
            MockResponse::json(200, r#"{"data":{"type":"subscriptionGroups","id":"g1","attributes":{"referenceName":"Pro"}}}"#)
        })
        .await;
        let resp = mock.client().get_subscription_group("g1").await.unwrap();
        assert_eq!(resp.data.attributes.reference_name, "Pro");
        let reqs = mock.requests();
        assert_eq!(reqs[0].method, "GET");
        assert_eq!(reqs[0].path, "/v1/subscriptionGroups/g1");
        assert_eq!(reqs[0].query, "");
        assert!(reqs[0].body.is_empty());
    }

    #[tokio::test]
    async fn get_app_availability_path() {
        let mock = MockApi::start(|_| {
            MockResponse::json(200, r#"{"data":{"type":"appAvailabilities","id":"av1","attributes":{"availableInNewTerritories":true}}}"#)
        })
        .await;
        let resp = mock.client().get_app_availability("app1").await.unwrap();
        assert!(resp.data.attributes.available_in_new_territories);
        assert_eq!(mock.requests()[0].path, "/v1/apps/app1/appAvailabilityV2");
    }

    #[tokio::test]
    async fn create_in_app_purchase_body() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                201,
                r#"{"data":{"type":"inAppPurchases","id":"new","attributes":{"name":"Coins"}}}"#,
            )
        })
        .await;
        let req = InAppPurchaseCreateRequest {
            data: InAppPurchaseCreateData {
                type_: "inAppPurchases".into(),
                attributes: InAppPurchaseCreateAttributes {
                    name: "Coins".into(),
                    product_id: "com.x.coins".into(),
                    in_app_purchase_type: "CONSUMABLE".into(),
                    ..Default::default()
                },
                relationships: InAppPurchaseCreateRelationships {
                    app: RelationshipData {
                        data: ResourceIdentifier {
                            type_: "apps".into(),
                            id: "a1".into(),
                        },
                    },
                },
            },
        };
        let resp = mock.client().create_in_app_purchase(&req).await.unwrap();
        assert_eq!(resp.data.id, "new");
        let reqs = mock.requests();
        assert_eq!(reqs[0].method, "POST");
        assert_eq!(reqs[0].path, "/v2/inAppPurchases");
        assert_eq!(
            reqs[0].json(),
            serde_json::json!({"data":{"type":"inAppPurchases","attributes":{"name":"Coins","productId":"com.x.coins","inAppPurchaseType":"CONSUMABLE"},"relationships":{"app":{"data":{"type":"apps","id":"a1"}}}}})
        );
    }

    #[tokio::test]
    async fn update_user_body() {
        let mock = MockApi::start(|_| {
            MockResponse::json(
                200,
                r#"{"data":{"type":"users","id":"u1","attributes":{}}}"#,
            )
        })
        .await;
        let req = UserUpdateRequest {
            data: UserUpdateData {
                type_: "users".into(),
                id: "u1".into(),
                attributes: UserUpdateAttributes {
                    all_apps_visible: Some(false),
                    ..Default::default()
                },
                relationships: None,
            },
        };
        mock.client().update_user("u1", &req).await.unwrap();
        let reqs = mock.requests();
        assert_eq!(reqs[0].method, "PATCH");
        assert_eq!(reqs[0].path, "/v1/users/u1");
        assert_eq!(
            reqs[0].json(),
            serde_json::json!({"data":{"type":"users","id":"u1","attributes":{"allAppsVisible":false}}})
        );
    }

    #[tokio::test]
    async fn assign_build_to_encryption_declaration_body() {
        let mock = MockApi::start(|_| MockResponse::empty(204)).await;
        mock.client()
            .assign_build_to_encryption_declaration("d1", "b/1")
            .await
            .unwrap();
        let reqs = mock.requests();
        assert_eq!(reqs[0].method, "PATCH");
        assert_eq!(
            reqs[0].path,
            "/v1/builds/b%2F1/relationships/appEncryptionDeclaration"
        );
        assert_eq!(
            String::from_utf8(reqs[0].body.clone()).unwrap(),
            r#"{"data":{"id":"d1","type":"appEncryptionDeclarations"}}"#
        );
    }

    #[tokio::test]
    async fn delete_win_back_offer() {
        let mock = MockApi::start(|_| MockResponse::empty(204)).await;
        mock.client().delete_win_back_offer("o1").await.unwrap();
        let reqs = mock.requests();
        assert_eq!(reqs[0].method, "DELETE");
        assert_eq!(reqs[0].path, "/v1/winBackOffers/o1");
        assert!(reqs[0].body.is_empty());
    }

    #[tokio::test]
    async fn delete_propagates_api_error() {
        let mock = MockApi::start(|_| {
            MockResponse::json(404, r#"{"errors":[{"status":"404","code":"NOT_FOUND","title":"Not Found","detail":"nope"}]}"#)
        })
        .await;
        let err = mock
            .client()
            .delete_android_to_ios_app_mapping_detail("m1")
            .await
            .unwrap_err();
        assert!(err.api().unwrap().is_not_found());
        assert_eq!(
            mock.requests()[0].path,
            "/v1/androidToIosAppMappingDetails/m1"
        );
    }

    #[tokio::test]
    async fn decode_error_is_wrapped() {
        let mock = MockApi::start(|_| MockResponse::json(200, "not json")).await;
        let err = mock.client().get_user("u1").await.unwrap_err();
        assert!(
            err.to_string()
                .starts_with("failed to unmarshal response: "),
            "{err}"
        );
    }
}
