package api

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
)

func TestPerformUploadOperations_HappyPath(t *testing.T) {
	var mu sync.Mutex
	var received [][]byte
	var headers []string

	storage := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPut {
			t.Errorf("upload method = %s, want PUT", r.Method)
		}
		body, _ := io.ReadAll(r.Body)
		mu.Lock()
		received = append(received, body)
		headers = append(headers, r.Header.Get("X-Test"))
		mu.Unlock()
		w.WriteHeader(http.StatusOK)
	}))
	defer storage.Close()

	// Three chunks of 4 bytes each from a 12-byte body.
	body := []byte("AAAABBBBCCCC")
	ops := []UploadOperation{
		{Method: "PUT", URL: storage.URL + "/0", Offset: 0, Length: 4,
			RequestHeaders: []RequestHeader{{Name: "X-Test", Value: "first"}}},
		{Method: "PUT", URL: storage.URL + "/1", Offset: 4, Length: 4,
			RequestHeaders: []RequestHeader{{Name: "X-Test", Value: "second"}}},
		{Method: "PUT", URL: storage.URL + "/2", Offset: 8, Length: 4,
			RequestHeaders: []RequestHeader{{Name: "X-Test", Value: "third"}}},
	}

	c := &Client{
		httpClient:    &http.Client{},
		tokenProvider: mockTokenProvider(t),
		baseURL:       storage.URL,
	}
	if err := c.PerformUploadOperations(context.Background(), ops, body); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(received) != 3 {
		t.Fatalf("received %d chunks, want 3", len(received))
	}
	want := []string{"AAAA", "BBBB", "CCCC"}
	for i, w := range want {
		if string(received[i]) != w {
			t.Errorf("chunk %d = %q, want %q", i, received[i], w)
		}
	}
	if headers[0] != "first" || headers[2] != "third" {
		t.Errorf("custom headers not propagated: %v", headers)
	}
}

func TestPerformUploadOperations_BoundsCheck(t *testing.T) {
	c := &Client{
		httpClient:    &http.Client{},
		tokenProvider: mockTokenProvider(t),
		baseURL:       "http://localhost",
	}
	body := []byte("ABCD")
	cases := []struct {
		name string
		op   UploadOperation
	}{
		{"negative offset", UploadOperation{URL: "http://example", Offset: -1, Length: 2}},
		{"zero length", UploadOperation{URL: "http://example", Offset: 0, Length: 0}},
		{"past end", UploadOperation{URL: "http://example", Offset: 2, Length: 10}},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			err := c.PerformUploadOperations(context.Background(), []UploadOperation{tc.op}, body)
			if err == nil {
				t.Fatal("expected error")
			}
			if !strings.Contains(err.Error(), "out of bounds") {
				t.Errorf("error %q does not mention bounds", err.Error())
			}
		})
	}
}

func TestPerformUploadOperations_ServerError(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte("nope"))
	}))
	defer srv.Close()
	c := &Client{
		httpClient:    &http.Client{},
		tokenProvider: mockTokenProvider(t),
		baseURL:       srv.URL,
	}
	body := []byte("AAAA")
	ops := []UploadOperation{{Method: "PUT", URL: srv.URL, Offset: 0, Length: 4}}
	err := c.PerformUploadOperations(context.Background(), ops, body)
	if err == nil {
		t.Fatal("expected error")
	}
	if !strings.Contains(err.Error(), "500") {
		t.Errorf("error %q does not mention status code", err.Error())
	}
}

func TestSha256Hex_KnownValue(t *testing.T) {
	// SHA-256 of "abc" per FIPS 180-4 test vectors.
	got := Sha256Hex([]byte("abc"))
	want := "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
	if got != want {
		t.Errorf("Sha256Hex(\"abc\") = %q, want %q", got, want)
	}
}

func TestUploadAppScreenshot_EndToEnd(t *testing.T) {
	// One server stands in for both the App Store Connect API and the
	// pre-signed storage URL. We route by path.
	var uploaded atomic.Bool
	var committed atomic.Bool

	var storageURL string
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch {
		case r.Method == http.MethodPost && r.URL.Path == "/v1/appScreenshots":
			// Reservation: respond with an upload operation pointing at /upload.
			resp := AppScreenshotResponse{
				Data: AppScreenshot{
					Type: "appScreenshots",
					ID:   "screen-1",
					Attributes: AppScreenshotAttributes{
						FileName: "shot.png",
						FileSize: 4,
						UploadOperations: []UploadOperation{
							{Method: "PUT", URL: storageURL + "/upload", Offset: 0, Length: 4,
								RequestHeaders: []RequestHeader{{Name: "Content-Type", Value: "image/png"}}},
						},
					},
				},
			}
			w.WriteHeader(http.StatusCreated)
			_ = json.NewEncoder(w).Encode(resp)
		case r.Method == http.MethodPut && r.URL.Path == "/upload":
			body, _ := io.ReadAll(r.Body)
			if string(body) != "PNG!" {
				t.Errorf("uploaded body = %q, want PNG!", body)
			}
			uploaded.Store(true)
			w.WriteHeader(http.StatusOK)
		case r.Method == http.MethodPatch && r.URL.Path == "/v1/appScreenshots/screen-1":
			var commit AppScreenshotUpdateRequest
			body, _ := io.ReadAll(r.Body)
			if err := json.Unmarshal(body, &commit); err != nil {
				t.Fatalf("commit body invalid: %v", err)
			}
			if commit.Data.Attributes.SourceFileChecksum != Sha256Hex([]byte("PNG!")) {
				t.Errorf("commit checksum = %q", commit.Data.Attributes.SourceFileChecksum)
			}
			if commit.Data.Attributes.Uploaded == nil || !*commit.Data.Attributes.Uploaded {
				t.Errorf("commit uploaded flag not true")
			}
			committed.Store(true)
			w.WriteHeader(http.StatusOK)
			_ = json.NewEncoder(w).Encode(AppScreenshotResponse{
				Data: AppScreenshot{Type: "appScreenshots", ID: "screen-1"},
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
	resp, err := c.UploadAppScreenshot(context.Background(), "set-1", "shot.png", []byte("PNG!"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp.Data.ID != "screen-1" {
		t.Errorf("final ID = %q, want screen-1", resp.Data.ID)
	}
	if !uploaded.Load() {
		t.Error("upload step did not run")
	}
	if !committed.Load() {
		t.Error("commit step did not run")
	}
}
