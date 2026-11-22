//! Ported from internal/asc/tools/product_pages.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerProductPagesTools`.
pub(super) fn register_product_pages_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_app_custom_product_pages",
            "description": "List custom product pages for an app",
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
                        "description": "Maximum number of pages to return (default 50)"
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
                "title": "List App Custom Product Pages",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_app_custom_product_pages),
    );
    r.register(
        tool(r##"{
            "name": "get_app_custom_product_page",
            "description": "Get details of a specific custom product page",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "page_id": {
                        "type": "string",
                        "description": "The custom product page ID"
                    }
                },
                "required": [
                    "page_id"
                ]
            },
            "annotations": {
                "title": "Get App Custom Product Page",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_custom_product_page),
    );
    r.register(
        tool(r##"{
            "name": "create_app_custom_product_page",
            "description": "Create a new custom product page",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The app ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "Name of the custom product page"
                    }
                },
                "required": [
                    "app_id",
                    "name"
                ]
            },
            "annotations": {
                "title": "Create App Custom Product Page",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_app_custom_product_page),
    );
    r.register(
        tool(r##"{
            "name": "update_app_custom_product_page",
            "description": "Update a custom product page",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "New name for the page"
                    },
                    "page_id": {
                        "type": "string",
                        "description": "The custom product page ID"
                    },
                    "visible": {
                        "type": "boolean",
                        "description": "Whether the page is visible"
                    }
                },
                "required": [
                    "page_id"
                ]
            },
            "annotations": {
                "title": "Update App Custom Product Page",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_app_custom_product_page),
    );
    r.register(
        tool(r##"{
            "name": "delete_app_custom_product_page",
            "description": "Delete a custom product page",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "page_id": {
                        "type": "string",
                        "description": "The custom product page ID to delete"
                    }
                },
                "required": [
                    "page_id"
                ]
            },
            "annotations": {
                "title": "Delete App Custom Product Page",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_app_custom_product_page),
    );
    r.register(
        tool(r##"{
            "name": "list_app_store_version_experiments",
            "description": "List A/B testing experiments for an app or an app store version. Provide app_id or version_id.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to list experiments for (preferred; v2 experiments attach to an app)"
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
                        "description": "Maximum number of experiments to return (default 50)"
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
                        "description": "The app store version ID to list experiments for"
                    }
                }
            },
            "annotations": {
                "title": "List App Store Version Experiments",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_app_store_version_experiments),
    );
    r.register(
        tool(r##"{
            "name": "get_app_store_version_experiment",
            "description": "Get details of a specific experiment",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "experiment_id": {
                        "type": "string",
                        "description": "The experiment ID"
                    }
                },
                "required": [
                    "experiment_id"
                ]
            },
            "annotations": {
                "title": "Get App Store Version Experiment",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_app_store_version_experiment),
    );
    r.register(
        tool(r##"{
            "name": "create_app_store_version_experiment",
            "description": "Create a new A/B testing experiment for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID (v2 experiments attach to an app, not a version)"
                    },
                    "name": {
                        "type": "string",
                        "description": "Name of the experiment"
                    },
                    "platform": {
                        "type": "string",
                        "description": "Platform for the experiment (IOS, MAC_OS, TV_OS, VISION_OS)"
                    },
                    "traffic_proportion": {
                        "type": "integer",
                        "description": "Percentage of traffic for the experiment (1-100)"
                    }
                },
                "required": [
                    "app_id",
                    "name",
                    "platform"
                ]
            },
            "annotations": {
                "title": "Create App Store Version Experiment",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_app_store_version_experiment),
    );
    r.register(
        tool(r##"{
            "name": "update_app_store_version_experiment",
            "description": "Update an experiment",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "experiment_id": {
                        "type": "string",
                        "description": "The experiment ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "New name for the experiment"
                    },
                    "started": {
                        "type": "boolean",
                        "description": "Whether the experiment is running"
                    },
                    "traffic_proportion": {
                        "type": "integer",
                        "description": "Percentage of traffic for the experiment (1-100)"
                    }
                },
                "required": [
                    "experiment_id"
                ]
            },
            "annotations": {
                "title": "Update App Store Version Experiment",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_app_store_version_experiment),
    );
    r.register(
        tool(r##"{
            "name": "delete_app_store_version_experiment",
            "description": "Delete an experiment",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "experiment_id": {
                        "type": "string",
                        "description": "The experiment ID to delete"
                    }
                },
                "required": [
                    "experiment_id"
                ]
            },
            "annotations": {
                "title": "Delete App Store Version Experiment",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_app_store_version_experiment),
    );
}

/// Ported from `Registry.handleListAppCustomProductPages`.
async fn handle_list_app_custom_product_pages(c: Arc<Client>, args: Args) -> HandlerResult {
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
    todo!("port handleListAppCustomProductPages from internal/asc/tools/product_pages.go")
}

/// Ported from `Registry.handleGetAppCustomProductPage`.
async fn handle_get_app_custom_product_page(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PageID string \"json:\\\"page_id\\\"\" }", "") {
        page_id: String => "page_id";
    }
    }
    todo!("port handleGetAppCustomProductPage from internal/asc/tools/product_pages.go")
}

/// Ported from `Registry.handleCreateAppCustomProductPage`.
async fn handle_create_app_custom_product_page(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Name string \"json:\\\"name\\\"\" }", "") {
        app_id: String => "app_id";
        name: String => "name";
    }
    }
    todo!("port handleCreateAppCustomProductPage from internal/asc/tools/product_pages.go")
}

/// Ported from `Registry.handleUpdateAppCustomProductPage`.
async fn handle_update_app_custom_product_page(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PageID string \"json:\\\"page_id\\\"\"; Name string \"json:\\\"name\\\"\"; Visible *bool \"json:\\\"visible\\\"\" }", "") {
        page_id: String => "page_id";
        name: String => "name";
        visible: Option<bool> => "visible";
    }
    }
    todo!("port handleUpdateAppCustomProductPage from internal/asc/tools/product_pages.go")
}

/// Ported from `Registry.handleDeleteAppCustomProductPage`.
async fn handle_delete_app_custom_product_page(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PageID string \"json:\\\"page_id\\\"\" }", "") {
        page_id: String => "page_id";
    }
    }
    todo!("port handleDeleteAppCustomProductPage from internal/asc/tools/product_pages.go")
}

/// Ported from `Registry.handleListAppStoreVersionExperiments`.
async fn handle_list_app_store_version_experiments(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; VersionID string \"json:\\\"version_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_id: String => "app_id";
        version_id: String => "version_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListAppStoreVersionExperiments from internal/asc/tools/product_pages.go")
}

/// Ported from `Registry.handleGetAppStoreVersionExperiment`.
async fn handle_get_app_store_version_experiment(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ExperimentID string \"json:\\\"experiment_id\\\"\" }", "") {
        experiment_id: String => "experiment_id";
    }
    }
    todo!("port handleGetAppStoreVersionExperiment from internal/asc/tools/product_pages.go")
}

/// Ported from `Registry.handleCreateAppStoreVersionExperiment`.
async fn handle_create_app_store_version_experiment(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Name string \"json:\\\"name\\\"\"; Platform string \"json:\\\"platform\\\"\"; TrafficProportion int \"json:\\\"traffic_proportion\\\"\" }", "") {
        app_id: String => "app_id";
        name: String => "name";
        platform: String => "platform";
        traffic_proportion: i64 => "traffic_proportion";
    }
    }
    todo!("port handleCreateAppStoreVersionExperiment from internal/asc/tools/product_pages.go")
}

/// Ported from `Registry.handleUpdateAppStoreVersionExperiment`.
async fn handle_update_app_store_version_experiment(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ExperimentID string \"json:\\\"experiment_id\\\"\"; Name string \"json:\\\"name\\\"\"; TrafficProportion *int \"json:\\\"traffic_proportion\\\"\"; Started *bool \"json:\\\"started\\\"\" }", "") {
        experiment_id: String => "experiment_id";
        name: String => "name";
        traffic_proportion: Option<i64> => "traffic_proportion";
        started: Option<bool> => "started";
    }
    }
    todo!("port handleUpdateAppStoreVersionExperiment from internal/asc/tools/product_pages.go")
}

/// Ported from `Registry.handleDeleteAppStoreVersionExperiment`.
async fn handle_delete_app_store_version_experiment(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ExperimentID string \"json:\\\"experiment_id\\\"\" }", "") {
        experiment_id: String => "experiment_id";
    }
    }
    todo!("port handleDeleteAppStoreVersionExperiment from internal/asc/tools/product_pages.go")
}

