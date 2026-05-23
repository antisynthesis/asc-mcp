# ADR-0002: Model Context Protocol Implementation

## Status

Accepted

## Context

The Model Context Protocol (MCP) is a standard for enabling AI assistants like Claude to interact with external tools and data sources. MCP defines:

- Transport: JSON-RPC 2.0 over stdio (primary) or HTTP+SSE
- Methods: initialize, tools/list, tools/call, and others
- Capability negotiation between client and server

We need to implement an MCP server that exposes App Store Connect functionality as tools.

## Decision

We will implement MCP using:

1. **Transports**: two transports share a single `server.Dispatcher`
   and the same tool registry.
   - **stdio** (`asc-mcp serve`) — JSON-RPC messages on stdin/stdout,
     one session per process. This is the primary transport for desktop
     clients that spawn the server as a subprocess.
   - **Streamable HTTP** (`asc-mcp serve-http`) — the 2025-06-18 spec's
     single-endpoint HTTP shape. `POST /mcp` for JSON-RPC; `DELETE /mcp`
     to end a session; `GET /mcp` reserved for server-initiated SSE
     (currently returns 405 because we have no pushed messages).
     Sessions are identified by an `Mcp-Session-Id` header the server
     assigns on initialize. Defenses include a request-body size cap, an
     optional Origin allowlist (against DNS rebinding), validation of
     the `MCP-Protocol-Version` header, and idle session reaping.
2. **Protocol**: JSON-RPC 2.0 with proper request/response handling.
3. **Protocol version**: The server prefers `2025-06-18` and negotiates
   down to `2025-03-26` or `2024-11-05` for older clients via the
   `InitializeResult.protocolVersion` field, as required by the MCP
   versioning spec.
4. **Methods implemented**:
   - `initialize` — protocol handshake, capability exchange, version
     negotiation, and `instructions` describing the server.
   - `ping` — liveness check returning an empty result.
   - `notifications/initialized` — client signals it is ready (no response).
   - `notifications/cancelled` — accepted and ignored (no per-request
     cancellation token tracking yet).
   - `tools/list` — return available tools, including `title`,
     `inputSchema`, optional `outputSchema`, and `annotations`.
   - `tools/call` — execute a tool, returning either a `content` block
     and/or `structuredContent`. Tool-execution failures are returned as
     `isError: true` results so the model can self-correct; only unknown
     tool names map to a JSON-RPC error.
5. **Standard library only**: use `encoding/json` for JSON handling and
   `bufio` for line-by-line reading. The CLI is built on Cobra (see
   ADR-0004); the MCP protocol layer has no third-party dependencies.

The server will run as a subprocess spawned by the MCP client (e.g.,
Claude Desktop).

## Consequences

### Positive

- Single binary supports both stdio (for desktop clients) and HTTP
  (for hosted deployments), without duplicating the dispatcher logic.
- HTTP transport is compatible with the existing Kubernetes manifests
  (`/healthz` probe, `--allowed-origins` for DNS-rebinding defense).
- Stdio is secure by default (no ports), trivial to test from a shell.
- Tools that accept files support both `file_path` (stdio-friendly)
  and base64 `file_data_base64` (HTTP-friendly).

### Negative

- HTTP session storage is in-memory and per-process: running multiple
  replicas requires sticky sessions or an external session store.
- HTTP transport does not yet implement the optional GET-for-SSE
  channel, so server-initiated messages (sampling, elicitation) are
  not supported in either transport.
- Stdio still cannot serve multiple clients from a single process.

### Mitigations

- Logging goes to stderr in both transports so stdout stays clean for
  the JSON-RPC stream when using stdio.
- The HTTP server reaps idle sessions after 30 minutes to bound
  memory growth.
- The CLI's `tools` subcommand lets operators inspect the catalog
  without a live MCP session.
