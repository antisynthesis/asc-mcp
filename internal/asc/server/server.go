// Package server provides the MCP server implementation for App Store Connect.
package server

import (
	"bufio"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log"
	"sync"
	"time"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/config"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
	"github.com/antisynthesis/asc-mcp/internal/asc/tools"
)

const (
	serverName    = "asc-mcp"
	serverTitle   = "App Store Connect"
	serverVersion = "1.0.0"

	// toolCallTimeout bounds the time a single tool invocation may take.
	// App Store Connect calls are usually fast; this prevents a stuck
	// upstream from hanging the MCP session.
	toolCallTimeout = 60 * time.Second

	// serverInstructions is returned in the initialize result so clients
	// can give the model an idea of what this server is good at.
	serverInstructions = `This server provides tools for managing Apple App Store Connect resources, including apps, builds, TestFlight, provisioning, in-app purchases, subscriptions, reviews, screenshots, localizations, analytics, Game Center, Xcode Cloud, and more. Tool names use snake_case and accept JSON arguments. Call tools/list to discover the available tools and their input schemas.`
)

// Server represents the MCP server for App Store Connect.
type Server struct {
	cfg         *config.Config
	client      *api.Client
	reader      *bufio.Reader
	writer      io.Writer
	writeMu     sync.Mutex
	initialized bool
	registry    *tools.Registry
}

// New creates a new MCP server instance.
func New(cfg *config.Config, r io.Reader, w io.Writer) (*Server, error) {
	client, err := api.NewClient(cfg.IssuerID, cfg.KeyID, cfg.PrivateKeyPath)
	if err != nil {
		return nil, fmt.Errorf("failed to create API client: %w", err)
	}

	registry := tools.NewRegistry(client)

	return &Server{
		cfg:      cfg,
		client:   client,
		reader:   bufio.NewReader(r),
		writer:   w,
		registry: registry,
	}, nil
}

// Run starts the MCP server and processes requests.
func (s *Server) Run() error {
	log.Printf("MCP server %s v%s starting (protocol %s)", serverName, serverVersion, mcp.ProtocolVersion)

	for {
		line, err := s.reader.ReadBytes('\n')
		if err != nil {
			if errors.Is(err, io.EOF) {
				log.Printf("client disconnected")
				return nil
			}
			return fmt.Errorf("failed to read request: %w", err)
		}

		if len(line) == 0 || (len(line) == 1 && line[0] == '\n') {
			continue
		}

		var req mcp.Request
		if err := json.Unmarshal(line, &req); err != nil {
			s.sendError(nil, mcp.ErrCodeParse, "Parse error", err.Error())
			continue
		}

		s.handleRequest(&req)
	}
}

// handleRequest dispatches a request to the appropriate handler.
func (s *Server) handleRequest(req *mcp.Request) {
	if req.JSONRPC != mcp.JSONRPCVersion {
		if !req.IsNotification() {
			s.sendError(req.ID, mcp.ErrCodeInvalidRequest, "Invalid Request", "jsonrpc must be 2.0")
		}
		return
	}

	switch req.Method {
	case "initialize":
		s.handleInitialize(req)
	case "notifications/initialized":
		// Client notification, no response permitted.
		log.Printf("client initialized")
	case "notifications/cancelled":
		// We do not currently track per-request cancellation tokens, so
		// just acknowledge the notification by ignoring it.
	case "ping":
		s.handlePing(req)
	case "tools/list":
		s.handleToolsList(req)
	case "tools/call":
		s.handleToolsCall(req)
	default:
		if req.IsNotification() {
			return
		}
		s.sendError(req.ID, mcp.ErrCodeMethodNotFound, "Method not found", req.Method)
	}
}

// handleInitialize handles the initialize request.
func (s *Server) handleInitialize(req *mcp.Request) {
	var params mcp.InitializeParams
	if req.Params != nil {
		if err := json.Unmarshal(req.Params, &params); err != nil {
			s.sendError(req.ID, mcp.ErrCodeInvalidParams, "Invalid params", err.Error())
			return
		}
	}

	negotiated := mcp.NegotiateProtocolVersion(params.ProtocolVersion)
	log.Printf("initializing with client: %s v%s (requested protocol %q, using %q)",
		params.ClientInfo.Name, params.ClientInfo.Version, params.ProtocolVersion, negotiated)

	result := mcp.InitializeResult{
		ProtocolVersion: negotiated,
		Capabilities: mcp.ServerCapability{
			Tools: &mcp.ToolsCapability{
				ListChanged: false,
			},
		},
		ServerInfo: mcp.ServerInfo{
			Name:    serverName,
			Title:   serverTitle,
			Version: serverVersion,
		},
		Instructions: serverInstructions,
	}

	s.initialized = true
	s.sendResult(req.ID, result)
}

// handlePing handles the ping request defined by the MCP spec.
// Per the spec, the server replies with an empty object.
func (s *Server) handlePing(req *mcp.Request) {
	if req.IsNotification() {
		return
	}
	s.sendResult(req.ID, struct{}{})
}

// handleToolsList handles the tools/list request.
func (s *Server) handleToolsList(req *mcp.Request) {
	if !s.initialized {
		s.sendError(req.ID, mcp.ErrCodeInvalidRequest, "Not initialized", "initialize must be called first")
		return
	}

	result := mcp.ToolsListResult{
		Tools: s.registry.ListTools(),
	}

	s.sendResult(req.ID, result)
}

// handleToolsCall handles the tools/call request.
func (s *Server) handleToolsCall(req *mcp.Request) {
	if !s.initialized {
		s.sendError(req.ID, mcp.ErrCodeInvalidRequest, "Not initialized", "initialize must be called first")
		return
	}

	var params mcp.ToolsCallParams
	if err := json.Unmarshal(req.Params, &params); err != nil {
		s.sendError(req.ID, mcp.ErrCodeInvalidParams, "Invalid params", err.Error())
		return
	}

	ctx, cancel := context.WithTimeout(context.Background(), toolCallTimeout)
	defer cancel()

	result, err := s.registry.CallTool(ctx, params.Name, params.Arguments)
	if err != nil {
		// Per the MCP spec, tool execution failures should be surfaced as
		// a successful response with isError=true so the model can self-
		// correct. We only emit a protocol-level error for unknown tools.
		if errors.Is(err, tools.ErrUnknownTool) {
			s.sendError(req.ID, mcp.ErrCodeMethodNotFound, "Unknown tool", params.Name)
			return
		}
		s.sendResult(req.ID, mcp.NewErrorResult(err.Error()))
		return
	}

	s.sendResult(req.ID, result)
}

// sendResult sends a successful response.
func (s *Server) sendResult(id json.RawMessage, result any) {
	resp := mcp.Response{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      id,
		Result:  result,
	}
	s.send(resp)
}

// sendError sends an error response.
func (s *Server) sendError(id json.RawMessage, code int, message, data string) {
	resp := mcp.Response{
		JSONRPC: mcp.JSONRPCVersion,
		ID:      id,
		Error: &mcp.RPCError{
			Code:    code,
			Message: message,
			Data:    data,
		},
	}
	s.send(resp)
}

// send writes a response to the output.
func (s *Server) send(resp mcp.Response) {
	s.writeMu.Lock()
	defer s.writeMu.Unlock()

	data, err := json.Marshal(resp)
	if err != nil {
		log.Printf("failed to marshal response: %v", err)
		return
	}

	data = append(data, '\n')
	if _, err := s.writer.Write(data); err != nil {
		log.Printf("failed to write response: %v", err)
	}
}
