package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// accessibilitySupportProperties returns the JSON Schema entries shared by
// the create and update accessibility declaration tools, one boolean per
// accessibility feature Apple tracks.
func accessibilitySupportProperties() map[string]mcp.Property {
	return map[string]mcp.Property{
		"supports_audio_descriptions": {
			Type:        "boolean",
			Description: "Whether the app supports audio descriptions",
		},
		"supports_captions": {
			Type:        "boolean",
			Description: "Whether the app supports captions",
		},
		"supports_dark_interface": {
			Type:        "boolean",
			Description: "Whether the app supports a dark interface",
		},
		"supports_differentiate_without_color_alone": {
			Type:        "boolean",
			Description: "Whether the app differentiates content without relying on color alone",
		},
		"supports_larger_text": {
			Type:        "boolean",
			Description: "Whether the app supports larger text",
		},
		"supports_reduced_motion": {
			Type:        "boolean",
			Description: "Whether the app supports reduced motion",
		},
		"supports_sufficient_contrast": {
			Type:        "boolean",
			Description: "Whether the app provides sufficient contrast",
		},
		"supports_voice_control": {
			Type:        "boolean",
			Description: "Whether the app supports Voice Control",
		},
		"supports_voiceover": {
			Type:        "boolean",
			Description: "Whether the app supports VoiceOver",
		},
	}
}

// accessibilitySupportParams carries the optional accessibility feature
// flags shared by the create and update handlers.
type accessibilitySupportParams struct {
	SupportsAudioDescriptions              *bool `json:"supports_audio_descriptions"`
	SupportsCaptions                       *bool `json:"supports_captions"`
	SupportsDarkInterface                  *bool `json:"supports_dark_interface"`
	SupportsDifferentiateWithoutColorAlone *bool `json:"supports_differentiate_without_color_alone"`
	SupportsLargerText                     *bool `json:"supports_larger_text"`
	SupportsReducedMotion                  *bool `json:"supports_reduced_motion"`
	SupportsSufficientContrast             *bool `json:"supports_sufficient_contrast"`
	SupportsVoiceControl                   *bool `json:"supports_voice_control"`
	SupportsVoiceover                      *bool `json:"supports_voiceover"`
}

// registerAccessibilityTools registers accessibility declaration tools.
func (r *Registry) registerAccessibilityTools() {
	// List accessibility declarations
	r.register(mcp.Tool{
		Name:        "list_accessibility_declarations",
		Description: "List the per-device-family accessibility declarations for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to list accessibility declarations for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of declarations to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
				"filter": {
					Type:        "object",
					Description: "JSON:API filter map. Supported keys: deviceFamily (IPHONE, IPAD, APPLE_TV, APPLE_WATCH, MAC, VISION), state (DRAFT, PUBLISHED, REPLACED). Values are arrays, e.g. {\"state\": [\"PUBLISHED\"]}.",
				},
				"fields": {
					Type:        "object",
					Description: "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return.",
				},
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Accessibility Declarations",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAccessibilityDeclarations)

	// Get accessibility declaration
	r.register(mcp.Tool{
		Name:        "get_accessibility_declaration",
		Description: "Get details of a specific accessibility declaration",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"declaration_id": {
					Type:        "string",
					Description: "The accessibility declaration ID",
				},
			},
			Required: []string{"declaration_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Accessibility Declaration",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetAccessibilityDeclaration)

	// Create accessibility declaration
	createProps := map[string]mcp.Property{
		"app_id": {
			Type:        "string",
			Description: "The App ID to create the declaration for",
		},
		"device_family": {
			Type:        "string",
			Description: "Device family: IPHONE, IPAD, APPLE_TV, APPLE_WATCH, MAC, or VISION",
		},
	}
	for name, prop := range accessibilitySupportProperties() {
		createProps[name] = prop
	}
	r.register(mcp.Tool{
		Name:        "create_accessibility_declaration",
		Description: "Create a draft accessibility declaration for an app and device family",
		InputSchema: mcp.JSONSchema{
			Type:       "object",
			Properties: createProps,
			Required:   []string{"app_id", "device_family"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Accessibility Declaration",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateAccessibilityDeclaration)

	// Update accessibility declaration
	updateProps := map[string]mcp.Property{
		"declaration_id": {
			Type:        "string",
			Description: "The accessibility declaration ID",
		},
		"publish": {
			Type:        "boolean",
			Description: "Set true to publish the draft declaration to the App Store",
		},
	}
	for name, prop := range accessibilitySupportProperties() {
		updateProps[name] = prop
	}
	r.register(mcp.Tool{
		Name:        "update_accessibility_declaration",
		Description: "Update a draft accessibility declaration; set publish=true to publish it to the App Store",
		InputSchema: mcp.JSONSchema{
			Type:       "object",
			Properties: updateProps,
			Required:   []string{"declaration_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update Accessibility Declaration",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateAccessibilityDeclaration)

	// Delete accessibility declaration
	r.register(mcp.Tool{
		Name:        "delete_accessibility_declaration",
		Description: "Delete a draft accessibility declaration",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"declaration_id": {
					Type:        "string",
					Description: "The accessibility declaration ID",
				},
			},
			Required: []string{"declaration_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Accessibility Declaration",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteAccessibilityDeclaration)
}

func (r *Registry) handleListAccessibilityDeclarations(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID  string              `json:"app_id"`
		Limit  int                 `json:"limit"`
		Cursor string              `json:"cursor"`
		Filter map[string][]string `json:"filter"`
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AccessibilityDeclarationsResponse, error) {
		return r.client.ListAccessibilityDeclarations(ctx, params.AppID, listOpts(limit, params.Filter, nil, params.Fields, nil))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list accessibility declarations: %v", err)), nil
	}

	return newListResult(formatAccessibilityDeclarations(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetAccessibilityDeclaration(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DeclarationID string `json:"declaration_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DeclarationID == "" {
		return mcp.NewErrorResult("declaration_id is required"), nil
	}

	resp, err := r.client.GetAccessibilityDeclaration(ctx, params.DeclarationID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get accessibility declaration: %v", err)), nil
	}

	return newDataResult(formatAccessibilityDeclaration(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateAccessibilityDeclaration(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID        string `json:"app_id"`
		DeviceFamily string `json:"device_family"`
		accessibilitySupportParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}
	if params.DeviceFamily == "" {
		return mcp.NewErrorResult("device_family is required"), nil
	}

	req := &api.AccessibilityDeclarationCreateRequest{
		Data: api.AccessibilityDeclarationCreateData{
			Type: "accessibilityDeclarations",
			Attributes: api.AccessibilityDeclarationCreateAttributes{
				DeviceFamily:                           params.DeviceFamily,
				SupportsAudioDescriptions:              params.SupportsAudioDescriptions,
				SupportsCaptions:                       params.SupportsCaptions,
				SupportsDarkInterface:                  params.SupportsDarkInterface,
				SupportsDifferentiateWithoutColorAlone: params.SupportsDifferentiateWithoutColorAlone,
				SupportsLargerText:                     params.SupportsLargerText,
				SupportsReducedMotion:                  params.SupportsReducedMotion,
				SupportsSufficientContrast:             params.SupportsSufficientContrast,
				SupportsVoiceControl:                   params.SupportsVoiceControl,
				SupportsVoiceover:                      params.SupportsVoiceover,
			},
			Relationships: api.AccessibilityDeclarationCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "apps",
						ID:   params.AppID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateAccessibilityDeclaration(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create accessibility declaration: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created accessibility declaration:\n%s", formatAccessibilityDeclaration(resp.Data)), resp.Data), nil
}

func (r *Registry) handleUpdateAccessibilityDeclaration(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DeclarationID string `json:"declaration_id"`
		Publish       *bool  `json:"publish"`
		accessibilitySupportParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DeclarationID == "" {
		return mcp.NewErrorResult("declaration_id is required"), nil
	}

	req := &api.AccessibilityDeclarationUpdateRequest{
		Data: api.AccessibilityDeclarationUpdateData{
			Type: "accessibilityDeclarations",
			ID:   params.DeclarationID,
			Attributes: &api.AccessibilityDeclarationUpdateAttributes{
				Publish:                                params.Publish,
				SupportsAudioDescriptions:              params.SupportsAudioDescriptions,
				SupportsCaptions:                       params.SupportsCaptions,
				SupportsDarkInterface:                  params.SupportsDarkInterface,
				SupportsDifferentiateWithoutColorAlone: params.SupportsDifferentiateWithoutColorAlone,
				SupportsLargerText:                     params.SupportsLargerText,
				SupportsReducedMotion:                  params.SupportsReducedMotion,
				SupportsSufficientContrast:             params.SupportsSufficientContrast,
				SupportsVoiceControl:                   params.SupportsVoiceControl,
				SupportsVoiceover:                      params.SupportsVoiceover,
			},
		},
	}

	resp, err := r.client.UpdateAccessibilityDeclaration(ctx, params.DeclarationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update accessibility declaration: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Accessibility declaration updated:\n%s", formatAccessibilityDeclaration(resp.Data)), resp.Data), nil
}

func (r *Registry) handleDeleteAccessibilityDeclaration(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DeclarationID string `json:"declaration_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DeclarationID == "" {
		return mcp.NewErrorResult("declaration_id is required"), nil
	}

	if err := r.client.DeleteAccessibilityDeclaration(ctx, params.DeclarationID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete accessibility declaration: %v", err)), nil
	}

	return mcp.NewSuccessResult("Accessibility declaration deleted successfully"), nil
}

func formatAccessibilityDeclarations(declarations []api.AccessibilityDeclaration) string {
	if len(declarations) == 0 {
		return "No accessibility declarations found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d accessibility declarations:\n\n", len(declarations)))

	for _, decl := range declarations {
		sb.WriteString(formatAccessibilityDeclaration(decl))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAccessibilityDeclaration(decl api.AccessibilityDeclaration) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Declaration ID: %s\n", decl.ID))

	attrs := decl.Attributes
	if attrs.DeviceFamily != "" {
		sb.WriteString(fmt.Sprintf("Device Family: %s\n", attrs.DeviceFamily))
	}
	if attrs.State != "" {
		sb.WriteString(fmt.Sprintf("State: %s\n", attrs.State))
	}

	features := []struct {
		label string
		value *bool
	}{
		{"Audio Descriptions", attrs.SupportsAudioDescriptions},
		{"Captions", attrs.SupportsCaptions},
		{"Dark Interface", attrs.SupportsDarkInterface},
		{"Differentiate Without Color Alone", attrs.SupportsDifferentiateWithoutColorAlone},
		{"Larger Text", attrs.SupportsLargerText},
		{"Reduced Motion", attrs.SupportsReducedMotion},
		{"Sufficient Contrast", attrs.SupportsSufficientContrast},
		{"Voice Control", attrs.SupportsVoiceControl},
		{"VoiceOver", attrs.SupportsVoiceover},
	}
	for _, f := range features {
		if f.value != nil {
			sb.WriteString(fmt.Sprintf("%s: %t\n", f.label, *f.value))
		}
	}

	return sb.String()
}
