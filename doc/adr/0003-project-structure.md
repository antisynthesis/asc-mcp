# ADR-0003: Project Structure

## Status

Accepted

## Context

Go projects benefit from consistent structure that separates concerns and makes code discoverable. The project needs to organize:

- Entry points
- Internal packages (not for external import)
- Operations and deployment configuration
- Scripts for development tasks
- Documentation

## Decision

We will use the following structure:

```
asc-mcp/
├── cmd/
│   └── asc-mcp/
│       └── main.go           # Minimal entry point
├── internal/
│   └── asc/
│       ├── api/              # App Store Connect API client
│       ├── cmd/              # Cobra command definitions
│       ├── config/           # Configuration handling
│       ├── mcp/              # MCP protocol types
│       ├── server/           # MCP server implementation
│       └── tools/            # MCP tool implementations
├── ops/
│   └── k8s/                  # Kubernetes manifests
├── script/                   # Development scripts (zsh)
├── e2e/                      # End-to-end tests
├── doc/
│   ├── adr/                  # Architecture Decision Records
│   └── openapi.yaml          # API specification
├── Dockerfile
├── Makefile
├── Tiltfile
└── go.mod
```

Key decisions:
- `internal/` prevents external import of implementation details
- Nested `internal/asc/` groups all App Store Connect related code
- `ops/` contains deployment configuration separate from application code
- `script/` uses zsh for consistency with macOS development environment
- `e2e/` separates integration tests from unit tests

## Consequences

### Positive

- Clear separation of concerns
- Internal packages cannot be accidentally imported by external code
- Easy to navigate and understand
- Follows established Go conventions
- Scripts and deployment separate from application logic

### Negative

- Deeper directory nesting for imports
- More files to manage

### Mitigations

- IDE navigation makes deep paths manageable
- Structure is self-documenting
