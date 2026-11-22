//! Customer review tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
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

/// Handles `list_customer_reviews`.
async fn handle_list_customer_reviews(c: Arc<Client>, args: Args) -> HandlerResult {
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

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_customer_reviews(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list customer reviews: {e}"))),
    };

    Ok(list_result(&format_customer_reviews(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_customer_review`.
async fn handle_get_customer_review(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { ReviewID string \"json:\\\"review_id\\\"\" }", "") {
        review_id: String => "review_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.review_id.is_empty() {
        return Ok(error_result("review_id is required"));
    }

    let resp = match c.get_customer_review(&params.review_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get customer review: {e}"))),
    };

    Ok(data_result(&format_customer_review(&resp.data), &resp.data))
}

/// Handles `create_customer_review_response`.
async fn handle_create_customer_review_response(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { ReviewID string \"json:\\\"review_id\\\"\"; ResponseBody string \"json:\\\"response_body\\\"\" }", "") {
        review_id: String => "review_id";
        response_body: String => "response_body";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.review_id.is_empty() {
        return Ok(error_result("review_id is required"));
    }
    if params.response_body.is_empty() {
        return Ok(error_result("response_body is required"));
    }

    let req = CustomerReviewResponseCreateRequest {
        data: CustomerReviewResponseCreateData {
            type_: "customerReviewResponses".into(),
            attributes: CustomerReviewResponseCreateAttributes { response_body: params.response_body.clone() },
            relationships: CustomerReviewResponseCreateRelationships {
                review: RelationshipData {
                    data: ResourceIdentifier { type_: "customerReviews".into(), id: params.review_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_customer_review_response(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create review response: {e}"))),
    };

    Ok(data_result(&format!("Created review response: {}", resp.data.id), &resp.data))
}

/// Handles `delete_customer_review_response`.
async fn handle_delete_customer_review_response(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { ResponseID string \"json:\\\"response_id\\\"\" }", "") {
        response_id: String => "response_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.response_id.is_empty() {
        return Ok(error_result("response_id is required"));
    }

    if let Err(e) = c.delete_customer_review_response(&params.response_id).await {
        return Ok(error_result(format!("Failed to delete review response: {e}")));
    }

    Ok(CallToolResult::success("Review response deleted successfully"))
}

/// Handles `list_customer_review_summarizations`.
async fn handle_list_customer_review_summarizations(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &JsonSlice::nil(), &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_customer_review_summarizations(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list customer review summarizations: {e}"))),
    };

    Ok(list_result(&format_customer_review_summarizations(&resp.data), &resp.data, &resp.links))
}

fn format_customer_reviews(reviews: &[CustomerReview]) -> String {
    if reviews.is_empty() {
        return "No customer reviews found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} customer reviews:\n\n", reviews.len());
    for review in reviews {
        sb.push_str(&format_customer_review(review));
        sb.push_str("\n---\n");
    }
    sb
}

fn format_customer_review(review: &CustomerReview) -> String {
    let a = &review.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "Review ID: {}", review.id);
    let _ = writeln!(sb, "Rating: {}/5", a.rating);
    if !a.title.is_empty() {
        let _ = writeln!(sb, "Title: {}", a.title);
    }
    if !a.body.is_empty() {
        let _ = writeln!(sb, "Body: {}", a.body);
    }
    if !a.reviewer_name.is_empty() {
        let _ = writeln!(sb, "Reviewer: {}", a.reviewer_name);
    }
    if !a.territory.is_empty() {
        let _ = writeln!(sb, "Territory: {}", a.territory);
    }
    if let Some(d) = &a.created_date {
        let _ = writeln!(sb, "Created: {}", d.format("2006-01-02"));
    }
    sb
}

fn format_customer_review_summarizations(summaries: &[CustomerReviewSummarization]) -> String {
    if summaries.is_empty() {
        return "No customer review summarizations found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} customer review summarizations:\n\n", summaries.len());
    for summary in summaries {
        let a = &summary.attributes;
        let _ = writeln!(sb, "Summarization ID: {}", summary.id);
        if !a.platform.is_empty() {
            let _ = writeln!(sb, "Platform: {}", a.platform);
        }
        if !a.locale.is_empty() {
            let _ = writeln!(sb, "Locale: {}", a.locale);
        }
        if let Some(d) = &a.created_date {
            let _ = writeln!(sb, "Created: {}", d.format("2006-01-02"));
        }
        if !a.text.is_empty() {
            let _ = writeln!(sb, "Summary: {}", a.text);
        }
        sb.push_str("\n---\n");
    }
    sb
}
