// Package server provides the MCP server implementation for App Store Connect.
package server

import (
	"context"
	"encoding/json"
	"errors"
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
	}, nil
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

	switch req.Method {
	case "initialize":
		return d.handleInitialize(req, sess)
	case "notifications/initialized":
		// Client signals it's ready. No response permitted.
		return nil
	case "notifications/cancelled":
		// We do not currently track per-request cancellation tokens, so
		// we acknowledge by ignoring.
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
	var params mcp.ToolsCallParams
	if err := json.Unmarshal(req.Params, &params); err != nil {
		return errorResp(req.ID, mcp.ErrCodeInvalidParams, "Invalid params", err.Error())
	}
	callCtx, cancel := context.WithTimeout(ctx, toolCallTimeout)
	defer cancel()
	result, err := d.registry.CallTool(callCtx, params.Name, params.Arguments)
	if err != nil {
		// Per the MCP CallToolResult guidance, only protocol-level
		// errors (unknown tool) become JSON-RPC errors; tool execution
		// failures are surfaced as isError=true so the model can self-
		// correct.
		if errors.Is(err, tools.ErrUnknownTool) {
			return errorResp(req.ID, mcp.ErrCodeMethodNotFound, "Unknown tool", params.Name)
		}
		return resultResp(req.ID, mcp.NewErrorResult(err.Error()))
	}
	return resultResp(req.ID, result)
}

// resultResp builds a successful JSON-RPC response.
func resultResp(id json.RawMessage, result any) *mcp.Response {
	return &mcp.Response{JSONRPC: mcp.JSONRPCVersion, ID: id, Result: result}
}

// errorResp builds an error JSON-RPC response.
func errorResp(id json.RawMessage, code int, message, data string) *mcp.Response {
	return &mcp.Response{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      id,
		Error:   &mcp.RPCError{Code: code, Message: message, Data: data},
	}
}
