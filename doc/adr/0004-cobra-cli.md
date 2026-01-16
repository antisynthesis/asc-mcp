# ADR-0004: Cobra CLI Framework

## Status

Accepted

## Context

While the primary function of asc-mcp is to run as an MCP server over stdio, users benefit from additional CLI commands for:

- Starting the server (`serve`)
- Validating configuration (`validate`)
- Listing available tools (`tools`)
- Displaying version information (`version`)

We need a CLI framework that provides consistent command structure, help generation, and flag parsing.

## Decision

We will use Cobra (github.com/spf13/cobra) as the CLI framework.

This is an exception to the "standard library only" guideline because:

1. Cobra is the de facto standard for Go CLIs
2. It has minimal dependencies (only spf13/pflag)
3. CLI parsing is not security-critical like cryptography
4. The alternative (hand-rolled CLI) would be significant effort for little benefit
5. Cobra provides excellent UX (auto-completion, help generation)

Commands implemented:
- `asc-mcp serve` - Start the MCP server
- `asc-mcp validate` - Check configuration validity
- `asc-mcp tools` - List available MCP tools
- `asc-mcp version` - Show version information
- `asc-mcp completion` - Generate shell completions (Cobra built-in)

## Consequences

### Positive

- Professional CLI experience
- Automatic help text generation
- Shell completion support
- Consistent flag parsing
- Well-tested, widely-used library

### Negative

- External dependency (Cobra + pflag + mousetrap)
- Increases binary size slightly
- Different from "standard library only" stance

### Mitigations

- Dependencies are minimal and well-maintained
- Security-critical code still uses standard library only
- Cobra is essentially an industry standard
