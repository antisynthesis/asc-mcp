package api

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"testing"
)

// boolPtrTest returns a pointer to the given bool.
func boolPtrTest(b bool) *bool { return &b }

func TestClient_ListReviewSubmissions(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/apps/123/reviewSubmissions" {
			t.Errorf("path = %q, want /v1/apps/123/reviewSubmissions", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}

		resp := ReviewSubmissionsResponse{
			Data: []ReviewSubmission{
				{
					Type: "reviewSubmissions",
					ID:   "sub-1",
					Attributes: ReviewSubmissionAttributes{
						Platform: "IOS",
						State:    "READY_FOR_REVIEW",
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.ListReviewSubmissions(context.Background(), "123", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 review submission, got %d", len(resp.Data))
	}

	if resp.Data[0].Attributes.State != "READY_FOR_REVIEW" {
		t.Errorf("state = %q, want READY_FOR_REVIEW", resp.Data[0].Attributes.State)
	}
}

func TestClient_CreateReviewSubmission(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/reviewSubmissions" {
			t.Errorf("path = %q, want /v1/reviewSubmissions", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req ReviewSubmissionCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Type != "reviewSubmissions" {
			t.Errorf("type = %q, want reviewSubmissions", req.Data.Type)
		}
		if req.Data.Relationships.App.Data.ID != "123" {
			t.Errorf("app id = %q, want 123", req.Data.Relationships.App.Data.ID)
		}

		resp := ReviewSubmissionResponse{
			Data: ReviewSubmission{
				Type: "reviewSubmissions",
				ID:   "sub-1",
				Attributes: ReviewSubmissionAttributes{
					Platform: "IOS",
					State:    "READY_FOR_REVIEW",
				},
			},
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &ReviewSubmissionCreateRequest{
		Data: ReviewSubmissionCreateData{
			Type:       "reviewSubmissions",
			Attributes: &ReviewSubmissionCreateAttributes{Platform: "IOS"},
			Relationships: ReviewSubmissionCreateRelationships{
				App: RelationshipData{
					Data: ResourceIdentifier{Type: "apps", ID: "123"},
				},
			},
		},
	}

	resp, err := client.CreateReviewSubmission(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "sub-1" {
		t.Errorf("id = %q, want sub-1", resp.Data.ID)
	}
}

func TestClient_UpdateReviewSubmission(t *testing.T) {
	tests := []struct {
		name      string
		submitted *bool
		canceled  *bool
	}{
		{name: "submit", submitted: boolPtrTest(true)},
		{name: "cancel", canceled: boolPtrTest(true)},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				if r.URL.Path != "/v1/reviewSubmissions/sub-1" {
					t.Errorf("path = %q, want /v1/reviewSubmissions/sub-1", r.URL.Path)
				}
				if r.Method != http.MethodPatch {
					t.Errorf("method = %q, want PATCH", r.Method)
				}

				body, _ := io.ReadAll(r.Body)
				var req ReviewSubmissionUpdateRequest
				if err := json.Unmarshal(body, &req); err != nil {
					t.Fatalf("failed to decode request: %v", err)
				}
				if tt.submitted != nil && (req.Data.Attributes.Submitted == nil || !*req.Data.Attributes.Submitted) {
					t.Error("expected submitted=true in request body")
				}
				if tt.canceled != nil && (req.Data.Attributes.Canceled == nil || !*req.Data.Attributes.Canceled) {
					t.Error("expected canceled=true in request body")
				}

				resp := ReviewSubmissionResponse{
					Data: ReviewSubmission{Type: "reviewSubmissions", ID: "sub-1"},
				}
				json.NewEncoder(w).Encode(resp)
			})

			client, server := newTestClient(t, handler)
			defer server.Close()

			req := &ReviewSubmissionUpdateRequest{
				Data: ReviewSubmissionUpdateData{
					Type: "reviewSubmissions",
					ID:   "sub-1",
					Attributes: ReviewSubmissionUpdateAttributes{
						Submitted: tt.submitted,
						Canceled:  tt.canceled,
					},
				},
			}

			resp, err := client.UpdateReviewSubmission(context.Background(), "sub-1", req)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if resp.Data.ID != "sub-1" {
				t.Errorf("id = %q, want sub-1", resp.Data.ID)
			}
		})
	}
}

func TestClient_CreateReviewSubmissionItem(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/reviewSubmissionItems" {
			t.Errorf("path = %q, want /v1/reviewSubmissionItems", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req ReviewSubmissionItemCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Relationships.ReviewSubmission.Data.ID != "sub-1" {
			t.Errorf("review submission id = %q, want sub-1", req.Data.Relationships.ReviewSubmission.Data.ID)
		}
		if req.Data.Relationships.AppStoreVersion == nil || req.Data.Relationships.AppStoreVersion.Data.ID != "ver-1" {
			t.Error("expected appStoreVersion relationship with id ver-1")
		}

		resp := ReviewSubmissionItemResponse{
			Data: ReviewSubmissionItem{Type: "reviewSubmissionItems", ID: "item-1"},
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &ReviewSubmissionItemCreateRequest{
		Data: ReviewSubmissionItemCreateData{
			Type: "reviewSubmissionItems",
			Relationships: ReviewSubmissionItemCreateRelationships{
				ReviewSubmission: RelationshipData{
					Data: ResourceIdentifier{Type: "reviewSubmissions", ID: "sub-1"},
				},
				AppStoreVersion: &RelationshipData{
					Data: ResourceIdentifier{Type: "appStoreVersions", ID: "ver-1"},
				},
			},
		},
	}

	resp, err := client.CreateReviewSubmissionItem(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "item-1" {
		t.Errorf("id = %q, want item-1", resp.Data.ID)
	}
}
