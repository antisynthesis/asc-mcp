package api

import (
	"context"
	"encoding/json"
	"fmt"
	"net/url"
	"time"
)

// Webhook API methods (App Store Connect API 4.0+)

// Webhook types

// WebhooksResponse represents a list of webhooks.
type WebhooksResponse struct {
	Data     []Webhook          `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// WebhookResponse represents a single webhook.
type WebhookResponse struct {
	Data     Webhook `json:"data"`
	Included []any   `json:"included,omitempty"`
}

// Webhook represents a webhook configured for an app.
type Webhook struct {
	Type       string            `json:"type"`
	ID         string            `json:"id"`
	Attributes WebhookAttributes `json:"attributes"`
}

// WebhookAttributes contains webhook attributes.
type WebhookAttributes struct {
	Enabled    bool     `json:"enabled,omitempty"`
	EventTypes []string `json:"eventTypes,omitempty"`
	Name       string   `json:"name,omitempty"`
	URL        string   `json:"url,omitempty"`
}

// WebhookCreateRequest represents a request to create a webhook.
type WebhookCreateRequest struct {
	Data WebhookCreateData `json:"data"`
}

// WebhookCreateData contains the data for creating a webhook.
type WebhookCreateData struct {
	Type          string                     `json:"type"`
	Attributes    WebhookCreateAttributes    `json:"attributes"`
	Relationships WebhookCreateRelationships `json:"relationships"`
}

// WebhookCreateAttributes contains attributes for creating a webhook.
// All fields are required by the API.
type WebhookCreateAttributes struct {
	Enabled    bool     `json:"enabled"`
	EventTypes []string `json:"eventTypes"`
	Name       string   `json:"name"`
	Secret     string   `json:"secret"`
	URL        string   `json:"url"`
}

// WebhookCreateRelationships contains relationships for creating a webhook.
type WebhookCreateRelationships struct {
	App RelationshipData `json:"app"`
}

// WebhookUpdateRequest represents a request to update a webhook.
type WebhookUpdateRequest struct {
	Data WebhookUpdateData `json:"data"`
}

// WebhookUpdateData contains the data for updating a webhook.
type WebhookUpdateData struct {
	Type       string                   `json:"type"`
	ID         string                   `json:"id"`
	Attributes *WebhookUpdateAttributes `json:"attributes,omitempty"`
}

// WebhookUpdateAttributes contains attributes for updating a webhook.
type WebhookUpdateAttributes struct {
	Enabled    *bool    `json:"enabled,omitempty"`
	EventTypes []string `json:"eventTypes,omitempty"`
	Name       string   `json:"name,omitempty"`
	Secret     string   `json:"secret,omitempty"`
	URL        string   `json:"url,omitempty"`
}

// Webhook delivery types

// WebhookDeliveriesResponse represents a list of webhook deliveries.
type WebhookDeliveriesResponse struct {
	Data     []WebhookDelivery  `json:"data"`
	Links    PagedDocumentLinks `json:"links"`
	Meta     *PagingInformation `json:"meta,omitempty"`
	Included []any              `json:"included,omitempty"`
}

// WebhookDeliveryResponse represents a single webhook delivery.
type WebhookDeliveryResponse struct {
	Data WebhookDelivery `json:"data"`
}

// WebhookDelivery represents a delivery attempt for a webhook event.
type WebhookDelivery struct {
	Type       string                    `json:"type"`
	ID         string                    `json:"id"`
	Attributes WebhookDeliveryAttributes `json:"attributes"`
}

// WebhookDeliveryAttributes contains webhook delivery attributes.
type WebhookDeliveryAttributes struct {
	CreatedDate   *time.Time                   `json:"createdDate,omitempty"`
	DeliveryState string                       `json:"deliveryState,omitempty"`
	ErrorMessage  string                       `json:"errorMessage,omitempty"`
	Redelivery    bool                         `json:"redelivery,omitempty"`
	SentDate      *time.Time                   `json:"sentDate,omitempty"`
	Request       *WebhookDeliveryRequestInfo  `json:"request,omitempty"`
	Response      *WebhookDeliveryResponseInfo `json:"response,omitempty"`
}

// WebhookDeliveryRequestInfo describes the outbound request of a delivery.
type WebhookDeliveryRequestInfo struct {
	URL string `json:"url,omitempty"`
}

// WebhookDeliveryResponseInfo describes the endpoint's response to a delivery.
type WebhookDeliveryResponseInfo struct {
	HTTPStatusCode int    `json:"httpStatusCode,omitempty"`
	Body           string `json:"body,omitempty"`
}

// WebhookDeliveryCreateRequest represents a request to redeliver a webhook
// event using a previous delivery as the template.
type WebhookDeliveryCreateRequest struct {
	Data WebhookDeliveryCreateData `json:"data"`
}

// WebhookDeliveryCreateData contains the data for creating a redelivery.
type WebhookDeliveryCreateData struct {
	Type          string                             `json:"type"`
	Relationships WebhookDeliveryCreateRelationships `json:"relationships"`
}

// WebhookDeliveryCreateRelationships contains relationships for a redelivery.
type WebhookDeliveryCreateRelationships struct {
	Template RelationshipData `json:"template"`
}

// Webhook ping types

// WebhookPingResponse represents a single webhook ping.
type WebhookPingResponse struct {
	Data WebhookPing `json:"data"`
}

// WebhookPing represents a test event sent to a webhook endpoint.
type WebhookPing struct {
	Type string `json:"type"`
	ID   string `json:"id"`
}

// WebhookPingCreateRequest represents a request to send a test ping event.
type WebhookPingCreateRequest struct {
	Data WebhookPingCreateData `json:"data"`
}

// WebhookPingCreateData contains the data for creating a webhook ping.
type WebhookPingCreateData struct {
	Type          string                         `json:"type"`
	Relationships WebhookPingCreateRelationships `json:"relationships"`
}

// WebhookPingCreateRelationships contains relationships for a webhook ping.
type WebhookPingCreateRelationships struct {
	Webhook RelationshipData `json:"webhook"`
}

// ListWebhooks returns the webhooks configured for an app.
func (c *Client) ListWebhooks(ctx context.Context, appID string, opts *ListOptions) (*WebhooksResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/apps/"+url.PathEscape(appID)+"/webhooks", query)
	if err != nil {
		return nil, err
	}

	var resp WebhooksResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetWebhook returns a single webhook.
func (c *Client) GetWebhook(ctx context.Context, webhookID string) (*WebhookResponse, error) {
	data, err := c.Get(ctx, "/v1/webhooks/"+url.PathEscape(webhookID), nil)
	if err != nil {
		return nil, err
	}

	var resp WebhookResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateWebhook creates a webhook for an app.
func (c *Client) CreateWebhook(ctx context.Context, req *WebhookCreateRequest) (*WebhookResponse, error) {
	data, err := c.Post(ctx, "/v1/webhooks", req)
	if err != nil {
		return nil, err
	}

	var resp WebhookResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateWebhook updates a webhook's configuration.
func (c *Client) UpdateWebhook(ctx context.Context, webhookID string, req *WebhookUpdateRequest) (*WebhookResponse, error) {
	data, err := c.Patch(ctx, "/v1/webhooks/"+url.PathEscape(webhookID), req)
	if err != nil {
		return nil, err
	}

	var resp WebhookResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteWebhook deletes a webhook.
func (c *Client) DeleteWebhook(ctx context.Context, webhookID string) error {
	return c.Delete(ctx, "/v1/webhooks/"+url.PathEscape(webhookID))
}

// ListWebhookDeliveries returns delivery attempts for a webhook.
func (c *Client) ListWebhookDeliveries(ctx context.Context, webhookID string, opts *ListOptions) (*WebhookDeliveriesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/webhooks/"+url.PathEscape(webhookID)+"/deliveries", query)
	if err != nil {
		return nil, err
	}

	var resp WebhookDeliveriesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateWebhookRedelivery requests redelivery of a webhook event using a
// previous delivery as the template.
func (c *Client) CreateWebhookRedelivery(ctx context.Context, req *WebhookDeliveryCreateRequest) (*WebhookDeliveryResponse, error) {
	data, err := c.Post(ctx, "/v1/webhookDeliveries", req)
	if err != nil {
		return nil, err
	}

	var resp WebhookDeliveryResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateWebhookPing sends a test ping event to a webhook endpoint.
func (c *Client) CreateWebhookPing(ctx context.Context, req *WebhookPingCreateRequest) (*WebhookPingResponse, error) {
	data, err := c.Post(ctx, "/v1/webhookPings", req)
	if err != nil {
		return nil, err
	}

	var resp WebhookPingResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}
