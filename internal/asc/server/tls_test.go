package server

import (
	"context"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"crypto/tls"
	"crypto/x509"
	"crypto/x509/pkix"
	"encoding/json"
	"encoding/pem"
	"errors"
	"io"
	"math/big"
	"net"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/antisynthesis/asc-mcp/internal/asc/config"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// writeSelfSignedCert generates an ECDSA self-signed cert valid for
// 127.0.0.1 and writes it as PEM to certPath and keyPath.
func writeSelfSignedCert(t *testing.T, certPath, keyPath string) {
	t.Helper()
	priv, err := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		t.Fatalf("generate key: %v", err)
	}
	template := x509.Certificate{
		SerialNumber: big.NewInt(1),
		Subject:      pkix.Name{CommonName: "asc-mcp-test"},
		NotBefore:    time.Now().Add(-time.Hour),
		NotAfter:     time.Now().Add(time.Hour),
		KeyUsage:     x509.KeyUsageDigitalSignature | x509.KeyUsageKeyEncipherment,
		ExtKeyUsage:  []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth},
		IPAddresses:  []net.IP{net.ParseIP("127.0.0.1")},
		DNSNames:     []string{"localhost"},
	}
	der, err := x509.CreateCertificate(rand.Reader, &template, &template, &priv.PublicKey, priv)
	if err != nil {
		t.Fatalf("create cert: %v", err)
	}
	if err := os.WriteFile(certPath, pem.EncodeToMemory(&pem.Block{Type: "CERTIFICATE", Bytes: der}), 0o600); err != nil {
		t.Fatalf("write cert: %v", err)
	}
	keyBytes, err := x509.MarshalECPrivateKey(priv)
	if err != nil {
		t.Fatalf("marshal key: %v", err)
	}
	if err := os.WriteFile(keyPath, pem.EncodeToMemory(&pem.Block{Type: "EC PRIVATE KEY", Bytes: keyBytes}), 0o600); err != nil {
		t.Fatalf("write key: %v", err)
	}
}

func newHTTPServerForTLS(t *testing.T) *HTTPServer {
	t.Helper()
	priv, _ := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	keyBytes, _ := x509.MarshalPKCS8PrivateKey(priv)
	dir := t.TempDir()
	keyPath := filepath.Join(dir, "asc.p8")
	_ = os.WriteFile(keyPath, pem.EncodeToMemory(&pem.Block{Type: "PRIVATE KEY", Bytes: keyBytes}), 0o600)
	cfg := &config.Config{IssuerID: "iss", KeyID: "kid", PrivateKeyPath: keyPath}
	hs, err := NewHTTPServer(cfg, HTTPOptions{})
	if err != nil {
		t.Fatalf("new: %v", err)
	}
	return hs
}

func TestValidateTLSPair(t *testing.T) {
	dir := t.TempDir()
	cert := filepath.Join(dir, "c.pem")
	key := filepath.Join(dir, "k.pem")
	writeSelfSignedCert(t, cert, key)

	t.Run("neither set means plain HTTP", func(t *testing.T) {
		use, err := validateTLSPair("", "")
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if use {
			t.Error("expected useTLS=false when both empty")
		}
	})
	t.Run("only cert is an error", func(t *testing.T) {
		_, err := validateTLSPair(cert, "")
		if err == nil {
			t.Fatal("expected error")
		}
	})
	t.Run("only key is an error", func(t *testing.T) {
		_, err := validateTLSPair("", key)
		if err == nil {
			t.Fatal("expected error")
		}
	})
	t.Run("missing cert file", func(t *testing.T) {
		_, err := validateTLSPair(filepath.Join(dir, "nope.pem"), key)
		if err == nil {
			t.Fatal("expected error")
		}
		if !strings.Contains(err.Error(), "tls cert") {
			t.Errorf("error %q does not mention tls cert", err.Error())
		}
	})
	t.Run("both set and readable", func(t *testing.T) {
		use, err := validateTLSPair(cert, key)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if !use {
			t.Error("expected useTLS=true")
		}
	})
}

func TestListenAndServe_TLS(t *testing.T) {
	dir := t.TempDir()
	cert := filepath.Join(dir, "c.pem")
	key := filepath.Join(dir, "k.pem")
	writeSelfSignedCert(t, cert, key)

	// Pick an ephemeral port by binding briefly.
	ln, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("listen: %v", err)
	}
	addr := ln.Addr().String()
	_ = ln.Close()

	hs := newHTTPServerForTLS(t)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	errCh := make(chan error, 1)
	go func() { errCh <- hs.ListenAndServe(ctx, addr, cert, key) }()

	// Poll until the server is up (TLS handshake will fail until the
	// listener is ready).
	client := &http.Client{
		Timeout:   2 * time.Second,
		Transport: &http.Transport{TLSClientConfig: &tls.Config{InsecureSkipVerify: true}}, //nolint:gosec
	}
	url := "https://" + addr + "/healthz"
	deadline := time.Now().Add(3 * time.Second)
	var resp *http.Response
	for time.Now().Before(deadline) {
		resp, err = client.Get(url)
		if err == nil {
			break
		}
		time.Sleep(20 * time.Millisecond)
	}
	if err != nil {
		t.Fatalf("never reachable: %v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		t.Errorf("status = %d, want 200", resp.StatusCode)
	}
	if resp.TLS == nil {
		t.Error("expected TLS connection state on response")
	}
	if resp.TLS != nil && resp.TLS.Version < tls.VersionTLS12 {
		t.Errorf("TLS version = %x, want >= TLS 1.2", resp.TLS.Version)
	}

	// Now hit /mcp with TLS too and verify we get a valid MCP response.
	body, _ := json.Marshal(mcp.Request{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Method:  "initialize",
		Params:  json.RawMessage(`{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"1"}}`),
	})
	mcpResp, err := client.Post("https://"+addr+"/mcp", "application/json", strings.NewReader(string(body)))
	if err != nil {
		t.Fatalf("post: %v", err)
	}
	defer mcpResp.Body.Close()
	if mcpResp.StatusCode != http.StatusOK {
		t.Errorf("mcp status = %d, want 200", mcpResp.StatusCode)
	}
	respBody, _ := io.ReadAll(mcpResp.Body)
	if !strings.Contains(string(respBody), "2025-06-18") {
		t.Errorf("response body does not include negotiated protocol: %s", respBody)
	}

	// Shut down.
	cancel()
	select {
	case err := <-errCh:
		if err != nil && !errors.Is(err, http.ErrServerClosed) {
			t.Errorf("shutdown error: %v", err)
		}
	case <-time.After(5 * time.Second):
		t.Fatal("server did not shut down in time")
	}
}

func TestListenAndServe_TLSConfigError(t *testing.T) {
	hs := newHTTPServerForTLS(t)
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	// Only --tls-cert set, no key: must error before binding.
	err := hs.ListenAndServe(ctx, "127.0.0.1:0", "/does/not/matter.pem", "")
	if err == nil {
		t.Fatal("expected error for half-configured TLS")
	}
	if !strings.Contains(err.Error(), "tls config invalid") {
		t.Errorf("error %q does not mention tls config", err.Error())
	}
}
