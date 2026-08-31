package api

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"sync/atomic"
	"testing"
)

func TestClient_ListBuildUploads(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/apps/123/buildUploads" {
			t.Errorf("path = %q, want /v1/apps/123/buildUploads", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}
		if got := r.URL.Query().Get("filter[state]"); got != "PROCESSING" {
			t.Errorf("filter[state] = %q, want PROCESSING", got)
		}

		resp := BuildUploadsResponse{
			Data: []BuildUpload{
				{
					Type: "buildUploads",
					ID:   "upload-1",
					Attributes: BuildUploadAttributes{
						CFBundleShortVersionString: "1.2.3",
						CFBundleVersion:            "42",
						Platform:                   "IOS",
						State:                      &BuildUploadState{State: "PROCESSING"},
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.ListBuildUploads(context.Background(), "123", &ListOptions{
		Filter: map[string][]string{"state": {"PROCESSING"}},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 build upload, got %d", len(resp.Data))
	}

	if resp.Data[0].Attributes.State.State != "PROCESSING" {
		t.Errorf("state = %q, want PROCESSING", resp.Data[0].Attributes.State.State)
	}
}

func TestClient_CreateBuildUpload(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/buildUploads" {
			t.Errorf("path = %q, want /v1/buildUploads", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req BuildUploadCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Type != "buildUploads" {
			t.Errorf("type = %q, want buildUploads", req.Data.Type)
		}
		if req.Data.Attributes.CFBundleVersion != "42" {
			t.Errorf("cfBundleVersion = %q, want 42", req.Data.Attributes.CFBundleVersion)
		}
		if req.Data.Relationships.App.Data.ID != "123" {
			t.Errorf("app id = %q, want 123", req.Data.Relationships.App.Data.ID)
		}

		resp := BuildUploadResponse{
			Data: BuildUpload{
				Type: "buildUploads",
				ID:   "upload-1",
				Attributes: BuildUploadAttributes{
					State: &BuildUploadState{State: "AWAITING_UPLOAD"},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &BuildUploadCreateRequest{
		Data: BuildUploadCreateData{
			Type: "buildUploads",
			Attributes: BuildUploadCreateAttributes{
				CFBundleShortVersionString: "1.2.3",
				CFBundleVersion:            "42",
				Platform:                   "IOS",
			},
			Relationships: BuildUploadCreateRelationships{
				App: RelationshipData{
					Data: ResourceIdentifier{Type: "apps", ID: "123"},
				},
			},
		},
	}

	resp, err := client.CreateBuildUpload(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "upload-1" {
		t.Errorf("id = %q, want upload-1", resp.Data.ID)
	}
}

func TestClient_GetBuildUpload(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/buildUploads/upload-1" {
			t.Errorf("path = %q, want /v1/buildUploads/upload-1", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}

		resp := BuildUploadResponse{
			Data: BuildUpload{
				Type: "buildUploads",
				ID:   "upload-1",
				Attributes: BuildUploadAttributes{
					State: &BuildUploadState{
						State:  "FAILED",
						Errors: []StateDetail{{Code: "INVALID_BINARY", Description: "bad ipa"}},
					},
				},
				Relationships: &BuildUploadRelationships{
					Build: &RelationshipData{
						Data: ResourceIdentifier{Type: "builds", ID: "build-9"},
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.GetBuildUpload(context.Background(), "upload-1", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.Attributes.State.State != "FAILED" {
		t.Errorf("state = %q, want FAILED", resp.Data.Attributes.State.State)
	}
	if len(resp.Data.Attributes.State.Errors) != 1 || resp.Data.Attributes.State.Errors[0].Code != "INVALID_BINARY" {
		t.Errorf("state errors = %+v, want one INVALID_BINARY", resp.Data.Attributes.State.Errors)
	}
	if resp.Data.Relationships.Build.Data.ID != "build-9" {
		t.Errorf("build id = %q, want build-9", resp.Data.Relationships.Build.Data.ID)
	}
}

func TestClient_DeleteBuildUpload(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/buildUploads/upload-1" {
			t.Errorf("path = %q, want /v1/buildUploads/upload-1", r.URL.Path)
		}
		if r.Method != http.MethodDelete {
			t.Errorf("method = %q, want DELETE", r.Method)
		}
		w.WriteHeader(http.StatusNoContent)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	if err := client.DeleteBuildUpload(context.Background(), "upload-1"); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestClient_ListBuildUploadFiles(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/buildUploads/upload-1/buildUploadFiles" {
			t.Errorf("path = %q, want /v1/buildUploads/upload-1/buildUploadFiles", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}

		resp := BuildUploadFilesResponse{
			Data: []BuildUploadFile{
				{
					Type: "buildUploadFiles",
					ID:   "file-1",
					Attributes: BuildUploadFileAttributes{
						FileName:           "MyApp.ipa",
						FileSize:           4,
						AssetType:          "ASSET",
						UTI:                "com.apple.ipa",
						AssetDeliveryState: &AssetDeliveryState{State: "COMPLETE"},
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.ListBuildUploadFiles(context.Background(), "upload-1", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 file, got %d", len(resp.Data))
	}
	if resp.Data[0].Attributes.AssetDeliveryState.State != "COMPLETE" {
		t.Errorf("delivery state = %q, want COMPLETE", resp.Data[0].Attributes.AssetDeliveryState.State)
	}
}

func TestUploadBuildFile_EndToEnd(t *testing.T) {
	// One server stands in for both the App Store Connect API and the
	// pre-signed storage URL. We route by path.
	var uploaded atomic.Bool
	var committed atomic.Bool

	var storageURL string
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch {
		case r.Method == http.MethodPost && r.URL.Path == "/v1/buildUploadFiles":
			var req BuildUploadFileCreateRequest
			body, _ := io.ReadAll(r.Body)
			if err := json.Unmarshal(body, &req); err != nil {
				t.Fatalf("reservation body invalid: %v", err)
			}
			if req.Data.Attributes.AssetType != "ASSET" {
				t.Errorf("assetType = %q, want ASSET", req.Data.Attributes.AssetType)
			}
			if req.Data.Attributes.UTI != "com.apple.ipa" {
				t.Errorf("uti = %q, want com.apple.ipa", req.Data.Attributes.UTI)
			}
			if req.Data.Relationships.BuildUpload.Data.ID != "upload-1" {
				t.Errorf("buildUpload id = %q, want upload-1", req.Data.Relationships.BuildUpload.Data.ID)
			}
			// Reservation: respond with an upload operation pointing at /upload.
			resp := BuildUploadFileResponse{
				Data: BuildUploadFile{
					Type: "buildUploadFiles",
					ID:   "file-1",
					Attributes: BuildUploadFileAttributes{
						FileName: "MyApp.ipa",
						FileSize: 4,
						UploadOperations: []UploadOperation{
							{Method: "PUT", URL: storageURL + "/upload", Offset: 0, Length: 4,
								RequestHeaders: []RequestHeader{{Name: "Content-Type", Value: "application/octet-stream"}}},
						},
					},
				},
			}
			w.WriteHeader(http.StatusCreated)
			_ = json.NewEncoder(w).Encode(resp)
		case r.Method == http.MethodPut && r.URL.Path == "/upload":
			body, _ := io.ReadAll(r.Body)
			if string(body) != "IPA!" {
				t.Errorf("uploaded body = %q, want IPA!", body)
			}
			uploaded.Store(true)
			w.WriteHeader(http.StatusOK)
		case r.Method == http.MethodPatch && r.URL.Path == "/v1/buildUploadFiles/file-1":
			var commit BuildUploadFileUpdateRequest
			body, _ := io.ReadAll(r.Body)
			if err := json.Unmarshal(body, &commit); err != nil {
				t.Fatalf("commit body invalid: %v", err)
			}
			checksums := commit.Data.Attributes.SourceFileChecksums
			if checksums == nil || checksums.File == nil {
				t.Fatalf("commit checksums missing: %+v", commit.Data.Attributes)
			}
			if checksums.File.Hash != Sha256Hex([]byte("IPA!")) {
				t.Errorf("commit checksum = %q", checksums.File.Hash)
			}
			if checksums.File.Algorithm != "SHA_256" {
				t.Errorf("commit algorithm = %q, want SHA_256", checksums.File.Algorithm)
			}
			if commit.Data.Attributes.Uploaded == nil || !*commit.Data.Attributes.Uploaded {
				t.Errorf("commit uploaded flag not true")
			}
			committed.Store(true)
			w.WriteHeader(http.StatusOK)
			_ = json.NewEncoder(w).Encode(BuildUploadFileResponse{
				Data: BuildUploadFile{Type: "buildUploadFiles", ID: "file-1"},
			})
		default:
			t.Errorf("unexpected request: %s %s", r.Method, r.URL.Path)
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer srv.Close()
	storageURL = srv.URL

	c := &Client{
		httpClient:    &http.Client{},
		tokenProvider: mockTokenProvider(t),
		baseURL:       srv.URL,
	}
	resp, err := c.UploadBuildFile(context.Background(), "upload-1", "MyApp.ipa", "ASSET", "com.apple.ipa", []byte("IPA!"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp.Data.ID != "file-1" {
		t.Errorf("final ID = %q, want file-1", resp.Data.ID)
	}
	if !uploaded.Load() {
		t.Error("upload step did not run")
	}
	if !committed.Load() {
		t.Error("commit step did not run")
	}
}

func TestUploadBuildFile_Validation(t *testing.T) {
	c := &Client{}

	tests := []struct {
		name     string
		fileName string
		body     []byte
	}{
		{name: "missing file name", fileName: "", body: []byte("IPA!")},
		{name: "empty body", fileName: "MyApp.ipa", body: nil},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if _, err := c.UploadBuildFile(context.Background(), "upload-1", tt.fileName, "ASSET", "com.apple.ipa", tt.body); err == nil {
				t.Error("expected error")
			}
		})
	}
}
