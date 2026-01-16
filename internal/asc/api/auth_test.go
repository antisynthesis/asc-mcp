package api

import (
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"encoding/pem"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"testing"
	"time"
)

// generateTestKey creates a test ECDSA P-256 private key in PEM format.
func generateTestKey(t *testing.T) ([]byte, *ecdsa.PrivateKey) {
	t.Helper()

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

	return pem.EncodeToMemory(pemBlock), privateKey
}

// createTestKeyFile creates a temporary key file for testing.
func createTestKeyFile(t *testing.T, content []byte) string {
	t.Helper()

	tmpDir := t.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")

	if err := os.WriteFile(keyPath, content, 0600); err != nil {
		t.Fatalf("failed to write test key: %v", err)
	}

	return keyPath
}

func TestNewTokenProvider(t *testing.T) {
	keyPEM, _ := generateTestKey(t)
	keyPath := createTestKeyFile(t, keyPEM)

	tests := []struct {
		name           string
		issuerID       string
		keyID          string
		privateKeyPath string
		wantErr        bool
		errContains    string
	}{
		{
			name:           "valid configuration",
			issuerID:       "test-issuer-id",
			keyID:          "TESTKEY123",
			privateKeyPath: keyPath,
			wantErr:        false,
		},
		{
			name:           "missing key file",
			issuerID:       "test-issuer-id",
			keyID:          "TESTKEY123",
			privateKeyPath: "/nonexistent/path/key.p8",
			wantErr:        true,
			errContains:    "failed to read private key",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			tp, err := NewTokenProvider(tt.issuerID, tt.keyID, tt.privateKeyPath)

			if tt.wantErr {
				if err == nil {
					t.Error("expected error, got nil")
				} else if tt.errContains != "" && !strings.Contains(err.Error(), tt.errContains) {
					t.Errorf("error %q does not contain %q", err.Error(), tt.errContains)
				}
				return
			}

			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}

			if tp == nil {
				t.Fatal("expected token provider, got nil")
			}

			if tp.issuerID != tt.issuerID {
				t.Errorf("issuerID = %q, want %q", tp.issuerID, tt.issuerID)
			}

			if tp.keyID != tt.keyID {
				t.Errorf("keyID = %q, want %q", tp.keyID, tt.keyID)
			}
		})
	}
}

func TestParsePrivateKey(t *testing.T) {
	tests := []struct {
		name        string
		keyData     func(t *testing.T) []byte
		wantErr     bool
		errContains string
	}{
		{
			name: "valid P-256 key",
			keyData: func(t *testing.T) []byte {
				keyPEM, _ := generateTestKey(t)
				return keyPEM
			},
			wantErr: false,
		},
		{
			name: "no PEM block",
			keyData: func(t *testing.T) []byte {
				return []byte("not a PEM block")
			},
			wantErr:     true,
			errContains: "no PEM block found",
		},
		{
			name: "invalid PEM content",
			keyData: func(t *testing.T) []byte {
				// Valid base64-encoded but invalid PKCS8 content
				return []byte("-----BEGIN PRIVATE KEY-----\nYWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXo=\n-----END PRIVATE KEY-----\n")
			},
			wantErr:     true,
			errContains: "failed to parse PKCS8",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			key, err := parsePrivateKey(tt.keyData(t))

			if tt.wantErr {
				if err == nil {
					t.Error("expected error, got nil")
				} else if tt.errContains != "" && !strings.Contains(err.Error(), tt.errContains) {
					t.Errorf("error %q does not contain %q", err.Error(), tt.errContains)
				}
				return
			}

			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}

			if key == nil {
				t.Fatal("expected key, got nil")
			}

			// Verify it's a P-256 curve
			if key.Curve != elliptic.P256() {
				t.Error("expected P-256 curve")
			}
		})
	}
}

func TestTokenProvider_GetToken(t *testing.T) {
	keyPEM, _ := generateTestKey(t)
	keyPath := createTestKeyFile(t, keyPEM)

	tp, err := NewTokenProvider("test-issuer", "TESTKEY123", keyPath)
	if err != nil {
		t.Fatalf("failed to create token provider: %v", err)
	}

	// First call should generate a token
	token1, err := tp.GetToken()
	if err != nil {
		t.Fatalf("failed to get token: %v", err)
	}

	if token1 == "" {
		t.Error("expected non-empty token")
	}

	// Verify token structure (header.payload.signature)
	parts := strings.Split(token1, ".")
	if len(parts) != 3 {
		t.Errorf("expected 3 parts in JWT, got %d", len(parts))
	}

	// Second call should return cached token
	token2, err := tp.GetToken()
	if err != nil {
		t.Fatalf("failed to get token second time: %v", err)
	}

	if token1 != token2 {
		t.Error("expected same token from cache")
	}
}

func TestTokenProvider_GenerateToken(t *testing.T) {
	keyPEM, _ := generateTestKey(t)
	keyPath := createTestKeyFile(t, keyPEM)

	tp, err := NewTokenProvider("test-issuer-id", "TESTKEY123", keyPath)
	if err != nil {
		t.Fatalf("failed to create token provider: %v", err)
	}

	token, expiresAt, err := tp.generateToken()
	if err != nil {
		t.Fatalf("failed to generate token: %v", err)
	}

	// Check token structure
	parts := strings.Split(token, ".")
	if len(parts) != 3 {
		t.Fatalf("expected 3 parts in JWT, got %d", len(parts))
	}

	// Decode and verify header
	headerJSON, err := base64.RawURLEncoding.DecodeString(parts[0])
	if err != nil {
		t.Fatalf("failed to decode header: %v", err)
	}

	var header map[string]string
	if err := json.Unmarshal(headerJSON, &header); err != nil {
		t.Fatalf("failed to unmarshal header: %v", err)
	}

	if header["alg"] != "ES256" {
		t.Errorf("header alg = %q, want ES256", header["alg"])
	}
	if header["typ"] != "JWT" {
		t.Errorf("header typ = %q, want JWT", header["typ"])
	}
	if header["kid"] != "TESTKEY123" {
		t.Errorf("header kid = %q, want TESTKEY123", header["kid"])
	}

	// Decode and verify payload
	payloadJSON, err := base64.RawURLEncoding.DecodeString(parts[1])
	if err != nil {
		t.Fatalf("failed to decode payload: %v", err)
	}

	var payload map[string]any
	if err := json.Unmarshal(payloadJSON, &payload); err != nil {
		t.Fatalf("failed to unmarshal payload: %v", err)
	}

	if payload["iss"] != "test-issuer-id" {
		t.Errorf("payload iss = %v, want test-issuer-id", payload["iss"])
	}
	if payload["aud"] != "appstoreconnect-v1" {
		t.Errorf("payload aud = %v, want appstoreconnect-v1", payload["aud"])
	}

	// Verify expiration is in the future
	if expiresAt.Before(time.Now()) {
		t.Error("token already expired")
	}

	// Verify expiration is approximately 15 minutes from now
	expectedExpiry := time.Now().Add(TokenDuration)
	if expiresAt.Before(expectedExpiry.Add(-time.Minute)) || expiresAt.After(expectedExpiry.Add(time.Minute)) {
		t.Errorf("expiry time %v not within expected range around %v", expiresAt, expectedExpiry)
	}
}

func TestTokenProvider_VerifyToken(t *testing.T) {
	keyPEM, _ := generateTestKey(t)
	keyPath := createTestKeyFile(t, keyPEM)

	tp, err := NewTokenProvider("test-issuer", "TESTKEY123", keyPath)
	if err != nil {
		t.Fatalf("failed to create token provider: %v", err)
	}

	token, _, err := tp.generateToken()
	if err != nil {
		t.Fatalf("failed to generate token: %v", err)
	}

	// Valid token should verify
	if !tp.verifyToken(token) {
		t.Error("valid token should verify")
	}

	// Tampered token should not verify
	tamperedToken := token[:len(token)-5] + "XXXXX"
	if tp.verifyToken(tamperedToken) {
		t.Error("tampered token should not verify")
	}

	// Invalid format should not verify
	if tp.verifyToken("invalid.token") {
		t.Error("invalid format should not verify")
	}

	// Empty token should not verify
	if tp.verifyToken("") {
		t.Error("empty token should not verify")
	}
}

func TestBase64URLEncode(t *testing.T) {
	tests := []struct {
		name  string
		input []byte
		want  string
	}{
		{
			name:  "empty",
			input: []byte{},
			want:  "",
		},
		{
			name:  "simple text",
			input: []byte("hello"),
			want:  "aGVsbG8",
		},
		{
			name:  "with special chars that need URL encoding",
			input: []byte{0xff, 0xfe, 0xfd},
			want:  "__79",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := base64URLEncode(tt.input)
			if got != tt.want {
				t.Errorf("base64URLEncode() = %q, want %q", got, tt.want)
			}

			// Verify no padding
			if strings.Contains(got, "=") {
				t.Error("base64URLEncode should not contain padding")
			}
		})
	}
}

func TestSplitToken(t *testing.T) {
	tests := []struct {
		name  string
		token string
		want  []string
	}{
		{
			name:  "valid JWT",
			token: "header.payload.signature",
			want:  []string{"header", "payload", "signature"},
		},
		{
			name:  "no dots",
			token: "invalid",
			want:  []string{"invalid"},
		},
		{
			name:  "empty",
			token: "",
			want:  []string{""},
		},
		{
			name:  "extra dots",
			token: "a.b.c.d",
			want:  []string{"a", "b", "c", "d"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := splitToken(tt.token)

			if len(got) != len(tt.want) {
				t.Errorf("splitToken() returned %d parts, want %d", len(got), len(tt.want))
				return
			}

			for i := range got {
				if got[i] != tt.want[i] {
					t.Errorf("splitToken()[%d] = %q, want %q", i, got[i], tt.want[i])
				}
			}
		})
	}
}

func TestTokenProvider_ConcurrentAccess(t *testing.T) {
	keyPEM, _ := generateTestKey(t)
	keyPath := createTestKeyFile(t, keyPEM)

	tp, err := NewTokenProvider("test-issuer", "TESTKEY123", keyPath)
	if err != nil {
		t.Fatalf("failed to create token provider: %v", err)
	}

	const numGoroutines = 100
	var wg sync.WaitGroup
	tokens := make([]string, numGoroutines)
	errors := make([]error, numGoroutines)

	for i := 0; i < numGoroutines; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			tokens[idx], errors[idx] = tp.GetToken()
		}(i)
	}

	wg.Wait()

	// Check for errors
	for i, err := range errors {
		if err != nil {
			t.Errorf("goroutine %d failed: %v", i, err)
		}
	}

	// All tokens should be the same (cached)
	firstToken := tokens[0]
	for i, token := range tokens {
		if token != firstToken {
			t.Errorf("goroutine %d got different token", i)
		}
	}
}

// Benchmarks

func BenchmarkTokenProvider_GenerateToken(b *testing.B) {
	privateKey, err := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		b.Fatalf("failed to generate key: %v", err)
	}

	tp := &TokenProvider{
		issuerID:   "test-issuer",
		keyID:      "TESTKEY123",
		privateKey: privateKey,
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, _, err := tp.generateToken()
		if err != nil {
			b.Fatalf("failed to generate token: %v", err)
		}
	}
}

func BenchmarkTokenProvider_GetToken_Cached(b *testing.B) {
	privateKey, err := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		b.Fatalf("failed to generate key: %v", err)
	}

	tp := &TokenProvider{
		issuerID:   "test-issuer",
		keyID:      "TESTKEY123",
		privateKey: privateKey,
	}

	// Pre-populate cache
	_, _ = tp.GetToken()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, err := tp.GetToken()
		if err != nil {
			b.Fatalf("failed to get token: %v", err)
		}
	}
}

func BenchmarkSign(b *testing.B) {
	privateKey, err := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		b.Fatalf("failed to generate key: %v", err)
	}

	tp := &TokenProvider{
		privateKey: privateKey,
	}

	data := []byte("test data to sign for benchmark purposes")

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, err := tp.sign(data)
		if err != nil {
			b.Fatalf("failed to sign: %v", err)
		}
	}
}

func BenchmarkBase64URLEncode(b *testing.B) {
	data := make([]byte, 1024)
	rand.Read(data)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = base64URLEncode(data)
	}
}
