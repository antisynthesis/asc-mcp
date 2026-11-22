//! Ported from internal/asc/tools/diagnostics.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerDiagnosticsTools`.
pub(super) fn register_diagnostics_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_perf_power_metrics",
            "description": "List performance and power metrics for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The app ID"
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
                        "description": "Maximum number of metrics to return (default 50)"
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
                "title": "List Perf Power Metrics",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_perf_power_metrics),
    );
    r.register(
        tool(r##"{
            "name": "list_diagnostic_signatures",
            "description": "List diagnostic signatures (crash/energy/disk reports) for a build",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "build_id": {
                        "type": "string",
                        "description": "The build ID"
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
                        "description": "Maximum number of signatures to return (default 50)"
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
                    "build_id"
                ]
            },
            "annotations": {
                "title": "List Diagnostic Signatures",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_diagnostic_signatures),
    );
    r.register(
        tool(r##"{
            "name": "list_diagnostic_logs",
            "description": "List diagnostic logs for a signature",
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
                        "description": "Maximum number of logs to return (default 50)"
                    },
                    "signature_id": {
                        "type": "string",
                        "description": "The diagnostic signature ID"
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
                    "signature_id"
                ]
            },
            "annotations": {
                "title": "List Diagnostic Logs",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_diagnostic_logs),
    );
    r.register(
        tool(r##"{
            "name": "list_app_store_review_attachments",
            "description": "List App Store review submission attachments for a version",
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
                        "description": "Maximum number of attachments to return (default 50)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The app store version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "List App Store Review Attachments",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_app_store_review_attachments),
    );
    r.register(
        tool(r##"{
            "name": "get_app_store_review_attachment",
            "description": "Get details of a review attachment",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "attachment_id": {
                        "type": "string",
                        "description": "The attachment ID"
                    }
                },
                "required": [
                    "attachment_id"
                ]
            },
            "annotations": {
                "title": "Get App Store Review Attachment",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_store_review_attachment),
    );
    r.register(
        tool(r##"{
            "name": "create_app_store_review_attachment",
            "description": "Create a new review attachment for submission",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "file_name": {
                        "type": "string",
                        "description": "Name of the file"
                    },
                    "file_size": {
                        "type": "integer",
                        "description": "Size of the file in bytes"
                    },
                    "review_detail_id": {
                        "type": "string",
                        "description": "The app store review detail ID"
                    }
                },
                "required": [
                    "review_detail_id",
                    "file_name",
                    "file_size"
                ]
            },
            "annotations": {
                "title": "Create App Store Review Attachment",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_app_store_review_attachment),
    );
    r.register(
        tool(r##"{
            "name": "delete_app_store_review_attachment",
            "description": "Delete a review attachment",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "attachment_id": {
                        "type": "string",
                        "description": "The attachment ID to delete"
                    }
                },
                "required": [
                    "attachment_id"
                ]
            },
            "annotations": {
                "title": "Delete App Store Review Attachment",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_app_store_review_attachment),
    );
    r.register(
        tool(r##"{
            "name": "get_routing_app_coverage",
            "description": "Get routing app coverage file information for a version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "version_id": {
                        "type": "string",
                        "description": "The app store version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Get Routing App Coverage",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_routing_app_coverage),
    );
    r.register(
        tool(r##"{
            "name": "create_routing_app_coverage",
            "description": "Create a routing app coverage file upload reservation",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "file_name": {
                        "type": "string",
                        "description": "Name of the GeoJSON file"
                    },
                    "file_size": {
                        "type": "integer",
                        "description": "Size of the file in bytes"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The app store version ID"
                    }
                },
                "required": [
                    "version_id",
                    "file_name",
                    "file_size"
                ]
            },
            "annotations": {
                "title": "Create Routing App Coverage",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_routing_app_coverage),
    );
    r.register(
        tool(r##"{
            "name": "delete_routing_app_coverage",
            "description": "Delete a routing app coverage file",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "coverage_id": {
                        "type": "string",
                        "description": "The routing app coverage ID to delete"
                    }
                },
                "required": [
                    "coverage_id"
                ]
            },
            "annotations": {
                "title": "Delete Routing App Coverage",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_routing_app_coverage),
    );
}

/// Ported from `Registry.handleListPerfPowerMetrics`.
async fn handle_list_perf_power_metrics(c: Arc<Client>, args: Args) -> HandlerResult {
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
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_perf_power_metrics(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list performance metrics: {e}"))),
    };

    Ok(list_result(&format_perf_power_metrics(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleListDiagnosticSignatures`.
async fn handle_list_diagnostic_signatures(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildID string \"json:\\\"build_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        build_id: String => "build_id";
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

    if params.build_id.is_empty() {
        return Ok(error_result("build_id is required"));
    }

    let limit = clamp_limit(params.limit);

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_diagnostic_signatures(&params.build_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list diagnostic signatures: {e}"))),
    };

    Ok(list_result(&format_diagnostic_signatures(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleListDiagnosticLogs`.
async fn handle_list_diagnostic_logs(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SignatureID string \"json:\\\"signature_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        signature_id: String => "signature_id";
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

    if params.signature_id.is_empty() {
        return Ok(error_result("signature_id is required"));
    }

    let limit = clamp_limit(params.limit);

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_diagnostic_logs(&params.signature_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list diagnostic logs: {e}"))),
    };

    Ok(list_result(&format_diagnostic_logs(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleListAppStoreReviewAttachments`.
async fn handle_list_app_store_review_attachments(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        version_id: String => "version_id";
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

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    let limit = clamp_limit(params.limit);

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_store_review_attachments(&params.version_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list review attachments: {e}"))),
    };

    Ok(list_result(&format_app_store_review_attachments(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetAppStoreReviewAttachment`.
async fn handle_get_app_store_review_attachment(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AttachmentID string \"json:\\\"attachment_id\\\"\" }", "") {
        attachment_id: String => "attachment_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.attachment_id.is_empty() {
        return Ok(error_result("attachment_id is required"));
    }

    let resp = match c.get_app_store_review_attachment(&params.attachment_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get review attachment: {e}"))),
    };

    Ok(data_result(&format_app_store_review_attachment(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateAppStoreReviewAttachment`.
async fn handle_create_app_store_review_attachment(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ReviewDetailID string \"json:\\\"review_detail_id\\\"\"; FileName string \"json:\\\"file_name\\\"\"; FileSize int \"json:\\\"file_size\\\"\" }", "") {
        review_detail_id: String => "review_detail_id";
        file_name: String => "file_name";
        file_size: i64 => "file_size";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.review_detail_id.is_empty() || params.file_name.is_empty() || params.file_size <= 0 {
        return Ok(error_result("review_detail_id, file_name, and file_size are required"));
    }

    let req = AppStoreReviewAttachmentCreateRequest {
        data: AppStoreReviewAttachmentCreateData {
            type_: "appStoreReviewAttachments".into(),
            attributes: AppStoreReviewAttachmentCreateAttributes { file_name: params.file_name.clone(), file_size: params.file_size },
            relationships: AppStoreReviewAttachmentCreateRelationships {
                app_store_review_detail: RelationshipData {
                    data: ResourceIdentifier { type_: "appStoreReviewDetails".into(), id: params.review_detail_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_app_store_review_attachment(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create review attachment: {e}"))),
    };

    Ok(data_result(&format!("Review attachment reservation created:\n{}", format_app_store_review_attachment(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleDeleteAppStoreReviewAttachment`.
async fn handle_delete_app_store_review_attachment(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AttachmentID string \"json:\\\"attachment_id\\\"\" }", "") {
        attachment_id: String => "attachment_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.attachment_id.is_empty() {
        return Ok(error_result("attachment_id is required"));
    }

    if let Err(e) = c.delete_app_store_review_attachment(&params.attachment_id).await {
        return Ok(error_result(format!("Failed to delete review attachment: {e}")));
    }

    Ok(CallToolResult::success("Review attachment deleted"))
}

/// Ported from `Registry.handleGetRoutingAppCoverage`.
async fn handle_get_routing_app_coverage(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    let resp = match c.get_routing_app_coverage(&params.version_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get routing app coverage: {e}"))),
    };

    Ok(data_result(&format_routing_app_coverage(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateRoutingAppCoverage`.
async fn handle_create_routing_app_coverage(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; FileName string \"json:\\\"file_name\\\"\"; FileSize int \"json:\\\"file_size\\\"\" }", "") {
        version_id: String => "version_id";
        file_name: String => "file_name";
        file_size: i64 => "file_size";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() || params.file_name.is_empty() || params.file_size <= 0 {
        return Ok(error_result("version_id, file_name, and file_size are required"));
    }

    let req = RoutingAppCoverageCreateRequest {
        data: RoutingAppCoverageCreateData {
            type_: "routingAppCoverages".into(),
            attributes: RoutingAppCoverageCreateAttributes { file_name: params.file_name.clone(), file_size: params.file_size },
            relationships: RoutingAppCoverageCreateRelationships {
                app_store_version: RelationshipData {
                    data: ResourceIdentifier { type_: "appStoreVersions".into(), id: params.version_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_routing_app_coverage(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create routing app coverage: {e}"))),
    };

    Ok(data_result(&format!("Routing app coverage reservation created:\n{}", format_routing_app_coverage(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleDeleteRoutingAppCoverage`.
async fn handle_delete_routing_app_coverage(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { CoverageID string \"json:\\\"coverage_id\\\"\" }", "") {
        coverage_id: String => "coverage_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.coverage_id.is_empty() {
        return Ok(error_result("coverage_id is required"));
    }

    if let Err(e) = c.delete_routing_app_coverage(&params.coverage_id).await {
        return Ok(error_result(format!("Failed to delete routing app coverage: {e}")));
    }

    Ok(CallToolResult::success("Routing app coverage deleted"))
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

/// Ported from `formatPerfPowerMetrics`.
fn format_perf_power_metrics(metrics: &[PerfPowerMetric]) -> String {
    if metrics.is_empty() {
        return "No performance metrics found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} performance metrics:\n\n", metrics.len());

    for m in metrics {
        let _ = writeln!(sb, "ID: {}", m.id);
        let _ = writeln!(sb, "Device Type: {}", m.attributes.device_type);
        let _ = writeln!(sb, "Metric Type: {}", m.attributes.metric_type);
        let _ = writeln!(sb, "Platform: {}", m.attributes.platform);
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatDiagnosticSignatures`.
fn format_diagnostic_signatures(signatures: &[DiagnosticSignature]) -> String {
    if signatures.is_empty() {
        return "No diagnostic signatures found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} diagnostic signatures:\n\n", signatures.len());

    for s in signatures {
        let _ = writeln!(sb, "ID: {}", s.id);
        let _ = writeln!(sb, "Diagnostic Type: {}", s.attributes.diagnostic_type);
        let _ = writeln!(sb, "Signature: {}", s.attributes.signature);
        let _ = writeln!(sb, "Weight: {:.2}", s.attributes.weight);
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatDiagnosticLogs`.
fn format_diagnostic_logs(logs: &[DiagnosticLog]) -> String {
    if logs.is_empty() {
        return "No diagnostic logs found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} diagnostic logs:\n\n", logs.len());

    for l in logs {
        let _ = writeln!(sb, "ID: {}", l.id);
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatAppStoreReviewAttachments`.
fn format_app_store_review_attachments(attachments: &[AppStoreReviewAttachment]) -> String {
    if attachments.is_empty() {
        return "No review attachments found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} review attachments:\n\n", attachments.len());

    for a in attachments {
        sb.push_str(&format_app_store_review_attachment(a));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatAppStoreReviewAttachment`.
fn format_app_store_review_attachment(a: &AppStoreReviewAttachment) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", a.id);
    let _ = writeln!(sb, "File Name: {}", a.attributes.file_name);
    let _ = writeln!(sb, "File Size: {} bytes", a.attributes.file_size);
    if let Some(st) = &a.attributes.asset_delivery_state {
        let _ = writeln!(sb, "State: {}", st.state);
    }
    sb
}

/// Ported from `formatRoutingAppCoverage`.
fn format_routing_app_coverage(coverage: &RoutingAppCoverage) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", coverage.id);
    let _ = writeln!(sb, "File Name: {}", coverage.attributes.file_name);
    let _ = writeln!(sb, "File Size: {} bytes", coverage.attributes.file_size);
    if let Some(st) = &coverage.attributes.asset_delivery_state {
        let _ = writeln!(sb, "State: {}", st.state);
    }
    sb
}
