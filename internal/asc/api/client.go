// Package api provides the HTTP client for App Store Connect API.
package api

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strings"
	"time"
)

const (
	// BaseURL is the App Store Connect API base URL.
	BaseURL = "https://api.appstoreconnect.apple.com"

	// DefaultTimeout is the default HTTP request timeout.
	DefaultTimeout = 30 * time.Second
)

// Client is an HTTP client for the App Store Connect API.
type Client struct {
	httpClient    *http.Client
	tokenProvider *TokenProvider
	baseURL       string
}

// NewClient creates a new App Store Connect API client.
func NewClient(issuerID, keyID, privateKeyPath string) (*Client, error) {
	tokenProvider, err := NewTokenProvider(issuerID, keyID, privateKeyPath)
	if err != nil {
		return nil, fmt.Errorf("failed to create token provider: %w", err)
	}

	return &Client{
		httpClient: &http.Client{
			Timeout: DefaultTimeout,
		},
		tokenProvider: tokenProvider,
		baseURL:       BaseURL,
	}, nil
}

// doRequest performs an HTTP request with authentication.
func (c *Client) doRequest(ctx context.Context, method, path string, query url.Values, body any) ([]byte, error) {
	token, err := c.tokenProvider.GetToken()
	if err != nil {
		return nil, fmt.Errorf("failed to get token: %w", err)
	}

	reqURL := c.baseURL + path
	if query != nil && len(query) > 0 {
		reqURL = reqURL + "?" + query.Encode()
	}

	var bodyReader io.Reader
	if body != nil {
		bodyData, err := json.Marshal(body)
		if err != nil {
			return nil, fmt.Errorf("failed to marshal request body: %w", err)
		}
		bodyReader = bytes.NewReader(bodyData)
	}

	req, err := http.NewRequestWithContext(ctx, method, reqURL, bodyReader)
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")

	resp, err := c.httpClient.Do(req)
	if err != nil {
		return nil, fmt.Errorf("request failed: %w", err)
	}
	defer resp.Body.Close()

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response: %w", err)
	}

	if resp.StatusCode >= 400 {
		var errResp ErrorResponse
		if err := json.Unmarshal(respBody, &errResp); err == nil && len(errResp.Errors) > 0 {
			errMsgs := make([]string, 0, len(errResp.Errors))
			for _, e := range errResp.Errors {
				errMsgs = append(errMsgs, fmt.Sprintf("%s: %s", e.Title, e.Detail))
			}
			return nil, fmt.Errorf("API error (%d): %s", resp.StatusCode, strings.Join(errMsgs, "; "))
		}
		return nil, fmt.Errorf("API error (%d): %s", resp.StatusCode, string(respBody))
	}

	return respBody, nil
}

// Get performs a GET request.
func (c *Client) Get(ctx context.Context, path string, query url.Values) ([]byte, error) {
	return c.doRequest(ctx, http.MethodGet, path, query, nil)
}

// Post performs a POST request.
func (c *Client) Post(ctx context.Context, path string, body any) ([]byte, error) {
	return c.doRequest(ctx, http.MethodPost, path, nil, body)
}

// Patch performs a PATCH request.
func (c *Client) Patch(ctx context.Context, path string, body any) ([]byte, error) {
	return c.doRequest(ctx, http.MethodPatch, path, nil, body)
}

// Delete performs a DELETE request.
func (c *Client) Delete(ctx context.Context, path string) error {
	_, err := c.doRequest(ctx, http.MethodDelete, path, nil, nil)
	return err
}

// Apps API methods

// ListApps returns a list of apps.
func (c *Client) ListApps(ctx context.Context, limit int) (*AppsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/apps", query)
	if err != nil {
		return nil, err
	}

	var resp AppsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetApp returns a single app by ID.
func (c *Client) GetApp(ctx context.Context, appID string) (*AppResponse, error) {
	data, err := c.Get(ctx, "/v1/apps/"+appID, nil)
	if err != nil {
		return nil, err
	}

	var resp AppResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetAppVersions returns versions for an app.
func (c *Client) GetAppVersions(ctx context.Context, appID string, limit int) (*AppStoreVersionsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/apps/"+appID+"/appStoreVersions", query)
	if err != nil {
		return nil, err
	}

	var resp AppStoreVersionsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// Builds API methods

// ListBuilds returns a list of builds.
func (c *Client) ListBuilds(ctx context.Context, appID string, limit int) (*BuildsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}
	if appID != "" {
		query.Set("filter[app]", appID)
	}

	data, err := c.Get(ctx, "/v1/builds", query)
	if err != nil {
		return nil, err
	}

	var resp BuildsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetBuild returns a single build by ID.
func (c *Client) GetBuild(ctx context.Context, buildID string) (*BuildResponse, error) {
	data, err := c.Get(ctx, "/v1/builds/"+buildID, nil)
	if err != nil {
		return nil, err
	}

	var resp BuildResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// Beta Groups API methods

// ListBetaGroups returns a list of beta groups.
func (c *Client) ListBetaGroups(ctx context.Context, appID string, limit int) (*BetaGroupsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}
	if appID != "" {
		query.Set("filter[app]", appID)
	}

	data, err := c.Get(ctx, "/v1/betaGroups", query)
	if err != nil {
		return nil, err
	}

	var resp BetaGroupsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateBetaGroup creates a new beta group.
func (c *Client) CreateBetaGroup(ctx context.Context, req *BetaGroupCreateRequest) (*BetaGroupResponse, error) {
	data, err := c.Post(ctx, "/v1/betaGroups", req)
	if err != nil {
		return nil, err
	}

	var resp BetaGroupResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteBetaGroup deletes a beta group.
func (c *Client) DeleteBetaGroup(ctx context.Context, betaGroupID string) error {
	return c.Delete(ctx, "/v1/betaGroups/"+betaGroupID)
}

// Beta Testers API methods

// ListBetaTesters returns a list of beta testers.
func (c *Client) ListBetaTesters(ctx context.Context, betaGroupID string, limit int) (*BetaTestersResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}
	if betaGroupID != "" {
		query.Set("filter[betaGroups]", betaGroupID)
	}

	data, err := c.Get(ctx, "/v1/betaTesters", query)
	if err != nil {
		return nil, err
	}

	var resp BetaTestersResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateBetaTester invites a new beta tester.
func (c *Client) CreateBetaTester(ctx context.Context, req *BetaTesterCreateRequest) (*BetaTesterResponse, error) {
	data, err := c.Post(ctx, "/v1/betaTesters", req)
	if err != nil {
		return nil, err
	}

	var resp BetaTesterResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteBetaTester removes a beta tester.
func (c *Client) DeleteBetaTester(ctx context.Context, betaTesterID string) error {
	return c.Delete(ctx, "/v1/betaTesters/"+betaTesterID)
}

// AddBetaTesterToGroup adds a beta tester to a group.
func (c *Client) AddBetaTesterToGroup(ctx context.Context, betaGroupID, betaTesterID string) error {
	body := map[string]any{
		"data": []map[string]string{
			{
				"type": "betaTesters",
				"id":   betaTesterID,
			},
		},
	}

	_, err := c.Post(ctx, "/v1/betaGroups/"+betaGroupID+"/relationships/betaTesters", body)
	return err
}

// RemoveBetaTesterFromGroup removes a beta tester from a group.
func (c *Client) RemoveBetaTesterFromGroup(ctx context.Context, betaGroupID, betaTesterID string) error {
	// This requires a DELETE with a body, which is non-standard
	// For now, we use the delete beta tester endpoint
	return c.Delete(ctx, "/v1/betaGroups/"+betaGroupID+"/relationships/betaTesters")
}

// Bundle IDs API methods

// ListBundleIDs returns a list of bundle IDs.
func (c *Client) ListBundleIDs(ctx context.Context, limit int) (*BundleIDsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/bundleIds", query)
	if err != nil {
		return nil, err
	}

	var resp BundleIDsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetBundleID returns a single bundle ID by ID.
func (c *Client) GetBundleID(ctx context.Context, bundleIDID string) (*BundleIDResponse, error) {
	data, err := c.Get(ctx, "/v1/bundleIds/"+bundleIDID, nil)
	if err != nil {
		return nil, err
	}

	var resp BundleIDResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// Devices API methods

// ListDevices returns a list of devices.
func (c *Client) ListDevices(ctx context.Context, limit int) (*DevicesResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/devices", query)
	if err != nil {
		return nil, err
	}

	var resp DevicesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// RegisterDevice registers a new device.
func (c *Client) RegisterDevice(ctx context.Context, req *DeviceCreateRequest) (*DeviceResponse, error) {
	data, err := c.Post(ctx, "/v1/devices", req)
	if err != nil {
		return nil, err
	}

	var resp DeviceResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// Certificates API methods

// ListCertificates returns a list of certificates.
func (c *Client) ListCertificates(ctx context.Context, limit int) (*CertificatesResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/certificates", query)
	if err != nil {
		return nil, err
	}

	var resp CertificatesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// Profiles API methods

// ListProfiles returns a list of provisioning profiles.
func (c *Client) ListProfiles(ctx context.Context, limit int) (*ProfilesResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/profiles", query)
	if err != nil {
		return nil, err
	}

	var resp ProfilesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetProfile returns a single profile by ID.
func (c *Client) GetProfile(ctx context.Context, profileID string) (*ProfileResponse, error) {
	data, err := c.Get(ctx, "/v1/profiles/"+profileID, nil)
	if err != nil {
		return nil, err
	}

	var resp ProfileResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// App Info API methods

// GetAppInfos returns app infos for an app.
func (c *Client) GetAppInfos(ctx context.Context, appID string) (*AppInfosResponse, error) {
	data, err := c.Get(ctx, "/v1/apps/"+appID+"/appInfos", nil)
	if err != nil {
		return nil, err
	}

	var resp AppInfosResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// App Info Localization API methods

// ListAppInfoLocalizations returns localizations for an app info.
func (c *Client) ListAppInfoLocalizations(ctx context.Context, appInfoID string) (*AppInfoLocalizationsResponse, error) {
	data, err := c.Get(ctx, "/v1/appInfos/"+appInfoID+"/appInfoLocalizations", nil)
	if err != nil {
		return nil, err
	}

	var resp AppInfoLocalizationsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetAppInfoLocalization returns a single app info localization by ID.
func (c *Client) GetAppInfoLocalization(ctx context.Context, localizationID string) (*AppInfoLocalizationResponse, error) {
	data, err := c.Get(ctx, "/v1/appInfoLocalizations/"+localizationID, nil)
	if err != nil {
		return nil, err
	}

	var resp AppInfoLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateAppInfoLocalization creates a new app info localization.
func (c *Client) CreateAppInfoLocalization(ctx context.Context, req *AppInfoLocalizationCreateRequest) (*AppInfoLocalizationResponse, error) {
	data, err := c.Post(ctx, "/v1/appInfoLocalizations", req)
	if err != nil {
		return nil, err
	}

	var resp AppInfoLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateAppInfoLocalization updates an app info localization.
func (c *Client) UpdateAppInfoLocalization(ctx context.Context, localizationID string, req *AppInfoLocalizationUpdateRequest) (*AppInfoLocalizationResponse, error) {
	data, err := c.Patch(ctx, "/v1/appInfoLocalizations/"+localizationID, req)
	if err != nil {
		return nil, err
	}

	var resp AppInfoLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteAppInfoLocalization deletes an app info localization.
func (c *Client) DeleteAppInfoLocalization(ctx context.Context, localizationID string) error {
	return c.Delete(ctx, "/v1/appInfoLocalizations/"+localizationID)
}

// App Store Version Localization API methods

// ListAppStoreVersionLocalizations returns localizations for a version.
func (c *Client) ListAppStoreVersionLocalizations(ctx context.Context, versionID string) (*AppStoreVersionLocalizationsResponse, error) {
	data, err := c.Get(ctx, "/v1/appStoreVersions/"+versionID+"/appStoreVersionLocalizations", nil)
	if err != nil {
		return nil, err
	}

	var resp AppStoreVersionLocalizationsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetAppStoreVersionLocalization returns a single version localization by ID.
func (c *Client) GetAppStoreVersionLocalization(ctx context.Context, localizationID string) (*AppStoreVersionLocalizationResponse, error) {
	data, err := c.Get(ctx, "/v1/appStoreVersionLocalizations/"+localizationID, nil)
	if err != nil {
		return nil, err
	}

	var resp AppStoreVersionLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateAppStoreVersionLocalization creates a new version localization.
func (c *Client) CreateAppStoreVersionLocalization(ctx context.Context, req *AppStoreVersionLocalizationCreateRequest) (*AppStoreVersionLocalizationResponse, error) {
	data, err := c.Post(ctx, "/v1/appStoreVersionLocalizations", req)
	if err != nil {
		return nil, err
	}

	var resp AppStoreVersionLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateAppStoreVersionLocalization updates a version localization.
func (c *Client) UpdateAppStoreVersionLocalization(ctx context.Context, localizationID string, req *AppStoreVersionLocalizationUpdateRequest) (*AppStoreVersionLocalizationResponse, error) {
	data, err := c.Patch(ctx, "/v1/appStoreVersionLocalizations/"+localizationID, req)
	if err != nil {
		return nil, err
	}

	var resp AppStoreVersionLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteAppStoreVersionLocalization deletes a version localization.
func (c *Client) DeleteAppStoreVersionLocalization(ctx context.Context, localizationID string) error {
	return c.Delete(ctx, "/v1/appStoreVersionLocalizations/"+localizationID)
}

// Customer Reviews API methods

// ListCustomerReviews returns customer reviews for an app.
func (c *Client) ListCustomerReviews(ctx context.Context, appID string, limit int) (*CustomerReviewsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/apps/"+appID+"/customerReviews", query)
	if err != nil {
		return nil, err
	}

	var resp CustomerReviewsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetCustomerReview returns a single customer review by ID.
func (c *Client) GetCustomerReview(ctx context.Context, reviewID string) (*CustomerReviewResponse, error) {
	data, err := c.Get(ctx, "/v1/customerReviews/"+reviewID, nil)
	if err != nil {
		return nil, err
	}

	var resp CustomerReviewResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateCustomerReviewResponse creates a response to a customer review.
func (c *Client) CreateCustomerReviewResponse(ctx context.Context, req *CustomerReviewResponseCreateRequest) (*CustomerReviewResponseV1Response, error) {
	data, err := c.Post(ctx, "/v1/customerReviewResponses", req)
	if err != nil {
		return nil, err
	}

	var resp CustomerReviewResponseV1Response
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteCustomerReviewResponse deletes a customer review response.
func (c *Client) DeleteCustomerReviewResponse(ctx context.Context, responseID string) error {
	return c.Delete(ctx, "/v1/customerReviewResponses/"+responseID)
}

// In-App Purchases API methods

// ListInAppPurchases returns in-app purchases for an app.
func (c *Client) ListInAppPurchases(ctx context.Context, appID string, limit int) (*InAppPurchasesResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v2/apps/"+appID+"/inAppPurchasesV2", query)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchasesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetInAppPurchase returns a single in-app purchase by ID.
func (c *Client) GetInAppPurchase(ctx context.Context, iapID string) (*InAppPurchaseResponse, error) {
	data, err := c.Get(ctx, "/v2/inAppPurchases/"+iapID, nil)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateInAppPurchase creates a new in-app purchase.
func (c *Client) CreateInAppPurchase(ctx context.Context, req *InAppPurchaseCreateRequest) (*InAppPurchaseResponse, error) {
	data, err := c.Post(ctx, "/v2/inAppPurchases", req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateInAppPurchase updates an in-app purchase.
func (c *Client) UpdateInAppPurchase(ctx context.Context, iapID string, req *InAppPurchaseUpdateRequest) (*InAppPurchaseResponse, error) {
	data, err := c.Patch(ctx, "/v2/inAppPurchases/"+iapID, req)
	if err != nil {
		return nil, err
	}

	var resp InAppPurchaseResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteInAppPurchase deletes an in-app purchase.
func (c *Client) DeleteInAppPurchase(ctx context.Context, iapID string) error {
	return c.Delete(ctx, "/v2/inAppPurchases/"+iapID)
}

// Subscriptions API methods

// ListSubscriptionGroups returns subscription groups for an app.
func (c *Client) ListSubscriptionGroups(ctx context.Context, appID string, limit int) (*SubscriptionGroupsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/apps/"+appID+"/subscriptionGroups", query)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionGroupsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetSubscriptionGroup returns a single subscription group by ID.
func (c *Client) GetSubscriptionGroup(ctx context.Context, groupID string) (*SubscriptionGroupResponse, error) {
	data, err := c.Get(ctx, "/v1/subscriptionGroups/"+groupID, nil)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionGroupResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListSubscriptions returns subscriptions for a subscription group.
func (c *Client) ListSubscriptions(ctx context.Context, groupID string, limit int) (*SubscriptionsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/subscriptionGroups/"+groupID+"/subscriptions", query)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetSubscription returns a single subscription by ID.
func (c *Client) GetSubscription(ctx context.Context, subscriptionID string) (*SubscriptionResponse, error) {
	data, err := c.Get(ctx, "/v1/subscriptions/"+subscriptionID, nil)
	if err != nil {
		return nil, err
	}

	var resp SubscriptionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// App Store Version API methods

// GetAppStoreVersion returns a single app store version by ID.
func (c *Client) GetAppStoreVersion(ctx context.Context, versionID string) (*AppStoreVersionResponse, error) {
	data, err := c.Get(ctx, "/v1/appStoreVersions/"+versionID, nil)
	if err != nil {
		return nil, err
	}

	var resp AppStoreVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateAppStoreVersion creates a new app store version.
func (c *Client) CreateAppStoreVersion(ctx context.Context, req *AppStoreVersionCreateRequest) (*AppStoreVersionResponse, error) {
	data, err := c.Post(ctx, "/v1/appStoreVersions", req)
	if err != nil {
		return nil, err
	}

	var resp AppStoreVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateAppStoreVersion updates an app store version.
func (c *Client) UpdateAppStoreVersion(ctx context.Context, versionID string, req *AppStoreVersionUpdateRequest) (*AppStoreVersionResponse, error) {
	data, err := c.Patch(ctx, "/v1/appStoreVersions/"+versionID, req)
	if err != nil {
		return nil, err
	}

	var resp AppStoreVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteAppStoreVersion deletes an app store version.
func (c *Client) DeleteAppStoreVersion(ctx context.Context, versionID string) error {
	return c.Delete(ctx, "/v1/appStoreVersions/"+versionID)
}

// App Store Version Submission API methods

// CreateAppStoreVersionSubmission submits an app store version for review.
func (c *Client) CreateAppStoreVersionSubmission(ctx context.Context, req *AppStoreVersionSubmissionCreateRequest) (*AppStoreVersionSubmissionResponse, error) {
	data, err := c.Post(ctx, "/v1/appStoreVersionSubmissions", req)
	if err != nil {
		return nil, err
	}

	var resp AppStoreVersionSubmissionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// App Store Review Detail API methods

// GetAppStoreReviewDetail returns review details for a version.
func (c *Client) GetAppStoreReviewDetail(ctx context.Context, versionID string) (*AppStoreReviewDetailResponse, error) {
	data, err := c.Get(ctx, "/v1/appStoreVersions/"+versionID+"/appStoreReviewDetail", nil)
	if err != nil {
		return nil, err
	}

	var resp AppStoreReviewDetailResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateAppStoreReviewDetail creates review details for a version.
func (c *Client) CreateAppStoreReviewDetail(ctx context.Context, req *AppStoreReviewDetailCreateRequest) (*AppStoreReviewDetailResponse, error) {
	data, err := c.Post(ctx, "/v1/appStoreReviewDetails", req)
	if err != nil {
		return nil, err
	}

	var resp AppStoreReviewDetailResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateAppStoreReviewDetail updates review details.
func (c *Client) UpdateAppStoreReviewDetail(ctx context.Context, detailID string, req *AppStoreReviewDetailUpdateRequest) (*AppStoreReviewDetailResponse, error) {
	data, err := c.Patch(ctx, "/v1/appStoreReviewDetails/"+detailID, req)
	if err != nil {
		return nil, err
	}

	var resp AppStoreReviewDetailResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// Phased Release API methods

// GetAppStoreVersionPhasedRelease returns phased release for a version.
func (c *Client) GetAppStoreVersionPhasedRelease(ctx context.Context, versionID string) (*AppStoreVersionPhasedReleaseResponse, error) {
	data, err := c.Get(ctx, "/v1/appStoreVersions/"+versionID+"/appStoreVersionPhasedRelease", nil)
	if err != nil {
		return nil, err
	}

	var resp AppStoreVersionPhasedReleaseResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateAppStoreVersionPhasedRelease creates a phased release.
func (c *Client) CreateAppStoreVersionPhasedRelease(ctx context.Context, req *AppStoreVersionPhasedReleaseCreateRequest) (*AppStoreVersionPhasedReleaseResponse, error) {
	data, err := c.Post(ctx, "/v1/appStoreVersionPhasedReleases", req)
	if err != nil {
		return nil, err
	}

	var resp AppStoreVersionPhasedReleaseResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateAppStoreVersionPhasedRelease updates a phased release.
func (c *Client) UpdateAppStoreVersionPhasedRelease(ctx context.Context, phasedReleaseID string, req *AppStoreVersionPhasedReleaseUpdateRequest) (*AppStoreVersionPhasedReleaseResponse, error) {
	data, err := c.Patch(ctx, "/v1/appStoreVersionPhasedReleases/"+phasedReleaseID, req)
	if err != nil {
		return nil, err
	}

	var resp AppStoreVersionPhasedReleaseResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteAppStoreVersionPhasedRelease deletes a phased release.
func (c *Client) DeleteAppStoreVersionPhasedRelease(ctx context.Context, phasedReleaseID string) error {
	return c.Delete(ctx, "/v1/appStoreVersionPhasedReleases/"+phasedReleaseID)
}

// App Screenshot API methods

// ListAppScreenshotSets returns screenshot sets for a version localization.
func (c *Client) ListAppScreenshotSets(ctx context.Context, localizationID string, limit int) (*AppScreenshotSetsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/appStoreVersionLocalizations/"+localizationID+"/appScreenshotSets", query)
	if err != nil {
		return nil, err
	}

	var resp AppScreenshotSetsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListAppScreenshots returns screenshots for a screenshot set.
func (c *Client) ListAppScreenshots(ctx context.Context, screenshotSetID string, limit int) (*AppScreenshotsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/appScreenshotSets/"+screenshotSetID+"/appScreenshots", query)
	if err != nil {
		return nil, err
	}

	var resp AppScreenshotsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetAppScreenshot returns a single screenshot by ID.
func (c *Client) GetAppScreenshot(ctx context.Context, screenshotID string) (*AppScreenshotResponse, error) {
	data, err := c.Get(ctx, "/v1/appScreenshots/"+screenshotID, nil)
	if err != nil {
		return nil, err
	}

	var resp AppScreenshotResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateAppScreenshot creates a new screenshot.
func (c *Client) CreateAppScreenshot(ctx context.Context, req *AppScreenshotCreateRequest) (*AppScreenshotResponse, error) {
	data, err := c.Post(ctx, "/v1/appScreenshots", req)
	if err != nil {
		return nil, err
	}

	var resp AppScreenshotResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateAppScreenshot updates a screenshot.
func (c *Client) UpdateAppScreenshot(ctx context.Context, screenshotID string, req *AppScreenshotUpdateRequest) (*AppScreenshotResponse, error) {
	data, err := c.Patch(ctx, "/v1/appScreenshots/"+screenshotID, req)
	if err != nil {
		return nil, err
	}

	var resp AppScreenshotResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteAppScreenshot deletes a screenshot.
func (c *Client) DeleteAppScreenshot(ctx context.Context, screenshotID string) error {
	return c.Delete(ctx, "/v1/appScreenshots/"+screenshotID)
}

// App Preview API methods

// ListAppPreviewSets returns preview sets for a version localization.
func (c *Client) ListAppPreviewSets(ctx context.Context, localizationID string, limit int) (*AppPreviewSetsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/appStoreVersionLocalizations/"+localizationID+"/appPreviewSets", query)
	if err != nil {
		return nil, err
	}

	var resp AppPreviewSetsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListAppPreviews returns previews for a preview set.
func (c *Client) ListAppPreviews(ctx context.Context, previewSetID string, limit int) (*AppPreviewsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/appPreviewSets/"+previewSetID+"/appPreviews", query)
	if err != nil {
		return nil, err
	}

	var resp AppPreviewsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetAppPreview returns a single preview by ID.
func (c *Client) GetAppPreview(ctx context.Context, previewID string) (*AppPreviewResponse, error) {
	data, err := c.Get(ctx, "/v1/appPreviews/"+previewID, nil)
	if err != nil {
		return nil, err
	}

	var resp AppPreviewResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateAppPreview creates a new preview.
func (c *Client) CreateAppPreview(ctx context.Context, req *AppPreviewCreateRequest) (*AppPreviewResponse, error) {
	data, err := c.Post(ctx, "/v1/appPreviews", req)
	if err != nil {
		return nil, err
	}

	var resp AppPreviewResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteAppPreview deletes a preview.
func (c *Client) DeleteAppPreview(ctx context.Context, previewID string) error {
	return c.Delete(ctx, "/v1/appPreviews/"+previewID)
}

// App Pre-Order API methods

// GetAppPreOrder returns pre-order info for an app.
func (c *Client) GetAppPreOrder(ctx context.Context, appID string) (*AppPreOrderResponse, error) {
	data, err := c.Get(ctx, "/v1/apps/"+appID+"/preOrder", nil)
	if err != nil {
		return nil, err
	}

	var resp AppPreOrderResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateAppPreOrder creates a pre-order.
func (c *Client) CreateAppPreOrder(ctx context.Context, req *AppPreOrderCreateRequest) (*AppPreOrderResponse, error) {
	data, err := c.Post(ctx, "/v1/appPreOrders", req)
	if err != nil {
		return nil, err
	}

	var resp AppPreOrderResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateAppPreOrder updates a pre-order.
func (c *Client) UpdateAppPreOrder(ctx context.Context, preOrderID string, req *AppPreOrderUpdateRequest) (*AppPreOrderResponse, error) {
	data, err := c.Patch(ctx, "/v1/appPreOrders/"+preOrderID, req)
	if err != nil {
		return nil, err
	}

	var resp AppPreOrderResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteAppPreOrder deletes a pre-order.
func (c *Client) DeleteAppPreOrder(ctx context.Context, preOrderID string) error {
	return c.Delete(ctx, "/v1/appPreOrders/"+preOrderID)
}

// App Event API methods

// ListAppEvents returns app events for an app.
func (c *Client) ListAppEvents(ctx context.Context, appID string, limit int) (*AppEventsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/apps/"+appID+"/appEvents", query)
	if err != nil {
		return nil, err
	}

	var resp AppEventsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetAppEvent returns a single app event by ID.
func (c *Client) GetAppEvent(ctx context.Context, eventID string) (*AppEventResponse, error) {
	data, err := c.Get(ctx, "/v1/appEvents/"+eventID, nil)
	if err != nil {
		return nil, err
	}

	var resp AppEventResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateAppEvent creates a new app event.
func (c *Client) CreateAppEvent(ctx context.Context, req *AppEventCreateRequest) (*AppEventResponse, error) {
	data, err := c.Post(ctx, "/v1/appEvents", req)
	if err != nil {
		return nil, err
	}

	var resp AppEventResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateAppEvent updates an app event.
func (c *Client) UpdateAppEvent(ctx context.Context, eventID string, req *AppEventUpdateRequest) (*AppEventResponse, error) {
	data, err := c.Patch(ctx, "/v1/appEvents/"+eventID, req)
	if err != nil {
		return nil, err
	}

	var resp AppEventResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteAppEvent deletes an app event.
func (c *Client) DeleteAppEvent(ctx context.Context, eventID string) error {
	return c.Delete(ctx, "/v1/appEvents/"+eventID)
}

// Analytics API methods

// ListAnalyticsReportRequests returns analytics report requests for an app.
func (c *Client) ListAnalyticsReportRequests(ctx context.Context, appID string, limit int) (*AnalyticsReportRequestsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/apps/"+appID+"/analyticsReportRequests", query)
	if err != nil {
		return nil, err
	}

	var resp AnalyticsReportRequestsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetAnalyticsReportRequest returns a single analytics report request.
func (c *Client) GetAnalyticsReportRequest(ctx context.Context, requestID string) (*AnalyticsReportRequestResponse, error) {
	data, err := c.Get(ctx, "/v1/analyticsReportRequests/"+requestID, nil)
	if err != nil {
		return nil, err
	}

	var resp AnalyticsReportRequestResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateAnalyticsReportRequest creates an analytics report request.
func (c *Client) CreateAnalyticsReportRequest(ctx context.Context, req *AnalyticsReportRequestCreateRequest) (*AnalyticsReportRequestResponse, error) {
	data, err := c.Post(ctx, "/v1/analyticsReportRequests", req)
	if err != nil {
		return nil, err
	}

	var resp AnalyticsReportRequestResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteAnalyticsReportRequest deletes an analytics report request.
func (c *Client) DeleteAnalyticsReportRequest(ctx context.Context, requestID string) error {
	return c.Delete(ctx, "/v1/analyticsReportRequests/"+requestID)
}

// ListAnalyticsReports returns analytics reports for a request.
func (c *Client) ListAnalyticsReports(ctx context.Context, requestID string, limit int) (*AnalyticsReportsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/analyticsReportRequests/"+requestID+"/reports", query)
	if err != nil {
		return nil, err
	}

	var resp AnalyticsReportsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListAnalyticsReportInstances returns instances for a report.
func (c *Client) ListAnalyticsReportInstances(ctx context.Context, reportID string, limit int) (*AnalyticsReportInstancesResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/analyticsReports/"+reportID+"/instances", query)
	if err != nil {
		return nil, err
	}

	var resp AnalyticsReportInstancesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListAnalyticsReportSegments returns segments for a report instance.
func (c *Client) ListAnalyticsReportSegments(ctx context.Context, instanceID string, limit int) (*AnalyticsReportSegmentsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/analyticsReportInstances/"+instanceID+"/segments", query)
	if err != nil {
		return nil, err
	}

	var resp AnalyticsReportSegmentsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// App Clip API methods

// ListAppClips returns app clips for an app.
func (c *Client) ListAppClips(ctx context.Context, appID string, limit int) (*AppClipsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/apps/"+appID+"/appClips", query)
	if err != nil {
		return nil, err
	}

	var resp AppClipsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetAppClip returns a single app clip by ID.
func (c *Client) GetAppClip(ctx context.Context, appClipID string) (*AppClipResponse, error) {
	data, err := c.Get(ctx, "/v1/appClips/"+appClipID, nil)
	if err != nil {
		return nil, err
	}

	var resp AppClipResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListAppClipDefaultExperiences returns default experiences for an app clip.
func (c *Client) ListAppClipDefaultExperiences(ctx context.Context, appClipID string, limit int) (*AppClipDefaultExperiencesResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/appClips/"+appClipID+"/appClipDefaultExperiences", query)
	if err != nil {
		return nil, err
	}

	var resp AppClipDefaultExperiencesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetAppClipDefaultExperience returns a single default experience.
func (c *Client) GetAppClipDefaultExperience(ctx context.Context, experienceID string) (*AppClipDefaultExperienceResponse, error) {
	data, err := c.Get(ctx, "/v1/appClipDefaultExperiences/"+experienceID, nil)
	if err != nil {
		return nil, err
	}

	var resp AppClipDefaultExperienceResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListAppClipAdvancedExperiences returns advanced experiences for an app clip.
func (c *Client) ListAppClipAdvancedExperiences(ctx context.Context, appClipID string, limit int) (*AppClipAdvancedExperiencesResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/appClips/"+appClipID+"/appClipAdvancedExperiences", query)
	if err != nil {
		return nil, err
	}

	var resp AppClipAdvancedExperiencesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetAppClipAdvancedExperience returns a single advanced experience.
func (c *Client) GetAppClipAdvancedExperience(ctx context.Context, experienceID string) (*AppClipAdvancedExperienceResponse, error) {
	data, err := c.Get(ctx, "/v1/appClipAdvancedExperiences/"+experienceID, nil)
	if err != nil {
		return nil, err
	}

	var resp AppClipAdvancedExperienceResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// Game Center API methods

// GetGameCenterDetail returns game center details for an app.
func (c *Client) GetGameCenterDetail(ctx context.Context, appID string) (*GameCenterDetailResponse, error) {
	data, err := c.Get(ctx, "/v1/apps/"+appID+"/gameCenterDetail", nil)
	if err != nil {
		return nil, err
	}

	var resp GameCenterDetailResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListGameCenterAchievements returns achievements for a game center detail.
func (c *Client) ListGameCenterAchievements(ctx context.Context, gameCenterDetailID string, limit int) (*GameCenterAchievementsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/gameCenterDetails/"+gameCenterDetailID+"/gameCenterAchievements", query)
	if err != nil {
		return nil, err
	}

	var resp GameCenterAchievementsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetGameCenterAchievement returns a single achievement.
func (c *Client) GetGameCenterAchievement(ctx context.Context, achievementID string) (*GameCenterAchievementResponse, error) {
	data, err := c.Get(ctx, "/v1/gameCenterAchievements/"+achievementID, nil)
	if err != nil {
		return nil, err
	}

	var resp GameCenterAchievementResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateGameCenterAchievement creates a new achievement.
func (c *Client) CreateGameCenterAchievement(ctx context.Context, req *GameCenterAchievementCreateRequest) (*GameCenterAchievementResponse, error) {
	data, err := c.Post(ctx, "/v1/gameCenterAchievements", req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterAchievementResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateGameCenterAchievement updates an achievement.
func (c *Client) UpdateGameCenterAchievement(ctx context.Context, achievementID string, req *GameCenterAchievementUpdateRequest) (*GameCenterAchievementResponse, error) {
	data, err := c.Patch(ctx, "/v1/gameCenterAchievements/"+achievementID, req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterAchievementResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteGameCenterAchievement deletes an achievement.
func (c *Client) DeleteGameCenterAchievement(ctx context.Context, achievementID string) error {
	return c.Delete(ctx, "/v1/gameCenterAchievements/"+achievementID)
}

// ListGameCenterLeaderboards returns leaderboards for a game center detail.
func (c *Client) ListGameCenterLeaderboards(ctx context.Context, gameCenterDetailID string, limit int) (*GameCenterLeaderboardsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/gameCenterDetails/"+gameCenterDetailID+"/gameCenterLeaderboards", query)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetGameCenterLeaderboard returns a single leaderboard.
func (c *Client) GetGameCenterLeaderboard(ctx context.Context, leaderboardID string) (*GameCenterLeaderboardResponse, error) {
	data, err := c.Get(ctx, "/v1/gameCenterLeaderboards/"+leaderboardID, nil)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateGameCenterLeaderboard creates a new leaderboard.
func (c *Client) CreateGameCenterLeaderboard(ctx context.Context, req *GameCenterLeaderboardCreateRequest) (*GameCenterLeaderboardResponse, error) {
	data, err := c.Post(ctx, "/v1/gameCenterLeaderboards", req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateGameCenterLeaderboard updates a leaderboard.
func (c *Client) UpdateGameCenterLeaderboard(ctx context.Context, leaderboardID string, req *GameCenterLeaderboardUpdateRequest) (*GameCenterLeaderboardResponse, error) {
	data, err := c.Patch(ctx, "/v1/gameCenterLeaderboards/"+leaderboardID, req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteGameCenterLeaderboard deletes a leaderboard.
func (c *Client) DeleteGameCenterLeaderboard(ctx context.Context, leaderboardID string) error {
	return c.Delete(ctx, "/v1/gameCenterLeaderboards/"+leaderboardID)
}

// Xcode Cloud API methods

// ListCiProducts returns Xcode Cloud products for an app.
func (c *Client) ListCiProducts(ctx context.Context, appID string, limit int) (*CiProductsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}
	if appID != "" {
		query.Set("filter[app]", appID)
	}

	data, err := c.Get(ctx, "/v1/ciProducts", query)
	if err != nil {
		return nil, err
	}

	var resp CiProductsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetCiProduct returns a single Xcode Cloud product.
func (c *Client) GetCiProduct(ctx context.Context, productID string) (*CiProductResponse, error) {
	data, err := c.Get(ctx, "/v1/ciProducts/"+productID, nil)
	if err != nil {
		return nil, err
	}

	var resp CiProductResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListCiWorkflows returns workflows for a product.
func (c *Client) ListCiWorkflows(ctx context.Context, productID string, limit int) (*CiWorkflowsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/ciProducts/"+productID+"/workflows", query)
	if err != nil {
		return nil, err
	}

	var resp CiWorkflowsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetCiWorkflow returns a single workflow.
func (c *Client) GetCiWorkflow(ctx context.Context, workflowID string) (*CiWorkflowResponse, error) {
	data, err := c.Get(ctx, "/v1/ciWorkflows/"+workflowID, nil)
	if err != nil {
		return nil, err
	}

	var resp CiWorkflowResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListCiBuildRuns returns build runs for a workflow.
func (c *Client) ListCiBuildRuns(ctx context.Context, workflowID string, limit int) (*CiBuildRunsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}

	data, err := c.Get(ctx, "/v1/ciWorkflows/"+workflowID+"/buildRuns", query)
	if err != nil {
		return nil, err
	}

	var resp CiBuildRunsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetCiBuildRun returns a single build run.
func (c *Client) GetCiBuildRun(ctx context.Context, buildRunID string) (*CiBuildRunResponse, error) {
	data, err := c.Get(ctx, "/v1/ciBuildRuns/"+buildRunID, nil)
	if err != nil {
		return nil, err
	}

	var resp CiBuildRunResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// StartCiBuildRun starts a new build run for a workflow.
func (c *Client) StartCiBuildRun(ctx context.Context, workflowID string) (*CiBuildRunResponse, error) {
	body := map[string]any{
		"data": map[string]any{
			"type": "ciBuildRuns",
			"relationships": map[string]any{
				"workflow": map[string]any{
					"data": map[string]string{
						"type": "ciWorkflows",
						"id":   workflowID,
					},
				},
			},
		},
	}

	data, err := c.Post(ctx, "/v1/ciBuildRuns", body)
	if err != nil {
		return nil, err
	}

	var resp CiBuildRunResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CancelCiBuildRun cancels a build run.
func (c *Client) CancelCiBuildRun(ctx context.Context, buildRunID string) error {
	return c.Delete(ctx, "/v1/ciBuildRuns/"+buildRunID)
}

// Sales and Finance API methods

// GetSalesReport returns sales reports.
func (c *Client) GetSalesReport(ctx context.Context, vendorNumber, reportType, reportSubType, frequency, reportDate string) ([]byte, error) {
	query := url.Values{}
	query.Set("filter[vendorNumber]", vendorNumber)
	query.Set("filter[reportType]", reportType)
	query.Set("filter[reportSubType]", reportSubType)
	query.Set("filter[frequency]", frequency)
	query.Set("filter[reportDate]", reportDate)

	data, err := c.Get(ctx, "/v1/salesReports", query)
	if err != nil {
		return nil, err
	}

	return data, nil
}

// GetFinanceReport returns finance reports.
func (c *Client) GetFinanceReport(ctx context.Context, vendorNumber, regionCode, reportType, reportDate string) ([]byte, error) {
	query := url.Values{}
	query.Set("filter[vendorNumber]", vendorNumber)
	query.Set("filter[regionCode]", regionCode)
	query.Set("filter[reportType]", reportType)
	query.Set("filter[reportDate]", reportDate)

	data, err := c.Get(ctx, "/v1/financeReports", query)
	if err != nil {
		return nil, err
	}

	return data, nil
}

// App Encryption API methods

// ListAppEncryptionDeclarations returns encryption declarations for an app.
func (c *Client) ListAppEncryptionDeclarations(ctx context.Context, appID string, limit int) (*AppEncryptionDeclarationsResponse, error) {
	query := url.Values{}
	if limit > 0 {
		query.Set("limit", fmt.Sprintf("%d", limit))
	}
	if appID != "" {
		query.Set("filter[app]", appID)
	}

	data, err := c.Get(ctx, "/v1/appEncryptionDeclarations", query)
	if err != nil {
		return nil, err
	}

	var resp AppEncryptionDeclarationsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetAppEncryptionDeclaration returns a single encryption declaration.
func (c *Client) GetAppEncryptionDeclaration(ctx context.Context, declarationID string) (*AppEncryptionDeclarationResponse, error) {
	data, err := c.Get(ctx, "/v1/appEncryptionDeclarations/"+declarationID, nil)
	if err != nil {
		return nil, err
	}

	var resp AppEncryptionDeclarationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateAppEncryptionDeclaration creates an encryption declaration.
func (c *Client) CreateAppEncryptionDeclaration(ctx context.Context, req *AppEncryptionDeclarationCreateRequest) (*AppEncryptionDeclarationResponse, error) {
	data, err := c.Post(ctx, "/v1/appEncryptionDeclarations", req)
	if err != nil {
		return nil, err
	}

	var resp AppEncryptionDeclarationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// AssignBuildToEncryptionDeclaration assigns a build to an encryption declaration.
func (c *Client) AssignBuildToEncryptionDeclaration(ctx context.Context, declarationID, buildID string) error {
	body := map[string]any{
		"data": []map[string]string{
			{
				"type": "builds",
				"id":   buildID,
			},
		},
	}

	_, err := c.Post(ctx, "/v1/appEncryptionDeclarations/"+declarationID+"/relationships/builds", body)
	return err
}
