package api

import (
	"context"
	"encoding/json"
	"fmt"
	"net/url"
	"time"
)

// Build Upload API methods (App Store Connect API 4.1+). These endpoints
// let callers deliver build binaries (IPA/PKG) directly through the API
// instead of Transporter/altool, using the same reserve/upload/commit
// asset protocol implemented in upload.go.

// Build upload types

// BuildUploadsResponse represents a list of build uploads.
type BuildUploadsResponse struct {
	Data     []BuildUpload      `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// BuildUploadResponse represents a single build upload.
type BuildUploadResponse struct {
	Data     BuildUpload `json:"data"`
	Included []any       `json:"included,omitempty"`
}

// BuildUpload represents an in-progress or completed build delivery.
type BuildUpload struct {
	Type          string                    `json:"type"`
	ID            string                    `json:"id"`
	Attributes    BuildUploadAttributes     `json:"attributes"`
	Relationships *BuildUploadRelationships `json:"relationships,omitempty"`
}

// BuildUploadAttributes contains build upload attributes.
type BuildUploadAttributes struct {
	CFBundleShortVersionString string            `json:"cfBundleShortVersionString,omitempty"`
	CFBundleVersion            string            `json:"cfBundleVersion,omitempty"`
	CreatedDate                *time.Time        `json:"createdDate,omitempty"`
	State                      *BuildUploadState `json:"state,omitempty"`
	Platform                   string            `json:"platform,omitempty"`
	UploadedDate               *time.Time        `json:"uploadedDate,omitempty"`
}

// BuildUploadState describes upload processing progress. State is one of
// AWAITING_UPLOAD, PROCESSING, FAILED, or COMPLETE; the detail slices
// carry Apple's per-issue codes and descriptions.
type BuildUploadState struct {
	Errors   []StateDetail `json:"errors,omitempty"`
	Warnings []StateDetail `json:"warnings,omitempty"`
	Infos    []StateDetail `json:"infos,omitempty"`
	State    string        `json:"state,omitempty"`
}

// StateDetail is a single coded message inside a BuildUploadState.
type StateDetail struct {
	Code        string `json:"code,omitempty"`
	Description string `json:"description,omitempty"`
}

// BuildUploadRelationships exposes the resources linked to a build
// upload; Build is populated once Apple has processed the binary.
type BuildUploadRelationships struct {
	Build     *RelationshipData `json:"build,omitempty"`
	AssetFile *RelationshipData `json:"assetFile,omitempty"`
}

// BuildUploadCreateRequest represents a request to create a build upload.
type BuildUploadCreateRequest struct {
	Data BuildUploadCreateData `json:"data"`
}

// BuildUploadCreateData contains the data for creating a build upload.
type BuildUploadCreateData struct {
	Type          string                         `json:"type"`
	Attributes    BuildUploadCreateAttributes    `json:"attributes"`
	Relationships BuildUploadCreateRelationships `json:"relationships"`
}

// BuildUploadCreateAttributes contains attributes for creating a build
// upload. All fields are required by the API.
type BuildUploadCreateAttributes struct {
	CFBundleShortVersionString string `json:"cfBundleShortVersionString"`
	CFBundleVersion            string `json:"cfBundleVersion"`
	Platform                   string `json:"platform"`
}

// BuildUploadCreateRelationships contains relationships for creating a
// build upload.
type BuildUploadCreateRelationships struct {
	App RelationshipData `json:"app"`
}

// Build upload file types

// BuildUploadFilesResponse represents a list of build upload files.
type BuildUploadFilesResponse struct {
	Data  []BuildUploadFile  `json:"data"`
	Links PagedDocumentLinks `json:"links"`
	Meta  *PagingInformation `json:"meta,omitempty"`
}

// BuildUploadFileResponse represents a single build upload file.
type BuildUploadFileResponse struct {
	Data BuildUploadFile `json:"data"`
}

// BuildUploadFile represents one asset file inside a build upload.
type BuildUploadFile struct {
	Type       string                    `json:"type"`
	ID         string                    `json:"id"`
	Attributes BuildUploadFileAttributes `json:"attributes"`
}

// BuildUploadFileAttributes contains build upload file attributes,
// including the upload operations issued at reservation time.
type BuildUploadFileAttributes struct {
	AssetDeliveryState  *AssetDeliveryState `json:"assetDeliveryState,omitempty"`
	AssetToken          string              `json:"assetToken,omitempty"`
	AssetType           string              `json:"assetType,omitempty"`
	FileName            string              `json:"fileName,omitempty"`
	FileSize            int64               `json:"fileSize,omitempty"`
	SourceFileChecksums *Checksums          `json:"sourceFileChecksums,omitempty"`
	UploadOperations    []UploadOperation   `json:"uploadOperations,omitempty"`
	UTI                 string              `json:"uti,omitempty"`
}

// Checksums carries the file (and optional composite) checksum used to
// commit a build upload file.
type Checksums struct {
	File      *Checksum `json:"file,omitempty"`
	Composite *Checksum `json:"composite,omitempty"`
}

// Checksum is a single hash value with its algorithm (MD5 or SHA_256).
type Checksum struct {
	Hash      string `json:"hash,omitempty"`
	Algorithm string `json:"algorithm,omitempty"`
}

// BuildUploadFileCreateRequest represents a request to reserve a build
// upload file.
type BuildUploadFileCreateRequest struct {
	Data BuildUploadFileCreateData `json:"data"`
}

// BuildUploadFileCreateData contains the data for reserving a build
// upload file.
type BuildUploadFileCreateData struct {
	Type          string                             `json:"type"`
	Attributes    BuildUploadFileCreateAttributes    `json:"attributes"`
	Relationships BuildUploadFileCreateRelationships `json:"relationships"`
}

// BuildUploadFileCreateAttributes contains attributes for reserving a
// build upload file. All fields are required by the API.
type BuildUploadFileCreateAttributes struct {
	AssetType string `json:"assetType"`
	FileName  string `json:"fileName"`
	FileSize  int64  `json:"fileSize"`
	UTI       string `json:"uti"`
}

// BuildUploadFileCreateRelationships contains relationships for
// reserving a build upload file.
type BuildUploadFileCreateRelationships struct {
	BuildUpload RelationshipData `json:"buildUpload"`
}

// BuildUploadFileUpdateRequest represents the commit PATCH for a build
// upload file.
type BuildUploadFileUpdateRequest struct {
	Data BuildUploadFileUpdateData `json:"data"`
}

// BuildUploadFileUpdateData contains the data for committing a build
// upload file.
type BuildUploadFileUpdateData struct {
	Type       string                           `json:"type"`
	ID         string                           `json:"id"`
	Attributes *BuildUploadFileUpdateAttributes `json:"attributes,omitempty"`
}

// BuildUploadFileUpdateAttributes contains attributes for committing a
// build upload file.
type BuildUploadFileUpdateAttributes struct {
	SourceFileChecksums *Checksums `json:"sourceFileChecksums,omitempty"`
	Uploaded            *bool      `json:"uploaded,omitempty"`
}

// ListBuildUploads returns the build uploads for an app.
func (c *Client) ListBuildUploads(ctx context.Context, appID string, opts *ListOptions) (*BuildUploadsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/apps/"+url.PathEscape(appID)+"/buildUploads", query)
	if err != nil {
		return nil, err
	}

	var resp BuildUploadsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetBuildUpload returns a single build upload.
func (c *Client) GetBuildUpload(ctx context.Context, buildUploadID string, opts *ListOptions) (*BuildUploadResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/buildUploads/"+url.PathEscape(buildUploadID), query)
	if err != nil {
		return nil, err
	}

	var resp BuildUploadResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateBuildUpload starts a new build upload for an app.
func (c *Client) CreateBuildUpload(ctx context.Context, req *BuildUploadCreateRequest) (*BuildUploadResponse, error) {
	data, err := c.Post(ctx, "/v1/buildUploads", req)
	if err != nil {
		return nil, err
	}

	var resp BuildUploadResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteBuildUpload deletes a build upload (e.g. an abandoned or failed
// delivery).
func (c *Client) DeleteBuildUpload(ctx context.Context, buildUploadID string) error {
	return c.Delete(ctx, "/v1/buildUploads/"+url.PathEscape(buildUploadID))
}

// ListBuildUploadFiles returns the asset files attached to a build
// upload, including their delivery states.
func (c *Client) ListBuildUploadFiles(ctx context.Context, buildUploadID string, opts *ListOptions) (*BuildUploadFilesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/buildUploads/"+url.PathEscape(buildUploadID)+"/buildUploadFiles", query)
	if err != nil {
		return nil, err
	}

	var resp BuildUploadFilesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateBuildUploadFile reserves an asset file slot on a build upload.
// The response carries the upload operations for the file's bytes.
func (c *Client) CreateBuildUploadFile(ctx context.Context, req *BuildUploadFileCreateRequest) (*BuildUploadFileResponse, error) {
	data, err := c.Post(ctx, "/v1/buildUploadFiles", req)
	if err != nil {
		return nil, err
	}

	var resp BuildUploadFileResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateBuildUploadFile commits (or amends) a build upload file.
func (c *Client) UpdateBuildUploadFile(ctx context.Context, fileID string, req *BuildUploadFileUpdateRequest) (*BuildUploadFileResponse, error) {
	data, err := c.Patch(ctx, "/v1/buildUploadFiles/"+url.PathEscape(fileID), req)
	if err != nil {
		return nil, err
	}

	var resp BuildUploadFileResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UploadBuildFile reserves, uploads, and commits one asset file on an
// existing build upload. assetType is ASSET, ASSET_DESCRIPTION, or
// ASSET_SPI; uti must be one of Apple's accepted identifiers (e.g.
// com.apple.ipa). The commit carries the SHA-256 of the body so Apple
// can verify the stored bytes.
func (c *Client) UploadBuildFile(ctx context.Context, buildUploadID, fileName, assetType, uti string, body []byte) (*BuildUploadFileResponse, error) {
	if fileName == "" {
		return nil, fmt.Errorf("file_name is required")
	}
	if len(body) == 0 {
		return nil, fmt.Errorf("file body is empty")
	}
	if len(body) > MaxUploadSize {
		return nil, fmt.Errorf("file body exceeds max size of %d bytes", MaxUploadSize)
	}
	createReq := &BuildUploadFileCreateRequest{
		Data: BuildUploadFileCreateData{
			Type: "buildUploadFiles",
			Attributes: BuildUploadFileCreateAttributes{
				AssetType: assetType,
				FileName:  fileName,
				FileSize:  int64(len(body)),
				UTI:       uti,
			},
			Relationships: BuildUploadFileCreateRelationships{
				BuildUpload: RelationshipData{
					Data: ResourceIdentifier{Type: "buildUploads", ID: buildUploadID},
				},
			},
		},
	}
	reservation, err := c.CreateBuildUploadFile(ctx, createReq)
	if err != nil {
		return nil, fmt.Errorf("reservation: %w", err)
	}
	ops := reservation.Data.Attributes.UploadOperations
	if err := c.PerformUploadOperations(ctx, ops, body); err != nil {
		return nil, fmt.Errorf("upload: %w", err)
	}
	uploaded := true
	commit := &BuildUploadFileUpdateRequest{
		Data: BuildUploadFileUpdateData{
			Type: "buildUploadFiles",
			ID:   reservation.Data.ID,
			Attributes: &BuildUploadFileUpdateAttributes{
				SourceFileChecksums: &Checksums{
					File: &Checksum{Hash: Sha256Hex(body), Algorithm: "SHA_256"},
				},
				Uploaded: &uploaded,
			},
		},
	}
	return c.UpdateBuildUploadFile(ctx, reservation.Data.ID, commit)
}
