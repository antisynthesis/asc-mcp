package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerMiscTools registers miscellaneous tools (EULA, categories, alternative distribution, etc).
func (r *Registry) registerMiscTools() {
	// End User License Agreement tools
	r.register(mcp.Tool{
		Name:        "get_end_user_license_agreement",
		Description: "Get the End User License Agreement for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get End User License Agreement",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetEndUserLicenseAgreement)

	r.register(mcp.Tool{
		Name:        "create_end_user_license_agreement",
		Description: "Create an End User License Agreement for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
				"agreement_text": {
					Type:        "string",
					Description: "The EULA text",
				},
				"territory_ids": {
					Type:        "array",
					Description: "List of territory IDs where this EULA applies",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"app_id", "agreement_text"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create End User License Agreement",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateEndUserLicenseAgreement)

	r.register(mcp.Tool{
		Name:        "update_end_user_license_agreement",
		Description: "Update an End User License Agreement",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"eula_id": {
					Type:        "string",
					Description: "The EULA ID",
				},
				"agreement_text": {
					Type:        "string",
					Description: "The updated EULA text",
				},
			},
			Required: []string{"eula_id", "agreement_text"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update End User License Agreement",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateEndUserLicenseAgreement)

	r.register(mcp.Tool{
		Name:        "delete_end_user_license_agreement",
		Description: "Delete an End User License Agreement (reverts to standard Apple EULA)",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"eula_id": {
					Type:        "string",
					Description: "The EULA ID to delete",
				},
			},
			Required: []string{"eula_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete End User License Agreement",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteEndUserLicenseAgreement)

	// App Categories tools
	r.register(mcp.Tool{
		Name:        "list_app_categories",
		Description: "List available App Store categories",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"limit": {
					Type:        "integer",
					Description: "Maximum number of categories to return (default 100)",
				},
				"cursor": {
					Type:        "string",
					Description: "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page.",
				},
				"filter": {
					Type:        "object",
					Description: "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"platform\": [\"IOS\"]} becomes filter[platform]=IOS.",
				},
				"sort": {
					Type:        "array",
					Description: "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
					Items:       &mcp.Property{Type: "string"},
				},
				"fields": {
					Type:        "object",
					Description: "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return.",
				},
				"include": {
					Type:        "array",
					Description: "Related resource names to include in the response.",
					Items:       &mcp.Property{Type: "string"},
				},
			},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List App Categories",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAppCategories)

	r.register(mcp.Tool{
		Name:        "get_app_category",
		Description: "Get details of a specific App Store category",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"category_id": {
					Type:        "string",
					Description: "The category ID",
				},
			},
			Required: []string{"category_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get App Category",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetAppCategory)

	// Alternative Distribution tools (EU DMA compliance)
	r.register(mcp.Tool{
		Name:        "list_alternative_distribution_keys",
		Description: "List alternative distribution keys for EU marketplace distribution",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"limit": {
					Type:        "integer",
					Description: "Maximum number of keys to return (default 50)",
				},
				"cursor": {
					Type:        "string",
					Description: "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page.",
				},
				"filter": {
					Type:        "object",
					Description: "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"platform\": [\"IOS\"]} becomes filter[platform]=IOS.",
				},
				"sort": {
					Type:        "array",
					Description: "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
					Items:       &mcp.Property{Type: "string"},
				},
				"fields": {
					Type:        "object",
					Description: "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return.",
				},
				"include": {
					Type:        "array",
					Description: "Related resource names to include in the response.",
					Items:       &mcp.Property{Type: "string"},
				},
			},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Alternative Distribution Keys",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAlternativeDistributionKeys)

	r.register(mcp.Tool{
		Name:        "get_alternative_distribution_key",
		Description: "Get a specific alternative distribution key",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"key_id": {
					Type:        "string",
					Description: "The alternative distribution key ID",
				},
			},
			Required: []string{"key_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Alternative Distribution Key",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetAlternativeDistributionKey)

	r.register(mcp.Tool{
		Name:        "create_alternative_distribution_key",
		Description: "Create a new alternative distribution key for EU marketplace",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Alternative Distribution Key",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateAlternativeDistributionKey)

	r.register(mcp.Tool{
		Name:        "delete_alternative_distribution_key",
		Description: "Delete an alternative distribution key",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"key_id": {
					Type:        "string",
					Description: "The alternative distribution key ID to delete",
				},
			},
			Required: []string{"key_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Alternative Distribution Key",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteAlternativeDistributionKey)

	// Get alternative distribution package for a version
	r.register(mcp.Tool{
		Name:        "get_alternative_distribution_package",
		Description: "Get the alternative distribution package for an App Store version (EU marketplace distribution)",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The App Store version ID",
				},
			},
			Required: []string{"version_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Alternative Distribution Package",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetAlternativeDistributionPackage)

	// Marketplace Search Detail tools
	r.register(mcp.Tool{
		Name:        "get_marketplace_search_detail",
		Description: "Get marketplace search details for an app (EU alternative marketplaces)",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Marketplace Search Detail",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetMarketplaceSearchDetail)

	r.register(mcp.Tool{
		Name:        "create_marketplace_search_detail",
		Description: "Create marketplace search detail for EU distribution",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
				"catalog_url": {
					Type:        "string",
					Description: "URL for the marketplace catalog",
				},
			},
			Required: []string{"app_id", "catalog_url"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Marketplace Search Detail",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateMarketplaceSearchDetail)

	r.register(mcp.Tool{
		Name:        "update_marketplace_search_detail",
		Description: "Update marketplace search detail",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"detail_id": {
					Type:        "string",
					Description: "The marketplace search detail ID",
				},
				"catalog_url": {
					Type:        "string",
					Description: "New URL for the marketplace catalog",
				},
			},
			Required: []string{"detail_id", "catalog_url"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update Marketplace Search Detail",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateMarketplaceSearchDetail)

	r.register(mcp.Tool{
		Name:        "delete_marketplace_search_detail",
		Description: "Delete marketplace search detail",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"detail_id": {
					Type:        "string",
					Description: "The marketplace search detail ID to delete",
				},
			},
			Required: []string{"detail_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Marketplace Search Detail",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteMarketplaceSearchDetail)
}

// EULA handlers
func (r *Registry) handleGetEndUserLicenseAgreement(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}

	resp, err := r.client.GetEndUserLicenseAgreement(ctx, params.AppID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get EULA: %v", err)), nil
	}

	return newDataResult(formatEndUserLicenseAgreement(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateEndUserLicenseAgreement(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID         string   `json:"app_id"`
		AgreementText string   `json:"agreement_text"`
		TerritoryIDs  []string `json:"territory_ids"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" || params.AgreementText == "" {
		return mcp.NewErrorResult("app_id and agreement_text are required"), nil
	}

	var territories []api.ResourceIdentifier
	for _, tid := range params.TerritoryIDs {
		territories = append(territories, api.ResourceIdentifier{Type: "territories", ID: tid})
	}

	req := &api.EndUserLicenseAgreementCreateRequest{
		Data: api.EndUserLicenseAgreementCreateData{
			Type: "endUserLicenseAgreements",
			Attributes: api.EndUserLicenseAgreementCreateAttributes{
				AgreementText: params.AgreementText,
			},
			Relationships: api.EndUserLicenseAgreementCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "apps", ID: params.AppID},
				},
				Territories: api.RelationshipDataList{
					Data: territories,
				},
			},
		},
	}

	resp, err := r.client.CreateEndUserLicenseAgreement(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create EULA: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("EULA created:\n%s", formatEndUserLicenseAgreement(resp.Data)), resp.Data), nil
}

func (r *Registry) handleUpdateEndUserLicenseAgreement(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		EULAID        string `json:"eula_id"`
		AgreementText string `json:"agreement_text"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.EULAID == "" || params.AgreementText == "" {
		return mcp.NewErrorResult("eula_id and agreement_text are required"), nil
	}

	req := &api.EndUserLicenseAgreementUpdateRequest{
		Data: api.EndUserLicenseAgreementUpdateData{
			Type: "endUserLicenseAgreements",
			ID:   params.EULAID,
			Attributes: api.EndUserLicenseAgreementUpdateAttributes{
				AgreementText: params.AgreementText,
			},
		},
	}

	resp, err := r.client.UpdateEndUserLicenseAgreement(ctx, params.EULAID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update EULA: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("EULA updated:\n%s", formatEndUserLicenseAgreement(resp.Data)), resp.Data), nil
}

func (r *Registry) handleDeleteEndUserLicenseAgreement(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		EULAID string `json:"eula_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.EULAID == "" {
		return mcp.NewErrorResult("eula_id is required"), nil
	}

	err := r.client.DeleteEndUserLicenseAgreement(ctx, params.EULAID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete EULA: %v", err)), nil
	}

	return mcp.NewSuccessResult("EULA deleted (reverted to standard Apple EULA)"), nil
}

// Category handlers
func (r *Registry) handleListAppCategories(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Limit   int                 `json:"limit"`
		Cursor  string              `json:"cursor"`
		Filter  map[string][]string `json:"filter"`
		Sort    []string            `json:"sort"`
		Fields  map[string][]string `json:"fields"`
		Include []string            `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 100
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AppCategoriesResponse, error) {
		return r.client.ListAppCategories(ctx, listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list app categories: %v", err)), nil
	}

	return newListResult(formatAppCategories(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetAppCategory(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		CategoryID string `json:"category_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.CategoryID == "" {
		return mcp.NewErrorResult("category_id is required"), nil
	}

	resp, err := r.client.GetAppCategory(ctx, params.CategoryID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app category: %v", err)), nil
	}

	return newDataResult(formatAppCategory(resp.Data), resp.Data), nil
}

// Alternative distribution handlers
func (r *Registry) handleListAlternativeDistributionKeys(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Limit   int                 `json:"limit"`
		Cursor  string              `json:"cursor"`
		Filter  map[string][]string `json:"filter"`
		Sort    []string            `json:"sort"`
		Fields  map[string][]string `json:"fields"`
		Include []string            `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AlternativeDistributionKeysResponse, error) {
		return r.client.ListAlternativeDistributionKeys(ctx, listOpts(limit, params.Filter, params.Sort, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list alternative distribution keys: %v", err)), nil
	}

	return newListResult(formatAlternativeDistributionKeys(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetAlternativeDistributionKey(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		KeyID string `json:"key_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.KeyID == "" {
		return mcp.NewErrorResult("key_id is required"), nil
	}

	resp, err := r.client.GetAlternativeDistributionKey(ctx, params.KeyID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get alternative distribution key: %v", err)), nil
	}

	return newDataResult(formatAlternativeDistributionKey(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateAlternativeDistributionKey(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}

	req := &api.AlternativeDistributionKeyCreateRequest{
		Data: api.AlternativeDistributionKeyCreateData{
			Type: "alternativeDistributionKeys",
			Relationships: api.AlternativeDistributionKeyCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "apps", ID: params.AppID},
				},
			},
		},
	}

	resp, err := r.client.CreateAlternativeDistributionKey(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create alternative distribution key: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Alternative distribution key created:\n%s", formatAlternativeDistributionKey(resp.Data)), resp.Data), nil
}

func (r *Registry) handleDeleteAlternativeDistributionKey(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		KeyID string `json:"key_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.KeyID == "" {
		return mcp.NewErrorResult("key_id is required"), nil
	}

	err := r.client.DeleteAlternativeDistributionKey(ctx, params.KeyID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete alternative distribution key: %v", err)), nil
	}

	return mcp.NewSuccessResult("Alternative distribution key deleted"), nil
}

func (r *Registry) handleGetAlternativeDistributionPackage(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	resp, err := r.client.GetAlternativeDistributionPackageForVersion(ctx, params.VersionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get alternative distribution package: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Alternative Distribution Package ID: %s\n", resp.Data.ID), resp.Data), nil
}

// Marketplace search detail handlers
func (r *Registry) handleGetMarketplaceSearchDetail(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}

	resp, err := r.client.GetMarketplaceSearchDetail(ctx, params.AppID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get marketplace search detail: %v", err)), nil
	}

	return newDataResult(formatMarketplaceSearchDetail(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateMarketplaceSearchDetail(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID      string `json:"app_id"`
		CatalogURL string `json:"catalog_url"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" || params.CatalogURL == "" {
		return mcp.NewErrorResult("app_id and catalog_url are required"), nil
	}

	req := &api.MarketplaceSearchDetailCreateRequest{
		Data: api.MarketplaceSearchDetailCreateData{
			Type: "marketplaceSearchDetails",
			Attributes: api.MarketplaceSearchDetailCreateAttributes{
				CatalogURL: params.CatalogURL,
			},
			Relationships: api.MarketplaceSearchDetailCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "apps", ID: params.AppID},
				},
			},
		},
	}

	resp, err := r.client.CreateMarketplaceSearchDetail(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create marketplace search detail: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Marketplace search detail created:\n%s", formatMarketplaceSearchDetail(resp.Data)), resp.Data), nil
}

func (r *Registry) handleUpdateMarketplaceSearchDetail(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DetailID   string `json:"detail_id"`
		CatalogURL string `json:"catalog_url"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DetailID == "" || params.CatalogURL == "" {
		return mcp.NewErrorResult("detail_id and catalog_url are required"), nil
	}

	req := &api.MarketplaceSearchDetailUpdateRequest{
		Data: api.MarketplaceSearchDetailUpdateData{
			Type: "marketplaceSearchDetails",
			ID:   params.DetailID,
			Attributes: api.MarketplaceSearchDetailUpdateAttributes{
				CatalogURL: params.CatalogURL,
			},
		},
	}

	resp, err := r.client.UpdateMarketplaceSearchDetail(ctx, params.DetailID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update marketplace search detail: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Marketplace search detail updated:\n%s", formatMarketplaceSearchDetail(resp.Data)), resp.Data), nil
}

func (r *Registry) handleDeleteMarketplaceSearchDetail(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DetailID string `json:"detail_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DetailID == "" {
		return mcp.NewErrorResult("detail_id is required"), nil
	}

	err := r.client.DeleteMarketplaceSearchDetail(ctx, params.DetailID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete marketplace search detail: %v", err)), nil
	}

	return mcp.NewSuccessResult("Marketplace search detail deleted"), nil
}

// Format helpers
func formatEndUserLicenseAgreement(eula api.EndUserLicenseAgreement) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", eula.ID))
	text := eula.Attributes.AgreementText
	if len(text) > 500 {
		text = text[:500] + "..."
	}
	sb.WriteString(fmt.Sprintf("Agreement Text:\n%s\n", text))
	return sb.String()
}

func formatAppCategories(categories []api.AppCategory) string {
	if len(categories) == 0 {
		return "No app categories found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d app categories:\n\n", len(categories)))

	for _, c := range categories {
		sb.WriteString(formatAppCategory(c))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAppCategory(c api.AppCategory) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", c.ID))
	if len(c.Attributes.Platforms) > 0 {
		sb.WriteString(fmt.Sprintf("Platforms: %s\n", strings.Join(c.Attributes.Platforms, ", ")))
	}
	return sb.String()
}

func formatAlternativeDistributionKeys(keys []api.AlternativeDistributionKey) string {
	if len(keys) == 0 {
		return "No alternative distribution keys found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d alternative distribution keys:\n\n", len(keys)))

	for _, k := range keys {
		sb.WriteString(formatAlternativeDistributionKey(k))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAlternativeDistributionKey(k api.AlternativeDistributionKey) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", k.ID))
	if k.Attributes.PublicKey != "" {
		// Show truncated public key
		pk := k.Attributes.PublicKey
		if len(pk) > 100 {
			pk = pk[:100] + "..."
		}
		sb.WriteString(fmt.Sprintf("Public Key: %s\n", pk))
	}
	return sb.String()
}

func formatMarketplaceSearchDetail(d api.MarketplaceSearchDetail) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", d.ID))
	sb.WriteString(fmt.Sprintf("Catalog URL: %s\n", d.Attributes.CatalogURL))
	return sb.String()
}
