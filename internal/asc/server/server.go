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

	// maxConcurrentRequests caps how many stdio requests may execute at
	// once. Requests must run concurrently so a notifications/cancelled
	// line can be read while the request it targets is still in flight,
	// but a misbehaving client must not be able to spawn unbounded
	// goroutines.
	maxConcurrentRequests = 16
)

// errCancelledByClient is recorded as the context cancellation cause when
// the client aborts an in-flight request via notifications/cancelled. Its
// presence tells the stdio response path to suppress output entirely: on
// 2026-07-28 (and, SHOULD-level, under the older cancellation utility) the
// server MUST NOT send further messages — including the response — for a
// cancelled request.
var errCancelledByClient = errors.New("request cancelled by notifications/cancelled")

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

	// inflight tracks the cancel function for every request currently
	// being dispatched, keyed by the raw JSON token of its JSON-RPC id
	// (so the string id "5" and the number id 5 never collide). It lets
	// a notifications/cancelled received on the read loop abort a
	// request running on another goroutine.
	inflightMu sync.Mutex
	inflight   map[string]context.CancelCauseFunc
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
		inflight:   make(map[string]context.CancelCauseFunc),
	}, nil
}

// Run starts the MCP server and processes requests on stdin until EOF.
//
// Requests (messages with an id) are dispatched concurrently so that the
// read loop stays free to receive a notifications/cancelled targeting a
// request still in flight — on stdio that notification is the client's
// only cancellation mechanism. Notifications are handled inline on the
// read loop. The write path is serialized by writeMu, so concurrent
// responses are safe. Run waits for all in-flight requests to finish
// before returning.
func (s *Server) Run() error {
	log.Printf("MCP server %s v%s starting (protocol %s)", serverName, serverVersion, mcp.ProtocolVersion)

	sem := make(chan struct{}, maxConcurrentRequests)
	var wg sync.WaitGroup
	defer wg.Wait()

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

		// notifications/cancelled must act while its target is still in
		// flight, so it is intercepted here on the read loop rather than
		// routed through Dispatch. Other notifications are also handled
		// inline to preserve their ordering relative to later requests.
		if req.Method == "notifications/cancelled" {
			s.handleCancelled(&req)
			continue
		}
		if req.IsNotification() {
			s.handleRequest(&req)
			continue
		}

		sem <- struct{}{}
		wg.Add(1)
		go func(r *mcp.Request) {
			defer wg.Done()
			defer func() { <-sem }()
			s.handleRequest(r)
		}(&req)
	}
}

// handleRequest dispatches a request and writes the response (if any)
// to the output stream. It is a thin wrapper that adapts the pure
// Dispatcher API to the stdio transport. Requests are registered in the
// in-flight table for their duration so notifications/cancelled can
// abort them; a request cancelled that way gets no response at all.
func (s *Server) handleRequest(req *mcp.Request) {
	ctx := context.Background()
	if !req.IsNotification() {
		cctx, cancel := context.WithCancelCause(ctx)
		defer cancel(nil)
		key := string(req.ID)
		s.trackInflight(key, cancel)
		defer s.untrackInflight(key)
		ctx = cctx
	}

	resp := s.dispatcher.Dispatch(ctx, req, s.session)
	if resp == nil {
		return
	}
	// Once a request has been cancelled via notifications/cancelled the
	// server MUST NOT send further messages for it, including its
	// response. A cancellation racing with completion may still slip
	// past this check; the spec permits that.
	if errors.Is(context.Cause(ctx), errCancelledByClient) {
		return
	}
	s.send(*resp)
}

// handleCancelled processes a notifications/cancelled received on stdio:
// it looks up the in-flight request named by params.requestId and cancels
// its context. Per the spec, malformed or unknown cancellations (already
// completed, never seen, or racing with completion) are ignored — no
// error, no response.
func (s *Server) handleCancelled(req *mcp.Request) {
	var params struct {
		RequestID json.RawMessage `json:"requestId"`
		Reason    string          `json:"reason,omitempty"`
	}
	if len(req.Params) == 0 || json.Unmarshal(req.Params, &params) != nil || len(params.RequestID) == 0 {
		return
	}
	// The raw JSON token is the canonical key: the string id "5" and the
	// number id 5 are distinct requests and must not match each other.
	s.inflightMu.Lock()
	cancel, ok := s.inflight[string(params.RequestID)]
	s.inflightMu.Unlock()
	if !ok {
		return
	}
	if params.Reason != "" {
		log.Printf("request %s cancelled by client: %s", params.RequestID, params.Reason)
	}
	cancel(errCancelledByClient)
}

// trackInflight registers the cancel function for a request about to be
// dispatched.
func (s *Server) trackInflight(key string, cancel context.CancelCauseFunc) {
	s.inflightMu.Lock()
	defer s.inflightMu.Unlock()
	s.inflight[key] = cancel
}

// untrackInflight removes a completed request from the in-flight table.
func (s *Server) untrackInflight(key string) {
	s.inflightMu.Lock()
	defer s.inflightMu.Unlock()
	delete(s.inflight, key)
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
