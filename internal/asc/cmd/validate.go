// Package cmd provides the command-line interface for asc-mcp.
package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"

	"github.com/antisynthesis/asc-mcp/internal/asc/config"
)

var validateCmd = &cobra.Command{
	Use:   "validate",
	Short: "Validate configuration",
	Long: `Validate that all required environment variables are set and the
private key file exists and is readable.

This command does not make any API calls - it only validates local configuration.`,
	RunE: runValidate,
}

func runValidate(cmd *cobra.Command, args []string) error {
	fmt.Println("Validating configuration...")
	fmt.Println()

	// Check environment variables
	issuerID := os.Getenv("ASC_ISSUER_ID")
	keyID := os.Getenv("ASC_KEY_ID")
	keyPath := os.Getenv("ASC_PRIVATE_KEY_PATH")

	hasErrors := false

	if issuerID == "" {
		fmt.Println("[FAIL] ASC_ISSUER_ID is not set")
		hasErrors = true
	} else {
		fmt.Printf("[OK]   ASC_ISSUER_ID is set (%s...)\n", issuerID[:8])
	}

	if keyID == "" {
		fmt.Println("[FAIL] ASC_KEY_ID is not set")
		hasErrors = true
	} else {
		fmt.Printf("[OK]   ASC_KEY_ID is set (%s)\n", keyID)
	}

	if keyPath == "" {
		fmt.Println("[FAIL] ASC_PRIVATE_KEY_PATH is not set")
		hasErrors = true
	} else {
		if _, err := os.Stat(keyPath); os.IsNotExist(err) {
			fmt.Printf("[FAIL] ASC_PRIVATE_KEY_PATH file not found: %s\n", keyPath)
			hasErrors = true
		} else if err != nil {
			fmt.Printf("[FAIL] ASC_PRIVATE_KEY_PATH error: %v\n", err)
			hasErrors = true
		} else {
			fmt.Printf("[OK]   ASC_PRIVATE_KEY_PATH exists (%s)\n", keyPath)
		}
	}

	fmt.Println()

	if hasErrors {
		return fmt.Errorf("configuration validation failed")
	}

	// Try to load full config to validate key parsing
	_, err := config.Load()
	if err != nil {
		fmt.Printf("[FAIL] Configuration load error: %v\n", err)
		return err
	}

	fmt.Println("[OK]   Configuration is valid")
	return nil
}
