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
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"

	dto "github.com/prometheus/client_model/go"

	"github.com/antisynthesis/asc-mcp/internal/asc/config"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

func newHTTPTestServer(t *testing.T) *httptest.Server {
	t.Helper()
	privateKey, err := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		t.Fatalf("generate key: %v", err)
	}
	keyBytes, err := x509.MarshalPKCS8PrivateKey(privateKey)
	if err != nil {
		t.Fatalf("marshal key: %v", err)
	}
	tmpDir := t.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	if err := os.WriteFile(keyPath, pem.EncodeToMemory(&pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}), 0o600); err != nil {
		t.Fatalf("write key: %v", err)
	}
	cfg := &config.Config{IssuerID: "iss", KeyID: "kid", PrivateKeyPath: keyPath}
	hs, err := NewHTTPServer(cfg, HTTPOptions{})
	if err != nil {
		t.Fatalf("new http server: %v", err)
	}
	return httptest.NewServer(hs.Handler())
}

func postJSON(t *testing.T, url string, body any, headers map[string]string) *http.Response {
	t.Helper()
	data, _ := json.Marshal(body)
	req, _ := http.NewRequest(http.MethodPost, url, bytes.NewReader(data))
	req.Header.Set("Content-Type", "application/json")
	for k, v := range headers {
		req.Header.Set(k, v)
	}
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("post: %v", err)
	}
	return resp
}

func decodeMCPResponse(t *testing.T, resp *http.Response) mcp.Response {
	t.Helper()
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	var out mcp.Response
	if err := json.Unmarshal(body, &out); err != nil {
		t.Fatalf("decode: %v (body=%q)", err, body)
	}
	return out
}

func TestHTTP_InitializeAssignsSessionID(t *testing.T) {
	srv := newHTTPTestServer(t)
	defer srv.Close()

	resp := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "initialize",
		Params:  json.RawMessage(`{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}`),
	}, nil)

	if resp.StatusCode != 200 {
		t.Fatalf("status = %d, want 200", resp.StatusCode)
	}
	sessID := resp.Header.Get(SessionHeader)
	if sessID == "" {
		t.Fatal("missing Mcp-Session-Id response header")
	}
	if got := resp.Header.Get(ProtocolVersionHeader); got != mcp.ProtocolVersion {
		t.Errorf("MCP-Protocol-Version = %q, want %q", got, mcp.ProtocolVersion)
	}
	out := decodeMCPResponse(t, resp)
	if out.Error != nil {
		t.Fatalf("unexpected error: %v", out.Error)
	}
}

func TestHTTP_RequiresSessionAfterInit(t *testing.T) {
	srv := newHTTPTestServer(t)
	defer srv.Close()

	// tools/list with no session header -> 400
	resp := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "tools/list",
	}, nil)
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusBadRequest {
		t.Errorf("status = %d, want 400", resp.StatusCode)
	}
}

func TestHTTP_UnknownSession(t *testing.T) {
	srv := newHTTPTestServer(t)
	defer srv.Close()

	resp := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "tools/list",
	}, map[string]string{SessionHeader: "bogus"})
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusNotFound {
		t.Errorf("status = %d, want 404", resp.StatusCode)
	}
}

func TestHTTP_EndToEndToolsList(t *testing.T) {
	srv := newHTTPTestServer(t)
	defer srv.Close()

	// Step 1: initialize.
	init := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "initialize",
		Params:  json.RawMessage(`{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}`),
	}, nil)
	sess := init.Header.Get(SessionHeader)
	init.Body.Close()
	if sess == "" {
		t.Fatal("no session id")
	}

	// Step 2: notifications/initialized — server replies 202 with no body.
	noteResp := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		Method:  "notifications/initialized",
	}, map[string]string{SessionHeader: sess, ProtocolVersionHeader: mcp.ProtocolVersion})
	noteResp.Body.Close()
	if noteResp.StatusCode != http.StatusAccepted {
		t.Errorf("notification status = %d, want 202", noteResp.StatusCode)
	}

	// Step 3: tools/list — should return all tools.
	list := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`2`),
		Method:  "tools/list",
	}, map[string]string{SessionHeader: sess, ProtocolVersionHeader: mcp.ProtocolVersion})
	out := decodeMCPResponse(t, list)
	if out.Error != nil {
		t.Fatalf("unexpected error: %v", out.Error)
	}
	resultJSON, _ := json.Marshal(out.Result)
	var lr mcp.ToolsListResult
	if err := json.Unmarshal(resultJSON, &lr); err != nil {
		t.Fatalf("decode result: %v", err)
	}
	if len(lr.Tools) < 200 {
		t.Errorf("tools = %d, want >= 200", len(lr.Tools))
	}
}

func TestHTTP_PingWithoutInit(t *testing.T) {
	srv := newHTTPTestServer(t)
	defer srv.Close()
	// First we need a session.
	init := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "initialize",
		Params:  json.RawMessage(`{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}`),
	}, nil)
	sess := init.Header.Get(SessionHeader)
	init.Body.Close()

	resp := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`2`),
		Method:  "ping",
	}, map[string]string{SessionHeader: sess})
	out := decodeMCPResponse(t, resp)
	if out.Error != nil {
		t.Fatalf("unexpected error: %v", out.Error)
	}
	if out.Result == nil {
		t.Error("expected empty object result")
	}
}

func TestHTTP_DeleteSession(t *testing.T) {
	srv := newHTTPTestServer(t)
	defer srv.Close()
	init := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "initialize",
		Params:  json.RawMessage(`{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}`),
	}, nil)
	sess := init.Header.Get(SessionHeader)
	init.Body.Close()

	req, _ := http.NewRequest(http.MethodDelete, srv.URL+"/mcp", nil)
	req.Header.Set(SessionHeader, sess)
	del, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("delete: %v", err)
	}
	del.Body.Close()
	if del.StatusCode != http.StatusNoContent {
		t.Errorf("delete status = %d, want 204", del.StatusCode)
	}

	// Subsequent call with the deleted session must 404.
	follow := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`2`),
		Method:  "tools/list",
	}, map[string]string{SessionHeader: sess})
	follow.Body.Close()
	if follow.StatusCode != http.StatusNotFound {
		t.Errorf("post-delete status = %d, want 404", follow.StatusCode)
	}
}

func TestHTTP_GetReturns405(t *testing.T) {
	srv := newHTTPTestServer(t)
	defer srv.Close()
	resp, err := http.Get(srv.URL + "/mcp")
	if err != nil {
		t.Fatalf("get: %v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusMethodNotAllowed {
		t.Errorf("status = %d, want 405", resp.StatusCode)
	}
}

func TestHTTP_RejectsOversizedBody(t *testing.T) {
	srv := newHTTPTestServer(t)
	defer srv.Close()
	big := strings.Repeat("x", MaxHTTPBodySize+100)
	req, _ := http.NewRequest(http.MethodPost, srv.URL+"/mcp", strings.NewReader(big))
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("post: %v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusRequestEntityTooLarge {
		t.Errorf("status = %d, want 413", resp.StatusCode)
	}
}

func TestHTTP_HealthEndpoint(t *testing.T) {
	srv := newHTTPTestServer(t)
	defer srv.Close()
	resp, err := http.Get(srv.URL + "/healthz")
	if err != nil {
		t.Fatalf("get: %v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		t.Errorf("status = %d, want 200", resp.StatusCode)
	}
}

func TestHTTP_OriginAllowlist(t *testing.T) {
	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(privateKey)
	tmpDir := t.TempDir()
	keyPath := filepath.Join(tmpDir, "k.p8")
	_ = os.WriteFile(keyPath, pem.EncodeToMemory(&pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}), 0o600)
	cfg := &config.Config{IssuerID: "iss", KeyID: "kid", PrivateKeyPath: keyPath}
	hs, err := NewHTTPServer(cfg, HTTPOptions{AllowedOrigins: []string{"https://allowed.example"}})
	if err != nil {
		t.Fatalf("new: %v", err)
	}
	srv := httptest.NewServer(hs.Handler())
	defer srv.Close()

	// Disallowed origin -> 403.
	req, _ := http.NewRequest(http.MethodPost, srv.URL+"/mcp", strings.NewReader(`{}`))
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Origin", "https://evil.example")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("do: %v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusForbidden {
		t.Errorf("disallowed origin status = %d, want 403", resp.StatusCode)
	}

	// Allowed origin proceeds (will get a 200/parse-style response).
	req2, _ := http.NewRequest(http.MethodPost, srv.URL+"/mcp", strings.NewReader(`{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}}`))
	req2.Header.Set("Content-Type", "application/json")
	req2.Header.Set("Origin", "https://allowed.example")
	resp2, err := http.DefaultClient.Do(req2)
	if err != nil {
		t.Fatalf("do: %v", err)
	}
	defer resp2.Body.Close()
	if resp2.StatusCode != http.StatusOK {
		t.Errorf("allowed origin status = %d, want 200", resp2.StatusCode)
	}
}

func newHTTPTestServerWithOpts(t *testing.T, opts HTTPOptions) *httptest.Server {
	t.Helper()
	privateKey, err := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		t.Fatalf("generate key: %v", err)
	}
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(privateKey)
	tmpDir := t.TempDir()
	keyPath := filepath.Join(tmpDir, "k.p8")
	_ = os.WriteFile(keyPath, pem.EncodeToMemory(&pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}), 0o600)
	cfg := &config.Config{IssuerID: "iss", KeyID: "kid", PrivateKeyPath: keyPath}
	hs, err := NewHTTPServer(cfg, opts)
	if err != nil {
		t.Fatalf("new: %v", err)
	}
	return httptest.NewServer(hs.Handler())
}

func TestHTTP_Auth_MissingToken(t *testing.T) {
	srv := newHTTPTestServerWithOpts(t, HTTPOptions{AuthTokens: []string{"s3cret"}})
	defer srv.Close()

	resp := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion, ID: json.RawMessage(`1`), Method: "initialize",
		Params: json.RawMessage(`{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}`),
	}, nil)
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusUnauthorized {
		t.Errorf("status = %d, want 401", resp.StatusCode)
	}
	if got := resp.Header.Get("WWW-Authenticate"); !strings.HasPrefix(got, "Bearer") {
		t.Errorf("WWW-Authenticate = %q, want Bearer challenge", got)
	}
}

func TestHTTP_Auth_WrongToken(t *testing.T) {
	srv := newHTTPTestServerWithOpts(t, HTTPOptions{AuthTokens: []string{"s3cret"}})
	defer srv.Close()

	resp := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion, ID: json.RawMessage(`1`), Method: "initialize",
		Params: json.RawMessage(`{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}`),
	}, map[string]string{"Authorization": "Bearer nope"})
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusUnauthorized {
		t.Errorf("status = %d, want 401", resp.StatusCode)
	}
}

func TestHTTP_Auth_CorrectToken(t *testing.T) {
	srv := newHTTPTestServerWithOpts(t, HTTPOptions{AuthTokens: []string{"primary", "rotating"}})
	defer srv.Close()

	// Either token in the set is accepted (rotation support).
	for _, tok := range []string{"primary", "rotating"} {
		resp := postJSON(t, srv.URL+"/mcp", mcp.Request{
			JSONRPC: mcp.JSONRPCVersion, ID: json.RawMessage(`1`), Method: "initialize",
			Params: json.RawMessage(`{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}`),
		}, map[string]string{"Authorization": "Bearer " + tok})
		if resp.StatusCode != http.StatusOK {
			t.Errorf("token %q: status = %d, want 200", tok, resp.StatusCode)
		}
		resp.Body.Close()
	}
}

func TestHTTP_Auth_DoesNotGateHealthz(t *testing.T) {
	srv := newHTTPTestServerWithOpts(t, HTTPOptions{AuthTokens: []string{"s3cret"}})
	defer srv.Close()
	// /healthz must remain reachable for K8s probes even when /mcp is locked down.
	resp, err := http.Get(srv.URL + "/healthz")
	if err != nil {
		t.Fatalf("get: %v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		t.Errorf("status = %d, want 200", resp.StatusCode)
	}
}

// metricValue reads a Prometheus counter or gauge metric from the
// registry by name. Returns -1 when the metric is missing.
func metricValue(t *testing.T, srv *HTTPServer, name string, labels map[string]string) float64 {
	t.Helper()
	mfs, err := srv.metrics.registry.Gather()
	if err != nil {
		t.Fatalf("gather: %v", err)
	}
	for _, mf := range mfs {
		if mf.GetName() != name {
			continue
		}
		for _, m := range mf.GetMetric() {
			if !labelsMatch(m.GetLabel(), labels) {
				continue
			}
			if m.Counter != nil {
				return m.Counter.GetValue()
			}
			if m.Gauge != nil {
				return m.Gauge.GetValue()
			}
		}
	}
	return -1
}

func labelsMatch(have []*dto.LabelPair, want map[string]string) bool {
	for k, v := range want {
		found := false
		for _, p := range have {
			if p.GetName() == k && p.GetValue() == v {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	return true
}

// metricsHelper builds an HTTPServer directly (not wrapped in httptest)
// so the test can read counter values from the registry.
func httpServerForMetrics(t *testing.T, opts HTTPOptions) (*HTTPServer, *httptest.Server) {
	t.Helper()
	privateKey, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(privateKey)
	tmpDir := t.TempDir()
	keyPath := filepath.Join(tmpDir, "k.p8")
	_ = os.WriteFile(keyPath, pem.EncodeToMemory(&pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}), 0o600)
	cfg := &config.Config{IssuerID: "iss", KeyID: "kid", PrivateKeyPath: keyPath}
	hs, err := NewHTTPServer(cfg, opts)
	if err != nil {
		t.Fatalf("new: %v", err)
	}
	srv := httptest.NewServer(hs.Handler())
	return hs, srv
}

func TestHTTP_MetricsEndpoint(t *testing.T) {
	_, srv := httpServerForMetrics(t, HTTPOptions{})
	defer srv.Close()
	// Trigger one request first; Prometheus omits zero-value series
	// from the text exposition format, so we need something to emit.
	init := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion, ID: json.RawMessage(`1`), Method: "initialize",
		Params: json.RawMessage(`{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}`),
	}, nil)
	init.Body.Close()

	resp, err := http.Get(srv.URL + "/metrics")
	if err != nil {
		t.Fatalf("get: %v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		t.Errorf("status = %d, want 200", resp.StatusCode)
	}
	body, _ := io.ReadAll(resp.Body)
	for _, want := range []string{
		"asc_mcp_http_requests_total",
		"asc_mcp_http_request_duration_seconds",
		"asc_mcp_session_active",
	} {
		if !strings.Contains(string(body), want) {
			t.Errorf("metrics output missing %q", want)
		}
	}
}

func TestHTTP_MetricsRecordRequests(t *testing.T) {
	hs, srv := httpServerForMetrics(t, HTTPOptions{})
	defer srv.Close()

	// Issue one initialize + one tools/list.
	init := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion, ID: json.RawMessage(`1`), Method: "initialize",
		Params: json.RawMessage(`{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}`),
	}, nil)
	sess := init.Header.Get(SessionHeader)
	init.Body.Close()
	list := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion, ID: json.RawMessage(`2`), Method: "tools/list",
	}, map[string]string{SessionHeader: sess})
	list.Body.Close()

	// At least two POSTs should be recorded with status=200.
	got := metricValue(t, hs, "asc_mcp_http_requests_total", map[string]string{"method": "POST", "status": "200"})
	if got < 2 {
		t.Errorf("asc_mcp_http_requests_total{method=POST,status=200} = %v, want >= 2", got)
	}
	// One session should be active.
	if active := metricValue(t, hs, "asc_mcp_session_active", nil); active != 1 {
		t.Errorf("active sessions = %v, want 1", active)
	}
	// One session should have been created.
	if created := metricValue(t, hs, "asc_mcp_session_created_total", nil); created != 1 {
		t.Errorf("created sessions = %v, want 1", created)
	}
}

func TestHTTP_MetricsAuthFailure(t *testing.T) {
	hs, srv := httpServerForMetrics(t, HTTPOptions{AuthTokens: []string{"secret"}})
	defer srv.Close()
	resp := postJSON(t, srv.URL+"/mcp", mcp.Request{
		JSONRPC: mcp.JSONRPCVersion, ID: json.RawMessage(`1`), Method: "initialize",
	}, nil)
	resp.Body.Close()
	if got := metricValue(t, hs, "asc_mcp_http_auth_failures_total", nil); got != 1 {
		t.Errorf("auth failures = %v, want 1", got)
	}
}
