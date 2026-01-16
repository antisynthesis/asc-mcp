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

1. **Transport**: stdio (stdin/stdout) as the primary and only transport
2. **Protocol**: JSON-RPC 2.0 with proper request/response handling
3. **Methods implemented**:
   - `initialize` - Protocol handshake and capability exchange
   - `tools/list` - Return available tools with JSON Schema
   - `tools/call` - Execute a tool and return results
4. **Standard library only**: Use `encoding/json` for JSON handling, `bufio` for line-by-line reading

The server will run as a subprocess spawned by the MCP client (e.g., Claude Desktop).

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
