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
