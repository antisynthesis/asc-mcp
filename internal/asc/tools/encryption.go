package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerEncryptionTools registers app encryption declaration tools.
func (r *Registry) registerEncryptionTools() {
	// List encryption declarations
	r.register(mcp.Tool{
		Name:        "list_encryption_declarations",
		Description: "List app encryption declarations",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "Filter by App ID (optional)",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of declarations to return (default 50)",
				},
			},
		},
	}, r.handleListEncryptionDeclarations)

	// Get encryption declaration
	r.register(mcp.Tool{
		Name:        "get_encryption_declaration",
		Description: "Get details of a specific encryption declaration",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"declaration_id": {
					Type:        "string",
					Description: "The encryption declaration ID",
				},
			},
			Required: []string{"declaration_id"},
		},
	}, r.handleGetEncryptionDeclaration)

	// Create encryption declaration
	r.register(mcp.Tool{
		Name:        "create_encryption_declaration",
		Description: "Create an encryption declaration for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID",
				},
				"uses_encryption": {
					Type:        "boolean",
					Description: "Whether the app uses encryption",
				},
				"exempt": {
					Type:        "boolean",
					Description: "Whether the app is exempt from export regulations",
				},
				"contains_proprietary_cryptography": {
					Type:        "boolean",
					Description: "Whether the app contains proprietary cryptography",
				},
				"contains_third_party_cryptography": {
					Type:        "boolean",
					Description: "Whether the app contains third-party cryptography",
				},
				"available_on_french_store": {
					Type:        "boolean",
					Description: "Whether the app is available on the French store",
				},
				"app_description": {
					Type:        "string",
					Description: "Description of how the app uses encryption",
				},
				"code_value": {
					Type:        "string",
					Description: "CCATS code value if applicable",
				},
			},
			Required: []string{"app_id", "uses_encryption"},
		},
	}, r.handleCreateEncryptionDeclaration)

	// Assign build to encryption declaration
	r.register(mcp.Tool{
		Name:        "assign_build_to_encryption_declaration",
		Description: "Assign a build to an encryption declaration",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"declaration_id": {
					Type:        "string",
					Description: "The encryption declaration ID",
				},
				"build_id": {
					Type:        "string",
					Description: "The build ID to assign",
				},
			},
			Required: []string{"declaration_id", "build_id"},
		},
	}, r.handleAssignBuildToEncryptionDeclaration)
}

func (r *Registry) handleListEncryptionDeclarations(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
		Limit int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListAppEncryptionDeclarations(context.Background(), params.AppID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list encryption declarations: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatEncryptionDeclarations(resp.Data)), nil
}

func (r *Registry) handleGetEncryptionDeclaration(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DeclarationID string `json:"declaration_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DeclarationID == "" {
		return nil, fmt.Errorf("declaration_id is required")
	}

	resp, err := r.client.GetAppEncryptionDeclaration(context.Background(), params.DeclarationID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get encryption declaration: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatEncryptionDeclaration(resp.Data)), nil
}

func (r *Registry) handleCreateEncryptionDeclaration(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID                           string `json:"app_id"`
		UsesEncryption                  bool   `json:"uses_encryption"`
		Exempt                          bool   `json:"exempt"`
		ContainsProprietaryCryptography bool   `json:"contains_proprietary_cryptography"`
		ContainsThirdPartyCryptography  bool   `json:"contains_third_party_cryptography"`
		AvailableOnFrenchStore          bool   `json:"available_on_french_store"`
		AppDescription                  string `json:"app_description"`
		CodeValue                       string `json:"code_value"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}

	req := &api.AppEncryptionDeclarationCreateRequest{
		Data: api.AppEncryptionDeclarationCreateData{
			Type: "appEncryptionDeclarations",
			Attributes: api.AppEncryptionDeclarationCreateAttributes{
				UsesEncryption:                  params.UsesEncryption,
				Exempt:                          params.Exempt,
				ContainsProprietaryCryptography: params.ContainsProprietaryCryptography,
				ContainsThirdPartyCryptography:  params.ContainsThirdPartyCryptography,
				AvailableOnFrenchStore:          params.AvailableOnFrenchStore,
				AppDescription:                  params.AppDescription,
				CodeValue:                       params.CodeValue,
			},
			Relationships: api.AppEncryptionDeclarationCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "apps",
						ID:   params.AppID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateAppEncryptionDeclaration(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create encryption declaration: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Created encryption declaration: %s", resp.Data.ID)), nil
}

func (r *Registry) handleAssignBuildToEncryptionDeclaration(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DeclarationID string `json:"declaration_id"`
		BuildID       string `json:"build_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DeclarationID == "" {
		return nil, fmt.Errorf("declaration_id is required")
	}
	if params.BuildID == "" {
		return nil, fmt.Errorf("build_id is required")
	}

	err := r.client.AssignBuildToEncryptionDeclaration(context.Background(), params.DeclarationID, params.BuildID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to assign build to encryption declaration: %v", err)), nil
	}

	return mcp.NewSuccessResult("Build assigned to encryption declaration successfully"), nil
}

func formatEncryptionDeclarations(declarations []api.AppEncryptionDeclaration) string {
	if len(declarations) == 0 {
		return "No encryption declarations found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d encryption declarations:\n\n", len(declarations)))

	for _, decl := range declarations {
		sb.WriteString(formatEncryptionDeclaration(decl))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatEncryptionDeclaration(decl api.AppEncryptionDeclaration) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", decl.ID))
	sb.WriteString(fmt.Sprintf("Uses Encryption: %t\n", decl.Attributes.UsesEncryption))
	sb.WriteString(fmt.Sprintf("Exempt: %t\n", decl.Attributes.Exempt))
	sb.WriteString(fmt.Sprintf("Contains Proprietary Cryptography: %t\n", decl.Attributes.ContainsProprietaryCryptography))
	sb.WriteString(fmt.Sprintf("Contains Third-Party Cryptography: %t\n", decl.Attributes.ContainsThirdPartyCryptography))
	sb.WriteString(fmt.Sprintf("Available on French Store: %t\n", decl.Attributes.AvailableOnFrenchStore))
	sb.WriteString(fmt.Sprintf("Platform: %s\n", decl.Attributes.Platform))
	sb.WriteString(fmt.Sprintf("State: %s\n", decl.Attributes.AppEncryptionDeclarationState))
	if decl.Attributes.AppDescription != "" {
		sb.WriteString(fmt.Sprintf("Description: %s\n", decl.Attributes.AppDescription))
	}
	if decl.Attributes.CodeValue != "" {
		sb.WriteString(fmt.Sprintf("CCATS Code: %s\n", decl.Attributes.CodeValue))
	}
	if decl.Attributes.DocumentURL != "" {
		sb.WriteString(fmt.Sprintf("Document URL: %s\n", decl.Attributes.DocumentURL))
	}
	return sb.String()
}
