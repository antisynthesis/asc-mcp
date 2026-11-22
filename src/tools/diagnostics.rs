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
    todo!("port handleListPerfPowerMetrics from internal/asc/tools/diagnostics.go")
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
    todo!("port handleListDiagnosticSignatures from internal/asc/tools/diagnostics.go")
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
    todo!("port handleListDiagnosticLogs from internal/asc/tools/diagnostics.go")
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
    todo!("port handleListAppStoreReviewAttachments from internal/asc/tools/diagnostics.go")
}

/// Ported from `Registry.handleGetAppStoreReviewAttachment`.
async fn handle_get_app_store_review_attachment(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AttachmentID string \"json:\\\"attachment_id\\\"\" }", "") {
        attachment_id: String => "attachment_id";
    }
    }
    todo!("port handleGetAppStoreReviewAttachment from internal/asc/tools/diagnostics.go")
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
    todo!("port handleCreateAppStoreReviewAttachment from internal/asc/tools/diagnostics.go")
}

/// Ported from `Registry.handleDeleteAppStoreReviewAttachment`.
async fn handle_delete_app_store_review_attachment(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AttachmentID string \"json:\\\"attachment_id\\\"\" }", "") {
        attachment_id: String => "attachment_id";
    }
    }
    todo!("port handleDeleteAppStoreReviewAttachment from internal/asc/tools/diagnostics.go")
}

/// Ported from `Registry.handleGetRoutingAppCoverage`.
async fn handle_get_routing_app_coverage(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    todo!("port handleGetRoutingAppCoverage from internal/asc/tools/diagnostics.go")
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
    todo!("port handleCreateRoutingAppCoverage from internal/asc/tools/diagnostics.go")
}

/// Ported from `Registry.handleDeleteRoutingAppCoverage`.
async fn handle_delete_routing_app_coverage(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { CoverageID string \"json:\\\"coverage_id\\\"\" }", "") {
        coverage_id: String => "coverage_id";
    }
    }
    todo!("port handleDeleteRoutingAppCoverage from internal/asc/tools/diagnostics.go")
}

