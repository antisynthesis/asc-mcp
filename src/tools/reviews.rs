//! Ported from internal/asc/tools/reviews.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerCustomerReviewTools`.
pub(super) fn register_customer_review_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_customer_reviews",
            "description": "List customer reviews for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to list reviews for"
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
                        "description": "Maximum number of reviews to return (default 50)"
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
                "title": "List Customer Reviews",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_customer_reviews),
    );
    r.register(
        tool(r##"{
            "name": "get_customer_review",
            "description": "Get details of a specific customer review",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "review_id": {
                        "type": "string",
                        "description": "The customer review ID"
                    }
                },
                "required": [
                    "review_id"
                ]
            },
            "annotations": {
                "title": "Get Customer Review",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_customer_review),
    );
    r.register(
        tool(r##"{
            "name": "create_customer_review_response",
            "description": "Create a response to a customer review",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "response_body": {
                        "type": "string",
                        "description": "The response text"
                    },
                    "review_id": {
                        "type": "string",
                        "description": "The customer review ID to respond to"
                    }
                },
                "required": [
                    "review_id",
                    "response_body"
                ]
            },
            "annotations": {
                "title": "Create Customer Review Response",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_customer_review_response),
    );
    r.register(
        tool(r##"{
            "name": "delete_customer_review_response",
            "description": "Delete a response to a customer review",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "response_id": {
                        "type": "string",
                        "description": "The customer review response ID"
                    }
                },
                "required": [
                    "response_id"
                ]
            },
            "annotations": {
                "title": "Delete Customer Review Response",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_customer_review_response),
    );
    r.register(
        tool(r##"{
            "name": "list_customer_review_summarizations",
            "description": "List Apple-generated summaries of an app's customer reviews per platform, locale, and territory",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to list review summarizations for"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Supported keys: platform (IOS, MAC_OS, TV_OS, VISION_OS), territory. Values are arrays, e.g. {\"platform\": [\"IOS\"], \"territory\": [\"USA\"]}."
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response (supported: territory).",
                        "items": {
                            "type": "string"
                        }
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of summarizations to return (default 50, max 200)"
                    }
                },
                "required": [
                    "app_id"
                ]
            },
            "annotations": {
                "title": "List Customer Review Summarizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_customer_review_summarizations),
    );
}

/// Ported from `Registry.handleListCustomerReviews`.
async fn handle_list_customer_reviews(c: Arc<Client>, args: Args) -> HandlerResult {
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
    todo!("port handleListCustomerReviews from internal/asc/tools/reviews.go")
}

/// Ported from `Registry.handleGetCustomerReview`.
async fn handle_get_customer_review(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ReviewID string \"json:\\\"review_id\\\"\" }", "") {
        review_id: String => "review_id";
    }
    }
    todo!("port handleGetCustomerReview from internal/asc/tools/reviews.go")
}

/// Ported from `Registry.handleCreateCustomerReviewResponse`.
async fn handle_create_customer_review_response(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ReviewID string \"json:\\\"review_id\\\"\"; ResponseBody string \"json:\\\"response_body\\\"\" }", "") {
        review_id: String => "review_id";
        response_body: String => "response_body";
    }
    }
    todo!("port handleCreateCustomerReviewResponse from internal/asc/tools/reviews.go")
}

/// Ported from `Registry.handleDeleteCustomerReviewResponse`.
async fn handle_delete_customer_review_response(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ResponseID string \"json:\\\"response_id\\\"\" }", "") {
        response_id: String => "response_id";
    }
    }
    todo!("port handleDeleteCustomerReviewResponse from internal/asc/tools/reviews.go")
}

/// Ported from `Registry.handleListCustomerReviewSummarizations`.
async fn handle_list_customer_review_summarizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListCustomerReviewSummarizations from internal/asc/tools/reviews.go")
}

