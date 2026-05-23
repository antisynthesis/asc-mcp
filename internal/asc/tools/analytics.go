package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerAnalyticsTools registers analytics tools.
func (r *Registry) registerAnalyticsTools() {
	// List analytics report requests
	r.register(mcp.Tool{
		Name:        "list_analytics_report_requests",
		Description: "List analytics report requests for an app",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of requests to return (default 50)",
				},
				"cursor": {
					Type:        "string",
					Description: "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page.",
				},
			},
			Required: []string{"app_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Analytics Report Requests",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAnalyticsReportRequests)

	// Get analytics report request
	r.register(mcp.Tool{
		Name:        "get_analytics_report_request",
		Description: "Get details of an analytics report request",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"request_id": {
					Type:        "string",
					Description: "The analytics report request ID",
				},
			},
			Required: []string{"request_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "Get Analytics Report Request",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleGetAnalyticsReportRequest)

	// Create analytics report request
	r.register(mcp.Tool{
		Name:        "create_analytics_report_request",
		Description: "Create a new analytics report request",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "The App ID",
				},
				"access_type": {
					Type:        "string",
					Description: "Access type (ONE_TIME_SNAPSHOT, ONGOING)",
				},
			},
			Required: []string{"app_id", "access_type"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Create Analytics Report Request",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(false),
			IdempotentHint:  mcp.BoolPtr(false),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleCreateAnalyticsReportRequest)

	// Delete analytics report request
	r.register(mcp.Tool{
		Name:        "delete_analytics_report_request",
		Description: "Delete an analytics report request",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"request_id": {
					Type:        "string",
					Description: "The analytics report request ID",
				},
			},
			Required: []string{"request_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:           "Delete Analytics Report Request",
			ReadOnlyHint:    mcp.BoolPtr(false),
			DestructiveHint: mcp.BoolPtr(true),
			IdempotentHint:  mcp.BoolPtr(true),
			OpenWorldHint:   mcp.BoolPtr(true),
		},
	}, r.handleDeleteAnalyticsReportRequest)

	// List analytics reports
	r.register(mcp.Tool{
		Name:        "list_analytics_reports",
		Description: "List analytics reports for a report request",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"request_id": {
					Type:        "string",
					Description: "The analytics report request ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of reports to return (default 50)",
				},
				"cursor": {
					Type:        "string",
					Description: "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page.",
				},
			},
			Required: []string{"request_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Analytics Reports",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAnalyticsReports)

	// List analytics report instances
	r.register(mcp.Tool{
		Name:        "list_analytics_report_instances",
		Description: "List instances for an analytics report",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"report_id": {
					Type:        "string",
					Description: "The analytics report ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of instances to return (default 50)",
				},
				"cursor": {
					Type:        "string",
					Description: "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page.",
				},
			},
			Required: []string{"report_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Analytics Report Instances",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAnalyticsReportInstances)

	// List analytics report segments
	r.register(mcp.Tool{
		Name:        "list_analytics_report_segments",
		Description: "List segments for an analytics report instance (contains download URLs)",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"instance_id": {
					Type:        "string",
					Description: "The analytics report instance ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of segments to return (default 50)",
				},
				"cursor": {
					Type:        "string",
					Description: "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page.",
				},
			},
			Required: []string{"instance_id"},
		},
		Annotations: &mcp.ToolAnnotations{
			Title:         "List Analytics Report Segments",
			ReadOnlyHint:  mcp.BoolPtr(true),
			OpenWorldHint: mcp.BoolPtr(true),
		},
	}, r.handleListAnalyticsReportSegments)
}

func (r *Registry) handleListAnalyticsReportRequests(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID  string `json:"app_id"`
		Limit  int    `json:"limit"`
		Cursor string `json:"cursor"`
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
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AnalyticsReportRequestsResponse, error) {
		return r.client.ListAnalyticsReportRequests(ctx, params.AppID, limit)
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list analytics report requests: %v", err)), nil
	}

	return newListResult(formatAnalyticsReportRequests(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleGetAnalyticsReportRequest(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		RequestID string `json:"request_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.RequestID == "" {
		return nil, fmt.Errorf("request_id is required")
	}

	resp, err := r.client.GetAnalyticsReportRequest(ctx, params.RequestID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get analytics report request: %v", err)), nil
	}

	return newDataResult(formatAnalyticsReportRequest(resp.Data), resp.Data), nil
}

func (r *Registry) handleCreateAnalyticsReportRequest(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID      string `json:"app_id"`
		AccessType string `json:"access_type"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.AppID == "" {
		return nil, fmt.Errorf("app_id is required")
	}
	if params.AccessType == "" {
		return nil, fmt.Errorf("access_type is required")
	}

	req := &api.AnalyticsReportRequestCreateRequest{
		Data: api.AnalyticsReportRequestCreateData{
			Type: "analyticsReportRequests",
			Attributes: api.AnalyticsReportRequestCreateAttributes{
				AccessType: params.AccessType,
			},
			Relationships: api.AnalyticsReportRequestCreateRelationships{
				App: api.RelationshipData{
					Data: api.ResourceIdentifier{
						Type: "apps",
						ID:   params.AppID,
					},
				},
			},
		},
	}

	resp, err := r.client.CreateAnalyticsReportRequest(ctx, req)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to create analytics report request: %v", err)), nil
	}

	return newDataResult(fmt.Sprintf("Created analytics report request: %s", resp.Data.ID), resp.Data), nil
}

func (r *Registry) handleDeleteAnalyticsReportRequest(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		RequestID string `json:"request_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.RequestID == "" {
		return nil, fmt.Errorf("request_id is required")
	}

	err := r.client.DeleteAnalyticsReportRequest(ctx, params.RequestID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to delete analytics report request: %v", err)), nil
	}

	return mcp.NewSuccessResult("Analytics report request deleted successfully"), nil
}

func (r *Registry) handleListAnalyticsReports(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		RequestID string `json:"request_id"`
		Limit     int    `json:"limit"`
		Cursor    string `json:"cursor"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.RequestID == "" {
		return nil, fmt.Errorf("request_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AnalyticsReportsResponse, error) {
		return r.client.ListAnalyticsReports(ctx, params.RequestID, limit)
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list analytics reports: %v", err)), nil
	}

	return newListResult(formatAnalyticsReports(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleListAnalyticsReportInstances(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ReportID string `json:"report_id"`
		Limit    int    `json:"limit"`
		Cursor   string `json:"cursor"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ReportID == "" {
		return nil, fmt.Errorf("report_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AnalyticsReportInstancesResponse, error) {
		return r.client.ListAnalyticsReportInstances(ctx, params.ReportID, limit)
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list analytics report instances: %v", err)), nil
	}

	return newListResult(formatAnalyticsReportInstances(resp.Data), resp.Data, resp.Links), nil
}

func (r *Registry) handleListAnalyticsReportSegments(ctx context.Context, args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		InstanceID string `json:"instance_id"`
		Limit      int    `json:"limit"`
		Cursor     string `json:"cursor"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.InstanceID == "" {
		return nil, fmt.Errorf("instance_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	resp, err := paginatedFetch(ctx, r.client, params.Cursor, func() (*api.AnalyticsReportSegmentsResponse, error) {
		return r.client.ListAnalyticsReportSegments(ctx, params.InstanceID, limit)
	})
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list analytics report segments: %v", err)), nil
	}

	return newListResult(formatAnalyticsReportSegments(resp.Data), resp.Data, resp.Links), nil
}

func formatAnalyticsReportRequests(requests []api.AnalyticsReportRequest) string {
	if len(requests) == 0 {
		return "No analytics report requests found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d analytics report requests:\n\n", len(requests)))

	for _, req := range requests {
		sb.WriteString(formatAnalyticsReportRequest(req))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatAnalyticsReportRequest(req api.AnalyticsReportRequest) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", req.ID))
	sb.WriteString(fmt.Sprintf("Access Type: %s\n", req.Attributes.AccessType))
	sb.WriteString(fmt.Sprintf("Stoppable: %t\n", req.Attributes.Stoppable))
	return sb.String()
}

func formatAnalyticsReports(reports []api.AnalyticsReport) string {
	if len(reports) == 0 {
		return "No analytics reports found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d analytics reports:\n\n", len(reports)))

	for _, report := range reports {
		sb.WriteString(fmt.Sprintf("ID: %s\n", report.ID))
		sb.WriteString(fmt.Sprintf("Category: %s\n", report.Attributes.Category))
		sb.WriteString(fmt.Sprintf("Name: %s\n", report.Attributes.Name))
		sb.WriteString("---\n")
	}

	return sb.String()
}

func formatAnalyticsReportInstances(instances []api.AnalyticsReportInstance) string {
	if len(instances) == 0 {
		return "No analytics report instances found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d analytics report instances:\n\n", len(instances)))

	for _, instance := range instances {
		sb.WriteString(fmt.Sprintf("ID: %s\n", instance.ID))
		sb.WriteString(fmt.Sprintf("Granularity: %s\n", instance.Attributes.Granularity))
		sb.WriteString(fmt.Sprintf("Processing Date: %s\n", instance.Attributes.ProcessingDate))
		sb.WriteString("---\n")
	}

	return sb.String()
}

func formatAnalyticsReportSegments(segments []api.AnalyticsReportSegment) string {
	if len(segments) == 0 {
		return "No analytics report segments found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d analytics report segments:\n\n", len(segments)))

	for _, segment := range segments {
		sb.WriteString(fmt.Sprintf("ID: %s\n", segment.ID))
		sb.WriteString(fmt.Sprintf("Size: %d bytes\n", segment.Attributes.SizeInBytes))
		sb.WriteString(fmt.Sprintf("Checksum: %s\n", segment.Attributes.Checksum))
		if segment.Attributes.URL != "" {
			sb.WriteString(fmt.Sprintf("Download URL: %s\n", segment.Attributes.URL))
		}
		sb.WriteString("---\n")
	}

	return sb.String()
}
