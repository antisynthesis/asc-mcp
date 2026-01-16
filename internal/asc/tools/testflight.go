// Package tools provides MCP tool implementations for App Store Connect.
package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerTestFlightTools registers TestFlight management tools.
func (r *Registry) registerTestFlightTools() {
	r.register(
		mcp.Tool{
			Name:        "list_beta_groups",
			Description: "List TestFlight beta groups. Can filter by app ID. Returns group name, tester counts, and public link information.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"app_id": {
						Type:        "string",
						Description: "Optional: Filter beta groups by app ID",
					},
					"limit": {
						Type:        "integer",
						Description: "Maximum number of beta groups to return (default: 50)",
						Default:     50,
					},
				},
			},
		},
		r.handleListBetaGroups,
	)

	r.register(
		mcp.Tool{
			Name:        "create_beta_group",
			Description: "Create a new TestFlight beta group for an app.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"app_id": {
						Type:        "string",
						Description: "The App Store Connect ID of the app",
					},
					"name": {
						Type:        "string",
						Description: "Name for the beta group",
					},
					"public_link_enabled": {
						Type:        "boolean",
						Description: "Whether to enable public link for the group (default: false)",
					},
					"feedback_enabled": {
						Type:        "boolean",
						Description: "Whether to enable feedback for the group (default: true)",
					},
				},
				Required: []string{"app_id", "name"},
			},
		},
		r.handleCreateBetaGroup,
	)

	r.register(
		mcp.Tool{
			Name:        "delete_beta_group",
			Description: "Delete a TestFlight beta group.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"beta_group_id": {
						Type:        "string",
						Description: "The App Store Connect ID of the beta group to delete",
					},
				},
				Required: []string{"beta_group_id"},
			},
		},
		r.handleDeleteBetaGroup,
	)

	r.register(
		mcp.Tool{
			Name:        "list_beta_testers",
			Description: "List TestFlight beta testers. Can filter by beta group ID. Returns tester email, name, invite status, and state.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"beta_group_id": {
						Type:        "string",
						Description: "Optional: Filter testers by beta group ID",
					},
					"limit": {
						Type:        "integer",
						Description: "Maximum number of testers to return (default: 50)",
						Default:     50,
					},
				},
			},
		},
		r.handleListBetaTesters,
	)

	r.register(
		mcp.Tool{
			Name:        "invite_beta_tester",
			Description: "Invite a new beta tester to TestFlight, optionally adding them to specific beta groups.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"email": {
						Type:        "string",
						Description: "Email address of the tester to invite",
					},
					"first_name": {
						Type:        "string",
						Description: "First name of the tester (optional)",
					},
					"last_name": {
						Type:        "string",
						Description: "Last name of the tester (optional)",
					},
					"beta_group_ids": {
						Type:        "array",
						Description: "Optional: IDs of beta groups to add the tester to",
					},
				},
				Required: []string{"email"},
			},
		},
		r.handleInviteBetaTester,
	)

	r.register(
		mcp.Tool{
			Name:        "remove_beta_tester",
			Description: "Remove a beta tester from TestFlight.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"beta_tester_id": {
						Type:        "string",
						Description: "The App Store Connect ID of the beta tester to remove",
					},
				},
				Required: []string{"beta_tester_id"},
			},
		},
		r.handleRemoveBetaTester,
	)

	r.register(
		mcp.Tool{
			Name:        "add_tester_to_group",
			Description: "Add an existing beta tester to a beta group.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"beta_group_id": {
						Type:        "string",
						Description: "The App Store Connect ID of the beta group",
					},
					"beta_tester_id": {
						Type:        "string",
						Description: "The App Store Connect ID of the beta tester",
					},
				},
				Required: []string{"beta_group_id", "beta_tester_id"},
			},
		},
		r.handleAddTesterToGroup,
	)
}

// handleListBetaGroups handles the list_beta_groups tool.
func (r *Registry) handleListBetaGroups(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
		Limit int    `json:"limit"`
	}
	params.Limit = 50

	if args != nil {
		if err := json.Unmarshal(args, &params); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
	}

	ctx := context.Background()
	resp, err := r.client.ListBetaGroups(ctx, params.AppID, params.Limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list beta groups: %v", err)), nil
	}

	if len(resp.Data) == 0 {
		return mcp.NewSuccessResult("No beta groups found."), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d beta groups:\n\n", len(resp.Data)))

	for _, group := range resp.Data {
		sb.WriteString(fmt.Sprintf("**%s**\n", group.Attributes.Name))
		sb.WriteString(fmt.Sprintf("  - ID: %s\n", group.ID))
		sb.WriteString(fmt.Sprintf("  - Internal Group: %v\n", group.Attributes.IsInternalGroup))
		sb.WriteString(fmt.Sprintf("  - Has Access to All Builds: %v\n", group.Attributes.HasAccessToAllBuilds))
		sb.WriteString(fmt.Sprintf("  - Feedback Enabled: %v\n", group.Attributes.FeedbackEnabled))
		sb.WriteString(fmt.Sprintf("  - Public Link Enabled: %v\n", group.Attributes.PublicLinkEnabled))
		if group.Attributes.PublicLink != "" {
			sb.WriteString(fmt.Sprintf("  - Public Link: %s\n", group.Attributes.PublicLink))
		}
		if group.Attributes.CreatedDate != nil {
			sb.WriteString(fmt.Sprintf("  - Created: %s\n", group.Attributes.CreatedDate.Format("2006-01-02")))
		}
		sb.WriteString("\n")
	}

	return mcp.NewSuccessResult(sb.String()), nil
}

// handleCreateBetaGroup handles the create_beta_group tool.
func (r *Registry) handleCreateBetaGroup(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID             string `json:"app_id"`
		Name              string `json:"name"`
		PublicLinkEnabled bool   `json:"public_link_enabled"`
		FeedbackEnabled   bool   `json:"feedback_enabled"`
	}
	params.FeedbackEnabled = true

	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}
	if params.Name == "" {
		return mcp.NewErrorResult("name is required"), nil
	}

	req := &api.BetaGroupCreateRequest{
		Data: api.BetaGroupCreateData{
			Type: "betaGroups",
			Attributes: api.BetaGroupCreateAttributes{
				Name:              params.Name,
				PublicLinkEnabled: params.PublicLinkEnabled,
				FeedbackEnabled:   params.FeedbackEnabled,
			},
			Relationships: api.BetaGroupCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "apps",
						ID:   params.AppID,
					},
				},
			},
		},
	}

	ctx := context.Background()
	resp, err := r.client.CreateBetaGroup(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create beta group: %v", err)), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Successfully created beta group **%s**\n\n", resp.Data.Attributes.Name))
	sb.WriteString(fmt.Sprintf("- ID: %s\n", resp.Data.ID))
	sb.WriteString(fmt.Sprintf("- Public Link Enabled: %v\n", resp.Data.Attributes.PublicLinkEnabled))
	sb.WriteString(fmt.Sprintf("- Feedback Enabled: %v\n", resp.Data.Attributes.FeedbackEnabled))

	return mcp.NewSuccessResult(sb.String()), nil
}

// handleDeleteBetaGroup handles the delete_beta_group tool.
func (r *Registry) handleDeleteBetaGroup(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BetaGroupID string `json:"beta_group_id"`
	}

	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BetaGroupID == "" {
		return mcp.NewErrorResult("beta_group_id is required"), nil
	}

	ctx := context.Background()
	if err := r.client.DeleteBetaGroup(ctx, params.BetaGroupID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete beta group: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Successfully deleted beta group %s", params.BetaGroupID)), nil
}

// handleListBetaTesters handles the list_beta_testers tool.
func (r *Registry) handleListBetaTesters(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BetaGroupID string `json:"beta_group_id"`
		Limit       int    `json:"limit"`
	}
	params.Limit = 50

	if args != nil {
		if err := json.Unmarshal(args, &params); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
	}

	ctx := context.Background()
	resp, err := r.client.ListBetaTesters(ctx, params.BetaGroupID, params.Limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list beta testers: %v", err)), nil
	}

	if len(resp.Data) == 0 {
		return mcp.NewSuccessResult("No beta testers found."), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d beta testers:\n\n", len(resp.Data)))

	for _, tester := range resp.Data {
		name := tester.Attributes.Email
		if tester.Attributes.FirstName != "" || tester.Attributes.LastName != "" {
			name = fmt.Sprintf("%s %s (%s)", tester.Attributes.FirstName, tester.Attributes.LastName, tester.Attributes.Email)
		}
		sb.WriteString(fmt.Sprintf("**%s**\n", name))
		sb.WriteString(fmt.Sprintf("  - ID: %s\n", tester.ID))
		sb.WriteString(fmt.Sprintf("  - State: %s\n", tester.Attributes.State))
		sb.WriteString(fmt.Sprintf("  - Invite Type: %s\n", tester.Attributes.InviteType))
		sb.WriteString("\n")
	}

	return mcp.NewSuccessResult(sb.String()), nil
}

// handleInviteBetaTester handles the invite_beta_tester tool.
func (r *Registry) handleInviteBetaTester(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Email        string   `json:"email"`
		FirstName    string   `json:"first_name"`
		LastName     string   `json:"last_name"`
		BetaGroupIDs []string `json:"beta_group_ids"`
	}

	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.Email == "" {
		return mcp.NewErrorResult("email is required"), nil
	}

	req := &api.BetaTesterCreateRequest{
		Data: api.BetaTesterCreateData{
			Type: "betaTesters",
			Attributes: api.BetaTesterCreateAttributes{
				Email:     params.Email,
				FirstName: params.FirstName,
				LastName:  params.LastName,
			},
		},
	}

	if len(params.BetaGroupIDs) > 0 {
		groups := make([]api.ResourceIdentifier, 0, len(params.BetaGroupIDs))
		for _, id := range params.BetaGroupIDs {
			groups = append(groups, api.ResourceIdentifier{
				Type: "betaGroups",
				ID:   id,
			})
		}
		req.Data.Relationships = &api.BetaTesterCreateRelationships{
			BetaGroups: &api.RelationshipDataList{
				Data: groups,
			},
		}
	}

	ctx := context.Background()
	resp, err := r.client.CreateBetaTester(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to invite beta tester: %v", err)), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Successfully invited beta tester **%s**\n\n", resp.Data.Attributes.Email))
	sb.WriteString(fmt.Sprintf("- ID: %s\n", resp.Data.ID))
	sb.WriteString(fmt.Sprintf("- State: %s\n", resp.Data.Attributes.State))

	return mcp.NewSuccessResult(sb.String()), nil
}

// handleRemoveBetaTester handles the remove_beta_tester tool.
func (r *Registry) handleRemoveBetaTester(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BetaTesterID string `json:"beta_tester_id"`
	}

	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BetaTesterID == "" {
		return mcp.NewErrorResult("beta_tester_id is required"), nil
	}

	ctx := context.Background()
	if err := r.client.DeleteBetaTester(ctx, params.BetaTesterID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to remove beta tester: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Successfully removed beta tester %s", params.BetaTesterID)), nil
}

// handleAddTesterToGroup handles the add_tester_to_group tool.
func (r *Registry) handleAddTesterToGroup(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BetaGroupID  string `json:"beta_group_id"`
		BetaTesterID string `json:"beta_tester_id"`
	}

	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BetaGroupID == "" {
		return mcp.NewErrorResult("beta_group_id is required"), nil
	}
	if params.BetaTesterID == "" {
		return mcp.NewErrorResult("beta_tester_id is required"), nil
	}

	ctx := context.Background()
	if err := r.client.AddBetaTesterToGroup(ctx, params.BetaGroupID, params.BetaTesterID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to add tester to group: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Successfully added beta tester %s to group %s", params.BetaTesterID, params.BetaGroupID)), nil
}
