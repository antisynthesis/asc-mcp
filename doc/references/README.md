# Reference Specifications

This directory contains reference specifications used in the development of asc-mcp.

## Contents

### Apple App Store Connect API

Location: `apple-asc/`

- `openapi.oas.json` - Official Apple App Store Connect OpenAPI 3.0 specification
  - Source: https://developer.apple.com/sample-code/app-store-connect/app-store-connect-openapi-specification.zip
  - Documentation: https://developer.apple.com/documentation/appstoreconnectapi

### Model Context Protocol (MCP)

Location: `mcp/`

- `schema.json` - JSON Schema for MCP protocol messages
- `index.mdx` - MCP specification overview
- `basic/` - Core protocol specifications
  - `lifecycle.mdx` - Connection lifecycle and initialization
  - `transports.mdx` - Transport mechanisms (stdio, HTTP+SSE)
- `server/` - Server capabilities
  - `tools.mdx` - Tool definitions and execution
  - `prompts.mdx` - Prompt templates
  - `resources.mdx` - Resource exposure
- `client/` - Client capabilities
  - `sampling.mdx` - LLM sampling requests
  - `roots.mdx` - Filesystem roots

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

# MCP Schema
curl -sL "https://raw.githubusercontent.com/modelcontextprotocol/specification/main/schema/2024-11-05/schema.json" \
  -o doc/references/mcp/schema.json
```
