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

// registerProvisioningTools registers provisioning management tools.
func (r *Registry) registerProvisioningTools() {
	r.register(
		mcp.Tool{
			Name:        "list_bundle_ids",
			Description: "List all registered bundle IDs in your App Store Connect account. Returns bundle identifier, name, platform, and seed ID.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"limit": {
						Type:        "integer",
						Description: "Maximum number of bundle IDs to return (default: 50)",
						Default:     50,
					},
				},
			},
		},
		r.handleListBundleIDs,
	)

	r.register(
		mcp.Tool{
			Name:        "get_bundle_id",
			Description: "Get detailed information about a specific bundle ID.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"bundle_id_id": {
						Type:        "string",
						Description: "The App Store Connect ID of the bundle ID resource (not the bundle identifier string)",
					},
				},
				Required: []string{"bundle_id_id"},
			},
		},
		r.handleGetBundleID,
	)

	r.register(
		mcp.Tool{
			Name:        "list_certificates",
			Description: "List all signing certificates in your App Store Connect account. Returns certificate name, type, serial number, and expiration date.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"limit": {
						Type:        "integer",
						Description: "Maximum number of certificates to return (default: 50)",
						Default:     50,
					},
				},
			},
		},
		r.handleListCertificates,
	)

	r.register(
		mcp.Tool{
			Name:        "list_profiles",
			Description: "List all provisioning profiles in your App Store Connect account. Returns profile name, type, state, UUID, and expiration date.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"limit": {
						Type:        "integer",
						Description: "Maximum number of profiles to return (default: 50)",
						Default:     50,
					},
				},
			},
		},
		r.handleListProfiles,
	)

	r.register(
		mcp.Tool{
			Name:        "list_devices",
			Description: "List all registered devices in your App Store Connect account. Returns device name, UDID, model, platform, and status.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"limit": {
						Type:        "integer",
						Description: "Maximum number of devices to return (default: 50)",
						Default:     50,
					},
				},
			},
		},
		r.handleListDevices,
	)

	r.register(
		mcp.Tool{
			Name:        "register_device",
			Description: "Register a new device for development or ad hoc distribution.",
			InputSchema: mcp.JSONSchema{
				Type: "object",
				Properties: map[string]mcp.Property{
					"name": {
						Type:        "string",
						Description: "A name for the device (e.g., 'John's iPhone 15')",
					},
					"udid": {
						Type:        "string",
						Description: "The device's UDID",
					},
					"platform": {
						Type:        "string",
						Description: "The device platform",
						Enum:        []string{"IOS", "MAC_OS"},
					},
				},
				Required: []string{"name", "udid", "platform"},
			},
		},
		r.handleRegisterDevice,
	)
}

// handleListBundleIDs handles the list_bundle_ids tool.
func (r *Registry) handleListBundleIDs(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Limit int `json:"limit"`
	}
	params.Limit = 50

	if args != nil {
		if err := json.Unmarshal(args, &params); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
	}

	ctx := context.Background()
	resp, err := r.client.ListBundleIDs(ctx, params.Limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list bundle IDs: %v", err)), nil
	}

	if len(resp.Data) == 0 {
		return mcp.NewSuccessResult("No bundle IDs found."), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d bundle IDs:\n\n", len(resp.Data)))

	for _, bundleID := range resp.Data {
		sb.WriteString(fmt.Sprintf("**%s**\n", bundleID.Attributes.Name))
		sb.WriteString(fmt.Sprintf("  - ID: %s\n", bundleID.ID))
		sb.WriteString(fmt.Sprintf("  - Identifier: %s\n", bundleID.Attributes.Identifier))
		sb.WriteString(fmt.Sprintf("  - Platform: %s\n", bundleID.Attributes.Platform))
		if bundleID.Attributes.SeedID != "" {
			sb.WriteString(fmt.Sprintf("  - Seed ID: %s\n", bundleID.Attributes.SeedID))
		}
		sb.WriteString("\n")
	}

	return mcp.NewSuccessResult(sb.String()), nil
}

// handleGetBundleID handles the get_bundle_id tool.
func (r *Registry) handleGetBundleID(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BundleIDID string `json:"bundle_id_id"`
	}

	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BundleIDID == "" {
		return mcp.NewErrorResult("bundle_id_id is required"), nil
	}

	ctx := context.Background()
	resp, err := r.client.GetBundleID(ctx, params.BundleIDID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get bundle ID: %v", err)), nil
	}

	bundleID := resp.Data
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("**%s**\n\n", bundleID.Attributes.Name))
	sb.WriteString(fmt.Sprintf("- ID: %s\n", bundleID.ID))
	sb.WriteString(fmt.Sprintf("- Identifier: %s\n", bundleID.Attributes.Identifier))
	sb.WriteString(fmt.Sprintf("- Platform: %s\n", bundleID.Attributes.Platform))
	if bundleID.Attributes.SeedID != "" {
		sb.WriteString(fmt.Sprintf("- Seed ID: %s\n", bundleID.Attributes.SeedID))
	}

	return mcp.NewSuccessResult(sb.String()), nil
}

// handleListCertificates handles the list_certificates tool.
func (r *Registry) handleListCertificates(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Limit int `json:"limit"`
	}
	params.Limit = 50

	if args != nil {
		if err := json.Unmarshal(args, &params); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
	}

	ctx := context.Background()
	resp, err := r.client.ListCertificates(ctx, params.Limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list certificates: %v", err)), nil
	}

	if len(resp.Data) == 0 {
		return mcp.NewSuccessResult("No certificates found."), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d certificates:\n\n", len(resp.Data)))

	for _, cert := range resp.Data {
		displayName := cert.Attributes.DisplayName
		if displayName == "" {
			displayName = cert.Attributes.Name
		}
		sb.WriteString(fmt.Sprintf("**%s**\n", displayName))
		sb.WriteString(fmt.Sprintf("  - ID: %s\n", cert.ID))
		sb.WriteString(fmt.Sprintf("  - Type: %s\n", cert.Attributes.CertificateType))
		sb.WriteString(fmt.Sprintf("  - Serial Number: %s\n", cert.Attributes.SerialNumber))
		if cert.Attributes.Platform != "" {
			sb.WriteString(fmt.Sprintf("  - Platform: %s\n", cert.Attributes.Platform))
		}
		if cert.Attributes.ExpirationDate != nil {
			sb.WriteString(fmt.Sprintf("  - Expires: %s\n", cert.Attributes.ExpirationDate.Format("2006-01-02")))
		}
		sb.WriteString("\n")
	}

	return mcp.NewSuccessResult(sb.String()), nil
}

// handleListProfiles handles the list_profiles tool.
func (r *Registry) handleListProfiles(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Limit int `json:"limit"`
	}
	params.Limit = 50

	if args != nil {
		if err := json.Unmarshal(args, &params); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
	}

	ctx := context.Background()
	resp, err := r.client.ListProfiles(ctx, params.Limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list profiles: %v", err)), nil
	}

	if len(resp.Data) == 0 {
		return mcp.NewSuccessResult("No provisioning profiles found."), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d provisioning profiles:\n\n", len(resp.Data)))

	for _, profile := range resp.Data {
		sb.WriteString(fmt.Sprintf("**%s**\n", profile.Attributes.Name))
		sb.WriteString(fmt.Sprintf("  - ID: %s\n", profile.ID))
		sb.WriteString(fmt.Sprintf("  - UUID: %s\n", profile.Attributes.UUID))
		sb.WriteString(fmt.Sprintf("  - Type: %s\n", profile.Attributes.ProfileType))
		sb.WriteString(fmt.Sprintf("  - State: %s\n", profile.Attributes.ProfileState))
		sb.WriteString(fmt.Sprintf("  - Platform: %s\n", profile.Attributes.Platform))
		if profile.Attributes.CreatedDate != nil {
			sb.WriteString(fmt.Sprintf("  - Created: %s\n", profile.Attributes.CreatedDate.Format("2006-01-02")))
		}
		if profile.Attributes.ExpirationDate != nil {
			sb.WriteString(fmt.Sprintf("  - Expires: %s\n", profile.Attributes.ExpirationDate.Format("2006-01-02")))
		}
		sb.WriteString("\n")
	}

	return mcp.NewSuccessResult(sb.String()), nil
}

// handleListDevices handles the list_devices tool.
func (r *Registry) handleListDevices(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Limit int `json:"limit"`
	}
	params.Limit = 50

	if args != nil {
		if err := json.Unmarshal(args, &params); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
	}

	ctx := context.Background()
	resp, err := r.client.ListDevices(ctx, params.Limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list devices: %v", err)), nil
	}

	if len(resp.Data) == 0 {
		return mcp.NewSuccessResult("No devices found."), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d devices:\n\n", len(resp.Data)))

	for _, device := range resp.Data {
		sb.WriteString(fmt.Sprintf("**%s**\n", device.Attributes.Name))
		sb.WriteString(fmt.Sprintf("  - ID: %s\n", device.ID))
		sb.WriteString(fmt.Sprintf("  - UDID: %s\n", device.Attributes.UDID))
		sb.WriteString(fmt.Sprintf("  - Model: %s\n", device.Attributes.Model))
		sb.WriteString(fmt.Sprintf("  - Device Class: %s\n", device.Attributes.DeviceClass))
		sb.WriteString(fmt.Sprintf("  - Platform: %s\n", device.Attributes.Platform))
		sb.WriteString(fmt.Sprintf("  - Status: %s\n", device.Attributes.Status))
		if device.Attributes.AddedDate != nil {
			sb.WriteString(fmt.Sprintf("  - Added: %s\n", device.Attributes.AddedDate.Format("2006-01-02")))
		}
		sb.WriteString("\n")
	}

	return mcp.NewSuccessResult(sb.String()), nil
}

// handleRegisterDevice handles the register_device tool.
func (r *Registry) handleRegisterDevice(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		Name     string `json:"name"`
		UDID     string `json:"udid"`
		Platform string `json:"platform"`
	}

	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.Name == "" {
		return mcp.NewErrorResult("name is required"), nil
	}
	if params.UDID == "" {
		return mcp.NewErrorResult("udid is required"), nil
	}
	if params.Platform == "" {
		return mcp.NewErrorResult("platform is required"), nil
	}

	req := &api.DeviceCreateRequest{
		Data: api.DeviceCreateData{
			Type: "devices",
			Attributes: api.DeviceCreateAttributes{
				Name:     params.Name,
				UDID:     params.UDID,
				Platform: params.Platform,
			},
		},
	}

	ctx := context.Background()
	resp, err := r.client.RegisterDevice(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to register device: %v", err)), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Successfully registered device **%s**\n\n", resp.Data.Attributes.Name))
	sb.WriteString(fmt.Sprintf("- ID: %s\n", resp.Data.ID))
	sb.WriteString(fmt.Sprintf("- UDID: %s\n", resp.Data.Attributes.UDID))
	sb.WriteString(fmt.Sprintf("- Model: %s\n", resp.Data.Attributes.Model))
	sb.WriteString(fmt.Sprintf("- Platform: %s\n", resp.Data.Attributes.Platform))
	sb.WriteString(fmt.Sprintf("- Status: %s\n", resp.Data.Attributes.Status))

	return mcp.NewSuccessResult(sb.String()), nil
}
