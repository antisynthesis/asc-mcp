# ADR-0007: MCP Tool Organization

## Status

Accepted (revised)

## Context

The MCP server exposes ~200 tools spanning the public App Store Connect
API surface. The tool catalog grew well beyond the original four-category
sketch, so the organization needs to:

- Group tools by API domain for maintainability
- Stay discoverable by users and assistants
- Keep handler signatures uniform so new tools follow a clear pattern
- Make registration mechanical and additive

## Decision

Tools live under `internal/asc/tools/`. Each domain has a `<domain>.go`
file with:

1. A `register<Domain>Tools()` method on `*Registry` that wires every
   tool in that domain via `r.register(mcp.Tool{...}, r.handle<Name>)`.
2. One `handle<Name>(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error)`
   per tool, owning argument parsing, validation, API invocation, and
   response formatting.

The central `NewRegistry` constructor in `internal/asc/tools/registry.go`
invokes every `register*Tools()` method, so a new domain is added by:

1. Creating `internal/asc/tools/<domain>.go` with the `register<Domain>Tools()`
   method and handlers.
2. Adding `r.register<Domain>Tools()` to `NewRegistry`.

Current domain files (each owning a related group of tools):

| File | Domain |
| --- | --- |
| `apps.go` | App management |
| `builds.go` | Builds |
| `testflight.go` | Beta groups, testers |
| `provisioning.go` | Bundle IDs, certificates, devices, profiles |
| `localizations.go` | App info & version localizations |
| `reviews.go` | Customer reviews and responses |
| `iap.go` / `subscriptions.go` | In-app purchases and subscriptions |
| `versions.go` | App Store versions, submissions |
| `phased_release.go` | Phased releases |
| `screenshots.go` | Screenshots and previews |
| `preorders.go` | Pre-orders |
| `events.go` | App events |
| `analytics.go` | Analytics report requests/data |
| `appclips.go` | App Clips |
| `gamecenter.go` | Game Center configuration |
| `xcode_cloud.go` | Xcode Cloud |
| `reports.go` | Sales and finance reports |
| `encryption.go` | Encryption declarations |
| `users.go` | Users, roles, invitations |
| `pricing.go` | Pricing |
| `availability.go` | Territory availability |
| `age_rating.go` | Age rating and IDFA |
| `beta_review.go` | Beta review and agreements |
| `sandbox.go` | Sandbox testers |
| `promoted_purchases.go` | Promoted IAPs and offer codes |
| `product_pages.go` | Custom product pages and experiments |
| `diagnostics.go` | Diagnostics and metrics |
| `misc.go` | EULA, categories, alternative distribution |

Conventions:

- Tool names use `snake_case`, follow `verb_noun` (e.g. `list_apps`,
  `create_beta_group`), and are globally unique.
- Required parameters are declared in the `inputSchema.required` list.
- Validation failures (missing or invalid arguments) return
  `(mcp.NewErrorResult(...), nil)` so the model sees the error and can
  self-correct, per the MCP spec guidance for `CallToolResult`.
- Handlers accept a `context.Context` and forward it to the API client,
  allowing the per-call timeout from the server to propagate.

## Consequences

### Positive

- New tools are additive — one file edit plus one registration line.
- Domain boundaries match the App Store Connect API surface, making
  related tools easy to locate.
- A uniform handler signature makes mechanical refactors safe.
- `tools list` CLI subcommand can group output by file/domain.

### Negative

- 200 tools means a long `tools/list` response; clients should treat the
  catalog as paginated in their UI.
- Some resources are accessed via more than one domain (for example,
  versions appear under both `versions.go` and `localizations.go`).
- Adding a tool requires touching the domain file and `NewRegistry`.

### Mitigations

- The MCP spec allows servers to set `tools.listChanged` and emit a
  notification when the catalog changes; we currently do not, since the
  catalog is static per build.
- Cross-domain references are documented inline in the tool description.
