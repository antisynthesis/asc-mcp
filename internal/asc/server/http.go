// Package server provides the MCP server implementation for App Store Connect.
package server

import (
	"context"
	"crypto/rand"
	"crypto/sha256"
	"crypto/subtle"
	"crypto/tls"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log"
	"log/slog"
	"net/http"
	"os"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/prometheus/client_golang/prometheus/promhttp"

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

	logger  *slog.Logger
	metrics *metrics
}

// HTTPOptions configures the HTTP transport. All fields are optional;
// the zero value disables the corresponding feature.
type HTTPOptions struct {
	// AllowedOrigins restricts the Origin header (DNS rebinding defense).
	AllowedOrigins []string
	// AuthTokens, when non-empty, requires `Authorization: Bearer <token>`
	// on every /mcp request and rejects anything else with 401. Pass
	// multiple to support graceful rotation.
	AuthTokens []string
	// Logger overrides the slog logger used for per-request structured
	// logs. When nil the server uses slog.Default().
	Logger *slog.Logger
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
	logger := opts.Logger
	if logger == nil {
		logger = slog.Default()
	}
	m := newMetrics()
	d.metrics = m
	d.SetLogger(logger)
	h := &HTTPServer{
		dispatcher:     d,
		sessions:       make(map[string]*Session),
		allowedOrigins: allowed,
		authTokens:     tokens,
		logger:         logger,
		metrics:        m,
	}
	go h.reapLoop()
	return h, nil
}

// Metrics returns the Prometheus registry the HTTPServer uses. Useful
// for tests that want to read counter values without scraping /metrics.
func (h *HTTPServer) Metrics() *metrics { return h.metrics }

// Handler returns the net/http handler for the MCP endpoint. Mount it
// at /mcp (or wherever the deployment prefers).
func (h *HTTPServer) Handler() http.Handler {
	mux := http.NewServeMux()
	mux.Handle("/mcp", h.observe(h))
	mux.HandleFunc("/healthz", func(w http.ResponseWriter, r *http.Request) {
		// Readiness probe: the dispatcher is alive if it constructed
		// successfully, which it has if we made it here.
		w.WriteHeader(http.StatusOK)
		_, _ = io.WriteString(w, "ok")
	})
	mux.Handle("/metrics", promhttp.HandlerFor(h.metrics.registry, promhttp.HandlerOpts{
		Registry: h.metrics.registry,
	}))
	return mux
}

// observe wraps the /mcp handler with structured logging and request
// metrics. The wrapper assigns a request id, captures the response
// status, and records the duration both as a Prometheus histogram and
// in the structured log line.
func (h *HTTPServer) observe(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		reqID := newRequestID()
		rec := &statusRecorder{ResponseWriter: w, status: http.StatusOK}
		// Stash the request id so handlers can include it in their own
		// log lines if they want.
		ctx := context.WithValue(r.Context(), requestIDKey{}, reqID)
		next.ServeHTTP(rec, r.WithContext(ctx))
		elapsed := time.Since(start)
		status := rec.status
		h.metrics.httpRequests.WithLabelValues(r.Method, strconv.Itoa(status)).Inc()
		h.metrics.httpDuration.WithLabelValues(r.Method).Observe(elapsed.Seconds())
		h.logger.LogAttrs(r.Context(), slog.LevelInfo, "http_request",
			slog.String("request_id", reqID),
			slog.String("method", r.Method),
			slog.String("path", r.URL.Path),
			slog.String("session_id", r.Header.Get(SessionHeader)),
			slog.Int("status", status),
			slog.Duration("duration", elapsed),
			slog.String("remote", r.RemoteAddr),
		)
	})
}

// requestIDKey is the context key for the per-request id assigned by
// the observe middleware.
type requestIDKey struct{}

// RequestIDFromContext returns the request id installed by the HTTP
// middleware, or "" when called outside an HTTP-handled request.
func RequestIDFromContext(ctx context.Context) string {
	v, _ := ctx.Value(requestIDKey{}).(string)
	return v
}

// statusRecorder captures the HTTP status code so the middleware can
// log and meter it after the handler returns.
type statusRecorder struct {
	http.ResponseWriter
	status      int
	wroteHeader bool
}

func (s *statusRecorder) WriteHeader(code int) {
	if !s.wroteHeader {
		s.status = code
		s.wroteHeader = true
	}
	s.ResponseWriter.WriteHeader(code)
}

// newRequestID returns an 8-byte hex id, short enough to fit comfortably
// in log lines without taking the spotlight from the message.
func newRequestID() string {
	var buf [8]byte
	if _, err := rand.Read(buf[:]); err != nil {
		return fmt.Sprintf("req-%d", time.Now().UnixNano())
	}
	return hex.EncodeToString(buf[:])
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
		h.metrics.authFailures.Inc()
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
		active := len(h.sessions)
		h.mu.Unlock()
		h.metrics.sessionsCreated.Inc()
		h.metrics.sessionsActive.Set(float64(active))
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
	active := len(h.sessions)
	h.mu.Unlock()
	if !existed {
		http.Error(w, "unknown session", http.StatusNotFound)
		return
	}
	h.metrics.sessionsActive.Set(float64(active))
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
		reaped := 0
		h.mu.Lock()
		for id, sess := range h.sessions {
			if sess.LastActive().Before(cutoff) {
				delete(h.sessions, id)
				reaped++
			}
		}
		active := len(h.sessions)
		h.mu.Unlock()
		if reaped > 0 {
			h.metrics.sessionsReaped.Add(float64(reaped))
			h.metrics.sessionsActive.Set(float64(active))
		}
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
//
// When tlsCertFile and tlsKeyFile are both non-empty, the server serves
// HTTPS using TLS 1.2 or newer. When neither is set, plain HTTP is
// used (suitable for deployments behind a TLS-terminating reverse
// proxy). Supplying only one of the pair is rejected as a config
// error to avoid silently downgrading.
func (h *HTTPServer) ListenAndServe(ctx context.Context, addr, tlsCertFile, tlsKeyFile string) error {
	useTLS, err := validateTLSPair(tlsCertFile, tlsKeyFile)
	if err != nil {
		return err
	}
	srv := &http.Server{
		Addr:              addr,
		Handler:           h.Handler(),
		ReadHeaderTimeout: 10 * time.Second,
		ReadTimeout:       60 * time.Second,
		WriteTimeout:      120 * time.Second,
		IdleTimeout:       2 * time.Minute,
		TLSConfig: &tls.Config{
			MinVersion: tls.VersionTLS12,
		},
	}
	errCh := make(chan error, 1)
	go func() {
		if useTLS {
			errCh <- srv.ListenAndServeTLS(tlsCertFile, tlsKeyFile)
		} else {
			errCh <- srv.ListenAndServe()
		}
	}()

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

// validateTLSPair returns true when both cert and key are non-empty
// (and readable). It returns an error when only one of the pair is set
// to prevent silent downgrades to plain HTTP.
func validateTLSPair(cert, key string) (bool, error) {
	switch {
	case cert == "" && key == "":
		return false, nil
	case cert == "" || key == "":
		return false, fmt.Errorf("tls config invalid: both --tls-cert and --tls-key must be set, or neither")
	}
	if _, err := os.Stat(cert); err != nil {
		return false, fmt.Errorf("tls cert: %w", err)
	}
	if _, err := os.Stat(key); err != nil {
		return false, fmt.Errorf("tls key: %w", err)
	}
	return true, nil
}
