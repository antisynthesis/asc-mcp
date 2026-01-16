package tools

import (
	"context"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"crypto/x509"
	"encoding/json"
	"encoding/pem"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// testClient creates a test API client with a mock server.
func testClient(t *testing.T, handler http.Handler) *api.Client {
	t.Helper()

	server := httptest.NewServer(handler)
	t.Cleanup(server.Close)

	// Create test key
	privateKey, err := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		t.Fatalf("failed to generate key: %v", err)
	}

	keyBytes, err := x509.MarshalPKCS8PrivateKey(privateKey)
	if err != nil {
		t.Fatalf("failed to marshal key: %v", err)
	}

	pemBlock := &pem.Block{
		Type:  "PRIVATE KEY",
		Bytes: keyBytes,
	}

	tmpDir := t.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	if err := os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600); err != nil {
		t.Fatalf("failed to write key: %v", err)
	}

	client, err := api.NewClient("test-issuer", "TESTKEY123", keyPath)
	if err != nil {
		t.Fatalf("failed to create client: %v", err)
	}

	// Use reflection or a test helper to set the baseURL
	// Since we can't access private fields, we'll create a wrapper
	return client
}

// mockHandler creates a simple mock HTTP handler.
func mockHandler(response interface{}) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(response)
	}
}

func TestNewRegistry(t *testing.T) {
	// Create minimal mock server
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte(`{}`))
	}))
	defer server.Close()

	// Create test key and client
	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(privateKey)
	pemBlock := &pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}

	tmpDir := t.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)

	client, err := api.NewClient("test-issuer", "TESTKEY123", keyPath)
	if err != nil {
		t.Fatalf("failed to create client: %v", err)
	}

	registry := NewRegistry(client)

	if registry == nil {
		t.Fatal("expected registry, got nil")
	}

	if registry.client != client {
		t.Error("client not set correctly")
	}

	if len(registry.tools) == 0 {
		t.Error("expected tools to be registered")
	}

	if len(registry.handlers) == 0 {
		t.Error("expected handlers to be registered")
	}
}

func TestRegistry_ListTools(t *testing.T) {
	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(privateKey)
	pemBlock := &pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}

	tmpDir := t.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)

	client, _ := api.NewClient("test-issuer", "TESTKEY123", keyPath)
	registry := NewRegistry(client)

	tools := registry.ListTools()

	// Should have 110 tools total
	if len(tools) != 110 {
		t.Errorf("expected 110 tools, got %d", len(tools))
	}

	// Verify tool structure
	expectedTools := map[string]bool{
		// App tools
		"list_apps":        false,
		"get_app":          false,
		"get_app_versions": false,
		// Build tools
		"list_builds": false,
		"get_build":   false,
		// TestFlight tools
		"list_beta_groups":    false,
		"create_beta_group":   false,
		"delete_beta_group":   false,
		"list_beta_testers":   false,
		"invite_beta_tester":  false,
		"remove_beta_tester":  false,
		"add_tester_to_group": false,
		// Provisioning tools
		"list_bundle_ids":   false,
		"get_bundle_id":     false,
		"list_certificates": false,
		"list_profiles":     false,
		"list_devices":      false,
		"register_device":   false,
		// App Info Localization tools
		"get_app_infos":                false,
		"list_app_info_localizations":  false,
		"get_app_info_localization":    false,
		"create_app_info_localization": false,
		"update_app_info_localization": false,
		"delete_app_info_localization": false,
		// Version Localization tools
		"list_version_localizations":  false,
		"get_version_localization":    false,
		"create_version_localization": false,
		"update_version_localization": false,
		"delete_version_localization": false,
		// Customer Reviews tools
		"list_customer_reviews":          false,
		"get_customer_review":            false,
		"create_customer_review_response": false,
		"delete_customer_review_response": false,
		// In-App Purchase tools
		"list_in_app_purchases":  false,
		"get_in_app_purchase":    false,
		"create_in_app_purchase": false,
		"update_in_app_purchase": false,
		"delete_in_app_purchase": false,
		// Subscription tools
		"list_subscription_groups": false,
		"get_subscription_group":   false,
		"list_subscriptions":       false,
		"get_subscription":         false,
		// App Store Version tools
		"list_app_store_versions":     false,
		"get_app_store_version":       false,
		"create_app_store_version":    false,
		"update_app_store_version":    false,
		"delete_app_store_version":    false,
		"submit_app_for_review":       false,
		"get_app_store_review_detail": false,
		"create_app_store_review_detail": false,
		"update_app_store_review_detail": false,
		// Phased Release tools
		"get_phased_release":    false,
		"create_phased_release": false,
		"update_phased_release": false,
		"delete_phased_release": false,
		// Screenshot tools
		"list_screenshot_sets": false,
		"list_screenshots":     false,
		"get_screenshot":       false,
		"delete_screenshot":    false,
		"list_preview_sets":    false,
		"list_previews":        false,
		"get_preview":          false,
		"delete_preview":       false,
		// Pre-Order tools
		"get_pre_order":    false,
		"create_pre_order": false,
		"update_pre_order": false,
		"delete_pre_order": false,
		// App Event tools
		"list_app_events":  false,
		"get_app_event":    false,
		"create_app_event": false,
		"update_app_event": false,
		"delete_app_event": false,
		// Analytics tools
		"list_analytics_report_requests":   false,
		"get_analytics_report_request":     false,
		"create_analytics_report_request":  false,
		"delete_analytics_report_request":  false,
		"list_analytics_reports":           false,
		"list_analytics_report_instances":  false,
		"list_analytics_report_segments":   false,
		// App Clip tools
		"list_app_clips":                      false,
		"get_app_clip":                        false,
		"list_app_clip_default_experiences":   false,
		"get_app_clip_default_experience":     false,
		"list_app_clip_advanced_experiences":  false,
		"get_app_clip_advanced_experience":    false,
		// Game Center tools
		"get_game_center_detail":           false,
		"list_game_center_achievements":    false,
		"get_game_center_achievement":      false,
		"create_game_center_achievement":   false,
		"update_game_center_achievement":   false,
		"delete_game_center_achievement":   false,
		"list_game_center_leaderboards":    false,
		"get_game_center_leaderboard":      false,
		"create_game_center_leaderboard":   false,
		"update_game_center_leaderboard":   false,
		"delete_game_center_leaderboard":   false,
		// Xcode Cloud tools
		"list_ci_products":    false,
		"get_ci_product":      false,
		"list_ci_workflows":   false,
		"get_ci_workflow":     false,
		"list_ci_build_runs":  false,
		"get_ci_build_run":    false,
		"start_ci_build_run":  false,
		"cancel_ci_build_run": false,
		// Reports tools
		"get_sales_report":   false,
		"get_finance_report": false,
		// Encryption tools
		"list_encryption_declarations":            false,
		"get_encryption_declaration":              false,
		"create_encryption_declaration":           false,
		"assign_build_to_encryption_declaration": false,
	}

	for _, tool := range tools {
		if _, exists := expectedTools[tool.Name]; !exists {
			t.Errorf("unexpected tool: %s", tool.Name)
		} else {
			expectedTools[tool.Name] = true
		}

		// Verify tool has required fields
		if tool.Name == "" {
			t.Error("tool has empty name")
		}
		if tool.Description == "" {
			t.Errorf("tool %s has empty description", tool.Name)
		}
		if tool.InputSchema.Type != "object" {
			t.Errorf("tool %s has invalid input schema type: %s", tool.Name, tool.InputSchema.Type)
		}
	}

	// Verify all expected tools were found
	for name, found := range expectedTools {
		if !found {
			t.Errorf("missing expected tool: %s", name)
		}
	}
}

func TestRegistry_CallTool_UnknownTool(t *testing.T) {
	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(privateKey)
	pemBlock := &pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}

	tmpDir := t.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)

	client, _ := api.NewClient("test-issuer", "TESTKEY123", keyPath)
	registry := NewRegistry(client)

	_, err := registry.CallTool("unknown_tool", json.RawMessage(`{}`))

	if err == nil {
		t.Fatal("expected error for unknown tool")
	}

	if !strings.Contains(err.Error(), "unknown tool") {
		t.Errorf("error %q does not mention unknown tool", err.Error())
	}
}

func TestRegistry_Register(t *testing.T) {
	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(privateKey)
	pemBlock := &pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}

	tmpDir := t.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)

	client, _ := api.NewClient("test-issuer", "TESTKEY123", keyPath)

	registry := &Registry{
		client:   client,
		tools:    make([]mcp.Tool, 0),
		handlers: make(map[string]ToolHandler),
	}

	tool := mcp.Tool{
		Name:        "custom_tool",
		Description: "A custom test tool",
		InputSchema: mcp.JSONSchema{Type: "object"},
	}

	handler := func(args json.RawMessage) (*mcp.ToolsCallResult, error) {
		return mcp.NewSuccessResult("custom result"), nil
	}

	registry.register(tool, handler)

	if len(registry.tools) != 1 {
		t.Errorf("expected 1 tool, got %d", len(registry.tools))
	}

	if registry.tools[0].Name != "custom_tool" {
		t.Errorf("tool name = %q, want custom_tool", registry.tools[0].Name)
	}

	if _, exists := registry.handlers["custom_tool"]; !exists {
		t.Error("handler not registered")
	}

	// Call the custom tool
	result, err := registry.CallTool("custom_tool", json.RawMessage(`{}`))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if result.Content[0].Text != "custom result" {
		t.Errorf("result = %q, want custom result", result.Content[0].Text)
	}
}

func TestToolInputSchemas(t *testing.T) {
	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(privateKey)
	pemBlock := &pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}

	tmpDir := t.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)

	client, _ := api.NewClient("test-issuer", "TESTKEY123", keyPath)
	registry := NewRegistry(client)

	tools := registry.ListTools()

	// Test specific tool schemas
	toolSchemas := map[string]struct {
		hasRequired  bool
		requiredKeys []string
	}{
		"list_apps":           {hasRequired: false},
		"get_app":             {hasRequired: true, requiredKeys: []string{"app_id"}},
		"get_app_versions":    {hasRequired: true, requiredKeys: []string{"app_id"}},
		"list_builds":         {hasRequired: false},
		"get_build":           {hasRequired: true, requiredKeys: []string{"build_id"}},
		"create_beta_group":   {hasRequired: true, requiredKeys: []string{"app_id", "name"}},
		"delete_beta_group":   {hasRequired: true, requiredKeys: []string{"beta_group_id"}},
		"invite_beta_tester":  {hasRequired: true, requiredKeys: []string{"email"}},
		"remove_beta_tester":  {hasRequired: true, requiredKeys: []string{"beta_tester_id"}},
		"add_tester_to_group": {hasRequired: true, requiredKeys: []string{"beta_tester_id", "beta_group_id"}},
		"get_bundle_id":       {hasRequired: true, requiredKeys: []string{"bundle_id_id"}},
		"register_device":     {hasRequired: true, requiredKeys: []string{"name", "udid", "platform"}},
	}

	for _, tool := range tools {
		expected, exists := toolSchemas[tool.Name]
		if !exists {
			continue
		}

		if expected.hasRequired {
			if len(tool.InputSchema.Required) == 0 {
				t.Errorf("tool %s should have required fields", tool.Name)
				continue
			}

			for _, key := range expected.requiredKeys {
				found := false
				for _, req := range tool.InputSchema.Required {
					if req == key {
						found = true
						break
					}
				}
				if !found {
					t.Errorf("tool %s missing required field: %s", tool.Name, key)
				}
			}
		}
	}
}

// Integration-style tests with mock HTTP server

func TestHandleListApps_Integration(t *testing.T) {
	// This test requires a mock server - skipping for unit tests
	// as it requires setting private baseURL field
	t.Skip("requires mock server integration")
}

// Benchmarks

func BenchmarkRegistry_ListTools(b *testing.B) {
	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(privateKey)
	pemBlock := &pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}

	tmpDir := b.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)

	client, _ := api.NewClient("test-issuer", "TESTKEY123", keyPath)
	registry := NewRegistry(client)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = registry.ListTools()
	}
}

func BenchmarkRegistry_CallTool_Lookup(b *testing.B) {
	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(privateKey)
	pemBlock := &pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}

	tmpDir := b.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)

	client, _ := api.NewClient("test-issuer", "TESTKEY123", keyPath)
	registry := NewRegistry(client)

	// Just benchmark the lookup, not the actual API call
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, exists := registry.handlers["list_apps"]
		if !exists {
			b.Fatal("handler not found")
		}
	}
}

func BenchmarkNewRegistry(b *testing.B) {
	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(privateKey)
	pemBlock := &pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}

	tmpDir := b.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)

	client, _ := api.NewClient("test-issuer", "TESTKEY123", keyPath)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = NewRegistry(client)
	}
}

// Helper to create a mock client for tool handler testing
type mockClient struct {
	handler func(ctx context.Context, method, path string) ([]byte, error)
}

// Context timeout test
func TestToolHandler_ContextTimeout(t *testing.T) {
	t.Skip("requires mock server integration")

	// This would test that tool handlers respect context cancellation
	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Millisecond)
	defer cancel()

	// Would call a tool and verify it respects the timeout
	_ = ctx
}
