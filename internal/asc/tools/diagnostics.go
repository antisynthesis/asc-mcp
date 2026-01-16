package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerDiagnosticsTools registers diagnostics, metrics, and review tools.
func (r *Registry) registerDiagnosticsTools() {
	// List performance power metrics
	r.register(mcp.Tool{
		Name:        "list_perf_power_metrics",
		Description: "List performance and power metrics for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The app ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of metrics to return (default 50)",
				},
			},
			Required: []string{"app_id"},
		},
	}, r.handleListPerfPowerMetrics)

	// List diagnostic signatures
	r.register(mcp.Tool{
		Name:        "list_diagnostic_signatures",
		Description: "List diagnostic signatures (crash/energy/disk reports) for a build",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"build_id": {
					Type:        "string",
					Description: "The build ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of signatures to return (default 50)",
				},
			},
			Required: []string{"build_id"},
		},
	}, r.handleListDiagnosticSignatures)

	// List diagnostic logs
	r.register(mcp.Tool{
		Name:        "list_diagnostic_logs",
		Description: "List diagnostic logs for a signature",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"signature_id": {
					Type:        "string",
					Description: "The diagnostic signature ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of logs to return (default 50)",
				},
			},
			Required: []string{"signature_id"},
		},
	}, r.handleListDiagnosticLogs)

	// List app store review attachments
	r.register(mcp.Tool{
		Name:        "list_app_store_review_attachments",
		Description: "List App Store review submission attachments for a version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The app store version ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of attachments to return (default 50)",
				},
			},
			Required: []string{"version_id"},
		},
	}, r.handleListAppStoreReviewAttachments)

	// Get app store review attachment
	r.register(mcp.Tool{
		Name:        "get_app_store_review_attachment",
		Description: "Get details of a review attachment",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"attachment_id": {
					Type:        "string",
					Description: "The attachment ID",
				},
			},
			Required: []string{"attachment_id"},
		},
	}, r.handleGetAppStoreReviewAttachment)

	// Create app store review attachment
	r.register(mcp.Tool{
		Name:        "create_app_store_review_attachment",
		Description: "Create a new review attachment for submission",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"review_detail_id": {
					Type:        "string",
					Description: "The app store review detail ID",
				},
				"file_name": {
					Type:        "string",
					Description: "Name of the file",
				},
				"file_size": {
					Type:        "integer",
					Description: "Size of the file in bytes",
				},
			},
			Required: []string{"review_detail_id", "file_name", "file_size"},
		},
	}, r.handleCreateAppStoreReviewAttachment)

	// Delete app store review attachment
	r.register(mcp.Tool{
		Name:        "delete_app_store_review_attachment",
		Description: "Delete a review attachment",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"attachment_id": {
					Type:        "string",
					Description: "The attachment ID to delete",
				},
			},
			Required: []string{"attachment_id"},
		},
	}, r.handleDeleteAppStoreReviewAttachment)

	// Get routing app coverage
	r.register(mcp.Tool{
		Name:        "get_routing_app_coverage",
		Description: "Get routing app coverage file information for a version",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The app store version ID",
				},
			},
			Required: []string{"version_id"},
		},
	}, r.handleGetRoutingAppCoverage)

	// Create routing app coverage
	r.register(mcp.Tool{
		Name:        "create_routing_app_coverage",
		Description: "Create a routing app coverage file upload reservation",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"version_id": {
					Type:        "string",
					Description: "The app store version ID",
				},
				"file_name": {
					Type:        "string",
					Description: "Name of the GeoJSON file",
				},
				"file_size": {
					Type:        "integer",
					Description: "Size of the file in bytes",
				},
			},
			Required: []string{"version_id", "file_name", "file_size"},
		},
	}, r.handleCreateRoutingAppCoverage)

	// Delete routing app coverage
	r.register(mcp.Tool{
		Name:        "delete_routing_app_coverage",
		Description: "Delete a routing app coverage file",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"coverage_id": {
					Type:        "string",
					Description: "The routing app coverage ID to delete",
				},
			},
			Required: []string{"coverage_id"},
		},
	}, r.handleDeleteRoutingAppCoverage)
}

func (r *Registry) handleListPerfPowerMetrics(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
		Limit int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListPerfPowerMetrics(context.Background(), params.AppID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list performance metrics: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatPerfPowerMetrics(resp.Data)), nil
}

func (r *Registry) handleListDiagnosticSignatures(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BuildID string `json:"build_id"`
		Limit   int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BuildID == "" {
		return nil, fmt.Errorf("build_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListDiagnosticSignatures(context.Background(), params.BuildID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list diagnostic signatures: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatDiagnosticSignatures(resp.Data)), nil
}

func (r *Registry) handleListDiagnosticLogs(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		SignatureID string `json:"signature_id"`
		Limit       int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.SignatureID == "" {
		return nil, fmt.Errorf("signature_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListDiagnosticLogs(context.Background(), params.SignatureID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list diagnostic logs: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatDiagnosticLogs(resp.Data)), nil
}

func (r *Registry) handleListAppStoreReviewAttachments(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
		Limit     int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return nil, fmt.Errorf("version_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListAppStoreReviewAttachments(context.Background(), params.VersionID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list review attachments: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppStoreReviewAttachments(resp.Data)), nil
}

func (r *Registry) handleGetAppStoreReviewAttachment(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AttachmentID string `json:"attachment_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AttachmentID == "" {
		return nil, fmt.Errorf("attachment_id is required")
	}

	resp, err := r.client.GetAppStoreReviewAttachment(context.Background(), params.AttachmentID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get review attachment: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatAppStoreReviewAttachment(resp.Data)), nil
}

func (r *Registry) handleCreateAppStoreReviewAttachment(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ReviewDetailID string `json:"review_detail_id"`
		FileName       string `json:"file_name"`
		FileSize       int    `json:"file_size"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ReviewDetailID == "" || params.FileName == "" || params.FileSize <= 0 {
		return nil, fmt.Errorf("review_detail_id, file_name, and file_size are required")
	}

	req := &api.AppStoreReviewAttachmentCreateRequest{
		Data: api.AppStoreReviewAttachmentCreateData{
			Type: "appStoreReviewAttachments",
			Attributes: api.AppStoreReviewAttachmentCreateAttributes{
				FileName: params.FileName,
				FileSize: params.FileSize,
			},
			Relationships: api.AppStoreReviewAttachmentCreateRelationships{
				AppStoreReviewDetail: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "appStoreReviewDetails", ID: params.ReviewDetailID},
				},
			},
		},
	}

	resp, err := r.client.CreateAppStoreReviewAttachment(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create review attachment: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Review attachment reservation created:\n%s", formatAppStoreReviewAttachment(resp.Data))), nil
}

func (r *Registry) handleDeleteAppStoreReviewAttachment(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AttachmentID string `json:"attachment_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AttachmentID == "" {
		return nil, fmt.Errorf("attachment_id is required")
	}

	err := r.client.DeleteAppStoreReviewAttachment(context.Background(), params.AttachmentID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete review attachment: %v", err)), nil
	}

	return mcp.NewSuccessResult("Review attachment deleted"), nil
}

func (r *Registry) handleGetRoutingAppCoverage(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" {
		return nil, fmt.Errorf("version_id is required")
	}

	resp, err := r.client.GetRoutingAppCoverage(context.Background(), params.VersionID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get routing app coverage: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatRoutingAppCoverage(resp.Data)), nil
}

func (r *Registry) handleCreateRoutingAppCoverage(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VersionID string `json:"version_id"`
		FileName  string `json:"file_name"`
		FileSize  int    `json:"file_size"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VersionID == "" || params.FileName == "" || params.FileSize <= 0 {
		return nil, fmt.Errorf("version_id, file_name, and file_size are required")
	}

	req := &api.RoutingAppCoverageCreateRequest{
		Data: api.RoutingAppCoverageCreateData{
			Type: "routingAppCoverages",
			Attributes: api.RoutingAppCoverageCreateAttributes{
				FileName: params.FileName,
				FileSize: params.FileSize,
			},
			Relationships: api.RoutingAppCoverageCreateRelationships{
				AppStoreVersion: api.RelationshipData{
					Data: api.ResourceIdentifier{Type: "appStoreVersions", ID: params.VersionID},
				},
			},
		},
	}

	resp, err := r.client.CreateRoutingAppCoverage(context.Background(), req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create routing app coverage: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Routing app coverage reservation created:\n%s", formatRoutingAppCoverage(resp.Data))), nil
}

func (r *Registry) handleDeleteRoutingAppCoverage(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		CoverageID string `json:"coverage_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.CoverageID == "" {
		return nil, fmt.Errorf("coverage_id is required")
	}

	err := r.client.DeleteRoutingAppCoverage(context.Background(), params.CoverageID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete routing app coverage: %v", err)), nil
	}

	return mcp.NewSuccessResult("Routing app coverage deleted"), nil
}

func formatPerfPowerMetrics(metrics []api.PerfPowerMetric) string {
	if len(metrics) == 0 {
		return "No performance metrics found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d performance metrics:\n\n", len(metrics)))

	for _, m := range metrics {
		sb.WriteString(fmt.Sprintf("ID: %s\n", m.ID))
		sb.WriteString(fmt.Sprintf("Device Type: %s\n", m.Attributes.DeviceType))
		sb.WriteString(fmt.Sprintf("Metric Type: %s\n", m.Attributes.MetricType))
		sb.WriteString(fmt.Sprintf("Platform: %s\n", m.Attributes.Platform))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatDiagnosticSignatures(signatures []api.DiagnosticSignature) string {
	if len(signatures) == 0 {
		return "No diagnostic signatures found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d diagnostic signatures:\n\n", len(signatures)))

	for _, s := range signatures {
		sb.WriteString(fmt.Sprintf("ID: %s\n", s.ID))
		sb.WriteString(fmt.Sprintf("Diagnostic Type: %s\n", s.Attributes.DiagnosticType))
		sb.WriteString(fmt.Sprintf("Signature: %s\n", s.Attributes.Signature))
		sb.WriteString(fmt.Sprintf("Weight: %.2f\n", s.Attributes.Weight))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatDiagnosticLogs(logs []api.DiagnosticLog) string {
	if len(logs) == 0 {
		return "No diagnostic logs found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d diagnostic logs:\n\n", len(logs)))

	for _, l := range logs {
		sb.WriteString(fmt.Sprintf("ID: %s\n", l.ID))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAppStoreReviewAttachments(attachments []api.AppStoreReviewAttachment) string {
	if len(attachments) == 0 {
		return "No review attachments found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d review attachments:\n\n", len(attachments)))

	for _, a := range attachments {
		sb.WriteString(formatAppStoreReviewAttachment(a))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAppStoreReviewAttachment(a api.AppStoreReviewAttachment) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", a.ID))
	sb.WriteString(fmt.Sprintf("File Name: %s\n", a.Attributes.FileName))
	sb.WriteString(fmt.Sprintf("File Size: %d bytes\n", a.Attributes.FileSize))
	if a.Attributes.AssetDeliveryState != nil {
		sb.WriteString(fmt.Sprintf("State: %s\n", a.Attributes.AssetDeliveryState.State))
	}
	return sb.String()
}

func formatRoutingAppCoverage(coverage api.RoutingAppCoverage) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", coverage.ID))
	sb.WriteString(fmt.Sprintf("File Name: %s\n", coverage.Attributes.FileName))
	sb.WriteString(fmt.Sprintf("File Size: %d bytes\n", coverage.Attributes.FileSize))
	if coverage.Attributes.AssetDeliveryState != nil {
		sb.WriteString(fmt.Sprintf("State: %s\n", coverage.Attributes.AssetDeliveryState.State))
	}
	return sb.String()
}
