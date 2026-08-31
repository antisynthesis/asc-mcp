// Package server provides the MCP server implementation for App Store Connect.
package server

import (
	"context"
	"encoding/json"
	"errors"
	"log/slog"
	"sync/atomic"
	"time"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/config"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
	"github.com/antisynthesis/asc-mcp/internal/asc/tools"
)

// Dispatcher handles JSON-RPC requests for an MCP server. It owns the
// long-lived state (config, API client, tool registry) that is shared
// across every transport and every session. Per-session state (such as
// whether the client has finished the initialize handshake) lives on
// the Session value passed into Dispatch.
type Dispatcher struct {
	cfg      *config.Config
	client   *api.Client
	registry *tools.Registry
	logger   *slog.Logger
	metrics  *metrics // optional; nil disables tool-call metrics
}

// NewDispatcher constructs a Dispatcher backed by an authenticated
// App Store Connect API client.
func NewDispatcher(cfg *config.Config) (*Dispatcher, error) {
	client, err := api.NewClient(cfg.IssuerID, cfg.KeyID, cfg.PrivateKeyPath)
	if err != nil {
		return nil, err
	}
	return &Dispatcher{
		cfg:      cfg,
		client:   client,
		registry: tools.NewRegistry(client),
		logger:   slog.Default(),
	}, nil
}

// SetLogger replaces the logger used for per-request structured logs.
// Callers typically configure this once during process startup.
func (d *Dispatcher) SetLogger(l *slog.Logger) {
	if l != nil {
		d.logger = l
	}
}

// Session carries the per-connection state that the dispatcher needs to
// enforce the initialize handshake. A Session is cheap to allocate and
// is shared by every request from a single MCP client.
type Session struct {
	initialized atomic.Bool

	// CreatedAt and LastActive let the HTTP transport reap idle
	// sessions; the stdio transport ignores them.
	CreatedAt  time.Time
	lastActive atomic.Value // time.Time
}

// NewSession returns a fresh Session timestamped at the current time.
func NewSession() *Session {
	s := &Session{CreatedAt: time.Now()}
	s.lastActive.Store(time.Now())
	return s
}

// Touch records that the session was just active.
func (s *Session) Touch() { s.lastActive.Store(time.Now()) }

// LastActive returns the most recent activity timestamp.
func (s *Session) LastActive() time.Time {
	v, _ := s.lastActive.Load().(time.Time)
	return v
}

// IsInitialized reports whether the client has completed the
// initialize handshake on this session.
func (s *Session) IsInitialized() bool { return s.initialized.Load() }

// Dispatch handles a single JSON-RPC request and returns the response
// to send back, or nil if the request was a notification or otherwise
// requires no response. The caller is responsible for delivering the
// response to the transport.
//
// The ctx applies to the entire request, including any downstream
// App Store Connect API call. A per-request tool-call timeout is
// layered on top inside the tools/call branch.
func (d *Dispatcher) Dispatch(ctx context.Context, req *mcp.Request, sess *Session) *mcp.Response {
	if req == nil {
		return nil
	}
	sess.Touch()

	if req.JSONRPC != mcp.JSONRPCVersion {
		if req.IsNotification() {
			return nil
		}
		return errorResp(req.ID, mcp.ErrCodeInvalidRequest, "Invalid Request", "jsonrpc must be 2.0")
	}

	// Era detection. 2026-07-28 clients have no initialize handshake:
	// they stamp the protocol version into params._meta on every request,
	// and server/discover only exists on the modern revision. initialize
	// itself is always legacy — NegotiateProtocolVersion answers a modern
	// probe there with the newest legacy version we speak.
	meta, err := mcp.ParseRequestMeta(req.Params)
	if err != nil {
		if req.IsNotification() {
			return nil
		}
		return errorResp(req.ID, mcp.ErrCodeInvalidParams, "Invalid params", err.Error())
	}
	if req.Method != "initialize" && (meta.ProtocolVersion != "" || req.Method == "server/discover") {
		return d.dispatchModern(ctx, req, meta)
	}

	switch req.Method {
	case "initialize":
		return d.handleInitialize(req, sess)
	case "notifications/initialized":
		// Client signals it's ready. No response permitted.
		return nil
	case "notifications/cancelled":
		// The stdio transport intercepts cancellation on its read loop
		// before Dispatch (it cancels the in-flight request's context);
		// on HTTP the client cancels by closing the connection, so a
		// notifications/cancelled arriving here is ignored per spec.
		return nil
	case "ping":
		if req.IsNotification() {
			return nil
		}
		return resultResp(req.ID, struct{}{})
	case "tools/list":
		return d.handleToolsList(req, sess)
	case "tools/call":
		return d.handleToolsCall(ctx, req, sess)
	default:
		if req.IsNotification() {
			return nil
		}
		return errorResp(req.ID, mcp.ErrCodeMethodNotFound, "Method not found", req.Method)
	}
}

// modernProtocolVersions lists the revisions accepted via per-request
// _meta (as opposed to the legacy initialize handshake). Future stateless
// revisions slot in here.
var modernProtocolVersions = []string{mcp.LatestProtocolVersion}

// cacheTTLMs is the freshness hint attached to cacheable modern results.
// One hour: the tool set is fixed for the lifetime of the process.
const cacheTTLMs int64 = 3_600_000

// serverInfoMeta builds the _meta serverInfo entry stamped into every
// modern result.
func serverInfoMeta() map[string]any {
	return mcp.ServerInfoMeta(mcp.ServerInfo{
		Name:    serverName,
		Title:   serverTitle,
		Version: serverVersion,
	})
}

// dispatchModern serves requests from 2026-07-28 clients. The modern
// revision is stateless — servers must not rely on prior requests for
// context — so there is no handshake and no session gating; every
// request is validated and served on its own.
func (d *Dispatcher) dispatchModern(ctx context.Context, req *mcp.Request, meta *mcp.RequestMeta) *mcp.Response {
	// server/discover is the version-discovery probe: it answers before
	// any _meta enforcement so clients can learn what we speak.
	if req.Method == "server/discover" {
		return d.handleDiscover(req)
	}

	// Every other modern request MUST carry a supported protocolVersion
	// and the clientCapabilities _meta key ({} is valid; absent is not).
	supported := false
	for _, v := range modernProtocolVersions {
		if v == meta.ProtocolVersion {
			supported = true
			break
		}
	}
	if !supported {
		if req.IsNotification() {
			return nil
		}
		return errorResp(req.ID, mcp.ErrCodeUnsupportedProtocolVersion, "Unsupported protocol version",
			mcp.UnsupportedProtocolVersionData{
				Supported: mcp.SupportedProtocolVersions,
				Requested: meta.ProtocolVersion,
			})
	}
	if meta.ClientCapabilities == nil {
		if req.IsNotification() {
			return nil
		}
		return errorResp(req.ID, mcp.ErrCodeInvalidParams, "Invalid params",
			"missing required _meta key "+mcp.MetaClientCapabilities)
	}

	switch req.Method {
	case "notifications/cancelled":
		// 2026-07-28 makes cancellation transport-specific: stdio clients
		// use this notification (intercepted by the stdio read loop
		// before Dispatch), while HTTP clients cancel by closing the
		// connection. One reaching Dispatch is therefore ignored.
		return nil
	case "tools/list":
		return d.handleModernToolsList(req)
	case "tools/call":
		return d.callTool(ctx, req, true)
	default:
		// ping and logging/setLevel were removed in 2026-07-28, and
		// subscriptions/listen is deliberately unimplemented: we
		// advertise listChanged:false and emit no notifications, so
		// there is nothing to listen to.
		if req.IsNotification() {
			return nil
		}
		return errorResp(req.ID, mcp.ErrCodeMethodNotFound, "Method not found", req.Method)
	}
}

// handleDiscover answers the mandatory 2026-07-28 discovery RPC. It is
// deliberately ungated — it doubles as the stdio backward-compat probe
// and must work as the very first message on a connection.
func (d *Dispatcher) handleDiscover(req *mcp.Request) *mcp.Response {
	if req.IsNotification() {
		return nil
	}
	return resultResp(req.ID, mcp.DiscoverResult{
		ResultType:        mcp.ResultTypeComplete,
		SupportedVersions: mcp.SupportedProtocolVersions,
		Capabilities: mcp.ServerCapability{
			Tools: &mcp.ToolsCapability{ListChanged: false},
		},
		Instructions: serverInstructions,
		TtlMs:        cacheTTLMs,
		CacheScope:   mcp.CacheScopePublic, // same result for every caller
		Meta:         serverInfoMeta(),
	})
}

// handleModernToolsList is tools/list for 2026-07-28 clients: no
// initialization gating, and the result carries the required cacheable-
// result fields. cacheScope is public because the tool set is identical
// for every caller (it must not vary per connection, and does not).
func (d *Dispatcher) handleModernToolsList(req *mcp.Request) *mcp.Response {
	ttl := cacheTTLMs
	return resultResp(req.ID, mcp.ToolsListResult{
		Tools:      d.registry.ListTools(),
		ResultType: mcp.ResultTypeComplete,
		TtlMs:      &ttl,
		CacheScope: mcp.CacheScopePublic,
		Meta:       serverInfoMeta(),
	})
}

// stampModern applies the 2026-07-28 result envelope to a tool result:
// resultType plus the serverInfo _meta key. Legacy results pass through
// untouched so their wire shape stays byte-identical for older clients.
func stampModern(result *mcp.ToolsCallResult, modern bool) *mcp.ToolsCallResult {
	if !modern || result == nil {
		return result
	}
	result.ResultType = mcp.ResultTypeComplete
	if result.Meta == nil {
		result.Meta = make(map[string]any)
	}
	for k, v := range serverInfoMeta() {
		if _, ok := result.Meta[k]; !ok {
			result.Meta[k] = v
		}
	}
	return result
}

func (d *Dispatcher) handleInitialize(req *mcp.Request, sess *Session) *mcp.Response {
	var params mcp.InitializeParams
	if len(req.Params) > 0 {
		if err := json.Unmarshal(req.Params, &params); err != nil {
			return errorResp(req.ID, mcp.ErrCodeInvalidParams, "Invalid params", err.Error())
		}
	}
	negotiated := mcp.NegotiateProtocolVersion(params.ProtocolVersion)
	sess.initialized.Store(true)
	return resultResp(req.ID, mcp.InitializeResult{
		ProtocolVersion: negotiated,
		Capabilities: mcp.ServerCapability{
			Tools: &mcp.ToolsCapability{ListChanged: false},
		},
		ServerInfo: mcp.ServerInfo{
			Name:    serverName,
			Title:   serverTitle,
			Version: serverVersion,
		},
		Instructions: serverInstructions,
	})
}

func (d *Dispatcher) handleToolsList(req *mcp.Request, sess *Session) *mcp.Response {
	if !sess.IsInitialized() {
		return errorResp(req.ID, mcp.ErrCodeInvalidRequest, "Not initialized", "initialize must be called first")
	}
	return resultResp(req.ID, mcp.ToolsListResult{Tools: d.registry.ListTools()})
}

func (d *Dispatcher) handleToolsCall(ctx context.Context, req *mcp.Request, sess *Session) *mcp.Response {
	if !sess.IsInitialized() {
		return errorResp(req.ID, mcp.ErrCodeInvalidRequest, "Not initialized", "initialize must be called first")
	}
	return d.callTool(ctx, req, false)
}

// callTool executes a tools/call request. It is shared by the legacy and
// modern paths; modern selects the 2026-07-28 result envelope and error
// codes.
func (d *Dispatcher) callTool(ctx context.Context, req *mcp.Request, modern bool) *mcp.Response {
	var params mcp.ToolsCallParams
	if err := json.Unmarshal(req.Params, &params); err != nil {
		return errorResp(req.ID, mcp.ErrCodeInvalidParams, "Invalid params", err.Error())
	}
	callCtx, cancel := context.WithTimeout(ctx, toolCallTimeout)
	defer cancel()

	start := time.Now()
	result, err := d.registry.CallTool(callCtx, params.Name, params.Arguments)
	elapsed := time.Since(start)

	outcome := "ok"
	if err != nil {
		if errors.Is(err, tools.ErrUnknownTool) {
			outcome = "unknown"
		} else {
			outcome = "error"
		}
	} else if result != nil && result.IsError {
		outcome = "error"
	}
	if d.metrics != nil {
		d.metrics.toolCalls.WithLabelValues(params.Name, outcome).Inc()
		d.metrics.toolDuration.WithLabelValues(params.Name).Observe(elapsed.Seconds())
	}
	d.logger.LogAttrs(ctx, slog.LevelInfo, "tool_call",
		slog.String("tool", params.Name),
		slog.String("result", outcome),
		slog.Duration("duration", elapsed),
	)

	if err != nil {
		// Per the MCP CallToolResult guidance, only protocol-level
		// errors (unknown tool) become JSON-RPC errors; tool execution
		// failures are surfaced as isError=true so the model can self-
		// correct.
		if errors.Is(err, tools.ErrUnknownTool) {
			// 2026-07-28 defines an unknown tool name as invalid params;
			// the legacy path keeps its historical -32601 mapping so
			// existing clients see an unchanged wire.
			if modern {
				return errorResp(req.ID, mcp.ErrCodeInvalidParams, "Unknown tool", params.Name)
			}
			return errorResp(req.ID, mcp.ErrCodeMethodNotFound, "Unknown tool", params.Name)
		}
		return resultResp(req.ID, stampModern(mcp.NewErrorResult(err.Error()), modern))
	}
	return resultResp(req.ID, stampModern(result, modern))
}

// resultResp builds a successful JSON-RPC response.
func resultResp(id json.RawMessage, result any) *mcp.Response {
	return &mcp.Response{JSONRPC: mcp.JSONRPCVersion, ID: id, Result: result}
}

// errorResp builds an error JSON-RPC response. data is usually a string
// but may be a structured value (e.g. UnsupportedProtocolVersionData).
func errorResp(id json.RawMessage, code int, message string, data any) *mcp.Response {
	return &mcp.Response{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      id,
		Error:   &mcp.RPCError{Code: code, Message: message, Data: data},
	}
}
