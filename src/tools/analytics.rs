//! Ported from internal/asc/tools/analytics.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerAnalyticsTools`.
pub(super) fn register_analytics_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_analytics_report_requests",
            "description": "List analytics report requests for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"platform\": [\"IOS\"]} becomes filter[platform]=IOS."
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response.",
                        "items": {
                            "type": "string"
                        }
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of requests to return (default 50)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "app_id"
                ]
            },
            "annotations": {
                "title": "List Analytics Report Requests",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_analytics_report_requests),
    );
    r.register(
        tool(r##"{
            "name": "get_analytics_report_request",
            "description": "Get details of an analytics report request",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "request_id": {
                        "type": "string",
                        "description": "The analytics report request ID"
                    }
                },
                "required": [
                    "request_id"
                ]
            },
            "annotations": {
                "title": "Get Analytics Report Request",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_analytics_report_request),
    );
    r.register(
        tool(r##"{
            "name": "create_analytics_report_request",
            "description": "Create a new analytics report request",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "access_type": {
                        "type": "string",
                        "description": "Access type (ONE_TIME_SNAPSHOT, ONGOING)"
                    },
                    "app_id": {
                        "type": "string",
                        "description": "The App ID"
                    }
                },
                "required": [
                    "app_id",
                    "access_type"
                ]
            },
            "annotations": {
                "title": "Create Analytics Report Request",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_analytics_report_request),
    );
    r.register(
        tool(r##"{
            "name": "delete_analytics_report_request",
            "description": "Delete an analytics report request",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "request_id": {
                        "type": "string",
                        "description": "The analytics report request ID"
                    }
                },
                "required": [
                    "request_id"
                ]
            },
            "annotations": {
                "title": "Delete Analytics Report Request",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_analytics_report_request),
    );
    r.register(
        tool(r##"{
            "name": "list_analytics_reports",
            "description": "List analytics reports for a report request",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"platform\": [\"IOS\"]} becomes filter[platform]=IOS."
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response.",
                        "items": {
                            "type": "string"
                        }
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of reports to return (default 50)"
                    },
                    "request_id": {
                        "type": "string",
                        "description": "The analytics report request ID"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "request_id"
                ]
            },
            "annotations": {
                "title": "List Analytics Reports",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_analytics_reports),
    );
    r.register(
        tool(r##"{
            "name": "list_analytics_report_instances",
            "description": "List instances for an analytics report",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"platform\": [\"IOS\"]} becomes filter[platform]=IOS."
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response.",
                        "items": {
                            "type": "string"
                        }
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of instances to return (default 50)"
                    },
                    "report_id": {
                        "type": "string",
                        "description": "The analytics report ID"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "report_id"
                ]
            },
            "annotations": {
                "title": "List Analytics Report Instances",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_analytics_report_instances),
    );
    r.register(
        tool(r##"{
            "name": "list_analytics_report_segments",
            "description": "List segments for an analytics report instance (contains download URLs)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as Next cursor in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"platform\": [\"IOS\"]} becomes filter[platform]=IOS."
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response.",
                        "items": {
                            "type": "string"
                        }
                    },
                    "instance_id": {
                        "type": "string",
                        "description": "The analytics report instance ID"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of segments to return (default 50)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "instance_id"
                ]
            },
            "annotations": {
                "title": "List Analytics Report Segments",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_analytics_report_segments),
    );
}

/// Ported from `Registry.handleListAnalyticsReportRequests`.
async fn handle_list_analytics_report_requests(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }

    let limit = clamp_limit(params.limit);

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_analytics_report_requests(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list analytics report requests: {e}"))),
    };

    Ok(list_result(&format_analytics_report_requests(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetAnalyticsReportRequest`.
async fn handle_get_analytics_report_request(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { RequestID string \"json:\\\"request_id\\\"\" }", "") {
        request_id: String => "request_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.request_id.is_empty() {
        return Ok(error_result("request_id is required"));
    }

    let resp = match c.get_analytics_report_request(&params.request_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get analytics report request: {e}"))),
    };

    Ok(data_result(&format_analytics_report_request(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateAnalyticsReportRequest`.
async fn handle_create_analytics_report_request(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; AccessType string \"json:\\\"access_type\\\"\" }", "") {
        app_id: String => "app_id";
        access_type: String => "access_type";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }
    if params.access_type.is_empty() {
        return Ok(error_result("access_type is required"));
    }

    let req = AnalyticsReportRequestCreateRequest {
        data: AnalyticsReportRequestCreateData {
            type_: "analyticsReportRequests".into(),
            attributes: AnalyticsReportRequestCreateAttributes { access_type: params.access_type.clone() },
            relationships: AnalyticsReportRequestCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
            },
        },
    };

    let resp = match c.create_analytics_report_request(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create analytics report request: {e}"))),
    };

    Ok(data_result(&format!("Created analytics report request: {}", resp.data.id), &resp.data))
}

/// Ported from `Registry.handleDeleteAnalyticsReportRequest`.
async fn handle_delete_analytics_report_request(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { RequestID string \"json:\\\"request_id\\\"\" }", "") {
        request_id: String => "request_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.request_id.is_empty() {
        return Ok(error_result("request_id is required"));
    }

    if let Err(e) = c.delete_analytics_report_request(&params.request_id).await {
        return Ok(error_result(format!("Failed to delete analytics report request: {e}")));
    }

    Ok(CallToolResult::success("Analytics report request deleted successfully"))
}

/// Ported from `Registry.handleListAnalyticsReports`.
async fn handle_list_analytics_reports(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { RequestID string \"json:\\\"request_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        request_id: String => "request_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.request_id.is_empty() {
        return Ok(error_result("request_id is required"));
    }

    let limit = clamp_limit(params.limit);

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_analytics_reports(&params.request_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list analytics reports: {e}"))),
    };

    Ok(list_result(&format_analytics_reports(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleListAnalyticsReportInstances`.
async fn handle_list_analytics_report_instances(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ReportID string \"json:\\\"report_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        report_id: String => "report_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.report_id.is_empty() {
        return Ok(error_result("report_id is required"));
    }

    let limit = clamp_limit(params.limit);

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_analytics_report_instances(&params.report_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list analytics report instances: {e}"))),
    };

    Ok(list_result(&format_analytics_report_instances(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleListAnalyticsReportSegments`.
async fn handle_list_analytics_report_segments(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { InstanceID string \"json:\\\"instance_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        instance_id: String => "instance_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.instance_id.is_empty() {
        return Ok(error_result("instance_id is required"));
    }

    let limit = clamp_limit(params.limit);

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_analytics_report_segments(&params.instance_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list analytics report segments: {e}"))),
    };

    Ok(list_result(&format_analytics_report_segments(&resp.data), &resp.data, &resp.links))
}

/// The `limit <= 0 → 50; limit > 200 → 200` clamp the Go handlers inline.
fn clamp_limit(limit: i64) -> i64 {
    let mut limit = limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }
    limit
}

/// Ported from `formatAnalyticsReportRequests`.
fn format_analytics_report_requests(requests: &[AnalyticsReportRequest]) -> String {
    if requests.is_empty() {
        return "No analytics report requests found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} analytics report requests:\n\n", requests.len());

    for req in requests {
        sb.push_str(&format_analytics_report_request(req));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatAnalyticsReportRequest`.
fn format_analytics_report_request(req: &AnalyticsReportRequest) -> String {
    let mut sb = String::new();
    let _ = write!(sb, "ID: {}\n", req.id);
    let _ = write!(sb, "Access Type: {}\n", req.attributes.access_type);
    let _ = write!(sb, "Stoppable: {}\n", req.attributes.stoppable);
    sb
}

/// Ported from `formatAnalyticsReports`.
fn format_analytics_reports(reports: &[AnalyticsReport]) -> String {
    if reports.is_empty() {
        return "No analytics reports found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} analytics reports:\n\n", reports.len());

    for report in reports {
        let _ = write!(sb, "ID: {}\n", report.id);
        let _ = write!(sb, "Category: {}\n", report.attributes.category);
        let _ = write!(sb, "Name: {}\n", report.attributes.name);
        sb.push_str("---\n");
    }

    sb
}

/// Ported from `formatAnalyticsReportInstances`.
fn format_analytics_report_instances(instances: &[AnalyticsReportInstance]) -> String {
    if instances.is_empty() {
        return "No analytics report instances found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} analytics report instances:\n\n", instances.len());

    for instance in instances {
        let _ = write!(sb, "ID: {}\n", instance.id);
        let _ = write!(sb, "Granularity: {}\n", instance.attributes.granularity);
        let _ = write!(sb, "Processing Date: {}\n", instance.attributes.processing_date);
        sb.push_str("---\n");
    }

    sb
}

/// Ported from `formatAnalyticsReportSegments`.
fn format_analytics_report_segments(segments: &[AnalyticsReportSegment]) -> String {
    if segments.is_empty() {
        return "No analytics report segments found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} analytics report segments:\n\n", segments.len());

    for segment in segments {
        let _ = write!(sb, "ID: {}\n", segment.id);
        let _ = write!(sb, "Size: {} bytes\n", segment.attributes.size_in_bytes);
        let _ = write!(sb, "Checksum: {}\n", segment.attributes.checksum);
        if !segment.attributes.url.is_empty() {
            let _ = write!(sb, "Download URL: {}\n", segment.attributes.url);
        }
        sb.push_str("---\n");
    }

    sb
}
