package api

import (
	"context"
	"encoding/json"
	"fmt"
	"net/url"
	"time"
)

// Apple-Hosted Background Assets API methods (App Store Connect API
// 4.0+). Background assets are Apple-hosted asset packs downloaded to
// devices outside the app binary. Each backgroundAssets record owns
// versions; each version carries an asset pack file and a manifest
// delivered via the same reserve/upload/commit protocol implemented in
// upload.go, then flows through internal beta, external beta, and App
// Store releases.

// Background asset types

// BackgroundAssetsResponse represents a list of background assets.
type BackgroundAssetsResponse struct {
	Data     []BackgroundAsset  `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// BackgroundAssetResponse represents a single background asset.
type BackgroundAssetResponse struct {
	Data     BackgroundAsset `json:"data"`
	Included []any           `json:"included,omitempty"`
}

// BackgroundAsset represents an Apple-hosted background asset pack.
type BackgroundAsset struct {
	Type          string                        `json:"type"`
	ID            string                        `json:"id"`
	Attributes    BackgroundAssetAttributes     `json:"attributes"`
	Relationships *BackgroundAssetRelationships `json:"relationships,omitempty"`
}

// BackgroundAssetAttributes contains background asset attributes.
// UsedBytes reports the storage consumed by the asset's versions
// against the app's Apple-hosted quota.
type BackgroundAssetAttributes struct {
	Archived            bool       `json:"archived,omitempty"`
	AssetPackIdentifier string     `json:"assetPackIdentifier,omitempty"`
	CreatedDate         *time.Time `json:"createdDate,omitempty"`
	UsedBytes           int64      `json:"usedBytes,omitempty"`
}

// BackgroundAssetRelationships exposes the resources linked to a
// background asset; the three version pointers identify which version
// is currently live in each release channel.
type BackgroundAssetRelationships struct {
	App                 *RelationshipData `json:"app,omitempty"`
	AppStoreVersion     *RelationshipData `json:"appStoreVersion,omitempty"`
	InternalBetaVersion *RelationshipData `json:"internalBetaVersion,omitempty"`
	ExternalBetaVersion *RelationshipData `json:"externalBetaVersion,omitempty"`
}

// BackgroundAssetCreateRequest represents a request to create a
// background asset.
type BackgroundAssetCreateRequest struct {
	Data BackgroundAssetCreateData `json:"data"`
}

// BackgroundAssetCreateData contains the data for creating a background
// asset.
type BackgroundAssetCreateData struct {
	Type          string                             `json:"type"`
	Attributes    BackgroundAssetCreateAttributes    `json:"attributes"`
	Relationships BackgroundAssetCreateRelationships `json:"relationships"`
}

// BackgroundAssetCreateAttributes contains attributes for creating a
// background asset. AssetPackIdentifier is required by the API.
type BackgroundAssetCreateAttributes struct {
	AssetPackIdentifier string `json:"assetPackIdentifier"`
}

// BackgroundAssetCreateRelationships contains relationships for
// creating a background asset.
type BackgroundAssetCreateRelationships struct {
	App RelationshipData `json:"app"`
}

// BackgroundAssetUpdateRequest represents a request to update a
// background asset (archive or unarchive it).
type BackgroundAssetUpdateRequest struct {
	Data BackgroundAssetUpdateData `json:"data"`
}

// BackgroundAssetUpdateData contains the data for updating a background
// asset.
type BackgroundAssetUpdateData struct {
	Type       string                           `json:"type"`
	ID         string                           `json:"id"`
	Attributes *BackgroundAssetUpdateAttributes `json:"attributes,omitempty"`
}

// BackgroundAssetUpdateAttributes contains attributes for updating a
// background asset.
type BackgroundAssetUpdateAttributes struct {
	Archived *bool `json:"archived,omitempty"`
}

// Background asset version types

// BackgroundAssetVersionsResponse represents a list of background asset
// versions.
type BackgroundAssetVersionsResponse struct {
	Data     []BackgroundAssetVersion `json:"data"`
	Links    PagedDocumentLinks       `json:"links"`
	Meta     *PagingInformation       `json:"meta,omitempty"`
	Included []any                    `json:"included,omitempty"`
}

// BackgroundAssetVersionResponse represents a single background asset
// version.
type BackgroundAssetVersionResponse struct {
	Data     BackgroundAssetVersion `json:"data"`
	Included []any                  `json:"included,omitempty"`
}

// BackgroundAssetVersion represents one uploadable version of a
// background asset pack.
type BackgroundAssetVersion struct {
	Type          string                               `json:"type"`
	ID            string                               `json:"id"`
	Attributes    BackgroundAssetVersionAttributes     `json:"attributes"`
	Relationships *BackgroundAssetVersionRelationships `json:"relationships,omitempty"`
}

// BackgroundAssetVersionAttributes contains background asset version
// attributes. State is one of AWAITING_UPLOAD, PROCESSING, FAILED, or
// COMPLETE; StateDetails carries Apple's per-issue codes and
// descriptions.
type BackgroundAssetVersionAttributes struct {
	CreatedDate  *time.Time                         `json:"createdDate,omitempty"`
	Platforms    []string                           `json:"platforms,omitempty"`
	State        string                             `json:"state,omitempty"`
	StateDetails *BackgroundAssetVersionStateDetail `json:"stateDetails,omitempty"`
	Version      string                             `json:"version,omitempty"`
	Locale       string                             `json:"locale,omitempty"`
}

// BackgroundAssetVersionStateDetail carries the coded messages Apple
// attaches to a version's processing state.
type BackgroundAssetVersionStateDetail struct {
	Errors   []StateDetail `json:"errors,omitempty"`
	Warnings []StateDetail `json:"warnings,omitempty"`
	Infos    []StateDetail `json:"infos,omitempty"`
}

// BackgroundAssetVersionRelationships exposes the resources linked to a
// background asset version, including its release records per channel
// and its asset/manifest upload files.
type BackgroundAssetVersionRelationships struct {
	BackgroundAsset     *RelationshipData `json:"backgroundAsset,omitempty"`
	InternalBetaRelease *RelationshipData `json:"internalBetaRelease,omitempty"`
	ExternalBetaRelease *RelationshipData `json:"externalBetaRelease,omitempty"`
	AppStoreRelease     *RelationshipData `json:"appStoreRelease,omitempty"`
	AssetFile           *RelationshipData `json:"assetFile,omitempty"`
	ManifestFile        *RelationshipData `json:"manifestFile,omitempty"`
}

// BackgroundAssetVersionCreateRequest represents a request to create a
// background asset version.
type BackgroundAssetVersionCreateRequest struct {
	Data BackgroundAssetVersionCreateData `json:"data"`
}

// BackgroundAssetVersionCreateData contains the data for creating a
// background asset version.
type BackgroundAssetVersionCreateData struct {
	Type          string                                    `json:"type"`
	Relationships BackgroundAssetVersionCreateRelationships `json:"relationships"`
}

// BackgroundAssetVersionCreateRelationships contains relationships for
// creating a background asset version.
type BackgroundAssetVersionCreateRelationships struct {
	BackgroundAsset RelationshipData `json:"backgroundAsset"`
}

// Background asset upload file types

// BackgroundAssetUploadFilesResponse represents a list of background
// asset upload files.
type BackgroundAssetUploadFilesResponse struct {
	Data  []BackgroundAssetUploadFile `json:"data"`
	Links PagedDocumentLinks          `json:"links"`
	Meta  *PagingInformation          `json:"meta,omitempty"`
}

// BackgroundAssetUploadFileResponse represents a single background
// asset upload file.
type BackgroundAssetUploadFileResponse struct {
	Data BackgroundAssetUploadFile `json:"data"`
}

// BackgroundAssetUploadFile represents one file (asset pack or
// manifest) on a background asset version.
type BackgroundAssetUploadFile struct {
	Type       string                              `json:"type"`
	ID         string                              `json:"id"`
	Attributes BackgroundAssetUploadFileAttributes `json:"attributes"`
}

// BackgroundAssetUploadFileAttributes contains background asset upload
// file attributes, including the upload operations issued at
// reservation time.
type BackgroundAssetUploadFileAttributes struct {
	AssetDeliveryState  *AssetDeliveryState `json:"assetDeliveryState,omitempty"`
	AssetToken          string              `json:"assetToken,omitempty"`
	AssetType           string              `json:"assetType,omitempty"`
	FileName            string              `json:"fileName,omitempty"`
	FileSize            int64               `json:"fileSize,omitempty"`
	SourceFileChecksums *Checksums          `json:"sourceFileChecksums,omitempty"`
	UploadOperations    []UploadOperation   `json:"uploadOperations,omitempty"`
}

// BackgroundAssetUploadFileCreateRequest represents a request to
// reserve a background asset upload file.
type BackgroundAssetUploadFileCreateRequest struct {
	Data BackgroundAssetUploadFileCreateData `json:"data"`
}

// BackgroundAssetUploadFileCreateData contains the data for reserving a
// background asset upload file.
type BackgroundAssetUploadFileCreateData struct {
	Type          string                                       `json:"type"`
	Attributes    BackgroundAssetUploadFileCreateAttributes    `json:"attributes"`
	Relationships BackgroundAssetUploadFileCreateRelationships `json:"relationships"`
}

// BackgroundAssetUploadFileCreateAttributes contains attributes for
// reserving a background asset upload file. All fields are required by
// the API; AssetType is ASSET or MANIFEST.
type BackgroundAssetUploadFileCreateAttributes struct {
	AssetType string `json:"assetType"`
	FileName  string `json:"fileName"`
	FileSize  int64  `json:"fileSize"`
}

// BackgroundAssetUploadFileCreateRelationships contains relationships
// for reserving a background asset upload file.
type BackgroundAssetUploadFileCreateRelationships struct {
	BackgroundAssetVersion RelationshipData `json:"backgroundAssetVersion"`
}

// BackgroundAssetUploadFileUpdateRequest represents the commit PATCH
// for a background asset upload file.
type BackgroundAssetUploadFileUpdateRequest struct {
	Data BackgroundAssetUploadFileUpdateData `json:"data"`
}

// BackgroundAssetUploadFileUpdateData contains the data for committing
// a background asset upload file.
type BackgroundAssetUploadFileUpdateData struct {
	Type       string                                     `json:"type"`
	ID         string                                     `json:"id"`
	Attributes *BackgroundAssetUploadFileUpdateAttributes `json:"attributes,omitempty"`
}

// BackgroundAssetUploadFileUpdateAttributes contains attributes for
// committing a background asset upload file.
type BackgroundAssetUploadFileUpdateAttributes struct {
	SourceFileChecksums *Checksums `json:"sourceFileChecksums,omitempty"`
	Uploaded            *bool      `json:"uploaded,omitempty"`
}

// Background asset version release types

// BackgroundAssetVersionReleaseResponse represents a single release
// record (internal beta, external beta, or App Store) for a background
// asset version.
type BackgroundAssetVersionReleaseResponse struct {
	Data     BackgroundAssetVersionRelease `json:"data"`
	Included []any                         `json:"included,omitempty"`
}

// BackgroundAssetVersionRelease represents the release state of a
// background asset version in one channel. The three release resource
// types share this shape; Type distinguishes them.
type BackgroundAssetVersionRelease struct {
	Type          string                                      `json:"type"`
	ID            string                                      `json:"id"`
	Attributes    BackgroundAssetVersionReleaseAttributes     `json:"attributes"`
	Relationships *BackgroundAssetVersionReleaseRelationships `json:"relationships,omitempty"`
}

// BackgroundAssetVersionReleaseAttributes contains the release state.
// Internal beta releases use READY_FOR_TESTING or SUPERSEDED; external
// beta and App Store releases add review states such as
// WAITING_FOR_REVIEW, IN_REVIEW, REJECTED, and READY_FOR_DISTRIBUTION.
type BackgroundAssetVersionReleaseAttributes struct {
	State string `json:"state,omitempty"`
}

// BackgroundAssetVersionReleaseRelationships exposes the version a
// release record belongs to.
type BackgroundAssetVersionReleaseRelationships struct {
	BackgroundAssetVersion *RelationshipData `json:"backgroundAssetVersion,omitempty"`
}

// ListBackgroundAssets returns the background assets for an app.
// Supports filter[versions.locale] and filter[versions.platforms] for
// the multi-locale read added in API 4.4.
func (c *Client) ListBackgroundAssets(ctx context.Context, appID string, opts *ListOptions) (*BackgroundAssetsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/apps/"+url.PathEscape(appID)+"/backgroundAssets", query)
	if err != nil {
		return nil, err
	}

	var resp BackgroundAssetsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetBackgroundAsset returns a single background asset.
func (c *Client) GetBackgroundAsset(ctx context.Context, backgroundAssetID string, opts *ListOptions) (*BackgroundAssetResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/backgroundAssets/"+url.PathEscape(backgroundAssetID), query)
	if err != nil {
		return nil, err
	}

	var resp BackgroundAssetResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateBackgroundAsset creates a background asset for an app.
func (c *Client) CreateBackgroundAsset(ctx context.Context, req *BackgroundAssetCreateRequest) (*BackgroundAssetResponse, error) {
	data, err := c.Post(ctx, "/v1/backgroundAssets", req)
	if err != nil {
		return nil, err
	}

	var resp BackgroundAssetResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateBackgroundAsset updates a background asset (archives or
// unarchives it).
func (c *Client) UpdateBackgroundAsset(ctx context.Context, backgroundAssetID string, req *BackgroundAssetUpdateRequest) (*BackgroundAssetResponse, error) {
	data, err := c.Patch(ctx, "/v1/backgroundAssets/"+url.PathEscape(backgroundAssetID), req)
	if err != nil {
		return nil, err
	}

	var resp BackgroundAssetResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListBackgroundAssetVersions returns the versions of a background
// asset, including their processing states and release states.
func (c *Client) ListBackgroundAssetVersions(ctx context.Context, backgroundAssetID string, opts *ListOptions) (*BackgroundAssetVersionsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/backgroundAssets/"+url.PathEscape(backgroundAssetID)+"/versions", query)
	if err != nil {
		return nil, err
	}

	var resp BackgroundAssetVersionsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetBackgroundAssetVersion returns a single background asset version.
func (c *Client) GetBackgroundAssetVersion(ctx context.Context, versionID string, opts *ListOptions) (*BackgroundAssetVersionResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/backgroundAssetVersions/"+url.PathEscape(versionID), query)
	if err != nil {
		return nil, err
	}

	var resp BackgroundAssetVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateBackgroundAssetVersion creates a new version on a background
// asset. The version number is assigned by Apple.
func (c *Client) CreateBackgroundAssetVersion(ctx context.Context, req *BackgroundAssetVersionCreateRequest) (*BackgroundAssetVersionResponse, error) {
	data, err := c.Post(ctx, "/v1/backgroundAssetVersions", req)
	if err != nil {
		return nil, err
	}

	var resp BackgroundAssetVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListBackgroundAssetUploadFiles returns the upload files (asset pack
// and manifest) attached to a background asset version, including their
// delivery states.
func (c *Client) ListBackgroundAssetUploadFiles(ctx context.Context, versionID string, opts *ListOptions) (*BackgroundAssetUploadFilesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/backgroundAssetVersions/"+url.PathEscape(versionID)+"/backgroundAssetUploadFiles", query)
	if err != nil {
		return nil, err
	}

	var resp BackgroundAssetUploadFilesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateBackgroundAssetUploadFile reserves an upload file slot on a
// background asset version. The response carries the upload operations
// for the file's bytes.
func (c *Client) CreateBackgroundAssetUploadFile(ctx context.Context, req *BackgroundAssetUploadFileCreateRequest) (*BackgroundAssetUploadFileResponse, error) {
	data, err := c.Post(ctx, "/v1/backgroundAssetUploadFiles", req)
	if err != nil {
		return nil, err
	}

	var resp BackgroundAssetUploadFileResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateBackgroundAssetUploadFile commits (or amends) a background
// asset upload file.
func (c *Client) UpdateBackgroundAssetUploadFile(ctx context.Context, fileID string, req *BackgroundAssetUploadFileUpdateRequest) (*BackgroundAssetUploadFileResponse, error) {
	data, err := c.Patch(ctx, "/v1/backgroundAssetUploadFiles/"+url.PathEscape(fileID), req)
	if err != nil {
		return nil, err
	}

	var resp BackgroundAssetUploadFileResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetBackgroundAssetVersionInternalBetaRelease returns the internal
// beta release record for a background asset version.
func (c *Client) GetBackgroundAssetVersionInternalBetaRelease(ctx context.Context, releaseID string) (*BackgroundAssetVersionReleaseResponse, error) {
	return c.getBackgroundAssetVersionRelease(ctx, "/v1/backgroundAssetVersionInternalBetaReleases/"+url.PathEscape(releaseID))
}

// GetBackgroundAssetVersionExternalBetaRelease returns the external
// beta (TestFlight) release record for a background asset version.
func (c *Client) GetBackgroundAssetVersionExternalBetaRelease(ctx context.Context, releaseID string) (*BackgroundAssetVersionReleaseResponse, error) {
	return c.getBackgroundAssetVersionRelease(ctx, "/v1/backgroundAssetVersionExternalBetaReleases/"+url.PathEscape(releaseID))
}

// GetBackgroundAssetVersionAppStoreRelease returns the App Store
// release record for a background asset version.
func (c *Client) GetBackgroundAssetVersionAppStoreRelease(ctx context.Context, releaseID string) (*BackgroundAssetVersionReleaseResponse, error) {
	return c.getBackgroundAssetVersionRelease(ctx, "/v1/backgroundAssetVersionAppStoreReleases/"+url.PathEscape(releaseID))
}

// getBackgroundAssetVersionRelease fetches one of the three release
// resource shapes, which share a single response structure.
func (c *Client) getBackgroundAssetVersionRelease(ctx context.Context, path string) (*BackgroundAssetVersionReleaseResponse, error) {
	data, err := c.Get(ctx, path, nil)
	if err != nil {
		return nil, err
	}

	var resp BackgroundAssetVersionReleaseResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UploadBackgroundAssetFile reserves, uploads, and commits one file on
// an existing background asset version. assetType is ASSET (the asset
// pack archive) or MANIFEST (its manifest plist). The commit carries
// the SHA-256 of the body so Apple can verify the stored bytes.
func (c *Client) UploadBackgroundAssetFile(ctx context.Context, versionID, fileName, assetType string, body []byte) (*BackgroundAssetUploadFileResponse, error) {
	if fileName == "" {
		return nil, fmt.Errorf("file_name is required")
	}
	if len(body) == 0 {
		return nil, fmt.Errorf("file body is empty")
	}
	if len(body) > MaxUploadSize {
		return nil, fmt.Errorf("file body exceeds max size of %d bytes", MaxUploadSize)
	}
	createReq := &BackgroundAssetUploadFileCreateRequest{
		Data: BackgroundAssetUploadFileCreateData{
			Type: "backgroundAssetUploadFiles",
			Attributes: BackgroundAssetUploadFileCreateAttributes{
				AssetType: assetType,
				FileName:  fileName,
				FileSize:  int64(len(body)),
			},
			Relationships: BackgroundAssetUploadFileCreateRelationships{
				BackgroundAssetVersion: RelationshipData{
					Data: ResourceIdentifier{Type: "backgroundAssetVersions", ID: versionID},
				},
			},
		},
	}
	reservation, err := c.CreateBackgroundAssetUploadFile(ctx, createReq)
	if err != nil {
		return nil, fmt.Errorf("reservation: %w", err)
	}
	ops := reservation.Data.Attributes.UploadOperations
	if err := c.PerformUploadOperations(ctx, ops, body); err != nil {
		return nil, fmt.Errorf("upload: %w", err)
	}
	uploaded := true
	commit := &BackgroundAssetUploadFileUpdateRequest{
		Data: BackgroundAssetUploadFileUpdateData{
			Type: "backgroundAssetUploadFiles",
			ID:   reservation.Data.ID,
			Attributes: &BackgroundAssetUploadFileUpdateAttributes{
				SourceFileChecksums: &Checksums{
					File: &Checksum{Hash: Sha256Hex(body), Algorithm: "SHA_256"},
				},
				Uploaded: &uploaded,
			},
		},
	}
	return c.UpdateBackgroundAssetUploadFile(ctx, reservation.Data.ID, commit)
}
