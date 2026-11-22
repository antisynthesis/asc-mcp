# ADR-0002: Model Context Protocol Implementation

## Status

Accepted

## Context

The Model Context Protocol (MCP) lets AI assistants call external tools
over JSON-RPC 2.0. The current revision, **2026-07-28**, is stateless:

- There is no `initialize` handshake. Every request carries its protocol
  version and client capabilities in `params._meta`.
- Servers MUST implement `server/discover`.
- Results carry `resultType`. List results also carry the caching hints
  `ttlMs` and `cacheScope`.
- The Streamable HTTP transport has no sessions. It requires mirrored
  `MCP-Protocol-Version`, `Mcp-Method` and `Mcp-Name` headers.

Earlier revisions (2024-11-05 through 2025-11-25) negotiate a version
through `initialize`. Over HTTP they use `Mcp-Session-Id` sessions.

We need to expose App Store Connect as MCP tools to current clients
without stranding clients that still speak a handshake revision.

## Decision

1. **Revision 2026-07-28 is the primary protocol.** It is served exactly as
   specified:
   - `server/discover`: rejects an unsupported `_meta` version with
     -32022.
   - Required `_meta` keys: a missing or malformed key gets -32602.
   - `tools/list` and `tools/call`: results carry `resultType`, and
     `_meta` carries `io.modelcontextprotocol/serverInfo`.
   - Caching: `ttlMs` and `cacheScope: "public"` on discover and
     `tools/list`.
   - `subscriptions/listen`: acknowledged with an empty filter, then
     closed gracefully. The tool set is static, so the server has
     nothing to notify.
   - Error codes: -32020, -32021 and -32022, each with its specified
     error data.
   - Methods the 2026-07-28 revision removed (`ping`, `logging/setLevel`)
     get -32601.
2. **Handshake revisions are served for compatibility and are
   deprecated.** `initialize` negotiates 2025-11-25, 2025-06-18,
   2025-03-26 or 2024-11-05. These paths will be removed in a future major
   release (see the CHANGELOG's Deprecated section).
3. **The era is chosen per request, never per connection.** A request is
   on the 2026-07-28 path if any of these hold:
   - its `_meta` names a protocol version;
   - it is `server/discover`;
   - over HTTP, its `MCP-Protocol-Version` header names 2026-07-28.

   `initialize` is always a handshake request.
4. **Transports** share one dispatcher and one tool registry:
   - **stdio** (`asc-mcp serve`):
     - Newline-delimited JSON-RPC on stdin/stdout.
     - Requests run concurrently, so `notifications/cancelled` can abort
       an in-flight request, and a cancelled request gets no response.
   - **Streamable HTTP** (`asc-mcp serve-http`, loopback by default) on
     `POST /mcp`:
     - 2026-07-28 requests are sessionless. Headers are validated
       byte-exactly; only `Mcp-Name` accepts the base64 sentinel.
     - Errors map to HTTP status codes as the spec requires.
     - Notifications get 202.
     - Handshake clients get `Mcp-Session-Id` sessions and may
       `DELETE /mcp`.
     - `GET /mcp` returns 405.
     - `Origin` is always validated: loopback, same-origin, or
       `--allowed-origins`.
     - Other defences: Bearer tokens, a 4 MiB body cap and optional TLS.
5. **Tool failures are results, not protocol errors.** Validation and API
   failures return `isError: true` so the model can self-correct. An
   unknown tool name is -32602 on 2026-07-28 and -32601 on the handshake
   path.
6. **Implementation**:
   - Tokio for I/O.
   - `axum`/`hyper` for HTTP.
   - The JSON wire-format layer from ADR-0009 for bodies.

## Consequences

### Positive

- Current clients get a stateless server that scales horizontally behind
  any load balancer.
- Handshake clients keep working unchanged during the deprecation window.
- One binary serves desktop subprocess use (stdio) and hosted deployments
  (HTTP).

### Negative

- Two protocol eras must be maintained until the handshake revisions are
  removed.
- Handshake-era HTTP sessions are held in memory per process, so replicas
  need sticky routing for those clients.

### Mitigations

- Era selection is isolated in the dispatcher and the HTTP front end. The
  deprecated paths can be deleted without touching tool code.
- Idle handshake sessions are reaped after 30 minutes.
- `tests/` covers both eras on both transports.
