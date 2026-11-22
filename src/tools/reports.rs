//! Ported from internal/asc/tools/reports.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerReportsTools`.
pub(super) fn register_reports_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "get_sales_report",
            "description": "Download sales and trends reports",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "frequency": {
                        "type": "string",
                        "description": "Report frequency (DAILY, WEEKLY, MONTHLY, YEARLY)"
                    },
                    "report_date": {
                        "type": "string",
                        "description": "Report date (YYYY-MM-DD)"
                    },
                    "report_sub_type": {
                        "type": "string",
                        "description": "Report sub-type (SUMMARY, DETAILED, OPT_IN)"
                    },
                    "report_type": {
                        "type": "string",
                        "description": "Report type (SALES, PRE_ORDER, NEWSSTAND, SUBSCRIPTION, SUBSCRIPTION_EVENT, SUBSCRIBER)"
                    },
                    "vendor_number": {
                        "type": "string",
                        "description": "Your vendor number"
                    }
                },
                "required": [
                    "vendor_number",
                    "report_type",
                    "report_sub_type",
                    "frequency",
                    "report_date"
                ]
            },
            "annotations": {
                "title": "Get Sales Report",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_sales_report),
    );
    r.register(
        tool(r##"{
            "name": "get_finance_report",
            "description": "Download financial reports",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "region_code": {
                        "type": "string",
                        "description": "Region code (e.g., US, EU, JP)"
                    },
                    "report_date": {
                        "type": "string",
                        "description": "Report date (YYYY-MM for financial periods)"
                    },
                    "report_type": {
                        "type": "string",
                        "description": "Report type (FINANCIAL, FINANCE_DETAIL)"
                    },
                    "vendor_number": {
                        "type": "string",
                        "description": "Your vendor number"
                    }
                },
                "required": [
                    "vendor_number",
                    "region_code",
                    "report_type",
                    "report_date"
                ]
            },
            "annotations": {
                "title": "Get Finance Report",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_finance_report),
    );
}

/// Ported from `Registry.handleGetSalesReport`.
async fn handle_get_sales_report(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VendorNumber string \"json:\\\"vendor_number\\\"\"; ReportType string \"json:\\\"report_type\\\"\"; ReportSubType string \"json:\\\"report_sub_type\\\"\"; Frequency string \"json:\\\"frequency\\\"\"; ReportDate string \"json:\\\"report_date\\\"\" }", "") {
        vendor_number: String => "vendor_number";
        report_type: String => "report_type";
        report_sub_type: String => "report_sub_type";
        frequency: String => "frequency";
        report_date: String => "report_date";
    }
    }
    todo!("port handleGetSalesReport from internal/asc/tools/reports.go")
}

/// Ported from `Registry.handleGetFinanceReport`.
async fn handle_get_finance_report(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VendorNumber string \"json:\\\"vendor_number\\\"\"; RegionCode string \"json:\\\"region_code\\\"\"; ReportType string \"json:\\\"report_type\\\"\"; ReportDate string \"json:\\\"report_date\\\"\" }", "") {
        vendor_number: String => "vendor_number";
        region_code: String => "region_code";
        report_type: String => "report_type";
        report_date: String => "report_date";
    }
    }
    todo!("port handleGetFinanceReport from internal/asc/tools/reports.go")
}

