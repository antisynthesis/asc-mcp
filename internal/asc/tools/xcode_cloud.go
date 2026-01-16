package tools

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerXcodeCloudTools registers Xcode Cloud tools.
func (r *Registry) registerXcodeCloudTools() {
	// List CI products
	r.register(mcp.Tool{
		Name:        "list_ci_products",
		Description: "List Xcode Cloud CI products",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"app_id": {
					Type:        "string",
					Description: "Filter by App ID (optional)",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of products to return (default 50)",
				},
			},
		},
	}, r.handleListCiProducts)

	// Get CI product
	r.register(mcp.Tool{
		Name:        "get_ci_product",
		Description: "Get details of a specific Xcode Cloud product",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"product_id": {
					Type:        "string",
					Description: "The CI product ID",
				},
			},
			Required: []string{"product_id"},
		},
	}, r.handleGetCiProduct)

	// List CI workflows
	r.register(mcp.Tool{
		Name:        "list_ci_workflows",
		Description: "List Xcode Cloud workflows for a product",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"product_id": {
					Type:        "string",
					Description: "The CI product ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of workflows to return (default 50)",
				},
			},
			Required: []string{"product_id"},
		},
	}, r.handleListCiWorkflows)

	// Get CI workflow
	r.register(mcp.Tool{
		Name:        "get_ci_workflow",
		Description: "Get details of a specific Xcode Cloud workflow",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"workflow_id": {
					Type:        "string",
					Description: "The CI workflow ID",
				},
			},
			Required: []string{"workflow_id"},
		},
	}, r.handleGetCiWorkflow)

	// List CI build runs
	r.register(mcp.Tool{
		Name:        "list_ci_build_runs",
		Description: "List Xcode Cloud build runs for a workflow",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"workflow_id": {
					Type:        "string",
					Description: "The CI workflow ID",
				},
				"limit": {
					Type:        "integer",
					Description: "Maximum number of build runs to return (default 50)",
				},
			},
			Required: []string{"workflow_id"},
		},
	}, r.handleListCiBuildRuns)

	// Get CI build run
	r.register(mcp.Tool{
		Name:        "get_ci_build_run",
		Description: "Get details of a specific Xcode Cloud build run",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"build_run_id": {
					Type:        "string",
					Description: "The CI build run ID",
				},
			},
			Required: []string{"build_run_id"},
		},
	}, r.handleGetCiBuildRun)

	// Start CI build run
	r.register(mcp.Tool{
		Name:        "start_ci_build_run",
		Description: "Start a new Xcode Cloud build run for a workflow",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"workflow_id": {
					Type:        "string",
					Description: "The CI workflow ID to start",
				},
			},
			Required: []string{"workflow_id"},
		},
	}, r.handleStartCiBuildRun)

	// Cancel CI build run
	r.register(mcp.Tool{
		Name:        "cancel_ci_build_run",
		Description: "Cancel a running Xcode Cloud build run",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"build_run_id": {
					Type:        "string",
					Description: "The CI build run ID to cancel",
				},
			},
			Required: []string{"build_run_id"},
		},
	}, r.handleCancelCiBuildRun)
}

func (r *Registry) handleListCiProducts(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		AppID string `json:"app_id"`
		Limit int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListCiProducts(context.Background(), params.AppID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list CI products: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatCiProducts(resp.Data)), nil
}

func (r *Registry) handleGetCiProduct(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ProductID string `json:"product_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ProductID == "" {
		return nil, fmt.Errorf("product_id is required")
	}

	resp, err := r.client.GetCiProduct(context.Background(), params.ProductID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get CI product: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatCiProduct(resp.Data)), nil
}

func (r *Registry) handleListCiWorkflows(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		ProductID string `json:"product_id"`
		Limit     int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.ProductID == "" {
		return nil, fmt.Errorf("product_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListCiWorkflows(context.Background(), params.ProductID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list CI workflows: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatCiWorkflows(resp.Data)), nil
}

func (r *Registry) handleGetCiWorkflow(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		WorkflowID string `json:"workflow_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.WorkflowID == "" {
		return nil, fmt.Errorf("workflow_id is required")
	}

	resp, err := r.client.GetCiWorkflow(context.Background(), params.WorkflowID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get CI workflow: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatCiWorkflow(resp.Data)), nil
}

func (r *Registry) handleListCiBuildRuns(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		WorkflowID string `json:"workflow_id"`
		Limit      int    `json:"limit"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.WorkflowID == "" {
		return nil, fmt.Errorf("workflow_id is required")
	}

	limit := params.Limit
	if limit <= 0 {
		limit = 50
	}

	resp, err := r.client.ListCiBuildRuns(context.Background(), params.WorkflowID, limit)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to list CI build runs: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatCiBuildRuns(resp.Data)), nil
}

func (r *Registry) handleGetCiBuildRun(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BuildRunID string `json:"build_run_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BuildRunID == "" {
		return nil, fmt.Errorf("build_run_id is required")
	}

	resp, err := r.client.GetCiBuildRun(context.Background(), params.BuildRunID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get CI build run: %v", err)), nil
	}

	return mcp.NewSuccessResult(formatCiBuildRun(resp.Data)), nil
}

func (r *Registry) handleStartCiBuildRun(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		WorkflowID string `json:"workflow_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.WorkflowID == "" {
		return nil, fmt.Errorf("workflow_id is required")
	}

	resp, err := r.client.StartCiBuildRun(context.Background(), params.WorkflowID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to start CI build run: %v", err)), nil
	}

	return mcp.NewSuccessResult(fmt.Sprintf("Started build run: %s (build #%d)", resp.Data.ID, resp.Data.Attributes.Number)), nil
}

func (r *Registry) handleCancelCiBuildRun(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		BuildRunID string `json:"build_run_id"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.BuildRunID == "" {
		return nil, fmt.Errorf("build_run_id is required")
	}

	err := r.client.CancelCiBuildRun(context.Background(), params.BuildRunID)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to cancel CI build run: %v", err)), nil
	}

	return mcp.NewSuccessResult("Build run cancelled successfully"), nil
}

func formatCiProducts(products []api.CiProduct) string {
	if len(products) == 0 {
		return "No CI products found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d CI products:\n\n", len(products)))

	for _, product := range products {
		sb.WriteString(formatCiProduct(product))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatCiProduct(product api.CiProduct) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", product.ID))
	sb.WriteString(fmt.Sprintf("Name: %s\n", product.Attributes.Name))
	sb.WriteString(fmt.Sprintf("Product Type: %s\n", product.Attributes.ProductType))
	if product.Attributes.CreatedDate != nil {
		sb.WriteString(fmt.Sprintf("Created: %s\n", product.Attributes.CreatedDate.Format("2006-01-02")))
	}
	return sb.String()
}

func formatCiWorkflows(workflows []api.CiWorkflow) string {
	if len(workflows) == 0 {
		return "No CI workflows found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d CI workflows:\n\n", len(workflows)))

	for _, workflow := range workflows {
		sb.WriteString(formatCiWorkflow(workflow))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatCiWorkflow(workflow api.CiWorkflow) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", workflow.ID))
	sb.WriteString(fmt.Sprintf("Name: %s\n", workflow.Attributes.Name))
	if workflow.Attributes.Description != "" {
		sb.WriteString(fmt.Sprintf("Description: %s\n", workflow.Attributes.Description))
	}
	sb.WriteString(fmt.Sprintf("Enabled: %t\n", workflow.Attributes.IsEnabled))
	sb.WriteString(fmt.Sprintf("Clean Build: %t\n", workflow.Attributes.Clean))
	if workflow.Attributes.ContainerFilePath != "" {
		sb.WriteString(fmt.Sprintf("Container File: %s\n", workflow.Attributes.ContainerFilePath))
	}
	if workflow.Attributes.LastModifiedDate != nil {
		sb.WriteString(fmt.Sprintf("Last Modified: %s\n", workflow.Attributes.LastModifiedDate.Format("2006-01-02 15:04")))
	}
	return sb.String()
}

func formatCiBuildRuns(runs []api.CiBuildRun) string {
	if len(runs) == 0 {
		return "No CI build runs found"
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Found %d CI build runs:\n\n", len(runs)))

	for _, run := range runs {
		sb.WriteString(formatCiBuildRun(run))
		sb.WriteString("\n---\n")
	}

	return sb.String()
}

func formatCiBuildRun(run api.CiBuildRun) string {
	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("ID: %s\n", run.ID))
	sb.WriteString(fmt.Sprintf("Build #%d\n", run.Attributes.Number))
	sb.WriteString(fmt.Sprintf("Progress: %s\n", run.Attributes.ExecutionProgress))
	if run.Attributes.CompletionStatus != "" {
		sb.WriteString(fmt.Sprintf("Status: %s\n", run.Attributes.CompletionStatus))
	}
	sb.WriteString(fmt.Sprintf("Start Reason: %s\n", run.Attributes.StartReason))
	sb.WriteString(fmt.Sprintf("Pull Request Build: %t\n", run.Attributes.IsPullRequestBuild))
	if run.Attributes.SourceCommit != nil {
		sb.WriteString(fmt.Sprintf("Commit: %s\n", run.Attributes.SourceCommit.CommitSha))
		if run.Attributes.SourceCommit.Message != "" {
			sb.WriteString(fmt.Sprintf("Message: %s\n", run.Attributes.SourceCommit.Message))
		}
	}
	if run.Attributes.CreatedDate != nil {
		sb.WriteString(fmt.Sprintf("Created: %s\n", run.Attributes.CreatedDate.Format("2006-01-02 15:04")))
	}
	if run.Attributes.StartedDate != nil {
		sb.WriteString(fmt.Sprintf("Started: %s\n", run.Attributes.StartedDate.Format("2006-01-02 15:04")))
	}
	if run.Attributes.FinishedDate != nil {
		sb.WriteString(fmt.Sprintf("Finished: %s\n", run.Attributes.FinishedDate.Format("2006-01-02 15:04")))
	}
	return sb.String()
}
