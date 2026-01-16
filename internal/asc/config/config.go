// Package config provides configuration loading for the App Store Connect MCP server.
package config

import (
	"fmt"
	"os"
)

// Config holds the configuration for the App Store Connect MCP server.
type Config struct {
	// IssuerID is the App Store Connect API Issuer ID.
	IssuerID string

	// KeyID is the App Store Connect API Key ID.
	KeyID string

	// PrivateKeyPath is the path to the .p8 private key file.
	PrivateKeyPath string
}

// Load loads configuration from environment variables.
func Load() (*Config, error) {
	cfg := &Config{
		IssuerID:       os.Getenv("ASC_ISSUER_ID"),
		KeyID:          os.Getenv("ASC_KEY_ID"),
		PrivateKeyPath: os.Getenv("ASC_PRIVATE_KEY_PATH"),
	}

	if cfg.IssuerID == "" {
		return nil, fmt.Errorf("ASC_ISSUER_ID environment variable is required")
	}

	if cfg.KeyID == "" {
		return nil, fmt.Errorf("ASC_KEY_ID environment variable is required")
	}

	if cfg.PrivateKeyPath == "" {
		return nil, fmt.Errorf("ASC_PRIVATE_KEY_PATH environment variable is required")
	}

	if _, err := os.Stat(cfg.PrivateKeyPath); os.IsNotExist(err) {
		return nil, fmt.Errorf("private key file not found: %s", cfg.PrivateKeyPath)
	}

	return cfg, nil
}
