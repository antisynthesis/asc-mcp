# ADR-0005: JWT Authentication for App Store Connect

## Status

Accepted

## Context

Apple App Store Connect API requires JWT authentication with specific requirements:

- Algorithm: ES256 (ECDSA with P-256 and SHA-256)
- Token lifetime: Maximum 20 minutes
- Claims required: iss (issuer ID), iat (issued at), exp (expiration), aud (audience)
- Audience: "appstoreconnect-v1"

Authentication requires three pieces of information:
1. Issuer ID (UUID from App Store Connect)
2. Key ID (10-character identifier)
3. Private key (.p8 file in PEM format)

## Decision

We will implement JWT generation with the following approach:

1. **Token Generation**:
   - Build header: `{"alg":"ES256","kid":"<key_id>","typ":"JWT"}`
   - Build claims: `{"iss":"<issuer_id>","iat":<now>,"exp":<now+10min>,"aud":"appstoreconnect-v1"}`
   - Sign with ES256 using standard library crypto

2. **Token Caching**:
   - Cache tokens for their lifetime (minus buffer)
   - Regenerate automatically before expiration
   - Use 10-minute lifetime (well under 20-min max)

3. **Configuration**:
   - Environment variables for all credentials
   - `ASC_ISSUER_ID`: Issuer ID from App Store Connect
   - `ASC_KEY_ID`: Key ID from App Store Connect
   - `ASC_PRIVATE_KEY_PATH`: Path to .p8 private key file

4. **Security**:
   - Never log or expose tokens
   - Private key read once at startup
   - Validate key format on load

## Consequences

### Positive

- Follows Apple's authentication specification exactly
- Standard library implementation is auditable
- Token caching reduces API latency
- Environment variables keep secrets out of config files

### Negative

- Private key must be accessible on filesystem
- Token expiration requires refresh logic
- ES256 implementation must be precise

### Mitigations

- Kubernetes Secrets for key storage in production
- Automatic token refresh with safety buffer
- Testing against Apple's API validates implementation
