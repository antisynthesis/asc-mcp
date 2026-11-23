//! Xcode Cloud tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
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

/// Handles `list_ci_products`.
async fn handle_list_ci_products(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_ci_products(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list CI products: {e}"))),
    };

    Ok(list_result(&format_ci_products(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_ci_product`.
async fn handle_get_ci_product(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { ProductID string \"json:\\\"product_id\\\"\" }", "") {
        product_id: String => "product_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.product_id.is_empty() {
        return Ok(error_result("product_id is required"));
    }

    let resp = match c.get_ci_product(&params.product_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get CI product: {e}"))),
    };

    Ok(data_result(&format_ci_product(&resp.data), &resp.data))
}

/// Handles `list_ci_workflows`.
async fn handle_list_ci_workflows(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { ProductID string \"json:\\\"product_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        product_id: String => "product_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.product_id.is_empty() {
        return Ok(error_result("product_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_ci_workflows(&params.product_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list CI workflows: {e}"))),
    };

    Ok(list_result(&format_ci_workflows(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_ci_workflow`.
async fn handle_get_ci_workflow(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { WorkflowID string \"json:\\\"workflow_id\\\"\" }", "") {
        workflow_id: String => "workflow_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.workflow_id.is_empty() {
        return Ok(error_result("workflow_id is required"));
    }

    let resp = match c.get_ci_workflow(&params.workflow_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get CI workflow: {e}"))),
    };

    Ok(data_result(&format_ci_workflow(&resp.data), &resp.data))
}

/// Handles `list_ci_build_runs`.
async fn handle_list_ci_build_runs(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { WorkflowID string \"json:\\\"workflow_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        workflow_id: String => "workflow_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.workflow_id.is_empty() {
        return Ok(error_result("workflow_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_ci_build_runs(&params.workflow_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list CI build runs: {e}"))),
    };

    Ok(list_result(&format_ci_build_runs(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_ci_build_run`.
async fn handle_get_ci_build_run(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { BuildRunID string \"json:\\\"build_run_id\\\"\" }", "") {
        build_run_id: String => "build_run_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.build_run_id.is_empty() {
        return Ok(error_result("build_run_id is required"));
    }

    let resp = match c.get_ci_build_run(&params.build_run_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get CI build run: {e}"))),
    };

    Ok(data_result(&format_ci_build_run(&resp.data), &resp.data))
}

/// Handles `start_ci_build_run`.
async fn handle_start_ci_build_run(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { WorkflowID string \"json:\\\"workflow_id\\\"\" }", "") {
        workflow_id: String => "workflow_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.workflow_id.is_empty() {
        return Ok(error_result("workflow_id is required"));
    }

    let resp = match c.start_ci_build_run(&params.workflow_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to start CI build run: {e}"))),
    };

    Ok(data_result(
        &format!("Started build run: {} (build #{})", resp.data.id, resp.data.attributes.number),
        &resp.data,
    ))
}

fn format_ci_products(products: &[CiProduct]) -> String {
    if products.is_empty() {
        return "No CI products found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} CI products:\n\n", products.len());
    for product in products {
        sb.push_str(&format_ci_product(product));
        sb.push_str("\n---\n");
    }
    sb
}

fn format_ci_product(product: &CiProduct) -> String {
    let a = &product.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", product.id);
    let _ = writeln!(sb, "Name: {}", a.name);
    let _ = writeln!(sb, "Product Type: {}", a.product_type);
    if let Some(d) = &a.created_date {
        let _ = writeln!(sb, "Created: {}", d.format("2006-01-02"));
    }
    sb
}

fn format_ci_workflows(workflows: &[CiWorkflow]) -> String {
    if workflows.is_empty() {
        return "No CI workflows found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} CI workflows:\n\n", workflows.len());
    for workflow in workflows {
        sb.push_str(&format_ci_workflow(workflow));
        sb.push_str("\n---\n");
    }
    sb
}

fn format_ci_workflow(workflow: &CiWorkflow) -> String {
    let a = &workflow.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", workflow.id);
    let _ = writeln!(sb, "Name: {}", a.name);
    if !a.description.is_empty() {
        let _ = writeln!(sb, "Description: {}", a.description);
    }
    let _ = writeln!(sb, "Enabled: {}", a.is_enabled);
    let _ = writeln!(sb, "Clean Build: {}", a.clean);
    if !a.container_file_path.is_empty() {
        let _ = writeln!(sb, "Container File: {}", a.container_file_path);
    }
    if let Some(d) = &a.last_modified_date {
        let _ = writeln!(sb, "Last Modified: {}", d.format("2006-01-02 15:04"));
    }
    sb
}

fn format_ci_build_runs(runs: &[CiBuildRun]) -> String {
    if runs.is_empty() {
        return "No CI build runs found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} CI build runs:\n\n", runs.len());
    for run in runs {
        sb.push_str(&format_ci_build_run(run));
        sb.push_str("\n---\n");
    }
    sb
}

fn format_ci_build_run(run: &CiBuildRun) -> String {
    let a = &run.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", run.id);
    let _ = writeln!(sb, "Build #{}", a.number);
    let _ = writeln!(sb, "Progress: {}", a.execution_progress);
    if !a.completion_status.is_empty() {
        let _ = writeln!(sb, "Status: {}", a.completion_status);
    }
    let _ = writeln!(sb, "Start Reason: {}", a.start_reason);
    let _ = writeln!(sb, "Pull Request Build: {}", a.is_pull_request_build);
    if let Some(sc) = &a.source_commit {
        let _ = writeln!(sb, "Commit: {}", sc.commit_sha);
        if !sc.message.is_empty() {
            let _ = writeln!(sb, "Message: {}", sc.message);
        }
    }
    if let Some(d) = &a.created_date {
        let _ = writeln!(sb, "Created: {}", d.format("2006-01-02 15:04"));
    }
    if let Some(d) = &a.started_date {
        let _ = writeln!(sb, "Started: {}", d.format("2006-01-02 15:04"));
    }
    if let Some(d) = &a.finished_date {
        let _ = writeln!(sb, "Finished: {}", d.format("2006-01-02 15:04"));
    }
    sb
}
