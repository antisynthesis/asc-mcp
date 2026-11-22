//! Shared helpers for list tools: pagination cursors, list options and
//! result construction.

use serde_json::{Map, Value};

use crate::api::model::PagedDocumentLinks;
use crate::api::{Client, ListOptions, Result};
use crate::mcp::{CallToolResult, text_content};
use crate::wire::{JsonDecode, JsonEncode, JsonMap, JsonSlice, json as wirejson};

/// The JSON Schema entry list tools use to opt into pagination.
pub fn cursor_property() -> Value {
    serde_json::json!({
        "type": "string",
        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
    })
}

/// Assembles list options, or `None` when nothing is set.
pub fn list_opts(
    limit: i64,
    filter: &JsonMap<JsonSlice<String>>,
    sort: &JsonSlice<String>,
    fields: &JsonMap<JsonSlice<String>>,
    include: &JsonSlice<String>,
) -> Option<ListOptions> {
    if limit <= 0 && filter.is_empty() && sort.is_empty() && fields.is_empty() && include.is_empty()
    {
        return None;
    }
    Some(ListOptions {
        limit,
        filter: filter.clone(),
        sort: sort.clone(),
        fields: fields.clone(),
        include: include.clone(),
    })
}

/// List options carrying only a limit (`None` when `limit <= 0`).
pub fn limit_opts(limit: i64) -> Option<ListOptions> {
    list_opts(
        limit,
        &JsonMap::nil(),
        &JsonSlice::nil(),
        &JsonMap::nil(),
        &JsonSlice::nil(),
    )
}

/// Fetches the first page via `fetch_first` when `cursor` is
/// empty, otherwise the page at the cursor URL (host-checked by
/// [`Client::get_url`]).
pub async fn paginated_fetch<T, F, Fut>(client: &Client, cursor: &str, fetch_first: F) -> Result<T>
where
    T: JsonDecode + Default,
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    if cursor.is_empty() {
        return fetch_first().await;
    }
    let data = client.get_url(cursor).await?;
    wirejson::from_slice(&data)
        .map_err(|e| crate::api::Error::msg(format!("failed to parse paginated response: {e}")))
}

pub fn pagination_footer(next_url: &str) -> String {
    if next_url.is_empty() {
        return String::new();
    }
    format!(
        "\n\n_More results available. Call again with `cursor={}` to fetch the next page._",
        crate::wire::fmt::quote(next_url)
    )
}

/// A list result: text plus
/// the next-page footer, and structured content
/// `{data, hasMore, links, nextPage}`.
pub fn list_result<D: JsonEncode + ?Sized>(
    content: &str,
    data: &D,
    links: &PagedDocumentLinks,
) -> CallToolResult {
    list_result_value(content, Some(wirejson::to_value(data)), links)
}

/// A list result with an explicit data value; `None` yields text only (no
/// structured content).
pub fn list_result_value(
    content: &str,
    data: Option<Value>,
    links: &PagedDocumentLinks,
) -> CallToolResult {
    let footer = pagination_footer(&links.next);
    let mut res = CallToolResult {
        content: vec![text_content(format!("{content}{footer}"))],
        ..Default::default()
    };
    if let Some(data) = data {
        // Keys are inserted in sorted order.
        let mut m = Map::new();
        m.insert("data".into(), data);
        m.insert("hasMore".into(), Value::Bool(!links.next.is_empty()));
        m.insert("links".into(), wirejson::to_value(links));
        m.insert("nextPage".into(), Value::String(links.next.clone()));
        res.structured_content = Some(Value::Object(m));
    }
    res
}

/// A text result with `data` as structured content.
pub fn data_result<D: JsonEncode + ?Sized>(content: &str, data: &D) -> CallToolResult {
    let mut res = CallToolResult::success(content);
    res.structured_content = Some(wirejson::to_value(data));
    res
}

/// A text-only result.
pub fn text_result(content: &str) -> CallToolResult {
    CallToolResult::success(content)
}

/// An `isError` tool result carrying `text`.
pub fn error_result(text: impl Into<String>) -> CallToolResult {
    CallToolResult::error(text)
}
