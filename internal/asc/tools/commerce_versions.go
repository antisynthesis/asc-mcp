package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerCommerceVersionTools registers in-app purchase, subscription
// and subscription group versioning tools (App Store Connect API 4.4.1).
//
// Commerce products are versioned the same way App Store versions are:
// create a version, edit that version's localizations and images through
// the v2 collections, then attach the version to a review submission
// with add_review_submission_item.
func (r *Registry) registerCommerceVersionTools() {
	// Create in-app purchase version
	r.register(mcp.Tool{
		Name:        "create_in_app_purchase_version",
		Description: "Create a new version of an in-app purchase. The version snapshots the product's current metadata; edit its localizations and images, then submit it with add_review_submission_item.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"iap_id": {
					Type:        "string",
					Description: "The in-app purchase ID",
				},
			},
			Required: []string{"iap_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create In App Purchase Version",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateInAppPurchaseVersion)

	// Get in-app purchase version
	r.register(mcp.Tool{
		Name:        "get_in_app_purchase_version",
		Description: "Get an in-app purchase version and its review state",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The in-app purchase version ID",
				},
			},
			Required: []string{"version_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get In App Purchase Version",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetInAppPurchaseVersion)

	// List in-app purchase versions
	r.register(mcp.Tool{
		Name:        "list_in_app_purchase_versions",
		Description: "List the versions of an in-app purchase",
		InputSchema: commerceVersionListSchema("iap_id", "The in-app purchase ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List In App Purchase Versions",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListInAppPurchaseVersions)

	// List in-app purchase version localizations
	r.register(mcp.Tool{
		Name:        "list_in_app_purchase_version_localizations",
		Description: "List the localizations of an in-app purchase version",
		InputSchema: commerceVersionListSchema("version_id", "The in-app purchase version ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List In App Purchase Version Localizations",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListInAppPurchaseVersionLocalizations)

	// Create in-app purchase localization
	r.register(mcp.Tool{
		Name:        "create_in_app_purchase_localization",
		Description: "Add a localization to an in-app purchase version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The in-app purchase version ID",
				},
				"locale": {
					Type:        "string",
					Description: "The locale, e.g. en-US",
				},
				"name": {
					Type:        "string",
					Description: "The display name shown to customers",
				},
				"description": {
					Type:        "string",
					Description: "The description shown to customers",
				},
			},
			Required: []string{"version_id", "locale", "name"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create In App Purchase Localization",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateInAppPurchaseLocalization)

	// Update in-app purchase localization
	r.register(mcp.Tool{
		Name:        "update_in_app_purchase_localization",
		Description: "Update the name or description of an in-app purchase localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The in-app purchase localization ID",
				},
				"name": {
					Type:        "string",
					Description: "The updated display name",
				},
				"description": {
					Type:        "string",
					Description: "The updated description",
				},
			},
			Required: []string{"localization_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update In App Purchase Localization",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateInAppPurchaseLocalization)

	// Delete in-app purchase localization
	r.register(mcp.Tool{
		Name:        "delete_in_app_purchase_localization",
		Description: "Delete an in-app purchase localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The in-app purchase localization ID",
				},
			},
			Required: []string{"localization_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete In App Purchase Localization",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteInAppPurchaseLocalization)

	// List in-app purchase version images
	r.register(mcp.Tool{
		Name:        "list_in_app_purchase_version_images",
		Description: "List the promotional images of an in-app purchase version",
		InputSchema: commerceVersionListSchema("version_id", "The in-app purchase version ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List In App Purchase Version Images",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListInAppPurchaseVersionImages)

	// Create in-app purchase image
	r.register(mcp.Tool{
		Name:        "create_in_app_purchase_image",
		Description: "Reserve a promotional image upload on an in-app purchase version. Returns the upload operations to perform, then commit with update_in_app_purchase_image.",
		InputSchema: commerceImageCreateSchema("The in-app purchase version ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create In App Purchase Image",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateInAppPurchaseImage)

	// Update in-app purchase image
	r.register(mcp.Tool{
		Name:        "update_in_app_purchase_image",
		Description: "Mark an in-app purchase image upload complete",
		InputSchema: commerceImageUpdateSchema("The in-app purchase image ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update In App Purchase Image",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateInAppPurchaseImage)

	// Delete in-app purchase image
	r.register(mcp.Tool{
		Name:        "delete_in_app_purchase_image",
		Description: "Delete an in-app purchase promotional image",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"image_id": {
					Type:        "string",
					Description: "The in-app purchase image ID",
				},
			},
			Required: []string{"image_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete In App Purchase Image",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteInAppPurchaseImage)

	// Create subscription version
	r.register(mcp.Tool{
		Name:        "create_subscription_version",
		Description: "Create a new version of a subscription. Edit its localizations and images, then submit it with add_review_submission_item.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"subscription_id": {
					Type:        "string",
					Description: "The subscription ID",
				},
			},
			Required: []string{"subscription_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Subscription Version",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateSubscriptionVersion)

	// Get subscription version
	r.register(mcp.Tool{
		Name:        "get_subscription_version",
		Description: "Get a subscription version and its review state",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The subscription version ID",
				},
			},
			Required: []string{"version_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Subscription Version",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetSubscriptionVersion)

	// List subscription versions
	r.register(mcp.Tool{
		Name:        "list_subscription_versions",
		Description: "List the versions of a subscription",
		InputSchema: commerceVersionListSchema("subscription_id", "The subscription ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Subscription Versions",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListSubscriptionVersions)

	// List subscription version localizations
	r.register(mcp.Tool{
		Name:        "list_subscription_version_localizations",
		Description: "List the localizations of a subscription version",
		InputSchema: commerceVersionListSchema("version_id", "The subscription version ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Subscription Version Localizations",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListSubscriptionVersionLocalizations)

	// Create subscription localization
	r.register(mcp.Tool{
		Name:        "create_subscription_localization",
		Description: "Add a localization to a subscription version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The subscription version ID",
				},
				"locale": {
					Type:        "string",
					Description: "The locale, e.g. en-US",
				},
				"name": {
					Type:        "string",
					Description: "The display name shown to customers",
				},
				"description": {
					Type:        "string",
					Description: "The description shown to customers",
				},
			},
			Required: []string{"version_id", "locale", "name"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Subscription Localization",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateSubscriptionLocalization)

	// Update subscription localization
	r.register(mcp.Tool{
		Name:        "update_subscription_localization",
		Description: "Update the name or description of a subscription localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The subscription localization ID",
				},
				"name": {
					Type:        "string",
					Description: "The updated display name",
				},
				"description": {
					Type:        "string",
					Description: "The updated description",
				},
			},
			Required: []string{"localization_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update Subscription Localization",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateSubscriptionLocalization)

	// Delete subscription localization
	r.register(mcp.Tool{
		Name:        "delete_subscription_localization",
		Description: "Delete a subscription localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The subscription localization ID",
				},
			},
			Required: []string{"localization_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Subscription Localization",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteSubscriptionLocalization)

	// List subscription version images
	r.register(mcp.Tool{
		Name:        "list_subscription_version_images",
		Description: "List the promotional images of a subscription version",
		InputSchema: commerceVersionListSchema("version_id", "The subscription version ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Subscription Version Images",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListSubscriptionVersionImages)

	// Create subscription image
	r.register(mcp.Tool{
		Name:        "create_subscription_image",
		Description: "Reserve a promotional image upload on a subscription version. Returns the upload operations to perform, then commit with update_subscription_image.",
		InputSchema: commerceImageCreateSchema("The subscription version ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Subscription Image",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateSubscriptionImage)

	// Update subscription image
	r.register(mcp.Tool{
		Name:        "update_subscription_image",
		Description: "Mark a subscription image upload complete",
		InputSchema: commerceImageUpdateSchema("The subscription image ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update Subscription Image",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateSubscriptionImage)

	// Delete subscription image
	r.register(mcp.Tool{
		Name:        "delete_subscription_image",
		Description: "Delete a subscription promotional image",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"image_id": {
					Type:        "string",
					Description: "The subscription image ID",
				},
			},
			Required: []string{"image_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Subscription Image",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteSubscriptionImage)

	// Create subscription group version
	r.register(mcp.Tool{
		Name:        "create_subscription_group_version",
		Description: "Create a new version of a subscription group. Edit its localizations, then submit it with add_review_submission_item.",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"group_id": {
					Type:        "string",
					Description: "The subscription group ID",
				},
			},
			Required: []string{"group_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Subscription Group Version",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateSubscriptionGroupVersion)

	// Get subscription group version
	r.register(mcp.Tool{
		Name:        "get_subscription_group_version",
		Description: "Get a subscription group version and its review state",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The subscription group version ID",
				},
			},
			Required: []string{"version_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Subscription Group Version",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetSubscriptionGroupVersion)

	// List subscription group versions
	r.register(mcp.Tool{
		Name:        "list_subscription_group_versions",
		Description: "List the versions of a subscription group",
		InputSchema: commerceVersionListSchema("group_id", "The subscription group ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Subscription Group Versions",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListSubscriptionGroupVersions)

	// List subscription group version localizations
	r.register(mcp.Tool{
		Name:        "list_subscription_group_version_localizations",
		Description: "List the localizations of a subscription group version",
		InputSchema: commerceVersionListSchema("version_id", "The subscription group version ID"),
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Subscription Group Version Localizations",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListSubscriptionGroupVersionLocalizations)

	// Create subscription group localization
	r.register(mcp.Tool{
		Name:        "create_subscription_group_localization",
		Description: "Add a localization to a subscription group version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The subscription group version ID",
				},
				"locale": {
					Type:        "string",
					Description: "The locale, e.g. en-US",
				},
				"name": {
					Type:        "string",
					Description: "The subscription group display name",
				},
				"custom_app_name": {
					Type:        "string",
					Description: "An alternate app name to show on the subscription management screen",
				},
			},
			Required: []string{"version_id", "locale", "name"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Subscription Group Localization",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateSubscriptionGroupLocalization)

	// Update subscription group localization
	r.register(mcp.Tool{
		Name:        "update_subscription_group_localization",
		Description: "Update the name or custom app name of a subscription group localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The subscription group localization ID",
				},
				"name": {
					Type:        "string",
					Description: "The updated subscription group display name",
				},
				"custom_app_name": {
					Type:        "string",
					Description: "The updated alternate app name",
				},
			},
			Required: []string{"localization_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update Subscription Group Localization",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateSubscriptionGroupLocalization)

	// Delete subscription group localization
	r.register(mcp.Tool{
		Name:        "delete_subscription_group_localization",
		Description: "Delete a subscription group localization",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"localization_id": {
					Type:        "string",
					Description: "The subscription group localization ID",
				},
			},
			Required: []string{"localization_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Subscription Group Localization",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteSubscriptionGroupLocalization)
}

// commerceVersionListSchema builds the input schema shared by the
// commerce list tools: one owning resource ID plus the standard
// pagination and JSON:API query knobs.
func commerceVersionListSchema(idKey, idDescription string) mcp.JSONSchema {
	return mcp.JSONSchema{
		Type: "object",
		Properties: map[string]mcp.Property{
			idKey: {
				Type:        "string",
				Description: idDescription,
			},
			"limit": {
				Type:        "integer",
				Description: "Maximum number of results to return (default 50)",
			},
			"cursor": cursorProperty(),
			"filter": {
				Type:        "object",
				Description: "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"APPROVED\"]} becomes filter[state]=APPROVED.",
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
		Required: []string{idKey},
	}
}

// commerceImageCreateSchema builds the input schema shared by the two
// commerce image reservation tools.
func commerceImageCreateSchema(versionDescription string) mcp.JSONSchema {
	return mcp.JSONSchema{
		Type: "object",
		Properties: map[string]mcp.Property{
			"version_id": {
				Type:        "string",
				Description: versionDescription,
			},
			"file_name": {
				Type:        "string",
				Description: "The image file name",
			},
			"file_size": {
				Type:        "integer",
				Description: "The image file size in bytes",
			},
		},
		Required: []string{"version_id", "file_name", "file_size"},
	}
}

// commerceImageUpdateSchema builds the input schema shared by the two
// commerce image commit tools.
func commerceImageUpdateSchema(imageDescription string) mcp.JSONSchema {
	return mcp.JSONSchema{
		Type: "object",
		Properties: map[string]mcp.Property{
			"image_id": {
				Type:        "string",
				Description: imageDescription,
			},
			"uploaded": {
				Type:        "boolean",
				Description: "Set true once every upload operation has completed (default true)",
			},
		},
		Required: []string{"image_id"},
	}
}

// commerceListParams captures the arguments every commerce list tool
// accepts. The owning resource ID is read separately because its JSON
// key differs per tool.
type commerceListParams struct {
	Limit   int                 `json:"limit"`
	Cursor  string              `json:"cursor"`
	Filter  map[string][]string `json:"filter"`
	Fields  map[string][]string `json:"fields"`
	Include []string            `json:"include"`
}

// opts clamps the limit the same way every other list tool does and
// assembles the query options.
func (p commerceListParams) opts(defaultLimit int) *api.ListOptions {
	limit := p.Limit
	if limit <= 0 {
		limit = defaultLimit
	}
	if limit > 200 {
		limit = 200
	}
	return listOpts(limit, p.Filter, nil, p.Fields, p.Include)
}

func (r *Registry) handleCreateInAppPurchaseVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		IAPID string `json:"iap_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.IAPID == "" {
		return mcp.NewErrorResult("iap_id is required"), nil
	}

	req := &api.InAppPurchaseVersionCreateRequest{
		Data: api.InAppPurchaseVersionCreateData{
			Type: "inAppPurchaseVersions",
			Relationships: api.InAppPurchaseVersionCreateRelationships{
				InAppPurchase: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "inAppPurchases", ID: params.IAPID},
				},
			},
		},
	}

	resp, err := r.client.CreateInAppPurchaseVersion(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create in-app purchase version: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("In-app purchase version created:\n%s", formatCommerceVersion(resp.Data.ID, resp.Data.Attributes)), resp.Data), nil
}

func (r *Registry) handleGetInAppPurchaseVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	resp, err := r.client.GetInAppPurchaseVersion(ctx, params.VersionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get in-app purchase version: %v", err)), nil
	}

	return newDataResult(formatCommerceVersion(resp.Data.ID, resp.Data.Attributes), resp.Data), nil
}

func (r *Registry) handleListInAppPurchaseVersions(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		IAPID string `json:"iap_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.IAPID == "" {
		return mcp.NewErrorResult("iap_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.InAppPurchaseVersionsResponse, error) {
		return r.client.ListInAppPurchaseVersions(ctx, params.IAPID, params.opts(50))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list in-app purchase versions: %v", err)), nil
	}

	versions := make([]commerceVersionRow, 0, len(resp.Data))
	for _, v := range resp.Data {
		versions = append(versions, commerceVersionRow{ID: v.ID, Attributes: v.Attributes})
	}

	return newListResult(formatCommerceVersions(versions), resp.Data, resp.Links), nil
}

func (r *Registry) handleListInAppPurchaseVersionLocalizations(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.InAppPurchaseLocalizationsResponse, error) {
		return r.client.ListInAppPurchaseVersionLocalizations(ctx, params.VersionID, params.opts(50))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list in-app purchase version localizations: %v", err)), nil
	}

	return newListResult(formatCommerceLocalizations(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleCreateInAppPurchaseLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID   string `json:"version_id"`
		Locale      string `json:"locale"`
		Name        string `json:"name"`
		Description string `json:"description"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}
	if params.Locale == "" {
		return mcp.NewErrorResult("locale is required"), nil
	}
	if params.Name == "" {
		return mcp.NewErrorResult("name is required"), nil
	}

	req := &api.InAppPurchaseLocalizationCreateRequest{
		Data: api.InAppPurchaseLocalizationCreateData{
			Type: "inAppPurchaseLocalizations",
			Attributes: api.InAppPurchaseLocalizationCreateAttributes{
				Name:        params.Name,
				Locale:      params.Locale,
				Description: params.Description,
			},
			Relationships: api.InAppPurchaseLocalizationCreateRelationships{
				Version: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "inAppPurchaseVersions", ID: params.VersionID},
				},
			},
		},
	}

	resp, err := r.client.CreateInAppPurchaseLocalization(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create in-app purchase localization: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("In-app purchase localization created:\n%s", formatCommerceLocalization(resp.Data)), resp.Data), nil
}

func (r *Registry) handleUpdateInAppPurchaseLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
		Name           string `json:"name"`
		Description    string `json:"description"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	req := &api.InAppPurchaseLocalizationUpdateRequest{
		Data: api.InAppPurchaseLocalizationUpdateData{
			Type: "inAppPurchaseLocalizations",
			ID:   params.LocalizationID,
			Attributes: api.InAppPurchaseLocalizationUpdateAttributes{
				Name:        params.Name,
				Description: params.Description,
			},
		},
	}

	resp, err := r.client.UpdateInAppPurchaseLocalization(ctx, params.LocalizationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update in-app purchase localization: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("In-app purchase localization updated:\n%s", formatCommerceLocalization(resp.Data)), resp.Data), nil
}

func (r *Registry) handleDeleteInAppPurchaseLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	if err := r.client.DeleteInAppPurchaseLocalization(ctx, params.LocalizationID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete in-app purchase localization: %v", err)), nil
	}

	return mcp.NewSuccessResult("In-app purchase localization deleted successfully"), nil
}

func (r *Registry) handleListInAppPurchaseVersionImages(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.InAppPurchaseImagesResponse, error) {
		return r.client.ListInAppPurchaseVersionImages(ctx, params.VersionID, params.opts(50))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list in-app purchase version images: %v", err)), nil
	}

	images := make([]commerceImageRow, 0, len(resp.Data))
	for _, img := range resp.Data {
		images = append(images, commerceImageRow{ID: img.ID, Attributes: img.Attributes})
	}

	return newListResult(formatCommerceImages(images), resp.Data, resp.Links), nil
}

func (r *Registry) handleCreateInAppPurchaseImage(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	params, errResult := parseCommerceImageCreate(args)
	if errResult != nil {
		return errResult, nil
	}

	req := &api.InAppPurchaseImageCreateRequest{
		Data: api.InAppPurchaseImageCreateData{
			Type: "inAppPurchaseImages",
			Attributes: api.CommerceImageCreateAttributes{
				FileName: params.FileName,
				FileSize: params.FileSize,
			},
			Relationships: api.InAppPurchaseImageCreateRelationships{
				Version: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "inAppPurchaseVersions", ID: params.VersionID},
				},
			},
		},
	}

	resp, err := r.client.CreateInAppPurchaseImage(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create in-app purchase image: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("In-app purchase image reserved:\n%s", formatCommerceImage(commerceImageRow{ID: resp.Data.ID, Attributes: resp.Data.Attributes})), resp.Data), nil
}

func (r *Registry) handleUpdateInAppPurchaseImage(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	imageID, uploaded, errResult := parseCommerceImageUpdate(args)
	if errResult != nil {
		return errResult, nil
	}

	req := &api.InAppPurchaseImageUpdateRequest{
		Data: api.InAppPurchaseImageUpdateData{
			Type:       "inAppPurchaseImages",
			ID:         imageID,
			Attributes: api.CommerceImageUpdateAttributes{Uploaded: uploaded},
		},
	}

	resp, err := r.client.UpdateInAppPurchaseImage(ctx, imageID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update in-app purchase image: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("In-app purchase image updated:\n%s", formatCommerceImage(commerceImageRow{ID: resp.Data.ID, Attributes: resp.Data.Attributes})), resp.Data), nil
}

func (r *Registry) handleDeleteInAppPurchaseImage(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ImageID string `json:"image_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ImageID == "" {
		return mcp.NewErrorResult("image_id is required"), nil
	}

	if err := r.client.DeleteInAppPurchaseImage(ctx, params.ImageID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete in-app purchase image: %v", err)), nil
	}

	return mcp.NewSuccessResult("In-app purchase image deleted successfully"), nil
}

func (r *Registry) handleCreateSubscriptionVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubscriptionID string `json:"subscription_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubscriptionID == "" {
		return mcp.NewErrorResult("subscription_id is required"), nil
	}

	req := &api.SubscriptionVersionCreateRequest{
		Data: api.SubscriptionVersionCreateData{
			Type: "subscriptionVersions",
			Relationships: api.SubscriptionVersionCreateRelationships{
				Subscription: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "subscriptions", ID: params.SubscriptionID},
				},
			},
		},
	}

	resp, err := r.client.CreateSubscriptionVersion(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create subscription version: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Subscription version created:\n%s", formatCommerceVersion(resp.Data.ID, resp.Data.Attributes)), resp.Data), nil
}

func (r *Registry) handleGetSubscriptionVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	resp, err := r.client.GetSubscriptionVersion(ctx, params.VersionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get subscription version: %v", err)), nil
	}

	return newDataResult(formatCommerceVersion(resp.Data.ID, resp.Data.Attributes), resp.Data), nil
}

func (r *Registry) handleListSubscriptionVersions(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SubscriptionID string `json:"subscription_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SubscriptionID == "" {
		return mcp.NewErrorResult("subscription_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.SubscriptionVersionsResponse, error) {
		return r.client.ListSubscriptionVersions(ctx, params.SubscriptionID, params.opts(50))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list subscription versions: %v", err)), nil
	}

	versions := make([]commerceVersionRow, 0, len(resp.Data))
	for _, v := range resp.Data {
		versions = append(versions, commerceVersionRow{ID: v.ID, Attributes: v.Attributes})
	}

	return newListResult(formatCommerceVersions(versions), resp.Data, resp.Links), nil
}

func (r *Registry) handleListSubscriptionVersionLocalizations(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.SubscriptionLocalizationsResponse, error) {
		return r.client.ListSubscriptionVersionLocalizations(ctx, params.VersionID, params.opts(50))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list subscription version localizations: %v", err)), nil
	}

	locs := make([]api.InAppPurchaseLocalization, 0, len(resp.Data))
	for _, loc := range resp.Data {
		locs = append(locs, api.InAppPurchaseLocalization{ID: loc.ID, Attributes: loc.Attributes})
	}

	return newListResult(formatCommerceLocalizations(locs), resp.Data, resp.Links), nil
}

func (r *Registry) handleCreateSubscriptionLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID   string `json:"version_id"`
		Locale      string `json:"locale"`
		Name        string `json:"name"`
		Description string `json:"description"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}
	if params.Locale == "" {
		return mcp.NewErrorResult("locale is required"), nil
	}
	if params.Name == "" {
		return mcp.NewErrorResult("name is required"), nil
	}

	req := &api.SubscriptionLocalizationCreateRequest{
		Data: api.SubscriptionLocalizationCreateData{
			Type: "subscriptionLocalizations",
			Attributes: api.InAppPurchaseLocalizationCreateAttributes{
				Name:        params.Name,
				Locale:      params.Locale,
				Description: params.Description,
			},
			Relationships: api.SubscriptionLocalizationRelationships{
				Version: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "subscriptionVersions", ID: params.VersionID},
				},
			},
		},
	}

	resp, err := r.client.CreateSubscriptionLocalization(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create subscription localization: %v", err)), nil
	}

	row := api.InAppPurchaseLocalization{ID: resp.Data.ID, Attributes: resp.Data.Attributes}
	return newDataResult(fmt.Sprintf("Subscription localization created:\n%s", formatCommerceLocalization(row)), resp.Data), nil
}

func (r *Registry) handleUpdateSubscriptionLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
		Name           string `json:"name"`
		Description    string `json:"description"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	req := &api.SubscriptionLocalizationUpdateRequest{
		Data: api.SubscriptionLocalizationUpdateData{
			Type: "subscriptionLocalizations",
			ID:   params.LocalizationID,
			Attributes: api.InAppPurchaseLocalizationUpdateAttributes{
				Name:        params.Name,
				Description: params.Description,
			},
		},
	}

	resp, err := r.client.UpdateSubscriptionLocalization(ctx, params.LocalizationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update subscription localization: %v", err)), nil
	}

	row := api.InAppPurchaseLocalization{ID: resp.Data.ID, Attributes: resp.Data.Attributes}
	return newDataResult(fmt.Sprintf("Subscription localization updated:\n%s", formatCommerceLocalization(row)), resp.Data), nil
}

func (r *Registry) handleDeleteSubscriptionLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	if err := r.client.DeleteSubscriptionLocalization(ctx, params.LocalizationID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete subscription localization: %v", err)), nil
	}

	return mcp.NewSuccessResult("Subscription localization deleted successfully"), nil
}

func (r *Registry) handleListSubscriptionVersionImages(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.SubscriptionImagesResponse, error) {
		return r.client.ListSubscriptionVersionImages(ctx, params.VersionID, params.opts(50))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list subscription version images: %v", err)), nil
	}

	images := make([]commerceImageRow, 0, len(resp.Data))
	for _, img := range resp.Data {
		images = append(images, commerceImageRow{ID: img.ID, Attributes: img.Attributes})
	}

	return newListResult(formatCommerceImages(images), resp.Data, resp.Links), nil
}

func (r *Registry) handleCreateSubscriptionImage(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	params, errResult := parseCommerceImageCreate(args)
	if errResult != nil {
		return errResult, nil
	}

	req := &api.SubscriptionImageCreateRequest{
		Data: api.SubscriptionImageCreateData{
			Type: "subscriptionImages",
			Attributes: api.CommerceImageCreateAttributes{
				FileName: params.FileName,
				FileSize: params.FileSize,
			},
			Relationships: api.SubscriptionLocalizationRelationships{
				Version: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "subscriptionVersions", ID: params.VersionID},
				},
			},
		},
	}

	resp, err := r.client.CreateSubscriptionImage(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create subscription image: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Subscription image reserved:\n%s", formatCommerceImage(commerceImageRow{ID: resp.Data.ID, Attributes: resp.Data.Attributes})), resp.Data), nil
}

func (r *Registry) handleUpdateSubscriptionImage(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	imageID, uploaded, errResult := parseCommerceImageUpdate(args)
	if errResult != nil {
		return errResult, nil
	}

	req := &api.SubscriptionImageUpdateRequest{
		Data: api.SubscriptionImageUpdateData{
			Type:       "subscriptionImages",
			ID:         imageID,
			Attributes: api.CommerceImageUpdateAttributes{Uploaded: uploaded},
		},
	}

	resp, err := r.client.UpdateSubscriptionImage(ctx, imageID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update subscription image: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Subscription image updated:\n%s", formatCommerceImage(commerceImageRow{ID: resp.Data.ID, Attributes: resp.Data.Attributes})), resp.Data), nil
}

func (r *Registry) handleDeleteSubscriptionImage(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ImageID string `json:"image_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ImageID == "" {
		return mcp.NewErrorResult("image_id is required"), nil
	}

	if err := r.client.DeleteSubscriptionImage(ctx, params.ImageID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete subscription image: %v", err)), nil
	}

	return mcp.NewSuccessResult("Subscription image deleted successfully"), nil
}

func (r *Registry) handleCreateSubscriptionGroupVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		GroupID string `json:"group_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.GroupID == "" {
		return mcp.NewErrorResult("group_id is required"), nil
	}

	req := &api.SubscriptionGroupVersionCreateRequest{
		Data: api.SubscriptionGroupVersionCreateData{
			Type: "subscriptionGroupVersions",
			Relationships: api.SubscriptionGroupVersionCreateRelationships{
				SubscriptionGroup: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "subscriptionGroups", ID: params.GroupID},
				},
			},
		},
	}

	resp, err := r.client.CreateSubscriptionGroupVersion(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create subscription group version: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Subscription group version created:\n%s", formatCommerceVersion(resp.Data.ID, resp.Data.Attributes)), resp.Data), nil
}

func (r *Registry) handleGetSubscriptionGroupVersion(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	resp, err := r.client.GetSubscriptionGroupVersion(ctx, params.VersionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get subscription group version: %v", err)), nil
	}

	return newDataResult(formatCommerceVersion(resp.Data.ID, resp.Data.Attributes), resp.Data), nil
}

func (r *Registry) handleListSubscriptionGroupVersions(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		GroupID string `json:"group_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.GroupID == "" {
		return mcp.NewErrorResult("group_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.SubscriptionGroupVersionsResponse, error) {
		return r.client.ListSubscriptionGroupVersions(ctx, params.GroupID, params.opts(50))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list subscription group versions: %v", err)), nil
	}

	versions := make([]commerceVersionRow, 0, len(resp.Data))
	for _, v := range resp.Data {
		versions = append(versions, commerceVersionRow{ID: v.ID, Attributes: v.Attributes})
	}

	return newListResult(formatCommerceVersions(versions), resp.Data, resp.Links), nil
}

func (r *Registry) handleListSubscriptionGroupVersionLocalizations(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
		commerceListParams
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.SubscriptionGroupLocalizationsResponse, error) {
		return r.client.ListSubscriptionGroupVersionLocalizations(ctx, params.VersionID, params.opts(50))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list subscription group version localizations: %v", err)), nil
	}

	return newListResult(formatSubscriptionGroupLocalizations(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleCreateSubscriptionGroupLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID     string `json:"version_id"`
		Locale        string `json:"locale"`
		Name          string `json:"name"`
		CustomAppName string `json:"custom_app_name"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return mcp.NewErrorResult("version_id is required"), nil
	}
	if params.Locale == "" {
		return mcp.NewErrorResult("locale is required"), nil
	}
	if params.Name == "" {
		return mcp.NewErrorResult("name is required"), nil
	}

	req := &api.SubscriptionGroupLocalizationCreateRequest{
		Data: api.SubscriptionGroupLocalizationCreateData{
			Type: "subscriptionGroupLocalizations",
			Attributes: api.SubscriptionGroupLocalizationCreateAttributes{
				Name:          params.Name,
				Locale:        params.Locale,
				CustomAppName: params.CustomAppName,
			},
			Relationships: api.SubscriptionLocalizationRelationships{
				Version: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "subscriptionGroupVersions", ID: params.VersionID},
				},
			},
		},
	}

	resp, err := r.client.CreateSubscriptionGroupLocalization(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create subscription group localization: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Subscription group localization created:\n%s", formatSubscriptionGroupLocalization(resp.Data)), resp.Data), nil
}

func (r *Registry) handleUpdateSubscriptionGroupLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
		Name           string `json:"name"`
		CustomAppName  string `json:"custom_app_name"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	req := &api.SubscriptionGroupLocalizationUpdateRequest{
		Data: api.SubscriptionGroupLocalizationUpdateData{
			Type: "subscriptionGroupLocalizations",
			ID:   params.LocalizationID,
			Attributes: api.SubscriptionGroupLocalizationUpdateAttributes{
				Name:          params.Name,
				CustomAppName: params.CustomAppName,
			},
		},
	}

	resp, err := r.client.UpdateSubscriptionGroupLocalization(ctx, params.LocalizationID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update subscription group localization: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Subscription group localization updated:\n%s", formatSubscriptionGroupLocalization(resp.Data)), resp.Data), nil
}

func (r *Registry) handleDeleteSubscriptionGroupLocalization(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		LocalizationID string `json:"localization_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.LocalizationID == "" {
		return mcp.NewErrorResult("localization_id is required"), nil
	}

	if err := r.client.DeleteSubscriptionGroupLocalization(ctx, params.LocalizationID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete subscription group localization: %v", err)), nil
	}

	return mcp.NewSuccessResult("Subscription group localization deleted successfully"), nil
}

// commerceImageCreateParams holds the validated arguments of the two
// commerce image reservation tools.
type commerceImageCreateParams struct {
	VersionID string `json:"version_id"`
	FileName  string `json:"file_name"`
	FileSize  int    `json:"file_size"`
}

// parseCommerceImageCreate decodes and validates the shared image
// reservation arguments, returning a tool error result when a required
// argument is missing.
func parseCommerceImageCreate(args json.RawMessage) (commerceImageCreateParams, *mcp.ToolsCallResult) {
	var params commerceImageCreateParams
	if err := json.Unmarshal(args, &params); err != nil {
		return params, mcp.NewErrorResult(fmt.Sprintf("invalid arguments: %v", err))
	}

	if params.VersionID == "" {
		return params, mcp.NewErrorResult("version_id is required")
	}
	if params.FileName == "" {
		return params, mcp.NewErrorResult("file_name is required")
	}
	if params.FileSize <= 0 {
		return params, mcp.NewErrorResult("file_size is required")
	}

	return params, nil
}

// parseCommerceImageUpdate decodes the shared image commit arguments.
// The uploaded flag defaults to true because committing an upload is the
// only thing the endpoint is used for.
func parseCommerceImageUpdate(args json.RawMessage) (string, *bool, *mcp.ToolsCallResult) {
	var params struct {
		ImageID  string `json:"image_id"`
		Uploaded *bool  `json:"uploaded"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return "", nil, mcp.NewErrorResult(fmt.Sprintf("invalid arguments: %v", err))
	}

	if params.ImageID == "" {
		return "", nil, mcp.NewErrorResult("image_id is required")
	}

	uploaded := params.Uploaded
	if uploaded == nil {
		uploaded = mcp.BoolPtr(true)
	}

	return params.ImageID, uploaded, nil
}

// commerceVersionRow flattens the three version resources into a single
// shape so one formatter serves all of them.
type commerceVersionRow struct {
	ID         string
	Attributes api.CommerceVersionAttributes
}

// commerceImageRow flattens the two commerce image resources.
type commerceImageRow struct {
	ID         string
	Attributes api.CommerceImageAttributes
}

func formatCommerceVersions(versions []commerceVersionRow) string {
	if len(versions) == 0 {
		return "No versions found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d versions:\n\n", len(versions)))

	for _, v := range versions {
		sb.WriteString(formatCommerceVersion(v.ID, v.Attributes))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatCommerceVersion(id string, attrs api.CommerceVersionAttributes) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", id))
	if attrs.Version > 0 {
		sb.WriteString(fmt.Sprintf("Version: %d\n", attrs.Version))
	}
	if attrs.State != "" {
		sb.WriteString(fmt.Sprintf("State: %s\n", attrs.State))
	}
	return sb.String()
}

func formatCommerceLocalizations(locs []api.InAppPurchaseLocalization) string {
	if len(locs) == 0 {
		return "No localizations found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d localizations:\n\n", len(locs)))

	for _, loc := range locs {
		sb.WriteString(formatCommerceLocalization(loc))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatCommerceLocalization(loc api.InAppPurchaseLocalization) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", loc.ID))
	sb.WriteString(fmt.Sprintf("Locale: %s\n", loc.Attributes.Locale))
	sb.WriteString(fmt.Sprintf("Name: %s\n", loc.Attributes.Name))
	if loc.Attributes.Description != "" {
		sb.WriteString(fmt.Sprintf("Description: %s\n", loc.Attributes.Description))
	}
	if loc.Attributes.State != "" {
		sb.WriteString(fmt.Sprintf("State: %s\n", loc.Attributes.State))
	}
	return sb.String()
}

func formatSubscriptionGroupLocalizations(locs []api.SubscriptionGroupLocalization) string {
	if len(locs) == 0 {
		return "No subscription group localizations found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d subscription group localizations:\n\n", len(locs)))

	for _, loc := range locs {
		sb.WriteString(formatSubscriptionGroupLocalization(loc))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatSubscriptionGroupLocalization(loc api.SubscriptionGroupLocalization) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", loc.ID))
	sb.WriteString(fmt.Sprintf("Locale: %s\n", loc.Attributes.Locale))
	sb.WriteString(fmt.Sprintf("Name: %s\n", loc.Attributes.Name))
	if loc.Attributes.CustomAppName != "" {
		sb.WriteString(fmt.Sprintf("Custom App Name: %s\n", loc.Attributes.CustomAppName))
	}
	if loc.Attributes.State != "" {
		sb.WriteString(fmt.Sprintf("State: %s\n", loc.Attributes.State))
	}
	return sb.String()
}

func formatCommerceImages(images []commerceImageRow) string {
	if len(images) == 0 {
		return "No images found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d images:\n\n", len(images)))

	for _, img := range images {
		sb.WriteString(formatCommerceImage(img))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatCommerceImage(img commerceImageRow) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", img.ID))
	if img.Attributes.FileName != "" {
		sb.WriteString(fmt.Sprintf("File Name: %s\n", img.Attributes.FileName))
	}
	if img.Attributes.FileSize > 0 {
		sb.WriteString(fmt.Sprintf("File Size: %d bytes\n", img.Attributes.FileSize))
	}
	if img.Attributes.State != "" {
		sb.WriteString(fmt.Sprintf("State: %s\n", img.Attributes.State))
	}
	if len(img.Attributes.UploadOperations) > 0 {
		sb.WriteString(fmt.Sprintf("Upload Operations: %d\n", len(img.Attributes.UploadOperations)))
	}
	return sb.String()
}
