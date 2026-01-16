package server

import (
	"bytes"
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

	"github.com/antisynthesis/asc-mcp/internal/asc/config"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
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

	if server.initialized {
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

	if !server.initialized {
		t.Error("server should be initialized after initialize request")
	}

	// Check result structure
	resultJSON, _ := json.Marshal(resp.Result)
	var result mcp.InitializeResult
	if err := json.Unmarshal(resultJSON, &result); err != nil {
		t.Fatalf("failed to unmarshal result: %v", err)
	}

	if result.ProtocolVersion != mcp.ProtocolVersion {
		t.Errorf("ProtocolVersion = %q, want %q", result.ProtocolVersion, mcp.ProtocolVersion)
	}

	if result.ServerInfo.Name != serverName {
		t.Errorf("ServerInfo.Name = %q, want %q", result.ServerInfo.Name, serverName)
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
	server.initialized = true

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

	// Should have 200 tools
	if len(result.Tools) != 200 {
		t.Errorf("expected 200 tools, got %d", len(result.Tools))
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
	server.initialized = true

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
