package mcp

import (
	"encoding/json"
	"testing"
)

func TestConstants(t *testing.T) {
	if JSONRPCVersion != "2.0" {
		t.Errorf("JSONRPCVersion = %q, want 2.0", JSONRPCVersion)
	}

	if ProtocolVersion != "2024-11-05" {
		t.Errorf("ProtocolVersion = %q, want 2024-11-05", ProtocolVersion)
	}
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
