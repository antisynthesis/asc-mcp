package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerUserTools registers user and role management tools.
func (r *Registry) registerUserTools() {
	// List users
	r.register(mcp.Tool{
		Name:        "list_users",
		Description: "List all users in the App Store Connect team",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"limit": {
					Type:        "integer",
					Description: "Maximum number of users to return (default 50)",
				},
			},
		},
	}, r.handleListUsers)

	// Get user
	r.register(mcp.Tool{
		Name:        "get_user",
		Description: "Get details of a specific user",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"user_id": {
					Type:        "string",
					Description: "The user ID",
				},
			},
			Required: []string{"user_id"},
		},
	}, r.handleGetUser)

	// Update user
	r.register(mcp.Tool{
		Name:        "update_user",
		Description: "Update a user's roles or all apps visibility",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"user_id": {
					Type:        "string",
					Description: "The user ID",
				},
				"roles": {
					Type:        "array",
					Description: "List of roles: ADMIN, FINANCE, ACCOUNT_HOLDER, SALES, MARKETING, APP_MANAGER, DEVELOPER, ACCESS_TO_REPORTS, CUSTOMER_SUPPORT, CREATE_APPS, CLOUD_MANAGED_DEVELOPER_ID, CLOUD_MANAGED_APP_DISTRIBUTION",
				},
				"all_apps_visible": {
					Type:        "boolean",
					Description: "Whether user can see all apps",
				},
			},
			Required: []string{"user_id"},
		},
	}, r.handleUpdateUser)

	// Delete user
	r.register(mcp.Tool{
		Name:        "delete_user",
		Description: "Remove a user from the App Store Connect team",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"user_id": {
					Type:        "string",
					Description: "The user ID to remove",
				},
			},
			Required: []string{"user_id"},
		},
	}, r.handleDeleteUser)

	// List user invitations
	r.register(mcp.Tool{
		Name:        "list_user_invitations",
		Description: "List pending user invitations",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"limit": {
					Type:        "integer",
					Description: "Maximum number of invitations to return (default 50)",
				},
			},
		},
	}, r.handleListUserInvitations)

	// Get user invitation
	r.register(mcp.Tool{
		Name:        "get_user_invitation",
		Description: "Get details of a specific user invitation",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"invitation_id": {
					Type:        "string",
					Description: "The user invitation ID",
				},
			},
			Required: []string{"invitation_id"},
		},
	}, r.handleGetUserInvitation)

	// Create user invitation
	r.register(mcp.Tool{
		Name:        "create_user_invitation",
		Description: "Invite a new user to the App Store Connect team",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"email": {
					Type:        "string",
					Description: "Email address of the user to invite",
				},
				"first_name": {
					Type:        "string",
					Description: "First name of the user",
				},
				"last_name": {
					Type:        "string",
					Description: "Last name of the user",
				},
				"roles": {
					Type:        "array",
					Description: "List of roles to assign: ADMIN, FINANCE, ACCOUNT_HOLDER, SALES, MARKETING, APP_MANAGER, DEVELOPER, ACCESS_TO_REPORTS, CUSTOMER_SUPPORT, CREATE_APPS, CLOUD_MANAGED_DEVELOPER_ID, CLOUD_MANAGED_APP_DISTRIBUTION",
				},
				"all_apps_visible": {
					Type:        "boolean",
					Description: "Whether user can see all apps (default true)",
				},
			},
			Required: []string{"email", "first_name", "last_name", "roles"},
		},
	}, r.handleCreateUserInvitation)

	// Delete user invitation (cancel pending invitation)
	r.register(mcp.Tool{
		Name:        "delete_user_invitation",
		Description: "Delete (cancel) a pending user invitation",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"invitation_id": {
					Type:        "string",
					Description: "The user invitation ID to delete",
				},
			},
			Required: []string{"invitation_id"},
		},
	}, r.handleDeleteUserInvitation)
}

func (r *Registry) handleListUsers(args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := r.client.ListUsers(context.Background(), limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list users: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatUsers(resp.Data)), nil
}

func (r *Registry) handleGetUser(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		UserID string `json:"user_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.UserID == "" {
		return nil, fmt.Errorf("user_id is required")
	}

	resp, err := r.client.GetUser(context.Background(), params.UserID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get user: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatUser(resp.Data)), nil
}

func (r *Registry) handleUpdateUser(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		UserID         string   `json:"user_id"`
		Roles          []string `json:"roles"`
		AllAppsVisible *bool    `json:"all_apps_visible"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.UserID == "" {
		return nil, fmt.Errorf("user_id is required")
	}

	req := &api.UserUpdateRequest{
		Data: api.UserUpdateData{
			Type: "users",
			ID:   params.UserID,
			Attributes: api.UserUpdateAttributes{
				Roles:          params.Roles,
				AllAppsVisible: params.AllAppsVisible,
			},
		},
	}

	resp, err := r.client.UpdateUser(context.Background(), params.UserID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update user: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("User updated successfully:\n%s", formatUser(resp.Data))), nil
}

func (r *Registry) handleDeleteUser(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		UserID string `json:"user_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.UserID == "" {
		return nil, fmt.Errorf("user_id is required")
	}

	err := r.client.DeleteUser(context.Background(), params.UserID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete user: %v", err)), nil
	}

	return mcp.NewSuccessResult("User removed successfully"), nil
}

func (r *Registry) handleListUserInvitations(args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := r.client.ListUserInvitations(context.Background(), limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list user invitations: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatUserInvitations(resp.Data)), nil
}

func (r *Registry) handleGetUserInvitation(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		InvitationID string `json:"invitation_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.InvitationID == "" {
		return nil, fmt.Errorf("invitation_id is required")
	}

	resp, err := r.client.GetUserInvitation(context.Background(), params.InvitationID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get user invitation: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatUserInvitation(resp.Data)), nil
}

func (r *Registry) handleCreateUserInvitation(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Email          string   `json:"email"`
		FirstName      string   `json:"first_name"`
		LastName       string   `json:"last_name"`
		Roles          []string `json:"roles"`
		AllAppsVisible *bool    `json:"all_apps_visible"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.Email == "" || params.FirstName == "" || params.LastName == "" {
		return nil, fmt.Errorf("email, first_name, and last_name are required")
	}

	if len(params.Roles) == 0 {
		return nil, fmt.Errorf("at least one role is required")
	}

	allAppsVisible := true
	if params.AllAppsVisible != nil {
		allAppsVisible = *params.AllAppsVisible
	}

	req := &api.UserInvitationCreateRequest{
		Data: api.UserInvitationCreateData{
			Type: "userInvitations",
			Attributes: api.UserInvitationCreateAttributes{
				Email:          params.Email,
				FirstName:      params.FirstName,
				LastName:       params.LastName,
				Roles:          params.Roles,
				AllAppsVisible: allAppsVisible,
			},
		},
	}

	resp, err := r.client.CreateUserInvitation(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create user invitation: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("User invitation sent:\n%s", formatUserInvitation(resp.Data))), nil
}

func (r *Registry) handleDeleteUserInvitation(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		InvitationID string `json:"invitation_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.InvitationID == "" {
		return nil, fmt.Errorf("invitation_id is required")
	}

	err := r.client.DeleteUserInvitation(context.Background(), params.InvitationID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete user invitation: %v", err)), nil
	}

	return mcp.NewSuccessResult("User invitation deleted"), nil
}

func formatUsers(users []api.User) string {
	if len(users) == 0 {
		return "No users found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d users:\n\n", len(users)))

	for _, user := range users {
		sb.WriteString(formatUser(user))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatUser(user api.User) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", user.ID))
	sb.WriteString(fmt.Sprintf("Username: %s\n", user.Attributes.Username))
	sb.WriteString(fmt.Sprintf("Name: %s %s\n", user.Attributes.FirstName, user.Attributes.LastName))
	if len(user.Attributes.Roles) > 0 {
		sb.WriteString(fmt.Sprintf("Roles: %s\n", strings.Join(user.Attributes.Roles, ", ")))
	}
	sb.WriteString(fmt.Sprintf("All Apps Visible: %t\n", user.Attributes.AllAppsVisible))
	sb.WriteString(fmt.Sprintf("Provisioning Allowed: %t\n", user.Attributes.ProvisioningAllowed))
	return sb.String()
}

func formatUserInvitations(invitations []api.UserInvitation) string {
	if len(invitations) == 0 {
		return "No pending user invitations found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d pending invitations:\n\n", len(invitations)))

	for _, inv := range invitations {
		sb.WriteString(formatUserInvitation(inv))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatUserInvitation(inv api.UserInvitation) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", inv.ID))
	sb.WriteString(fmt.Sprintf("Email: %s\n", inv.Attributes.Email))
	sb.WriteString(fmt.Sprintf("Name: %s %s\n", inv.Attributes.FirstName, inv.Attributes.LastName))
	if len(inv.Attributes.Roles) > 0 {
		sb.WriteString(fmt.Sprintf("Roles: %s\n", strings.Join(inv.Attributes.Roles, ", ")))
	}
	sb.WriteString(fmt.Sprintf("All Apps Visible: %t\n", inv.Attributes.AllAppsVisible))
	if inv.Attributes.ExpirationDate != nil {
		sb.WriteString(fmt.Sprintf("Expires: %s\n", inv.Attributes.ExpirationDate.Format("2006-01-02 15:04")))
	}
	return sb.String()
}
