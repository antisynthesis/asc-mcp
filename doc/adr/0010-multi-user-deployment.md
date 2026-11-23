# ADR-0010: Multi-User Deployment Model

## Status

Accepted

## Context

The server runs as a shared service on an agent platform. There, many
users, each driving one or more autonomous agents, call App Store Connect
tools concurrently. Each user may act for a different App Store Connect
team. The platform already runs an OAuth 2.1 authorization server. A
single process-wide API key would not work, and neither would a static
bearer token:

- every user would act with the same App Store Connect permissions;
- no request could be attributed to a user; and
- one runaway agent could exhaust the service for everyone.

## Decision

1. **The HTTP transport is an OAuth 2.1 resource server.** It follows the
   MCP authorization specification:
   - Discovery:
     - RFC 9728 Protected Resource Metadata;
     - `WWW-Authenticate` challenges on 401 and 403, carrying
       `resource_metadata` and `scope`.
   - Token validation:
     - JWT access tokens are verified against the issuer's JWKS,
       discovered via RFC 8414 or OIDC metadata after an issuer check;
     - the audience must be the server's canonical URI (RFC 8707);
     - `exp`/`nbf` are enforced;
     - only asymmetric algorithms are accepted.
   - Scopes:
     - `asc:read` for read-only tools and other read operations;
     - `asc:write` for state-changing tools; write implies read.
2. **Callers supply their App Store Connect key per request.** The key
   travels in `X-ASC-Issuer-ID`, `X-ASC-Key-ID` and `X-ASC-Private-Key`
   headers:
   - The server mints its own short-lived App Store Connect JWT from it.
     A key-digest-indexed pool caches the client, which shares one
     connection pool, and is bounded and idle-evicted.
   - Neither the OAuth access token nor any caller-minted App Store
     Connect token is forwarded upstream; the MCP specification forbids
     token passthrough.
   - The server stores no App Store Connect secrets.
3. **Request context, not shared state.**
   - Caller identity, the caller's client, the request ID, the trace ID
     and the upload policy travel with each request as a task-local
     context.
   - Handlers receive only their caller's client.
4. **Containment for autonomous agents:**
   - a per-caller concurrency limit (429 with `Retry-After`);
   - per-call timeouts, with cancellation on client disconnect;
   - a configurable body limit;
   - deprecated handshake sessions bound to their creator.

   Over HTTP, `file_path` uploads are disabled unless `--upload-dir`
   names a directory. Symlinks are resolved before the containment check.
5. **Statelessness for scale.** The 2026-07-28 path keeps no per-client
   state, so replicas scale horizontally without sticky routing.
   Per-process state is a cache only: pooled clients, JWKS keys and
   concurrency counters.

## Consequences

**Positive**
- Every call is attributable (subject, client ID, request ID, trace ID)
  and limited to the caller's own App Store Connect key.
- Standard MCP clients discover the platform's authorization server
  without custom configuration.
- Horizontal scaling needs no shared session store.

**Negative**
- Private keys travel on every request, so TLS is mandatory in
  deployment. The server warns when per-request credentials are enabled
  on a non-loopback listener without in-process TLS.
- The concurrency limit is per process. Platform-wide quotas belong in
  the gateway.
- Only JWT access tokens are supported. Opaque tokens would need RFC 7662
  introspection.

**Mitigations**
- `tests/multiuser.rs` covers metadata discovery, every 401 and 403 path,
  algorithm confusion, audience, issuer and expiry rejection, credential
  isolation, upload containment, the concurrency limit and session
  ownership.
