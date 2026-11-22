# Reference Specifications

This directory contains reference specifications used in the development of asc-mcp.

## Contents

### Apple App Store Connect API

Location: `apple-asc/`

- `openapi.oas.json` - Official Apple App Store Connect OpenAPI 3.0 specification
  - Source: https://developer.apple.com/sample-code/app-store-connect/app-store-connect-openapi-specification.zip
  - Documentation: https://developer.apple.com/documentation/appstoreconnectapi

### Model Context Protocol (MCP)

Location: `mcp/` — revision **2026-07-28** (the latest released revision;
the server also serves the legacy 2024-11-05 … 2025-11-25 handshake).

- `schema.json`, `schema.ts` - JSON Schema and TypeScript source of truth for protocol messages
- `index.mdx`, `changelog.mdx`, `deprecated.mdx` - Specification overview, changes since 2025-11-25, deprecations
- `basic/` - Base protocol (`index.mdx`), versioning
  - `patterns/` - Cancellation, progress, subscriptions, multi round-trip requests
  - `transports/` - stdio and Streamable HTTP
- `server/` - Discovery, tools, resources, prompts
  - `utilities/` - Caching, pagination, completion, logging
- `client/` - Elicitation, roots, sampling

Source: https://github.com/modelcontextprotocol/modelcontextprotocol
Documentation: https://modelcontextprotocol.io/

## Usage

These specifications are provided for reference during development and are not modified by this project. When implementing new features or debugging issues, consult these specifications for authoritative protocol and API details.

## Updates

To update these specifications:

```bash
# Apple App Store Connect API
curl -L -o /tmp/asc.zip "https://developer.apple.com/sample-code/app-store-connect/app-store-connect-openapi-specification.zip"
unzip -o /tmp/asc.zip -d doc/references/apple-asc/

# MCP schema (repeat for the .mdx pages under docs/specification/<rev>/)
REV=2026-07-28
curl -sL "https://raw.githubusercontent.com/modelcontextprotocol/modelcontextprotocol/main/schema/$REV/schema.json" \
  -o doc/references/mcp/schema.json
curl -sL "https://raw.githubusercontent.com/modelcontextprotocol/modelcontextprotocol/main/schema/$REV/schema.ts" \
  -o doc/references/mcp/schema.ts
```
