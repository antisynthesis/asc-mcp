# Changelog

All notable changes to this project will be documented here. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the
project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

For published releases, the auto-generated section between the
`v0.0.0` heading and the next versioned heading is overwritten by
goreleaser when a `vX.Y.Z` tag is pushed. Hand-maintained notes live
under `## [Unreleased]` until a release is cut.

## [Unreleased]

### Added
- MCP 2026-07-28 protocol support alongside the existing handshake era.
  Modern clients carry their protocol version and capabilities in
  per-request `_meta`, discover the server through the new mandatory
  `server/discover` RPC, and receive `resultType` with `ttlMs` /
  `cacheScope` caching hints. Adds the specification's reserved error
  codes (-32020, -32021, -32022) and serves modern Streamable HTTP
  requests without sessions, validating the required `Mcp-Method` and
  `Mcp-Name` headers.
- `notifications/cancelled` handling on stdio: requests dispatch
  concurrently and a cancelled request produces no response.
- 153 tools for App Store Connect capability areas added between API
  4.0 and 4.4.1: review submissions, webhooks, accessibility
  declarations, customer review summarizations, app tags, territory age
  ratings, Android to iOS app mapping, build uploads, TestFlight beta
  feedback, Apple-hosted background assets, Game Center leaderboard
  sets, activities, challenges and player submissions, in-app purchase
  and subscription versioning, subscription plan availability, in-app
  purchase offer codes, and app price schedule writes.
- Native TLS support on the HTTP transport via `--tls-cert` /
  `--tls-key` flags (TLS 1.2 minimum). Missing one of the pair is
  rejected so the listener cannot silently downgrade to plain HTTP.
- GitHub Actions CI workflow that runs gofmt, `go mod tidy` drift,
  `go vet`, race-enabled tests, a release-style static build,
  staticcheck, and govulncheck on every push and pull request.
- GitHub Actions release workflow driven by goreleaser, triggered by
  `vX.Y.Z` tags. Produces darwin/linux × amd64/arm64 archives, a
  checksum file, a GitHub release with grouped notes, and a
  multi-arch container image published to GHCR.
- `CHANGELOG.md` (this file).

### Changed
- Bundled App Store Connect OpenAPI reference updated from 4.2 to 4.4.1.
- App availability now uses the v2 endpoints and `appAvailabilityV2`.
- Game Center achievements and leaderboards, and app store version
  experiments, moved off endpoints Apple deprecated to their versioned
  v2 replacements. Build encryption declaration assignment now goes
  through the build relationship.
- `submit_app_for_review` drives the review submissions flow, which
  replaced the removed app store version submission endpoint. It now
  requires `app_id` alongside `version_id`.
- Argument validation failures come back as tool errors rather than
  protocol errors, so a model can see and correct them.

### Removed
- Pre-order and IDFA declaration tools, sandbox tester creation and
  deletion, build run cancellation, and app-scoped alternative
  distribution package listing. Apple removed each of these operations
  from the API; ending a pre-order is now
  `end_app_availability_pre_order`.
- Game Center version release tools. Apple deprecated the entire
  releases family; Game Center content reaches the store through
  `add_review_submission_item`.

## [1.0.0] - 2026-05-23

### Added
- MCP 2025-06-18 protocol implementation across both `serve` (stdio)
  and `serve-http` (Streamable HTTP) transports, including
  `initialize` with protocol version negotiation, `ping`,
  `notifications/initialized`, `notifications/cancelled`,
  `tools/list`, and `tools/call`.
- 203 tools covering the App Store Connect API surface: apps,
  builds, TestFlight, provisioning, localizations, reviews, IAPs,
  subscriptions, versions, phased releases, screenshots, previews,
  pre-orders, app events, analytics, app clips, Game Center,
  Xcode Cloud, users, pricing, availability, age rating, sandbox
  testers, promoted purchases, product pages, diagnostics, EULAs,
  alternative distribution, and asset uploads.
- Tool annotations (`readOnlyHint`, `destructiveHint`,
  `idempotentHint`, `openWorldHint`, `title`) on every tool so
  clients can gate destructive calls and render rich pickers.
- `structuredContent` alongside the markdown response for every
  list/get/create/update handler so clients can chain tool calls
  programmatically.
- Cursor pagination across every `list_*` tool plus optional
  `filter`, `sort`, `fields`, and `include` query knobs mapped to
  App Store Connect's JSON:API endpoints.
- Three asset upload tools (`upload_app_screenshot`,
  `upload_app_preview`, `upload_review_attachment`) that orchestrate
  Apple's reserve / chunked-PUT / commit handshake. Files can be
  passed by local path (stdio) or base64-encoded bytes (HTTP).
- Production-grade HTTP transport: Bearer token auth (multi-token
  for rotation, SHA-256 digest comparison via constant-time compare),
  Origin allowlist, 4 MiB request body cap, request timeouts, idle
  session reaping, Prometheus metrics on `/metrics`, and structured
  JSON logs through `log/slog`.
- HTTP client hardening: bounded retry with backoff on 429/5xx,
  `Retry-After` honoured, response size capped at 25 MiB, typed
  `*APIError` with `IsAuth` / `IsNotFound` / `IsRateLimited`
  classifiers, `url.PathEscape` on every interpolated ID.
- Kubernetes manifests with exec liveness/readiness/startup probes
  driven by `asc-mcp validate`.
