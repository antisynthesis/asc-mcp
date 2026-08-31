package api

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"testing"
)

func TestClient_ListWebhooks(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/apps/123/webhooks" {
			t.Errorf("path = %q, want /v1/apps/123/webhooks", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}

		resp := WebhooksResponse{
			Data: []Webhook{
				{
					Type: "webhooks",
					ID:   "hook-1",
					Attributes: WebhookAttributes{
						Enabled:    true,
						EventTypes: []string{"BUILD_UPLOAD_STATE_UPDATED"},
						Name:       "CI hook",
						URL:        "https://example.com/hook",
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.ListWebhooks(context.Background(), "123", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 webhook, got %d", len(resp.Data))
	}

	if resp.Data[0].Attributes.Name != "CI hook" {
		t.Errorf("name = %q, want CI hook", resp.Data[0].Attributes.Name)
	}
}

func TestClient_CreateWebhook(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/webhooks" {
			t.Errorf("path = %q, want /v1/webhooks", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req WebhookCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Type != "webhooks" {
			t.Errorf("type = %q, want webhooks", req.Data.Type)
		}
		if req.Data.Attributes.Secret != "s3cret" {
			t.Errorf("secret = %q, want s3cret", req.Data.Attributes.Secret)
		}
		if req.Data.Relationships.App.Data.ID != "123" {
			t.Errorf("app id = %q, want 123", req.Data.Relationships.App.Data.ID)
		}

		resp := WebhookResponse{
			Data: Webhook{
				Type: "webhooks",
				ID:   "hook-1",
				Attributes: WebhookAttributes{
					Enabled: true,
					Name:    "CI hook",
					URL:     "https://example.com/hook",
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &WebhookCreateRequest{
		Data: WebhookCreateData{
			Type: "webhooks",
			Attributes: WebhookCreateAttributes{
				Enabled:    true,
				EventTypes: []string{"BUILD_UPLOAD_STATE_UPDATED"},
				Name:       "CI hook",
				Secret:     "s3cret",
				URL:        "https://example.com/hook",
			},
			Relationships: WebhookCreateRelationships{
				App: RelationshipData{
					Data: ResourceIdentifier{Type: "apps", ID: "123"},
				},
			},
		},
	}

	resp, err := client.CreateWebhook(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "hook-1" {
		t.Errorf("id = %q, want hook-1", resp.Data.ID)
	}
}

func TestClient_UpdateWebhook(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/webhooks/hook-1" {
			t.Errorf("path = %q, want /v1/webhooks/hook-1", r.URL.Path)
		}
		if r.Method != http.MethodPatch {
			t.Errorf("method = %q, want PATCH", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req WebhookUpdateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.ID != "hook-1" {
			t.Errorf("id = %q, want hook-1", req.Data.ID)
		}
		if req.Data.Attributes == nil || req.Data.Attributes.Enabled == nil || *req.Data.Attributes.Enabled {
			t.Error("expected enabled=false in update attributes")
		}

		resp := WebhookResponse{
			Data: Webhook{
				Type:       "webhooks",
				ID:         "hook-1",
				Attributes: WebhookAttributes{Enabled: false},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &WebhookUpdateRequest{
		Data: WebhookUpdateData{
			Type:       "webhooks",
			ID:         "hook-1",
			Attributes: &WebhookUpdateAttributes{Enabled: boolPtrTest(false)},
		},
	}

	resp, err := client.UpdateWebhook(context.Background(), "hook-1", req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.Attributes.Enabled {
		t.Error("expected webhook to be disabled")
	}
}

func TestClient_DeleteWebhook(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/webhooks/hook-1" {
			t.Errorf("path = %q, want /v1/webhooks/hook-1", r.URL.Path)
		}
		if r.Method != http.MethodDelete {
			t.Errorf("method = %q, want DELETE", r.Method)
		}
		w.WriteHeader(http.StatusNoContent)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	if err := client.DeleteWebhook(context.Background(), "hook-1"); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestClient_ListWebhookDeliveries(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/webhooks/hook-1/deliveries" {
			t.Errorf("path = %q, want /v1/webhooks/hook-1/deliveries", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}
		if got := r.URL.Query().Get("filter[deliveryState]"); got != "FAILED" {
			t.Errorf("filter[deliveryState] = %q, want FAILED", got)
		}

		resp := WebhookDeliveriesResponse{
			Data: []WebhookDelivery{
				{
					Type: "webhookDeliveries",
					ID:   "delivery-1",
					Attributes: WebhookDeliveryAttributes{
						DeliveryState: "FAILED",
						ErrorMessage:  "connection refused",
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	opts := &ListOptions{Filter: map[string][]string{"deliveryState": {"FAILED"}}}
	resp, err := client.ListWebhookDeliveries(context.Background(), "hook-1", opts)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 delivery, got %d", len(resp.Data))
	}

	if resp.Data[0].Attributes.DeliveryState != "FAILED" {
		t.Errorf("deliveryState = %q, want FAILED", resp.Data[0].Attributes.DeliveryState)
	}
}

func TestClient_CreateWebhookRedelivery(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/webhookDeliveries" {
			t.Errorf("path = %q, want /v1/webhookDeliveries", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req WebhookDeliveryCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Relationships.Template.Data.ID != "delivery-1" {
			t.Errorf("template id = %q, want delivery-1", req.Data.Relationships.Template.Data.ID)
		}

		resp := WebhookDeliveryResponse{
			Data: WebhookDelivery{
				Type:       "webhookDeliveries",
				ID:         "delivery-2",
				Attributes: WebhookDeliveryAttributes{Redelivery: true},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &WebhookDeliveryCreateRequest{
		Data: WebhookDeliveryCreateData{
			Type: "webhookDeliveries",
			Relationships: WebhookDeliveryCreateRelationships{
				Template: RelationshipData{
					Data: ResourceIdentifier{Type: "webhookDeliveries", ID: "delivery-1"},
				},
			},
		},
	}

	resp, err := client.CreateWebhookRedelivery(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if !resp.Data.Attributes.Redelivery {
		t.Error("expected redelivery to be true")
	}
}

func TestClient_CreateWebhookPing(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/webhookPings" {
			t.Errorf("path = %q, want /v1/webhookPings", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req WebhookPingCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Relationships.Webhook.Data.ID != "hook-1" {
			t.Errorf("webhook id = %q, want hook-1", req.Data.Relationships.Webhook.Data.ID)
		}

		resp := WebhookPingResponse{
			Data: WebhookPing{Type: "webhookPings", ID: "ping-1"},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &WebhookPingCreateRequest{
		Data: WebhookPingCreateData{
			Type: "webhookPings",
			Relationships: WebhookPingCreateRelationships{
				Webhook: RelationshipData{
					Data: ResourceIdentifier{Type: "webhooks", ID: "hook-1"},
				},
			},
		},
	}

	resp, err := client.CreateWebhookPing(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "ping-1" {
		t.Errorf("id = %q, want ping-1", resp.Data.ID)
	}
}
