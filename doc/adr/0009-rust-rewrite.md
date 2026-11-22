# ADR-0009: Rewrite in Rust with Wire Parity

## Status

Accepted. Supersedes ADR-0001 (standard-library cryptography), ADR-0003
(Go project structure) and ADR-0004 (Cobra CLI).

## Context

The server was written in Go. It exposes 356 MCP tools whose observable
behaviour is relied on by MCP clients and, through them, by language
models:

- the tool definitions (names, descriptions, JSON Schemas, annotations);
- the exact requests sent to App Store Connect;
- the wording of the text results; and
- the shape of `structuredContent`.

The decision was taken to rewrite the server in Rust. In the same change
the MCP layer was brought fully in line with the latest released protocol
revision, 2026-07-28, while legacy handshake clients stay supported.

A rewrite of this size carries a high risk of small, silent behaviour
changes: a renamed JSON key, a dropped `omitempty`, a float printed
differently, or an error worded differently. Those changes would reach
users without any compile-time signal.

## Decision

1. **Rust on Tokio**, with:
   - `reqwest` (rustls) for the App Store Connect client;
   - `axum`/`hyper` for Streamable HTTP;
   - `p256`/`sha2` (RustCrypto) for ES256 JWTs;
   - `clap` for the CLI;
   - `prometheus` for metrics.
2. **A Go-semantics compatibility layer** (`src/go/`) instead of
   `serde`-derived models. It reproduces the parts of Go's standard
   library that define the wire format:
   - `encoding/json`: decoding in document order; exact-then-case-
     insensitive key matching; `null` handling; first-error reporting
     with Go's exact messages; `omitempty`; nil versus empty slices and
     maps; sorted map keys; float formatting; HTML-safe escaping.
   - `fmt` verbs, `strconv.Quote`, `time` RFC 3339 parsing and layouts,
     `net/url` escaping, and `os` error text.

   API models and tool argument structs are declared with the `go_struct!`
   macro, which derives both directions from one field list.
3. **Generated surface.** A one-off generator read the Go AST and the Go
   registry's runtime output and produced:
   - the API model structs;
   - the endpoint method signatures;
   - the tool registrations (definitions embedded verbatim as JSON); and
   - the handler argument structs.

   Handler and endpoint bodies were ported by hand.
4. **Differential parity testing** (`tests/parity/`):
   - The harness runs the Go server (rebuilt from git history) and the
     Rust server side by side against a mock App Store Connect API.
   - The mock synthesizes every response from Apple's OpenAPI document,
     in several modes: rich, alternate, sparse, empty, 404, and raw
     error.
   - Every tool runs through a matrix of argument variants.
   - The harness compares every outgoing request and the complete
     `tools/call` result.

   CI runs it on every change.
5. **Protocol conformance over parity.** Where the Go server deviated from
   a MUST or SHOULD of MCP 2026-07-28, the Rust server follows the spec;
   each such change is listed in the CHANGELOG. Examples:
   - `server/discover` version checks;
   - HTTP header validation and era detection;
   - `Origin` validation;
   - loopback default bind;
   - `subscriptions/listen`.

## Consequences

**Positive**
- Memory safety and no data races in the concurrent stdio and HTTP
  transports, with a single static binary (musl) and no runtime.
- Wire compatibility is measured, not assumed. The harness catches
  regressions anywhere in the 356-tool surface.
- The compatibility layer keeps future ports from Go-era code
  mechanical.

**Negative**
- The compatibility layer is unusual Rust. Contributors must use
  `go_struct!`/`GoSlice`/`GoMap` for wire types rather than `serde`
  derives, or output will drift.
- A few Go runtime details cannot be reproduced and are documented as
  accepted differences:
  - the text of Go `net/http` transport errors;
  - Go runtime metrics (`go_*`);
  - normalisation of exponent literals in a handful of decode-error
    messages.
- Running the parity harness needs a Go toolchain, because the reference
  binary is rebuilt from history.

**Mitigations**
- `src/go/` is covered by unit tests built from observed Go output.
- `make parity` and the CI `parity` job keep the Go reference
  reproducible.
