// Package api provides types for the App Store Connect API.
package api

import "time"

// Response wrapper types following JSON:API specification.

// PagedDocumentLinks contains pagination links.
type PagedDocumentLinks struct {
	Self  string `json:"self,omitempty"`
	First string `json:"first,omitempty"`
	Next  string `json:"next,omitempty"`
}

// PagingInformation contains pagination metadata.
type PagingInformation struct {
	Paging struct {
		Total int `json:"total"`
		Limit int `json:"limit"`
	} `json:"paging"`
}

// ErrorResponse represents an API error response.
type ErrorResponse struct {
	Errors []APIError `json:"errors"`
}

// APIError represents a single API error.
type APIError struct {
	ID     string `json:"id,omitempty"`
	Status string `json:"status"`
	Code   string `json:"code"`
	Title  string `json:"title"`
	Detail string `json:"detail"`
}

// App types

// AppsResponse represents a list of apps response.
type AppsResponse struct {
	Data     []App              `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// AppResponse represents a single app response.
type AppResponse struct {
	Data     App   `json:"data"`
	Included []any `json:"included,omitempty"`
}

// App represents an App Store Connect app.
type App struct {
	Type       string        `json:"type"`
	ID         string        `json:"id"`
	Attributes AppAttributes `json:"attributes"`
}

// AppAttributes contains app attributes.
type AppAttributes struct {
	Name                         string `json:"name,omitempty"`
	BundleID                     string `json:"bundleId,omitempty"`
	SKU                          string `json:"sku,omitempty"`
	PrimaryLocale                string `json:"primaryLocale,omitempty"`
	IsOrEverWasMadeForKids       bool   `json:"isOrEverWasMadeForKids,omitempty"`
	ContentRightsDeclaration     string `json:"contentRightsDeclaration,omitempty"`
	StreamlinedPurchasingEnabled bool   `json:"streamlinedPurchasingEnabled,omitempty"`
}

// Build types

// BuildsResponse represents a list of builds response.
type BuildsResponse struct {
	Data     []Build            `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// BuildResponse represents a single build response.
type BuildResponse struct {
	Data     Build `json:"data"`
	Included []any `json:"included,omitempty"`
}

// Build represents an App Store Connect build.
type Build struct {
	Type       string          `json:"type"`
	ID         string          `json:"id"`
	Attributes BuildAttributes `json:"attributes"`
}

// BuildAttributes contains build attributes.
type BuildAttributes struct {
	Version                 string     `json:"version,omitempty"`
	UploadedDate            *time.Time `json:"uploadedDate,omitempty"`
	ExpirationDate          *time.Time `json:"expirationDate,omitempty"`
	Expired                 bool       `json:"expired,omitempty"`
	MinOsVersion            string     `json:"minOsVersion,omitempty"`
	LsMinimumSystemVersion  string     `json:"lsMinimumSystemVersion,omitempty"`
	ComputedMinMacOsVersion string     `json:"computedMinMacOsVersion,omitempty"`
	IconAssetToken          any        `json:"iconAssetToken,omitempty"`
	ProcessingState         string     `json:"processingState,omitempty"`
	BuildAudienceType       string     `json:"buildAudienceType,omitempty"`
	UsesNonExemptEncryption bool       `json:"usesNonExemptEncryption,omitempty"`
}

// AppStoreVersion types

// AppStoreVersionsResponse represents a list of app store versions.
type AppStoreVersionsResponse struct {
	Data     []AppStoreVersion  `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// AppStoreVersionResponse represents a single app store version.
type AppStoreVersionResponse struct {
	Data     AppStoreVersion `json:"data"`
	Included []any           `json:"included,omitempty"`
}

// AppStoreVersion represents an App Store version.
type AppStoreVersion struct {
	Type       string                    `json:"type"`
	ID         string                    `json:"id"`
	Attributes AppStoreVersionAttributes `json:"attributes"`
}

// AppStoreVersionAttributes contains app store version attributes.
type AppStoreVersionAttributes struct {
	Platform            string     `json:"platform,omitempty"`
	VersionString       string     `json:"versionString,omitempty"`
	AppStoreState       string     `json:"appStoreState,omitempty"`
	Copyright           string     `json:"copyright,omitempty"`
	ReleaseType         string     `json:"releaseType,omitempty"`
	EarliestReleaseDate *time.Time `json:"earliestReleaseDate,omitempty"`
	Downloadable        bool       `json:"downloadable,omitempty"`
	CreatedDate         *time.Time `json:"createdDate,omitempty"`
}

// BetaGroup types

// BetaGroupsResponse represents a list of beta groups.
type BetaGroupsResponse struct {
	Data     []BetaGroup        `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// BetaGroupResponse represents a single beta group.
type BetaGroupResponse struct {
	Data     BetaGroup `json:"data"`
	Included []any     `json:"included,omitempty"`
}

// BetaGroup represents a TestFlight beta group.
type BetaGroup struct {
	Type       string              `json:"type"`
	ID         string              `json:"id"`
	Attributes BetaGroupAttributes `json:"attributes"`
}

// BetaGroupAttributes contains beta group attributes.
type BetaGroupAttributes struct {
	Name                             string     `json:"name,omitempty"`
	CreatedDate                      *time.Time `json:"createdDate,omitempty"`
	IsInternalGroup                  bool       `json:"isInternalGroup,omitempty"`
	HasAccessToAllBuilds             bool       `json:"hasAccessToAllBuilds,omitempty"`
	PublicLinkEnabled                bool       `json:"publicLinkEnabled,omitempty"`
	PublicLinkID                     string     `json:"publicLinkId,omitempty"`
	PublicLinkLimitEnabled           bool       `json:"publicLinkLimitEnabled,omitempty"`
	PublicLinkLimit                  int        `json:"publicLinkLimit,omitempty"`
	PublicLink                       string     `json:"publicLink,omitempty"`
	FeedbackEnabled                  bool       `json:"feedbackEnabled,omitempty"`
	IosBuildsAvailableForTesterCount int        `json:"iosBuildsAvailableForTesterCount,omitempty"`
}

// BetaTester types

// BetaTestersResponse represents a list of beta testers.
type BetaTestersResponse struct {
	Data     []BetaTester       `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// BetaTesterResponse represents a single beta tester.
type BetaTesterResponse struct {
	Data     BetaTester `json:"data"`
	Included []any      `json:"included,omitempty"`
}

// BetaTester represents a TestFlight beta tester.
type BetaTester struct {
	Type       string               `json:"type"`
	ID         string               `json:"id"`
	Attributes BetaTesterAttributes `json:"attributes"`
}

// BetaTesterAttributes contains beta tester attributes.
type BetaTesterAttributes struct {
	FirstName  string `json:"firstName,omitempty"`
	LastName   string `json:"lastName,omitempty"`
	Email      string `json:"email,omitempty"`
	InviteType string `json:"inviteType,omitempty"`
	State      string `json:"state,omitempty"`
}

// BundleID types

// BundleIDsResponse represents a list of bundle IDs.
type BundleIDsResponse struct {
	Data     []BundleID         `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// BundleIDResponse represents a single bundle ID.
type BundleIDResponse struct {
	Data     BundleID `json:"data"`
	Included []any    `json:"included,omitempty"`
}

// BundleID represents a registered bundle identifier.
type BundleID struct {
	Type       string             `json:"type"`
	ID         string             `json:"id"`
	Attributes BundleIDAttributes `json:"attributes"`
}

// BundleIDAttributes contains bundle ID attributes.
type BundleIDAttributes struct {
	Name       string `json:"name,omitempty"`
	Identifier string `json:"identifier,omitempty"`
	Platform   string `json:"platform,omitempty"`
	SeedID     string `json:"seedId,omitempty"`
}

// Device types

// DevicesResponse represents a list of devices.
type DevicesResponse struct {
	Data     []Device           `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// DeviceResponse represents a single device.
type DeviceResponse struct {
	Data     Device `json:"data"`
	Included []any  `json:"included,omitempty"`
}

// Device represents a registered device.
type Device struct {
	Type       string           `json:"type"`
	ID         string           `json:"id"`
	Attributes DeviceAttributes `json:"attributes"`
}

// DeviceAttributes contains device attributes.
type DeviceAttributes struct {
	Name        string     `json:"name,omitempty"`
	DeviceClass string     `json:"deviceClass,omitempty"`
	Model       string     `json:"model,omitempty"`
	UDID        string     `json:"udid,omitempty"`
	Platform    string     `json:"platform,omitempty"`
	Status      string     `json:"status,omitempty"`
	AddedDate   *time.Time `json:"addedDate,omitempty"`
}

// Certificate types

// CertificatesResponse represents a list of certificates.
type CertificatesResponse struct {
	Data     []Certificate      `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// CertificateResponse represents a single certificate.
type CertificateResponse struct {
	Data     Certificate `json:"data"`
	Included []any       `json:"included,omitempty"`
}

// Certificate represents a signing certificate.
type Certificate struct {
	Type       string                `json:"type"`
	ID         string                `json:"id"`
	Attributes CertificateAttributes `json:"attributes"`
}

// CertificateAttributes contains certificate attributes.
type CertificateAttributes struct {
	Name               string     `json:"name,omitempty"`
	CertificateType    string     `json:"certificateType,omitempty"`
	DisplayName        string     `json:"displayName,omitempty"`
	SerialNumber       string     `json:"serialNumber,omitempty"`
	Platform           string     `json:"platform,omitempty"`
	ExpirationDate     *time.Time `json:"expirationDate,omitempty"`
	CertificateContent string     `json:"certificateContent,omitempty"`
}

// Profile types

// ProfilesResponse represents a list of provisioning profiles.
type ProfilesResponse struct {
	Data     []Profile          `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// ProfileResponse represents a single provisioning profile.
type ProfileResponse struct {
	Data     Profile `json:"data"`
	Included []any   `json:"included,omitempty"`
}

// Profile represents a provisioning profile.
type Profile struct {
	Type       string            `json:"type"`
	ID         string            `json:"id"`
	Attributes ProfileAttributes `json:"attributes"`
}

// ProfileAttributes contains provisioning profile attributes.
type ProfileAttributes struct {
	Name           string     `json:"name,omitempty"`
	Platform       string     `json:"platform,omitempty"`
	ProfileType    string     `json:"profileType,omitempty"`
	ProfileState   string     `json:"profileState,omitempty"`
	ProfileContent string     `json:"profileContent,omitempty"`
	UUID           string     `json:"uuid,omitempty"`
	CreatedDate    *time.Time `json:"createdDate,omitempty"`
	ExpirationDate *time.Time `json:"expirationDate,omitempty"`
}

// Request types for creating/updating resources

// BetaGroupCreateRequest represents a request to create a beta group.
type BetaGroupCreateRequest struct {
	Data BetaGroupCreateData `json:"data"`
}

// BetaGroupCreateData contains the data for creating a beta group.
type BetaGroupCreateData struct {
	Type          string                       `json:"type"`
	Attributes    BetaGroupCreateAttributes    `json:"attributes"`
	Relationships BetaGroupCreateRelationships `json:"relationships"`
}

// BetaGroupCreateAttributes contains attributes for creating a beta group.
type BetaGroupCreateAttributes struct {
	Name                   string `json:"name"`
	IsInternalGroup        bool   `json:"isInternalGroup,omitempty"`
	HasAccessToAllBuilds   bool   `json:"hasAccessToAllBuilds,omitempty"`
	PublicLinkEnabled      bool   `json:"publicLinkEnabled,omitempty"`
	PublicLinkLimitEnabled bool   `json:"publicLinkLimitEnabled,omitempty"`
	PublicLinkLimit        int    `json:"publicLinkLimit,omitempty"`
	FeedbackEnabled        bool   `json:"feedbackEnabled,omitempty"`
}

// BetaGroupCreateRelationships contains relationships for creating a beta group.
type BetaGroupCreateRelationships struct {
	App RelationshipData `json:"app"`
}

// RelationshipData contains relationship data.
type RelationshipData struct {
	Data ResourceIdentifier `json:"data"`
}

// ResourceIdentifier identifies a resource by type and ID.
type ResourceIdentifier struct {
	Type string `json:"type"`
	ID   string `json:"id"`
}

// BetaTesterCreateRequest represents a request to create/invite a beta tester.
type BetaTesterCreateRequest struct {
	Data BetaTesterCreateData `json:"data"`
}

// BetaTesterCreateData contains the data for creating a beta tester.
type BetaTesterCreateData struct {
	Type          string                         `json:"type"`
	Attributes    BetaTesterCreateAttributes     `json:"attributes"`
	Relationships *BetaTesterCreateRelationships `json:"relationships,omitempty"`
}

// BetaTesterCreateAttributes contains attributes for creating a beta tester.
type BetaTesterCreateAttributes struct {
	Email     string `json:"email"`
	FirstName string `json:"firstName,omitempty"`
	LastName  string `json:"lastName,omitempty"`
}

// BetaTesterCreateRelationships contains relationships for creating a beta tester.
type BetaTesterCreateRelationships struct {
	BetaGroups *RelationshipDataList `json:"betaGroups,omitempty"`
	Builds     *RelationshipDataList `json:"builds,omitempty"`
}

// RelationshipDataList contains a list of relationship data.
type RelationshipDataList struct {
	Data []ResourceIdentifier `json:"data"`
}

// DeviceCreateRequest represents a request to register a device.
type DeviceCreateRequest struct {
	Data DeviceCreateData `json:"data"`
}

// DeviceCreateData contains the data for registering a device.
type DeviceCreateData struct {
	Type       string                 `json:"type"`
	Attributes DeviceCreateAttributes `json:"attributes"`
}

// DeviceCreateAttributes contains attributes for registering a device.
type DeviceCreateAttributes struct {
	Name     string `json:"name"`
	UDID     string `json:"udid"`
	Platform string `json:"platform"`
}

// AppInfo types

// AppInfosResponse represents a list of app infos.
type AppInfosResponse struct {
	Data     []AppInfo          `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// AppInfoResponse represents a single app info.
type AppInfoResponse struct {
	Data     AppInfo `json:"data"`
	Included []any   `json:"included,omitempty"`
}

// AppInfo represents app information.
type AppInfo struct {
	Type       string            `json:"type"`
	ID         string            `json:"id"`
	Attributes AppInfoAttributes `json:"attributes"`
}

// AppInfoAttributes contains app info attributes.
type AppInfoAttributes struct {
	AppStoreState     string `json:"appStoreState,omitempty"`
	AppStoreAgeRating string `json:"appStoreAgeRating,omitempty"`
	BrazilAgeRating   string `json:"brazilAgeRating,omitempty"`
	KidsAgeBand       string `json:"kidsAgeBand,omitempty"`
	BrazilAgeRatingV2 string `json:"brazilAgeRatingV2,omitempty"`
	State             string `json:"state,omitempty"`
	PrimaryCategory   string `json:"primaryCategory,omitempty"`
	SecondaryCategory string `json:"secondaryCategory,omitempty"`
}

// AppInfoLocalization types

// AppInfoLocalizationsResponse represents a list of app info localizations.
type AppInfoLocalizationsResponse struct {
	Data     []AppInfoLocalization `json:"data"`
	Links    PagedDocumentLinks    `json:"links"`
	Meta     *PagingInformation    `json:"meta,omitempty"`
	Included []any                 `json:"included,omitempty"`
}

// AppInfoLocalizationResponse represents a single app info localization.
type AppInfoLocalizationResponse struct {
	Data     AppInfoLocalization `json:"data"`
	Included []any               `json:"included,omitempty"`
}

// AppInfoLocalization represents localized app information.
type AppInfoLocalization struct {
	Type       string                        `json:"type"`
	ID         string                        `json:"id"`
	Attributes AppInfoLocalizationAttributes `json:"attributes"`
}

// AppInfoLocalizationAttributes contains app info localization attributes.
type AppInfoLocalizationAttributes struct {
	Locale            string `json:"locale,omitempty"`
	Name              string `json:"name,omitempty"`
	Subtitle          string `json:"subtitle,omitempty"`
	PrivacyPolicyURL  string `json:"privacyPolicyUrl,omitempty"`
	PrivacyChoicesURL string `json:"privacyChoicesUrl,omitempty"`
	PrivacyPolicyText string `json:"privacyPolicyText,omitempty"`
}

// AppInfoLocalizationCreateRequest represents a request to create an app info localization.
type AppInfoLocalizationCreateRequest struct {
	Data AppInfoLocalizationCreateData `json:"data"`
}

// AppInfoLocalizationCreateData contains the data for creating an app info localization.
type AppInfoLocalizationCreateData struct {
	Type          string                                 `json:"type"`
	Attributes    AppInfoLocalizationCreateAttributes    `json:"attributes"`
	Relationships AppInfoLocalizationCreateRelationships `json:"relationships"`
}

// AppInfoLocalizationCreateAttributes contains attributes for creating an app info localization.
type AppInfoLocalizationCreateAttributes struct {
	Locale            string `json:"locale"`
	Name              string `json:"name"`
	Subtitle          string `json:"subtitle,omitempty"`
	PrivacyPolicyURL  string `json:"privacyPolicyUrl,omitempty"`
	PrivacyChoicesURL string `json:"privacyChoicesUrl,omitempty"`
	PrivacyPolicyText string `json:"privacyPolicyText,omitempty"`
}

// AppInfoLocalizationCreateRelationships contains relationships for creating an app info localization.
type AppInfoLocalizationCreateRelationships struct {
	AppInfo RelationshipData `json:"appInfo"`
}

// AppInfoLocalizationUpdateRequest represents a request to update an app info localization.
type AppInfoLocalizationUpdateRequest struct {
	Data AppInfoLocalizationUpdateData `json:"data"`
}

// AppInfoLocalizationUpdateData contains the data for updating an app info localization.
type AppInfoLocalizationUpdateData struct {
	Type       string                              `json:"type"`
	ID         string                              `json:"id"`
	Attributes AppInfoLocalizationUpdateAttributes `json:"attributes"`
}

// AppInfoLocalizationUpdateAttributes contains attributes for updating an app info localization.
type AppInfoLocalizationUpdateAttributes struct {
	Name              string `json:"name,omitempty"`
	Subtitle          string `json:"subtitle,omitempty"`
	PrivacyPolicyURL  string `json:"privacyPolicyUrl,omitempty"`
	PrivacyChoicesURL string `json:"privacyChoicesUrl,omitempty"`
	PrivacyPolicyText string `json:"privacyPolicyText,omitempty"`
}

// AppStoreVersionLocalization types

// AppStoreVersionLocalizationsResponse represents a list of version localizations.
type AppStoreVersionLocalizationsResponse struct {
	Data     []AppStoreVersionLocalization `json:"data"`
	Links    PagedDocumentLinks            `json:"links"`
	Meta     *PagingInformation            `json:"meta,omitempty"`
	Included []any                         `json:"included,omitempty"`
}

// AppStoreVersionLocalizationResponse represents a single version localization.
type AppStoreVersionLocalizationResponse struct {
	Data     AppStoreVersionLocalization `json:"data"`
	Included []any                       `json:"included,omitempty"`
}

// AppStoreVersionLocalization represents a localized app store version.
type AppStoreVersionLocalization struct {
	Type       string                                `json:"type"`
	ID         string                                `json:"id"`
	Attributes AppStoreVersionLocalizationAttributes `json:"attributes"`
}

// AppStoreVersionLocalizationAttributes contains version localization attributes.
type AppStoreVersionLocalizationAttributes struct {
	Locale          string `json:"locale,omitempty"`
	Description     string `json:"description,omitempty"`
	Keywords        string `json:"keywords,omitempty"`
	WhatsNew        string `json:"whatsNew,omitempty"`
	PromotionalText string `json:"promotionalText,omitempty"`
	MarketingURL    string `json:"marketingUrl,omitempty"`
	SupportURL      string `json:"supportUrl,omitempty"`
}

// AppStoreVersionLocalizationCreateRequest represents a request to create a version localization.
type AppStoreVersionLocalizationCreateRequest struct {
	Data AppStoreVersionLocalizationCreateData `json:"data"`
}

// AppStoreVersionLocalizationCreateData contains the data for creating a version localization.
type AppStoreVersionLocalizationCreateData struct {
	Type          string                                         `json:"type"`
	Attributes    AppStoreVersionLocalizationCreateAttributes    `json:"attributes"`
	Relationships AppStoreVersionLocalizationCreateRelationships `json:"relationships"`
}

// AppStoreVersionLocalizationCreateAttributes contains attributes for creating a version localization.
type AppStoreVersionLocalizationCreateAttributes struct {
	Locale          string `json:"locale"`
	Description     string `json:"description,omitempty"`
	Keywords        string `json:"keywords,omitempty"`
	WhatsNew        string `json:"whatsNew,omitempty"`
	PromotionalText string `json:"promotionalText,omitempty"`
	MarketingURL    string `json:"marketingUrl,omitempty"`
	SupportURL      string `json:"supportUrl,omitempty"`
}

// AppStoreVersionLocalizationCreateRelationships contains relationships for creating a version localization.
type AppStoreVersionLocalizationCreateRelationships struct {
	AppStoreVersion RelationshipData `json:"appStoreVersion"`
}

// AppStoreVersionLocalizationUpdateRequest represents a request to update a version localization.
type AppStoreVersionLocalizationUpdateRequest struct {
	Data AppStoreVersionLocalizationUpdateData `json:"data"`
}

// AppStoreVersionLocalizationUpdateData contains the data for updating a version localization.
type AppStoreVersionLocalizationUpdateData struct {
	Type       string                                      `json:"type"`
	ID         string                                      `json:"id"`
	Attributes AppStoreVersionLocalizationUpdateAttributes `json:"attributes"`
}

// AppStoreVersionLocalizationUpdateAttributes contains attributes for updating a version localization.
type AppStoreVersionLocalizationUpdateAttributes struct {
	Description     string `json:"description,omitempty"`
	Keywords        string `json:"keywords,omitempty"`
	WhatsNew        string `json:"whatsNew,omitempty"`
	PromotionalText string `json:"promotionalText,omitempty"`
	MarketingURL    string `json:"marketingUrl,omitempty"`
	SupportURL      string `json:"supportUrl,omitempty"`
}

// Customer Review types

// CustomerReviewsResponse represents a list of customer reviews.
type CustomerReviewsResponse struct {
	Data     []CustomerReview   `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// CustomerReviewResponse represents a single customer review.
type CustomerReviewResponse struct {
	Data     CustomerReview `json:"data"`
	Included []any          `json:"included,omitempty"`
}

// CustomerReview represents a customer review.
type CustomerReview struct {
	Type       string                   `json:"type"`
	ID         string                   `json:"id"`
	Attributes CustomerReviewAttributes `json:"attributes"`
}

// CustomerReviewAttributes contains customer review attributes.
type CustomerReviewAttributes struct {
	Rating       int        `json:"rating,omitempty"`
	Title        string     `json:"title,omitempty"`
	Body         string     `json:"body,omitempty"`
	ReviewerName string     `json:"reviewerNickname,omitempty"`
	CreatedDate  *time.Time `json:"createdDate,omitempty"`
	Territory    string     `json:"territory,omitempty"`
}

// CustomerReviewResponseV1 represents a response to a customer review.
type CustomerReviewResponseV1 struct {
	Type       string                             `json:"type"`
	ID         string                             `json:"id"`
	Attributes CustomerReviewResponseV1Attributes `json:"attributes"`
}

// CustomerReviewResponseV1Attributes contains review response attributes.
type CustomerReviewResponseV1Attributes struct {
	ResponseBody string     `json:"responseBody,omitempty"`
	LastModified *time.Time `json:"lastModifiedDate,omitempty"`
	State        string     `json:"state,omitempty"`
}

// CustomerReviewResponseV1Response represents a single review response.
type CustomerReviewResponseV1Response struct {
	Data     CustomerReviewResponseV1 `json:"data"`
	Included []any                    `json:"included,omitempty"`
}

// CustomerReviewResponseCreateRequest represents a request to create a review response.
type CustomerReviewResponseCreateRequest struct {
	Data CustomerReviewResponseCreateData `json:"data"`
}

// CustomerReviewResponseCreateData contains the data for creating a review response.
type CustomerReviewResponseCreateData struct {
	Type          string                                    `json:"type"`
	Attributes    CustomerReviewResponseCreateAttributes    `json:"attributes"`
	Relationships CustomerReviewResponseCreateRelationships `json:"relationships"`
}

// CustomerReviewResponseCreateAttributes contains attributes for creating a review response.
type CustomerReviewResponseCreateAttributes struct {
	ResponseBody string `json:"responseBody"`
}

// CustomerReviewResponseCreateRelationships contains relationships for creating a review response.
type CustomerReviewResponseCreateRelationships struct {
	Review RelationshipData `json:"review"`
}

// In-App Purchase types

// InAppPurchasesResponse represents a list of in-app purchases.
type InAppPurchasesResponse struct {
	Data     []InAppPurchase    `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// InAppPurchaseResponse represents a single in-app purchase.
type InAppPurchaseResponse struct {
	Data     InAppPurchase `json:"data"`
	Included []any         `json:"included,omitempty"`
}

// InAppPurchase represents an in-app purchase.
type InAppPurchase struct {
	Type       string                  `json:"type"`
	ID         string                  `json:"id"`
	Attributes InAppPurchaseAttributes `json:"attributes"`
}

// InAppPurchaseAttributes contains in-app purchase attributes.
type InAppPurchaseAttributes struct {
	Name                      string `json:"name,omitempty"`
	ProductID                 string `json:"productId,omitempty"`
	InAppPurchaseType         string `json:"inAppPurchaseType,omitempty"`
	State                     string `json:"state,omitempty"`
	ReviewNote                string `json:"reviewNote,omitempty"`
	FamilySharable            bool   `json:"familySharable,omitempty"`
	ContentHosting            bool   `json:"contentHosting,omitempty"`
	AvailableInAllTerritories bool   `json:"availableInAllTerritories,omitempty"`
}

// InAppPurchaseCreateRequest represents a request to create an in-app purchase.
type InAppPurchaseCreateRequest struct {
	Data InAppPurchaseCreateData `json:"data"`
}

// InAppPurchaseCreateData contains the data for creating an in-app purchase.
type InAppPurchaseCreateData struct {
	Type          string                           `json:"type"`
	Attributes    InAppPurchaseCreateAttributes    `json:"attributes"`
	Relationships InAppPurchaseCreateRelationships `json:"relationships"`
}

// InAppPurchaseCreateAttributes contains attributes for creating an in-app purchase.
type InAppPurchaseCreateAttributes struct {
	Name                      string `json:"name"`
	ProductID                 string `json:"productId"`
	InAppPurchaseType         string `json:"inAppPurchaseType"`
	ReviewNote                string `json:"reviewNote,omitempty"`
	FamilySharable            bool   `json:"familySharable,omitempty"`
	AvailableInAllTerritories bool   `json:"availableInAllTerritories,omitempty"`
}

// InAppPurchaseCreateRelationships contains relationships for creating an in-app purchase.
type InAppPurchaseCreateRelationships struct {
	App RelationshipData `json:"app"`
}

// InAppPurchaseUpdateRequest represents a request to update an in-app purchase.
type InAppPurchaseUpdateRequest struct {
	Data InAppPurchaseUpdateData `json:"data"`
}

// InAppPurchaseUpdateData contains the data for updating an in-app purchase.
type InAppPurchaseUpdateData struct {
	Type       string                        `json:"type"`
	ID         string                        `json:"id"`
	Attributes InAppPurchaseUpdateAttributes `json:"attributes"`
}

// InAppPurchaseUpdateAttributes contains attributes for updating an in-app purchase.
type InAppPurchaseUpdateAttributes struct {
	Name                      string `json:"name,omitempty"`
	ReviewNote                string `json:"reviewNote,omitempty"`
	FamilySharable            *bool  `json:"familySharable,omitempty"`
	AvailableInAllTerritories *bool  `json:"availableInAllTerritories,omitempty"`
}

// Subscription types

// SubscriptionsResponse represents a list of subscriptions.
type SubscriptionsResponse struct {
	Data     []Subscription     `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// SubscriptionResponse represents a single subscription.
type SubscriptionResponse struct {
	Data     Subscription `json:"data"`
	Included []any        `json:"included,omitempty"`
}

// Subscription represents an auto-renewable subscription.
type Subscription struct {
	Type       string                 `json:"type"`
	ID         string                 `json:"id"`
	Attributes SubscriptionAttributes `json:"attributes"`
}

// SubscriptionAttributes contains subscription attributes.
type SubscriptionAttributes struct {
	Name                      string `json:"name,omitempty"`
	ProductID                 string `json:"productId,omitempty"`
	FamilySharable            bool   `json:"familySharable,omitempty"`
	State                     string `json:"state,omitempty"`
	SubscriptionPeriod        string `json:"subscriptionPeriod,omitempty"`
	ReviewNote                string `json:"reviewNote,omitempty"`
	GroupLevel                int    `json:"groupLevel,omitempty"`
	AvailableInAllTerritories bool   `json:"availableInAllTerritories,omitempty"`
}

// SubscriptionGroupsResponse represents a list of subscription groups.
type SubscriptionGroupsResponse struct {
	Data     []SubscriptionGroup `json:"data"`
	Links    PagedDocumentLinks  `json:"links"`
	Meta     *PagingInformation  `json:"meta,omitempty"`
	Included []any               `json:"included,omitempty"`
}

// SubscriptionGroupResponse represents a single subscription group.
type SubscriptionGroupResponse struct {
	Data     SubscriptionGroup `json:"data"`
	Included []any             `json:"included,omitempty"`
}

// SubscriptionGroup represents a subscription group.
type SubscriptionGroup struct {
	Type       string                      `json:"type"`
	ID         string                      `json:"id"`
	Attributes SubscriptionGroupAttributes `json:"attributes"`
}

// SubscriptionGroupAttributes contains subscription group attributes.
type SubscriptionGroupAttributes struct {
	ReferenceName string `json:"referenceName,omitempty"`
}

// App Store Version Submission types

// AppStoreVersionSubmissionResponse represents a version submission response.
type AppStoreVersionSubmissionResponse struct {
	Data     AppStoreVersionSubmission `json:"data"`
	Included []any                     `json:"included,omitempty"`
}

// AppStoreVersionSubmission represents a version submission.
type AppStoreVersionSubmission struct {
	Type string `json:"type"`
	ID   string `json:"id"`
}

// AppStoreVersionSubmissionCreateRequest represents a request to submit a version.
type AppStoreVersionSubmissionCreateRequest struct {
	Data AppStoreVersionSubmissionCreateData `json:"data"`
}

// AppStoreVersionSubmissionCreateData contains the data for creating a submission.
type AppStoreVersionSubmissionCreateData struct {
	Type          string                                       `json:"type"`
	Relationships AppStoreVersionSubmissionCreateRelationships `json:"relationships"`
}

// AppStoreVersionSubmissionCreateRelationships contains relationships for creating a submission.
type AppStoreVersionSubmissionCreateRelationships struct {
	AppStoreVersion RelationshipData `json:"appStoreVersion"`
}

// AppStoreVersionCreateRequest represents a request to create a version.
type AppStoreVersionCreateRequest struct {
	Data AppStoreVersionCreateData `json:"data"`
}

// AppStoreVersionCreateData contains the data for creating a version.
type AppStoreVersionCreateData struct {
	Type          string                             `json:"type"`
	Attributes    AppStoreVersionCreateAttributes    `json:"attributes"`
	Relationships AppStoreVersionCreateRelationships `json:"relationships"`
}

// AppStoreVersionCreateAttributes contains attributes for creating a version.
type AppStoreVersionCreateAttributes struct {
	Platform            string     `json:"platform"`
	VersionString       string     `json:"versionString"`
	Copyright           string     `json:"copyright,omitempty"`
	ReleaseType         string     `json:"releaseType,omitempty"`
	EarliestReleaseDate *time.Time `json:"earliestReleaseDate,omitempty"`
}

// AppStoreVersionCreateRelationships contains relationships for creating a version.
type AppStoreVersionCreateRelationships struct {
	App   RelationshipData  `json:"app"`
	Build *RelationshipData `json:"build,omitempty"`
}

// AppStoreVersionUpdateRequest represents a request to update a version.
type AppStoreVersionUpdateRequest struct {
	Data AppStoreVersionUpdateData `json:"data"`
}

// AppStoreVersionUpdateData contains the data for updating a version.
type AppStoreVersionUpdateData struct {
	Type       string                          `json:"type"`
	ID         string                          `json:"id"`
	Attributes AppStoreVersionUpdateAttributes `json:"attributes"`
}

// AppStoreVersionUpdateAttributes contains attributes for updating a version.
type AppStoreVersionUpdateAttributes struct {
	VersionString       string     `json:"versionString,omitempty"`
	Copyright           string     `json:"copyright,omitempty"`
	ReleaseType         string     `json:"releaseType,omitempty"`
	EarliestReleaseDate *time.Time `json:"earliestReleaseDate,omitempty"`
	Downloadable        *bool      `json:"downloadable,omitempty"`
}

// App Store Review Detail types

// AppStoreReviewDetailResponse represents app store review detail.
type AppStoreReviewDetailResponse struct {
	Data     AppStoreReviewDetail `json:"data"`
	Included []any                `json:"included,omitempty"`
}

// AppStoreReviewDetail represents review details for submission.
type AppStoreReviewDetail struct {
	Type       string                         `json:"type"`
	ID         string                         `json:"id"`
	Attributes AppStoreReviewDetailAttributes `json:"attributes"`
}

// AppStoreReviewDetailAttributes contains review detail attributes.
type AppStoreReviewDetailAttributes struct {
	ContactFirstName    string `json:"contactFirstName,omitempty"`
	ContactLastName     string `json:"contactLastName,omitempty"`
	ContactPhone        string `json:"contactPhone,omitempty"`
	ContactEmail        string `json:"contactEmail,omitempty"`
	DemoAccountName     string `json:"demoAccountName,omitempty"`
	DemoAccountPassword string `json:"demoAccountPassword,omitempty"`
	DemoAccountRequired bool   `json:"demoAccountRequired,omitempty"`
	Notes               string `json:"notes,omitempty"`
}

// AppStoreReviewDetailCreateRequest represents a request to create review details.
type AppStoreReviewDetailCreateRequest struct {
	Data AppStoreReviewDetailCreateData `json:"data"`
}

// AppStoreReviewDetailCreateData contains the data for creating review details.
type AppStoreReviewDetailCreateData struct {
	Type          string                                  `json:"type"`
	Attributes    AppStoreReviewDetailCreateAttributes    `json:"attributes"`
	Relationships AppStoreReviewDetailCreateRelationships `json:"relationships"`
}

// AppStoreReviewDetailCreateAttributes contains attributes for creating review details.
type AppStoreReviewDetailCreateAttributes struct {
	ContactFirstName    string `json:"contactFirstName,omitempty"`
	ContactLastName     string `json:"contactLastName,omitempty"`
	ContactPhone        string `json:"contactPhone,omitempty"`
	ContactEmail        string `json:"contactEmail,omitempty"`
	DemoAccountName     string `json:"demoAccountName,omitempty"`
	DemoAccountPassword string `json:"demoAccountPassword,omitempty"`
	DemoAccountRequired *bool  `json:"demoAccountRequired,omitempty"`
	Notes               string `json:"notes,omitempty"`
}

// AppStoreReviewDetailCreateRelationships contains relationships for creating review details.
type AppStoreReviewDetailCreateRelationships struct {
	AppStoreVersion RelationshipData `json:"appStoreVersion"`
}

// AppStoreReviewDetailUpdateRequest represents a request to update review details.
type AppStoreReviewDetailUpdateRequest struct {
	Data AppStoreReviewDetailUpdateData `json:"data"`
}

// AppStoreReviewDetailUpdateData contains the data for updating review details.
type AppStoreReviewDetailUpdateData struct {
	Type       string                               `json:"type"`
	ID         string                               `json:"id"`
	Attributes AppStoreReviewDetailUpdateAttributes `json:"attributes"`
}

// AppStoreReviewDetailUpdateAttributes contains attributes for updating review details.
type AppStoreReviewDetailUpdateAttributes struct {
	ContactFirstName    string `json:"contactFirstName,omitempty"`
	ContactLastName     string `json:"contactLastName,omitempty"`
	ContactPhone        string `json:"contactPhone,omitempty"`
	ContactEmail        string `json:"contactEmail,omitempty"`
	DemoAccountName     string `json:"demoAccountName,omitempty"`
	DemoAccountPassword string `json:"demoAccountPassword,omitempty"`
	DemoAccountRequired *bool  `json:"demoAccountRequired,omitempty"`
	Notes               string `json:"notes,omitempty"`
}

// Phased Release types

// AppStoreVersionPhasedReleaseResponse represents a phased release response.
type AppStoreVersionPhasedReleaseResponse struct {
	Data     AppStoreVersionPhasedRelease `json:"data"`
	Included []any                        `json:"included,omitempty"`
}

// AppStoreVersionPhasedRelease represents a phased release.
type AppStoreVersionPhasedRelease struct {
	Type       string                                 `json:"type"`
	ID         string                                 `json:"id"`
	Attributes AppStoreVersionPhasedReleaseAttributes `json:"attributes"`
}

// AppStoreVersionPhasedReleaseAttributes contains phased release attributes.
type AppStoreVersionPhasedReleaseAttributes struct {
	PhasedReleaseState string     `json:"phasedReleaseState,omitempty"`
	StartDate          *time.Time `json:"startDate,omitempty"`
	TotalPauseDuration int        `json:"totalPauseDuration,omitempty"`
	CurrentDayNumber   int        `json:"currentDayNumber,omitempty"`
}

// AppStoreVersionPhasedReleaseCreateRequest represents a request to create a phased release.
type AppStoreVersionPhasedReleaseCreateRequest struct {
	Data AppStoreVersionPhasedReleaseCreateData `json:"data"`
}

// AppStoreVersionPhasedReleaseCreateData contains the data for creating a phased release.
type AppStoreVersionPhasedReleaseCreateData struct {
	Type          string                                          `json:"type"`
	Attributes    AppStoreVersionPhasedReleaseCreateAttributes    `json:"attributes"`
	Relationships AppStoreVersionPhasedReleaseCreateRelationships `json:"relationships"`
}

// AppStoreVersionPhasedReleaseCreateAttributes contains attributes for creating a phased release.
type AppStoreVersionPhasedReleaseCreateAttributes struct {
	PhasedReleaseState string `json:"phasedReleaseState,omitempty"`
}

// AppStoreVersionPhasedReleaseCreateRelationships contains relationships for creating a phased release.
type AppStoreVersionPhasedReleaseCreateRelationships struct {
	AppStoreVersion RelationshipData `json:"appStoreVersion"`
}

// AppStoreVersionPhasedReleaseUpdateRequest represents a request to update a phased release.
type AppStoreVersionPhasedReleaseUpdateRequest struct {
	Data AppStoreVersionPhasedReleaseUpdateData `json:"data"`
}

// AppStoreVersionPhasedReleaseUpdateData contains the data for updating a phased release.
type AppStoreVersionPhasedReleaseUpdateData struct {
	Type       string                                       `json:"type"`
	ID         string                                       `json:"id"`
	Attributes AppStoreVersionPhasedReleaseUpdateAttributes `json:"attributes"`
}

// AppStoreVersionPhasedReleaseUpdateAttributes contains attributes for updating a phased release.
type AppStoreVersionPhasedReleaseUpdateAttributes struct {
	PhasedReleaseState string `json:"phasedReleaseState,omitempty"`
}

// App Screenshot types

// AppScreenshotSetsResponse represents a list of screenshot sets.
type AppScreenshotSetsResponse struct {
	Data     []AppScreenshotSet `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// AppScreenshotSetResponse represents a single screenshot set.
type AppScreenshotSetResponse struct {
	Data     AppScreenshotSet `json:"data"`
	Included []any            `json:"included,omitempty"`
}

// AppScreenshotSet represents a screenshot set.
type AppScreenshotSet struct {
	Type       string                     `json:"type"`
	ID         string                     `json:"id"`
	Attributes AppScreenshotSetAttributes `json:"attributes"`
}

// AppScreenshotSetAttributes contains screenshot set attributes.
type AppScreenshotSetAttributes struct {
	ScreenshotDisplayType string `json:"screenshotDisplayType,omitempty"`
}

// AppScreenshotsResponse represents a list of screenshots.
type AppScreenshotsResponse struct {
	Data     []AppScreenshot    `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// AppScreenshotResponse represents a single screenshot.
type AppScreenshotResponse struct {
	Data     AppScreenshot `json:"data"`
	Included []any         `json:"included,omitempty"`
}

// AppScreenshot represents an app screenshot.
type AppScreenshot struct {
	Type       string                  `json:"type"`
	ID         string                  `json:"id"`
	Attributes AppScreenshotAttributes `json:"attributes"`
}

// AppScreenshotAttributes contains screenshot attributes.
type AppScreenshotAttributes struct {
	FileSize           int                 `json:"fileSize,omitempty"`
	FileName           string              `json:"fileName,omitempty"`
	SourceFileChecksum string              `json:"sourceFileChecksum,omitempty"`
	ImageAsset         *ImageAsset         `json:"imageAsset,omitempty"`
	AssetToken         string              `json:"assetToken,omitempty"`
	AssetType          string              `json:"assetType,omitempty"`
	UploadOperations   []UploadOperation   `json:"uploadOperations,omitempty"`
	AssetDeliveryState *AssetDeliveryState `json:"assetDeliveryState,omitempty"`
}

// ImageAsset represents an image asset.
type ImageAsset struct {
	TemplateURL string `json:"templateUrl,omitempty"`
	Width       int    `json:"width,omitempty"`
	Height      int    `json:"height,omitempty"`
}

// UploadOperation represents an upload operation.
type UploadOperation struct {
	Method         string          `json:"method,omitempty"`
	URL            string          `json:"url,omitempty"`
	Length         int             `json:"length,omitempty"`
	Offset         int             `json:"offset,omitempty"`
	RequestHeaders []RequestHeader `json:"requestHeaders,omitempty"`
}

// RequestHeader represents a request header.
type RequestHeader struct {
	Name  string `json:"name,omitempty"`
	Value string `json:"value,omitempty"`
}

// AssetDeliveryState represents asset delivery state.
type AssetDeliveryState struct {
	Errors   []APIError `json:"errors,omitempty"`
	Warnings []APIError `json:"warnings,omitempty"`
	State    string     `json:"state,omitempty"`
}

// AppScreenshotCreateRequest represents a request to create a screenshot.
type AppScreenshotCreateRequest struct {
	Data AppScreenshotCreateData `json:"data"`
}

// AppScreenshotCreateData contains the data for creating a screenshot.
type AppScreenshotCreateData struct {
	Type          string                           `json:"type"`
	Attributes    AppScreenshotCreateAttributes    `json:"attributes"`
	Relationships AppScreenshotCreateRelationships `json:"relationships"`
}

// AppScreenshotCreateAttributes contains attributes for creating a screenshot.
type AppScreenshotCreateAttributes struct {
	FileSize int    `json:"fileSize"`
	FileName string `json:"fileName"`
}

// AppScreenshotCreateRelationships contains relationships for creating a screenshot.
type AppScreenshotCreateRelationships struct {
	AppScreenshotSet RelationshipData `json:"appScreenshotSet"`
}

// AppScreenshotUpdateRequest represents a request to update a screenshot.
type AppScreenshotUpdateRequest struct {
	Data AppScreenshotUpdateData `json:"data"`
}

// AppScreenshotUpdateData contains the data for updating a screenshot.
type AppScreenshotUpdateData struct {
	Type       string                        `json:"type"`
	ID         string                        `json:"id"`
	Attributes AppScreenshotUpdateAttributes `json:"attributes"`
}

// AppScreenshotUpdateAttributes contains attributes for updating a screenshot.
type AppScreenshotUpdateAttributes struct {
	SourceFileChecksum string `json:"sourceFileChecksum,omitempty"`
	Uploaded           *bool  `json:"uploaded,omitempty"`
}

// App Preview types

// AppPreviewSetsResponse represents a list of preview sets.
type AppPreviewSetsResponse struct {
	Data     []AppPreviewSet    `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// AppPreviewSetResponse represents a single preview set.
type AppPreviewSetResponse struct {
	Data     AppPreviewSet `json:"data"`
	Included []any         `json:"included,omitempty"`
}

// AppPreviewSet represents a preview set.
type AppPreviewSet struct {
	Type       string                  `json:"type"`
	ID         string                  `json:"id"`
	Attributes AppPreviewSetAttributes `json:"attributes"`
}

// AppPreviewSetAttributes contains preview set attributes.
type AppPreviewSetAttributes struct {
	PreviewType string `json:"previewType,omitempty"`
}

// AppPreviewsResponse represents a list of previews.
type AppPreviewsResponse struct {
	Data     []AppPreview       `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// AppPreviewResponse represents a single preview.
type AppPreviewResponse struct {
	Data     AppPreview `json:"data"`
	Included []any      `json:"included,omitempty"`
}

// AppPreview represents an app preview.
type AppPreview struct {
	Type       string               `json:"type"`
	ID         string               `json:"id"`
	Attributes AppPreviewAttributes `json:"attributes"`
}

// AppPreviewAttributes contains preview attributes.
type AppPreviewAttributes struct {
	FileSize             int                 `json:"fileSize,omitempty"`
	FileName             string              `json:"fileName,omitempty"`
	SourceFileChecksum   string              `json:"sourceFileChecksum,omitempty"`
	PreviewFrameTimeCode string              `json:"previewFrameTimeCode,omitempty"`
	MimeType             string              `json:"mimeType,omitempty"`
	VideoURL             string              `json:"videoUrl,omitempty"`
	PreviewImage         *ImageAsset         `json:"previewImage,omitempty"`
	UploadOperations     []UploadOperation   `json:"uploadOperations,omitempty"`
	AssetDeliveryState   *AssetDeliveryState `json:"assetDeliveryState,omitempty"`
}

// AppPreviewCreateRequest represents a request to create a preview.
type AppPreviewCreateRequest struct {
	Data AppPreviewCreateData `json:"data"`
}

// AppPreviewCreateData contains the data for creating a preview.
type AppPreviewCreateData struct {
	Type          string                        `json:"type"`
	Attributes    AppPreviewCreateAttributes    `json:"attributes"`
	Relationships AppPreviewCreateRelationships `json:"relationships"`
}

// AppPreviewCreateAttributes contains attributes for creating a preview.
type AppPreviewCreateAttributes struct {
	FileSize             int    `json:"fileSize"`
	FileName             string `json:"fileName"`
	PreviewFrameTimeCode string `json:"previewFrameTimeCode,omitempty"`
	MimeType             string `json:"mimeType,omitempty"`
}

// AppPreviewCreateRelationships contains relationships for creating a preview.
type AppPreviewCreateRelationships struct {
	AppPreviewSet RelationshipData `json:"appPreviewSet"`
}

// App Pre-Order types

// AppPreOrderResponse represents a pre-order response.
type AppPreOrderResponse struct {
	Data     AppPreOrder `json:"data"`
	Included []any       `json:"included,omitempty"`
}

// AppPreOrder represents an app pre-order.
type AppPreOrder struct {
	Type       string                `json:"type"`
	ID         string                `json:"id"`
	Attributes AppPreOrderAttributes `json:"attributes"`
}

// AppPreOrderAttributes contains pre-order attributes.
type AppPreOrderAttributes struct {
	PreOrderAvailableDate string `json:"preOrderAvailableDate,omitempty"`
	AppReleaseDate        string `json:"appReleaseDate,omitempty"`
}

// AppPreOrderCreateRequest represents a request to create a pre-order.
type AppPreOrderCreateRequest struct {
	Data AppPreOrderCreateData `json:"data"`
}

// AppPreOrderCreateData contains the data for creating a pre-order.
type AppPreOrderCreateData struct {
	Type          string                         `json:"type"`
	Attributes    AppPreOrderCreateAttributes    `json:"attributes"`
	Relationships AppPreOrderCreateRelationships `json:"relationships"`
}

// AppPreOrderCreateAttributes contains attributes for creating a pre-order.
type AppPreOrderCreateAttributes struct {
	AppReleaseDate string `json:"appReleaseDate,omitempty"`
}

// AppPreOrderCreateRelationships contains relationships for creating a pre-order.
type AppPreOrderCreateRelationships struct {
	App RelationshipData `json:"app"`
}

// AppPreOrderUpdateRequest represents a request to update a pre-order.
type AppPreOrderUpdateRequest struct {
	Data AppPreOrderUpdateData `json:"data"`
}

// AppPreOrderUpdateData contains the data for updating a pre-order.
type AppPreOrderUpdateData struct {
	Type       string                      `json:"type"`
	ID         string                      `json:"id"`
	Attributes AppPreOrderUpdateAttributes `json:"attributes"`
}

// AppPreOrderUpdateAttributes contains attributes for updating a pre-order.
type AppPreOrderUpdateAttributes struct {
	AppReleaseDate string `json:"appReleaseDate,omitempty"`
}

// App Event types

// AppEventsResponse represents a list of app events.
type AppEventsResponse struct {
	Data     []AppEvent         `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// AppEventResponse represents a single app event.
type AppEventResponse struct {
	Data     AppEvent `json:"data"`
	Included []any    `json:"included,omitempty"`
}

// AppEvent represents an app event.
type AppEvent struct {
	Type       string             `json:"type"`
	ID         string             `json:"id"`
	Attributes AppEventAttributes `json:"attributes"`
}

// AppEventAttributes contains app event attributes.
type AppEventAttributes struct {
	ReferenceName              string              `json:"referenceName,omitempty"`
	Badge                      string              `json:"badge,omitempty"`
	EventState                 string              `json:"eventState,omitempty"`
	DeepLink                   string              `json:"deepLink,omitempty"`
	PurchaseRequirement        string              `json:"purchaseRequirement,omitempty"`
	PrimaryLocale              string              `json:"primaryLocale,omitempty"`
	Priority                   string              `json:"priority,omitempty"`
	Purpose                    string              `json:"purpose,omitempty"`
	TerritorySchedules         []TerritorySchedule `json:"territorySchedules,omitempty"`
	ArchivedTerritorySchedules []TerritorySchedule `json:"archivedTerritorySchedules,omitempty"`
}

// TerritorySchedule represents a territory schedule for an event.
type TerritorySchedule struct {
	Territories  []string   `json:"territories,omitempty"`
	PublishStart *time.Time `json:"publishStart,omitempty"`
	EventStart   *time.Time `json:"eventStart,omitempty"`
	EventEnd     *time.Time `json:"eventEnd,omitempty"`
}

// AppEventCreateRequest represents a request to create an app event.
type AppEventCreateRequest struct {
	Data AppEventCreateData `json:"data"`
}

// AppEventCreateData contains the data for creating an app event.
type AppEventCreateData struct {
	Type          string                      `json:"type"`
	Attributes    AppEventCreateAttributes    `json:"attributes"`
	Relationships AppEventCreateRelationships `json:"relationships"`
}

// AppEventCreateAttributes contains attributes for creating an app event.
type AppEventCreateAttributes struct {
	ReferenceName       string              `json:"referenceName"`
	Badge               string              `json:"badge,omitempty"`
	DeepLink            string              `json:"deepLink,omitempty"`
	PurchaseRequirement string              `json:"purchaseRequirement,omitempty"`
	PrimaryLocale       string              `json:"primaryLocale,omitempty"`
	Priority            string              `json:"priority,omitempty"`
	Purpose             string              `json:"purpose,omitempty"`
	TerritorySchedules  []TerritorySchedule `json:"territorySchedules,omitempty"`
}

// AppEventCreateRelationships contains relationships for creating an app event.
type AppEventCreateRelationships struct {
	App RelationshipData `json:"app"`
}

// AppEventUpdateRequest represents a request to update an app event.
type AppEventUpdateRequest struct {
	Data AppEventUpdateData `json:"data"`
}

// AppEventUpdateData contains the data for updating an app event.
type AppEventUpdateData struct {
	Type       string                   `json:"type"`
	ID         string                   `json:"id"`
	Attributes AppEventUpdateAttributes `json:"attributes"`
}

// AppEventUpdateAttributes contains attributes for updating an app event.
type AppEventUpdateAttributes struct {
	ReferenceName       string              `json:"referenceName,omitempty"`
	Badge               string              `json:"badge,omitempty"`
	DeepLink            string              `json:"deepLink,omitempty"`
	PurchaseRequirement string              `json:"purchaseRequirement,omitempty"`
	PrimaryLocale       string              `json:"primaryLocale,omitempty"`
	Priority            string              `json:"priority,omitempty"`
	Purpose             string              `json:"purpose,omitempty"`
	TerritorySchedules  []TerritorySchedule `json:"territorySchedules,omitempty"`
}

// Analytics types

// AnalyticsReportRequestsResponse represents a list of analytics report requests.
type AnalyticsReportRequestsResponse struct {
	Data     []AnalyticsReportRequest `json:"data"`
	Links    PagedDocumentLinks       `json:"links"`
	Meta     *PagingInformation       `json:"meta,omitempty"`
	Included []any                    `json:"included,omitempty"`
}

// AnalyticsReportRequestResponse represents a single analytics report request.
type AnalyticsReportRequestResponse struct {
	Data     AnalyticsReportRequest `json:"data"`
	Included []any                  `json:"included,omitempty"`
}

// AnalyticsReportRequest represents an analytics report request.
type AnalyticsReportRequest struct {
	Type       string                           `json:"type"`
	ID         string                           `json:"id"`
	Attributes AnalyticsReportRequestAttributes `json:"attributes"`
}

// AnalyticsReportRequestAttributes contains analytics report request attributes.
type AnalyticsReportRequestAttributes struct {
	AccessType string `json:"accessType,omitempty"`
	Stoppable  bool   `json:"stoppable,omitempty"`
}

// AnalyticsReportRequestCreateRequest represents a request to create an analytics report request.
type AnalyticsReportRequestCreateRequest struct {
	Data AnalyticsReportRequestCreateData `json:"data"`
}

// AnalyticsReportRequestCreateData contains the data for creating an analytics report request.
type AnalyticsReportRequestCreateData struct {
	Type          string                                    `json:"type"`
	Attributes    AnalyticsReportRequestCreateAttributes    `json:"attributes"`
	Relationships AnalyticsReportRequestCreateRelationships `json:"relationships"`
}

// AnalyticsReportRequestCreateAttributes contains attributes for creating an analytics report request.
type AnalyticsReportRequestCreateAttributes struct {
	AccessType string `json:"accessType"`
}

// AnalyticsReportRequestCreateRelationships contains relationships for creating an analytics report request.
type AnalyticsReportRequestCreateRelationships struct {
	App RelationshipData `json:"app"`
}

// AnalyticsReportsResponse represents a list of analytics reports.
type AnalyticsReportsResponse struct {
	Data     []AnalyticsReport  `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// AnalyticsReportResponse represents a single analytics report.
type AnalyticsReportResponse struct {
	Data     AnalyticsReport `json:"data"`
	Included []any           `json:"included,omitempty"`
}

// AnalyticsReport represents an analytics report.
type AnalyticsReport struct {
	Type       string                    `json:"type"`
	ID         string                    `json:"id"`
	Attributes AnalyticsReportAttributes `json:"attributes"`
}

// AnalyticsReportAttributes contains analytics report attributes.
type AnalyticsReportAttributes struct {
	Category string `json:"category,omitempty"`
	Name     string `json:"name,omitempty"`
}

// AnalyticsReportInstancesResponse represents a list of report instances.
type AnalyticsReportInstancesResponse struct {
	Data     []AnalyticsReportInstance `json:"data"`
	Links    PagedDocumentLinks        `json:"links"`
	Meta     *PagingInformation        `json:"meta,omitempty"`
	Included []any                     `json:"included,omitempty"`
}

// AnalyticsReportInstance represents an analytics report instance.
type AnalyticsReportInstance struct {
	Type       string                            `json:"type"`
	ID         string                            `json:"id"`
	Attributes AnalyticsReportInstanceAttributes `json:"attributes"`
}

// AnalyticsReportInstanceAttributes contains report instance attributes.
type AnalyticsReportInstanceAttributes struct {
	Granularity    string `json:"granularity,omitempty"`
	ProcessingDate string `json:"processingDate,omitempty"`
}

// AnalyticsReportSegmentsResponse represents a list of report segments.
type AnalyticsReportSegmentsResponse struct {
	Data     []AnalyticsReportSegment `json:"data"`
	Links    PagedDocumentLinks       `json:"links"`
	Meta     *PagingInformation       `json:"meta,omitempty"`
	Included []any                    `json:"included,omitempty"`
}

// AnalyticsReportSegment represents an analytics report segment.
type AnalyticsReportSegment struct {
	Type       string                           `json:"type"`
	ID         string                           `json:"id"`
	Attributes AnalyticsReportSegmentAttributes `json:"attributes"`
}

// AnalyticsReportSegmentAttributes contains report segment attributes.
type AnalyticsReportSegmentAttributes struct {
	Checksum    string `json:"checksum,omitempty"`
	SizeInBytes int    `json:"sizeInBytes,omitempty"`
	URL         string `json:"url,omitempty"`
}

// App Clip types

// AppClipsResponse represents a list of app clips.
type AppClipsResponse struct {
	Data     []AppClip          `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// AppClipResponse represents a single app clip.
type AppClipResponse struct {
	Data     AppClip `json:"data"`
	Included []any   `json:"included,omitempty"`
}

// AppClip represents an app clip.
type AppClip struct {
	Type       string            `json:"type"`
	ID         string            `json:"id"`
	Attributes AppClipAttributes `json:"attributes"`
}

// AppClipAttributes contains app clip attributes.
type AppClipAttributes struct {
	BundleID string `json:"bundleId,omitempty"`
}

// AppClipDefaultExperiencesResponse represents a list of default experiences.
type AppClipDefaultExperiencesResponse struct {
	Data     []AppClipDefaultExperience `json:"data"`
	Links    PagedDocumentLinks         `json:"links"`
	Meta     *PagingInformation         `json:"meta,omitempty"`
	Included []any                      `json:"included,omitempty"`
}

// AppClipDefaultExperienceResponse represents a single default experience.
type AppClipDefaultExperienceResponse struct {
	Data     AppClipDefaultExperience `json:"data"`
	Included []any                    `json:"included,omitempty"`
}

// AppClipDefaultExperience represents an app clip default experience.
type AppClipDefaultExperience struct {
	Type       string                             `json:"type"`
	ID         string                             `json:"id"`
	Attributes AppClipDefaultExperienceAttributes `json:"attributes"`
}

// AppClipDefaultExperienceAttributes contains default experience attributes.
type AppClipDefaultExperienceAttributes struct {
	Action string `json:"action,omitempty"`
}

// AppClipAdvancedExperiencesResponse represents a list of advanced experiences.
type AppClipAdvancedExperiencesResponse struct {
	Data     []AppClipAdvancedExperience `json:"data"`
	Links    PagedDocumentLinks          `json:"links"`
	Meta     *PagingInformation          `json:"meta,omitempty"`
	Included []any                       `json:"included,omitempty"`
}

// AppClipAdvancedExperienceResponse represents a single advanced experience.
type AppClipAdvancedExperienceResponse struct {
	Data     AppClipAdvancedExperience `json:"data"`
	Included []any                     `json:"included,omitempty"`
}

// AppClipAdvancedExperience represents an app clip advanced experience.
type AppClipAdvancedExperience struct {
	Type       string                              `json:"type"`
	ID         string                              `json:"id"`
	Attributes AppClipAdvancedExperienceAttributes `json:"attributes"`
}

// AppClipAdvancedExperienceAttributes contains advanced experience attributes.
type AppClipAdvancedExperienceAttributes struct {
	Action           string `json:"action,omitempty"`
	IsPoweredBy      bool   `json:"isPoweredBy,omitempty"`
	Place            *Place `json:"place,omitempty"`
	PlaceStatus      string `json:"placeStatus,omitempty"`
	BusinessCategory string `json:"businessCategory,omitempty"`
	DefaultLanguage  string `json:"defaultLanguage,omitempty"`
	Removed          bool   `json:"removed,omitempty"`
	Link             string `json:"link,omitempty"`
	Version          int    `json:"version,omitempty"`
	Status           string `json:"status,omitempty"`
}

// Place represents a place for an app clip experience.
type Place struct {
	PlaceID      string       `json:"placeId,omitempty"`
	Names        []string     `json:"names,omitempty"`
	MainAddress  *Address     `json:"mainAddress,omitempty"`
	DisplayPoint *Point       `json:"displayPoint,omitempty"`
	MapAction    string       `json:"mapAction,omitempty"`
	Relationship string       `json:"relationship,omitempty"`
	PhoneNumber  *PhoneNumber `json:"phoneNumber,omitempty"`
	HomepageURL  string       `json:"homepageUrl,omitempty"`
	Categories   []string     `json:"categories,omitempty"`
}

// Address represents an address.
type Address struct {
	StreetAddress []string `json:"streetAddress,omitempty"`
	Floor         string   `json:"floor,omitempty"`
	Neighborhood  string   `json:"neighborhood,omitempty"`
	Locality      string   `json:"locality,omitempty"`
	StateProvince string   `json:"stateProvince,omitempty"`
	PostalCode    string   `json:"postalCode,omitempty"`
	CountryCode   string   `json:"countryCode,omitempty"`
}

// Point represents a geographic point.
type Point struct {
	Coordinates *Coordinates `json:"coordinates,omitempty"`
	Source      string       `json:"source,omitempty"`
}

// Coordinates represents geographic coordinates.
type Coordinates struct {
	Latitude  float64 `json:"latitude,omitempty"`
	Longitude float64 `json:"longitude,omitempty"`
}

// PhoneNumber represents a phone number.
type PhoneNumber struct {
	Number string `json:"number,omitempty"`
	Type   string `json:"type,omitempty"`
	Intent string `json:"intent,omitempty"`
}

// Game Center types

// GameCenterAchievementsResponse represents a list of achievements.
type GameCenterAchievementsResponse struct {
	Data     []GameCenterAchievement `json:"data"`
	Links    PagedDocumentLinks      `json:"links"`
	Meta     *PagingInformation      `json:"meta,omitempty"`
	Included []any                   `json:"included,omitempty"`
}

// GameCenterAchievementResponse represents a single achievement.
type GameCenterAchievementResponse struct {
	Data     GameCenterAchievement `json:"data"`
	Included []any                 `json:"included,omitempty"`
}

// GameCenterAchievement represents a Game Center achievement.
type GameCenterAchievement struct {
	Type       string                          `json:"type"`
	ID         string                          `json:"id"`
	Attributes GameCenterAchievementAttributes `json:"attributes"`
}

// GameCenterAchievementAttributes contains achievement attributes.
type GameCenterAchievementAttributes struct {
	ReferenceName    string `json:"referenceName,omitempty"`
	VendorIdentifier string `json:"vendorIdentifier,omitempty"`
	Points           int    `json:"points,omitempty"`
	ShowBeforeEarned bool   `json:"showBeforeEarned,omitempty"`
	Repeatable       bool   `json:"repeatable,omitempty"`
	Archived         bool   `json:"archived,omitempty"`
}

// GameCenterAchievementCreateRequest represents a request to create an achievement.
type GameCenterAchievementCreateRequest struct {
	Data GameCenterAchievementCreateData `json:"data"`
}

// GameCenterAchievementCreateData contains the data for creating an achievement.
type GameCenterAchievementCreateData struct {
	Type          string                                   `json:"type"`
	Attributes    GameCenterAchievementCreateAttributes    `json:"attributes"`
	Relationships GameCenterAchievementCreateRelationships `json:"relationships"`
}

// GameCenterAchievementCreateAttributes contains attributes for creating an achievement.
type GameCenterAchievementCreateAttributes struct {
	ReferenceName    string `json:"referenceName"`
	VendorIdentifier string `json:"vendorIdentifier"`
	Points           int    `json:"points"`
	ShowBeforeEarned bool   `json:"showBeforeEarned,omitempty"`
	Repeatable       bool   `json:"repeatable,omitempty"`
}

// GameCenterAchievementCreateRelationships contains relationships for creating an achievement.
type GameCenterAchievementCreateRelationships struct {
	GameCenterDetail RelationshipData `json:"gameCenterDetail"`
}

// GameCenterAchievementUpdateRequest represents a request to update an achievement.
type GameCenterAchievementUpdateRequest struct {
	Data GameCenterAchievementUpdateData `json:"data"`
}

// GameCenterAchievementUpdateData contains the data for updating an achievement.
type GameCenterAchievementUpdateData struct {
	Type       string                                `json:"type"`
	ID         string                                `json:"id"`
	Attributes GameCenterAchievementUpdateAttributes `json:"attributes"`
}

// GameCenterAchievementUpdateAttributes contains attributes for updating an achievement.
type GameCenterAchievementUpdateAttributes struct {
	ReferenceName    string `json:"referenceName,omitempty"`
	Points           *int   `json:"points,omitempty"`
	ShowBeforeEarned *bool  `json:"showBeforeEarned,omitempty"`
	Repeatable       *bool  `json:"repeatable,omitempty"`
	Archived         *bool  `json:"archived,omitempty"`
}

// GameCenterLeaderboardsResponse represents a list of leaderboards.
type GameCenterLeaderboardsResponse struct {
	Data     []GameCenterLeaderboard `json:"data"`
	Links    PagedDocumentLinks      `json:"links"`
	Meta     *PagingInformation      `json:"meta,omitempty"`
	Included []any                   `json:"included,omitempty"`
}

// GameCenterLeaderboardResponse represents a single leaderboard.
type GameCenterLeaderboardResponse struct {
	Data     GameCenterLeaderboard `json:"data"`
	Included []any                 `json:"included,omitempty"`
}

// GameCenterLeaderboard represents a Game Center leaderboard.
type GameCenterLeaderboard struct {
	Type       string                          `json:"type"`
	ID         string                          `json:"id"`
	Attributes GameCenterLeaderboardAttributes `json:"attributes"`
}

// GameCenterLeaderboardAttributes contains leaderboard attributes.
type GameCenterLeaderboardAttributes struct {
	ReferenceName       string     `json:"referenceName,omitempty"`
	VendorIdentifier    string     `json:"vendorIdentifier,omitempty"`
	SubmissionType      string     `json:"submissionType,omitempty"`
	ScoreSortType       string     `json:"scoreSortType,omitempty"`
	ScoreRangeStart     string     `json:"scoreRangeStart,omitempty"`
	ScoreRangeEnd       string     `json:"scoreRangeEnd,omitempty"`
	RecurrenceStartDate *time.Time `json:"recurrenceStartDate,omitempty"`
	RecurrenceDuration  string     `json:"recurrenceDuration,omitempty"`
	RecurrenceRule      string     `json:"recurrenceRule,omitempty"`
	Archived            bool       `json:"archived,omitempty"`
}

// GameCenterLeaderboardCreateRequest represents a request to create a leaderboard.
type GameCenterLeaderboardCreateRequest struct {
	Data GameCenterLeaderboardCreateData `json:"data"`
}

// GameCenterLeaderboardCreateData contains the data for creating a leaderboard.
type GameCenterLeaderboardCreateData struct {
	Type          string                                   `json:"type"`
	Attributes    GameCenterLeaderboardCreateAttributes    `json:"attributes"`
	Relationships GameCenterLeaderboardCreateRelationships `json:"relationships"`
}

// GameCenterLeaderboardCreateAttributes contains attributes for creating a leaderboard.
type GameCenterLeaderboardCreateAttributes struct {
	ReferenceName       string     `json:"referenceName"`
	VendorIdentifier    string     `json:"vendorIdentifier"`
	SubmissionType      string     `json:"submissionType"`
	ScoreSortType       string     `json:"scoreSortType"`
	ScoreRangeStart     string     `json:"scoreRangeStart,omitempty"`
	ScoreRangeEnd       string     `json:"scoreRangeEnd,omitempty"`
	RecurrenceStartDate *time.Time `json:"recurrenceStartDate,omitempty"`
	RecurrenceDuration  string     `json:"recurrenceDuration,omitempty"`
	RecurrenceRule      string     `json:"recurrenceRule,omitempty"`
}

// GameCenterLeaderboardCreateRelationships contains relationships for creating a leaderboard.
type GameCenterLeaderboardCreateRelationships struct {
	GameCenterDetail RelationshipData `json:"gameCenterDetail"`
}

// GameCenterLeaderboardUpdateRequest represents a request to update a leaderboard.
type GameCenterLeaderboardUpdateRequest struct {
	Data GameCenterLeaderboardUpdateData `json:"data"`
}

// GameCenterLeaderboardUpdateData contains the data for updating a leaderboard.
type GameCenterLeaderboardUpdateData struct {
	Type       string                                `json:"type"`
	ID         string                                `json:"id"`
	Attributes GameCenterLeaderboardUpdateAttributes `json:"attributes"`
}

// GameCenterLeaderboardUpdateAttributes contains attributes for updating a leaderboard.
type GameCenterLeaderboardUpdateAttributes struct {
	ReferenceName       string     `json:"referenceName,omitempty"`
	SubmissionType      string     `json:"submissionType,omitempty"`
	ScoreSortType       string     `json:"scoreSortType,omitempty"`
	ScoreRangeStart     string     `json:"scoreRangeStart,omitempty"`
	ScoreRangeEnd       string     `json:"scoreRangeEnd,omitempty"`
	RecurrenceStartDate *time.Time `json:"recurrenceStartDate,omitempty"`
	RecurrenceDuration  string     `json:"recurrenceDuration,omitempty"`
	RecurrenceRule      string     `json:"recurrenceRule,omitempty"`
	Archived            *bool      `json:"archived,omitempty"`
}

// GameCenterDetailsResponse represents game center details.
type GameCenterDetailsResponse struct {
	Data     []GameCenterDetail `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// GameCenterDetailResponse represents a single game center detail.
type GameCenterDetailResponse struct {
	Data     GameCenterDetail `json:"data"`
	Included []any            `json:"included,omitempty"`
}

// GameCenterDetail represents game center details for an app.
type GameCenterDetail struct {
	Type       string                     `json:"type"`
	ID         string                     `json:"id"`
	Attributes GameCenterDetailAttributes `json:"attributes"`
}

// GameCenterDetailAttributes contains game center detail attributes.
type GameCenterDetailAttributes struct {
	ArcadeEnabled    bool `json:"arcadeEnabled,omitempty"`
	ChallengeEnabled bool `json:"challengeEnabled,omitempty"`
}

// Xcode Cloud types

// CiBuildRunsResponse represents a list of build runs.
type CiBuildRunsResponse struct {
	Data     []CiBuildRun       `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// CiBuildRunResponse represents a single build run.
type CiBuildRunResponse struct {
	Data     CiBuildRun `json:"data"`
	Included []any      `json:"included,omitempty"`
}

// CiBuildRun represents an Xcode Cloud build run.
type CiBuildRun struct {
	Type       string               `json:"type"`
	ID         string               `json:"id"`
	Attributes CiBuildRunAttributes `json:"attributes"`
}

// CiBuildRunAttributes contains build run attributes.
type CiBuildRunAttributes struct {
	Number             int           `json:"number,omitempty"`
	CreatedDate        *time.Time    `json:"createdDate,omitempty"`
	StartedDate        *time.Time    `json:"startedDate,omitempty"`
	FinishedDate       *time.Time    `json:"finishedDate,omitempty"`
	SourceCommit       *SourceCommit `json:"sourceCommit,omitempty"`
	DestinationCommit  *SourceCommit `json:"destinationCommit,omitempty"`
	IsPullRequestBuild bool          `json:"isPullRequestBuild,omitempty"`
	ExecutionProgress  string        `json:"executionProgress,omitempty"`
	CompletionStatus   string        `json:"completionStatus,omitempty"`
	StartReason        string        `json:"startReason,omitempty"`
	CancelReason       string        `json:"cancelReason,omitempty"`
}

// SourceCommit represents a source commit.
type SourceCommit struct {
	CommitSha string  `json:"commitSha,omitempty"`
	Author    *Author `json:"author,omitempty"`
	Committer *Author `json:"committer,omitempty"`
	Message   string  `json:"message,omitempty"`
	WebURL    string  `json:"webUrl,omitempty"`
}

// Author represents a commit author.
type Author struct {
	DisplayName string `json:"displayName,omitempty"`
}

// CiWorkflowsResponse represents a list of workflows.
type CiWorkflowsResponse struct {
	Data     []CiWorkflow       `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// CiWorkflowResponse represents a single workflow.
type CiWorkflowResponse struct {
	Data     CiWorkflow `json:"data"`
	Included []any      `json:"included,omitempty"`
}

// CiWorkflow represents an Xcode Cloud workflow.
type CiWorkflow struct {
	Type       string               `json:"type"`
	ID         string               `json:"id"`
	Attributes CiWorkflowAttributes `json:"attributes"`
}

// CiWorkflowAttributes contains workflow attributes.
type CiWorkflowAttributes struct {
	Name                       string                      `json:"name,omitempty"`
	Description                string                      `json:"description,omitempty"`
	BranchStartCondition       *BranchStartCondition       `json:"branchStartCondition,omitempty"`
	TagStartCondition          *TagStartCondition          `json:"tagStartCondition,omitempty"`
	PullRequestStartCondition  *PullRequestStartCondition  `json:"pullRequestStartCondition,omitempty"`
	ScheduledStartCondition    *ScheduledStartCondition    `json:"scheduledStartCondition,omitempty"`
	ManualBranchStartCondition *ManualBranchStartCondition `json:"manualBranchStartCondition,omitempty"`
	Actions                    []WorkflowAction            `json:"actions,omitempty"`
	IsEnabled                  bool                        `json:"isEnabled,omitempty"`
	IsLockedForEditing         bool                        `json:"isLockedForEditing,omitempty"`
	Clean                      bool                        `json:"clean,omitempty"`
	ContainerFilePath          string                      `json:"containerFilePath,omitempty"`
	LastModifiedDate           *time.Time                  `json:"lastModifiedDate,omitempty"`
}

// BranchStartCondition represents a branch start condition.
type BranchStartCondition struct {
	Source              *PatternCondition    `json:"source,omitempty"`
	FilesAndFoldersRule *FilesAndFoldersRule `json:"filesAndFoldersRule,omitempty"`
	AutoCancel          bool                 `json:"autoCancel,omitempty"`
}

// TagStartCondition represents a tag start condition.
type TagStartCondition struct {
	Source              *PatternCondition    `json:"source,omitempty"`
	FilesAndFoldersRule *FilesAndFoldersRule `json:"filesAndFoldersRule,omitempty"`
	AutoCancel          bool                 `json:"autoCancel,omitempty"`
}

// PullRequestStartCondition represents a pull request start condition.
type PullRequestStartCondition struct {
	Source              *PatternCondition    `json:"source,omitempty"`
	Destination         *PatternCondition    `json:"destination,omitempty"`
	FilesAndFoldersRule *FilesAndFoldersRule `json:"filesAndFoldersRule,omitempty"`
	AutoCancel          bool                 `json:"autoCancel,omitempty"`
}

// ScheduledStartCondition represents a scheduled start condition.
type ScheduledStartCondition struct {
	Source   *PatternCondition `json:"source,omitempty"`
	Schedule *Schedule         `json:"schedule,omitempty"`
}

// ManualBranchStartCondition represents a manual branch start condition.
type ManualBranchStartCondition struct {
	Source *PatternCondition `json:"source,omitempty"`
}

// PatternCondition represents a pattern condition.
type PatternCondition struct {
	Patterns   []Pattern `json:"patterns,omitempty"`
	IsAllMatch bool      `json:"isAllMatch,omitempty"`
}

// Pattern represents a pattern.
type Pattern struct {
	Pattern  string `json:"pattern,omitempty"`
	IsPrefix bool   `json:"isPrefix,omitempty"`
}

// FilesAndFoldersRule represents a files and folders rule.
type FilesAndFoldersRule struct {
	Mode  string   `json:"mode,omitempty"`
	Paths []string `json:"paths,omitempty"`
}

// Schedule represents a schedule.
type Schedule struct {
	Frequency string   `json:"frequency,omitempty"`
	Days      []string `json:"days,omitempty"`
	Hour      int      `json:"hour,omitempty"`
	Minute    int      `json:"minute,omitempty"`
	Timezone  string   `json:"timezone,omitempty"`
}

// WorkflowAction represents a workflow action.
type WorkflowAction struct {
	Name                      string             `json:"name,omitempty"`
	ActionType                string             `json:"actionType,omitempty"`
	Destination               string             `json:"destination,omitempty"`
	BuildDistributionAudience string             `json:"buildDistributionAudience,omitempty"`
	TestConfiguration         *TestConfiguration `json:"testConfiguration,omitempty"`
	Scheme                    string             `json:"scheme,omitempty"`
	Platform                  string             `json:"platform,omitempty"`
	IsRequiredToPass          bool               `json:"isRequiredToPass,omitempty"`
}

// TestConfiguration represents a test configuration.
type TestConfiguration struct {
	Kind             string            `json:"kind,omitempty"`
	TestPlanName     string            `json:"testPlanName,omitempty"`
	TestDestinations []TestDestination `json:"testDestinations,omitempty"`
}

// TestDestination represents a test destination.
type TestDestination struct {
	DeviceTypeName       string `json:"deviceTypeName,omitempty"`
	DeviceTypeIdentifier string `json:"deviceTypeIdentifier,omitempty"`
	RuntimeName          string `json:"runtimeName,omitempty"`
	RuntimeIdentifier    string `json:"runtimeIdentifier,omitempty"`
	Kind                 string `json:"kind,omitempty"`
}

// CiProductsResponse represents a list of products.
type CiProductsResponse struct {
	Data     []CiProduct        `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// CiProductResponse represents a single product.
type CiProductResponse struct {
	Data     CiProduct `json:"data"`
	Included []any     `json:"included,omitempty"`
}

// CiProduct represents an Xcode Cloud product.
type CiProduct struct {
	Type       string              `json:"type"`
	ID         string              `json:"id"`
	Attributes CiProductAttributes `json:"attributes"`
}

// CiProductAttributes contains product attributes.
type CiProductAttributes struct {
	Name        string     `json:"name,omitempty"`
	CreatedDate *time.Time `json:"createdDate,omitempty"`
	ProductType string     `json:"productType,omitempty"`
}

// Sales and Finance types

// SalesReportsResponse represents a list of sales reports.
type SalesReportsResponse struct {
	Data []byte `json:"data,omitempty"`
}

// FinanceReportsResponse represents a list of finance reports.
type FinanceReportsResponse struct {
	Data []byte `json:"data,omitempty"`
}

// App Encryption types

// AppEncryptionDeclarationsResponse represents a list of encryption declarations.
type AppEncryptionDeclarationsResponse struct {
	Data     []AppEncryptionDeclaration `json:"data"`
	Links    PagedDocumentLinks         `json:"links"`
	Meta     *PagingInformation         `json:"meta,omitempty"`
	Included []any                      `json:"included,omitempty"`
}

// AppEncryptionDeclarationResponse represents a single encryption declaration.
type AppEncryptionDeclarationResponse struct {
	Data     AppEncryptionDeclaration `json:"data"`
	Included []any                    `json:"included,omitempty"`
}

// AppEncryptionDeclaration represents an encryption declaration.
type AppEncryptionDeclaration struct {
	Type       string                             `json:"type"`
	ID         string                             `json:"id"`
	Attributes AppEncryptionDeclarationAttributes `json:"attributes"`
}

// AppEncryptionDeclarationAttributes contains encryption declaration attributes.
type AppEncryptionDeclarationAttributes struct {
	AppDescription                  string `json:"appDescription,omitempty"`
	CreatedDate                     string `json:"createdDate,omitempty"`
	UsesEncryption                  bool   `json:"usesEncryption,omitempty"`
	Exempt                          bool   `json:"exempt,omitempty"`
	ContainsProprietaryCryptography bool   `json:"containsProprietaryCryptography,omitempty"`
	ContainsThirdPartyCryptography  bool   `json:"containsThirdPartyCryptography,omitempty"`
	AvailableOnFrenchStore          bool   `json:"availableOnFrenchStore,omitempty"`
	Platform                        string `json:"platform,omitempty"`
	UploadedDate                    string `json:"uploadedDate,omitempty"`
	DocumentURL                     string `json:"documentUrl,omitempty"`
	DocumentName                    string `json:"documentName,omitempty"`
	DocumentType                    string `json:"documentType,omitempty"`
	AppEncryptionDeclarationState   string `json:"appEncryptionDeclarationState,omitempty"`
	CodeValue                       string `json:"codeValue,omitempty"`
}

// AppEncryptionDeclarationCreateRequest represents a request to create an encryption declaration.
type AppEncryptionDeclarationCreateRequest struct {
	Data AppEncryptionDeclarationCreateData `json:"data"`
}

// AppEncryptionDeclarationCreateData contains the data for creating an encryption declaration.
type AppEncryptionDeclarationCreateData struct {
	Type          string                                      `json:"type"`
	Attributes    AppEncryptionDeclarationCreateAttributes    `json:"attributes"`
	Relationships AppEncryptionDeclarationCreateRelationships `json:"relationships"`
}

// AppEncryptionDeclarationCreateAttributes contains attributes for creating an encryption declaration.
type AppEncryptionDeclarationCreateAttributes struct {
	AppDescription                  string `json:"appDescription,omitempty"`
	UsesEncryption                  bool   `json:"usesEncryption"`
	Exempt                          bool   `json:"exempt,omitempty"`
	ContainsProprietaryCryptography bool   `json:"containsProprietaryCryptography,omitempty"`
	ContainsThirdPartyCryptography  bool   `json:"containsThirdPartyCryptography,omitempty"`
	AvailableOnFrenchStore          bool   `json:"availableOnFrenchStore,omitempty"`
	CodeValue                       string `json:"codeValue,omitempty"`
}

// AppEncryptionDeclarationCreateRelationships contains relationships for creating an encryption declaration.
type AppEncryptionDeclarationCreateRelationships struct {
	App RelationshipData `json:"app"`
}
