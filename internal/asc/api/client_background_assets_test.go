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

func TestClient_ListBackgroundAssets(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/apps/123/backgroundAssets" {
			t.Errorf("path = %q, want /v1/apps/123/backgroundAssets", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}
		if got := r.URL.Query().Get("filter[versions.locale]"); got != "en-US" {
			t.Errorf("filter[versions.locale] = %q, want en-US", got)
		}

		resp := BackgroundAssetsResponse{
			Data: []BackgroundAsset{
				{
					Type: "backgroundAssets",
					ID:   "asset-1",
					Attributes: BackgroundAssetAttributes{
						AssetPackIdentifier: "com.example.game.levels1",
						UsedBytes:           1024,
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.ListBackgroundAssets(context.Background(), "123", &ListOptions{
		Filter: map[string][]string{"versions.locale": {"en-US"}},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 background asset, got %d", len(resp.Data))
	}

	if resp.Data[0].Attributes.AssetPackIdentifier != "com.example.game.levels1" {
		t.Errorf("assetPackIdentifier = %q, want com.example.game.levels1", resp.Data[0].Attributes.AssetPackIdentifier)
	}
	if resp.Data[0].Attributes.UsedBytes != 1024 {
		t.Errorf("usedBytes = %d, want 1024", resp.Data[0].Attributes.UsedBytes)
	}
}

func TestClient_CreateBackgroundAsset(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/backgroundAssets" {
			t.Errorf("path = %q, want /v1/backgroundAssets", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req BackgroundAssetCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Type != "backgroundAssets" {
			t.Errorf("type = %q, want backgroundAssets", req.Data.Type)
		}
		if req.Data.Attributes.AssetPackIdentifier != "com.example.game.levels1" {
			t.Errorf("assetPackIdentifier = %q, want com.example.game.levels1", req.Data.Attributes.AssetPackIdentifier)
		}
		if req.Data.Relationships.App.Data.ID != "123" {
			t.Errorf("app id = %q, want 123", req.Data.Relationships.App.Data.ID)
		}

		resp := BackgroundAssetResponse{
			Data: BackgroundAsset{
				Type: "backgroundAssets",
				ID:   "asset-1",
				Attributes: BackgroundAssetAttributes{
					AssetPackIdentifier: "com.example.game.levels1",
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &BackgroundAssetCreateRequest{
		Data: BackgroundAssetCreateData{
			Type: "backgroundAssets",
			Attributes: BackgroundAssetCreateAttributes{
				AssetPackIdentifier: "com.example.game.levels1",
			},
			Relationships: BackgroundAssetCreateRelationships{
				App: RelationshipData{
					Data: ResourceIdentifier{Type: "apps", ID: "123"},
				},
			},
		},
	}

	resp, err := client.CreateBackgroundAsset(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "asset-1" {
		t.Errorf("id = %q, want asset-1", resp.Data.ID)
	}
}

func TestClient_UpdateBackgroundAsset(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/backgroundAssets/asset-1" {
			t.Errorf("path = %q, want /v1/backgroundAssets/asset-1", r.URL.Path)
		}
		if r.Method != http.MethodPatch {
			t.Errorf("method = %q, want PATCH", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req BackgroundAssetUpdateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Attributes == nil || req.Data.Attributes.Archived == nil || !*req.Data.Attributes.Archived {
			t.Errorf("archived = %+v, want true", req.Data.Attributes)
		}

		resp := BackgroundAssetResponse{
			Data: BackgroundAsset{
				Type: "backgroundAssets",
				ID:   "asset-1",
				Attributes: BackgroundAssetAttributes{
					Archived: true,
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	archived := true
	req := &BackgroundAssetUpdateRequest{
		Data: BackgroundAssetUpdateData{
			Type:       "backgroundAssets",
			ID:         "asset-1",
			Attributes: &BackgroundAssetUpdateAttributes{Archived: &archived},
		},
	}

	resp, err := client.UpdateBackgroundAsset(context.Background(), "asset-1", req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if !resp.Data.Attributes.Archived {
		t.Error("archived = false, want true")
	}
}

func TestClient_ListBackgroundAssetVersions(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/backgroundAssets/asset-1/versions" {
			t.Errorf("path = %q, want /v1/backgroundAssets/asset-1/versions", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}
		if got := r.URL.Query().Get("filter[state]"); got != "COMPLETE" {
			t.Errorf("filter[state] = %q, want COMPLETE", got)
		}

		resp := BackgroundAssetVersionsResponse{
			Data: []BackgroundAssetVersion{
				{
					Type: "backgroundAssetVersions",
					ID:   "version-1",
					Attributes: BackgroundAssetVersionAttributes{
						State:   "COMPLETE",
						Version: "3",
						Locale:  "en-US",
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.ListBackgroundAssetVersions(context.Background(), "asset-1", &ListOptions{
		Filter: map[string][]string{"state": {"COMPLETE"}},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 version, got %d", len(resp.Data))
	}
	if resp.Data[0].Attributes.State != "COMPLETE" {
		t.Errorf("state = %q, want COMPLETE", resp.Data[0].Attributes.State)
	}
}

func TestClient_GetBackgroundAssetVersion(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/backgroundAssetVersions/version-1" {
			t.Errorf("path = %q, want /v1/backgroundAssetVersions/version-1", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}

		resp := BackgroundAssetVersionResponse{
			Data: BackgroundAssetVersion{
				Type: "backgroundAssetVersions",
				ID:   "version-1",
				Attributes: BackgroundAssetVersionAttributes{
					State: "FAILED",
					StateDetails: &BackgroundAssetVersionStateDetail{
						Errors: []StateDetail{{Code: "INVALID_MANIFEST", Description: "bad manifest"}},
					},
				},
				Relationships: &BackgroundAssetVersionRelationships{
					AppStoreRelease: &RelationshipData{
						Data: ResourceIdentifier{Type: "backgroundAssetVersionAppStoreReleases", ID: "release-9"},
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.GetBackgroundAssetVersion(context.Background(), "version-1", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.Attributes.State != "FAILED" {
		t.Errorf("state = %q, want FAILED", resp.Data.Attributes.State)
	}
	details := resp.Data.Attributes.StateDetails
	if details == nil || len(details.Errors) != 1 || details.Errors[0].Code != "INVALID_MANIFEST" {
		t.Errorf("state details = %+v, want one INVALID_MANIFEST error", details)
	}
	if resp.Data.Relationships.AppStoreRelease.Data.ID != "release-9" {
		t.Errorf("app store release id = %q, want release-9", resp.Data.Relationships.AppStoreRelease.Data.ID)
	}
}

func TestClient_CreateBackgroundAssetVersion(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/backgroundAssetVersions" {
			t.Errorf("path = %q, want /v1/backgroundAssetVersions", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req BackgroundAssetVersionCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Type != "backgroundAssetVersions" {
			t.Errorf("type = %q, want backgroundAssetVersions", req.Data.Type)
		}
		if req.Data.Relationships.BackgroundAsset.Data.ID != "asset-1" {
			t.Errorf("backgroundAsset id = %q, want asset-1", req.Data.Relationships.BackgroundAsset.Data.ID)
		}

		resp := BackgroundAssetVersionResponse{
			Data: BackgroundAssetVersion{
				Type: "backgroundAssetVersions",
				ID:   "version-1",
				Attributes: BackgroundAssetVersionAttributes{
					State: "AWAITING_UPLOAD",
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &BackgroundAssetVersionCreateRequest{
		Data: BackgroundAssetVersionCreateData{
			Type: "backgroundAssetVersions",
			Relationships: BackgroundAssetVersionCreateRelationships{
				BackgroundAsset: RelationshipData{
					Data: ResourceIdentifier{Type: "backgroundAssets", ID: "asset-1"},
				},
			},
		},
	}

	resp, err := client.CreateBackgroundAssetVersion(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "version-1" {
		t.Errorf("id = %q, want version-1", resp.Data.ID)
	}
}

func TestClient_ListBackgroundAssetUploadFiles(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/backgroundAssetVersions/version-1/backgroundAssetUploadFiles" {
			t.Errorf("path = %q, want /v1/backgroundAssetVersions/version-1/backgroundAssetUploadFiles", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}

		resp := BackgroundAssetUploadFilesResponse{
			Data: []BackgroundAssetUploadFile{
				{
					Type: "backgroundAssetUploadFiles",
					ID:   "file-1",
					Attributes: BackgroundAssetUploadFileAttributes{
						FileName:           "levels1.aar",
						FileSize:           4,
						AssetType:          "ASSET",
						AssetDeliveryState: &AssetDeliveryState{State: "COMPLETE"},
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.ListBackgroundAssetUploadFiles(context.Background(), "version-1", nil)
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

func TestClient_GetBackgroundAssetVersionReleases(t *testing.T) {
	tests := []struct {
		name     string
		wantPath string
		call     func(c *Client) (*BackgroundAssetVersionReleaseResponse, error)
	}{
		{
			name:     "internal beta",
			wantPath: "/v1/backgroundAssetVersionInternalBetaReleases/release-1",
			call: func(c *Client) (*BackgroundAssetVersionReleaseResponse, error) {
				return c.GetBackgroundAssetVersionInternalBetaRelease(context.Background(), "release-1")
			},
		},
		{
			name:     "external beta",
			wantPath: "/v1/backgroundAssetVersionExternalBetaReleases/release-1",
			call: func(c *Client) (*BackgroundAssetVersionReleaseResponse, error) {
				return c.GetBackgroundAssetVersionExternalBetaRelease(context.Background(), "release-1")
			},
		},
		{
			name:     "app store",
			wantPath: "/v1/backgroundAssetVersionAppStoreReleases/release-1",
			call: func(c *Client) (*BackgroundAssetVersionReleaseResponse, error) {
				return c.GetBackgroundAssetVersionAppStoreRelease(context.Background(), "release-1")
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

				resp := BackgroundAssetVersionReleaseResponse{
					Data: BackgroundAssetVersionRelease{
						ID: "release-1",
						Attributes: BackgroundAssetVersionReleaseAttributes{
							State: "READY_FOR_TESTING",
						},
					},
				}

				json.NewEncoder(w).Encode(resp)
			})

			client, server := newTestClient(t, handler)
			defer server.Close()

			resp, err := tt.call(client)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}

			if resp.Data.Attributes.State != "READY_FOR_TESTING" {
				t.Errorf("state = %q, want READY_FOR_TESTING", resp.Data.Attributes.State)
			}
		})
	}
}

func TestUploadBackgroundAssetFile_EndToEnd(t *testing.T) {
	// One server stands in for both the App Store Connect API and the
	// pre-signed storage URL. We route by path.
	var uploaded atomic.Bool
	var committed atomic.Bool

	var storageURL string
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch {
		case r.Method == http.MethodPost && r.URL.Path == "/v1/backgroundAssetUploadFiles":
			var req BackgroundAssetUploadFileCreateRequest
			body, _ := io.ReadAll(r.Body)
			if err := json.Unmarshal(body, &req); err != nil {
				t.Fatalf("reservation body invalid: %v", err)
			}
			if req.Data.Attributes.AssetType != "MANIFEST" {
				t.Errorf("assetType = %q, want MANIFEST", req.Data.Attributes.AssetType)
			}
			if req.Data.Relationships.BackgroundAssetVersion.Data.ID != "version-1" {
				t.Errorf("backgroundAssetVersion id = %q, want version-1", req.Data.Relationships.BackgroundAssetVersion.Data.ID)
			}
			// Reservation: respond with an upload operation pointing at /upload.
			resp := BackgroundAssetUploadFileResponse{
				Data: BackgroundAssetUploadFile{
					Type: "backgroundAssetUploadFiles",
					ID:   "file-1",
					Attributes: BackgroundAssetUploadFileAttributes{
						FileName: "Manifest.plist",
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
			if string(body) != "PACK" {
				t.Errorf("uploaded body = %q, want PACK", body)
			}
			uploaded.Store(true)
			w.WriteHeader(http.StatusOK)
		case r.Method == http.MethodPatch && r.URL.Path == "/v1/backgroundAssetUploadFiles/file-1":
			var commit BackgroundAssetUploadFileUpdateRequest
			body, _ := io.ReadAll(r.Body)
			if err := json.Unmarshal(body, &commit); err != nil {
				t.Fatalf("commit body invalid: %v", err)
			}
			checksums := commit.Data.Attributes.SourceFileChecksums
			if checksums == nil || checksums.File == nil {
				t.Fatalf("commit checksums missing: %+v", commit.Data.Attributes)
			}
			if checksums.File.Hash != Sha256Hex([]byte("PACK")) {
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
			_ = json.NewEncoder(w).Encode(BackgroundAssetUploadFileResponse{
				Data: BackgroundAssetUploadFile{Type: "backgroundAssetUploadFiles", ID: "file-1"},
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
	resp, err := c.UploadBackgroundAssetFile(context.Background(), "version-1", "Manifest.plist", "MANIFEST", []byte("PACK"))
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

func TestUploadBackgroundAssetFile_Validation(t *testing.T) {
	c := &Client{}

	tests := []struct {
		name     string
		fileName string
		body     []byte
	}{
		{name: "missing file name", fileName: "", body: []byte("PACK")},
		{name: "empty body", fileName: "levels1.aar", body: nil},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if _, err := c.UploadBackgroundAssetFile(context.Background(), "version-1", tt.fileName, "ASSET", tt.body); err == nil {
				t.Error("expected error")
			}
		})
	}
}
