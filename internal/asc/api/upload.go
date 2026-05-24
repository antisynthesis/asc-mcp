// Package api provides upload helpers for App Store Connect assets.
//
// App Store Connect uses a three-step upload protocol for screenshots,
// previews, review attachments, and similar binary assets:
//
//  1. Reservation — POST to the asset endpoint with the file name,
//     size, and (for some asset types) MIME type. The response
//     contains an UploadOperations list describing how to send the
//     bytes in one or more chunks.
//  2. Upload — issue each UploadOperation against its target URL with
//     the supplied headers and the matching slice of the file body.
//     These targets are signed pre-authorized URLs at Apple's storage
//     edge; they do NOT use the App Store Connect bearer token.
//  3. Commit — PATCH the asset with the SHA-256 checksum of the file
//     and uploaded=true so Apple can verify the bytes were stored
//     intact.
//
// PerformUploadOperations performs step 2 against a fully reserved
// asset's operations. The reservation and commit steps are still issued
// via the resource-specific Client methods; the tool layer composes
// them via UploadAndCommit.
package api

import (
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"io"
	"net/http"
	"time"
)

// MaxUploadSize bounds how large a single asset upload may be. Apple's
// limits vary per asset type (~10 MiB for screenshots, ~500 MiB for
// previews); we cap at the larger of the two to protect the process
// while keeping legitimate uploads possible.
const MaxUploadSize = 600 * 1024 * 1024 // 600 MiB

// UploadHTTPTimeout is the per-chunk request timeout for asset uploads.
// Apple's pre-signed URLs accept large chunks, and a fixed 30s like the
// API client is too aggressive for big preview videos. The caller's
// context still bounds the overall upload.
const UploadHTTPTimeout = 5 * time.Minute

// PerformUploadOperations sends each upload operation to its
// pre-authorized URL. It is the caller's responsibility to have
// already issued the reservation that returned `ops` and to call
// CommitUpload afterwards with the SHA-256 of `body`.
func (c *Client) PerformUploadOperations(ctx context.Context, ops []UploadOperation, body []byte) error {
	if len(body) > MaxUploadSize {
		return fmt.Errorf("upload body exceeds max size of %d bytes", MaxUploadSize)
	}
	if len(ops) == 0 {
		return fmt.Errorf("no upload operations to perform")
	}
	hc := &http.Client{Timeout: UploadHTTPTimeout}
	for i, op := range ops {
		if op.Offset < 0 || op.Length <= 0 || op.Offset+op.Length > len(body) {
			return fmt.Errorf("upload op %d out of bounds: offset=%d length=%d body=%d",
				i, op.Offset, op.Length, len(body))
		}
		chunk := body[op.Offset : op.Offset+op.Length]
		method := op.Method
		if method == "" {
			method = http.MethodPut
		}
		req, err := http.NewRequestWithContext(ctx, method, op.URL, bytes.NewReader(chunk))
		if err != nil {
			return fmt.Errorf("upload op %d: build request: %w", i, err)
		}
		for _, h := range op.RequestHeaders {
			req.Header.Set(h.Name, h.Value)
		}
		resp, err := hc.Do(req)
		if err != nil {
			return fmt.Errorf("upload op %d: %w", i, err)
		}
		// Drain and close so the connection can be reused for the next chunk.
		respBody, _ := io.ReadAll(io.LimitReader(resp.Body, 64*1024))
		resp.Body.Close()
		if resp.StatusCode >= 400 {
			return fmt.Errorf("upload op %d failed with %d: %s",
				i, resp.StatusCode, string(respBody))
		}
	}
	return nil
}

// Sha256Hex returns the lowercase hex SHA-256 of body, which is what
// Apple expects for the commit PATCH on an asset upload.
func Sha256Hex(body []byte) string {
	sum := sha256.Sum256(body)
	return hex.EncodeToString(sum[:])
}

// UploadAppScreenshot reserves, uploads, and commits a screenshot in
// one call. The reservation is created against the supplied screenshot
// set; the response includes the final screenshot record after the
// commit PATCH has been accepted.
func (c *Client) UploadAppScreenshot(ctx context.Context, screenshotSetID, fileName string, body []byte) (*AppScreenshotResponse, error) {
	if fileName == "" {
		return nil, fmt.Errorf("file_name is required")
	}
	if len(body) == 0 {
		return nil, fmt.Errorf("file body is empty")
	}
	if len(body) > MaxUploadSize {
		return nil, fmt.Errorf("file body exceeds max size of %d bytes", MaxUploadSize)
	}
	uploaded := false
	createReq := &AppScreenshotCreateRequest{
		Data: AppScreenshotCreateData{
			Type: "appScreenshots",
			Attributes: AppScreenshotCreateAttributes{
				FileName: fileName,
				FileSize: len(body),
			},
			Relationships: AppScreenshotCreateRelationships{
				AppScreenshotSet: RelationshipData{
					Data: ResourceIdentifier{Type: "appScreenshotSets", ID: screenshotSetID},
				},
			},
		},
	}
	reservation, err := c.CreateAppScreenshot(ctx, createReq)
	if err != nil {
		return nil, fmt.Errorf("reservation: %w", err)
	}
	ops := reservation.Data.Attributes.UploadOperations
	if err := c.PerformUploadOperations(ctx, ops, body); err != nil {
		return nil, fmt.Errorf("upload: %w", err)
	}
	uploaded = true
	commit := &AppScreenshotUpdateRequest{
		Data: AppScreenshotUpdateData{
			Type: "appScreenshots",
			ID:   reservation.Data.ID,
			Attributes: AppScreenshotUpdateAttributes{
				SourceFileChecksum: Sha256Hex(body),
				Uploaded:           &uploaded,
			},
		},
	}
	return c.UpdateAppScreenshot(ctx, reservation.Data.ID, commit)
}

// UploadAppPreview reserves, uploads, and commits an app preview video.
// The MIME type defaults to video/mp4 if not provided.
func (c *Client) UploadAppPreview(ctx context.Context, previewSetID, fileName, mimeType string, body []byte) (*AppPreviewResponse, error) {
	if fileName == "" {
		return nil, fmt.Errorf("file_name is required")
	}
	if len(body) == 0 {
		return nil, fmt.Errorf("file body is empty")
	}
	if len(body) > MaxUploadSize {
		return nil, fmt.Errorf("file body exceeds max size of %d bytes", MaxUploadSize)
	}
	if mimeType == "" {
		mimeType = "video/mp4"
	}
	uploaded := false
	createReq := &AppPreviewCreateRequest{
		Data: AppPreviewCreateData{
			Type: "appPreviews",
			Attributes: AppPreviewCreateAttributes{
				FileName:             fileName,
				FileSize:             len(body),
				MimeType:             mimeType,
				PreviewFrameTimeCode: "",
			},
			Relationships: AppPreviewCreateRelationships{
				AppPreviewSet: RelationshipData{
					Data: ResourceIdentifier{Type: "appPreviewSets", ID: previewSetID},
				},
			},
		},
	}
	reservation, err := c.CreateAppPreview(ctx, createReq)
	if err != nil {
		return nil, fmt.Errorf("reservation: %w", err)
	}
	ops := reservation.Data.Attributes.UploadOperations
	if err := c.PerformUploadOperations(ctx, ops, body); err != nil {
		return nil, fmt.Errorf("upload: %w", err)
	}
	uploaded = true
	commit := &AppPreviewUpdateRequest{
		Data: AppPreviewUpdateData{
			Type: "appPreviews",
			ID:   reservation.Data.ID,
			Attributes: AppPreviewUpdateAttributes{
				SourceFileChecksum: Sha256Hex(body),
				Uploaded:           &uploaded,
			},
		},
	}
	return c.UpdateAppPreview(ctx, reservation.Data.ID, commit)
}

// UploadAppStoreReviewAttachment reserves, uploads, and commits a
// review attachment (typically a PDF or screenshot) bound to an App
// Store review detail.
func (c *Client) UploadAppStoreReviewAttachment(ctx context.Context, reviewDetailID, fileName string, body []byte) (*AppStoreReviewAttachmentResponse, error) {
	if fileName == "" {
		return nil, fmt.Errorf("file_name is required")
	}
	if len(body) == 0 {
		return nil, fmt.Errorf("file body is empty")
	}
	if len(body) > MaxUploadSize {
		return nil, fmt.Errorf("file body exceeds max size of %d bytes", MaxUploadSize)
	}
	uploaded := false
	createReq := &AppStoreReviewAttachmentCreateRequest{
		Data: AppStoreReviewAttachmentCreateData{
			Type: "appStoreReviewAttachments",
			Attributes: AppStoreReviewAttachmentCreateAttributes{
				FileName: fileName,
				FileSize: len(body),
			},
			Relationships: AppStoreReviewAttachmentCreateRelationships{
				AppStoreReviewDetail: RelationshipData{
					Data: ResourceIdentifier{Type: "appStoreReviewDetails", ID: reviewDetailID},
				},
			},
		},
	}
	reservation, err := c.CreateAppStoreReviewAttachment(ctx, createReq)
	if err != nil {
		return nil, fmt.Errorf("reservation: %w", err)
	}
	ops := reservation.Data.Attributes.UploadOperations
	if err := c.PerformUploadOperations(ctx, ops, body); err != nil {
		return nil, fmt.Errorf("upload: %w", err)
	}
	uploaded = true
	commit := &AppStoreReviewAttachmentUpdateRequest{
		Data: AppStoreReviewAttachmentUpdateData{
			Type: "appStoreReviewAttachments",
			ID:   reservation.Data.ID,
			Attributes: AppStoreReviewAttachmentUpdateAttributes{
				SourceFileChecksum: Sha256Hex(body),
				Uploaded:           &uploaded,
			},
		},
	}
	return c.UpdateAppStoreReviewAttachment(ctx, reservation.Data.ID, commit)
}
