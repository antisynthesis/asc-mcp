// Package tools provides MCP tool implementations for App Store Connect.
package tools

import (
	"encoding/json"
	"fmt"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// ToolHandler is a function that handles a tool call.
type ToolHandler func(args json.RawMessage) (*mcp.ToolsCallResult, error)

// Registry manages tool definitions and handlers.
type Registry struct {
	client   *api.Client
	tools    []mcp.Tool
	handlers map[string]ToolHandler
}

// NewRegistry creates a new tool registry.
func NewRegistry(client *api.Client) *Registry {
	r := &Registry{
		client:   client,
		tools:    make([]mcp.Tool, 0),
		handlers: make(map[string]ToolHandler),
	}

	// Core app management
	r.registerAppTools()
	r.registerBuildTools()
	r.registerTestFlightTools()
	r.registerProvisioningTools()

	// Localization
	r.registerAppInfoLocalizationTools()
	r.registerVersionLocalizationTools()

	// Customer reviews
	r.registerCustomerReviewTools()

	// In-app purchases and subscriptions
	r.registerInAppPurchaseTools()
	r.registerSubscriptionTools()

	// App Store versions and submissions
	r.registerVersionSubmissionTools()
	r.registerPhasedReleaseTools()

	// Screenshots and previews
	r.registerScreenshotTools()

	// Pre-orders
	r.registerPreOrderTools()

	// App events
	r.registerAppEventTools()

	// Analytics
	r.registerAnalyticsTools()

	// App clips
	r.registerAppClipTools()

	// Game Center
	r.registerGameCenterTools()

	// Xcode Cloud
	r.registerXcodeCloudTools()

	// Reports
	r.registerReportsTools()

	// Encryption
	r.registerEncryptionTools()

	return r
}

// ListTools returns all registered tool definitions.
func (r *Registry) ListTools() []mcp.Tool {
	return r.tools
}

// CallTool executes a tool by name.
func (r *Registry) CallTool(name string, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	handler, ok := r.handlers[name]
	if !ok {
		return nil, fmt.Errorf("unknown tool: %s", name)
	}

	return handler(args)
}

// register adds a tool to the registry.
func (r *Registry) register(tool mcp.Tool, handler ToolHandler) {
	r.tools = append(r.tools, tool)
	r.handlers[tool.Name] = handler
}
