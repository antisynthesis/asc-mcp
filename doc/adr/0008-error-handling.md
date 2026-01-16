# ADR-0008: Error Handling Strategy

## Status

Accepted

## Context

The MCP server handles errors at multiple levels:

1. MCP protocol errors (invalid JSON-RPC)
2. Tool execution errors (invalid input, failed operations)
3. App Store Connect API errors (HTTP errors, rate limits)
4. Configuration errors (missing credentials)

Errors must be communicated clearly to:
- The MCP client (Claude)
- Human users debugging issues
- Operators monitoring production

## Decision

We will implement a layered error handling strategy:

1. **JSON-RPC Errors** (protocol level):
   - Use standard error codes: -32700 (parse), -32600 (invalid request), -32601 (method not found), -32602 (invalid params), -32603 (internal)
   - Return structured error responses per JSON-RPC 2.0 spec

2. **Tool Errors** (application level):
   - Return `isError: true` in tool response content
   - Include human-readable error message
   - Preserve API error details for debugging

3. **API Errors** (external):
   - Parse App Store Connect error responses
   - Map HTTP status codes to meaningful messages
   - Handle rate limiting with appropriate messaging
   - Include request ID for Apple support escalation

4. **Logging**:
   - Log to stderr (separate from protocol stdout)
   - Include context (tool name, request ID)
   - Log at appropriate levels (error, warn, info)

Error Message Format:
```
Error: <brief description>
Details: <specific information>
Code: <error code if applicable>
```

## Consequences

### Positive

- Clear separation of error types
- AI assistants receive actionable error messages
- Debugging information preserved
- Protocol compliance maintained

### Negative

- Error handling adds code complexity
- Must maintain error code mappings
- Verbose for simple cases

### Mitigations

- Helper functions for common error patterns
- Centralized error code definitions
- Tests verify error format consistency
