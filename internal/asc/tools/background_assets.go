package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerBackgroundAssetTools registers Apple-Hosted Background Assets
// tools (App Store Connect API 4.0+). Background assets are asset packs
// Apple hosts and delivers to devices outside the app binary; each
// asset owns versions whose asset pack and manifest files are delivered
// via the reserve/upload/commit protocol and then released to internal
// beta, external beta (TestFlight), and the App Store.
func (r *Registry) registerBackgroundAssetTools() {
	// List background assets
	r.register(mcp.Tool{
		Name:        "list_background_assets",
		Description: "List the Apple-hosted background assets for an app, including each asset pack's identifier, archived state, and storage used (usedBytes). Filter versions.locale to read per-locale asset packs.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to list background assets for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of background assets to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
				"filter": {
					Type:        "object",
					Description: "JSON:API filter map. Supported keys: archived (true/false), assetPackIdentifier, versions.locale (e.g. en-US), versions.platforms (IOS, MAC_OS, TV_OS, VISION_OS). Values are arrays, e.g. {\"versions.locale\": [\"en-US\"]}.",
				},
				"sort": {
					Type:        "array",
					Description: "Sort order, e.g. assetPackIdentifier, -createdDate",
					Items:       &mcp.Property{Type: "string"},
				},
				"include": {
					Type:        "array",
					Description: "Related resource names to include in the response (supported: appStoreVersion, internalBetaVersion, externalBetaVersion).",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Background Assets",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListBackgroundAssets)

	// Get background asset
	r.register(mcp.Tool{
		Name:        "get_background_asset",
		Description: "Get details of a background asset, including its asset pack identifier, archived state, storage used (usedBytes), and the version currently live in each release channel",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"background_asset_id": {
					Type:        "string",
					Description: "The Background Asset ID",
				},
				"include": {
					Type:        "array",
					Description: "Related resource names to include in the response (supported: appStoreVersion, internalBetaVersion, externalBetaVersion).",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"background_asset_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Background Asset",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetBackgroundAsset)

	// Create background asset
	r.register(mcp.Tool{
		Name:        "create_background_asset",
		Description: "Create an Apple-hosted background asset for an app. The asset pack identifier must match the identifier your app requests at runtime. Next, call create_background_asset_version to open a version for upload.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to create the background asset for",
				},
				"asset_pack_identifier": {
					Type:        "string",
					Description: "The asset pack identifier (reverse-DNS style, e.g. com.example.game.levels1)",
				},
			},
			Required: []string{"app_id", "asset_pack_identifier"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Background Asset",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateBackgroundAsset)

	// Update background asset (archive/unarchive)
	r.register(mcp.Tool{
		Name:        "update_background_asset",
		Description: "Archive or unarchive a background asset. Archiving stops distribution of the asset pack without deleting it.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"background_asset_id": {
					Type:        "string",
					Description: "The Background Asset ID",
				},
				"archived": {
					Type:        "boolean",
					Description: "true to archive the asset, false to unarchive it",
				},
			},
			Required: []string{"background_asset_id", "archived"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update Background Asset",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateBackgroundAsset)

	// List background asset versions
	r.register(mcp.Tool{
		Name:        "list_background_asset_versions",
		Description: "List the versions of a background asset, including each version's processing state, state details, and per-channel release states (internal beta, external beta, App Store)",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"background_asset_id": {
					Type:        "string",
					Description: "The Background Asset ID to list versions for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of versions to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
				"filter": {
					Type:        "object",
					Description: "JSON:API filter map. Supported keys: locale, platforms (IOS, MAC_OS, TV_OS, VISION_OS), state (AWAITING_UPLOAD, PROCESSING, FAILED, COMPLETE), version, internalBetaRelease.state, externalBetaRelease.state, appStoreRelease.state. Values are arrays, e.g. {\"state\": [\"COMPLETE\"]}.",
				},
				"sort": {
					Type:        "array",
					Description: "Sort order, e.g. -version",
					Items:       &mcp.Property{Type: "string"},
				},
				"include": {
					Type:        "array",
					Description: "Related resource names to include in the response (supported: internalBetaRelease, externalBetaRelease, appStoreRelease, assetFile, manifestFile).",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"background_asset_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Background Asset Versions",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListBackgroundAssetVersions)

	// Get background asset version
	r.register(mcp.Tool{
		Name:        "get_background_asset_version",
		Description: "Get a background asset version, including its processing state (AWAITING_UPLOAD, PROCESSING, FAILED, COMPLETE), state detail errors and warnings, and the IDs of its per-channel release records",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The Background Asset Version ID",
				},
				"include": {
					Type:        "array",
					Description: "Related resource names to include in the response (supported: internalBetaRelease, externalBetaRelease, appStoreRelease, assetFile, manifestFile).",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"version_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Background Asset Version",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetBackgroundAssetVersion)

	// Create background asset version
	r.register(mcp.Tool{
		Name:        "create_background_asset_version",
		Description: "Create a new version on a background asset. Apple assigns the version number. Next, call upload_background_asset_file to deliver the asset pack and its manifest.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"background_asset_id": {
					Type:        "string",
					Description: "The Background Asset ID to create a version on",
				},
			},
			Required: []string{"background_asset_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Background Asset Version",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateBackgroundAssetVersion)

	// Upload background asset file
	r.register(mcp.Tool{
		Name:        "upload_background_asset_file",
		Description: "Reserve, upload, and commit one file on a background asset version created by create_background_asset_version. Upload the asset pack archive (asset_type ASSET) and its manifest plist (asset_type MANIFEST); both are required before Apple processes the version. Provide exactly one of file_path or file_data_base64. After committing both files, poll get_background_asset_version until the state is COMPLETE or FAILED.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The Background Asset Version ID returned by create_background_asset_version",
				},
				"file_name": {
					Type:        "string",
					Description: "The original file name (e.g. levels1.aar or Manifest.plist)",
				},
				"asset_type": {
					Type:        "string",
					Description: "The file slot to fill: ASSET (the asset pack archive, default) or MANIFEST (its manifest plist)",
					Enum:        []string{"ASSET", "MANIFEST"},
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
			Required: []string{"version_id", "file_name"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Upload Background Asset File",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUploadBackgroundAssetFile)

	// List background asset upload files
	r.register(mcp.Tool{
		Name:        "list_background_asset_upload_files",
		Description: "List the upload files (asset pack and manifest) attached to a background asset version, including each file's asset delivery state",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The Background Asset Version ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of files to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
			},
			Required: []string{"version_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Background Asset Upload Files",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListBackgroundAssetUploadFiles)

	// Get background asset version release
	r.register(mcp.Tool{
		Name:        "get_background_asset_version_release",
		Description: "Get the release state of a background asset version in one channel: internal beta, external beta (TestFlight), or App Store. Release IDs come from get_background_asset_version or list_background_asset_versions relationships/includes.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"release_type": {
					Type:        "string",
					Description: "The release channel: INTERNAL_BETA (states READY_FOR_TESTING, SUPERSEDED), EXTERNAL_BETA (adds review states like WAITING_FOR_REVIEW, IN_REVIEW, REJECTED), or APP_STORE (adds PREPARE_FOR_SUBMISSION, ACCEPTED, READY_FOR_DISTRIBUTION)",
					Enum:        []string{"INTERNAL_BETA", "EXTERNAL_BETA", "APP_STORE"},
				},
				"release_id": {
					Type:        "string",
					Description: "The release record ID for the chosen channel",
				},
			},
			Required: []string{"release_type", "release_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Background Asset Version Release",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetBackgroundAssetVersionRelease)
}

func (r *Registry) handleListBackgroundAssets(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID   string              `json:"app_id"`
		Limit   int                 `json:"limit"`
		Cursor  string              `json:"cursor"`
		Filter  map[string][]string `json:"filter"`
		Sort    []string            `json:"sort"`
		Include []string            `json:"include"`
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.BackgroundAssetsResponse, error) {
		return r.client.ListBackgroundAssets(ctx, params.AppID, listOpts(limit, params.Filter, params.Sort, nil, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list background assets: %v", err)), nil
	}

	return newListResult(formatBackgroundAssets(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetBackgroundAsset(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BackgroundAssetID string   `json:"background_asset_id"`
		Include           []string `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BackgroundAssetID == "" {
		return mcp.NewErrorResult("background_asset_id is required"), nil
	}

	resp, err := r.client.GetBackgroundAsset(ctx, params.BackgroundAssetID, listOpts(0, nil, nil, nil, params.Include))
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get background asset: %v", err)), nil
	}

	return newDataResult(formatBackgroundAsset(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateBackgroundAsset(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID               string `json:"app_id"`
		AssetPackIdentifier string `json:"asset_pack_identifier"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}
	if params.AssetPackIdentifier == "" {
		return mcp.NewErrorResult("asset_pack_identifier is required"), nil
	}

	req := &api.BackgroundAssetCreateRequest{
		Data: api.BackgroundAssetCreateData{
			Type: "backgroundAssets",
			Attributes: api.BackgroundAssetCreateAttributes{
				AssetPackIdentifier: params.AssetPackIdentifier,
			},
			Relationships: api.BackgroundAssetCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "apps",
						ID:   params.AppID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateBackgroundAsset(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create background asset: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created background asset:\n%s\nNext, call create_background_asset_version with background_asset_id=%s to open a version for upload.",
		formatBackgroundAsset(resp.Data), resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleUpdateBackgroundAsset(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BackgroundAssetID string `json:"background_asset_id"`
		Archived          *bool  `json:"archived"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BackgroundAssetID == "" {
		return mcp.NewErrorResult("background_asset_id is required"), nil
	}
	if params.Archived == nil {
		return mcp.NewErrorResult("archived is required"), nil
	}

	req := &api.BackgroundAssetUpdateRequest{
		Data: api.BackgroundAssetUpdateData{
			Type: "backgroundAssets",
			ID:   params.BackgroundAssetID,
			Attributes: &api.BackgroundAssetUpdateAttributes{
				Archived: params.Archived,
			},
		},
	}

	resp, err := r.client.UpdateBackgroundAsset(ctx, params.BackgroundAssetID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update background asset: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Background asset updated:\n%s", formatBackgroundAsset(resp.Data)), resp.Data), nil
}

func (r *Registry) handleListBackgroundAssetVersions(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BackgroundAssetID string              `json:"background_asset_id"`
		Limit             int                 `json:"limit"`
		Cursor            string              `json:"cursor"`
		Filter            map[string][]string `json:"filter"`
		Sort              []string            `json:"sort"`
		Include           []string            `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BackgroundAssetID == "" {
		return mcp.NewErrorResult("background_asset_id is required"), nil
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.BackgroundAssetVersionsResponse, error) {
		return r.client.ListBackgroundAssetVersions(ctx, params.BackgroundAssetID, listOpts(limit, params.Filter, params.Sort, nil, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list background asset versions: %v", err)), nil
	}

	return newListResult(formatBackgroundAssetVersions(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetBackgroundAssetVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string   `json:"version_id"`
		Include   []string `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	resp, err := r.client.GetBackgroundAssetVersion(ctx, params.VersionID, listOpts(0, nil, nil, nil, params.Include))
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get background asset version: %v", err)), nil
	}

	return newDataResult(formatBackgroundAssetVersion(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateBackgroundAssetVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BackgroundAssetID string `json:"background_asset_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BackgroundAssetID == "" {
		return mcp.NewErrorResult("background_asset_id is required"), nil
	}

	req := &api.BackgroundAssetVersionCreateRequest{
		Data: api.BackgroundAssetVersionCreateData{
			Type: "backgroundAssetVersions",
			Relationships: api.BackgroundAssetVersionCreateRelationships{
				BackgroundAsset: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "backgroundAssets",
						ID:   params.BackgroundAssetID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateBackgroundAssetVersion(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create background asset version: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created background asset version:\n%s\nNext, call upload_background_asset_file with version_id=%s to deliver the asset pack (ASSET) and its manifest (MANIFEST).",
		formatBackgroundAssetVersion(resp.Data), resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleUploadBackgroundAssetFile(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID      string `json:"version_id"`
		FileName       string `json:"file_name"`
		AssetType      string `json:"asset_type"`
		FilePath       string `json:"file_path"`
		FileDataBase64 string `json:"file_data_base64"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("invalid arguments: %v", err)), nil
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}
	if params.FileName == "" {
		return mcp.NewErrorResult("file_name is required"), nil
	}
	assetType := params.AssetType
	if assetType == "" {
		assetType = "ASSET"
	}
	body, err := loadUploadBody(params.FilePath, params.FileDataBase64)
	if err != nil {
		return mcp.NewErrorResult(err.Error()), nil
	}

	resp, err := r.client.UploadBackgroundAssetFile(ctx, params.VersionID, params.FileName, assetType, body)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to upload background asset file: %v", err)), nil
	}

	text := fmt.Sprintf("Uploaded background asset file %s (%s, %s, %d bytes) to version %s. Poll get_background_asset_version for processing status.",
		resp.Data.ID, params.FileName, assetType, len(body), params.VersionID)
	return newDataResult(text, resp.Data), nil
}

func (r *Registry) handleListBackgroundAssetUploadFiles(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
		Limit     int    `json:"limit"`
		Cursor    string `json:"cursor"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.BackgroundAssetUploadFilesResponse, error) {
		return r.client.ListBackgroundAssetUploadFiles(ctx, params.VersionID, listOpts(limit, nil, nil, nil, nil))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list background asset upload files: %v", err)), nil
	}

	return newListResult(formatBackgroundAssetUploadFiles(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetBackgroundAssetVersionRelease(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ReleaseType string `json:"release_type"`
		ReleaseID   string `json:"release_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ReleaseType == "" {
		return mcp.NewErrorResult("release_type is required"), nil
	}
	if params.ReleaseID == "" {
		return mcp.NewErrorResult("release_id is required"), nil
	}

	var (
		resp *api.BackgroundAssetVersionReleaseResponse
		err  error
	)
	switch params.ReleaseType {
	case "INTERNAL_BETA":
		resp, err = r.client.GetBackgroundAssetVersionInternalBetaRelease(ctx, params.ReleaseID)
	case "EXTERNAL_BETA":
		resp, err = r.client.GetBackgroundAssetVersionExternalBetaRelease(ctx, params.ReleaseID)
	case "APP_STORE":
		resp, err = r.client.GetBackgroundAssetVersionAppStoreRelease(ctx, params.ReleaseID)
	default:
		return mcp.NewErrorResult("release_type must be INTERNAL_BETA, EXTERNAL_BETA, or APP_STORE"), nil
	}
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get background asset version release: %v", err)), nil
	}

	return newDataResult(formatBackgroundAssetVersionRelease(resp.Data), resp.Data), nil
}

func formatBackgroundAssets(assets []api.BackgroundAsset) string {
	if len(assets) == 0 {
		return "No background assets found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d background assets:\n\n", len(assets)))

	for _, asset := range assets {
		sb.WriteString(formatBackgroundAsset(asset))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatBackgroundAsset(asset api.BackgroundAsset) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Background Asset ID: %s\n", asset.ID))
	if asset.Attributes.AssetPackIdentifier != "" {
		sb.WriteString(fmt.Sprintf("Asset Pack Identifier: %s\n", asset.Attributes.AssetPackIdentifier))
	}
	sb.WriteString(fmt.Sprintf("Archived: %t\n", asset.Attributes.Archived))
	if asset.Attributes.UsedBytes > 0 {
		sb.WriteString(fmt.Sprintf("Used Bytes: %d\n", asset.Attributes.UsedBytes))
	}
	if asset.Attributes.CreatedDate != nil {
		sb.WriteString(fmt.Sprintf("Created: %s\n", asset.Attributes.CreatedDate.Format("2006-01-02 15:04:05")))
	}
	if rel := asset.Relationships; rel != nil {
		if rel.AppStoreVersion != nil && rel.AppStoreVersion.Data.ID != "" {
			sb.WriteString(fmt.Sprintf("App Store Version ID: %s\n", rel.AppStoreVersion.Data.ID))
		}
		if rel.InternalBetaVersion != nil && rel.InternalBetaVersion.Data.ID != "" {
			sb.WriteString(fmt.Sprintf("Internal Beta Version ID: %s\n", rel.InternalBetaVersion.Data.ID))
		}
		if rel.ExternalBetaVersion != nil && rel.ExternalBetaVersion.Data.ID != "" {
			sb.WriteString(fmt.Sprintf("External Beta Version ID: %s\n", rel.ExternalBetaVersion.Data.ID))
		}
	}
	return sb.String()
}

func formatBackgroundAssetVersions(versions []api.BackgroundAssetVersion) string {
	if len(versions) == 0 {
		return "No background asset versions found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d background asset versions:\n\n", len(versions)))

	for _, version := range versions {
		sb.WriteString(formatBackgroundAssetVersion(version))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatBackgroundAssetVersion(version api.BackgroundAssetVersion) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Background Asset Version ID: %s\n", version.ID))
	if version.Attributes.Version != "" {
		sb.WriteString(fmt.Sprintf("Version: %s\n", version.Attributes.Version))
	}
	if version.Attributes.Locale != "" {
		sb.WriteString(fmt.Sprintf("Locale: %s\n", version.Attributes.Locale))
	}
	if len(version.Attributes.Platforms) > 0 {
		sb.WriteString(fmt.Sprintf("Platforms: %s\n", strings.Join(version.Attributes.Platforms, ", ")))
	}
	if version.Attributes.State != "" {
		sb.WriteString(fmt.Sprintf("State: %s\n", version.Attributes.State))
	}
	if details := version.Attributes.StateDetails; details != nil {
		for _, detail := range details.Errors {
			sb.WriteString(fmt.Sprintf("Error [%s]: %s\n", detail.Code, detail.Description))
		}
		for _, detail := range details.Warnings {
			sb.WriteString(fmt.Sprintf("Warning [%s]: %s\n", detail.Code, detail.Description))
		}
		for _, detail := range details.Infos {
			sb.WriteString(fmt.Sprintf("Info [%s]: %s\n", detail.Code, detail.Description))
		}
	}
	if version.Attributes.CreatedDate != nil {
		sb.WriteString(fmt.Sprintf("Created: %s\n", version.Attributes.CreatedDate.Format("2006-01-02 15:04:05")))
	}
	if rel := version.Relationships; rel != nil {
		if rel.InternalBetaRelease != nil && rel.InternalBetaRelease.Data.ID != "" {
			sb.WriteString(fmt.Sprintf("Internal Beta Release ID: %s\n", rel.InternalBetaRelease.Data.ID))
		}
		if rel.ExternalBetaRelease != nil && rel.ExternalBetaRelease.Data.ID != "" {
			sb.WriteString(fmt.Sprintf("External Beta Release ID: %s\n", rel.ExternalBetaRelease.Data.ID))
		}
		if rel.AppStoreRelease != nil && rel.AppStoreRelease.Data.ID != "" {
			sb.WriteString(fmt.Sprintf("App Store Release ID: %s\n", rel.AppStoreRelease.Data.ID))
		}
		if rel.AssetFile != nil && rel.AssetFile.Data.ID != "" {
			sb.WriteString(fmt.Sprintf("Asset File ID: %s\n", rel.AssetFile.Data.ID))
		}
		if rel.ManifestFile != nil && rel.ManifestFile.Data.ID != "" {
			sb.WriteString(fmt.Sprintf("Manifest File ID: %s\n", rel.ManifestFile.Data.ID))
		}
	}
	return sb.String()
}

func formatBackgroundAssetUploadFiles(files []api.BackgroundAssetUploadFile) string {
	if len(files) == 0 {
		return "No background asset upload files found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d background asset upload files:\n\n", len(files)))

	for _, file := range files {
		sb.WriteString(fmt.Sprintf("File ID: %s\n", file.ID))
		if file.Attributes.FileName != "" {
			sb.WriteString(fmt.Sprintf("Name: %s (%d bytes)\n", file.Attributes.FileName, file.Attributes.FileSize))
		}
		if file.Attributes.AssetType != "" {
			sb.WriteString(fmt.Sprintf("Asset Type: %s\n", file.Attributes.AssetType))
		}
		if file.Attributes.AssetDeliveryState != nil && file.Attributes.AssetDeliveryState.State != "" {
			sb.WriteString(fmt.Sprintf("Delivery State: %s\n", file.Attributes.AssetDeliveryState.State))
		}
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatBackgroundAssetVersionRelease(release api.BackgroundAssetVersionRelease) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Release ID: %s\n", release.ID))
	if release.Type != "" {
		sb.WriteString(fmt.Sprintf("Type: %s\n", release.Type))
	}
	if release.Attributes.State != "" {
		sb.WriteString(fmt.Sprintf("State: %s\n", release.Attributes.State))
	}
	if rel := release.Relationships; rel != nil && rel.BackgroundAssetVersion != nil && rel.BackgroundAssetVersion.Data.ID != "" {
		sb.WriteString(fmt.Sprintf("Background Asset Version ID: %s\n", rel.BackgroundAssetVersion.Data.ID))
	}
	return sb.String()
}
