// Package api provides the App Store Connect API client.
package api

import (
	"crypto/ecdsa"
	"crypto/rand"
	"crypto/sha256"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"encoding/pem"
	"fmt"
	"math/big"
	"os"
	"sync"
	"time"
)

const (
	// TokenDuration is the validity period for JWT tokens.
	// App Store Connect allows up to 20 minutes.
	TokenDuration = 15 * time.Minute

	// TokenRefreshBuffer is how early to refresh the token before expiry.
	TokenRefreshBuffer = 2 * time.Minute
)

// TokenProvider manages JWT tokens for App Store Connect API authentication.
type TokenProvider struct {
	issuerID   string
	keyID      string
	privateKey *ecdsa.PrivateKey

	mu        sync.RWMutex
	token     string
	expiresAt time.Time
}

// NewTokenProvider creates a new token provider.
func NewTokenProvider(issuerID, keyID, privateKeyPath string) (*TokenProvider, error) {
	keyData, err := os.ReadFile(privateKeyPath)
	if err != nil {
		return nil, fmt.Errorf("failed to read private key: %w", err)
	}

	privateKey, err := parsePrivateKey(keyData)
	if err != nil {
		return nil, fmt.Errorf("failed to parse private key: %w", err)
	}

	return &TokenProvider{
		issuerID:   issuerID,
		keyID:      keyID,
		privateKey: privateKey,
	}, nil
}

// parsePrivateKey parses a PEM-encoded ECDSA private key.
func parsePrivateKey(data []byte) (*ecdsa.PrivateKey, error) {
	block, _ := pem.Decode(data)
	if block == nil {
		return nil, fmt.Errorf("no PEM block found in private key")
	}

	key, err := x509.ParsePKCS8PrivateKey(block.Bytes)
	if err != nil {
		return nil, fmt.Errorf("failed to parse PKCS8 private key: %w", err)
	}

	ecKey, ok := key.(*ecdsa.PrivateKey)
	if !ok {
		return nil, fmt.Errorf("private key is not ECDSA")
	}

	return ecKey, nil
}

// GetToken returns a valid JWT token, generating a new one if necessary.
func (tp *TokenProvider) GetToken() (string, error) {
	tp.mu.RLock()
	if tp.token != "" && time.Now().Add(TokenRefreshBuffer).Before(tp.expiresAt) {
		token := tp.token
		tp.mu.RUnlock()
		return token, nil
	}
	tp.mu.RUnlock()

	tp.mu.Lock()
	defer tp.mu.Unlock()

	// Double-check after acquiring write lock
	if tp.token != "" && time.Now().Add(TokenRefreshBuffer).Before(tp.expiresAt) {
		return tp.token, nil
	}

	token, expiresAt, err := tp.generateToken()
	if err != nil {
		return "", err
	}

	tp.token = token
	tp.expiresAt = expiresAt

	return token, nil
}

// generateToken creates a new JWT token using ES256.
func (tp *TokenProvider) generateToken() (string, time.Time, error) {
	now := time.Now()
	expiresAt := now.Add(TokenDuration)

	header := map[string]string{
		"alg": "ES256",
		"typ": "JWT",
		"kid": tp.keyID,
	}

	payload := map[string]any{
		"iss": tp.issuerID,
		"iat": now.Unix(),
		"exp": expiresAt.Unix(),
		"aud": "appstoreconnect-v1",
	}

	headerJSON, err := json.Marshal(header)
	if err != nil {
		return "", time.Time{}, fmt.Errorf("failed to marshal header: %w", err)
	}

	payloadJSON, err := json.Marshal(payload)
	if err != nil {
		return "", time.Time{}, fmt.Errorf("failed to marshal payload: %w", err)
	}

	headerB64 := base64URLEncode(headerJSON)
	payloadB64 := base64URLEncode(payloadJSON)

	signingInput := headerB64 + "." + payloadB64

	signature, err := tp.sign([]byte(signingInput))
	if err != nil {
		return "", time.Time{}, fmt.Errorf("failed to sign token: %w", err)
	}

	signatureB64 := base64URLEncode(signature)

	token := signingInput + "." + signatureB64

	return token, expiresAt, nil
}

// sign creates an ES256 signature for the given data.
func (tp *TokenProvider) sign(data []byte) ([]byte, error) {
	hash := sha256.Sum256(data)

	r, s, err := ecdsa.Sign(rand.Reader, tp.privateKey, hash[:])
	if err != nil {
		return nil, fmt.Errorf("failed to sign: %w", err)
	}

	// Convert r and s to fixed-size byte arrays (32 bytes each for P-256)
	curveBits := tp.privateKey.Curve.Params().BitSize
	keyBytes := (curveBits + 7) / 8

	rBytes := r.Bytes()
	sBytes := s.Bytes()

	signature := make([]byte, 2*keyBytes)

	// Pad r and s to keyBytes length
	copy(signature[keyBytes-len(rBytes):keyBytes], rBytes)
	copy(signature[2*keyBytes-len(sBytes):], sBytes)

	return signature, nil
}

// base64URLEncode encodes data using base64url encoding without padding.
func base64URLEncode(data []byte) string {
	return base64.RawURLEncoding.EncodeToString(data)
}

// verifyToken checks if a token's signature is valid (for testing).
func (tp *TokenProvider) verifyToken(tokenStr string) bool {
	parts := splitToken(tokenStr)
	if len(parts) != 3 {
		return false
	}

	signingInput := parts[0] + "." + parts[1]
	signature, err := base64.RawURLEncoding.DecodeString(parts[2])
	if err != nil {
		return false
	}

	hash := sha256.Sum256([]byte(signingInput))

	keyBytes := (tp.privateKey.Curve.Params().BitSize + 7) / 8
	if len(signature) != 2*keyBytes {
		return false
	}

	r := new(big.Int).SetBytes(signature[:keyBytes])
	s := new(big.Int).SetBytes(signature[keyBytes:])

	return ecdsa.Verify(&tp.privateKey.PublicKey, hash[:], r, s)
}

// splitToken splits a JWT token into its parts.
func splitToken(token string) []string {
	var parts []string
	start := 0
	for i := 0; i < len(token); i++ {
		if token[i] == '.' {
			parts = append(parts, token[start:i])
			start = i + 1
		}
	}
	parts = append(parts, token[start:])
	return parts
}
