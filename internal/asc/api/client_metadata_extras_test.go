package api

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"testing"
)

// TestClient_MetadataExtras_ListEndpoints exercises the read-only list
// endpoints added in App Store Connect API 4.0-4.2 with a table of
// path expectations.
func TestClient_MetadataExtras_ListEndpoints(t *testing.T) {
	tests := []struct {
		name     string
		wantPath string
		call     func(ctx context.Context, c *Client) (int, error)
		payload  any
	}{
		{
			name:     "ListAccessibilityDeclarations",
			wantPath: "/v1/apps/123/accessibilityDeclarations",
			payload: AccessibilityDeclarationsResponse{
				Data: []AccessibilityDeclaration{{Type: "accessibilityDeclarations", ID: "decl-1"}},
			},
			call: func(ctx context.Context, c *Client) (int, error) {
				resp, err := c.ListAccessibilityDeclarations(ctx, "123", nil)
				if err != nil {
					return 0, err
				}
				return len(resp.Data), nil
			},
		},
		{
			name:     "ListCustomerReviewSummarizations",
			wantPath: "/v1/apps/123/customerReviewSummarizations",
			payload: CustomerReviewSummarizationsResponse{
				Data: []CustomerReviewSummarization{{Type: "customerReviewSummarizations", ID: "sum-1"}},
			},
			call: func(ctx context.Context, c *Client) (int, error) {
				resp, err := c.ListCustomerReviewSummarizations(ctx, "123", nil)
				if err != nil {
					return 0, err
				}
				return len(resp.Data), nil
			},
		},
		{
			name:     "ListAppTags",
			wantPath: "/v1/apps/123/appTags",
			payload: AppTagsResponse{
				Data: []AppTag{{Type: "appTags", ID: "tag-1"}},
			},
			call: func(ctx context.Context, c *Client) (int, error) {
				resp, err := c.ListAppTags(ctx, "123", nil)
				if err != nil {
					return 0, err
				}
				return len(resp.Data), nil
			},
		},
		{
			name:     "ListAppTagTerritories",
			wantPath: "/v1/appTags/tag-1/territories",
			payload: TerritoriesResponse{
				Data: []Territory{{Type: "territories", ID: "USA"}},
			},
			call: func(ctx context.Context, c *Client) (int, error) {
				resp, err := c.ListAppTagTerritories(ctx, "tag-1", nil)
				if err != nil {
					return 0, err
				}
				return len(resp.Data), nil
			},
		},
		{
			name:     "ListTerritoryAgeRatings",
			wantPath: "/v1/appInfos/info-1/territoryAgeRatings",
			payload: TerritoryAgeRatingsResponse{
				Data: []TerritoryAgeRating{{Type: "territoryAgeRatings", ID: "rating-1"}},
			},
			call: func(ctx context.Context, c *Client) (int, error) {
				resp, err := c.ListTerritoryAgeRatings(ctx, "info-1", nil)
				if err != nil {
					return 0, err
				}
				return len(resp.Data), nil
			},
		},
		{
			name:     "ListAndroidToIosAppMappingDetails",
			wantPath: "/v1/apps/123/androidToIosAppMappingDetails",
			payload: AndroidToIosAppMappingDetailsResponse{
				Data: []AndroidToIosAppMappingDetail{{Type: "androidToIosAppMappingDetails", ID: "map-1"}},
			},
			call: func(ctx context.Context, c *Client) (int, error) {
				resp, err := c.ListAndroidToIosAppMappingDetails(ctx, "123", nil)
				if err != nil {
					return 0, err
				}
				return len(resp.Data), nil
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				if r.URL.Path != tt.wantPath {
					t.Errorf("path = %q, want %q", r.URL.Path, tt.wantPath)
				}
				if r.Method != http.MethodGet {
					t.Errorf("method = %q, want GET", r.Method)
				}
				json.NewEncoder(w).Encode(tt.payload)
			})

			client, server := newTestClient(t, handler)
			defer server.Close()

			count, err := tt.call(context.Background(), client)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if count != 1 {
				t.Errorf("expected 1 resource, got %d", count)
			}
		})
	}
}

func TestClient_CreateAccessibilityDeclaration(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/accessibilityDeclarations" {
			t.Errorf("path = %q, want /v1/accessibilityDeclarations", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req AccessibilityDeclarationCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Type != "accessibilityDeclarations" {
			t.Errorf("type = %q, want accessibilityDeclarations", req.Data.Type)
		}
		if req.Data.Attributes.DeviceFamily != "IPHONE" {
			t.Errorf("deviceFamily = %q, want IPHONE", req.Data.Attributes.DeviceFamily)
		}
		if req.Data.Relationships.App.Data.ID != "123" {
			t.Errorf("app id = %q, want 123", req.Data.Relationships.App.Data.ID)
		}

		resp := AccessibilityDeclarationResponse{
			Data: AccessibilityDeclaration{
				Type: "accessibilityDeclarations",
				ID:   "decl-1",
				Attributes: AccessibilityDeclarationAttributes{
					DeviceFamily:      "IPHONE",
					State:             "DRAFT",
					SupportsVoiceover: boolPtrTest(true),
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &AccessibilityDeclarationCreateRequest{
		Data: AccessibilityDeclarationCreateData{
			Type: "accessibilityDeclarations",
			Attributes: AccessibilityDeclarationCreateAttributes{
				DeviceFamily:      "IPHONE",
				SupportsVoiceover: boolPtrTest(true),
			},
			Relationships: AccessibilityDeclarationCreateRelationships{
				App: RelationshipData{
					Data: ResourceIdentifier{Type: "apps", ID: "123"},
				},
			},
		},
	}

	resp, err := client.CreateAccessibilityDeclaration(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.Attributes.State != "DRAFT" {
		t.Errorf("state = %q, want DRAFT", resp.Data.Attributes.State)
	}
}

func TestClient_UpdateAccessibilityDeclaration(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/accessibilityDeclarations/decl-1" {
			t.Errorf("path = %q, want /v1/accessibilityDeclarations/decl-1", r.URL.Path)
		}
		if r.Method != http.MethodPatch {
			t.Errorf("method = %q, want PATCH", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req AccessibilityDeclarationUpdateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Attributes == nil || req.Data.Attributes.Publish == nil || !*req.Data.Attributes.Publish {
			t.Error("expected publish=true in update attributes")
		}

		resp := AccessibilityDeclarationResponse{
			Data: AccessibilityDeclaration{
				Type:       "accessibilityDeclarations",
				ID:         "decl-1",
				Attributes: AccessibilityDeclarationAttributes{State: "PUBLISHED"},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &AccessibilityDeclarationUpdateRequest{
		Data: AccessibilityDeclarationUpdateData{
			Type:       "accessibilityDeclarations",
			ID:         "decl-1",
			Attributes: &AccessibilityDeclarationUpdateAttributes{Publish: boolPtrTest(true)},
		},
	}

	resp, err := client.UpdateAccessibilityDeclaration(context.Background(), "decl-1", req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.Attributes.State != "PUBLISHED" {
		t.Errorf("state = %q, want PUBLISHED", resp.Data.Attributes.State)
	}
}

func TestClient_DeleteAccessibilityDeclaration(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/accessibilityDeclarations/decl-1" {
			t.Errorf("path = %q, want /v1/accessibilityDeclarations/decl-1", r.URL.Path)
		}
		if r.Method != http.MethodDelete {
			t.Errorf("method = %q, want DELETE", r.Method)
		}
		w.WriteHeader(http.StatusNoContent)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	if err := client.DeleteAccessibilityDeclaration(context.Background(), "decl-1"); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestClient_UpdateAppTag(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/appTags/tag-1" {
			t.Errorf("path = %q, want /v1/appTags/tag-1", r.URL.Path)
		}
		if r.Method != http.MethodPatch {
			t.Errorf("method = %q, want PATCH", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req AppTagUpdateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Attributes.VisibleInAppStore == nil || *req.Data.Attributes.VisibleInAppStore {
			t.Error("expected visibleInAppStore=false in update attributes")
		}

		resp := AppTagResponse{
			Data: AppTag{
				Type: "appTags",
				ID:   "tag-1",
				Attributes: AppTagAttributes{
					Name:              "Photo Editing",
					VisibleInAppStore: boolPtrTest(false),
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &AppTagUpdateRequest{
		Data: AppTagUpdateData{
			Type:       "appTags",
			ID:         "tag-1",
			Attributes: AppTagUpdateAttributes{VisibleInAppStore: boolPtrTest(false)},
		},
	}

	resp, err := client.UpdateAppTag(context.Background(), "tag-1", req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.Attributes.VisibleInAppStore == nil || *resp.Data.Attributes.VisibleInAppStore {
		t.Error("expected tag to be hidden")
	}
}

func TestClient_CreateAndroidToIosAppMappingDetail(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/androidToIosAppMappingDetails" {
			t.Errorf("path = %q, want /v1/androidToIosAppMappingDetails", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req AndroidToIosAppMappingDetailCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Attributes.PackageName != "com.example.app" {
			t.Errorf("packageName = %q, want com.example.app", req.Data.Attributes.PackageName)
		}
		if len(req.Data.Attributes.AppSigningKeyPublicCertificateSha256Fingerprints) != 1 {
			t.Errorf("expected 1 fingerprint, got %d", len(req.Data.Attributes.AppSigningKeyPublicCertificateSha256Fingerprints))
		}
		if req.Data.Relationships.App.Data.ID != "123" {
			t.Errorf("app id = %q, want 123", req.Data.Relationships.App.Data.ID)
		}

		resp := AndroidToIosAppMappingDetailResponse{
			Data: AndroidToIosAppMappingDetail{
				Type:       "androidToIosAppMappingDetails",
				ID:         "map-1",
				Attributes: AndroidToIosAppMappingDetailAttributes{PackageName: "com.example.app"},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &AndroidToIosAppMappingDetailCreateRequest{
		Data: AndroidToIosAppMappingDetailCreateData{
			Type: "androidToIosAppMappingDetails",
			Attributes: AndroidToIosAppMappingDetailCreateAttributes{
				PackageName: "com.example.app",
				AppSigningKeyPublicCertificateSha256Fingerprints: []string{"AA:BB:CC"},
			},
			Relationships: AndroidToIosAppMappingDetailCreateRelationships{
				App: RelationshipData{
					Data: ResourceIdentifier{Type: "apps", ID: "123"},
				},
			},
		},
	}

	resp, err := client.CreateAndroidToIosAppMappingDetail(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "map-1" {
		t.Errorf("id = %q, want map-1", resp.Data.ID)
	}
}

func TestClient_DeleteAndroidToIosAppMappingDetail(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/androidToIosAppMappingDetails/map-1" {
			t.Errorf("path = %q, want /v1/androidToIosAppMappingDetails/map-1", r.URL.Path)
		}
		if r.Method != http.MethodDelete {
			t.Errorf("method = %q, want DELETE", r.Method)
		}
		w.WriteHeader(http.StatusNoContent)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	if err := client.DeleteAndroidToIosAppMappingDetail(context.Background(), "map-1"); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}
