//! Ported from internal/asc/tools/xcode_cloud.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerXcodeCloudTools`.
pub(super) fn register_xcode_cloud_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_ci_products",
            "description": "List Xcode Cloud CI products",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "Filter by App ID (optional)"
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
                        "description": "Maximum number of products to return (default 50)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    }
                }
            },
            "annotations": {
                "title": "List Ci Products",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_ci_products),
    );
    r.register(
        tool(r##"{
            "name": "get_ci_product",
            "description": "Get details of a specific Xcode Cloud product",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "product_id": {
                        "type": "string",
                        "description": "The CI product ID"
                    }
                },
                "required": [
                    "product_id"
                ]
            },
            "annotations": {
                "title": "Get Ci Product",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_ci_product),
    );
    r.register(
        tool(r##"{
            "name": "list_ci_workflows",
            "description": "List Xcode Cloud workflows for a product",
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
                        "description": "Maximum number of workflows to return (default 50)"
                    },
                    "product_id": {
                        "type": "string",
                        "description": "The CI product ID"
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
                    "product_id"
                ]
            },
            "annotations": {
                "title": "List Ci Workflows",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_ci_workflows),
    );
    r.register(
        tool(r##"{
            "name": "get_ci_workflow",
            "description": "Get details of a specific Xcode Cloud workflow",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "workflow_id": {
                        "type": "string",
                        "description": "The CI workflow ID"
                    }
                },
                "required": [
                    "workflow_id"
                ]
            },
            "annotations": {
                "title": "Get Ci Workflow",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_ci_workflow),
    );
    r.register(
        tool(r##"{
            "name": "list_ci_build_runs",
            "description": "List Xcode Cloud build runs for a workflow",
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
                        "description": "Maximum number of build runs to return (default 50)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    },
                    "workflow_id": {
                        "type": "string",
                        "description": "The CI workflow ID"
                    }
                },
                "required": [
                    "workflow_id"
                ]
            },
            "annotations": {
                "title": "List Ci Build Runs",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_ci_build_runs),
    );
    r.register(
        tool(r##"{
            "name": "get_ci_build_run",
            "description": "Get details of a specific Xcode Cloud build run",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "build_run_id": {
                        "type": "string",
                        "description": "The CI build run ID"
                    }
                },
                "required": [
                    "build_run_id"
                ]
            },
            "annotations": {
                "title": "Get Ci Build Run",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_ci_build_run),
    );
    r.register(
        tool(r##"{
            "name": "start_ci_build_run",
            "description": "Start a new Xcode Cloud build run for a workflow",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "workflow_id": {
                        "type": "string",
                        "description": "The CI workflow ID to start"
                    }
                },
                "required": [
                    "workflow_id"
                ]
            },
            "annotations": {
                "title": "Start Ci Build Run",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_start_ci_build_run),
    );
}

/// Ported from `Registry.handleListCiProducts`.
async fn handle_list_ci_products(c: Arc<Client>, args: Args) -> HandlerResult {
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
    todo!("port handleListCiProducts from internal/asc/tools/xcode_cloud.go")
}

/// Ported from `Registry.handleGetCiProduct`.
async fn handle_get_ci_product(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ProductID string \"json:\\\"product_id\\\"\" }", "") {
        product_id: String => "product_id";
    }
    }
    todo!("port handleGetCiProduct from internal/asc/tools/xcode_cloud.go")
}

/// Ported from `Registry.handleListCiWorkflows`.
async fn handle_list_ci_workflows(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ProductID string \"json:\\\"product_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        product_id: String => "product_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListCiWorkflows from internal/asc/tools/xcode_cloud.go")
}

/// Ported from `Registry.handleGetCiWorkflow`.
async fn handle_get_ci_workflow(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { WorkflowID string \"json:\\\"workflow_id\\\"\" }", "") {
        workflow_id: String => "workflow_id";
    }
    }
    todo!("port handleGetCiWorkflow from internal/asc/tools/xcode_cloud.go")
}

/// Ported from `Registry.handleListCiBuildRuns`.
async fn handle_list_ci_build_runs(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { WorkflowID string \"json:\\\"workflow_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        workflow_id: String => "workflow_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListCiBuildRuns from internal/asc/tools/xcode_cloud.go")
}

/// Ported from `Registry.handleGetCiBuildRun`.
async fn handle_get_ci_build_run(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildRunID string \"json:\\\"build_run_id\\\"\" }", "") {
        build_run_id: String => "build_run_id";
    }
    }
    todo!("port handleGetCiBuildRun from internal/asc/tools/xcode_cloud.go")
}

/// Ported from `Registry.handleStartCiBuildRun`.
async fn handle_start_ci_build_run(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { WorkflowID string \"json:\\\"workflow_id\\\"\" }", "") {
        workflow_id: String => "workflow_id";
    }
    }
    todo!("port handleStartCiBuildRun from internal/asc/tools/xcode_cloud.go")
}

