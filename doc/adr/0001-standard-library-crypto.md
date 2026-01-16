# ADR-0001: Use Standard Library for Cryptographic Operations

## Status

Accepted

## Context

The asc-mcp server requires JWT authentication with ES256 (ECDSA with P-256 curve and SHA-256) to communicate with the Apple App Store Connect API. There are several options for implementing this:

1. Use external cryptographic libraries (e.g., golang-jwt/jwt)
2. Use Go standard library crypto packages
3. Use Apple's own SDK (not available for Go)

The project has a hard requirement to avoid external dependencies where possible, keeping the codebase minimal and auditable.

## Decision

We will use Go's standard library crypto packages for all cryptographic operations:

- `crypto/ecdsa` for ECDSA key handling and signing
- `crypto/x509` for parsing PEM-encoded private keys
- `crypto/sha256` for SHA-256 hashing
- `encoding/base64` for Base64URL encoding
- `encoding/pem` for PEM block parsing

The JWT generation will be implemented manually following RFC 7519 and the ES256 algorithm specification.

## Consequences

### Positive

- Zero external dependencies for cryptographic operations
- Full control over the implementation
- Easier security auditing with fewer moving parts
- No supply chain risks from third-party crypto libraries
- Standard library is well-tested and maintained by the Go team

### Negative

- More code to write and maintain
- Must ensure correct implementation of JWT spec
- Cannot easily swap algorithms without code changes
- No automatic handling of edge cases that mature libraries might cover

### Mitigations

- Comprehensive testing of JWT generation against known-good implementations
- Code review focused on security-critical sections
- Limited scope (only ES256 needed for App Store Connect)
