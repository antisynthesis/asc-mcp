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
				},
			},
			Required: []string{"app_id", "agreement_text"},
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
			},
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
			},
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
	}, r.handleDeleteAlternativeDistributionKey)

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
	}, r.handleDeleteMarketplaceSearchDetail)
}

// EULA handlers
func (r *Registry) handleGetEndUserLicenseAgreement(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}

	resp, err := r.client.GetEndUserLicenseAgreement(context.Background(), params.AppID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get EULA: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatEndUserLicenseAgreement(resp.Data)), nil
}

func (r *Registry) handleCreateEndUserLicenseAgreement(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID         string   `json:"app_id"`
		AgreementText string   `json:"agreement_text"`
		TerritoryIDs  []string `json:"territory_ids"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" || params.AgreementText == "" {
		return nil, fmt.Errorf("app_id and agreement_text are required")
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

	resp, err := r.client.CreateEndUserLicenseAgreement(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create EULA: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("EULA created:\n%s", formatEndUserLicenseAgreement(resp.Data))), nil
}

func (r *Registry) handleUpdateEndUserLicenseAgreement(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		EULAID        string `json:"eula_id"`
		AgreementText string `json:"agreement_text"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.EULAID == "" || params.AgreementText == "" {
		return nil, fmt.Errorf("eula_id and agreement_text are required")
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

	resp, err := r.client.UpdateEndUserLicenseAgreement(context.Background(), params.EULAID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update EULA: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("EULA updated:\n%s", formatEndUserLicenseAgreement(resp.Data))), nil
}

func (r *Registry) handleDeleteEndUserLicenseAgreement(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		EULAID string `json:"eula_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.EULAID == "" {
		return nil, fmt.Errorf("eula_id is required")
	}

	err := r.client.DeleteEndUserLicenseAgreement(context.Background(), params.EULAID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete EULA: %v", err)), nil
	}

	return mcp.NewSuccessResult("EULA deleted (reverted to standard Apple EULA)"), nil
}

// Category handlers
func (r *Registry) handleListAppCategories(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Limit int `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 100
	}

	resp, err := r.client.ListAppCategories(context.Background(), limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list app categories: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppCategories(resp.Data)), nil
}

func (r *Registry) handleGetAppCategory(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		CategoryID string `json:"category_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.CategoryID == "" {
		return nil, fmt.Errorf("category_id is required")
	}

	resp, err := r.client.GetAppCategory(context.Background(), params.CategoryID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get app category: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppCategory(resp.Data)), nil
}

// Alternative distribution handlers
func (r *Registry) handleListAlternativeDistributionKeys(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Limit int `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListAlternativeDistributionKeys(context.Background(), limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list alternative distribution keys: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAlternativeDistributionKeys(resp.Data)), nil
}

func (r *Registry) handleGetAlternativeDistributionKey(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		KeyID string `json:"key_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.KeyID == "" {
		return nil, fmt.Errorf("key_id is required")
	}

	resp, err := r.client.GetAlternativeDistributionKey(context.Background(), params.KeyID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get alternative distribution key: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAlternativeDistributionKey(resp.Data)), nil
}

func (r *Registry) handleCreateAlternativeDistributionKey(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
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

	resp, err := r.client.CreateAlternativeDistributionKey(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create alternative distribution key: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Alternative distribution key created:\n%s", formatAlternativeDistributionKey(resp.Data))), nil
}

func (r *Registry) handleDeleteAlternativeDistributionKey(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		KeyID string `json:"key_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.KeyID == "" {
		return nil, fmt.Errorf("key_id is required")
	}

	err := r.client.DeleteAlternativeDistributionKey(context.Background(), params.KeyID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete alternative distribution key: %v", err)), nil
	}

	return mcp.NewSuccessResult("Alternative distribution key deleted"), nil
}

// Marketplace search detail handlers
func (r *Registry) handleGetMarketplaceSearchDetail(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}

	resp, err := r.client.GetMarketplaceSearchDetail(context.Background(), params.AppID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get marketplace search detail: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatMarketplaceSearchDetail(resp.Data)), nil
}

func (r *Registry) handleCreateMarketplaceSearchDetail(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID      string `json:"app_id"`
		CatalogURL string `json:"catalog_url"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" || params.CatalogURL == "" {
		return nil, fmt.Errorf("app_id and catalog_url are required")
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

	resp, err := r.client.CreateMarketplaceSearchDetail(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create marketplace search detail: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Marketplace search detail created:\n%s", formatMarketplaceSearchDetail(resp.Data))), nil
}

func (r *Registry) handleUpdateMarketplaceSearchDetail(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DetailID   string `json:"detail_id"`
		CatalogURL string `json:"catalog_url"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DetailID == "" || params.CatalogURL == "" {
		return nil, fmt.Errorf("detail_id and catalog_url are required")
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

	resp, err := r.client.UpdateMarketplaceSearchDetail(context.Background(), params.DetailID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update marketplace search detail: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Marketplace search detail updated:\n%s", formatMarketplaceSearchDetail(resp.Data))), nil
}

func (r *Registry) handleDeleteMarketplaceSearchDetail(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DetailID string `json:"detail_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DetailID == "" {
		return nil, fmt.Errorf("detail_id is required")
	}

	err := r.client.DeleteMarketplaceSearchDetail(context.Background(), params.DetailID)
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
