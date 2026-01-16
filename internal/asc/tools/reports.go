package tools

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerReportsTools registers sales and finance report tools.
func (r *Registry) registerReportsTools() {
	// Get sales report
	r.register(mcp.Tool{
		Name:        "get_sales_report",
		Description: "Download sales and trends reports",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"vendor_number": {
					Type:        "string",
					Description: "Your vendor number",
				},
				"report_type": {
					Type:        "string",
					Description: "Report type (SALES, PRE_ORDER, NEWSSTAND, SUBSCRIPTION, SUBSCRIPTION_EVENT, SUBSCRIBER)",
				},
				"report_sub_type": {
					Type:        "string",
					Description: "Report sub-type (SUMMARY, DETAILED, OPT_IN)",
				},
				"frequency": {
					Type:        "string",
					Description: "Report frequency (DAILY, WEEKLY, MONTHLY, YEARLY)",
				},
				"report_date": {
					Type:        "string",
					Description: "Report date (YYYY-MM-DD)",
				},
			},
			Required: []string{"vendor_number", "report_type", "report_sub_type", "frequency", "report_date"},
		},
	}, r.handleGetSalesReport)

	// Get finance report
	r.register(mcp.Tool{
		Name:        "get_finance_report",
		Description: "Download financial reports",
		InputSchema: mcp.JSONSchema{
			Type: "object",
			Properties: map[string]mcp.Property{
				"vendor_number": {
					Type:        "string",
					Description: "Your vendor number",
				},
				"region_code": {
					Type:        "string",
					Description: "Region code (e.g., US, EU, JP)",
				},
				"report_type": {
					Type:        "string",
					Description: "Report type (FINANCIAL, FINANCE_DETAIL)",
				},
				"report_date": {
					Type:        "string",
					Description: "Report date (YYYY-MM for financial periods)",
				},
			},
			Required: []string{"vendor_number", "region_code", "report_type", "report_date"},
		},
	}, r.handleGetFinanceReport)
}

func (r *Registry) handleGetSalesReport(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VendorNumber  string `json:"vendor_number"`
		ReportType    string `json:"report_type"`
		ReportSubType string `json:"report_sub_type"`
		Frequency     string `json:"frequency"`
		ReportDate    string `json:"report_date"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VendorNumber == "" {
		return nil, fmt.Errorf("vendor_number is required")
	}
	if params.ReportType == "" {
		return nil, fmt.Errorf("report_type is required")
	}
	if params.ReportSubType == "" {
		return nil, fmt.Errorf("report_sub_type is required")
	}
	if params.Frequency == "" {
		return nil, fmt.Errorf("frequency is required")
	}
	if params.ReportDate == "" {
		return nil, fmt.Errorf("report_date is required")
	}

	data, err := r.client.GetSalesReport(context.Background(), params.VendorNumber, params.ReportType, params.ReportSubType, params.Frequency, params.ReportDate)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get sales report: %v", err)), nil
	}

	// Sales reports are typically gzip-compressed TSV files
	return mcp.NewSuccessResult(fmt.Sprintf("Sales report downloaded (%d bytes). Data is gzip-compressed TSV format.\n\nFirst 1000 bytes:\n%s", len(data), truncateString(string(data), 1000))), nil
}

func (r *Registry) handleGetFinanceReport(args json.RawMessage) (*mcp.ToolsCallResult, error) {
	var params struct {
		VendorNumber string `json:"vendor_number"`
		RegionCode   string `json:"region_code"`
		ReportType   string `json:"report_type"`
		ReportDate   string `json:"report_date"`
	}
	if err := json.Unmarshal(args, &params); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if params.VendorNumber == "" {
		return nil, fmt.Errorf("vendor_number is required")
	}
	if params.RegionCode == "" {
		return nil, fmt.Errorf("region_code is required")
	}
	if params.ReportType == "" {
		return nil, fmt.Errorf("report_type is required")
	}
	if params.ReportDate == "" {
		return nil, fmt.Errorf("report_date is required")
	}

	data, err := r.client.GetFinanceReport(context.Background(), params.VendorNumber, params.RegionCode, params.ReportType, params.ReportDate)
	if err != nil {
		return mcp.NewErrorResult(fmt.Sprintf("Failed to get finance report: %v", err)), nil
	}

	// Finance reports are typically gzip-compressed TSV files
	return mcp.NewSuccessResult(fmt.Sprintf("Finance report downloaded (%d bytes). Data is gzip-compressed TSV format.\n\nFirst 1000 bytes:\n%s", len(data), truncateString(string(data), 1000))), nil
}

func truncateString(s string, maxLen int) string {
	if len(s) <= maxLen {
		return s
	}
	return s[:maxLen] + "..."
}
