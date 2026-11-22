# ADR-0009: Rust Implementation and JSON Wire Format

## Status

Accepted. Replaces the deprecated ADR-0001, ADR-0003 and ADR-0004.

## Context

asc-mcp exposes 356 MCP tools. MCP clients, and the language models
behind them, depend on everything observable about those tools:

- the tool definitions (names, descriptions, JSON Schemas, annotations);
- the exact requests sent to App Store Connect;
- the wording of text results; and
- the shape of `structuredContent`.

Small accidental changes break clients silently, with no compile-time
signal:

- a renamed JSON key;
- a dropped `omitempty`;
- a float printed differently;
- re-worded decode errors.

The server also handles concurrent stdio requests, HTTP traffic and
signed asset uploads, so memory safety and predictable concurrency
matter.

## Decision

1. **Rust on Tokio**, with:
   - `reqwest` (rustls) for the App Store Connect client;
   - `axum`/`hyper` for Streamable HTTP;
   - `p256`/`sha2` (RustCrypto) for ES256 JWTs;
   - `clap` for the CLI;
   - `prometheus` for metrics.

   The release is a single static (musl) binary.
2. **An explicit JSON wire-format layer** (`src/wire/`) instead of
   `serde`-derived models. Wire types are declared with the `json_struct!`
   macro, which defines decoding and encoding from one field list. The
   layer pins down:
   - **Decoding:**
     - object members in document order;
     - keys matched exactly first, then case-insensitively;
     - `null` is a no-op for scalars and structs and resets slices, maps
       and optionals;
     - unknown keys are ignored;
     - the first type mismatch is reported, with stable error wording.
   - **Encoding:**
     - fields in declaration order;
     - `omitempty`;
     - nil versus empty slices and maps (`JsonSlice`, `JsonMap`);
     - sorted map keys;
     - shortest round-trip float formatting;
     - HTML-safe escaping.
   - **Shared conventions:**
     - RFC 3339 timestamps with layout formatting (`Timestamp`);
     - `%v`/`%q`-style text formatting helpers (`wire::fmt`);
     - URL path and query escaping (`wire::url`).
3. **Tool definitions are data.** Each tool's definition is embedded as
   JSON next to its handler, so `tools/list` output is reviewed as data
   and stays byte-stable.
4. **Protocol conformance first.** The MCP layer follows revision
   2026-07-28 exactly. The handshake revisions are kept for compatibility
   and are deprecated (ADR-0002).

## Consequences

**Positive**
- No data races in the concurrent transports. No runtime is needed.
- Wire output is deterministic and reviewable. Changing a key, flag or
  format is an explicit edit to a `json_struct!` field list or a
  definition, never a side effect.

**Negative**
- The wire layer is less familiar than `serde`. Contributors must use
  `json_struct!`, `JsonSlice` and `JsonMap` for wire types, or output
  will drift.
- Pre-signed upload PUTs carry reqwest's implicit `Accept: */*`.
  RFC 9110 treats that as equivalent to omitting the header.

**Mitigations**
- `src/wire/` has unit tests for every rule above.
- Integration tests in `tests/` cover both protocol eras on both
  transports, and every endpoint group against an in-process mock API.
