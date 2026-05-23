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

// Server represents the stdio MCP server. It owns one Dispatcher and
// one Session for the lifetime of the stdin/stdout connection.
type Server struct {
	cfg        *config.Config
	client     *api.Client
	reader     *bufio.Reader
	writer     io.Writer
	writeMu    sync.Mutex
	dispatcher *Dispatcher
	session    *Session
	registry   *tools.Registry
}

// New creates a new MCP server bound to the given streams.
func New(cfg *config.Config, r io.Reader, w io.Writer) (*Server, error) {
	d, err := NewDispatcher(cfg)
	if err != nil {
		return nil, fmt.Errorf("failed to create dispatcher: %w", err)
	}
	return &Server{
		cfg:        cfg,
		client:     d.client,
		reader:     bufio.NewReader(r),
		writer:     w,
		dispatcher: d,
		session:    NewSession(),
		registry:   d.registry,
	}, nil
}

// Run starts the MCP server and processes requests on stdin until EOF.
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

// handleRequest dispatches a request and writes the response (if any)
// to the output stream. It is a thin wrapper that adapts the pure
// Dispatcher API to the stdio transport.
func (s *Server) handleRequest(req *mcp.Request) {
	resp := s.dispatcher.Dispatch(context.Background(), req, s.session)
	if resp == nil {
		return
	}
	s.send(*resp)
}

// initialized reports whether the underlying session has completed the
// initialize handshake. Retained for the test surface.
func (s *Server) initialized() bool { return s.session.IsInitialized() }

// sendResult sends a successful response. Used by tests.
func (s *Server) sendResult(id json.RawMessage, result any) {
	s.send(*resultResp(id, result))
}

// sendError sends an error response. Used by tests.
func (s *Server) sendError(id json.RawMessage, code int, message, data string) {
	s.send(*errorResp(id, code, message, data))
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
