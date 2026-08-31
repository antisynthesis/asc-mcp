package api

import (
	"context"
	"encoding/json"
	"fmt"
	"net/url"
)

// Commerce API methods (App Store Connect API 4.2-4.4.1).
//
// In-app purchases and subscriptions moved to a versioned model in 4.4.1:
// the product resource is now a long-lived container and every editable
// piece of metadata hangs off a version. Creating a version snapshots the
// product's current metadata; the version's localizations and images are
// created through the v2 collections, which point at a version rather
// than at the product. Versions reach App Review through
// reviewSubmissionItems, the same way App Store versions do.
//
// The rest of this file fills in the commerce endpoints that had no client
// coverage: IAP price points, price schedules and availability, the
// subscriptionPlanAvailability resource that replaces the deprecated
// subscriptionAvailability (4.4), IAP offer codes (4.2), subscription
// price point equalizations (4.4.1), and the price-schedule writes that
// make app and IAP pricing settable instead of read-only.

// Commerce version types

// CommerceVersionAttributes contains the attributes shared by the
// inAppPurchaseVersion, subscriptionVersion and subscriptionGroupVersion
// resources: a monotonically increasing version number and the review
// state of that version.
type CommerceVersionAttributes struct {
	Version int    `json:"version,omitempty"`
	State   string `json:"state,omitempty"`
}

// InAppPurchaseVersionsResponse represents a list of in-app purchase versions.
type InAppPurchaseVersionsResponse struct {
	Data     []InAppPurchaseVersion `json:"data"`
	Links    PagedDocumentLinks     `json:"links"`
	Meta     *PagingInformation     `json:"meta,omitempty"`
	Included []any                  `json:"included,omitempty"`
}

// InAppPurchaseVersionResponse represents a single in-app purchase version.
type InAppPurchaseVersionResponse struct {
	Data     InAppPurchaseVersion `json:"data"`
	Included []any                `json:"included,omitempty"`
}

// InAppPurchaseVersion represents a version of an in-app purchase.
type InAppPurchaseVersion struct {
	Type       string                    `json:"type"`
	ID         string                    `json:"id"`
	Attributes CommerceVersionAttributes `json:"attributes"`
}

// InAppPurchaseVersionCreateRequest represents a request to create an
// in-app purchase version.
type InAppPurchaseVersionCreateRequest struct {
	Data InAppPurchaseVersionCreateData `json:"data"`
}

// InAppPurchaseVersionCreateData contains the data for creating an in-app purchase version.
type InAppPurchaseVersionCreateData struct {
	Type          string                                  `json:"type"`
	Relationships InAppPurchaseVersionCreateRelationships `json:"relationships"`
}

// InAppPurchaseVersionCreateRelationships contains relationships for creating an in-app purchase version.
type InAppPurchaseVersionCreateRelationships struct {
	InAppPurchase RelationshipData `json:"inAppPurchase"`
}

// SubscriptionVersionsResponse represents a list of subscription versions.
type SubscriptionVersionsResponse struct {
	Data     []SubscriptionVersion `json:"data"`
	Links    PagedDocumentLinks    `json:"links"`
	Meta     *PagingInformation    `json:"meta,omitempty"`
	Included []any                 `json:"included,omitempty"`
}

// SubscriptionVersionResponse represents a single subscription version.
type SubscriptionVersionResponse struct {
	Data     SubscriptionVersion `json:"data"`
	Included []any               `json:"included,omitempty"`
}

// SubscriptionVersion represents a version of a subscription.
type SubscriptionVersion struct {
	Type       string                    `json:"type"`
	ID         string                    `json:"id"`
	Attributes CommerceVersionAttributes `json:"attributes"`
}

// SubscriptionVersionCreateRequest represents a request to create a subscription version.
type SubscriptionVersionCreateRequest struct {
	Data SubscriptionVersionCreateData `json:"data"`
}

// SubscriptionVersionCreateData contains the data for creating a subscription version.
type SubscriptionVersionCreateData struct {
	Type          string                                 `json:"type"`
	Relationships SubscriptionVersionCreateRelationships `json:"relationships"`
}

// SubscriptionVersionCreateRelationships contains relationships for creating a subscription version.
type SubscriptionVersionCreateRelationships struct {
	Subscription RelationshipData `json:"subscription"`
}

// SubscriptionGroupVersionsResponse represents a list of subscription group versions.
type SubscriptionGroupVersionsResponse struct {
	Data     []SubscriptionGroupVersion `json:"data"`
	Links    PagedDocumentLinks         `json:"links"`
	Meta     *PagingInformation         `json:"meta,omitempty"`
	Included []any                      `json:"included,omitempty"`
}

// SubscriptionGroupVersionResponse represents a single subscription group version.
type SubscriptionGroupVersionResponse struct {
	Data     SubscriptionGroupVersion `json:"data"`
	Included []any                    `json:"included,omitempty"`
}

// SubscriptionGroupVersion represents a version of a subscription group.
type SubscriptionGroupVersion struct {
	Type       string                    `json:"type"`
	ID         string                    `json:"id"`
	Attributes CommerceVersionAttributes `json:"attributes"`
}

// SubscriptionGroupVersionCreateRequest represents a request to create a
// subscription group version.
type SubscriptionGroupVersionCreateRequest struct {
	Data SubscriptionGroupVersionCreateData `json:"data"`
}

// SubscriptionGroupVersionCreateData contains the data for creating a subscription group version.
type SubscriptionGroupVersionCreateData struct {
	Type          string                                      `json:"type"`
	Relationships SubscriptionGroupVersionCreateRelationships `json:"relationships"`
}

// SubscriptionGroupVersionCreateRelationships contains relationships for creating a subscription group version.
type SubscriptionGroupVersionCreateRelationships struct {
	SubscriptionGroup RelationshipData `json:"subscriptionGroup"`
}

// Version-scoped localization and image types
//
// The v2 collections carry the same resource types as the v1 ones
// ("inAppPurchaseLocalizations", "subscriptionImages", ...); only the
// create request differs, pointing at a version instead of the product.

// InAppPurchaseLocalizationsResponse represents a list of in-app purchase localizations.
type InAppPurchaseLocalizationsResponse struct {
	Data     []InAppPurchaseLocalization `json:"data"`
	Links    PagedDocumentLinks          `json:"links"`
	Meta     *PagingInformation          `json:"meta,omitempty"`
	Included []any                       `json:"included,omitempty"`
}

// InAppPurchaseLocalizationResponse represents a single in-app purchase localization.
type InAppPurchaseLocalizationResponse struct {
	Data     InAppPurchaseLocalization `json:"data"`
	Included []any                     `json:"included,omitempty"`
}

// InAppPurchaseLocalization represents the display name and description
// of an in-app purchase in one locale.
type InAppPurchaseLocalization struct {
	Type       string                              `json:"type"`
	ID         string                              `json:"id"`
	Attributes InAppPurchaseLocalizationAttributes `json:"attributes"`
}

// InAppPurchaseLocalizationAttributes contains in-app purchase localization attributes.
type InAppPurchaseLocalizationAttributes struct {
	Name        string `json:"name,omitempty"`
	Locale      string `json:"locale,omitempty"`
	Description string `json:"description,omitempty"`
	State       string `json:"state,omitempty"`
}

// InAppPurchaseLocalizationCreateRequest represents a request to create a
// version-scoped in-app purchase localization (POST /v2/inAppPurchaseLocalizations).
type InAppPurchaseLocalizationCreateRequest struct {
	Data InAppPurchaseLocalizationCreateData `json:"data"`
}

// InAppPurchaseLocalizationCreateData contains the data for creating an in-app purchase localization.
type InAppPurchaseLocalizationCreateData struct {
	Type          string                                       `json:"type"`
	Attributes    InAppPurchaseLocalizationCreateAttributes    `json:"attributes"`
	Relationships InAppPurchaseLocalizationCreateRelationships `json:"relationships"`
}

// InAppPurchaseLocalizationCreateAttributes contains attributes for creating an in-app purchase localization.
type InAppPurchaseLocalizationCreateAttributes struct {
	Name        string `json:"name"`
	Locale      string `json:"locale"`
	Description string `json:"description,omitempty"`
}

// InAppPurchaseLocalizationCreateRelationships contains relationships for creating an in-app purchase localization.
type InAppPurchaseLocalizationCreateRelationships struct {
	Version RelationshipData `json:"version"`
}

// InAppPurchaseLocalizationUpdateRequest represents a request to update an in-app purchase localization.
type InAppPurchaseLocalizationUpdateRequest struct {
	Data InAppPurchaseLocalizationUpdateData `json:"data"`
}

// InAppPurchaseLocalizationUpdateData contains the data for updating an in-app purchase localization.
type InAppPurchaseLocalizationUpdateData struct {
	Type       string                                    `json:"type"`
	ID         string                                    `json:"id"`
	Attributes InAppPurchaseLocalizationUpdateAttributes `json:"attributes"`
}

// InAppPurchaseLocalizationUpdateAttributes contains attributes for updating an in-app purchase localization.
type InAppPurchaseLocalizationUpdateAttributes struct {
	Name        string `json:"name,omitempty"`
	Description string `json:"description,omitempty"`
}

// CommerceImageAttributes contains the attributes shared by the
// inAppPurchaseImage and subscriptionImage resources.
type CommerceImageAttributes struct {
	FileSize           int               `json:"fileSize,omitempty"`
	FileName           string            `json:"fileName,omitempty"`
	SourceFileChecksum string            `json:"sourceFileChecksum,omitempty"`
	AssetToken         string            `json:"assetToken,omitempty"`
	ImageAsset         *ImageAsset       `json:"imageAsset,omitempty"`
	UploadOperations   []UploadOperation `json:"uploadOperations,omitempty"`
	State              string            `json:"state,omitempty"`
}

// InAppPurchaseImagesResponse represents a list of in-app purchase images.
type InAppPurchaseImagesResponse struct {
	Data     []InAppPurchaseImage `json:"data"`
	Links    PagedDocumentLinks   `json:"links"`
	Meta     *PagingInformation   `json:"meta,omitempty"`
	Included []any                `json:"included,omitempty"`
}

// InAppPurchaseImageResponse represents a single in-app purchase image.
type InAppPurchaseImageResponse struct {
	Data     InAppPurchaseImage `json:"data"`
	Included []any              `json:"included,omitempty"`
}

// InAppPurchaseImage represents the promotional image of an in-app purchase.
type InAppPurchaseImage struct {
	Type       string                  `json:"type"`
	ID         string                  `json:"id"`
	Attributes CommerceImageAttributes `json:"attributes"`
}

// InAppPurchaseImageCreateRequest represents a request to reserve a
// version-scoped in-app purchase image (POST /v2/inAppPurchaseImages).
type InAppPurchaseImageCreateRequest struct {
	Data InAppPurchaseImageCreateData `json:"data"`
}

// InAppPurchaseImageCreateData contains the data for creating an in-app purchase image.
type InAppPurchaseImageCreateData struct {
	Type          string                                `json:"type"`
	Attributes    CommerceImageCreateAttributes         `json:"attributes"`
	Relationships InAppPurchaseImageCreateRelationships `json:"relationships"`
}

// CommerceImageCreateAttributes contains the attributes required to
// reserve an image upload for either commerce image resource.
type CommerceImageCreateAttributes struct {
	FileName string `json:"fileName"`
	FileSize int    `json:"fileSize"`
}

// InAppPurchaseImageCreateRelationships contains relationships for creating an in-app purchase image.
type InAppPurchaseImageCreateRelationships struct {
	Version RelationshipData `json:"version"`
}

// CommerceImageUpdateAttributes contains attributes for committing a
// commerce image upload. Set Uploaded to true once every upload
// operation returned by the create call has been performed.
type CommerceImageUpdateAttributes struct {
	Uploaded *bool `json:"uploaded,omitempty"`
}

// InAppPurchaseImageUpdateRequest represents a request to update an in-app purchase image.
type InAppPurchaseImageUpdateRequest struct {
	Data InAppPurchaseImageUpdateData `json:"data"`
}

// InAppPurchaseImageUpdateData contains the data for updating an in-app purchase image.
type InAppPurchaseImageUpdateData struct {
	Type       string                        `json:"type"`
	ID         string                        `json:"id"`
	Attributes CommerceImageUpdateAttributes `json:"attributes"`
}

// SubscriptionLocalizationsResponse represents a list of subscription localizations.
type SubscriptionLocalizationsResponse struct {
	Data     []SubscriptionLocalization `json:"data"`
	Links    PagedDocumentLinks         `json:"links"`
	Meta     *PagingInformation         `json:"meta,omitempty"`
	Included []any                      `json:"included,omitempty"`
}

// SubscriptionLocalizationResponse represents a single subscription localization.
type SubscriptionLocalizationResponse struct {
	Data     SubscriptionLocalization `json:"data"`
	Included []any                    `json:"included,omitempty"`
}

// SubscriptionLocalization represents the display name and description of
// a subscription in one locale.
type SubscriptionLocalization struct {
	Type       string                              `json:"type"`
	ID         string                              `json:"id"`
	Attributes InAppPurchaseLocalizationAttributes `json:"attributes"`
}

// SubscriptionLocalizationCreateRequest represents a request to create a
// version-scoped subscription localization (POST /v2/subscriptionLocalizations).
type SubscriptionLocalizationCreateRequest struct {
	Data SubscriptionLocalizationCreateData `json:"data"`
}

// SubscriptionLocalizationCreateData contains the data for creating a subscription localization.
type SubscriptionLocalizationCreateData struct {
	Type          string                                    `json:"type"`
	Attributes    InAppPurchaseLocalizationCreateAttributes `json:"attributes"`
	Relationships SubscriptionLocalizationRelationships     `json:"relationships"`
}

// SubscriptionLocalizationRelationships points a subscription
// localization or image at its owning subscription version.
type SubscriptionLocalizationRelationships struct {
	Version RelationshipData `json:"version"`
}

// SubscriptionLocalizationUpdateRequest represents a request to update a subscription localization.
type SubscriptionLocalizationUpdateRequest struct {
	Data SubscriptionLocalizationUpdateData `json:"data"`
}

// SubscriptionLocalizationUpdateData contains the data for updating a subscription localization.
type SubscriptionLocalizationUpdateData struct {
	Type       string                                    `json:"type"`
	ID         string                                    `json:"id"`
	Attributes InAppPurchaseLocalizationUpdateAttributes `json:"attributes"`
}

// SubscriptionImagesResponse represents a list of subscription images.
type SubscriptionImagesResponse struct {
	Data     []SubscriptionImage `json:"data"`
	Links    PagedDocumentLinks  `json:"links"`
	Meta     *PagingInformation  `json:"meta,omitempty"`
	Included []any               `json:"included,omitempty"`
}

// SubscriptionImageResponse represents a single subscription image.
type SubscriptionImageResponse struct {
	Data     SubscriptionImage `json:"data"`
	Included []any             `json:"included,omitempty"`
}

// SubscriptionImage represents the promotional image of a subscription.
type SubscriptionImage struct {
	Type       string                  `json:"type"`
	ID         string                  `json:"id"`
	Attributes CommerceImageAttributes `json:"attributes"`
}

// SubscriptionImageCreateRequest represents a request to reserve a
// version-scoped subscription image (POST /v2/subscriptionImages).
type SubscriptionImageCreateRequest struct {
	Data SubscriptionImageCreateData `json:"data"`
}

// SubscriptionImageCreateData contains the data for creating a subscription image.
type SubscriptionImageCreateData struct {
	Type          string                                `json:"type"`
	Attributes    CommerceImageCreateAttributes         `json:"attributes"`
	Relationships SubscriptionLocalizationRelationships `json:"relationships"`
}

// SubscriptionImageUpdateRequest represents a request to update a subscription image.
type SubscriptionImageUpdateRequest struct {
	Data SubscriptionImageUpdateData `json:"data"`
}

// SubscriptionImageUpdateData contains the data for updating a subscription image.
type SubscriptionImageUpdateData struct {
	Type       string                        `json:"type"`
	ID         string                        `json:"id"`
	Attributes CommerceImageUpdateAttributes `json:"attributes"`
}

// SubscriptionGroupLocalizationsResponse represents a list of subscription group localizations.
type SubscriptionGroupLocalizationsResponse struct {
	Data     []SubscriptionGroupLocalization `json:"data"`
	Links    PagedDocumentLinks              `json:"links"`
	Meta     *PagingInformation              `json:"meta,omitempty"`
	Included []any                           `json:"included,omitempty"`
}

// SubscriptionGroupLocalizationResponse represents a single subscription group localization.
type SubscriptionGroupLocalizationResponse struct {
	Data     SubscriptionGroupLocalization `json:"data"`
	Included []any                         `json:"included,omitempty"`
}

// SubscriptionGroupLocalization represents a subscription group's display
// name in one locale.
type SubscriptionGroupLocalization struct {
	Type       string                                  `json:"type"`
	ID         string                                  `json:"id"`
	Attributes SubscriptionGroupLocalizationAttributes `json:"attributes"`
}

// SubscriptionGroupLocalizationAttributes contains subscription group localization attributes.
type SubscriptionGroupLocalizationAttributes struct {
	Name          string `json:"name,omitempty"`
	CustomAppName string `json:"customAppName,omitempty"`
	Locale        string `json:"locale,omitempty"`
	State         string `json:"state,omitempty"`
}

// SubscriptionGroupLocalizationCreateRequest represents a request to
// create a version-scoped subscription group localization
// (POST /v2/subscriptionGroupLocalizations).
type SubscriptionGroupLocalizationCreateRequest struct {
	Data SubscriptionGroupLocalizationCreateData `json:"data"`
}

// SubscriptionGroupLocalizationCreateData contains the data for creating a subscription group localization.
type SubscriptionGroupLocalizationCreateData struct {
	Type          string                                        `json:"type"`
	Attributes    SubscriptionGroupLocalizationCreateAttributes `json:"attributes"`
	Relationships SubscriptionLocalizationRelationships         `json:"relationships"`
}

// SubscriptionGroupLocalizationCreateAttributes contains attributes for creating a subscription group localization.
type SubscriptionGroupLocalizationCreateAttributes struct {
	Name          string `json:"name"`
	Locale        string `json:"locale"`
	CustomAppName string `json:"customAppName,omitempty"`
}

// SubscriptionGroupLocalizationUpdateRequest represents a request to update a subscription group localization.
type SubscriptionGroupLocalizationUpdateRequest struct {
	Data SubscriptionGroupLocalizationUpdateData `json:"data"`
}

// SubscriptionGroupLocalizationUpdateData contains the data for updating a subscription group localization.
type SubscriptionGroupLocalizationUpdateData struct {
	Type       string                                        `json:"type"`
	ID         string                                        `json:"id"`
	Attributes SubscriptionGroupLocalizationUpdateAttributes `json:"attributes"`
}

// SubscriptionGroupLocalizationUpdateAttributes contains attributes for updating a subscription group localization.
type SubscriptionGroupLocalizationUpdateAttributes struct {
	Name          string `json:"name,omitempty"`
	CustomAppName string `json:"customAppName,omitempty"`
}

// In-app purchase offer code types (App Store Connect API 4.2)

// InAppPurchaseOfferCodesResponse represents a list of in-app purchase offer codes.
type InAppPurchaseOfferCodesResponse struct {
	Data     []InAppPurchaseOfferCode `json:"data"`
	Links    PagedDocumentLinks       `json:"links"`
	Meta     *PagingInformation       `json:"meta,omitempty"`
	Included []any                    `json:"included,omitempty"`
}

// InAppPurchaseOfferCodeResponse represents a single in-app purchase offer code.
type InAppPurchaseOfferCodeResponse struct {
	Data     InAppPurchaseOfferCode `json:"data"`
	Included []any                  `json:"included,omitempty"`
}

// InAppPurchaseOfferCode represents an offer code for a non-subscription
// in-app purchase.
type InAppPurchaseOfferCode struct {
	Type       string                           `json:"type"`
	ID         string                           `json:"id"`
	Attributes InAppPurchaseOfferCodeAttributes `json:"attributes"`
}

// InAppPurchaseOfferCodeAttributes contains in-app purchase offer code attributes.
type InAppPurchaseOfferCodeAttributes struct {
	Name                  string   `json:"name,omitempty"`
	CustomerEligibilities []string `json:"customerEligibilities,omitempty"`
	ProductionCodeCount   int      `json:"productionCodeCount,omitempty"`
	SandboxCodeCount      int      `json:"sandboxCodeCount,omitempty"`
	Active                bool     `json:"active,omitempty"`
}

// InAppPurchaseOfferCodeCreateRequest represents a request to create an
// in-app purchase offer code. The per-territory prices are declared
// inline through the prices relationship and the included array.
type InAppPurchaseOfferCodeCreateRequest struct {
	Data     InAppPurchaseOfferCodeCreateData      `json:"data"`
	Included []InAppPurchaseOfferPriceInlineCreate `json:"included,omitempty"`
}

// InAppPurchaseOfferCodeCreateData contains the data for creating an in-app purchase offer code.
type InAppPurchaseOfferCodeCreateData struct {
	Type          string                                    `json:"type"`
	Attributes    InAppPurchaseOfferCodeCreateAttributes    `json:"attributes"`
	Relationships InAppPurchaseOfferCodeCreateRelationships `json:"relationships"`
}

// InAppPurchaseOfferCodeCreateAttributes contains attributes for creating an in-app purchase offer code.
type InAppPurchaseOfferCodeCreateAttributes struct {
	Name                  string   `json:"name"`
	CustomerEligibilities []string `json:"customerEligibilities"`
}

// InAppPurchaseOfferCodeCreateRelationships contains relationships for creating an in-app purchase offer code.
type InAppPurchaseOfferCodeCreateRelationships struct {
	InAppPurchase RelationshipData     `json:"inAppPurchase"`
	Prices        RelationshipDataList `json:"prices"`
}

// InAppPurchaseOfferPriceInlineCreate represents an offer price declared
// inline in an offer code create request.
type InAppPurchaseOfferPriceInlineCreate struct {
	Type          string                                            `json:"type"`
	ID            string                                            `json:"id,omitempty"`
	Relationships *InAppPurchaseOfferPriceInlineCreateRelationships `json:"relationships,omitempty"`
}

// InAppPurchaseOfferPriceInlineCreateRelationships contains relationships for an inline offer price.
type InAppPurchaseOfferPriceInlineCreateRelationships struct {
	Territory  RelationshipData `json:"territory"`
	PricePoint RelationshipData `json:"pricePoint"`
}

// InAppPurchaseOfferCodeUpdateRequest represents a request to update an in-app purchase offer code.
type InAppPurchaseOfferCodeUpdateRequest struct {
	Data InAppPurchaseOfferCodeUpdateData `json:"data"`
}

// InAppPurchaseOfferCodeUpdateData contains the data for updating an in-app purchase offer code.
type InAppPurchaseOfferCodeUpdateData struct {
	Type       string                    `json:"type"`
	ID         string                    `json:"id"`
	Attributes OfferCodeActiveAttributes `json:"attributes"`
}

// OfferCodeActiveAttributes toggles an offer code, custom code or
// one-time-use code batch on or off. Deactivating is the only mutation
// Apple allows on an issued batch.
type OfferCodeActiveAttributes struct {
	Active *bool `json:"active,omitempty"`
}

// InAppPurchaseOfferPricesResponse represents a list of in-app purchase offer prices.
type InAppPurchaseOfferPricesResponse struct {
	Data     []InAppPurchaseOfferPrice `json:"data"`
	Links    PagedDocumentLinks        `json:"links"`
	Meta     *PagingInformation        `json:"meta,omitempty"`
	Included []any                     `json:"included,omitempty"`
}

// InAppPurchaseOfferPrice represents the price of an offer code in one
// territory. The resource carries no attributes; the interesting data is
// in its territory and pricePoint relationships.
type InAppPurchaseOfferPrice struct {
	Type          string                               `json:"type"`
	ID            string                               `json:"id"`
	Relationships InAppPurchaseOfferPriceRelationships `json:"relationships"`
}

// InAppPurchaseOfferPriceRelationships contains in-app purchase offer price relationships.
type InAppPurchaseOfferPriceRelationships struct {
	Territory  *RelationshipData `json:"territory,omitempty"`
	PricePoint *RelationshipData `json:"pricePoint,omitempty"`
}

// InAppPurchaseOfferCodeCustomCodesResponse represents a list of custom offer codes.
type InAppPurchaseOfferCodeCustomCodesResponse struct {
	Data     []InAppPurchaseOfferCodeCustomCode `json:"data"`
	Links    PagedDocumentLinks                 `json:"links"`
	Meta     *PagingInformation                 `json:"meta,omitempty"`
	Included []any                              `json:"included,omitempty"`
}

// InAppPurchaseOfferCodeCustomCodeResponse represents a single custom offer code.
type InAppPurchaseOfferCodeCustomCodeResponse struct {
	Data     InAppPurchaseOfferCodeCustomCode `json:"data"`
	Included []any                            `json:"included,omitempty"`
}

// InAppPurchaseOfferCodeCustomCode represents a batch of redemptions of a
// single memorable code (e.g. "LAUNCH2026").
type InAppPurchaseOfferCodeCustomCode struct {
	Type       string                                     `json:"type"`
	ID         string                                     `json:"id"`
	Attributes InAppPurchaseOfferCodeCustomCodeAttributes `json:"attributes"`
}

// InAppPurchaseOfferCodeCustomCodeAttributes contains custom offer code attributes.
type InAppPurchaseOfferCodeCustomCodeAttributes struct {
	CustomCode     string `json:"customCode,omitempty"`
	NumberOfCodes  int    `json:"numberOfCodes,omitempty"`
	CreatedDate    string `json:"createdDate,omitempty"`
	ExpirationDate string `json:"expirationDate,omitempty"`
	Active         bool   `json:"active,omitempty"`
}

// InAppPurchaseOfferCodeCustomCodeCreateRequest represents a request to create a custom offer code.
type InAppPurchaseOfferCodeCustomCodeCreateRequest struct {
	Data InAppPurchaseOfferCodeCustomCodeCreateData `json:"data"`
}

// InAppPurchaseOfferCodeCustomCodeCreateData contains the data for creating a custom offer code.
type InAppPurchaseOfferCodeCustomCodeCreateData struct {
	Type          string                                           `json:"type"`
	Attributes    InAppPurchaseOfferCodeCustomCodeCreateAttributes `json:"attributes"`
	Relationships OfferCodeRelationship                            `json:"relationships"`
}

// InAppPurchaseOfferCodeCustomCodeCreateAttributes contains attributes for creating a custom offer code.
type InAppPurchaseOfferCodeCustomCodeCreateAttributes struct {
	CustomCode     string `json:"customCode"`
	NumberOfCodes  int    `json:"numberOfCodes"`
	ExpirationDate string `json:"expirationDate,omitempty"`
}

// OfferCodeRelationship points a code batch at its owning offer code.
type OfferCodeRelationship struct {
	OfferCode RelationshipData `json:"offerCode"`
}

// InAppPurchaseOfferCodeCustomCodeUpdateRequest represents a request to update a custom offer code.
type InAppPurchaseOfferCodeCustomCodeUpdateRequest struct {
	Data InAppPurchaseOfferCodeCustomCodeUpdateData `json:"data"`
}

// InAppPurchaseOfferCodeCustomCodeUpdateData contains the data for updating a custom offer code.
type InAppPurchaseOfferCodeCustomCodeUpdateData struct {
	Type       string                    `json:"type"`
	ID         string                    `json:"id"`
	Attributes OfferCodeActiveAttributes `json:"attributes"`
}

// InAppPurchaseOfferCodeOneTimeUseCodesResponse represents a list of one-time-use code batches.
type InAppPurchaseOfferCodeOneTimeUseCodesResponse struct {
	Data     []InAppPurchaseOfferCodeOneTimeUseCode `json:"data"`
	Links    PagedDocumentLinks                     `json:"links"`
	Meta     *PagingInformation                     `json:"meta,omitempty"`
	Included []any                                  `json:"included,omitempty"`
}

// InAppPurchaseOfferCodeOneTimeUseCodeResponse represents a single one-time-use code batch.
type InAppPurchaseOfferCodeOneTimeUseCodeResponse struct {
	Data     InAppPurchaseOfferCodeOneTimeUseCode `json:"data"`
	Included []any                                `json:"included,omitempty"`
}

// InAppPurchaseOfferCodeOneTimeUseCode represents a batch of unique,
// single-redemption offer codes. The generated codes themselves are
// downloaded as CSV from the batch's values endpoint.
type InAppPurchaseOfferCodeOneTimeUseCode struct {
	Type       string                                         `json:"type"`
	ID         string                                         `json:"id"`
	Attributes InAppPurchaseOfferCodeOneTimeUseCodeAttributes `json:"attributes"`
}

// InAppPurchaseOfferCodeOneTimeUseCodeAttributes contains one-time-use code batch attributes.
type InAppPurchaseOfferCodeOneTimeUseCodeAttributes struct {
	NumberOfCodes  int    `json:"numberOfCodes,omitempty"`
	CreatedDate    string `json:"createdDate,omitempty"`
	ExpirationDate string `json:"expirationDate,omitempty"`
	Active         bool   `json:"active,omitempty"`
	Environment    string `json:"environment,omitempty"`
}

// InAppPurchaseOfferCodeOneTimeUseCodeCreateRequest represents a request to create a one-time-use code batch.
type InAppPurchaseOfferCodeOneTimeUseCodeCreateRequest struct {
	Data InAppPurchaseOfferCodeOneTimeUseCodeCreateData `json:"data"`
}

// InAppPurchaseOfferCodeOneTimeUseCodeCreateData contains the data for creating a one-time-use code batch.
type InAppPurchaseOfferCodeOneTimeUseCodeCreateData struct {
	Type          string                                               `json:"type"`
	Attributes    InAppPurchaseOfferCodeOneTimeUseCodeCreateAttributes `json:"attributes"`
	Relationships OfferCodeRelationship                                `json:"relationships"`
}

// InAppPurchaseOfferCodeOneTimeUseCodeCreateAttributes contains attributes for creating a one-time-use code batch.
type InAppPurchaseOfferCodeOneTimeUseCodeCreateAttributes struct {
	NumberOfCodes  int    `json:"numberOfCodes"`
	ExpirationDate string `json:"expirationDate"`
	Environment    string `json:"environment,omitempty"`
}

// InAppPurchaseOfferCodeOneTimeUseCodeUpdateRequest represents a request to update a one-time-use code batch.
type InAppPurchaseOfferCodeOneTimeUseCodeUpdateRequest struct {
	Data InAppPurchaseOfferCodeOneTimeUseCodeUpdateData `json:"data"`
}

// InAppPurchaseOfferCodeOneTimeUseCodeUpdateData contains the data for updating a one-time-use code batch.
type InAppPurchaseOfferCodeOneTimeUseCodeUpdateData struct {
	Type       string                    `json:"type"`
	ID         string                    `json:"id"`
	Attributes OfferCodeActiveAttributes `json:"attributes"`
}

// In-app purchase pricing and availability types

// InAppPurchasePricePointsResponse represents a list of in-app purchase price points.
type InAppPurchasePricePointsResponse struct {
	Data     []InAppPurchasePricePoint `json:"data"`
	Links    PagedDocumentLinks        `json:"links"`
	Meta     *PagingInformation        `json:"meta,omitempty"`
	Included []any                     `json:"included,omitempty"`
}

// InAppPurchasePricePoint represents a price tier available to an in-app
// purchase in one territory.
type InAppPurchasePricePoint struct {
	Type       string                            `json:"type"`
	ID         string                            `json:"id"`
	Attributes InAppPurchasePricePointAttributes `json:"attributes"`
}

// InAppPurchasePricePointAttributes contains in-app purchase price point attributes.
type InAppPurchasePricePointAttributes struct {
	CustomerPrice string `json:"customerPrice,omitempty"`
	Proceeds      string `json:"proceeds,omitempty"`
}

// InAppPurchasePriceScheduleResponse represents a single in-app purchase price schedule.
type InAppPurchasePriceScheduleResponse struct {
	Data     InAppPurchasePriceSchedule `json:"data"`
	Included []any                      `json:"included,omitempty"`
}

// InAppPurchasePriceSchedule represents the price schedule of an in-app
// purchase: a base territory plus the manual and automatic prices
// derived from it.
type InAppPurchasePriceSchedule struct {
	Type string `json:"type"`
	ID   string `json:"id"`
}

// InAppPurchasePriceScheduleCreateRequest represents a request to set an
// in-app purchase's prices. Prices are declared inline through the
// manualPrices relationship and the included array.
type InAppPurchasePriceScheduleCreateRequest struct {
	Data     InAppPurchasePriceScheduleCreateData `json:"data"`
	Included []InAppPurchasePriceInlineCreate     `json:"included,omitempty"`
}

// InAppPurchasePriceScheduleCreateData contains the data for creating an in-app purchase price schedule.
type InAppPurchasePriceScheduleCreateData struct {
	Type          string                                        `json:"type"`
	Relationships InAppPurchasePriceScheduleCreateRelationships `json:"relationships"`
}

// InAppPurchasePriceScheduleCreateRelationships contains relationships for creating an in-app purchase price schedule.
type InAppPurchasePriceScheduleCreateRelationships struct {
	InAppPurchase RelationshipData     `json:"inAppPurchase"`
	BaseTerritory RelationshipData     `json:"baseTerritory"`
	ManualPrices  RelationshipDataList `json:"manualPrices"`
}

// InAppPurchasePriceInlineCreate represents a price declared inline in an
// in-app purchase price schedule create request.
type InAppPurchasePriceInlineCreate struct {
	Type          string                                       `json:"type"`
	ID            string                                       `json:"id,omitempty"`
	Attributes    *CommercePriceInlineAttributes               `json:"attributes,omitempty"`
	Relationships *InAppPurchasePriceInlineCreateRelationships `json:"relationships,omitempty"`
}

// CommercePriceInlineAttributes contains the scheduling window shared by
// inline app prices and in-app purchase prices. Omit both dates to make
// the price effective immediately and indefinitely.
type CommercePriceInlineAttributes struct {
	StartDate string `json:"startDate,omitempty"`
	EndDate   string `json:"endDate,omitempty"`
}

// InAppPurchasePriceInlineCreateRelationships contains relationships for an inline in-app purchase price.
type InAppPurchasePriceInlineCreateRelationships struct {
	InAppPurchasePricePoint RelationshipData `json:"inAppPurchasePricePoint"`
}

// InAppPurchasePricesResponse represents a list of in-app purchase prices.
type InAppPurchasePricesResponse struct {
	Data     []InAppPurchasePrice `json:"data"`
	Links    PagedDocumentLinks   `json:"links"`
	Meta     *PagingInformation   `json:"meta,omitempty"`
	Included []any                `json:"included,omitempty"`
}

// InAppPurchasePrice represents one scheduled price of an in-app purchase.
type InAppPurchasePrice struct {
	Type       string                       `json:"type"`
	ID         string                       `json:"id"`
	Attributes InAppPurchasePriceAttributes `json:"attributes"`
}

// InAppPurchasePriceAttributes contains in-app purchase price attributes.
type InAppPurchasePriceAttributes struct {
	StartDate string `json:"startDate,omitempty"`
	EndDate   string `json:"endDate,omitempty"`
	Manual    bool   `json:"manual,omitempty"`
}

// InAppPurchaseAvailabilityResponse represents in-app purchase availability.
type InAppPurchaseAvailabilityResponse struct {
	Data     InAppPurchaseAvailability `json:"data"`
	Included []any                     `json:"included,omitempty"`
}

// InAppPurchaseAvailability represents the territories an in-app purchase
// is available in.
type InAppPurchaseAvailability struct {
	Type       string                              `json:"type"`
	ID         string                              `json:"id"`
	Attributes InAppPurchaseAvailabilityAttributes `json:"attributes"`
}

// InAppPurchaseAvailabilityAttributes contains in-app purchase availability attributes.
type InAppPurchaseAvailabilityAttributes struct {
	AvailableInNewTerritories bool `json:"availableInNewTerritories"`
}

// InAppPurchaseAvailabilityCreateRequest represents a request to set an
// in-app purchase's territory availability.
type InAppPurchaseAvailabilityCreateRequest struct {
	Data InAppPurchaseAvailabilityCreateData `json:"data"`
}

// InAppPurchaseAvailabilityCreateData contains the data for creating in-app purchase availability.
type InAppPurchaseAvailabilityCreateData struct {
	Type          string                                       `json:"type"`
	Attributes    InAppPurchaseAvailabilityAttributes          `json:"attributes"`
	Relationships InAppPurchaseAvailabilityCreateRelationships `json:"relationships"`
}

// InAppPurchaseAvailabilityCreateRelationships contains relationships for creating in-app purchase availability.
type InAppPurchaseAvailabilityCreateRelationships struct {
	InAppPurchase        RelationshipData     `json:"inAppPurchase"`
	AvailableTerritories RelationshipDataList `json:"availableTerritories"`
}

// Subscription plan availability types (App Store Connect API 4.4)

// SubscriptionPlanAvailabilitiesResponse represents a list of subscription plan availabilities.
type SubscriptionPlanAvailabilitiesResponse struct {
	Data     []SubscriptionPlanAvailability `json:"data"`
	Links    PagedDocumentLinks             `json:"links"`
	Meta     *PagingInformation             `json:"meta,omitempty"`
	Included []any                          `json:"included,omitempty"`
}

// SubscriptionPlanAvailabilityResponse represents a single subscription plan availability.
type SubscriptionPlanAvailabilityResponse struct {
	Data     SubscriptionPlanAvailability `json:"data"`
	Included []any                        `json:"included,omitempty"`
}

// SubscriptionPlanAvailability represents the territories one billing
// plan of a subscription is available in. It replaces the deprecated
// subscriptionAvailability resource, which could not distinguish the
// monthly plan from the upfront (pre-paid) plan.
type SubscriptionPlanAvailability struct {
	Type       string                                 `json:"type"`
	ID         string                                 `json:"id"`
	Attributes SubscriptionPlanAvailabilityAttributes `json:"attributes"`
}

// SubscriptionPlanAvailabilityAttributes contains subscription plan availability attributes.
type SubscriptionPlanAvailabilityAttributes struct {
	AvailableInNewTerritories *bool  `json:"availableInNewTerritories,omitempty"`
	PlanType                  string `json:"planType,omitempty"`
}

// SubscriptionPlanAvailabilityCreateRequest represents a request to configure
// a subscription plan's territory availability.
type SubscriptionPlanAvailabilityCreateRequest struct {
	Data SubscriptionPlanAvailabilityCreateData `json:"data"`
}

// SubscriptionPlanAvailabilityCreateData contains the data for creating a subscription plan availability.
type SubscriptionPlanAvailabilityCreateData struct {
	Type          string                                          `json:"type"`
	Attributes    SubscriptionPlanAvailabilityAttributes          `json:"attributes"`
	Relationships SubscriptionPlanAvailabilityCreateRelationships `json:"relationships"`
}

// SubscriptionPlanAvailabilityCreateRelationships contains relationships for creating a subscription plan availability.
type SubscriptionPlanAvailabilityCreateRelationships struct {
	Subscription         RelationshipData     `json:"subscription"`
	AvailableTerritories RelationshipDataList `json:"availableTerritories"`
}

// SubscriptionPlanAvailabilityUpdateRequest represents a request to update a subscription plan availability.
type SubscriptionPlanAvailabilityUpdateRequest struct {
	Data SubscriptionPlanAvailabilityUpdateData `json:"data"`
}

// SubscriptionPlanAvailabilityUpdateData contains the data for updating a subscription plan availability.
type SubscriptionPlanAvailabilityUpdateData struct {
	Type          string                                           `json:"type"`
	ID            string                                           `json:"id"`
	Attributes    *SubscriptionPlanAvailabilityUpdateAttributes    `json:"attributes,omitempty"`
	Relationships *SubscriptionPlanAvailabilityUpdateRelationships `json:"relationships,omitempty"`
}

// SubscriptionPlanAvailabilityUpdateAttributes contains attributes for updating a subscription plan availability.
type SubscriptionPlanAvailabilityUpdateAttributes struct {
	AvailableInNewTerritories *bool `json:"availableInNewTerritories,omitempty"`
}

// SubscriptionPlanAvailabilityUpdateRelationships contains relationships for updating a subscription plan availability.
type SubscriptionPlanAvailabilityUpdateRelationships struct {
	AvailableTerritories *RelationshipDataList `json:"availableTerritories,omitempty"`
}

// App price schedule write types

// AppPriceScheduleCreateRequest represents a request to set an app's
// prices. The base territory anchors the automatic price conversions;
// manual prices are declared inline through the included array.
type AppPriceScheduleCreateRequest struct {
	Data     AppPriceScheduleCreateData `json:"data"`
	Included []AppPriceInlineCreate     `json:"included,omitempty"`
}

// AppPriceScheduleCreateData contains the data for creating an app price schedule.
type AppPriceScheduleCreateData struct {
	Type          string                              `json:"type"`
	Relationships AppPriceScheduleCreateRelationships `json:"relationships"`
}

// AppPriceScheduleCreateRelationships contains relationships for creating an app price schedule.
type AppPriceScheduleCreateRelationships struct {
	App           RelationshipData     `json:"app"`
	BaseTerritory RelationshipData     `json:"baseTerritory"`
	ManualPrices  RelationshipDataList `json:"manualPrices"`
}

// AppPriceInlineCreate represents an app price declared inline in an app
// price schedule create request.
type AppPriceInlineCreate struct {
	Type          string                             `json:"type"`
	ID            string                             `json:"id,omitempty"`
	Attributes    *CommercePriceInlineAttributes     `json:"attributes,omitempty"`
	Relationships *AppPriceInlineCreateRelationships `json:"relationships,omitempty"`
}

// AppPriceInlineCreateRelationships contains relationships for an inline app price.
type AppPriceInlineCreateRelationships struct {
	AppPricePoint RelationshipData `json:"appPricePoint"`
}

// In-app purchase version methods

// CreateInAppPurchaseVersion creates a new version of an in-app
// purchase, snapshotting its current metadata for editing.
func (c *Client) CreateInAppPurchaseVersion(ctx context.Context, req *InAppPurchaseVersionCreateRequest) (*InAppPurchaseVersionResponse, error) {
	data, err := c.Post(ctx, "/v1/inAppPurchaseVersions", req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetInAppPurchaseVersion returns a single in-app purchase version.
func (c *Client) GetInAppPurchaseVersion(ctx context.Context, versionID string) (*InAppPurchaseVersionResponse, error) {
	data, err := c.Get(ctx, "/v1/inAppPurchaseVersions/"+url.PathEscape(versionID), nil)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListInAppPurchaseVersions returns the versions of an in-app purchase.
func (c *Client) ListInAppPurchaseVersions(ctx context.Context, iapID string, opts *ListOptions) (*InAppPurchaseVersionsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v2/inAppPurchases/"+url.PathEscape(iapID)+"/versions", query)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseVersionsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListInAppPurchaseVersionLocalizations returns the localizations of an
// in-app purchase version.
func (c *Client) ListInAppPurchaseVersionLocalizations(ctx context.Context, versionID string, opts *ListOptions) (*InAppPurchaseLocalizationsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/inAppPurchaseVersions/"+url.PathEscape(versionID)+"/localizations", query)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseLocalizationsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListInAppPurchaseVersionImages returns the images of an in-app purchase version.
func (c *Client) ListInAppPurchaseVersionImages(ctx context.Context, versionID string, opts *ListOptions) (*InAppPurchaseImagesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/inAppPurchaseVersions/"+url.PathEscape(versionID)+"/images", query)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseImagesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateInAppPurchaseLocalization creates a localization on an in-app
// purchase version.
func (c *Client) CreateInAppPurchaseLocalization(ctx context.Context, req *InAppPurchaseLocalizationCreateRequest) (*InAppPurchaseLocalizationResponse, error) {
	data, err := c.Post(ctx, "/v2/inAppPurchaseLocalizations", req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateInAppPurchaseLocalization updates an in-app purchase localization.
func (c *Client) UpdateInAppPurchaseLocalization(ctx context.Context, localizationID string, req *InAppPurchaseLocalizationUpdateRequest) (*InAppPurchaseLocalizationResponse, error) {
	data, err := c.Patch(ctx, "/v2/inAppPurchaseLocalizations/"+url.PathEscape(localizationID), req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteInAppPurchaseLocalization deletes an in-app purchase localization.
func (c *Client) DeleteInAppPurchaseLocalization(ctx context.Context, localizationID string) error {
	return c.Delete(ctx, "/v2/inAppPurchaseLocalizations/"+url.PathEscape(localizationID))
}

// CreateInAppPurchaseImage reserves an image upload on an in-app
// purchase version. The response carries the upload operations to
// perform before committing with UpdateInAppPurchaseImage.
func (c *Client) CreateInAppPurchaseImage(ctx context.Context, req *InAppPurchaseImageCreateRequest) (*InAppPurchaseImageResponse, error) {
	data, err := c.Post(ctx, "/v2/inAppPurchaseImages", req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseImageResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateInAppPurchaseImage commits an in-app purchase image upload.
func (c *Client) UpdateInAppPurchaseImage(ctx context.Context, imageID string, req *InAppPurchaseImageUpdateRequest) (*InAppPurchaseImageResponse, error) {
	data, err := c.Patch(ctx, "/v2/inAppPurchaseImages/"+url.PathEscape(imageID), req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseImageResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteInAppPurchaseImage deletes an in-app purchase image.
func (c *Client) DeleteInAppPurchaseImage(ctx context.Context, imageID string) error {
	return c.Delete(ctx, "/v2/inAppPurchaseImages/"+url.PathEscape(imageID))
}

// Subscription version methods

// CreateSubscriptionVersion creates a new version of a subscription.
func (c *Client) CreateSubscriptionVersion(ctx context.Context, req *SubscriptionVersionCreateRequest) (*SubscriptionVersionResponse, error) {
	data, err := c.Post(ctx, "/v1/subscriptionVersions", req)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetSubscriptionVersion returns a single subscription version.
func (c *Client) GetSubscriptionVersion(ctx context.Context, versionID string) (*SubscriptionVersionResponse, error) {
	data, err := c.Get(ctx, "/v1/subscriptionVersions/"+url.PathEscape(versionID), nil)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListSubscriptionVersions returns the versions of a subscription.
func (c *Client) ListSubscriptionVersions(ctx context.Context, subscriptionID string, opts *ListOptions) (*SubscriptionVersionsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/subscriptions/"+url.PathEscape(subscriptionID)+"/versions", query)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionVersionsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListSubscriptionVersionLocalizations returns the localizations of a
// subscription version.
func (c *Client) ListSubscriptionVersionLocalizations(ctx context.Context, versionID string, opts *ListOptions) (*SubscriptionLocalizationsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/subscriptionVersions/"+url.PathEscape(versionID)+"/localizations", query)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionLocalizationsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListSubscriptionVersionImages returns the images of a subscription version.
func (c *Client) ListSubscriptionVersionImages(ctx context.Context, versionID string, opts *ListOptions) (*SubscriptionImagesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/subscriptionVersions/"+url.PathEscape(versionID)+"/images", query)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionImagesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateSubscriptionLocalization creates a localization on a subscription version.
func (c *Client) CreateSubscriptionLocalization(ctx context.Context, req *SubscriptionLocalizationCreateRequest) (*SubscriptionLocalizationResponse, error) {
	data, err := c.Post(ctx, "/v2/subscriptionLocalizations", req)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateSubscriptionLocalization updates a subscription localization.
func (c *Client) UpdateSubscriptionLocalization(ctx context.Context, localizationID string, req *SubscriptionLocalizationUpdateRequest) (*SubscriptionLocalizationResponse, error) {
	data, err := c.Patch(ctx, "/v2/subscriptionLocalizations/"+url.PathEscape(localizationID), req)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteSubscriptionLocalization deletes a subscription localization.
func (c *Client) DeleteSubscriptionLocalization(ctx context.Context, localizationID string) error {
	return c.Delete(ctx, "/v2/subscriptionLocalizations/"+url.PathEscape(localizationID))
}

// CreateSubscriptionImage reserves an image upload on a subscription version.
func (c *Client) CreateSubscriptionImage(ctx context.Context, req *SubscriptionImageCreateRequest) (*SubscriptionImageResponse, error) {
	data, err := c.Post(ctx, "/v2/subscriptionImages", req)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionImageResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateSubscriptionImage commits a subscription image upload.
func (c *Client) UpdateSubscriptionImage(ctx context.Context, imageID string, req *SubscriptionImageUpdateRequest) (*SubscriptionImageResponse, error) {
	data, err := c.Patch(ctx, "/v2/subscriptionImages/"+url.PathEscape(imageID), req)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionImageResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteSubscriptionImage deletes a subscription image.
func (c *Client) DeleteSubscriptionImage(ctx context.Context, imageID string) error {
	return c.Delete(ctx, "/v2/subscriptionImages/"+url.PathEscape(imageID))
}

// Subscription group version methods

// CreateSubscriptionGroupVersion creates a new version of a subscription group.
func (c *Client) CreateSubscriptionGroupVersion(ctx context.Context, req *SubscriptionGroupVersionCreateRequest) (*SubscriptionGroupVersionResponse, error) {
	data, err := c.Post(ctx, "/v1/subscriptionGroupVersions", req)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionGroupVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetSubscriptionGroupVersion returns a single subscription group version.
func (c *Client) GetSubscriptionGroupVersion(ctx context.Context, versionID string) (*SubscriptionGroupVersionResponse, error) {
	data, err := c.Get(ctx, "/v1/subscriptionGroupVersions/"+url.PathEscape(versionID), nil)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionGroupVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListSubscriptionGroupVersions returns the versions of a subscription group.
func (c *Client) ListSubscriptionGroupVersions(ctx context.Context, groupID string, opts *ListOptions) (*SubscriptionGroupVersionsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/subscriptionGroups/"+url.PathEscape(groupID)+"/versions", query)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionGroupVersionsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListSubscriptionGroupVersionLocalizations returns the localizations of
// a subscription group version.
func (c *Client) ListSubscriptionGroupVersionLocalizations(ctx context.Context, versionID string, opts *ListOptions) (*SubscriptionGroupLocalizationsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/subscriptionGroupVersions/"+url.PathEscape(versionID)+"/localizations", query)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionGroupLocalizationsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateSubscriptionGroupLocalization creates a localization on a
// subscription group version.
func (c *Client) CreateSubscriptionGroupLocalization(ctx context.Context, req *SubscriptionGroupLocalizationCreateRequest) (*SubscriptionGroupLocalizationResponse, error) {
	data, err := c.Post(ctx, "/v2/subscriptionGroupLocalizations", req)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionGroupLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateSubscriptionGroupLocalization updates a subscription group localization.
func (c *Client) UpdateSubscriptionGroupLocalization(ctx context.Context, localizationID string, req *SubscriptionGroupLocalizationUpdateRequest) (*SubscriptionGroupLocalizationResponse, error) {
	data, err := c.Patch(ctx, "/v2/subscriptionGroupLocalizations/"+url.PathEscape(localizationID), req)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionGroupLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteSubscriptionGroupLocalization deletes a subscription group localization.
func (c *Client) DeleteSubscriptionGroupLocalization(ctx context.Context, localizationID string) error {
	return c.Delete(ctx, "/v2/subscriptionGroupLocalizations/"+url.PathEscape(localizationID))
}

// In-app purchase offer code methods

// ListInAppPurchaseOfferCodes returns the offer codes of an in-app purchase.
func (c *Client) ListInAppPurchaseOfferCodes(ctx context.Context, iapID string, opts *ListOptions) (*InAppPurchaseOfferCodesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v2/inAppPurchases/"+url.PathEscape(iapID)+"/offerCodes", query)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseOfferCodesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetInAppPurchaseOfferCode returns a single in-app purchase offer code.
func (c *Client) GetInAppPurchaseOfferCode(ctx context.Context, offerCodeID string) (*InAppPurchaseOfferCodeResponse, error) {
	data, err := c.Get(ctx, "/v1/inAppPurchaseOfferCodes/"+url.PathEscape(offerCodeID), nil)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseOfferCodeResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateInAppPurchaseOfferCode creates an in-app purchase offer code.
func (c *Client) CreateInAppPurchaseOfferCode(ctx context.Context, req *InAppPurchaseOfferCodeCreateRequest) (*InAppPurchaseOfferCodeResponse, error) {
	data, err := c.Post(ctx, "/v1/inAppPurchaseOfferCodes", req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseOfferCodeResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateInAppPurchaseOfferCode activates or deactivates an in-app
// purchase offer code.
func (c *Client) UpdateInAppPurchaseOfferCode(ctx context.Context, offerCodeID string, req *InAppPurchaseOfferCodeUpdateRequest) (*InAppPurchaseOfferCodeResponse, error) {
	data, err := c.Patch(ctx, "/v1/inAppPurchaseOfferCodes/"+url.PathEscape(offerCodeID), req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseOfferCodeResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListInAppPurchaseOfferCodePrices returns the per-territory prices of an
// in-app purchase offer code.
func (c *Client) ListInAppPurchaseOfferCodePrices(ctx context.Context, offerCodeID string, opts *ListOptions) (*InAppPurchaseOfferPricesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/inAppPurchaseOfferCodes/"+url.PathEscape(offerCodeID)+"/prices", query)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseOfferPricesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListInAppPurchaseOfferCodeCustomCodes returns the custom code batches
// of an in-app purchase offer code.
func (c *Client) ListInAppPurchaseOfferCodeCustomCodes(ctx context.Context, offerCodeID string, opts *ListOptions) (*InAppPurchaseOfferCodeCustomCodesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/inAppPurchaseOfferCodes/"+url.PathEscape(offerCodeID)+"/customCodes", query)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseOfferCodeCustomCodesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateInAppPurchaseOfferCodeCustomCode creates a custom code batch.
func (c *Client) CreateInAppPurchaseOfferCodeCustomCode(ctx context.Context, req *InAppPurchaseOfferCodeCustomCodeCreateRequest) (*InAppPurchaseOfferCodeCustomCodeResponse, error) {
	data, err := c.Post(ctx, "/v1/inAppPurchaseOfferCodeCustomCodes", req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseOfferCodeCustomCodeResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateInAppPurchaseOfferCodeCustomCode activates or deactivates a
// custom code batch.
func (c *Client) UpdateInAppPurchaseOfferCodeCustomCode(ctx context.Context, customCodeID string, req *InAppPurchaseOfferCodeCustomCodeUpdateRequest) (*InAppPurchaseOfferCodeCustomCodeResponse, error) {
	data, err := c.Patch(ctx, "/v1/inAppPurchaseOfferCodeCustomCodes/"+url.PathEscape(customCodeID), req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseOfferCodeCustomCodeResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListInAppPurchaseOfferCodeOneTimeUseCodes returns the one-time-use code
// batches of an in-app purchase offer code.
func (c *Client) ListInAppPurchaseOfferCodeOneTimeUseCodes(ctx context.Context, offerCodeID string, opts *ListOptions) (*InAppPurchaseOfferCodeOneTimeUseCodesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/inAppPurchaseOfferCodes/"+url.PathEscape(offerCodeID)+"/oneTimeUseCodes", query)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseOfferCodeOneTimeUseCodesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateInAppPurchaseOfferCodeOneTimeUseCode creates a one-time-use code batch.
func (c *Client) CreateInAppPurchaseOfferCodeOneTimeUseCode(ctx context.Context, req *InAppPurchaseOfferCodeOneTimeUseCodeCreateRequest) (*InAppPurchaseOfferCodeOneTimeUseCodeResponse, error) {
	data, err := c.Post(ctx, "/v1/inAppPurchaseOfferCodeOneTimeUseCodes", req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseOfferCodeOneTimeUseCodeResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateInAppPurchaseOfferCodeOneTimeUseCode activates or deactivates a
// one-time-use code batch.
func (c *Client) UpdateInAppPurchaseOfferCodeOneTimeUseCode(ctx context.Context, codeID string, req *InAppPurchaseOfferCodeOneTimeUseCodeUpdateRequest) (*InAppPurchaseOfferCodeOneTimeUseCodeResponse, error) {
	data, err := c.Patch(ctx, "/v1/inAppPurchaseOfferCodeOneTimeUseCodes/"+url.PathEscape(codeID), req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseOfferCodeOneTimeUseCodeResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetInAppPurchaseOfferCodeOneTimeUseCodeValues downloads the generated
// codes of a one-time-use batch. Apple returns them as CSV, not JSON, so
// the raw body is handed back to the caller.
func (c *Client) GetInAppPurchaseOfferCodeOneTimeUseCodeValues(ctx context.Context, codeID string) (string, error) {
	data, err := c.Get(ctx, "/v1/inAppPurchaseOfferCodeOneTimeUseCodes/"+url.PathEscape(codeID)+"/values", nil)
	if err != nil {
		return "", err
	}

	return string(data), nil
}

// In-app purchase pricing and availability methods

// ListInAppPurchasePricePoints returns the price points available to an
// in-app purchase.
func (c *Client) ListInAppPurchasePricePoints(ctx context.Context, iapID string, opts *ListOptions) (*InAppPurchasePricePointsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v2/inAppPurchases/"+url.PathEscape(iapID)+"/pricePoints", query)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchasePricePointsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetInAppPurchasePriceSchedule returns the price schedule of an in-app purchase.
func (c *Client) GetInAppPurchasePriceSchedule(ctx context.Context, iapID string) (*InAppPurchasePriceScheduleResponse, error) {
	data, err := c.Get(ctx, "/v2/inAppPurchases/"+url.PathEscape(iapID)+"/iapPriceSchedule", nil)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchasePriceScheduleResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateInAppPurchasePriceSchedule replaces an in-app purchase's price schedule.
func (c *Client) CreateInAppPurchasePriceSchedule(ctx context.Context, req *InAppPurchasePriceScheduleCreateRequest) (*InAppPurchasePriceScheduleResponse, error) {
	data, err := c.Post(ctx, "/v1/inAppPurchasePriceSchedules", req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchasePriceScheduleResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListInAppPurchasePriceSchedulePrices returns the prices of an in-app
// purchase price schedule. Manual prices are the ones the developer set;
// automatic prices are the ones Apple derived for other territories.
func (c *Client) ListInAppPurchasePriceSchedulePrices(ctx context.Context, scheduleID string, automatic bool, opts *ListOptions) (*InAppPurchasePricesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	relationship := "manualPrices"
	if automatic {
		relationship = "automaticPrices"
	}

	data, err := c.Get(ctx, "/v1/inAppPurchasePriceSchedules/"+url.PathEscape(scheduleID)+"/"+relationship, query)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchasePricesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetInAppPurchaseAvailability returns the territory availability of an
// in-app purchase.
func (c *Client) GetInAppPurchaseAvailability(ctx context.Context, iapID string) (*InAppPurchaseAvailabilityResponse, error) {
	data, err := c.Get(ctx, "/v2/inAppPurchases/"+url.PathEscape(iapID)+"/inAppPurchaseAvailability", nil)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseAvailabilityResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateInAppPurchaseAvailability sets the territory availability of an
// in-app purchase.
func (c *Client) CreateInAppPurchaseAvailability(ctx context.Context, req *InAppPurchaseAvailabilityCreateRequest) (*InAppPurchaseAvailabilityResponse, error) {
	data, err := c.Post(ctx, "/v1/inAppPurchaseAvailabilities", req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseAvailabilityResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListInAppPurchaseAvailableTerritories returns the territories covered
// by an in-app purchase availability.
func (c *Client) ListInAppPurchaseAvailableTerritories(ctx context.Context, availabilityID string, opts *ListOptions) (*TerritoriesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/inAppPurchaseAvailabilities/"+url.PathEscape(availabilityID)+"/availableTerritories", query)
	if err != nil {
		return nil, err
	}

	var resp TerritoriesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// Subscription plan availability methods

// ListSubscriptionPlanAvailabilities returns the plan availabilities of a
// subscription, one per billing plan.
func (c *Client) ListSubscriptionPlanAvailabilities(ctx context.Context, subscriptionID string, opts *ListOptions) (*SubscriptionPlanAvailabilitiesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/subscriptions/"+url.PathEscape(subscriptionID)+"/planAvailabilities", query)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionPlanAvailabilitiesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetSubscriptionPlanAvailability returns a single subscription plan availability.
func (c *Client) GetSubscriptionPlanAvailability(ctx context.Context, availabilityID string) (*SubscriptionPlanAvailabilityResponse, error) {
	data, err := c.Get(ctx, "/v1/subscriptionPlanAvailabilities/"+url.PathEscape(availabilityID), nil)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionPlanAvailabilityResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateSubscriptionPlanAvailability configures the territories one
// billing plan of a subscription is available in.
func (c *Client) CreateSubscriptionPlanAvailability(ctx context.Context, req *SubscriptionPlanAvailabilityCreateRequest) (*SubscriptionPlanAvailabilityResponse, error) {
	data, err := c.Post(ctx, "/v1/subscriptionPlanAvailabilities", req)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionPlanAvailabilityResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateSubscriptionPlanAvailability updates a subscription plan availability.
func (c *Client) UpdateSubscriptionPlanAvailability(ctx context.Context, availabilityID string, req *SubscriptionPlanAvailabilityUpdateRequest) (*SubscriptionPlanAvailabilityResponse, error) {
	data, err := c.Patch(ctx, "/v1/subscriptionPlanAvailabilities/"+url.PathEscape(availabilityID), req)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionPlanAvailabilityResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListSubscriptionPlanAvailableTerritories returns the territories
// covered by a subscription plan availability.
func (c *Client) ListSubscriptionPlanAvailableTerritories(ctx context.Context, availabilityID string, opts *ListOptions) (*TerritoriesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/subscriptionPlanAvailabilities/"+url.PathEscape(availabilityID)+"/availableTerritories", query)
	if err != nil {
		return nil, err
	}

	var resp TerritoriesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// Subscription price point methods

// GetSubscriptionPricePoint returns a single subscription price point.
func (c *Client) GetSubscriptionPricePoint(ctx context.Context, pricePointID string) (*SubscriptionPricePointResponse, error) {
	data, err := c.Get(ctx, "/v1/subscriptionPricePoints/"+url.PathEscape(pricePointID), nil)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionPricePointResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListSubscriptionPricePointEqualizations returns the price points in
// other territories that are equivalent to the given one.
func (c *Client) ListSubscriptionPricePointEqualizations(ctx context.Context, pricePointID string, opts *ListOptions) (*SubscriptionPricePointsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/subscriptionPricePoints/"+url.PathEscape(pricePointID)+"/equalizations", query)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionPricePointsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListSubscriptionPricePointAdjustedEqualizations returns equalized price
// points adjusted for a pre-paid (upfront) plan, so a monthly price point
// can be matched against the upfront plan's price point
// (App Store Connect API 4.4.1). Pass filter[upfrontPricePointId] and
// filter[planType] through opts.
func (c *Client) ListSubscriptionPricePointAdjustedEqualizations(ctx context.Context, pricePointID string, opts *ListOptions) (*SubscriptionPricePointsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/subscriptionPricePoints/"+url.PathEscape(pricePointID)+"/adjustedEqualizations", query)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionPricePointsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// App price schedule and price point methods

// CreateAppPriceSchedule replaces an app's price schedule.
func (c *Client) CreateAppPriceSchedule(ctx context.Context, req *AppPriceScheduleCreateRequest) (*AppPriceScheduleResponse, error) {
	data, err := c.Post(ctx, "/v1/appPriceSchedules", req)
	if err != nil {
		return nil, err
	}

	var resp AppPriceScheduleResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetAppPricePoint returns a single app price point.
func (c *Client) GetAppPricePoint(ctx context.Context, pricePointID string) (*AppPricePointResponse, error) {
	data, err := c.Get(ctx, "/v3/appPricePoints/"+url.PathEscape(pricePointID), nil)
	if err != nil {
		return nil, err
	}

	var resp AppPricePointResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListAppPricePointEqualizations returns the app price points in other
// territories that are equivalent to the given one.
func (c *Client) ListAppPricePointEqualizations(ctx context.Context, pricePointID string, opts *ListOptions) (*AppPricePointsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v3/appPricePoints/"+url.PathEscape(pricePointID)+"/equalizations", query)
	if err != nil {
		return nil, err
	}

	var resp AppPricePointsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}
