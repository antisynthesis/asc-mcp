// Package mcp provides MCP protocol types.
//
// The types in this package follow the Model Context Protocol specification.
// See: https://modelcontextprotocol.io/specification
package mcp

import "encoding/json"

const (
	// JSONRPCVersion is the JSON-RPC version used by MCP.
	JSONRPCVersion = "2.0"

	// LatestProtocolVersion is the newest MCP protocol revision this server
	// speaks: the modern, stateless revision. It has no initialize handshake;
	// clients select it per request via params._meta and the HTTP
	// MCP-Protocol-Version header.
	LatestProtocolVersion = "2026-07-28"

	// ProtocolVersion is the preferred LEGACY handshake version: the version
	// returned by initialize when the client's requested version is not a
	// legacy revision this server speaks. 2025-11-25 is the last
	// handshake-based revision, so it carries the newest legacy semantics.
	ProtocolVersion = "2025-11-25"
)

// SupportedProtocolVersions lists every MCP protocol revision this server
// can speak, newest first. Note that the newest revision (2026-07-28) is
// stateless and is never negotiated via initialize; see
// LegacyHandshakeVersions.
var SupportedProtocolVersions = []string{
	"2026-07-28",
	"2025-11-25",
	"2025-06-18",
	"2025-03-26",
	"2024-11-05",
}

// LegacyHandshakeVersions lists the protocol revisions negotiable via the
// initialize handshake, newest first. The 2026-07-28 revision is excluded
// because it has no initialize handshake at all: a client that sends
// initialize is by definition speaking a legacy revision.
var LegacyHandshakeVersions = []string{
	"2025-11-25",
	"2025-06-18",
	"2025-03-26",
	"2024-11-05",
}

// JSON-RPC error codes.
//
// The range -32020..-32099 is reserved exclusively for the MCP
// specification; implementations MUST NOT allocate their own codes there.
// The range -32000..-32019 is legacy/implementation-defined and new code
// MUST NOT allocate there either. Never emit -32002 or -32042.
const (
	ErrCodeParse          = -32700
	ErrCodeInvalidRequest = -32600
	ErrCodeMethodNotFound = -32601
	ErrCodeInvalidParams  = -32602
	ErrCodeInternal       = -32603

	// ErrCodeHeaderMismatch reports that the HTTP MCP-Protocol-Version
	// header disagrees with the protocol version in the request body.
	ErrCodeHeaderMismatch = -32020

	// ErrCodeMissingClientCapability reports that the server needs a client
	// capability the client did not declare; the error data carries
	// requiredCapabilities.
	ErrCodeMissingClientCapability = -32021

	// ErrCodeUnsupportedProtocolVersion reports that the requested protocol
	// version is not supported; the error data lists the supported versions
	// and echoes the requested one (see UnsupportedProtocolVersionData).
	ErrCodeUnsupportedProtocolVersion = -32022
)

// UnsupportedProtocolVersionData is the error data carried by
// ErrCodeUnsupportedProtocolVersion responses.
type UnsupportedProtocolVersionData struct {
	Supported []string `json:"supported"`
	Requested string   `json:"requested"`
}

// Reserved _meta keys defined by the MCP specification.
const (
	// MetaProtocolVersion carries the protocol revision on every 2026-07-28
	// request (REQUIRED).
	MetaProtocolVersion = "io.modelcontextprotocol/protocolVersion"

	// MetaClientCapabilities carries the client's capabilities on every
	// 2026-07-28 request (REQUIRED; {} is valid and means none).
	MetaClientCapabilities = "io.modelcontextprotocol/clientCapabilities"

	// MetaClientInfo carries the client implementation info (SHOULD).
	MetaClientInfo = "io.modelcontextprotocol/clientInfo"

	// MetaServerInfo carries the server implementation info, stamped into
	// every modern result (SHOULD).
	MetaServerInfo = "io.modelcontextprotocol/serverInfo"

	// MetaLogLevel carries the requested logging level.
	MetaLogLevel = "io.modelcontextprotocol/logLevel"

	// MetaSubscriptionID carries a resource subscription identifier.
	MetaSubscriptionID = "io.modelcontextprotocol/subscriptionId"
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

	// Extensions carries protocol extensions keyed by reverse-DNS names
	// (e.g. "io.modelcontextprotocol/tasks"). This server declares no
	// extensions of its own but accepts client ones without erroring.
	Extensions map[string]json.RawMessage `json:"extensions,omitempty"`
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

	// Extensions carries protocol extensions keyed by reverse-DNS names
	// (e.g. "io.modelcontextprotocol/tasks"). This server declares none.
	Extensions map[string]any `json:"extensions,omitempty"`
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
	Name        string `json:"name"`
	Title       string `json:"title,omitempty"`
	Version     string `json:"version"`
	Description string `json:"description,omitempty"`
}

// Result type values. On 2026-07-28 every result MUST include resultType;
// clients treat absence as "complete", which is why omitempty keeps legacy
// responses byte-identical.
const (
	ResultTypeComplete      = "complete"
	ResultTypeInputRequired = "input_required"
)

// CacheScope values for cacheable results.
const (
	CacheScopePublic  = "public"
	CacheScopePrivate = "private"
)

// ToolsListResult represents the result of tools/list.
//
// On 2026-07-28 the result MUST additionally carry resultType, ttlMs
// (freshness hint in milliseconds; 0 = immediately stale) and cacheScope.
// TtlMs is a pointer so legacy responses omit it while a modern ttlMs:0 is
// still representable.
type ToolsListResult struct {
	Tools      []Tool         `json:"tools"`
	NextCursor string         `json:"nextCursor,omitempty"`
	ResultType string         `json:"resultType,omitempty"`
	TtlMs      *int64         `json:"ttlMs,omitempty"`
	CacheScope string         `json:"cacheScope,omitempty"`
	Meta       map[string]any `json:"_meta,omitempty"`
}

// DiscoverResult is the result of the server/discover request (2026-07-28).
type DiscoverResult struct {
	ResultType        string           `json:"resultType"`        // always "complete"
	SupportedVersions []string         `json:"supportedVersions"` // protocol revisions this server accepts
	Capabilities      ServerCapability `json:"capabilities"`
	Instructions      string           `json:"instructions,omitempty"`
	TtlMs             int64            `json:"ttlMs"`           // REQUIRED (CacheableResult)
	CacheScope        string           `json:"cacheScope"`      // "public" | "private"
	Meta              map[string]any   `json:"_meta,omitempty"` // SHOULD carry MetaServerInfo
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
//
// On 2026-07-28 the result MUST include resultType; tools/call results do
// NOT carry ttlMs/cacheScope.
type ToolsCallResult struct {
	Content           []ContentBlock `json:"content"`
	StructuredContent any            `json:"structuredContent,omitempty"`
	IsError           bool           `json:"isError,omitempty"`
	ResultType        string         `json:"resultType,omitempty"`
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
// version proposed by the client during initialize. If the requested
// version is a legacy handshake version it is echoed back; otherwise the
// preferred legacy version is returned and the client decides whether it
// can speak it. The 2026-07-28 revision is deliberately never negotiated
// here: it has no initialize handshake, so a client sending initialize
// with protocolVersion "2026-07-28" gets the latest legacy version back.
func NegotiateProtocolVersion(requested string) string {
	for _, v := range LegacyHandshakeVersions {
		if v == requested {
			return v
		}
	}
	return ProtocolVersion
}

// RequestMeta is the reserved metadata carried in params._meta on
// 2026-07-28 requests. Every request (not notification) MUST carry
// protocolVersion and clientCapabilities ({} is valid and means no
// optional capabilities); clientInfo SHOULD be present.
type RequestMeta struct {
	ProtocolVersion    string            // io.modelcontextprotocol/protocolVersion
	ClientCapabilities *ClientCapability // io.modelcontextprotocol/clientCapabilities (nil = absent)
	ClientInfo         *ClientInfo       // io.modelcontextprotocol/clientInfo
	ProgressToken      json.RawMessage   // progressToken (string or integer)
}

// ParseRequestMeta extracts RequestMeta from raw request params. Returns a
// zero-value (not nil) RequestMeta when params or _meta is absent; returns
// an error only on malformed JSON. A ClientCapabilities key that is present
// (even as {}) yields a non-nil pointer, letting the dispatcher distinguish
// "absent" from "declared empty" when enforcing the REQUIRED rule.
func ParseRequestMeta(params json.RawMessage) (*RequestMeta, error) {
	meta := &RequestMeta{}
	if len(params) == 0 {
		return meta, nil
	}

	var envelope struct {
		Meta map[string]json.RawMessage `json:"_meta"`
	}
	if err := json.Unmarshal(params, &envelope); err != nil {
		return nil, err
	}
	if envelope.Meta == nil {
		return meta, nil
	}

	if raw, ok := envelope.Meta[MetaProtocolVersion]; ok {
		if err := json.Unmarshal(raw, &meta.ProtocolVersion); err != nil {
			return nil, err
		}
	}
	if raw, ok := envelope.Meta[MetaClientCapabilities]; ok {
		caps := &ClientCapability{}
		if err := json.Unmarshal(raw, caps); err != nil {
			return nil, err
		}
		meta.ClientCapabilities = caps
	}
	if raw, ok := envelope.Meta[MetaClientInfo]; ok {
		info := &ClientInfo{}
		if err := json.Unmarshal(raw, info); err != nil {
			return nil, err
		}
		meta.ClientInfo = info
	}
	if raw, ok := envelope.Meta["progressToken"]; ok {
		meta.ProgressToken = raw
	}
	return meta, nil
}

// ServerInfoMeta builds the _meta map carrying the server implementation
// info. Servers SHOULD include it in every modern result so clients can
// identify the implementation without a handshake.
func ServerInfoMeta(info ServerInfo) map[string]any {
	return map[string]any{MetaServerInfo: info}
}
