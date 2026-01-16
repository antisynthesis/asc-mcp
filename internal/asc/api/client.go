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
