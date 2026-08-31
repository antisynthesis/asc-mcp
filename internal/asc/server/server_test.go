package server

import (
	"bytes"
	"context"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"crypto/x509"
	"encoding/json"
	"encoding/pem"
	"io"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"testing"
	"time"

	"github.com/antisynthesis/asc-mcp/internal/asc/config"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
	"github.com/antisynthesis/asc-mcp/internal/asc/tools"
)

// testSetup creates a test configuration with temporary key file.
func testSetup(t *testing.T) *config.Config {
	t.Helper()

	// Generate test key
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

	return &config.Config{
		IssuerID:       "test-issuer",
		KeyID:          "TESTKEY123",
		PrivateKeyPath: keyPath,
	}
}

// sendRequest sends a JSON-RPC request and returns the response.
func sendRequest(t *testing.T, s *Server, req mcp.Request) mcp.Response {
	t.Helper()

	// Create pipes for communication
	inputReader, inputWriter := io.Pipe()
	outputReader, outputWriter := io.Pipe()

	// Replace server's reader and writer
	s.reader = nil // Will be set by writing to inputWriter
	s.writer = outputWriter

	// Send request in goroutine
	go func() {
		data, _ := json.Marshal(req)
		data = append(data, '\n')
		inputWriter.Write(data)
		inputWriter.Close()
	}()

	// Read response
	var resp mcp.Response
	decoder := json.NewDecoder(outputReader)
	if err := decoder.Decode(&resp); err != nil && err != io.EOF {
		t.Fatalf("failed to decode response: %v", err)
	}

	inputReader.Close()
	outputWriter.Close()

	return resp
}

func TestNew(t *testing.T) {
	cfg := testSetup(t)

	input := bytes.NewReader(nil)
	output := &bytes.Buffer{}

	server, err := New(cfg, input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	if server == nil {
		t.Fatal("expected server, got nil")
	}

	if server.cfg != cfg {
		t.Error("config not set correctly")
	}

	if server.client == nil {
		t.Error("client not initialized")
	}

	if server.registry == nil {
		t.Error("registry not initialized")
	}

	if server.initialized() {
		t.Error("server should not be initialized")
	}
}

func TestServer_HandleInitialize(t *testing.T) {
	cfg := testSetup(t)

	input := &bytes.Buffer{}
	output := &bytes.Buffer{}

	server, err := New(cfg, input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "initialize",
		Params: json.RawMessage(`{
			"protocolVersion": "2024-11-05",
			"capabilities": {},
			"clientInfo": {"name": "test-client", "version": "1.0.0"}
		}`),
	}

	server.handleRequest(&req)

	// Parse response
	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp.Error != nil {
		t.Fatalf("unexpected error: %v", resp.Error)
	}

	if !server.initialized() {
		t.Error("server should be initialized after initialize request")
	}

	// Check result structure
	resultJSON, _ := json.Marshal(resp.Result)
	var result mcp.InitializeResult
	if err := json.Unmarshal(resultJSON, &result); err != nil {
		t.Fatalf("failed to unmarshal result: %v", err)
	}

	// Client requested 2024-11-05; server should echo that back since it is
	// in SupportedProtocolVersions.
	if result.ProtocolVersion != "2024-11-05" {
		t.Errorf("ProtocolVersion = %q, want 2024-11-05", result.ProtocolVersion)
	}

	if result.ServerInfo.Name != serverName {
		t.Errorf("ServerInfo.Name = %q, want %q", result.ServerInfo.Name, serverName)
	}

	if result.Instructions == "" {
		t.Error("expected Instructions to be set in initialize result")
	}
}

func TestServer_HandleInitialize_NegotiatesLatest(t *testing.T) {
	cfg := testSetup(t)

	output := &bytes.Buffer{}
	server, err := New(cfg, &bytes.Buffer{}, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "initialize",
		Params: json.RawMessage(`{
			"protocolVersion": "2099-01-01",
			"capabilities": {},
			"clientInfo": {"name": "test-client", "version": "1.0.0"}
		}`),
	}
	server.handleRequest(&req)

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}
	resultJSON, _ := json.Marshal(resp.Result)
	var result mcp.InitializeResult
	if err := json.Unmarshal(resultJSON, &result); err != nil {
		t.Fatalf("failed to unmarshal result: %v", err)
	}
	if result.ProtocolVersion != mcp.ProtocolVersion {
		t.Errorf("unknown version should fall back to latest %q, got %q", mcp.ProtocolVersion, result.ProtocolVersion)
	}
}

func TestServer_HandlePing(t *testing.T) {
	cfg := testSetup(t)
	output := &bytes.Buffer{}
	server, err := New(cfg, &bytes.Buffer{}, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`42`),
		Method:  "ping",
	}
	server.handleRequest(&req)

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}
	if resp.Error != nil {
		t.Fatalf("unexpected error: %v", resp.Error)
	}
	if resp.Result == nil {
		t.Fatal("expected empty result object for ping")
	}
}

func TestServer_HandleToolsList(t *testing.T) {
	cfg := testSetup(t)

	input := &bytes.Buffer{}
	output := &bytes.Buffer{}

	server, err := New(cfg, input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	// First initialize the server
	server.session.initialized.Store(true)

	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`2`),
		Method:  "tools/list",
	}

	server.handleRequest(&req)

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp.Error != nil {
		t.Fatalf("unexpected error: %v", resp.Error)
	}

	// Check that tools are returned
	resultJSON, _ := json.Marshal(resp.Result)
	var result mcp.ToolsListResult
	if err := json.Unmarshal(resultJSON, &result); err != nil {
		t.Fatalf("failed to unmarshal result: %v", err)
	}

	if len(result.Tools) == 0 {
		t.Error("expected tools to be returned")
	}

	// Should have 356 tools (221 base + 3 upload tools + 6 build upload
	// tools + 8 beta feedback tools + 10 background asset tools + 53
	// Game Center content tools + 59 commerce tools).
	if len(result.Tools) != 356 {
		t.Errorf("expected 356 tools, got %d", len(result.Tools))
	}
}

func TestServer_HandleToolsList_NotInitialized(t *testing.T) {
	cfg := testSetup(t)

	input := &bytes.Buffer{}
	output := &bytes.Buffer{}

	server, err := New(cfg, input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	// Don't initialize

	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "tools/list",
	}

	server.handleRequest(&req)

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp.Error == nil {
		t.Fatal("expected error for uninitialized server")
	}

	if resp.Error.Code != mcp.ErrCodeInvalidRequest {
		t.Errorf("Error.Code = %d, want %d", resp.Error.Code, mcp.ErrCodeInvalidRequest)
	}
}

func TestServer_HandleToolsCall_NotInitialized(t *testing.T) {
	cfg := testSetup(t)

	input := &bytes.Buffer{}
	output := &bytes.Buffer{}

	server, err := New(cfg, input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "tools/call",
		Params:  json.RawMessage(`{"name": "list_apps"}`),
	}

	server.handleRequest(&req)

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp.Error == nil {
		t.Fatal("expected error for uninitialized server")
	}
}

func TestServer_HandleRequest_InvalidJSONRPC(t *testing.T) {
	cfg := testSetup(t)

	input := &bytes.Buffer{}
	output := &bytes.Buffer{}

	server, err := New(cfg, input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	req := mcp.Request{
		JSONRPC: "1.0", // Invalid version
		ID:      json.RawMessage(`1`),
		Method:  "initialize",
	}

	server.handleRequest(&req)

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp.Error == nil {
		t.Fatal("expected error for invalid jsonrpc version")
	}

	if resp.Error.Code != mcp.ErrCodeInvalidRequest {
		t.Errorf("Error.Code = %d, want %d", resp.Error.Code, mcp.ErrCodeInvalidRequest)
	}
}

func TestServer_HandleRequest_MethodNotFound(t *testing.T) {
	cfg := testSetup(t)

	input := &bytes.Buffer{}
	output := &bytes.Buffer{}

	server, err := New(cfg, input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "unknown/method",
	}

	server.handleRequest(&req)

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp.Error == nil {
		t.Fatal("expected error for unknown method")
	}

	if resp.Error.Code != mcp.ErrCodeMethodNotFound {
		t.Errorf("Error.Code = %d, want %d", resp.Error.Code, mcp.ErrCodeMethodNotFound)
	}
}

func TestServer_SendResult(t *testing.T) {
	cfg := testSetup(t)

	input := &bytes.Buffer{}
	output := &bytes.Buffer{}

	server, err := New(cfg, input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	server.sendResult(json.RawMessage(`1`), map[string]string{"status": "ok"})

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp.JSONRPC != mcp.JSONRPCVersion {
		t.Errorf("JSONRPC = %q, want %q", resp.JSONRPC, mcp.JSONRPCVersion)
	}

	if resp.Error != nil {
		t.Errorf("unexpected error: %v", resp.Error)
	}

	if resp.Result == nil {
		t.Error("expected result")
	}
}

func TestServer_SendError(t *testing.T) {
	cfg := testSetup(t)

	input := &bytes.Buffer{}
	output := &bytes.Buffer{}

	server, err := New(cfg, input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	server.sendError(json.RawMessage(`1`), mcp.ErrCodeInternal, "Internal error", "details")

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp.Error == nil {
		t.Fatal("expected error")
	}

	if resp.Error.Code != mcp.ErrCodeInternal {
		t.Errorf("Error.Code = %d, want %d", resp.Error.Code, mcp.ErrCodeInternal)
	}

	if resp.Error.Message != "Internal error" {
		t.Errorf("Error.Message = %q, want Internal error", resp.Error.Message)
	}
}

func TestServer_Run_ParseError(t *testing.T) {
	cfg := testSetup(t)

	// Send invalid JSON
	input := bytes.NewReader([]byte("not valid json\n"))
	output := &bytes.Buffer{}

	server, err := New(cfg, input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	// Run should handle the parse error and continue (then EOF)
	_ = server.Run()

	// Check that a parse error was sent
	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp.Error == nil {
		t.Fatal("expected parse error")
	}

	if resp.Error.Code != mcp.ErrCodeParse {
		t.Errorf("Error.Code = %d, want %d", resp.Error.Code, mcp.ErrCodeParse)
	}
}

func TestServer_Run_EmptyLines(t *testing.T) {
	cfg := testSetup(t)

	// Send empty lines followed by valid request
	input := strings.NewReader("\n\n\n")
	output := &bytes.Buffer{}

	server, err := New(cfg, input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	// Should handle empty lines gracefully and exit on EOF
	err = server.Run()
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
}

func TestServer_ConcurrentWrites(t *testing.T) {
	cfg := testSetup(t)

	input := &bytes.Buffer{}
	output := &bytes.Buffer{}

	server, err := New(cfg, input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	// Simulate concurrent writes
	var wg sync.WaitGroup
	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			server.sendResult(json.RawMessage(`1`), map[string]int{"id": id})
		}(i)
	}

	wg.Wait()

	// All writes should have completed without panic
	// The output should contain 10 JSON objects
	responses := strings.Split(strings.TrimSpace(output.String()), "\n")
	if len(responses) != 10 {
		t.Errorf("expected 10 responses, got %d", len(responses))
	}
}

func TestServer_NotificationsInitialized(t *testing.T) {
	cfg := testSetup(t)

	input := &bytes.Buffer{}
	output := &bytes.Buffer{}

	server, err := New(cfg, input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	// notifications/initialized is a notification (no response expected)
	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		Method:  "notifications/initialized",
	}

	server.handleRequest(&req)

	// No response should be sent for notifications
	if output.Len() != 0 {
		t.Error("expected no response for notification")
	}
}

// modernMeta builds tools-era params carrying the required 2026-07-28
// _meta keys.
func modernMeta(protocolVersion string, withCaps bool) json.RawMessage {
	meta := map[string]any{
		mcp.MetaProtocolVersion: protocolVersion,
	}
	if withCaps {
		meta[mcp.MetaClientCapabilities] = map[string]any{}
	}
	params, _ := json.Marshal(map[string]any{"_meta": meta})
	return params
}

func TestServer_Discover_PreInitialize(t *testing.T) {
	cfg := testSetup(t)

	// server/discover must work as the FIRST message on stdin, before
	// initialize, so exercise it through Run rather than handleRequest.
	line, _ := json.Marshal(mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "server/discover",
	})
	input := bytes.NewReader(append(line, '\n'))
	output := &bytes.Buffer{}

	server, err := New(cfg, input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}
	if err := server.Run(); err != nil {
		t.Fatalf("Run failed: %v", err)
	}

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}
	if resp.Error != nil {
		t.Fatalf("unexpected error: %v", resp.Error)
	}
	if server.initialized() {
		t.Error("server/discover must not mark the session initialized")
	}

	resultJSON, _ := json.Marshal(resp.Result)
	var result mcp.DiscoverResult
	if err := json.Unmarshal(resultJSON, &result); err != nil {
		t.Fatalf("failed to unmarshal result: %v", err)
	}

	for _, want := range []string{"2026-07-28", "2025-06-18"} {
		found := false
		for _, v := range result.SupportedVersions {
			if v == want {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("supportedVersions missing %q: %v", want, result.SupportedVersions)
		}
	}
	if result.Capabilities.Tools == nil {
		t.Error("expected capabilities.tools to be present")
	}
	if result.ResultType != mcp.ResultTypeComplete {
		t.Errorf("resultType = %q, want %q", result.ResultType, mcp.ResultTypeComplete)
	}
	if result.TtlMs <= 0 {
		t.Errorf("ttlMs = %d, want > 0", result.TtlMs)
	}
	if result.CacheScope != mcp.CacheScopePublic {
		t.Errorf("cacheScope = %q, want public", result.CacheScope)
	}
	info, ok := result.Meta[mcp.MetaServerInfo].(map[string]any)
	if !ok {
		t.Fatalf("_meta[%q] missing or wrong type: %v", mcp.MetaServerInfo, result.Meta)
	}
	if info["name"] != serverName {
		t.Errorf("_meta serverInfo name = %v, want %q", info["name"], serverName)
	}
}

func TestServer_ModernToolsList_NoHandshake(t *testing.T) {
	cfg := testSetup(t)
	output := &bytes.Buffer{}
	server, err := New(cfg, &bytes.Buffer{}, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	// No initialize: modern requests are stateless and must be served
	// regardless of handshake state.
	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "tools/list",
		Params:  modernMeta("2026-07-28", true),
	}
	server.handleRequest(&req)

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}
	if resp.Error != nil {
		t.Fatalf("unexpected error: %v", resp.Error)
	}

	resultJSON, _ := json.Marshal(resp.Result)
	var result mcp.ToolsListResult
	if err := json.Unmarshal(resultJSON, &result); err != nil {
		t.Fatalf("failed to unmarshal result: %v", err)
	}
	if len(result.Tools) == 0 {
		t.Error("expected tools to be returned")
	}
	if result.ResultType != mcp.ResultTypeComplete {
		t.Errorf("resultType = %q, want %q", result.ResultType, mcp.ResultTypeComplete)
	}
	if result.TtlMs == nil {
		t.Error("expected ttlMs to be set on modern tools/list")
	}
	if result.CacheScope != mcp.CacheScopePublic {
		t.Errorf("cacheScope = %q, want public", result.CacheScope)
	}
}

func TestServer_Modern_UnsupportedProtocolVersion(t *testing.T) {
	cfg := testSetup(t)
	output := &bytes.Buffer{}
	server, err := New(cfg, &bytes.Buffer{}, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	// A legacy revision stamped into _meta marks the request modern, but
	// only the latest revision is valid there.
	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "tools/list",
		Params:  modernMeta("2025-06-18", true),
	}
	server.handleRequest(&req)

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}
	if resp.Error == nil {
		t.Fatal("expected error for unsupported protocol version")
	}
	if resp.Error.Code != mcp.ErrCodeUnsupportedProtocolVersion {
		t.Errorf("Error.Code = %d, want %d", resp.Error.Code, mcp.ErrCodeUnsupportedProtocolVersion)
	}

	dataJSON, _ := json.Marshal(resp.Error.Data)
	var data mcp.UnsupportedProtocolVersionData
	if err := json.Unmarshal(dataJSON, &data); err != nil {
		t.Fatalf("failed to unmarshal error data: %v", err)
	}
	if data.Requested != "2025-06-18" {
		t.Errorf("data.requested = %q, want 2025-06-18", data.Requested)
	}
	if len(data.Supported) != len(mcp.SupportedProtocolVersions) {
		t.Fatalf("data.supported = %v, want %v", data.Supported, mcp.SupportedProtocolVersions)
	}
	for i, v := range mcp.SupportedProtocolVersions {
		if data.Supported[i] != v {
			t.Errorf("data.supported[%d] = %q, want %q", i, data.Supported[i], v)
		}
	}
}

func TestServer_Modern_MissingClientCapabilities(t *testing.T) {
	cfg := testSetup(t)
	output := &bytes.Buffer{}
	server, err := New(cfg, &bytes.Buffer{}, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "tools/list",
		Params:  modernMeta("2026-07-28", false),
	}
	server.handleRequest(&req)

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}
	if resp.Error == nil {
		t.Fatal("expected error for missing clientCapabilities")
	}
	if resp.Error.Code != mcp.ErrCodeInvalidParams {
		t.Errorf("Error.Code = %d, want %d", resp.Error.Code, mcp.ErrCodeInvalidParams)
	}
}

func TestServer_Modern_PingRemoved(t *testing.T) {
	cfg := testSetup(t)
	output := &bytes.Buffer{}
	server, err := New(cfg, &bytes.Buffer{}, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	// ping was removed in 2026-07-28; a modern-marked ping is
	// method-not-found even though the legacy path still answers it.
	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "ping",
		Params:  modernMeta("2026-07-28", true),
	}
	server.handleRequest(&req)

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}
	if resp.Error == nil {
		t.Fatal("expected error for modern ping")
	}
	if resp.Error.Code != mcp.ErrCodeMethodNotFound {
		t.Errorf("Error.Code = %d, want %d", resp.Error.Code, mcp.ErrCodeMethodNotFound)
	}
}

func TestServer_HandleInitialize_ModernVersionFallsBack(t *testing.T) {
	cfg := testSetup(t)
	output := &bytes.Buffer{}
	server, err := New(cfg, &bytes.Buffer{}, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	// 2026-07-28 has no initialize handshake, so a client proposing it
	// there negotiates down to the newest legacy version.
	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "initialize",
		Params: json.RawMessage(`{
			"protocolVersion": "2026-07-28",
			"capabilities": {},
			"clientInfo": {"name": "test-client", "version": "1.0.0"}
		}`),
	}
	server.handleRequest(&req)

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}
	resultJSON, _ := json.Marshal(resp.Result)
	var result mcp.InitializeResult
	if err := json.Unmarshal(resultJSON, &result); err != nil {
		t.Fatalf("failed to unmarshal result: %v", err)
	}
	if result.ProtocolVersion != "2025-11-25" {
		t.Errorf("ProtocolVersion = %q, want 2025-11-25", result.ProtocolVersion)
	}
}

func TestServer_LegacyToolsList_NoModernKeys(t *testing.T) {
	cfg := testSetup(t)
	output := &bytes.Buffer{}
	server, err := New(cfg, &bytes.Buffer{}, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}
	server.session.initialized.Store(true)

	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "tools/list",
	}
	server.handleRequest(&req)

	// Regression guard: the legacy wire shape must stay byte-identical,
	// so none of the 2026-07-28 result fields may leak through omitempty.
	raw := output.String()
	for _, key := range []string{`"resultType"`, `"ttlMs"`, `"cacheScope"`} {
		if strings.Contains(raw, key) {
			t.Errorf("legacy tools/list response contains modern key %s", key)
		}
	}
}

// registerFakeTool installs a test-only tool handler on the server's
// registry so cancellation and concurrency can be exercised without
// hitting the App Store Connect API.
func registerFakeTool(s *Server, name string, handler tools.ToolHandler) {
	s.registry.Register(mcp.Tool{
		Name:        name,
		Description: "test-only tool",
		InputSchema: mcp.JSONSchema{Type: "object"},
	}, handler)
}

// writeRequestLine marshals a request and writes it as one stdio line.
func writeRequestLine(t *testing.T, w io.Writer, req mcp.Request) {
	t.Helper()
	data, err := json.Marshal(req)
	if err != nil {
		t.Fatalf("failed to marshal request: %v", err)
	}
	if _, err := w.Write(append(data, '\n')); err != nil {
		t.Fatalf("failed to write request: %v", err)
	}
}

// waitForRun waits for the Run goroutine to finish and fails the test if
// it errors or hangs.
func waitForRun(t *testing.T, done <-chan error) {
	t.Helper()
	select {
	case err := <-done:
		if err != nil {
			t.Fatalf("Run failed: %v", err)
		}
	case <-time.After(10 * time.Second):
		t.Fatal("Run did not return after EOF")
	}
}

func TestServer_Run_CancelledRequest_NoResponse(t *testing.T) {
	cfg := testSetup(t)

	inputReader, inputWriter := io.Pipe()
	output := &bytes.Buffer{}

	server, err := New(cfg, inputReader, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}
	server.session.initialized.Store(true)

	started := make(chan struct{})
	registerFakeTool(server, "slow_tool", func(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
		close(started)
		<-ctx.Done()
		// Completing "successfully" after cancellation must still be
		// suppressed: no message may be sent for a cancelled request.
		return mcp.NewSuccessResult("finished after cancel"), nil
	})

	done := make(chan error, 1)
	go func() { done <- server.Run() }()

	writeRequestLine(t, inputWriter, mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`7`),
		Method:  "tools/call",
		Params:  json.RawMessage(`{"name":"slow_tool"}`),
	})

	// The read loop must stay free while the tool call is in flight,
	// otherwise the cancellation below could never be read in time.
	select {
	case <-started:
	case <-time.After(5 * time.Second):
		t.Fatal("tool call never started; requests are not dispatched concurrently")
	}

	writeRequestLine(t, inputWriter, mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		Method:  "notifications/cancelled",
		Params:  json.RawMessage(`{"requestId":7,"reason":"user gave up"}`),
	})
	inputWriter.Close()

	waitForRun(t, done)

	// The server MUST NOT send further messages (including the response)
	// for the cancelled request, and the notification itself gets none.
	if got := output.String(); got != "" {
		t.Errorf("expected no output for cancelled request, got %q", got)
	}
}

func TestServer_Run_CancelUnknownID_Ignored(t *testing.T) {
	cfg := testSetup(t)

	// Unknown and malformed cancellations SHOULD be ignored: no error
	// response, no output at all.
	var input bytes.Buffer
	for _, params := range []string{
		`{"requestId":999,"reason":"never existed"}`,
		`{"reason":"missing requestId"}`,
		`{`,
	} {
		line, _ := json.Marshal(mcp.Request{
			JSONRPC: mcp.JSONRPCVersion,
			Method:  "notifications/cancelled",
			Params:  json.RawMessage(params),
		})
		input.Write(append(line, '\n'))
	}
	output := &bytes.Buffer{}

	server, err := New(cfg, &input, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}
	if err := server.Run(); err != nil {
		t.Fatalf("Run failed: %v", err)
	}
	if output.Len() != 0 {
		t.Errorf("expected no output for ignored cancellations, got %q", output.String())
	}
}

func TestServer_Run_CancelIDTypeMismatch(t *testing.T) {
	cfg := testSetup(t)

	inputReader, inputWriter := io.Pipe()
	output := &bytes.Buffer{}

	server, err := New(cfg, inputReader, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}
	server.session.initialized.Store(true)

	started := make(chan struct{})
	release := make(chan struct{})
	registerFakeTool(server, "slow_tool", func(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
		close(started)
		select {
		case <-ctx.Done():
			return mcp.NewErrorResult("cancelled"), nil
		case <-release:
			return mcp.NewSuccessResult("completed"), nil
		}
	})

	done := make(chan error, 1)
	go func() { done <- server.Run() }()

	writeRequestLine(t, inputWriter, mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`7`),
		Method:  "tools/call",
		Params:  json.RawMessage(`{"name":"slow_tool"}`),
	})
	select {
	case <-started:
	case <-time.After(5 * time.Second):
		t.Fatal("tool call never started")
	}

	// The string id "7" must not match the in-flight number id 7.
	writeRequestLine(t, inputWriter, mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		Method:  "notifications/cancelled",
		Params:  json.RawMessage(`{"requestId":"7"}`),
	})
	close(release)
	inputWriter.Close()

	waitForRun(t, done)

	var resp mcp.Response
	if err := json.NewDecoder(output).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}
	if resp.Error != nil {
		t.Fatalf("unexpected error: %v", resp.Error)
	}
	resultJSON, _ := json.Marshal(resp.Result)
	var result mcp.ToolsCallResult
	if err := json.Unmarshal(resultJSON, &result); err != nil {
		t.Fatalf("failed to unmarshal result: %v", err)
	}
	if result.IsError {
		t.Errorf("string requestId %q cancelled the number-id request: %v", "7", result.Content)
	}
}

func TestServer_Run_ConcurrentRequests(t *testing.T) {
	cfg := testSetup(t)

	inputReader, inputWriter := io.Pipe()
	output := &bytes.Buffer{}

	server, err := New(cfg, inputReader, output)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}
	server.session.initialized.Store(true)

	// first_tool refuses to finish until second_tool has started, so both
	// responses arriving proves the requests interleaved.
	secondStarted := make(chan struct{})
	registerFakeTool(server, "first_tool", func(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
		select {
		case <-secondStarted:
			return mcp.NewSuccessResult("first"), nil
		case <-time.After(5 * time.Second):
			return mcp.NewErrorResult("second request never started; dispatch is sequential"), nil
		}
	})
	registerFakeTool(server, "second_tool", func(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
		close(secondStarted)
		return mcp.NewSuccessResult("second"), nil
	})

	done := make(chan error, 1)
	go func() { done <- server.Run() }()

	writeRequestLine(t, inputWriter, mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "tools/call",
		Params:  json.RawMessage(`{"name":"first_tool"}`),
	})
	writeRequestLine(t, inputWriter, mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`2`),
		Method:  "tools/call",
		Params:  json.RawMessage(`{"name":"second_tool"}`),
	})
	inputWriter.Close()

	waitForRun(t, done)

	// Both requests must respond; responses may arrive in either order.
	lines := strings.Split(strings.TrimSpace(output.String()), "\n")
	if len(lines) != 2 {
		t.Fatalf("expected 2 responses, got %d: %q", len(lines), output.String())
	}
	seen := make(map[string]bool)
	for _, line := range lines {
		var resp mcp.Response
		if err := json.Unmarshal([]byte(line), &resp); err != nil {
			t.Fatalf("failed to decode response line %q: %v", line, err)
		}
		if resp.Error != nil {
			t.Errorf("unexpected error for id %s: %v", resp.ID, resp.Error)
		}
		resultJSON, _ := json.Marshal(resp.Result)
		var result mcp.ToolsCallResult
		if err := json.Unmarshal(resultJSON, &result); err != nil {
			t.Fatalf("failed to unmarshal result: %v", err)
		}
		if result.IsError {
			t.Errorf("tool call id %s failed: %v", resp.ID, result.Content)
		}
		seen[string(resp.ID)] = true
	}
	for _, id := range []string{"1", "2"} {
		if !seen[id] {
			t.Errorf("no response for request id %s; got %v", id, seen)
		}
	}
}

func TestDispatcher_ContextCancellationAbortsToolCall(t *testing.T) {
	cfg := testSetup(t)

	// The HTTP transport cancels by closing the connection: r.Context()
	// flows into Dispatch and from there into the tool call. This models
	// that path at the Dispatcher level.
	server, err := New(cfg, &bytes.Buffer{}, &bytes.Buffer{})
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}
	server.session.initialized.Store(true)

	started := make(chan struct{})
	registerFakeTool(server, "hang_tool", func(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
		close(started)
		<-ctx.Done()
		return nil, ctx.Err()
	})

	ctx, cancel := context.WithCancel(context.Background())
	go func() {
		<-started
		cancel()
	}()

	respCh := make(chan *mcp.Response, 1)
	go func() {
		respCh <- server.dispatcher.Dispatch(ctx, &mcp.Request{
			JSONRPC: mcp.JSONRPCVersion,
			ID:      json.RawMessage(`1`),
			Method:  "tools/call",
			Params:  json.RawMessage(`{"name":"hang_tool"}`),
		}, server.session)
	}()

	select {
	case resp := <-respCh:
		if resp == nil {
			t.Fatal("expected a response")
		}
		if resp.Error != nil {
			t.Fatalf("unexpected protocol error: %v", resp.Error)
		}
		resultJSON, _ := json.Marshal(resp.Result)
		var result mcp.ToolsCallResult
		if err := json.Unmarshal(resultJSON, &result); err != nil {
			t.Fatalf("failed to unmarshal result: %v", err)
		}
		if !result.IsError {
			t.Error("expected isError result for a cancelled tool call")
		}
	case <-time.After(5 * time.Second):
		t.Fatal("Dispatch did not return after context cancellation; HTTP disconnects would not abort tool calls")
	}
}

// Benchmarks

func BenchmarkServer_HandleInitialize(b *testing.B) {
	// Setup
	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(privateKey)
	pemBlock := &pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}

	tmpDir := b.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)

	cfg := &config.Config{
		IssuerID:       "test-issuer",
		KeyID:          "TESTKEY123",
		PrivateKeyPath: keyPath,
	}

	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "initialize",
		Params:  json.RawMessage(`{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}`),
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		output := &bytes.Buffer{}
		server, _ := New(cfg, &bytes.Buffer{}, output)
		server.handleRequest(&req)
	}
}

func BenchmarkServer_HandleToolsList(b *testing.B) {
	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(privateKey)
	pemBlock := &pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}

	tmpDir := b.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)

	cfg := &config.Config{
		IssuerID:       "test-issuer",
		KeyID:          "TESTKEY123",
		PrivateKeyPath: keyPath,
	}

	output := &bytes.Buffer{}
	server, _ := New(cfg, &bytes.Buffer{}, output)
	server.session.initialized.Store(true)

	req := mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "tools/list",
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		output.Reset()
		server.handleRequest(&req)
	}
}

func BenchmarkServer_SendResult(b *testing.B) {
	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(privateKey)
	pemBlock := &pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}

	tmpDir := b.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)

	cfg := &config.Config{
		IssuerID:       "test-issuer",
		KeyID:          "TESTKEY123",
		PrivateKeyPath: keyPath,
	}

	output := &bytes.Buffer{}
	server, _ := New(cfg, &bytes.Buffer{}, output)

	result := map[string]string{"status": "ok", "message": "success"}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		output.Reset()
		server.sendResult(json.RawMessage(`1`), result)
	}
}
