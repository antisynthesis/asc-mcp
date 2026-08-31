package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerBuildUploadTools registers build upload tools (App Store
// Connect API 4.1+), which deliver build binaries through the API
// instead of Transporter/altool.
func (r *Registry) registerBuildUploadTools() {
	// Start build upload
	r.register(mcp.Tool{
		Name:        "start_build_upload",
		Description: "Start a build upload for an app. Creates the buildUploads record that upload_build_file delivers the binary into. The version attributes must match the binary's Info.plist values.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to upload a build for",
				},
				"cf_bundle_short_version_string": {
					Type:        "string",
					Description: "The binary's CFBundleShortVersionString (marketing version, e.g. 1.2.3)",
				},
				"cf_bundle_version": {
					Type:        "string",
					Description: "The binary's CFBundleVersion (build number, e.g. 42)",
				},
				"platform": {
					Type:        "string",
					Description: "The platform: IOS, MAC_OS, TV_OS, or VISION_OS",
					Enum:        []string{"IOS", "MAC_OS", "TV_OS", "VISION_OS"},
				},
			},
			Required: []string{"app_id", "cf_bundle_short_version_string", "cf_bundle_version", "platform"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Start Build Upload",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleStartBuildUpload)

	// Upload build file
	r.register(mcp.Tool{
		Name:        "upload_build_file",
		Description: "Reserve, upload, and commit a build binary (or auxiliary asset) on an existing build upload created by start_build_upload. Provide exactly one of file_path or file_data_base64. After committing, poll get_build_upload until the state is COMPLETE or FAILED.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"build_upload_id": {
					Type:        "string",
					Description: "The Build Upload ID returned by start_build_upload",
				},
				"file_name": {
					Type:        "string",
					Description: "The original file name (e.g. MyApp.ipa)",
				},
				"asset_type": {
					Type:        "string",
					Description: "The asset slot to fill: ASSET (the binary itself, default), ASSET_DESCRIPTION, or ASSET_SPI",
					Enum:        []string{"ASSET", "ASSET_DESCRIPTION", "ASSET_SPI"},
				},
				"uti": {
					Type:        "string",
					Description: "Uniform type identifier of the file. Defaults to com.apple.ipa.",
					Enum:        []string{"com.apple.ipa", "com.apple.pkg", "com.pkware.zip-archive", "com.apple.binary-property-list", "com.apple.xml-property-list"},
				},
				"file_path": {
					Type:        "string",
					Description: "Absolute path to the file on the server's local filesystem. Use this when the MCP server and the file are on the same host (most stdio scenarios).",
				},
				"file_data_base64": {
					Type:        "string",
					Description: "Standard base64 (RFC 4648) encoded bytes of the file. Use this when the client must transport the file across a remote transport such as HTTP.",
				},
			},
			Required: []string{"build_upload_id", "file_name"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Upload Build File",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUploadBuildFile)

	// Get build upload
	r.register(mcp.Tool{
		Name:        "get_build_upload",
		Description: "Get the status of a build upload, including its processing state (AWAITING_UPLOAD, PROCESSING, FAILED, COMPLETE), any state errors or warnings, and the resulting build ID once processing completes",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"build_upload_id": {
					Type:        "string",
					Description: "The Build Upload ID",
				},
			},
			Required: []string{"build_upload_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Build Upload",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetBuildUpload)

	// List build uploads
	r.register(mcp.Tool{
		Name:        "list_build_uploads",
		Description: "List the build uploads for an app, including their processing states",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to list build uploads for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of build uploads to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
				"filter": {
					Type:        "object",
					Description: "JSON:API filter map. Supported keys: cfBundleShortVersionString, cfBundleVersion, platform (IOS, MAC_OS, TV_OS, VISION_OS), state. Values are arrays, e.g. {\"state\": [\"PROCESSING\"]}.",
				},
				"sort": {
					Type:        "array",
					Description: "Sort order, e.g. -uploadedDate, cfBundleVersion",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Build Uploads",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListBuildUploads)

	// List build upload files
	r.register(mcp.Tool{
		Name:        "list_build_upload_files",
		Description: "List the asset files attached to a build upload, including each file's asset delivery state",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"build_upload_id": {
					Type:        "string",
					Description: "The Build Upload ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of files to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
			},
			Required: []string{"build_upload_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Build Upload Files",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListBuildUploadFiles)

	// Delete build upload
	r.register(mcp.Tool{
		Name:        "delete_build_upload",
		Description: "Delete a build upload (e.g. an abandoned or failed delivery)",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"build_upload_id": {
					Type:        "string",
					Description: "The Build Upload ID",
				},
			},
			Required: []string{"build_upload_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Build Upload",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteBuildUpload)
}

func (r *Registry) handleStartBuildUpload(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID                      string `json:"app_id"`
		CFBundleShortVersionString string `json:"cf_bundle_short_version_string"`
		CFBundleVersion            string `json:"cf_bundle_version"`
		Platform                   string `json:"platform"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}
	if params.CFBundleShortVersionString == "" {
		return mcp.NewErrorResult("cf_bundle_short_version_string is required"), nil
	}
	if params.CFBundleVersion == "" {
		return mcp.NewErrorResult("cf_bundle_version is required"), nil
	}
	if params.Platform == "" {
		return mcp.NewErrorResult("platform is required"), nil
	}

	req := &api.BuildUploadCreateRequest{
		Data: api.BuildUploadCreateData{
			Type: "buildUploads",
			Attributes: api.BuildUploadCreateAttributes{
				CFBundleShortVersionString: params.CFBundleShortVersionString,
				CFBundleVersion:            params.CFBundleVersion,
				Platform:                   params.Platform,
			},
			Relationships: api.BuildUploadCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "apps",
						ID:   params.AppID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateBuildUpload(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to start build upload: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Started build upload:\n%s\nNext, call upload_build_file with build_upload_id=%s to deliver the binary.",
		formatBuildUpload(resp.Data), resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleUploadBuildFile(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BuildUploadID  string `json:"build_upload_id"`
		FileName       string `json:"file_name"`
		AssetType      string `json:"asset_type"`
		UTI            string `json:"uti"`
		FilePath       string `json:"file_path"`
		FileDataBase64 string `json:"file_data_base64"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("invalid arguments: %v", err)), nil
	}

	if params.BuildUploadID == "" {
		return mcp.NewErrorResult("build_upload_id is required"), nil
	}
	if params.FileName == "" {
		return mcp.NewErrorResult("file_name is required"), nil
	}
	assetType := params.AssetType
	if assetType == "" {
		assetType = "ASSET"
	}
	uti := params.UTI
	if uti == "" {
		uti = "com.apple.ipa"
	}
	body, err := loadUploadBody(params.FilePath, params.FileDataBase64)
	if err != nil {
		return mcp.NewErrorResult(err.Error()), nil
	}

	resp, err := r.client.UploadBuildFile(ctx, params.BuildUploadID, params.FileName, assetType, uti, body)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to upload build file: %v", err)), nil
	}

	text := fmt.Sprintf("Uploaded build file %s (%s, %d bytes) to build upload %s. Poll get_build_upload for processing status.",
		resp.Data.ID, params.FileName, len(body), params.BuildUploadID)
	return newDataResult(text, resp.Data), nil
}

func (r *Registry) handleGetBuildUpload(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BuildUploadID string `json:"build_upload_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BuildUploadID == "" {
		return mcp.NewErrorResult("build_upload_id is required"), nil
	}

	resp, err := r.client.GetBuildUpload(ctx, params.BuildUploadID, nil)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get build upload: %v", err)), nil
	}

	return newDataResult(formatBuildUpload(resp.Data), resp.Data), nil
}

func (r *Registry) handleListBuildUploads(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID  string              `json:"app_id"`
		Limit  int                 `json:"limit"`
		Cursor string              `json:"cursor"`
		Filter map[string][]string `json:"filter"`
		Sort   []string            `json:"sort"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.BuildUploadsResponse, error) {
		return r.client.ListBuildUploads(ctx, params.AppID, listOpts(limit, params.Filter, params.Sort, nil, nil))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list build uploads: %v", err)), nil
	}

	return newListResult(formatBuildUploads(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleListBuildUploadFiles(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BuildUploadID string `json:"build_upload_id"`
		Limit         int    `json:"limit"`
		Cursor        string `json:"cursor"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BuildUploadID == "" {
		return mcp.NewErrorResult("build_upload_id is required"), nil
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.BuildUploadFilesResponse, error) {
		return r.client.ListBuildUploadFiles(ctx, params.BuildUploadID, listOpts(limit, nil, nil, nil, nil))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list build upload files: %v", err)), nil
	}

	return newListResult(formatBuildUploadFiles(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleDeleteBuildUpload(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BuildUploadID string `json:"build_upload_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BuildUploadID == "" {
		return mcp.NewErrorResult("build_upload_id is required"), nil
	}

	if err := r.client.DeleteBuildUpload(ctx, params.BuildUploadID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete build upload: %v", err)), nil
	}

	return mcp.NewSuccessResult("Build upload deleted successfully"), nil
}

func formatBuildUploads(uploads []api.BuildUpload) string {
	if len(uploads) == 0 {
		return "No build uploads found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d build uploads:\n\n", len(uploads)))

	for _, upload := range uploads {
		sb.WriteString(formatBuildUpload(upload))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatBuildUpload(upload api.BuildUpload) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Build Upload ID: %s\n", upload.ID))
	if upload.Attributes.CFBundleShortVersionString != "" {
		sb.WriteString(fmt.Sprintf("Version: %s (%s)\n",
			upload.Attributes.CFBundleShortVersionString, upload.Attributes.CFBundleVersion))
	}
	if upload.Attributes.Platform != "" {
		sb.WriteString(fmt.Sprintf("Platform: %s\n", upload.Attributes.Platform))
	}
	if state := upload.Attributes.State; state != nil {
		sb.WriteString(fmt.Sprintf("State: %s\n", state.State))
		for _, detail := range state.Errors {
			sb.WriteString(fmt.Sprintf("Error [%s]: %s\n", detail.Code, detail.Description))
		}
		for _, detail := range state.Warnings {
			sb.WriteString(fmt.Sprintf("Warning [%s]: %s\n", detail.Code, detail.Description))
		}
	}
	if upload.Attributes.CreatedDate != nil {
		sb.WriteString(fmt.Sprintf("Created: %s\n", upload.Attributes.CreatedDate.Format("2006-01-02 15:04:05")))
	}
	if upload.Attributes.UploadedDate != nil {
		sb.WriteString(fmt.Sprintf("Uploaded: %s\n", upload.Attributes.UploadedDate.Format("2006-01-02 15:04:05")))
	}
	if rel := upload.Relationships; rel != nil && rel.Build != nil && rel.Build.Data.ID != "" {
		sb.WriteString(fmt.Sprintf("Build ID: %s\n", rel.Build.Data.ID))
	}
	return sb.String()
}

func formatBuildUploadFiles(files []api.BuildUploadFile) string {
	if len(files) == 0 {
		return "No build upload files found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d build upload files:\n\n", len(files)))

	for _, file := range files {
		sb.WriteString(fmt.Sprintf("File ID: %s\n", file.ID))
		if file.Attributes.FileName != "" {
			sb.WriteString(fmt.Sprintf("Name: %s (%d bytes)\n", file.Attributes.FileName, file.Attributes.FileSize))
		}
		if file.Attributes.AssetType != "" {
			sb.WriteString(fmt.Sprintf("Asset Type: %s\n", file.Attributes.AssetType))
		}
		if file.Attributes.UTI != "" {
			sb.WriteString(fmt.Sprintf("UTI: %s\n", file.Attributes.UTI))
		}
		if file.Attributes.AssetDeliveryState != nil && file.Attributes.AssetDeliveryState.State != "" {
			sb.WriteString(fmt.Sprintf("Delivery State: %s\n", file.Attributes.AssetDeliveryState.State))
		}
		sb.WriteString("\n---\n")
	}

	return sb.String()
}
