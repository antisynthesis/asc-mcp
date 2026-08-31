package api

import (
	"context"
	"encoding/json"
	"fmt"
	"net/url"
	"time"
)

// App Store metadata endpoints added in App Store Connect API 4.0-4.2:
// accessibility declarations, customer review summarizations, app tags,
// territory age ratings, and Android-to-iOS app mappings.

// Accessibility declaration types

// AccessibilityDeclarationsResponse represents a list of accessibility declarations.
type AccessibilityDeclarationsResponse struct {
	Data     []AccessibilityDeclaration `json:"data"`
	Links    PagedDocumentLinks         `json:"links"`
	Meta     *PagingInformation         `json:"meta,omitempty"`
	Included []any                      `json:"included,omitempty"`
}

// AccessibilityDeclarationResponse represents a single accessibility declaration.
type AccessibilityDeclarationResponse struct {
	Data AccessibilityDeclaration `json:"data"`
}

// AccessibilityDeclaration represents a per-device-family accessibility declaration.
type AccessibilityDeclaration struct {
	Type       string                             `json:"type"`
	ID         string                             `json:"id"`
	Attributes AccessibilityDeclarationAttributes `json:"attributes"`
}

// AccessibilityDeclarationAttributes contains accessibility declaration attributes.
type AccessibilityDeclarationAttributes struct {
	DeviceFamily                           string `json:"deviceFamily,omitempty"`
	State                                  string `json:"state,omitempty"`
	SupportsAudioDescriptions              *bool  `json:"supportsAudioDescriptions,omitempty"`
	SupportsCaptions                       *bool  `json:"supportsCaptions,omitempty"`
	SupportsDarkInterface                  *bool  `json:"supportsDarkInterface,omitempty"`
	SupportsDifferentiateWithoutColorAlone *bool  `json:"supportsDifferentiateWithoutColorAlone,omitempty"`
	SupportsLargerText                     *bool  `json:"supportsLargerText,omitempty"`
	SupportsReducedMotion                  *bool  `json:"supportsReducedMotion,omitempty"`
	SupportsSufficientContrast             *bool  `json:"supportsSufficientContrast,omitempty"`
	SupportsVoiceControl                   *bool  `json:"supportsVoiceControl,omitempty"`
	SupportsVoiceover                      *bool  `json:"supportsVoiceover,omitempty"`
}

// AccessibilityDeclarationCreateRequest represents a request to create an
// accessibility declaration.
type AccessibilityDeclarationCreateRequest struct {
	Data AccessibilityDeclarationCreateData `json:"data"`
}

// AccessibilityDeclarationCreateData contains the data for creating an
// accessibility declaration.
type AccessibilityDeclarationCreateData struct {
	Type          string                                      `json:"type"`
	Attributes    AccessibilityDeclarationCreateAttributes    `json:"attributes"`
	Relationships AccessibilityDeclarationCreateRelationships `json:"relationships"`
}

// AccessibilityDeclarationCreateAttributes contains attributes for creating
// an accessibility declaration. DeviceFamily is required.
type AccessibilityDeclarationCreateAttributes struct {
	DeviceFamily                           string `json:"deviceFamily"`
	SupportsAudioDescriptions              *bool  `json:"supportsAudioDescriptions,omitempty"`
	SupportsCaptions                       *bool  `json:"supportsCaptions,omitempty"`
	SupportsDarkInterface                  *bool  `json:"supportsDarkInterface,omitempty"`
	SupportsDifferentiateWithoutColorAlone *bool  `json:"supportsDifferentiateWithoutColorAlone,omitempty"`
	SupportsLargerText                     *bool  `json:"supportsLargerText,omitempty"`
	SupportsReducedMotion                  *bool  `json:"supportsReducedMotion,omitempty"`
	SupportsSufficientContrast             *bool  `json:"supportsSufficientContrast,omitempty"`
	SupportsVoiceControl                   *bool  `json:"supportsVoiceControl,omitempty"`
	SupportsVoiceover                      *bool  `json:"supportsVoiceover,omitempty"`
}

// AccessibilityDeclarationCreateRelationships contains relationships for
// creating an accessibility declaration.
type AccessibilityDeclarationCreateRelationships struct {
	App RelationshipData `json:"app"`
}

// AccessibilityDeclarationUpdateRequest represents a request to update an
// accessibility declaration.
type AccessibilityDeclarationUpdateRequest struct {
	Data AccessibilityDeclarationUpdateData `json:"data"`
}

// AccessibilityDeclarationUpdateData contains the data for updating an
// accessibility declaration.
type AccessibilityDeclarationUpdateData struct {
	Type       string                                    `json:"type"`
	ID         string                                    `json:"id"`
	Attributes *AccessibilityDeclarationUpdateAttributes `json:"attributes,omitempty"`
}

// AccessibilityDeclarationUpdateAttributes contains attributes for updating
// an accessibility declaration. Setting Publish to true publishes the draft
// declaration to the App Store.
type AccessibilityDeclarationUpdateAttributes struct {
	Publish                                *bool `json:"publish,omitempty"`
	SupportsAudioDescriptions              *bool `json:"supportsAudioDescriptions,omitempty"`
	SupportsCaptions                       *bool `json:"supportsCaptions,omitempty"`
	SupportsDarkInterface                  *bool `json:"supportsDarkInterface,omitempty"`
	SupportsDifferentiateWithoutColorAlone *bool `json:"supportsDifferentiateWithoutColorAlone,omitempty"`
	SupportsLargerText                     *bool `json:"supportsLargerText,omitempty"`
	SupportsReducedMotion                  *bool `json:"supportsReducedMotion,omitempty"`
	SupportsSufficientContrast             *bool `json:"supportsSufficientContrast,omitempty"`
	SupportsVoiceControl                   *bool `json:"supportsVoiceControl,omitempty"`
	SupportsVoiceover                      *bool `json:"supportsVoiceover,omitempty"`
}

// Customer review summarization types

// CustomerReviewSummarizationsResponse represents a list of customer review
// summarizations.
type CustomerReviewSummarizationsResponse struct {
	Data     []CustomerReviewSummarization `json:"data"`
	Links    PagedDocumentLinks            `json:"links"`
	Meta     *PagingInformation            `json:"meta,omitempty"`
	Included []any                         `json:"included,omitempty"`
}

// CustomerReviewSummarization represents an Apple-generated summary of
// customer reviews for a platform, locale, and territory.
type CustomerReviewSummarization struct {
	Type       string                                `json:"type"`
	ID         string                                `json:"id"`
	Attributes CustomerReviewSummarizationAttributes `json:"attributes"`
}

// CustomerReviewSummarizationAttributes contains review summarization attributes.
type CustomerReviewSummarizationAttributes struct {
	CreatedDate *time.Time `json:"createdDate,omitempty"`
	Locale      string     `json:"locale,omitempty"`
	Platform    string     `json:"platform,omitempty"`
	Text        string     `json:"text,omitempty"`
}

// App tag types

// AppTagsResponse represents a list of app tags.
type AppTagsResponse struct {
	Data     []AppTag           `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// AppTagResponse represents a single app tag.
type AppTagResponse struct {
	Data AppTag `json:"data"`
}

// AppTag represents an Apple-created tag applied to an app.
type AppTag struct {
	Type       string           `json:"type"`
	ID         string           `json:"id"`
	Attributes AppTagAttributes `json:"attributes"`
}

// AppTagAttributes contains app tag attributes.
type AppTagAttributes struct {
	Name              string `json:"name,omitempty"`
	VisibleInAppStore *bool  `json:"visibleInAppStore,omitempty"`
}

// AppTagUpdateRequest represents a request to update an app tag's visibility.
type AppTagUpdateRequest struct {
	Data AppTagUpdateData `json:"data"`
}

// AppTagUpdateData contains the data for updating an app tag.
type AppTagUpdateData struct {
	Type       string                 `json:"type"`
	ID         string                 `json:"id"`
	Attributes AppTagUpdateAttributes `json:"attributes"`
}

// AppTagUpdateAttributes contains attributes for updating an app tag.
// Setting VisibleInAppStore to false opts the app out of the tag.
type AppTagUpdateAttributes struct {
	VisibleInAppStore *bool `json:"visibleInAppStore,omitempty"`
}

// Territory age rating types

// TerritoryAgeRatingsResponse represents a list of territory age ratings.
type TerritoryAgeRatingsResponse struct {
	Data     []TerritoryAgeRating `json:"data"`
	Links    PagedDocumentLinks   `json:"links"`
	Meta     *PagingInformation   `json:"meta,omitempty"`
	Included []any                `json:"included,omitempty"`
}

// TerritoryAgeRating represents an app's age rating in a specific territory.
type TerritoryAgeRating struct {
	Type       string                       `json:"type"`
	ID         string                       `json:"id"`
	Attributes TerritoryAgeRatingAttributes `json:"attributes"`
}

// TerritoryAgeRatingAttributes contains territory age rating attributes.
type TerritoryAgeRatingAttributes struct {
	AppStoreAgeRating string `json:"appStoreAgeRating,omitempty"`
}

// Android-to-iOS app mapping types

// AndroidToIosAppMappingDetailsResponse represents a list of Android-to-iOS
// app mapping details.
type AndroidToIosAppMappingDetailsResponse struct {
	Data     []AndroidToIosAppMappingDetail `json:"data"`
	Links    PagedDocumentLinks             `json:"links"`
	Meta     *PagingInformation             `json:"meta,omitempty"`
	Included []any                          `json:"included,omitempty"`
}

// AndroidToIosAppMappingDetailResponse represents a single Android-to-iOS
// app mapping detail.
type AndroidToIosAppMappingDetailResponse struct {
	Data AndroidToIosAppMappingDetail `json:"data"`
}

// AndroidToIosAppMappingDetail maps an Android package to an iOS app.
type AndroidToIosAppMappingDetail struct {
	Type       string                                 `json:"type"`
	ID         string                                 `json:"id"`
	Attributes AndroidToIosAppMappingDetailAttributes `json:"attributes"`
}

// AndroidToIosAppMappingDetailAttributes contains mapping detail attributes.
type AndroidToIosAppMappingDetailAttributes struct {
	PackageName                                      string   `json:"packageName,omitempty"`
	AppSigningKeyPublicCertificateSha256Fingerprints []string `json:"appSigningKeyPublicCertificateSha256Fingerprints,omitempty"`
}

// AndroidToIosAppMappingDetailCreateRequest represents a request to create an
// Android-to-iOS app mapping.
type AndroidToIosAppMappingDetailCreateRequest struct {
	Data AndroidToIosAppMappingDetailCreateData `json:"data"`
}

// AndroidToIosAppMappingDetailCreateData contains the data for creating a mapping.
type AndroidToIosAppMappingDetailCreateData struct {
	Type          string                                          `json:"type"`
	Attributes    AndroidToIosAppMappingDetailCreateAttributes    `json:"attributes"`
	Relationships AndroidToIosAppMappingDetailCreateRelationships `json:"relationships"`
}

// AndroidToIosAppMappingDetailCreateAttributes contains attributes for
// creating a mapping. Both fields are required by the API.
type AndroidToIosAppMappingDetailCreateAttributes struct {
	PackageName                                      string   `json:"packageName"`
	AppSigningKeyPublicCertificateSha256Fingerprints []string `json:"appSigningKeyPublicCertificateSha256Fingerprints"`
}

// AndroidToIosAppMappingDetailCreateRelationships contains relationships for
// creating a mapping.
type AndroidToIosAppMappingDetailCreateRelationships struct {
	App RelationshipData `json:"app"`
}

// AndroidToIosAppMappingDetailUpdateRequest represents a request to update an
// Android-to-iOS app mapping.
type AndroidToIosAppMappingDetailUpdateRequest struct {
	Data AndroidToIosAppMappingDetailUpdateData `json:"data"`
}

// AndroidToIosAppMappingDetailUpdateData contains the data for updating a mapping.
type AndroidToIosAppMappingDetailUpdateData struct {
	Type       string                                        `json:"type"`
	ID         string                                        `json:"id"`
	Attributes *AndroidToIosAppMappingDetailUpdateAttributes `json:"attributes,omitempty"`
}

// AndroidToIosAppMappingDetailUpdateAttributes contains attributes for
// updating a mapping.
type AndroidToIosAppMappingDetailUpdateAttributes struct {
	PackageName                                      string   `json:"packageName,omitempty"`
	AppSigningKeyPublicCertificateSha256Fingerprints []string `json:"appSigningKeyPublicCertificateSha256Fingerprints,omitempty"`
}

// Accessibility declaration methods

// ListAccessibilityDeclarations returns the accessibility declarations for an app.
func (c *Client) ListAccessibilityDeclarations(ctx context.Context, appID string, opts *ListOptions) (*AccessibilityDeclarationsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/apps/"+url.PathEscape(appID)+"/accessibilityDeclarations", query)
	if err != nil {
		return nil, err
	}

	var resp AccessibilityDeclarationsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetAccessibilityDeclaration returns a single accessibility declaration.
func (c *Client) GetAccessibilityDeclaration(ctx context.Context, declarationID string) (*AccessibilityDeclarationResponse, error) {
	data, err := c.Get(ctx, "/v1/accessibilityDeclarations/"+url.PathEscape(declarationID), nil)
	if err != nil {
		return nil, err
	}

	var resp AccessibilityDeclarationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateAccessibilityDeclaration creates an accessibility declaration for an
// app and device family.
func (c *Client) CreateAccessibilityDeclaration(ctx context.Context, req *AccessibilityDeclarationCreateRequest) (*AccessibilityDeclarationResponse, error) {
	data, err := c.Post(ctx, "/v1/accessibilityDeclarations", req)
	if err != nil {
		return nil, err
	}

	var resp AccessibilityDeclarationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateAccessibilityDeclaration updates an accessibility declaration.
// Setting publish=true in the request publishes the draft declaration.
func (c *Client) UpdateAccessibilityDeclaration(ctx context.Context, declarationID string, req *AccessibilityDeclarationUpdateRequest) (*AccessibilityDeclarationResponse, error) {
	data, err := c.Patch(ctx, "/v1/accessibilityDeclarations/"+url.PathEscape(declarationID), req)
	if err != nil {
		return nil, err
	}

	var resp AccessibilityDeclarationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteAccessibilityDeclaration deletes an accessibility declaration.
func (c *Client) DeleteAccessibilityDeclaration(ctx context.Context, declarationID string) error {
	return c.Delete(ctx, "/v1/accessibilityDeclarations/"+url.PathEscape(declarationID))
}

// Customer review summarization methods

// ListCustomerReviewSummarizations returns Apple-generated summaries of an
// app's customer reviews.
func (c *Client) ListCustomerReviewSummarizations(ctx context.Context, appID string, opts *ListOptions) (*CustomerReviewSummarizationsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/apps/"+url.PathEscape(appID)+"/customerReviewSummarizations", query)
	if err != nil {
		return nil, err
	}

	var resp CustomerReviewSummarizationsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// App tag methods

// ListAppTags returns the Apple-created tags applied to an app.
func (c *Client) ListAppTags(ctx context.Context, appID string, opts *ListOptions) (*AppTagsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/apps/"+url.PathEscape(appID)+"/appTags", query)
	if err != nil {
		return nil, err
	}

	var resp AppTagsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateAppTag updates an app tag's App Store visibility (opt-out).
func (c *Client) UpdateAppTag(ctx context.Context, appTagID string, req *AppTagUpdateRequest) (*AppTagResponse, error) {
	data, err := c.Patch(ctx, "/v1/appTags/"+url.PathEscape(appTagID), req)
	if err != nil {
		return nil, err
	}

	var resp AppTagResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListAppTagTerritories returns the territories where an app tag applies.
func (c *Client) ListAppTagTerritories(ctx context.Context, appTagID string, opts *ListOptions) (*TerritoriesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/appTags/"+url.PathEscape(appTagID)+"/territories", query)
	if err != nil {
		return nil, err
	}

	var resp TerritoriesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// Territory age rating methods

// ListTerritoryAgeRatings returns an app's age ratings per territory for an
// app info.
func (c *Client) ListTerritoryAgeRatings(ctx context.Context, appInfoID string, opts *ListOptions) (*TerritoryAgeRatingsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/appInfos/"+url.PathEscape(appInfoID)+"/territoryAgeRatings", query)
	if err != nil {
		return nil, err
	}

	var resp TerritoryAgeRatingsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// Android-to-iOS app mapping methods

// ListAndroidToIosAppMappingDetails returns the Android-to-iOS app mappings
// for an app.
func (c *Client) ListAndroidToIosAppMappingDetails(ctx context.Context, appID string, opts *ListOptions) (*AndroidToIosAppMappingDetailsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/apps/"+url.PathEscape(appID)+"/androidToIosAppMappingDetails", query)
	if err != nil {
		return nil, err
	}

	var resp AndroidToIosAppMappingDetailsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetAndroidToIosAppMappingDetail returns a single Android-to-iOS app mapping.
func (c *Client) GetAndroidToIosAppMappingDetail(ctx context.Context, mappingID string) (*AndroidToIosAppMappingDetailResponse, error) {
	data, err := c.Get(ctx, "/v1/androidToIosAppMappingDetails/"+url.PathEscape(mappingID), nil)
	if err != nil {
		return nil, err
	}

	var resp AndroidToIosAppMappingDetailResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateAndroidToIosAppMappingDetail creates an Android-to-iOS app mapping.
func (c *Client) CreateAndroidToIosAppMappingDetail(ctx context.Context, req *AndroidToIosAppMappingDetailCreateRequest) (*AndroidToIosAppMappingDetailResponse, error) {
	data, err := c.Post(ctx, "/v1/androidToIosAppMappingDetails", req)
	if err != nil {
		return nil, err
	}

	var resp AndroidToIosAppMappingDetailResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateAndroidToIosAppMappingDetail updates an Android-to-iOS app mapping.
func (c *Client) UpdateAndroidToIosAppMappingDetail(ctx context.Context, mappingID string, req *AndroidToIosAppMappingDetailUpdateRequest) (*AndroidToIosAppMappingDetailResponse, error) {
	data, err := c.Patch(ctx, "/v1/androidToIosAppMappingDetails/"+url.PathEscape(mappingID), req)
	if err != nil {
		return nil, err
	}

	var resp AndroidToIosAppMappingDetailResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteAndroidToIosAppMappingDetail deletes an Android-to-iOS app mapping.
func (c *Client) DeleteAndroidToIosAppMappingDetail(ctx context.Context, mappingID string) error {
	return c.Delete(ctx, "/v1/androidToIosAppMappingDetails/"+url.PathEscape(mappingID))
}
