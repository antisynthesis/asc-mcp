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

1. **Transport**: stdio (stdin/stdout) as the primary and only transport.
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

- Simple deployment model (single binary)
- No network configuration required
- Secure by default (no exposed ports)
- Easy testing via command line
- Compatible with Claude Desktop and other MCP clients

### Negative

- Cannot be shared across multiple clients simultaneously
- No remote access without additional tooling
- Debugging requires capturing stdio

### Mitigations

- Logging to stderr (separate from protocol stdout)
- CLI commands for testing tools without full MCP session
- Future HTTP transport could be added if needed
