// Package server provides the MCP server implementation for App Store Connect.
package server

import (
	"context"
	"crypto/rand"
	"crypto/sha256"
	"crypto/subtle"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log"
	"net/http"
	"strings"
	"sync"
	"time"

	"github.com/antisynthesis/asc-mcp/internal/asc/config"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

const (
	// SessionHeader is the HTTP header that carries the session id
	// assigned by the server on initialize and required on every
	// subsequent request.
	SessionHeader = "Mcp-Session-Id"

	// ProtocolVersionHeader is the HTTP header that carries the
	// negotiated MCP protocol version on every post-initialize request.
	ProtocolVersionHeader = "MCP-Protocol-Version"

	// MaxHTTPBodySize bounds the JSON-RPC request body the server is
	// willing to read. Real MCP messages are tiny; this protects the
	// process from a misbehaving client.
	MaxHTTPBodySize = 4 * 1024 * 1024 // 4 MiB

	// sessionIdleTimeout is how long an HTTP session may remain
	// untouched before the server discards it.
	sessionIdleTimeout = 30 * time.Minute
)

// HTTPServer is a Streamable HTTP transport for the MCP dispatcher,
// matching the 2025-06-18 specification. One Dispatcher backs many
// concurrent sessions; the session id is generated on initialize and
// echoed in the Mcp-Session-Id response header.
type HTTPServer struct {
	dispatcher *Dispatcher

	mu       sync.RWMutex
	sessions map[string]*Session

	// allowedOrigins is the optional whitelist used to reject
	// cross-origin requests; if empty, all origins are accepted (the
	// caller is expected to use a fronting reverse proxy that enforces
	// its own policy in that case).
	allowedOrigins map[string]struct{}

	// authTokens, when non-empty, makes every /mcp request require a
	// Bearer token from this set. Tokens are stored as SHA-256 digests
	// so log scrapes and core dumps can't leak the raw value.
	authTokens map[[32]byte]struct{}
}

// HTTPOptions configures the HTTP transport. Both fields are optional;
// the zero value disables the corresponding check.
type HTTPOptions struct {
	// AllowedOrigins restricts the Origin header (DNS rebinding defense).
	AllowedOrigins []string
	// AuthTokens, when non-empty, requires `Authorization: Bearer <token>`
	// on every /mcp request and rejects anything else with 401. Pass
	// multiple to support graceful rotation.
	AuthTokens []string
}

// NewHTTPServer constructs an HTTP transport for the MCP dispatcher.
func NewHTTPServer(cfg *config.Config, opts HTTPOptions) (*HTTPServer, error) {
	d, err := NewDispatcher(cfg)
	if err != nil {
		return nil, err
	}
	allowed := make(map[string]struct{}, len(opts.AllowedOrigins))
	for _, o := range opts.AllowedOrigins {
		allowed[strings.ToLower(o)] = struct{}{}
	}
	tokens := make(map[[32]byte]struct{}, len(opts.AuthTokens))
	for _, t := range opts.AuthTokens {
		t = strings.TrimSpace(t)
		if t == "" {
			continue
		}
		tokens[sha256.Sum256([]byte(t))] = struct{}{}
	}
	h := &HTTPServer{
		dispatcher:     d,
		sessions:       make(map[string]*Session),
		allowedOrigins: allowed,
		authTokens:     tokens,
	}
	go h.reapLoop()
	return h, nil
}

// Handler returns the net/http handler for the MCP endpoint. Mount it
// at /mcp (or wherever the deployment prefers).
func (h *HTTPServer) Handler() http.Handler {
	mux := http.NewServeMux()
	mux.Handle("/mcp", h)
	mux.HandleFunc("/healthz", func(w http.ResponseWriter, r *http.Request) {
		// Readiness probe: the dispatcher is alive if it constructed
		// successfully, which it has if we made it here.
		w.WriteHeader(http.StatusOK)
		_, _ = io.WriteString(w, "ok")
	})
	return mux
}

// ServeHTTP implements the Streamable HTTP MCP transport on a single
// endpoint. POST submits a JSON-RPC message and gets back either a
// single JSON response or 202 Accepted (for notifications). DELETE
// terminates a session. GET is reserved for the server-initiated SSE
// stream which this server does not currently produce, so it returns
// 405.
func (h *HTTPServer) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if !h.checkOrigin(r) {
		http.Error(w, "origin not allowed", http.StatusForbidden)
		return
	}
	if !h.checkAuth(r) {
		w.Header().Set("WWW-Authenticate", `Bearer realm="asc-mcp"`)
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	switch r.Method {
	case http.MethodPost:
		h.handlePost(w, r)
	case http.MethodDelete:
		h.handleDelete(w, r)
	case http.MethodGet:
		// The server does not push any server-initiated messages, so
		// the optional GET-for-SSE channel is not implemented.
		w.Header().Set("Allow", "POST, DELETE")
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	case http.MethodOptions:
		w.Header().Set("Allow", "POST, DELETE, OPTIONS")
		w.WriteHeader(http.StatusNoContent)
	default:
		w.Header().Set("Allow", "POST, DELETE, OPTIONS")
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}

func (h *HTTPServer) handlePost(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(io.LimitReader(r.Body, MaxHTTPBodySize+1))
	if err != nil {
		http.Error(w, "failed to read request body", http.StatusBadRequest)
		return
	}
	if len(body) > MaxHTTPBodySize {
		http.Error(w, "request body too large", http.StatusRequestEntityTooLarge)
		return
	}

	var req mcp.Request
	if err := json.Unmarshal(body, &req); err != nil {
		h.writeJSON(w, http.StatusBadRequest, errorResp(nil, mcp.ErrCodeParse, "Parse error", err.Error()))
		return
	}

	// Identify the session.
	sessID := r.Header.Get(SessionHeader)
	isInit := req.Method == "initialize"

	var sess *Session
	if isInit {
		// initialize creates (or recreates) the session.
		sessID = newSessionID()
		sess = NewSession()
		h.mu.Lock()
		h.sessions[sessID] = sess
		h.mu.Unlock()
	} else {
		if sessID == "" {
			http.Error(w, "missing "+SessionHeader+" header", http.StatusBadRequest)
			return
		}
		h.mu.RLock()
		sess = h.sessions[sessID]
		h.mu.RUnlock()
		if sess == nil {
			http.Error(w, "unknown session", http.StatusNotFound)
			return
		}
		// Validate the protocol version header per spec.
		if pv := r.Header.Get(ProtocolVersionHeader); pv != "" {
			if !knownProtocolVersion(pv) {
				http.Error(w, "unsupported "+ProtocolVersionHeader, http.StatusBadRequest)
				return
			}
		}
	}

	resp := h.dispatcher.Dispatch(r.Context(), &req, sess)

	// Notifications (and anything else that produced no response) get
	// 202 Accepted with no body, per the spec.
	if resp == nil {
		if isInit {
			// initialize must produce a response; reaching here means
			// the request was malformed enough that even initialize
			// returned nil (e.g. notification-style with no id). Drop
			// the freshly-created session and accept silently.
			h.mu.Lock()
			delete(h.sessions, sessID)
			h.mu.Unlock()
		}
		w.WriteHeader(http.StatusAccepted)
		return
	}

	if isInit {
		w.Header().Set(SessionHeader, sessID)
	}
	w.Header().Set(ProtocolVersionHeader, mcp.ProtocolVersion)
	h.writeJSON(w, http.StatusOK, resp)
}

func (h *HTTPServer) handleDelete(w http.ResponseWriter, r *http.Request) {
	sessID := r.Header.Get(SessionHeader)
	if sessID == "" {
		http.Error(w, "missing "+SessionHeader+" header", http.StatusBadRequest)
		return
	}
	h.mu.Lock()
	_, existed := h.sessions[sessID]
	delete(h.sessions, sessID)
	h.mu.Unlock()
	if !existed {
		http.Error(w, "unknown session", http.StatusNotFound)
		return
	}
	w.WriteHeader(http.StatusNoContent)
}

func (h *HTTPServer) writeJSON(w http.ResponseWriter, status int, resp *mcp.Response) {
	data, err := json.Marshal(resp)
	if err != nil {
		log.Printf("http: failed to marshal response: %v", err)
		http.Error(w, "internal error", http.StatusInternalServerError)
		return
	}
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	if _, err := w.Write(data); err != nil {
		log.Printf("http: failed to write response: %v", err)
	}
}

// checkOrigin returns true when the request is permitted to proceed.
// When no origins are configured, all requests pass; when origins are
// configured, the Origin header must match one of them.
func (h *HTTPServer) checkOrigin(r *http.Request) bool {
	if len(h.allowedOrigins) == 0 {
		return true
	}
	origin := strings.ToLower(r.Header.Get("Origin"))
	if origin == "" {
		// Same-origin requests from a browser will have an Origin; CLI
		// callers usually do not. Allow when missing so curl etc. work.
		return true
	}
	_, ok := h.allowedOrigins[origin]
	return ok
}

// checkAuth returns true when the request carries a valid Bearer token
// (or when no tokens are configured, in which case the endpoint is
// open). Tokens are compared in constant time against stored SHA-256
// digests so timing side-channels and core-dump scrapes cannot leak the
// raw secret.
func (h *HTTPServer) checkAuth(r *http.Request) bool {
	if len(h.authTokens) == 0 {
		return true
	}
	header := r.Header.Get("Authorization")
	const prefix = "Bearer "
	if !strings.HasPrefix(header, prefix) {
		return false
	}
	presented := sha256.Sum256([]byte(strings.TrimSpace(header[len(prefix):])))
	// Iterate the configured set in constant time per entry. The map
	// has at most a handful of rotation tokens, so this is fine; we
	// still use subtle.ConstantTimeCompare to avoid leaking which token
	// matched (or how close a near-miss got).
	match := 0
	for digest := range h.authTokens {
		d := digest
		match |= subtle.ConstantTimeCompare(presented[:], d[:])
	}
	return match == 1
}

// reapLoop periodically discards sessions that have been idle for
// longer than sessionIdleTimeout.
func (h *HTTPServer) reapLoop() {
	t := time.NewTicker(5 * time.Minute)
	defer t.Stop()
	for range t.C {
		cutoff := time.Now().Add(-sessionIdleTimeout)
		h.mu.Lock()
		for id, sess := range h.sessions {
			if sess.LastActive().Before(cutoff) {
				delete(h.sessions, id)
			}
		}
		h.mu.Unlock()
	}
}

// newSessionID returns a cryptographically random hex session id.
func newSessionID() string {
	var buf [16]byte
	if _, err := rand.Read(buf[:]); err != nil {
		// crypto/rand should not fail; fall back to time-based id so
		// the server keeps serving.
		return fmt.Sprintf("sess-%d", time.Now().UnixNano())
	}
	return hex.EncodeToString(buf[:])
}

// knownProtocolVersion reports whether the given version string is one
// of the protocol versions this server understands.
func knownProtocolVersion(v string) bool {
	for _, p := range mcp.SupportedProtocolVersions {
		if p == v {
			return true
		}
	}
	return false
}

// ListenAndServe binds the HTTP transport to addr and serves until
// ctx is cancelled. It is a convenience for the CLI entry point;
// callers that need finer control can use Handler() directly.
func (h *HTTPServer) ListenAndServe(ctx context.Context, addr string) error {
	srv := &http.Server{
		Addr:              addr,
		Handler:           h.Handler(),
		ReadHeaderTimeout: 10 * time.Second,
		ReadTimeout:       60 * time.Second,
		WriteTimeout:      120 * time.Second,
		IdleTimeout:       2 * time.Minute,
	}
	errCh := make(chan error, 1)
	go func() { errCh <- srv.ListenAndServe() }()

	select {
	case <-ctx.Done():
		shutdownCtx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
		defer cancel()
		return srv.Shutdown(shutdownCtx)
	case err := <-errCh:
		if errors.Is(err, http.ErrServerClosed) {
			return nil
		}
		return err
	}
}
