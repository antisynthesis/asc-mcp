# ADR-0007: MCP Tool Organization

## Status

Accepted (revised)

## Context

The MCP server exposes 356 tools spanning the public App Store Connect
API surface. The tool catalog grew well beyond the original four-category
sketch, so the organization needs to:

- Group tools by API domain for maintainability
- Stay discoverable by users and assistants
- Keep handler signatures uniform so new tools follow a clear pattern
- Make registration mechanical and additive

## Decision

Tools live under `src/tools/`, one module per App Store Connect domain.
Each module has:

1. A `register_<domain>_tools(r: &mut Registry)` function. It registers
   every tool in the domain with `r.register(tool(r##"{...}"##), handler!(handle_<name>))`.
   The JSON literal is the tool's exact definition: name, description,
   `inputSchema` and annotations.
2. One `async fn handle_<name>(c: Arc<Client>, args: Args) -> HandlerResult`
   per tool. It owns argument decoding (a local `json_struct!` params type),
   validation, the API call and response formatting.

`src/tools/all.rs` calls every `register_*` function in a fixed order, so
`tools/list` is deterministic. A new domain is added by:

1. Creating `src/tools/<domain>.rs` with its register function and
   handlers. List it in `src/tools/modules.rs`.
2. Calling the register function from `src/tools/all.rs`.

API endpoint methods live in `src/api/endpoints/<area>.rs` as `impl Client`
blocks. Resource and request models live in `src/api/model/`.

Domain modules (each owning a related group of tools):

| Module | Domain |
| --- | --- |
| `apps` | App management |
| `builds`, `build_uploads` | Builds and build uploads |
| `testflight`, `beta_review`, `beta_feedback` | TestFlight groups, testers, review, localizations, feedback |
| `provisioning` | Bundle IDs, certificates, devices, profiles |
| `localizations` | App info and version localizations |
| `reviews` | Customer reviews, responses, summarizations |
| `iap`, `subscriptions`, `promoted_purchases`, `iap_offer_codes` | In-app purchases, subscriptions, offers |
| `commerce_versions`, `commerce_pricing`, `pricing` | Product versions, localizations, images, prices, availability |
| `versions`, `review_submissions`, `phased_release` | App Store versions and submissions |
| `screenshots`, `uploads` | Screenshots, previews and asset uploads |
| `events`, `appclips`, `product_pages` | In-app events, App Clips, custom product pages and experiments |
| `analytics`, `reports`, `diagnostics` | Analytics, sales/finance reports, diagnostics |
| `gamecenter*` | Game Center detail, achievements, leaderboards, sets, activities, challenges, submissions |
| `xcode_cloud` | Xcode Cloud |
| `encryption`, `age_rating`, `availability`, `preorders` | Compliance, ratings, availability |
| `users`, `sandbox` | Users, invitations, sandbox testers |
| `webhooks`, `accessibility`, `app_tags`, `android_mapping`, `background_assets`, `misc` | Other App Store Connect areas |

Conventions:

- Tool names use `snake_case`, follow `verb_noun` (e.g. `list_apps`,
  `create_beta_group`), and are globally unique.
- Required parameters are declared in the `inputSchema.required` list.
- Validation failures (missing or invalid arguments) return
  `Ok(error_result(...))` so the model sees the error and can
  self-correct, per the MCP spec guidance for `CallToolResult`.
- Handlers run on their own task. The server's 60 s per-call timeout and
  client cancellation abort them by dropping the future.

## Consequences

### Positive

- New tools are additive: one module edit, plus one registration line for a new domain.
- Domain boundaries match the App Store Connect API surface, making
  related tools easy to locate.
- A uniform handler signature makes mechanical refactors safe.
- The `asc-mcp tools` subcommand lists the catalog without credentials.

### Negative

- 356 tools means a long `tools/list` response; clients should treat the
  catalog as paginated in their UI.
- Some resources are accessed via more than one domain (for example,
  versions appear under both `versions` and `localizations`).
- Adding a domain requires touching `all.rs` and `modules.rs`.

### Mitigations

- The MCP spec allows servers to set `tools.listChanged` and emit a
  notification when the catalog changes; we currently do not, since the
  catalog is static per build.
- Cross-domain references are documented inline in the tool description.
