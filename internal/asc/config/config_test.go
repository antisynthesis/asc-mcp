package config

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestLoad(t *testing.T) {
	// Create a temporary key file
	tmpDir := t.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	if err := os.WriteFile(keyPath, []byte("test key content"), 0600); err != nil {
		t.Fatalf("failed to create test key file: %v", err)
	}

	tests := []struct {
		name        string
		envVars     map[string]string
		wantErr     bool
		errContains string
		validate    func(t *testing.T, cfg *Config)
	}{
		{
			name: "valid configuration",
			envVars: map[string]string{
				"ASC_ISSUER_ID":        "test-issuer-id",
				"ASC_KEY_ID":           "TESTKEY123",
				"ASC_PRIVATE_KEY_PATH": keyPath,
			},
			wantErr: false,
			validate: func(t *testing.T, cfg *Config) {
				if cfg.IssuerID != "test-issuer-id" {
					t.Errorf("IssuerID = %q, want test-issuer-id", cfg.IssuerID)
				}
				if cfg.KeyID != "TESTKEY123" {
					t.Errorf("KeyID = %q, want TESTKEY123", cfg.KeyID)
				}
				if cfg.PrivateKeyPath != keyPath {
					t.Errorf("PrivateKeyPath = %q, want %q", cfg.PrivateKeyPath, keyPath)
				}
			},
		},
		{
			name: "missing issuer ID",
			envVars: map[string]string{
				"ASC_KEY_ID":           "TESTKEY123",
				"ASC_PRIVATE_KEY_PATH": keyPath,
			},
			wantErr:     true,
			errContains: "ASC_ISSUER_ID",
		},
		{
			name: "missing key ID",
			envVars: map[string]string{
				"ASC_ISSUER_ID":        "test-issuer-id",
				"ASC_PRIVATE_KEY_PATH": keyPath,
			},
			wantErr:     true,
			errContains: "ASC_KEY_ID",
		},
		{
			name: "missing private key path",
			envVars: map[string]string{
				"ASC_ISSUER_ID": "test-issuer-id",
				"ASC_KEY_ID":    "TESTKEY123",
			},
			wantErr:     true,
			errContains: "ASC_PRIVATE_KEY_PATH",
		},
		{
			name: "nonexistent key file",
			envVars: map[string]string{
				"ASC_ISSUER_ID":        "test-issuer-id",
				"ASC_KEY_ID":           "TESTKEY123",
				"ASC_PRIVATE_KEY_PATH": "/nonexistent/path/key.p8",
			},
			wantErr:     true,
			errContains: "private key file not found",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Clear all relevant env vars
			os.Unsetenv("ASC_ISSUER_ID")
			os.Unsetenv("ASC_KEY_ID")
			os.Unsetenv("ASC_PRIVATE_KEY_PATH")

			// Set test env vars
			for k, v := range tt.envVars {
				os.Setenv(k, v)
			}

			// Cleanup after test
			defer func() {
				for k := range tt.envVars {
					os.Unsetenv(k)
				}
			}()

			cfg, err := Load()

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

			if cfg == nil {
				t.Fatal("expected config, got nil")
			}

			if tt.validate != nil {
				tt.validate(t, cfg)
			}
		})
	}
}

func TestLoad_EmptyValues(t *testing.T) {
	// Test that empty string values are treated as missing
	tmpDir := t.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	os.WriteFile(keyPath, []byte("test"), 0600)

	tests := []struct {
		name        string
		issuerID    string
		keyID       string
		keyPath     string
		errContains string
	}{
		{
			name:        "empty issuer ID",
			issuerID:    "",
			keyID:       "KEY123",
			keyPath:     keyPath,
			errContains: "ASC_ISSUER_ID",
		},
		{
			name:        "empty key ID",
			issuerID:    "issuer123",
			keyID:       "",
			keyPath:     keyPath,
			errContains: "ASC_KEY_ID",
		},
		{
			name:        "empty key path",
			issuerID:    "issuer123",
			keyID:       "KEY123",
			keyPath:     "",
			errContains: "ASC_PRIVATE_KEY_PATH",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			os.Setenv("ASC_ISSUER_ID", tt.issuerID)
			os.Setenv("ASC_KEY_ID", tt.keyID)
			os.Setenv("ASC_PRIVATE_KEY_PATH", tt.keyPath)

			defer func() {
				os.Unsetenv("ASC_ISSUER_ID")
				os.Unsetenv("ASC_KEY_ID")
				os.Unsetenv("ASC_PRIVATE_KEY_PATH")
			}()

			_, err := Load()

			if err == nil {
				t.Error("expected error, got nil")
			} else if !strings.Contains(err.Error(), tt.errContains) {
				t.Errorf("error %q does not contain %q", err.Error(), tt.errContains)
			}
		})
	}
}

// Benchmark

func BenchmarkLoad(b *testing.B) {
	tmpDir := b.TempDir()
	keyPath := filepath.Join(tmpDir, "test_key.p8")
	os.WriteFile(keyPath, []byte("test key content"), 0600)

	os.Setenv("ASC_ISSUER_ID", "test-issuer")
	os.Setenv("ASC_KEY_ID", "TESTKEY123")
	os.Setenv("ASC_PRIVATE_KEY_PATH", keyPath)

	defer func() {
		os.Unsetenv("ASC_ISSUER_ID")
		os.Unsetenv("ASC_KEY_ID")
		os.Unsetenv("ASC_PRIVATE_KEY_PATH")
	}()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, err := Load()
		if err != nil {
			b.Fatalf("unexpected error: %v", err)
		}
	}
}
