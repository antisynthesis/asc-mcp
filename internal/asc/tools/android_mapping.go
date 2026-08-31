package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerAndroidMappingTools registers Android-to-iOS app mapping tools.
func (r *Registry) registerAndroidMappingTools() {
	// List Android-to-iOS app mappings
	r.register(mcp.Tool{
		Name:        "list_android_to_ios_app_mappings",
		Description: "List the Android-to-iOS app mappings for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to list mappings for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of mappings to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
				"fields": {
					Type:        "object",
					Description: "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return.",
				},
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Android-to-iOS App Mappings",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAndroidToIosAppMappings)

	// Get Android-to-iOS app mapping
	r.register(mcp.Tool{
		Name:        "get_android_to_ios_app_mapping",
		Description: "Get details of a specific Android-to-iOS app mapping",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"mapping_id": {
					Type:        "string",
					Description: "The Android-to-iOS app mapping detail ID",
				},
			},
			Required: []string{"mapping_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Android-to-iOS App Mapping",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetAndroidToIosAppMapping)

	// Create Android-to-iOS app mapping
	r.register(mcp.Tool{
		Name:        "create_android_to_ios_app_mapping",
		Description: "Map an Android package to an iOS app so users migrating platforms are directed to it",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to map the Android package to",
				},
				"package_name": {
					Type:        "string",
					Description: "The Android package name, e.g. com.example.app",
				},
				"sha256_fingerprints": {
					Type:        "array",
					Description: "SHA-256 fingerprints of the Android app signing key's public certificates",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"app_id", "package_name", "sha256_fingerprints"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Android-to-iOS App Mapping",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateAndroidToIosAppMapping)

	// Update Android-to-iOS app mapping
	r.register(mcp.Tool{
		Name:        "update_android_to_ios_app_mapping",
		Description: "Update an Android-to-iOS app mapping's package name or signing key fingerprints",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"mapping_id": {
					Type:        "string",
					Description: "The Android-to-iOS app mapping detail ID",
				},
				"package_name": {
					Type:        "string",
					Description: "New Android package name",
				},
				"sha256_fingerprints": {
					Type:        "array",
					Description: "New SHA-256 fingerprints of the Android app signing key's public certificates (replaces the existing list)",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"mapping_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update Android-to-iOS App Mapping",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateAndroidToIosAppMapping)

	// Delete Android-to-iOS app mapping
	r.register(mcp.Tool{
		Name:        "delete_android_to_ios_app_mapping",
		Description: "Delete an Android-to-iOS app mapping",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"mapping_id": {
					Type:        "string",
					Description: "The Android-to-iOS app mapping detail ID",
				},
			},
			Required: []string{"mapping_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Android-to-iOS App Mapping",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteAndroidToIosAppMapping)
}

func (r *Registry) handleListAndroidToIosAppMappings(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID  string              `json:"app_id"`
		Limit  int                 `json:"limit"`
		Cursor string              `json:"cursor"`
		Fields map[string][]string `json:"fields"`
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AndroidToIosAppMappingDetailsResponse, error) {
		return r.client.ListAndroidToIosAppMappingDetails(ctx, params.AppID, listOpts(limit, nil, nil, params.Fields, nil))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list Android-to-iOS app mappings: %v", err)), nil
	}

	return newListResult(formatAndroidToIosAppMappings(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetAndroidToIosAppMapping(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		MappingID string `json:"mapping_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.MappingID == "" {
		return mcp.NewErrorResult("mapping_id is required"), nil
	}

	resp, err := r.client.GetAndroidToIosAppMappingDetail(ctx, params.MappingID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get Android-to-iOS app mapping: %v", err)), nil
	}

	return newDataResult(formatAndroidToIosAppMapping(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateAndroidToIosAppMapping(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID              string   `json:"app_id"`
		PackageName        string   `json:"package_name"`
		Sha256Fingerprints []string `json:"sha256_fingerprints"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}
	if params.PackageName == "" {
		return mcp.NewErrorResult("package_name is required"), nil
	}
	if len(params.Sha256Fingerprints) == 0 {
		return mcp.NewErrorResult("sha256_fingerprints is required"), nil
	}

	req := &api.AndroidToIosAppMappingDetailCreateRequest{
		Data: api.AndroidToIosAppMappingDetailCreateData{
			Type: "androidToIosAppMappingDetails",
			Attributes: api.AndroidToIosAppMappingDetailCreateAttributes{
				PackageName: params.PackageName,
				AppSigningKeyPublicCertificateSha256Fingerprints: params.Sha256Fingerprints,
			},
			Relationships: api.AndroidToIosAppMappingDetailCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "apps",
						ID:   params.AppID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateAndroidToIosAppMappingDetail(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create Android-to-iOS app mapping: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created Android-to-iOS app mapping:\n%s", formatAndroidToIosAppMapping(resp.Data)), resp.Data), nil
}

func (r *Registry) handleUpdateAndroidToIosAppMapping(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		MappingID          string   `json:"mapping_id"`
		PackageName        string   `json:"package_name"`
		Sha256Fingerprints []string `json:"sha256_fingerprints"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.MappingID == "" {
		return mcp.NewErrorResult("mapping_id is required"), nil
	}

	req := &api.AndroidToIosAppMappingDetailUpdateRequest{
		Data: api.AndroidToIosAppMappingDetailUpdateData{
			Type: "androidToIosAppMappingDetails",
			ID:   params.MappingID,
			Attributes: &api.AndroidToIosAppMappingDetailUpdateAttributes{
				PackageName: params.PackageName,
				AppSigningKeyPublicCertificateSha256Fingerprints: params.Sha256Fingerprints,
			},
		},
	}

	resp, err := r.client.UpdateAndroidToIosAppMappingDetail(ctx, params.MappingID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update Android-to-iOS app mapping: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Android-to-iOS app mapping updated:\n%s", formatAndroidToIosAppMapping(resp.Data)), resp.Data), nil
}

func (r *Registry) handleDeleteAndroidToIosAppMapping(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		MappingID string `json:"mapping_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.MappingID == "" {
		return mcp.NewErrorResult("mapping_id is required"), nil
	}

	if err := r.client.DeleteAndroidToIosAppMappingDetail(ctx, params.MappingID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete Android-to-iOS app mapping: %v", err)), nil
	}

	return mcp.NewSuccessResult("Android-to-iOS app mapping deleted successfully"), nil
}

func formatAndroidToIosAppMappings(mappings []api.AndroidToIosAppMappingDetail) string {
	if len(mappings) == 0 {
		return "No Android-to-iOS app mappings found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d Android-to-iOS app mappings:\n\n", len(mappings)))

	for _, mapping := range mappings {
		sb.WriteString(formatAndroidToIosAppMapping(mapping))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAndroidToIosAppMapping(mapping api.AndroidToIosAppMappingDetail) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Mapping ID: %s\n", mapping.ID))
	if mapping.Attributes.PackageName != "" {
		sb.WriteString(fmt.Sprintf("Package Name: %s\n", mapping.Attributes.PackageName))
	}
	if len(mapping.Attributes.AppSigningKeyPublicCertificateSha256Fingerprints) > 0 {
		sb.WriteString(fmt.Sprintf("SHA-256 Fingerprints: %s\n", strings.Join(mapping.Attributes.AppSigningKeyPublicCertificateSha256Fingerprints, ", ")))
	}
	return sb.String()
}
