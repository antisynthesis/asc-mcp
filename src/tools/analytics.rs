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
    todo!("port handleListAnalyticsReportRequests from internal/asc/tools/analytics.go")
}

/// Ported from `Registry.handleGetAnalyticsReportRequest`.
async fn handle_get_analytics_report_request(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { RequestID string \"json:\\\"request_id\\\"\" }", "") {
        request_id: String => "request_id";
    }
    }
    todo!("port handleGetAnalyticsReportRequest from internal/asc/tools/analytics.go")
}

/// Ported from `Registry.handleCreateAnalyticsReportRequest`.
async fn handle_create_analytics_report_request(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; AccessType string \"json:\\\"access_type\\\"\" }", "") {
        app_id: String => "app_id";
        access_type: String => "access_type";
    }
    }
    todo!("port handleCreateAnalyticsReportRequest from internal/asc/tools/analytics.go")
}

/// Ported from `Registry.handleDeleteAnalyticsReportRequest`.
async fn handle_delete_analytics_report_request(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { RequestID string \"json:\\\"request_id\\\"\" }", "") {
        request_id: String => "request_id";
    }
    }
    todo!("port handleDeleteAnalyticsReportRequest from internal/asc/tools/analytics.go")
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
    todo!("port handleListAnalyticsReports from internal/asc/tools/analytics.go")
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
    todo!("port handleListAnalyticsReportInstances from internal/asc/tools/analytics.go")
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
    todo!("port handleListAnalyticsReportSegments from internal/asc/tools/analytics.go")
}

