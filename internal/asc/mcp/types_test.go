package mcp

import (
	"encoding/json"
	"testing"
)

func TestConstants(t *testing.T) {
	if JSONRPCVersion != "2.0" {
		t.Errorf("JSONRPCVersion = %q, want 2.0", JSONRPCVersion)
	}

	if LatestProtocolVersion != "2026-07-28" {
		t.Errorf("LatestProtocolVersion = %q, want 2026-07-28", LatestProtocolVersion)
	}

	if ProtocolVersion != "2025-11-25" {
		t.Errorf("ProtocolVersion = %q, want 2025-11-25", ProtocolVersion)
	}

	if len(SupportedProtocolVersions) == 0 {
		t.Fatal("SupportedProtocolVersions must not be empty")
	}
	if SupportedProtocolVersions[0] != LatestProtocolVersion {
		t.Errorf("SupportedProtocolVersions[0] = %q, want %q", SupportedProtocolVersions[0], LatestProtocolVersion)
	}

	want := []string{"2026-07-28", "2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05"}
	if len(SupportedProtocolVersions) != len(want) {
		t.Fatalf("SupportedProtocolVersions has %d entries, want %d", len(SupportedProtocolVersions), len(want))
	}
	for i, v := range want {
		if SupportedProtocolVersions[i] != v {
			t.Errorf("SupportedProtocolVersions[%d] = %q, want %q", i, SupportedProtocolVersions[i], v)
		}
	}

	wantLegacy := []string{"2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05"}
	if len(LegacyHandshakeVersions) != len(wantLegacy) {
		t.Fatalf("LegacyHandshakeVersions has %d entries, want %d", len(LegacyHandshakeVersions), len(wantLegacy))
	}
	for i, v := range wantLegacy {
		if LegacyHandshakeVersions[i] != v {
			t.Errorf("LegacyHandshakeVersions[%d] = %q, want %q", i, LegacyHandshakeVersions[i], v)
		}
	}
	if LegacyHandshakeVersions[0] != ProtocolVersion {
		t.Errorf("LegacyHandshakeVersions[0] = %q, want %q", LegacyHandshakeVersions[0], ProtocolVersion)
	}
}

func TestNegotiateProtocolVersion(t *testing.T) {
	tests := []struct {
		name      string
		requested string
		want      string
	}{
		{"latest legacy", "2025-11-25", "2025-11-25"},
		{"prior legacy", "2025-06-18", "2025-06-18"},
		{"older supported", "2024-11-05", "2024-11-05"},
		{"interim supported", "2025-03-26", "2025-03-26"},
		// 2026-07-28 has no initialize handshake, so it must never be
		// negotiated here: the client gets the legacy fallback, not an echo.
		{"modern falls back to legacy", "2026-07-28", ProtocolVersion},
		{"unknown falls back to latest legacy", "1999-01-01", ProtocolVersion},
		{"empty falls back to latest legacy", "", ProtocolVersion},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := NegotiateProtocolVersion(tt.requested)
			if got != tt.want {
				t.Errorf("NegotiateProtocolVersion(%q) = %q, want %q", tt.requested, got, tt.want)
			}
		})
	}
}

func TestRequest_IsNotification(t *testing.T) {
	t.Run("with id", func(t *testing.T) {
		r := Request{ID: json.RawMessage(`1`)}
		if r.IsNotification() {
			t.Error("request with id should not be a notification")
		}
	})
	t.Run("without id", func(t *testing.T) {
		r := Request{}
		if !r.IsNotification() {
			t.Error("request without id should be a notification")
		}
	})
}

func TestErrorCodes(t *testing.T) {
	tests := []struct {
		name string
		code int
		want int
	}{
		{"ErrCodeParse", ErrCodeParse, -32700},
		{"ErrCodeInvalidRequest", ErrCodeInvalidRequest, -32600},
		{"ErrCodeMethodNotFound", ErrCodeMethodNotFound, -32601},
		{"ErrCodeInvalidParams", ErrCodeInvalidParams, -32602},
		{"ErrCodeInternal", ErrCodeInternal, -32603},
		{"ErrCodeHeaderMismatch", ErrCodeHeaderMismatch, -32020},
		{"ErrCodeMissingClientCapability", ErrCodeMissingClientCapability, -32021},
		{"ErrCodeUnsupportedProtocolVersion", ErrCodeUnsupportedProtocolVersion, -32022},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.code != tt.want {
				t.Errorf("%s = %d, want %d", tt.name, tt.code, tt.want)
			}
		})
	}
}

func TestRequest_JSON(t *testing.T) {
	tests := []struct {
		name    string
		jsonStr string
		want    Request
	}{
		{
			name:    "initialize request",
			jsonStr: `{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05"}}`,
			want: Request{
				JSONRPC: "2.0",
				Method:  "initialize",
			},
		},
		{
			name:    "tools/list request",
			jsonStr: `{"jsonrpc":"2.0","id":"abc","method":"tools/list"}`,
			want: Request{
				JSONRPC: "2.0",
				Method:  "tools/list",
			},
		},
		{
			name:    "notification (no id)",
			jsonStr: `{"jsonrpc":"2.0","method":"notifications/initialized"}`,
			want: Request{
				JSONRPC: "2.0",
				Method:  "notifications/initialized",
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var req Request
			if err := json.Unmarshal([]byte(tt.jsonStr), &req); err != nil {
				t.Fatalf("failed to unmarshal: %v", err)
			}

			if req.JSONRPC != tt.want.JSONRPC {
				t.Errorf("JSONRPC = %q, want %q", req.JSONRPC, tt.want.JSONRPC)
			}

			if req.Method != tt.want.Method {
				t.Errorf("Method = %q, want %q", req.Method, tt.want.Method)
			}
		})
	}
}

func TestResponse_JSON(t *testing.T) {
	t.Run("success response", func(t *testing.T) {
		resp := Response{
			JSONRPC: JSONRPCVersion,
			ID:      json.RawMessage(`1`),
			Result:  map[string]string{"status": "ok"},
		}

		data, err := json.Marshal(resp)
		if err != nil {
			t.Fatalf("failed to marshal: %v", err)
		}

		// Unmarshal and verify
		var decoded Response
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatalf("failed to unmarshal: %v", err)
		}

		if decoded.JSONRPC != JSONRPCVersion {
			t.Errorf("JSONRPC = %q, want %q", decoded.JSONRPC, JSONRPCVersion)
		}

		if decoded.Error != nil {
			t.Error("expected no error")
		}
	})

	t.Run("error response", func(t *testing.T) {
		resp := Response{
			JSONRPC: JSONRPCVersion,
			ID:      json.RawMessage(`1`),
			Error: &RPCError{
				Code:    ErrCodeMethodNotFound,
				Message: "Method not found",
				Data:    "unknown_method",
			},
		}

		data, err := json.Marshal(resp)
		if err != nil {
			t.Fatalf("failed to marshal: %v", err)
		}

		var decoded Response
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatalf("failed to unmarshal: %v", err)
		}

		if decoded.Error == nil {
			t.Fatal("expected error")
		}

		if decoded.Error.Code != ErrCodeMethodNotFound {
			t.Errorf("Error.Code = %d, want %d", decoded.Error.Code, ErrCodeMethodNotFound)
		}

		if decoded.Error.Message != "Method not found" {
			t.Errorf("Error.Message = %q, want Method not found", decoded.Error.Message)
		}
	})
}

func TestInitializeParams_JSON(t *testing.T) {
	jsonStr := `{
		"protocolVersion": "2024-11-05",
		"capabilities": {
			"roots": {"listChanged": true}
		},
		"clientInfo": {
			"name": "test-client",
			"version": "1.0.0"
		}
	}`

	var params InitializeParams
	if err := json.Unmarshal([]byte(jsonStr), &params); err != nil {
		t.Fatalf("failed to unmarshal: %v", err)
	}

	if params.ProtocolVersion != "2024-11-05" {
		t.Errorf("ProtocolVersion = %q, want 2024-11-05", params.ProtocolVersion)
	}

	if params.ClientInfo.Name != "test-client" {
		t.Errorf("ClientInfo.Name = %q, want test-client", params.ClientInfo.Name)
	}

	if params.ClientInfo.Version != "1.0.0" {
		t.Errorf("ClientInfo.Version = %q, want 1.0.0", params.ClientInfo.Version)
	}

	if params.Capabilities.Roots == nil {
		t.Fatal("expected Roots capability")
	}

	if !params.Capabilities.Roots.ListChanged {
		t.Error("Roots.ListChanged should be true")
	}
}

func TestInitializeResult_JSON(t *testing.T) {
	result := InitializeResult{
		ProtocolVersion: ProtocolVersion,
		Capabilities: ServerCapability{
			Tools: &ToolsCapability{
				ListChanged: false,
			},
		},
		ServerInfo: ServerInfo{
			Name:    "test-server",
			Version: "1.0.0",
		},
	}

	data, err := json.Marshal(result)
	if err != nil {
		t.Fatalf("failed to marshal: %v", err)
	}

	var decoded InitializeResult
	if err := json.Unmarshal(data, &decoded); err != nil {
		t.Fatalf("failed to unmarshal: %v", err)
	}

	if decoded.ProtocolVersion != ProtocolVersion {
		t.Errorf("ProtocolVersion = %q, want %q", decoded.ProtocolVersion, ProtocolVersion)
	}

	if decoded.ServerInfo.Name != "test-server" {
		t.Errorf("ServerInfo.Name = %q, want test-server", decoded.ServerInfo.Name)
	}

	if decoded.Capabilities.Tools == nil {
		t.Error("expected Tools capability")
	}
}

func TestTool_JSON(t *testing.T) {
	tool := Tool{
		Name:        "test_tool",
		Description: "A test tool",
		InputSchema: JSONSchema{
			Type: "object",
			Properties: map[string]Property{
				"param1": {
					Type:        "string",
					Description: "First parameter",
				},
				"param2": {
					Type:    "integer",
					Default: 10,
				},
			},
			Required: []string{"param1"},
		},
	}

	data, err := json.Marshal(tool)
	if err != nil {
		t.Fatalf("failed to marshal: %v", err)
	}

	var decoded Tool
	if err := json.Unmarshal(data, &decoded); err != nil {
		t.Fatalf("failed to unmarshal: %v", err)
	}

	if decoded.Name != "test_tool" {
		t.Errorf("Name = %q, want test_tool", decoded.Name)
	}

	if decoded.InputSchema.Type != "object" {
		t.Errorf("InputSchema.Type = %q, want object", decoded.InputSchema.Type)
	}

	if len(decoded.InputSchema.Properties) != 2 {
		t.Errorf("InputSchema.Properties count = %d, want 2", len(decoded.InputSchema.Properties))
	}

	if len(decoded.InputSchema.Required) != 1 || decoded.InputSchema.Required[0] != "param1" {
		t.Error("InputSchema.Required should be [param1]")
	}
}

func TestToolsCallParams_JSON(t *testing.T) {
	jsonStr := `{
		"name": "list_apps",
		"arguments": {"limit": 50}
	}`

	var params ToolsCallParams
	if err := json.Unmarshal([]byte(jsonStr), &params); err != nil {
		t.Fatalf("failed to unmarshal: %v", err)
	}

	if params.Name != "list_apps" {
		t.Errorf("Name = %q, want list_apps", params.Name)
	}

	if params.Arguments == nil {
		t.Error("expected Arguments")
	}
}

func TestToolsCallResult_JSON(t *testing.T) {
	t.Run("success result", func(t *testing.T) {
		result := ToolsCallResult{
			Content: []ContentBlock{
				{Type: "text", Text: "Success message"},
			},
			IsError: false,
		}

		data, err := json.Marshal(result)
		if err != nil {
			t.Fatalf("failed to marshal: %v", err)
		}

		var decoded ToolsCallResult
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatalf("failed to unmarshal: %v", err)
		}

		if decoded.IsError {
			t.Error("IsError should be false")
		}

		if len(decoded.Content) != 1 {
			t.Fatalf("expected 1 content block, got %d", len(decoded.Content))
		}

		if decoded.Content[0].Text != "Success message" {
			t.Errorf("Content[0].Text = %q, want Success message", decoded.Content[0].Text)
		}
	})

	t.Run("error result", func(t *testing.T) {
		result := ToolsCallResult{
			Content: []ContentBlock{
				{Type: "text", Text: "Error message"},
			},
			IsError: true,
		}

		data, err := json.Marshal(result)
		if err != nil {
			t.Fatalf("failed to marshal: %v", err)
		}

		var decoded ToolsCallResult
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatalf("failed to unmarshal: %v", err)
		}

		if !decoded.IsError {
			t.Error("IsError should be true")
		}
	})
}

func TestNewTextContent(t *testing.T) {
	content := NewTextContent("Hello, World!")

	if content.Type != "text" {
		t.Errorf("Type = %q, want text", content.Type)
	}

	if content.Text != "Hello, World!" {
		t.Errorf("Text = %q, want Hello, World!", content.Text)
	}
}

func TestNewSuccessResult(t *testing.T) {
	result := NewSuccessResult("Operation completed")

	if result.IsError {
		t.Error("IsError should be false")
	}

	if len(result.Content) != 1 {
		t.Fatalf("expected 1 content block, got %d", len(result.Content))
	}

	if result.Content[0].Type != "text" {
		t.Errorf("Content[0].Type = %q, want text", result.Content[0].Type)
	}

	if result.Content[0].Text != "Operation completed" {
		t.Errorf("Content[0].Text = %q, want Operation completed", result.Content[0].Text)
	}
}

func TestNewErrorResult(t *testing.T) {
	result := NewErrorResult("Something went wrong")

	if !result.IsError {
		t.Error("IsError should be true")
	}

	if len(result.Content) != 1 {
		t.Fatalf("expected 1 content block, got %d", len(result.Content))
	}

	if result.Content[0].Type != "text" {
		t.Errorf("Content[0].Type = %q, want text", result.Content[0].Type)
	}

	if result.Content[0].Text != "Something went wrong" {
		t.Errorf("Content[0].Text = %q, want Something went wrong", result.Content[0].Text)
	}
}

func TestToolsListResult_JSON(t *testing.T) {
	result := ToolsListResult{
		Tools: []Tool{
			{
				Name:        "tool1",
				Description: "First tool",
				InputSchema: JSONSchema{Type: "object"},
			},
			{
				Name:        "tool2",
				Description: "Second tool",
				InputSchema: JSONSchema{Type: "object"},
			},
		},
	}

	data, err := json.Marshal(result)
	if err != nil {
		t.Fatalf("failed to marshal: %v", err)
	}

	var decoded ToolsListResult
	if err := json.Unmarshal(data, &decoded); err != nil {
		t.Fatalf("failed to unmarshal: %v", err)
	}

	if len(decoded.Tools) != 2 {
		t.Errorf("expected 2 tools, got %d", len(decoded.Tools))
	}
}

func TestParseRequestMeta(t *testing.T) {
	t.Run("full modern _meta", func(t *testing.T) {
		params := json.RawMessage(`{
			"name": "list_apps",
			"_meta": {
				"io.modelcontextprotocol/protocolVersion": "2026-07-28",
				"io.modelcontextprotocol/clientCapabilities": {"elicitation": {}, "extensions": {"io.example/thing": {"x": 1}}},
				"io.modelcontextprotocol/clientInfo": {"name": "test-client", "version": "1.0.0"},
				"progressToken": "tok-1"
			}
		}`)

		meta, err := ParseRequestMeta(params)
		if err != nil {
			t.Fatalf("ParseRequestMeta returned error: %v", err)
		}

		if meta.ProtocolVersion != "2026-07-28" {
			t.Errorf("ProtocolVersion = %q, want 2026-07-28", meta.ProtocolVersion)
		}
		if meta.ClientCapabilities == nil {
			t.Fatal("expected non-nil ClientCapabilities")
		}
		if meta.ClientCapabilities.Elicitation == nil {
			t.Error("expected Elicitation capability")
		}
		if _, ok := meta.ClientCapabilities.Extensions["io.example/thing"]; !ok {
			t.Error("expected extension io.example/thing to be preserved")
		}
		if meta.ClientInfo == nil || meta.ClientInfo.Name != "test-client" {
			t.Errorf("ClientInfo = %+v, want name test-client", meta.ClientInfo)
		}
		if string(meta.ProgressToken) != `"tok-1"` {
			t.Errorf("ProgressToken = %q, want %q", meta.ProgressToken, `"tok-1"`)
		}
	})

	t.Run("empty params", func(t *testing.T) {
		meta, err := ParseRequestMeta(nil)
		if err != nil {
			t.Fatalf("ParseRequestMeta returned error: %v", err)
		}
		if meta == nil {
			t.Fatal("expected zero-value RequestMeta, got nil")
		}
		if meta.ProtocolVersion != "" || meta.ClientCapabilities != nil || meta.ClientInfo != nil {
			t.Errorf("expected zero-value RequestMeta, got %+v", meta)
		}
	})

	t.Run("params without _meta", func(t *testing.T) {
		meta, err := ParseRequestMeta(json.RawMessage(`{"name":"list_apps"}`))
		if err != nil {
			t.Fatalf("ParseRequestMeta returned error: %v", err)
		}
		if meta.ClientCapabilities != nil {
			t.Error("ClientCapabilities should be nil when _meta is absent")
		}
	})

	t.Run("clientCapabilities present as empty object", func(t *testing.T) {
		params := json.RawMessage(`{"_meta": {"io.modelcontextprotocol/clientCapabilities": {}}}`)
		meta, err := ParseRequestMeta(params)
		if err != nil {
			t.Fatalf("ParseRequestMeta returned error: %v", err)
		}
		if meta.ClientCapabilities == nil {
			t.Error("ClientCapabilities should be non-nil when present as {}")
		}
	})

	t.Run("clientCapabilities absent", func(t *testing.T) {
		params := json.RawMessage(`{"_meta": {"io.modelcontextprotocol/protocolVersion": "2026-07-28"}}`)
		meta, err := ParseRequestMeta(params)
		if err != nil {
			t.Fatalf("ParseRequestMeta returned error: %v", err)
		}
		if meta.ClientCapabilities != nil {
			t.Error("ClientCapabilities should be nil when the key is absent")
		}
	})

	t.Run("malformed protocolVersion type", func(t *testing.T) {
		params := json.RawMessage(`{"_meta": {"io.modelcontextprotocol/protocolVersion": 42}}`)
		if _, err := ParseRequestMeta(params); err == nil {
			t.Error("expected error for non-string protocolVersion")
		}
	})

	t.Run("malformed params JSON", func(t *testing.T) {
		if _, err := ParseRequestMeta(json.RawMessage(`{`)); err == nil {
			t.Error("expected error for malformed params")
		}
	})
}

func TestResultTypeConstants(t *testing.T) {
	if ResultTypeComplete != "complete" {
		t.Errorf("ResultTypeComplete = %q, want complete", ResultTypeComplete)
	}
	if ResultTypeInputRequired != "input_required" {
		t.Errorf("ResultTypeInputRequired = %q, want input_required", ResultTypeInputRequired)
	}
}

func TestMetaKeyConstants(t *testing.T) {
	tests := []struct {
		name string
		got  string
		want string
	}{
		{"MetaProtocolVersion", MetaProtocolVersion, "io.modelcontextprotocol/protocolVersion"},
		{"MetaClientCapabilities", MetaClientCapabilities, "io.modelcontextprotocol/clientCapabilities"},
		{"MetaClientInfo", MetaClientInfo, "io.modelcontextprotocol/clientInfo"},
		{"MetaServerInfo", MetaServerInfo, "io.modelcontextprotocol/serverInfo"},
		{"MetaLogLevel", MetaLogLevel, "io.modelcontextprotocol/logLevel"},
		{"MetaSubscriptionID", MetaSubscriptionID, "io.modelcontextprotocol/subscriptionId"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.got != tt.want {
				t.Errorf("%s = %q, want %q", tt.name, tt.got, tt.want)
			}
		})
	}
}

func TestDiscoverResult_JSON(t *testing.T) {
	result := DiscoverResult{
		ResultType:        ResultTypeComplete,
		SupportedVersions: SupportedProtocolVersions,
		Capabilities: ServerCapability{
			Tools: &ToolsCapability{},
		},
		Instructions: "Use the tools.",
		TtlMs:        60000,
		CacheScope:   CacheScopePublic,
		Meta:         ServerInfoMeta(ServerInfo{Name: "test-server", Version: "1.0.0"}),
	}

	data, err := json.Marshal(result)
	if err != nil {
		t.Fatalf("failed to marshal: %v", err)
	}

	// Assert the exact wire field names.
	var raw map[string]json.RawMessage
	if err := json.Unmarshal(data, &raw); err != nil {
		t.Fatalf("failed to unmarshal into map: %v", err)
	}
	for _, key := range []string{"resultType", "supportedVersions", "capabilities", "instructions", "ttlMs", "cacheScope", "_meta"} {
		if _, ok := raw[key]; !ok {
			t.Errorf("marshaled DiscoverResult missing key %q", key)
		}
	}

	var decoded DiscoverResult
	if err := json.Unmarshal(data, &decoded); err != nil {
		t.Fatalf("failed to unmarshal: %v", err)
	}

	if decoded.ResultType != ResultTypeComplete {
		t.Errorf("ResultType = %q, want %q", decoded.ResultType, ResultTypeComplete)
	}
	if len(decoded.SupportedVersions) != len(SupportedProtocolVersions) {
		t.Errorf("SupportedVersions count = %d, want %d", len(decoded.SupportedVersions), len(SupportedProtocolVersions))
	}
	if decoded.TtlMs != 60000 {
		t.Errorf("TtlMs = %d, want 60000", decoded.TtlMs)
	}
	if decoded.CacheScope != CacheScopePublic {
		t.Errorf("CacheScope = %q, want %q", decoded.CacheScope, CacheScopePublic)
	}
	if decoded.Meta[MetaServerInfo] == nil {
		t.Error("expected _meta to carry MetaServerInfo")
	}
}

func TestToolsListResult_ModernFields(t *testing.T) {
	t.Run("legacy shape is byte-identical", func(t *testing.T) {
		result := ToolsListResult{
			Tools: []Tool{
				{Name: "tool1", Description: "First tool", InputSchema: JSONSchema{Type: "object"}},
			},
		}

		data, err := json.Marshal(result)
		if err != nil {
			t.Fatalf("failed to marshal: %v", err)
		}

		want := `{"tools":[{"name":"tool1","description":"First tool","inputSchema":{"type":"object"}}]}`
		if string(data) != want {
			t.Errorf("legacy ToolsListResult marshaled to %s, want %s", data, want)
		}
	})

	t.Run("modern fields round-trip", func(t *testing.T) {
		ttl := int64(0)
		result := ToolsListResult{
			Tools:      []Tool{},
			ResultType: ResultTypeComplete,
			TtlMs:      &ttl,
			CacheScope: CacheScopePrivate,
			Meta:       ServerInfoMeta(ServerInfo{Name: "test-server", Version: "1.0.0"}),
		}

		data, err := json.Marshal(result)
		if err != nil {
			t.Fatalf("failed to marshal: %v", err)
		}

		var raw map[string]json.RawMessage
		if err := json.Unmarshal(data, &raw); err != nil {
			t.Fatalf("failed to unmarshal into map: %v", err)
		}
		for _, key := range []string{"resultType", "ttlMs", "cacheScope", "_meta"} {
			if _, ok := raw[key]; !ok {
				t.Errorf("marshaled ToolsListResult missing key %q", key)
			}
		}
		if string(raw["ttlMs"]) != "0" {
			t.Errorf("ttlMs = %s, want 0 (ttlMs:0 must be representable)", raw["ttlMs"])
		}

		var decoded ToolsListResult
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatalf("failed to unmarshal: %v", err)
		}
		if decoded.TtlMs == nil || *decoded.TtlMs != 0 {
			t.Errorf("TtlMs = %v, want pointer to 0", decoded.TtlMs)
		}
		if decoded.CacheScope != CacheScopePrivate {
			t.Errorf("CacheScope = %q, want %q", decoded.CacheScope, CacheScopePrivate)
		}
	})
}

func TestToolsCallResult_ResultType(t *testing.T) {
	t.Run("legacy shape omits resultType", func(t *testing.T) {
		result := ToolsCallResult{
			Content: []ContentBlock{NewTextContent("ok")},
		}

		data, err := json.Marshal(result)
		if err != nil {
			t.Fatalf("failed to marshal: %v", err)
		}

		want := `{"content":[{"type":"text","text":"ok"}]}`
		if string(data) != want {
			t.Errorf("legacy ToolsCallResult marshaled to %s, want %s", data, want)
		}
	})

	t.Run("modern resultType round-trips", func(t *testing.T) {
		result := ToolsCallResult{
			Content:    []ContentBlock{NewTextContent("ok")},
			ResultType: ResultTypeComplete,
		}

		data, err := json.Marshal(result)
		if err != nil {
			t.Fatalf("failed to marshal: %v", err)
		}

		var decoded ToolsCallResult
		if err := json.Unmarshal(data, &decoded); err != nil {
			t.Fatalf("failed to unmarshal: %v", err)
		}
		if decoded.ResultType != ResultTypeComplete {
			t.Errorf("ResultType = %q, want %q", decoded.ResultType, ResultTypeComplete)
		}
	})
}

func TestUnsupportedProtocolVersionData_JSON(t *testing.T) {
	data, err := json.Marshal(UnsupportedProtocolVersionData{
		Supported: SupportedProtocolVersions,
		Requested: "1999-01-01",
	})
	if err != nil {
		t.Fatalf("failed to marshal: %v", err)
	}

	var raw map[string]json.RawMessage
	if err := json.Unmarshal(data, &raw); err != nil {
		t.Fatalf("failed to unmarshal into map: %v", err)
	}
	if _, ok := raw["supported"]; !ok {
		t.Error("marshaled data missing key supported")
	}
	if _, ok := raw["requested"]; !ok {
		t.Error("marshaled data missing key requested")
	}
	if string(raw["requested"]) != `"1999-01-01"` {
		t.Errorf("requested = %s, want %q", raw["requested"], "1999-01-01")
	}
}

func TestServerInfoMeta(t *testing.T) {
	meta := ServerInfoMeta(ServerInfo{Name: "asc-mcp", Version: "1.0.0"})

	info, ok := meta[MetaServerInfo].(ServerInfo)
	if !ok {
		t.Fatalf("meta[%q] = %T, want ServerInfo", MetaServerInfo, meta[MetaServerInfo])
	}
	if info.Name != "asc-mcp" {
		t.Errorf("Name = %q, want asc-mcp", info.Name)
	}
}

// Benchmarks

func BenchmarkRequest_Unmarshal(b *testing.B) {
	jsonStr := []byte(`{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"list_apps","arguments":{"limit":50}}}`)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		var req Request
		if err := json.Unmarshal(jsonStr, &req); err != nil {
			b.Fatalf("failed to unmarshal: %v", err)
		}
	}
}

func BenchmarkResponse_Marshal(b *testing.B) {
	resp := Response{
		JSONRPC: JSONRPCVersion,
		ID:      json.RawMessage(`1`),
		Result: ToolsCallResult{
			Content: []ContentBlock{
				{Type: "text", Text: "This is a sample response text with some content"},
			},
		},
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, err := json.Marshal(resp)
		if err != nil {
			b.Fatalf("failed to marshal: %v", err)
		}
	}
}

func BenchmarkTool_Marshal(b *testing.B) {
	tool := Tool{
		Name:        "test_tool",
		Description: "A test tool with multiple parameters",
		InputSchema: JSONSchema{
			Type: "object",
			Properties: map[string]Property{
				"param1": {Type: "string", Description: "First parameter"},
				"param2": {Type: "integer", Description: "Second parameter"},
				"param3": {Type: "boolean", Description: "Third parameter"},
			},
			Required: []string{"param1"},
		},
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, err := json.Marshal(tool)
		if err != nil {
			b.Fatalf("failed to marshal: %v", err)
		}
	}
}

func BenchmarkNewSuccessResult(b *testing.B) {
	text := "This is a sample success message for benchmarking purposes"

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = NewSuccessResult(text)
	}
}
