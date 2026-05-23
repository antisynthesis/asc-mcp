// Package tools provides MCP tool implementations for App Store Connect.
package tools

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// MaxLoadedFileSize bounds how large an upload payload may be when
// loaded from disk or decoded from base64 before being passed to the
// API client. The API layer also enforces api.MaxUploadSize.
const MaxLoadedFileSize = api.MaxUploadSize

// registerUploadTools registers the three asset upload tools.
func (r *Registry) registerUploadTools() {
	pathArg := mcp.Property{
		Type:        "string",
		Description: "Absolute path to the file on the server's local filesystem. Use this when the MCP server and the file are on the same host (most stdio scenarios).",
	}
	dataArg := mcp.Property{
		Type:        "string",
		Description: "Standard base64 (RFC 4648) encoded bytes of the file. Use this when the client must transport the file across a remote transport such as HTTP.",
	}

	r.register(mcp.Tool{
		Name:        "upload_app_screenshot",
		Description: "Reserve, upload, and commit an App Store screenshot to an existing screenshot set. Provide exactly one of file_path or file_data_base64. Returns the final screenshot record once Apple has confirmed the bytes.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"screenshot_set_id": {Type: "string", Description: "The App Screenshot Set ID that the new screenshot will belong to."},
				"file_name":         {Type: "string", Description: "The original file name (e.g. iphone6.5-1.png). Apple uses this verbatim in the asset URL."},
				"file_path":         pathArg,
				"file_data_base64":  dataArg,
			},
			Required: []string{"screenshot_set_id", "file_name"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Upload App Screenshot",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUploadAppScreenshot)

	r.register(mcp.Tool{
		Name:        "upload_app_preview",
		Description: "Reserve, upload, and commit an App Store preview video to an existing preview set. Provide exactly one of file_path or file_data_base64. mime_type defaults to video/mp4 if omitted. Returns the final preview record once Apple has confirmed the bytes.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"preview_set_id":   {Type: "string", Description: "The App Preview Set ID that the new preview will belong to."},
				"file_name":        {Type: "string", Description: "The original file name (e.g. iphone6.5-1.mp4)."},
				"mime_type":        {Type: "string", Description: "MIME type of the preview file. Defaults to video/mp4."},
				"file_path":        pathArg,
				"file_data_base64": dataArg,
			},
			Required: []string{"preview_set_id", "file_name"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Upload App Preview",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUploadAppPreview)

	r.register(mcp.Tool{
		Name:        "upload_review_attachment",
		Description: "Reserve, upload, and commit an App Store review attachment (PDF, screenshot, or other supporting material) to an App Store Review Detail. Provide exactly one of file_path or file_data_base64.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"review_detail_id": {Type: "string", Description: "The App Store Review Detail ID that the attachment will belong to."},
				"file_name":        {Type: "string", Description: "The original file name."},
				"file_path":        pathArg,
				"file_data_base64": dataArg,
			},
			Required: []string{"review_detail_id", "file_name"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Upload Review Attachment",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUploadReviewAttachment)
}

// loadUploadBody returns the bytes to upload, resolving exactly one of
// the two supported sources. file_path is preferred for local stdio
// sessions; file_data_base64 is for remote transports.
func loadUploadBody(filePath, fileDataBase64 string) ([]byte, error) {
	hasPath := filePath != ""
	hasData := fileDataBase64 != ""
	if hasPath == hasData {
		return nil, fmt.Errorf("provide exactly one of file_path or file_data_base64")
	}
	if hasData {
		// Accept both standard and URL-safe base64, with or without
		// padding, so callers don't have to think about which flavor
		// their transport produced.
		b, err := decodeBase64Flexible(fileDataBase64)
		if err != nil {
			return nil, fmt.Errorf("file_data_base64 is not valid base64: %w", err)
		}
		if len(b) > MaxLoadedFileSize {
			return nil, fmt.Errorf("file_data exceeds max size of %d bytes", MaxLoadedFileSize)
		}
		return b, nil
	}
	abs, err := filepath.Abs(filePath)
	if err != nil {
		return nil, fmt.Errorf("resolve file_path: %w", err)
	}
	info, err := os.Stat(abs)
	if err != nil {
		return nil, fmt.Errorf("stat file_path: %w", err)
	}
	if info.IsDir() {
		return nil, fmt.Errorf("file_path is a directory")
	}
	if info.Size() > MaxLoadedFileSize {
		return nil, fmt.Errorf("file at %s exceeds max size of %d bytes", abs, MaxLoadedFileSize)
	}
	return os.ReadFile(abs)
}

// decodeBase64Flexible decodes data as base64, accepting standard and
// URL-safe alphabets with optional padding.
func decodeBase64Flexible(s string) ([]byte, error) {
	for _, enc := range []*base64.Encoding{
		base64.StdEncoding,
		base64.RawStdEncoding,
		base64.URLEncoding,
		base64.RawURLEncoding,
	} {
		if b, err := enc.DecodeString(s); err == nil {
			return b, nil
		}
	}
	return nil, fmt.Errorf("unrecognized base64 encoding")
}

func (r *Registry) handleUploadAppScreenshot(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ScreenshotSetID string `json:"screenshot_set_id"`
		FileName        string `json:"file_name"`
		FilePath        string `json:"file_path"`
		FileDataBase64  string `json:"file_data_base64"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("invalid arguments: %v", err)), nil
	}
	if params.ScreenshotSetID == "" {
		return mcp.NewErrorResult("screenshot_set_id is required"), nil
	}
	if params.FileName == "" {
		return mcp.NewErrorResult("file_name is required"), nil
	}
	body, err := loadUploadBody(params.FilePath, params.FileDataBase64)
	if err != nil {
		return mcp.NewErrorResult(err.Error()), nil
	}
	resp, err := r.client.UploadAppScreenshot(ctx, params.ScreenshotSetID, params.FileName, body)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to upload screenshot: %v", err)), nil
	}
	text := fmt.Sprintf("Uploaded screenshot %s (%s, %d bytes) to set %s.",
		resp.Data.ID, params.FileName, len(body), params.ScreenshotSetID)
	return newDataResult(text, resp.Data), nil
}

func (r *Registry) handleUploadAppPreview(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		PreviewSetID   string `json:"preview_set_id"`
		FileName       string `json:"file_name"`
		MimeType       string `json:"mime_type"`
		FilePath       string `json:"file_path"`
		FileDataBase64 string `json:"file_data_base64"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("invalid arguments: %v", err)), nil
	}
	if params.PreviewSetID == "" {
		return mcp.NewErrorResult("preview_set_id is required"), nil
	}
	if params.FileName == "" {
		return mcp.NewErrorResult("file_name is required"), nil
	}
	body, err := loadUploadBody(params.FilePath, params.FileDataBase64)
	if err != nil {
		return mcp.NewErrorResult(err.Error()), nil
	}
	resp, err := r.client.UploadAppPreview(ctx, params.PreviewSetID, params.FileName, params.MimeType, body)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to upload preview: %v", err)), nil
	}
	text := fmt.Sprintf("Uploaded preview %s (%s, %d bytes) to set %s.",
		resp.Data.ID, params.FileName, len(body), params.PreviewSetID)
	return newDataResult(text, resp.Data), nil
}

func (r *Registry) handleUploadReviewAttachment(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ReviewDetailID string `json:"review_detail_id"`
		FileName       string `json:"file_name"`
		FilePath       string `json:"file_path"`
		FileDataBase64 string `json:"file_data_base64"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("invalid arguments: %v", err)), nil
	}
	if params.ReviewDetailID == "" {
		return mcp.NewErrorResult("review_detail_id is required"), nil
	}
	if params.FileName == "" {
		return mcp.NewErrorResult("file_name is required"), nil
	}
	body, err := loadUploadBody(params.FilePath, params.FileDataBase64)
	if err != nil {
		return mcp.NewErrorResult(err.Error()), nil
	}
	resp, err := r.client.UploadAppStoreReviewAttachment(ctx, params.ReviewDetailID, params.FileName, body)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to upload review attachment: %v", err)), nil
	}
	text := fmt.Sprintf("Uploaded review attachment %s (%s, %d bytes) to review detail %s.",
		resp.Data.ID, params.FileName, len(body), params.ReviewDetailID)
	return newDataResult(text, resp.Data), nil
}
