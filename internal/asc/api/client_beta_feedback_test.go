package api

import (
	"context"
	"encoding/json"
	"net/http"
	"testing"
)

func TestClient_ListBetaFeedbackCrashSubmissions(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/apps/123/betaFeedbackCrashSubmissions" {
			t.Errorf("path = %q, want /v1/apps/123/betaFeedbackCrashSubmissions", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}
		if got := r.URL.Query().Get("filter[build]"); got != "build-9" {
			t.Errorf("filter[build] = %q, want build-9", got)
		}

		resp := BetaFeedbackCrashSubmissionsResponse{
			Data: []BetaFeedbackCrashSubmission{
				{
					Type: "betaFeedbackCrashSubmissions",
					ID:   "crash-1",
					Attributes: BetaFeedbackDeviceAttributes{
						Comment:     "crashed on launch",
						DeviceModel: "iPhone16,1",
						OSVersion:   "18.2",
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.ListBetaFeedbackCrashSubmissions(context.Background(), "123", &ListOptions{
		Filter: map[string][]string{"build": {"build-9"}},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 submission, got %d", len(resp.Data))
	}
	if resp.Data[0].Attributes.DeviceModel != "iPhone16,1" {
		t.Errorf("device model = %q, want iPhone16,1", resp.Data[0].Attributes.DeviceModel)
	}
}

func TestClient_GetBetaFeedbackCrashLog(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/betaFeedbackCrashSubmissions/crash-1/crashLog" {
			t.Errorf("path = %q, want /v1/betaFeedbackCrashSubmissions/crash-1/crashLog", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}

		resp := BetaCrashLogResponse{
			Data: BetaCrashLog{
				Type:       "betaCrashLogs",
				ID:         "crash-1",
				Attributes: BetaCrashLogAttributes{LogText: "Exception Type: EXC_CRASH"},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.GetBetaFeedbackCrashLog(context.Background(), "crash-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.Attributes.LogText != "Exception Type: EXC_CRASH" {
		t.Errorf("log text = %q", resp.Data.Attributes.LogText)
	}
}

func TestClient_DeleteBetaFeedbackCrashSubmission(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/betaFeedbackCrashSubmissions/crash-1" {
			t.Errorf("path = %q, want /v1/betaFeedbackCrashSubmissions/crash-1", r.URL.Path)
		}
		if r.Method != http.MethodDelete {
			t.Errorf("method = %q, want DELETE", r.Method)
		}
		w.WriteHeader(http.StatusNoContent)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	if err := client.DeleteBetaFeedbackCrashSubmission(context.Background(), "crash-1"); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestClient_ListBetaFeedbackScreenshotSubmissions(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/apps/123/betaFeedbackScreenshotSubmissions" {
			t.Errorf("path = %q, want /v1/apps/123/betaFeedbackScreenshotSubmissions", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}

		resp := BetaFeedbackScreenshotSubmissionsResponse{
			Data: []BetaFeedbackScreenshotSubmission{
				{
					Type: "betaFeedbackScreenshotSubmissions",
					ID:   "shot-1",
					Attributes: BetaFeedbackDeviceAttributes{
						Comment: "misaligned button",
						Screenshots: []BetaFeedbackScreenshotImage{
							{URL: "https://example.com/img.png", Width: 1179, Height: 2556},
						},
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.ListBetaFeedbackScreenshotSubmissions(context.Background(), "123", nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 submission, got %d", len(resp.Data))
	}
	shots := resp.Data[0].Attributes.Screenshots
	if len(shots) != 1 || shots[0].URL != "https://example.com/img.png" {
		t.Errorf("screenshots = %+v, want one with URL", shots)
	}
}

func TestClient_GetBetaFeedbackScreenshotSubmission(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/betaFeedbackScreenshotSubmissions/shot-1" {
			t.Errorf("path = %q, want /v1/betaFeedbackScreenshotSubmissions/shot-1", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}

		resp := BetaFeedbackScreenshotSubmissionResponse{
			Data: BetaFeedbackScreenshotSubmission{
				Type: "betaFeedbackScreenshotSubmissions",
				ID:   "shot-1",
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.GetBetaFeedbackScreenshotSubmission(context.Background(), "shot-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "shot-1" {
		t.Errorf("id = %q, want shot-1", resp.Data.ID)
	}
}

func TestClient_GetBetaBuildUsageMetrics(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/builds/build-9/metrics/betaBuildUsages" {
			t.Errorf("path = %q, want /v1/builds/build-9/metrics/betaBuildUsages", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}
		if got := r.URL.Query().Get("limit"); got != "5" {
			t.Errorf("limit = %q, want 5", got)
		}

		resp := BetaBuildUsagesResponse{
			Data: []BetaBuildUsageGroup{
				{
					DataPoints: []BetaBuildUsageDataPoint{
						{Values: BetaBuildUsageValues{InstallCount: 12, SessionCount: 40, CrashCount: 2}},
					},
				},
			},
		}

		json.NewEncoder(w).Encode(resp)
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.GetBetaBuildUsageMetrics(context.Background(), "build-9", 5)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 || len(resp.Data[0].DataPoints) != 1 {
		t.Fatalf("unexpected data shape: %+v", resp.Data)
	}
	if resp.Data[0].DataPoints[0].Values.InstallCount != 12 {
		t.Errorf("install count = %d, want 12", resp.Data[0].DataPoints[0].Values.InstallCount)
	}
}
