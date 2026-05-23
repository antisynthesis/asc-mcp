package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerSandboxTools registers sandbox tester tools.
func (r *Registry) registerSandboxTools() {
	// List sandbox testers
	r.register(mcp.Tool{
		Name:        "list_sandbox_testers",
		Description: "List sandbox testers for the account",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"limit": {
					Type:        "integer",
					Description: "Maximum number of testers to return (default 50)",
				},
				"cursor": {
					Type:        "string",
					Description: "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page.",
				},
			},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Sandbox Testers",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListSandboxTesters)

	// Create sandbox tester
	r.register(mcp.Tool{
		Name:        "create_sandbox_tester",
		Description: "Create a new sandbox tester account",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"email": {
					Type:        "string",
					Description: "Email address for the sandbox tester",
				},
				"password": {
					Type:        "string",
					Description: "Password for the sandbox account",
				},
				"first_name": {
					Type:        "string",
					Description: "First name of the tester",
				},
				"last_name": {
					Type:        "string",
					Description: "Last name of the tester",
				},
				"secret_question": {
					Type:        "string",
					Description: "Security question",
				},
				"secret_answer": {
					Type:        "string",
					Description: "Answer to security question",
				},
				"birth_date": {
					Type:        "string",
					Description: "Birth date (YYYY-MM-DD)",
				},
				"app_store_territory": {
					Type:        "string",
					Description: "App Store territory code (e.g., USA, GBR)",
				},
			},
			Required: []string{"email", "password", "first_name", "last_name", "secret_question", "secret_answer", "birth_date", "app_store_territory"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Sandbox Tester",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateSandboxTester)

	// Update sandbox tester
	r.register(mcp.Tool{
		Name:        "update_sandbox_tester",
		Description: "Update a sandbox tester's territory or interruptable setting",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"tester_id": {
					Type:        "string",
					Description: "The sandbox tester ID",
				},
				"territory": {
					Type:        "string",
					Description: "New App Store territory code",
				},
				"interruptable": {
					Type:        "boolean",
					Description: "Whether purchases can be interrupted for testing",
				},
				"subscription_renewal_rate": {
					Type:        "string",
					Description: "Subscription renewal rate: MONTHLY_RENEWAL_EVERY_ONE_HOUR, etc.",
				},
			},
			Required: []string{"tester_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update Sandbox Tester",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateSandboxTester)

	// Delete sandbox tester
	r.register(mcp.Tool{
		Name:        "delete_sandbox_tester",
		Description: "Delete a sandbox tester account",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"tester_id": {
					Type:        "string",
					Description: "The sandbox tester ID to delete",
				},
			},
			Required: []string{"tester_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Sandbox Tester",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteSandboxTester)
}

func (r *Registry) handleListSandboxTesters(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Limit  int    `json:"limit"`
		Cursor string `json:"cursor"`
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.SandboxTestersResponse, error) {
		return r.client.ListSandboxTesters(ctx, limit)
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list sandbox testers: %v", err)), nil
	}

	return newListResult(formatSandboxTesters(resp.Data), resp.Links), nil
}

func (r *Registry) handleCreateSandboxTester(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Email             string `json:"email"`
		Password          string `json:"password"`
		FirstName         string `json:"first_name"`
		LastName          string `json:"last_name"`
		SecretQuestion    string `json:"secret_question"`
		SecretAnswer      string `json:"secret_answer"`
		BirthDate         string `json:"birth_date"`
		AppStoreTerritory string `json:"app_store_territory"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.Email == "" || params.Password == "" || params.FirstName == "" || params.LastName == "" {
		return nil, fmt.Errorf("email, password, first_name, and last_name are required")
	}

	req := &api.SandboxTesterCreateRequest{
		Data: api.SandboxTesterCreateData{
			Type: "sandboxTesters",
			Attributes: api.SandboxTesterCreateAttributes{
				Email:             params.Email,
				Password:          params.Password,
				FirstName:         params.FirstName,
				LastName:          params.LastName,
				SecretQuestion:    params.SecretQuestion,
				SecretAnswer:      params.SecretAnswer,
				BirthDate:         params.BirthDate,
				AppStoreTerritory: params.AppStoreTerritory,
			},
		},
	}

	resp, err := r.client.CreateSandboxTester(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create sandbox tester: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Sandbox tester created:\n%s", formatSandboxTester(resp.Data))), nil
}

func (r *Registry) handleUpdateSandboxTester(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		TesterID                string `json:"tester_id"`
		Territory               string `json:"territory"`
		Interruptable           *bool  `json:"interruptable"`
		SubscriptionRenewalRate string `json:"subscription_renewal_rate"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.TesterID == "" {
		return nil, fmt.Errorf("tester_id is required")
	}

	req := &api.SandboxTesterUpdateRequest{
		Data: api.SandboxTesterUpdateData{
			Type: "sandboxTesters",
			ID:   params.TesterID,
			Attributes: api.SandboxTesterUpdateAttributes{
				Territory:               params.Territory,
				Interruptable:           params.Interruptable,
				SubscriptionRenewalRate: params.SubscriptionRenewalRate,
			},
		},
	}

	resp, err := r.client.UpdateSandboxTester(ctx, params.TesterID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update sandbox tester: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Sandbox tester updated:\n%s", formatSandboxTester(resp.Data))), nil
}

func (r *Registry) handleDeleteSandboxTester(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		TesterID string `json:"tester_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.TesterID == "" {
		return nil, fmt.Errorf("tester_id is required")
	}

	err := r.client.DeleteSandboxTester(ctx, params.TesterID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete sandbox tester: %v", err)), nil
	}

	return mcp.NewSuccessResult("Sandbox tester deleted"), nil
}

func formatSandboxTesters(testers []api.SandboxTester) string {
	if len(testers) == 0 {
		return "No sandbox testers found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d sandbox testers:\n\n", len(testers)))

	for _, tester := range testers {
		sb.WriteString(formatSandboxTester(tester))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatSandboxTester(tester api.SandboxTester) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", tester.ID))
	sb.WriteString(fmt.Sprintf("Email: %s\n", tester.Attributes.Email))
	sb.WriteString(fmt.Sprintf("Name: %s %s\n", tester.Attributes.FirstName, tester.Attributes.LastName))
	if tester.Attributes.AppStoreTerritory != "" {
		sb.WriteString(fmt.Sprintf("Territory: %s\n", tester.Attributes.AppStoreTerritory))
	}
	sb.WriteString(fmt.Sprintf("Interruptable: %t\n", tester.Attributes.Interruptable))
	return sb.String()
}
