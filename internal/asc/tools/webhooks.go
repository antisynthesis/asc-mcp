package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerWebhookTools registers webhook management tools.
func (r *Registry) registerWebhookTools() {
	// List webhooks
	r.register(mcp.Tool{
		Name:        "list_webhooks",
		Description: "List the webhooks configured for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to list webhooks for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of webhooks to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
				"fields": {
					Type:        "object",
					Description: "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return, e.g. {\"webhooks\": [\"name\", \"url\", \"enabled\"]}.",
				},
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Webhooks",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListWebhooks)

	// Get webhook
	r.register(mcp.Tool{
		Name:        "get_webhook",
		Description: "Get details of a specific webhook",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"webhook_id": {
					Type:        "string",
					Description: "The webhook ID",
				},
			},
			Required: []string{"webhook_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Webhook",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetWebhook)

	// Create webhook
	r.register(mcp.Tool{
		Name:        "create_webhook",
		Description: "Create a webhook for an app to receive App Store Connect event notifications",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID to create the webhook for",
				},
				"name": {
					Type:        "string",
					Description: "A name for the webhook",
				},
				"url": {
					Type:        "string",
					Description: "The HTTPS endpoint URL that receives event payloads",
				},
				"secret": {
					Type:        "string",
					Description: "Shared secret used to sign webhook payloads",
				},
				"event_types": {
					Type:        "array",
					Description: "Event types to subscribe to, e.g. APP_STORE_VERSION_APP_VERSION_STATE_UPDATED, BUILD_UPLOAD_STATE_UPDATED, BETA_FEEDBACK_CRASH_SUBMISSION_CREATED",
					Items:       &mcp.Property{Type: "string"},
				},
				"enabled": {
					Type:        "boolean",
					Description: "Whether the webhook is enabled on creation (default true)",
				},
			},
			Required: []string{"app_id", "name", "url", "secret", "event_types"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Webhook",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateWebhook)

	// Update webhook
	r.register(mcp.Tool{
		Name:        "update_webhook",
		Description: "Update a webhook's configuration (name, URL, secret, event types, or enabled state)",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"webhook_id": {
					Type:        "string",
					Description: "The webhook ID",
				},
				"name": {
					Type:        "string",
					Description: "New name for the webhook",
				},
				"url": {
					Type:        "string",
					Description: "New HTTPS endpoint URL",
				},
				"secret": {
					Type:        "string",
					Description: "New shared secret for signing payloads",
				},
				"event_types": {
					Type:        "array",
					Description: "New list of event types to subscribe to (replaces the existing list)",
					Items:       &mcp.Property{Type: "string"},
				},
				"enabled": {
					Type:        "boolean",
					Description: "Enable or disable the webhook",
				},
			},
			Required: []string{"webhook_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Update Webhook",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleUpdateWebhook)

	// Delete webhook
	r.register(mcp.Tool{
		Name:        "delete_webhook",
		Description: "Delete a webhook",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"webhook_id": {
					Type:        "string",
					Description: "The webhook ID",
				},
			},
			Required: []string{"webhook_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Webhook",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteWebhook)

	// List webhook deliveries
	r.register(mcp.Tool{
		Name:        "list_webhook_deliveries",
		Description: "List delivery attempts for a webhook, including delivery state and endpoint responses",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"webhook_id": {
					Type:        "string",
					Description: "The webhook ID to list deliveries for",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of deliveries to return (default 50, max 200)",
				},
				"cursor": cursorProperty(),
				"filter": {
					Type:        "object",
					Description: "JSON:API filter map. Supported keys: deliveryState (SUCCEEDED, FAILED, PENDING), createdDateGreaterThanOrEqualTo, createdDateLessThan. Values are arrays, e.g. {\"deliveryState\": [\"FAILED\"]}.",
				},
				"fields": {
					Type:        "object",
					Description: "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return.",
				},
				"include": {
					Type:        "array",
					Description: "Related resource names to include in the response (supported: event).",
					Items:       &mcp.Property{Type: "string"},
				},
			},
			Required: []string{"webhook_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Webhook Deliveries",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListWebhookDeliveries)

	// Ping webhook
	r.register(mcp.Tool{
		Name:        "ping_webhook",
		Description: "Send a test ping event to a webhook endpoint to verify it is reachable",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"webhook_id": {
					Type:        "string",
					Description: "The webhook ID to ping",
				},
			},
			Required: []string{"webhook_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Ping Webhook",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handlePingWebhook)

	// Redeliver webhook delivery
	r.register(mcp.Tool{
		Name:        "redeliver_webhook_delivery",
		Description: "Request redelivery of a webhook event using a previous delivery as the template",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"delivery_id": {
					Type:        "string",
					Description: "The webhook delivery ID to use as the redelivery template",
				},
			},
			Required: []string{"delivery_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Redeliver Webhook Delivery",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleRedeliverWebhookDelivery)
}

func (r *Registry) handleListWebhooks(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
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

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.WebhooksResponse, error) {
		return r.client.ListWebhooks(ctx, params.AppID, listOpts(limit, nil, nil, params.Fields, nil))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list webhooks: %v", err)), nil
	}

	return newListResult(formatWebhooks(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetWebhook(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		WebhookID string `json:"webhook_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.WebhookID == "" {
		return mcp.NewErrorResult("webhook_id is required"), nil
	}

	resp, err := r.client.GetWebhook(ctx, params.WebhookID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get webhook: %v", err)), nil
	}

	return newDataResult(formatWebhook(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateWebhook(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID      string   `json:"app_id"`
		Name       string   `json:"name"`
		URL        string   `json:"url"`
		Secret     string   `json:"secret"`
		EventTypes []string `json:"event_types"`
		Enabled    *bool    `json:"enabled"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return mcp.NewErrorResult("app_id is required"), nil
	}
	if params.Name == "" {
		return mcp.NewErrorResult("name is required"), nil
	}
	if params.URL == "" {
		return mcp.NewErrorResult("url is required"), nil
	}
	if params.Secret == "" {
		return mcp.NewErrorResult("secret is required"), nil
	}
	if len(params.EventTypes) == 0 {
		return mcp.NewErrorResult("event_types is required"), nil
	}

	enabled := true
	if params.Enabled != nil {
		enabled = *params.Enabled
	}

	req := &api.WebhookCreateRequest{
		Data: api.WebhookCreateData{
			Type: "webhooks",
			Attributes: api.WebhookCreateAttributes{
				Enabled:    enabled,
				EventTypes: params.EventTypes,
				Name:       params.Name,
				Secret:     params.Secret,
				URL:        params.URL,
			},
			Relationships: api.WebhookCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "apps",
						ID:   params.AppID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateWebhook(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create webhook: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created webhook:\n%s", formatWebhook(resp.Data)), resp.Data), nil
}

func (r *Registry) handleUpdateWebhook(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		WebhookID  string   `json:"webhook_id"`
		Name       string   `json:"name"`
		URL        string   `json:"url"`
		Secret     string   `json:"secret"`
		EventTypes []string `json:"event_types"`
		Enabled    *bool    `json:"enabled"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.WebhookID == "" {
		return mcp.NewErrorResult("webhook_id is required"), nil
	}

	req := &api.WebhookUpdateRequest{
		Data: api.WebhookUpdateData{
			Type: "webhooks",
			ID:   params.WebhookID,
			Attributes: &api.WebhookUpdateAttributes{
				Enabled:    params.Enabled,
				EventTypes: params.EventTypes,
				Name:       params.Name,
				Secret:     params.Secret,
				URL:        params.URL,
			},
		},
	}

	resp, err := r.client.UpdateWebhook(ctx, params.WebhookID, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to update webhook: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Webhook updated:\n%s", formatWebhook(resp.Data)), resp.Data), nil
}

func (r *Registry) handleDeleteWebhook(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		WebhookID string `json:"webhook_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.WebhookID == "" {
		return mcp.NewErrorResult("webhook_id is required"), nil
	}

	if err := r.client.DeleteWebhook(ctx, params.WebhookID); err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete webhook: %v", err)), nil
	}

	return mcp.NewSuccessResult("Webhook deleted successfully"), nil
}

func (r *Registry) handleListWebhookDeliveries(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		WebhookID string              `json:"webhook_id"`
		Limit     int                 `json:"limit"`
		Cursor    string              `json:"cursor"`
		Filter    map[string][]string `json:"filter"`
		Fields    map[string][]string `json:"fields"`
		Include   []string            `json:"include"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.WebhookID == "" {
		return mcp.NewErrorResult("webhook_id is required"), nil
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.WebhookDeliveriesResponse, error) {
		return r.client.ListWebhookDeliveries(ctx, params.WebhookID, listOpts(limit, params.Filter, nil, params.Fields, params.Include))
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list webhook deliveries: %v", err)), nil
	}

	return newListResult(formatWebhookDeliveries(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handlePingWebhook(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		WebhookID string `json:"webhook_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.WebhookID == "" {
		return mcp.NewErrorResult("webhook_id is required"), nil
	}

	req := &api.WebhookPingCreateRequest{
		Data: api.WebhookPingCreateData{
			Type: "webhookPings",
			Relationships: api.WebhookPingCreateRelationships{
				Webhook: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "webhooks",
						ID:   params.WebhookID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateWebhookPing(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to ping webhook: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Ping event sent: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleRedeliverWebhookDelivery(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		DeliveryID string `json:"delivery_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.DeliveryID == "" {
		return mcp.NewErrorResult("delivery_id is required"), nil
	}

	req := &api.WebhookDeliveryCreateRequest{
		Data: api.WebhookDeliveryCreateData{
			Type: "webhookDeliveries",
			Relationships: api.WebhookDeliveryCreateRelationships{
				Template: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "webhookDeliveries",
						ID:   params.DeliveryID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateWebhookRedelivery(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to request redelivery: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Redelivery requested:\n%s", formatWebhookDelivery(resp.Data)), resp.Data), nil
}

func formatWebhooks(webhooks []api.Webhook) string {
	if len(webhooks) == 0 {
		return "No webhooks found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d webhooks:\n\n", len(webhooks)))

	for _, webhook := range webhooks {
		sb.WriteString(formatWebhook(webhook))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatWebhook(webhook api.Webhook) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Webhook ID: %s\n", webhook.ID))
	if webhook.Attributes.Name != "" {
		sb.WriteString(fmt.Sprintf("Name: %s\n", webhook.Attributes.Name))
	}
	if webhook.Attributes.URL != "" {
		sb.WriteString(fmt.Sprintf("URL: %s\n", webhook.Attributes.URL))
	}
	sb.WriteString(fmt.Sprintf("Enabled: %t\n", webhook.Attributes.Enabled))
	if len(webhook.Attributes.EventTypes) > 0 {
		sb.WriteString(fmt.Sprintf("Event Types: %s\n", strings.Join(webhook.Attributes.EventTypes, ", ")))
	}
	return sb.String()
}

func formatWebhookDeliveries(deliveries []api.WebhookDelivery) string {
	if len(deliveries) == 0 {
		return "No webhook deliveries found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d webhook deliveries:\n\n", len(deliveries)))

	for _, delivery := range deliveries {
		sb.WriteString(formatWebhookDelivery(delivery))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatWebhookDelivery(delivery api.WebhookDelivery) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Delivery ID: %s\n", delivery.ID))
	if delivery.Attributes.DeliveryState != "" {
		sb.WriteString(fmt.Sprintf("State: %s\n", delivery.Attributes.DeliveryState))
	}
	if delivery.Attributes.Redelivery {
		sb.WriteString("Redelivery: Yes\n")
	}
	if delivery.Attributes.CreatedDate != nil {
		sb.WriteString(fmt.Sprintf("Created: %s\n", delivery.Attributes.CreatedDate.Format("2006-01-02 15:04:05")))
	}
	if delivery.Attributes.SentDate != nil {
		sb.WriteString(fmt.Sprintf("Sent: %s\n", delivery.Attributes.SentDate.Format("2006-01-02 15:04:05")))
	}
	if delivery.Attributes.ErrorMessage != "" {
		sb.WriteString(fmt.Sprintf("Error: %s\n", delivery.Attributes.ErrorMessage))
	}
	if delivery.Attributes.Response != nil && delivery.Attributes.Response.HTTPStatusCode != 0 {
		sb.WriteString(fmt.Sprintf("Response Status: %d\n", delivery.Attributes.Response.HTTPStatusCode))
	}
	return sb.String()
}
