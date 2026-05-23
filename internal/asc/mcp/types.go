// Package mcp provides MCP protocol types.
//
// The types in this package follow the Model Context Protocol specification.
// See: https://modelcontextprotocol.io/specification
package mcp

import "encoding/json"

const (
	// JSONRPCVersion is the JSON-RPC version used by MCP.
	JSONRPCVersion = "2.0"

	// ProtocolVersion is the MCP protocol version this server prefers.
	ProtocolVersion = "2025-06-18"
)

// SupportedProtocolVersions lists every MCP protocol revision this server
// can speak, newest first. During initialize the server picks the latest
// version that the client also supports.
var SupportedProtocolVersions = []string{
	"2025-06-18",
	"2025-03-26",
	"2024-11-05",
}

// JSON-RPC error codes.
const (
	ErrCodeParse          = -32700
	ErrCodeInvalidRequest = -32600
	ErrCodeMethodNotFound = -32601
	ErrCodeInvalidParams  = -32602
	ErrCodeInternal       = -32603
)

// Request represents a JSON-RPC 2.0 request.
type Request struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      json.RawMessage `json:"id,omitempty"`
	Method  string          `json:"method"`
	Params  json.RawMessage `json:"params,omitempty"`
}

// IsNotification reports whether the request is a JSON-RPC notification
// (no id field, so no response is expected).
func (r *Request) IsNotification() bool {
	return len(r.ID) == 0
}

// Response represents a JSON-RPC 2.0 response.
type Response struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      json.RawMessage `json:"id,omitempty"`
	Result  any             `json:"result,omitempty"`
	Error   *RPCError       `json:"error,omitempty"`
}

// RPCError represents a JSON-RPC 2.0 error.
type RPCError struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
	Data    any    `json:"data,omitempty"`
}

// InitializeParams represents parameters for the initialize request.
type InitializeParams struct {
	ProtocolVersion string           `json:"protocolVersion"`
	Capabilities    ClientCapability `json:"capabilities"`
	ClientInfo      ClientInfo       `json:"clientInfo"`
}

// ClientCapability represents client capabilities.
type ClientCapability struct {
	Roots        *RootsCapability    `json:"roots,omitempty"`
	Sampling     *SamplingCapability `json:"sampling,omitempty"`
	Elicitation  *EmptyCapability    `json:"elicitation,omitempty"`
	Experimental map[string]any      `json:"experimental,omitempty"`
}

// RootsCapability represents roots capability.
type RootsCapability struct {
	ListChanged bool `json:"listChanged,omitempty"`
}

// SamplingCapability represents sampling capability.
type SamplingCapability struct{}

// EmptyCapability is used for capabilities that carry no fields today.
type EmptyCapability struct{}

// ClientInfo represents information about the client.
type ClientInfo struct {
	Name    string `json:"name"`
	Title   string `json:"title,omitempty"`
	Version string `json:"version"`
}

// InitializeResult represents the result of initialization.
type InitializeResult struct {
	ProtocolVersion string           `json:"protocolVersion"`
	Capabilities    ServerCapability `json:"capabilities"`
	ServerInfo      ServerInfo       `json:"serverInfo"`
	Instructions    string           `json:"instructions,omitempty"`
	Meta            map[string]any   `json:"_meta,omitempty"`
}

// ServerCapability represents server capabilities.
type ServerCapability struct {
	Tools        *ToolsCapability `json:"tools,omitempty"`
	Logging      *EmptyCapability `json:"logging,omitempty"`
	Resources    *ResourcesCap    `json:"resources,omitempty"`
	Prompts      *PromptsCap      `json:"prompts,omitempty"`
	Completions  *EmptyCapability `json:"completions,omitempty"`
	Experimental map[string]any   `json:"experimental,omitempty"`
}

// ToolsCapability represents tools capability.
type ToolsCapability struct {
	ListChanged bool `json:"listChanged,omitempty"`
}

// ResourcesCap represents the resources capability.
type ResourcesCap struct {
	Subscribe   bool `json:"subscribe,omitempty"`
	ListChanged bool `json:"listChanged,omitempty"`
}

// PromptsCap represents the prompts capability.
type PromptsCap struct {
	ListChanged bool `json:"listChanged,omitempty"`
}

// ServerInfo represents information about the server.
type ServerInfo struct {
	Name    string `json:"name"`
	Title   string `json:"title,omitempty"`
	Version string `json:"version"`
}

// ToolsListResult represents the result of tools/list.
type ToolsListResult struct {
	Tools      []Tool `json:"tools"`
	NextCursor string `json:"nextCursor,omitempty"`
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name         string           `json:"name"`
	Title        string           `json:"title,omitempty"`
	Description  string           `json:"description"`
	InputSchema  JSONSchema       `json:"inputSchema"`
	OutputSchema *JSONSchema      `json:"outputSchema,omitempty"`
	Annotations  *ToolAnnotations `json:"annotations,omitempty"`
	Meta         map[string]any   `json:"_meta,omitempty"`
}

// ToolAnnotations carries optional behavior hints about a tool.
// See the MCP spec for the meaning of each hint.
type ToolAnnotations struct {
	Title           string `json:"title,omitempty"`
	ReadOnlyHint    *bool  `json:"readOnlyHint,omitempty"`
	DestructiveHint *bool  `json:"destructiveHint,omitempty"`
	IdempotentHint  *bool  `json:"idempotentHint,omitempty"`
	OpenWorldHint   *bool  `json:"openWorldHint,omitempty"`
}

// BoolPtr is a small helper that returns a pointer to b. It is convenient
// for setting the optional ToolAnnotations hint fields.
func BoolPtr(b bool) *bool { return &b }

// JSONSchema represents a JSON Schema for tool input.
type JSONSchema struct {
	Type                 string              `json:"type"`
	Properties           map[string]Property `json:"properties,omitempty"`
	Required             []string            `json:"required,omitempty"`
	AdditionalProperties any                 `json:"additionalProperties,omitempty"`
}

// Property represents a JSON Schema property.
type Property struct {
	Type        string    `json:"type,omitempty"`
	Description string    `json:"description,omitempty"`
	Enum        []string  `json:"enum,omitempty"`
	Default     any       `json:"default,omitempty"`
	Items       *Property `json:"items,omitempty"`
	Format      string    `json:"format,omitempty"`
	Minimum     *float64  `json:"minimum,omitempty"`
	Maximum     *float64  `json:"maximum,omitempty"`
}

// ToolsCallParams represents parameters for tools/call.
type ToolsCallParams struct {
	Name      string          `json:"name"`
	Arguments json.RawMessage `json:"arguments,omitempty"`
}

// ToolsCallResult represents the result of tools/call.
type ToolsCallResult struct {
	Content           []ContentBlock `json:"content"`
	StructuredContent any            `json:"structuredContent,omitempty"`
	IsError           bool           `json:"isError,omitempty"`
	Meta              map[string]any `json:"_meta,omitempty"`
}

// ContentBlock represents a content block in tool results.
type ContentBlock struct {
	Type        string         `json:"type"`
	Text        string         `json:"text,omitempty"`
	Data        string         `json:"data,omitempty"`
	MimeType    string         `json:"mimeType,omitempty"`
	Annotations *Annotations   `json:"annotations,omitempty"`
	Meta        map[string]any `json:"_meta,omitempty"`
}

// Annotations is the audience/priority hint carried by content blocks.
type Annotations struct {
	Audience []string `json:"audience,omitempty"`
	Priority *float64 `json:"priority,omitempty"`
}

// NewTextContent creates a text content block.
func NewTextContent(text string) ContentBlock {
	return ContentBlock{
		Type: "text",
		Text: text,
	}
}

// NewSuccessResult creates a successful tool result.
func NewSuccessResult(text string) *ToolsCallResult {
	return &ToolsCallResult{
		Content: []ContentBlock{NewTextContent(text)},
	}
}

// NewErrorResult creates an error tool result. The error is reported as a
// successful JSON-RPC response with isError=true so that the model can see
// the failure and self-correct, as recommended by the MCP specification.
func NewErrorResult(text string) *ToolsCallResult {
	return &ToolsCallResult{
		Content: []ContentBlock{NewTextContent(text)},
		IsError: true,
	}
}

// NegotiateProtocolVersion picks the protocol version to use given the
// version proposed by the client. If the requested version is supported
// it is echoed back; otherwise the latest supported version is returned
// and the client decides whether it can speak it.
func NegotiateProtocolVersion(requested string) string {
	for _, v := range SupportedProtocolVersions {
		if v == requested {
			return v
		}
	}
	return ProtocolVersion
}
