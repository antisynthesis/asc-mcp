package api

import (
	"context"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"net/url"
	"testing"
	"time"
)

// mockTokenProvider creates a mock token provider for testing.
func mockTokenProvider(t *testing.T) *TokenProvider {
	t.Helper()

	privateKey, err := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		t.Fatalf("failed to generate key: %v", err)
	}

	return &TokenProvider{
		issuerID:   "test-issuer",
		keyID:      "TESTKEY123",
		privateKey: privateKey,
	}
}

// newTestClient creates a test client with a mock server.
func newTestClient(t *testing.T, handler http.Handler) (*Client, *httptest.Server) {
	t.Helper()

	server := httptest.NewServer(handler)

	client := &Client{
		httpClient:    &http.Client{Timeout: 10 * time.Second},
		tokenProvider: mockTokenProvider(t),
		baseURL:       server.URL,
	}

	return client, server
}

func TestClient_Get(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Verify method
		if r.Method != http.MethodGet {
			t.Errorf("expected GET, got %s", r.Method)
		}

		// Verify authorization header
		auth := r.Header.Get("Authorization")
		if auth == "" {
			t.Error("missing Authorization header")
		}
		if len(auth) < 8 || auth[:7] != "Bearer " {
			t.Error("Authorization header should start with 'Bearer '")
		}

		// Verify content type
		if r.Header.Get("Content-Type") != "application/json" {
			t.Errorf("Content-Type = %q, want application/json", r.Header.Get("Content-Type"))
		}

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		json.NewEncoder(w).Encode(map[string]string{"status": "ok"})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	ctx := context.Background()
	data, err := client.Get(ctx, "/test", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var resp map[string]string
	if err := json.Unmarshal(data, &resp); err != nil {
		t.Fatalf("failed to unmarshal response: %v", err)
	}

	if resp["status"] != "ok" {
		t.Errorf("status = %q, want ok", resp["status"])
	}
}

func TestClient_Get_WithQuery(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Verify query parameters
		if r.URL.Query().Get("limit") != "10" {
			t.Errorf("limit = %q, want 10", r.URL.Query().Get("limit"))
		}
		if r.URL.Query().Get("filter") != "test" {
			t.Errorf("filter = %q, want test", r.URL.Query().Get("filter"))
		}

		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{}`))
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	ctx := context.Background()
	query := url.Values{}
	query.Set("limit", "10")
	query.Set("filter", "test")

	_, err := client.Get(ctx, "/test", query)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestClient_Post(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			t.Errorf("expected POST, got %s", r.Method)
		}

		var body map[string]string
		if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
			t.Fatalf("failed to decode body: %v", err)
		}

		if body["name"] != "test" {
			t.Errorf("name = %q, want test", body["name"])
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(map[string]string{"id": "123"})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	ctx := context.Background()
	data, err := client.Post(ctx, "/test", map[string]string{"name": "test"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var resp map[string]string
	if err := json.Unmarshal(data, &resp); err != nil {
		t.Fatalf("failed to unmarshal response: %v", err)
	}

	if resp["id"] != "123" {
		t.Errorf("id = %q, want 123", resp["id"])
	}
}

func TestClient_Delete(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodDelete {
			t.Errorf("expected DELETE, got %s", r.Method)
		}

		w.WriteHeader(http.StatusNoContent)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	ctx := context.Background()
	err := client.Delete(ctx, "/test/123")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestClient_ErrorResponse(t *testing.T) {
	tests := []struct {
		name        string
		statusCode  int
		body        string
		errContains string
	}{
		{
			name:       "API error with details",
			statusCode: http.StatusBadRequest,
			body: `{
				"errors": [
					{"title": "Invalid Parameter", "detail": "The value is not valid"}
				]
			}`,
			errContains: "Invalid Parameter: The value is not valid",
		},
		{
			name:        "API error without details",
			statusCode:  http.StatusInternalServerError,
			body:        `{"message": "internal error"}`,
			errContains: "API error (500)",
		},
		{
			name:        "not found",
			statusCode:  http.StatusNotFound,
			body:        `{}`,
			errContains: "API error (404)",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				w.WriteHeader(tt.statusCode)
				w.Write([]byte(tt.body))
			})

			client, server := newTestClient(t, handler)
			defer server.Close()

			ctx := context.Background()
			_, err := client.Get(ctx, "/test", nil)

			if err == nil {
				t.Fatal("expected error, got nil")
			}

			if !contains(err.Error(), tt.errContains) {
				t.Errorf("error %q does not contain %q", err.Error(), tt.errContains)
			}
		})
	}
}

func TestClient_ContextCancellation(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		time.Sleep(100 * time.Millisecond)
		w.WriteHeader(http.StatusOK)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Millisecond)
	defer cancel()

	_, err := client.Get(ctx, "/test", nil)
	if err == nil {
		t.Fatal("expected error due to context cancellation")
	}
}

func TestClient_ListApps(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/apps" {
			t.Errorf("path = %q, want /v1/apps", r.URL.Path)
		}

		resp := AppsResponse{
			Data: []App{
				{
					Type: "apps",
					ID:   "123",
					Attributes: AppAttributes{
						Name:          "Test App",
						BundleID:      "com.test.app",
						SKU:           "TEST123",
						PrimaryLocale: "en-US",
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	ctx := context.Background()
	resp, err := client.ListApps(ctx, 50)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 app, got %d", len(resp.Data))
	}

	if resp.Data[0].Attributes.Name != "Test App" {
		t.Errorf("name = %q, want Test App", resp.Data[0].Attributes.Name)
	}
}

func TestClient_GetApp(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/apps/123" {
			t.Errorf("path = %q, want /v1/apps/123", r.URL.Path)
		}

		resp := AppResponse{
			Data: App{
				Type: "apps",
				ID:   "123",
				Attributes: AppAttributes{
					Name:     "Test App",
					BundleID: "com.test.app",
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	ctx := context.Background()
	resp, err := client.GetApp(ctx, "123")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "123" {
		t.Errorf("id = %q, want 123", resp.Data.ID)
	}
}

func TestClient_ListBuilds(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/builds" {
			t.Errorf("path = %q, want /v1/builds", r.URL.Path)
		}

		// Check filter parameter
		if r.URL.Query().Get("filter[app]") != "app123" {
			t.Errorf("filter[app] = %q, want app123", r.URL.Query().Get("filter[app]"))
		}

		resp := BuildsResponse{
			Data: []Build{
				{
					Type: "builds",
					ID:   "build1",
					Attributes: BuildAttributes{
						Version:         "1.0.0",
						ProcessingState: "VALID",
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	ctx := context.Background()
	resp, err := client.ListBuilds(ctx, "app123", 50)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 build, got %d", len(resp.Data))
	}
}

func TestClient_ListBetaGroups(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		resp := BetaGroupsResponse{
			Data: []BetaGroup{
				{
					Type: "betaGroups",
					ID:   "group1",
					Attributes: BetaGroupAttributes{
						Name:              "External Testers",
						PublicLinkEnabled: true,
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	ctx := context.Background()
	resp, err := client.ListBetaGroups(ctx, "", 50)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 beta group, got %d", len(resp.Data))
	}

	if resp.Data[0].Attributes.Name != "External Testers" {
		t.Errorf("name = %q, want External Testers", resp.Data[0].Attributes.Name)
	}
}

func TestClient_ListDevices(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		resp := DevicesResponse{
			Data: []Device{
				{
					Type: "devices",
					ID:   "device1",
					Attributes: DeviceAttributes{
						Name:     "iPhone 15",
						UDID:     "00000000-0000-0000-0000-000000000001",
						Platform: "IOS",
						Status:   "ENABLED",
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	ctx := context.Background()
	resp, err := client.ListDevices(ctx, 50)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 device, got %d", len(resp.Data))
	}
}

// Helper function
func contains(s, substr string) bool {
	return len(s) >= len(substr) && (s == substr || len(s) > 0 && containsHelper(s, substr))
}

func containsHelper(s, substr string) bool {
	for i := 0; i <= len(s)-len(substr); i++ {
		if s[i:i+len(substr)] == substr {
			return true
		}
	}
	return false
}

// Benchmarks

func BenchmarkClient_Get(b *testing.B) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte(`{"data": []}`))
	})

	server := httptest.NewServer(handler)
	defer server.Close()

	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	client := &Client{
		httpClient:    &http.Client{Timeout: 10 * time.Second},
		tokenProvider: &TokenProvider{issuerID: "test", keyID: "TEST", privateKey: privateKey},
		baseURL:       server.URL,
	}

	ctx := context.Background()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, err := client.Get(ctx, "/test", nil)
		if err != nil {
			b.Fatalf("request failed: %v", err)
		}
	}
}

func BenchmarkClient_Post(b *testing.B) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusCreated)
		w.Write([]byte(`{"data": {"id": "123"}}`))
	})

	server := httptest.NewServer(handler)
	defer server.Close()

	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	client := &Client{
		httpClient:    &http.Client{Timeout: 10 * time.Second},
		tokenProvider: &TokenProvider{issuerID: "test", keyID: "TEST", privateKey: privateKey},
		baseURL:       server.URL,
	}

	ctx := context.Background()
	body := map[string]string{"name": "test"}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, err := client.Post(ctx, "/test", body)
		if err != nil {
			b.Fatalf("request failed: %v", err)
		}
	}
}
