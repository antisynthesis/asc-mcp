//! Sales and finance report tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
pub(super) fn register_reports_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "get_sales_report",
            "description": "Download sales and trends reports",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "frequency": {
                        "type": "string",
                        "description": "Report frequency (DAILY, WEEKLY, MONTHLY, YEARLY)"
                    },
                    "report_date": {
                        "type": "string",
                        "description": "Report date (YYYY-MM-DD)"
                    },
                    "report_sub_type": {
                        "type": "string",
                        "description": "Report sub-type (SUMMARY, DETAILED, OPT_IN)"
                    },
                    "report_type": {
                        "type": "string",
                        "description": "Report type (SALES, PRE_ORDER, NEWSSTAND, SUBSCRIPTION, SUBSCRIPTION_EVENT, SUBSCRIBER)"
                    },
                    "vendor_number": {
                        "type": "string",
                        "description": "Your vendor number"
                    }
                },
                "required": [
                    "vendor_number",
                    "report_type",
                    "report_sub_type",
                    "frequency",
                    "report_date"
                ]
            },
            "annotations": {
                "title": "Get Sales Report",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_sales_report),
    );
    r.register(
        tool(r##"{
            "name": "get_finance_report",
            "description": "Download financial reports",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "region_code": {
                        "type": "string",
                        "description": "Region code (e.g., US, EU, JP)"
                    },
                    "report_date": {
                        "type": "string",
                        "description": "Report date (YYYY-MM for financial periods)"
                    },
                    "report_type": {
                        "type": "string",
                        "description": "Report type (FINANCIAL, FINANCE_DETAIL)"
                    },
                    "vendor_number": {
                        "type": "string",
                        "description": "Your vendor number"
                    }
                },
                "required": [
                    "vendor_number",
                    "region_code",
                    "report_type",
                    "report_date"
                ]
            },
            "annotations": {
                "title": "Get Finance Report",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_finance_report),
    );
}

/// Handles `get_sales_report`.
async fn handle_get_sales_report(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { VendorNumber string \"json:\\\"vendor_number\\\"\"; ReportType string \"json:\\\"report_type\\\"\"; ReportSubType string \"json:\\\"report_sub_type\\\"\"; Frequency string \"json:\\\"frequency\\\"\"; ReportDate string \"json:\\\"report_date\\\"\" }", "") {
        vendor_number: String => "vendor_number";
        report_type: String => "report_type";
        report_sub_type: String => "report_sub_type";
        frequency: String => "frequency";
        report_date: String => "report_date";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.vendor_number.is_empty() {
        return Ok(error_result("vendor_number is required"));
    }
    if params.report_type.is_empty() {
        return Ok(error_result("report_type is required"));
    }
    if params.report_sub_type.is_empty() {
        return Ok(error_result("report_sub_type is required"));
    }
    if params.frequency.is_empty() {
        return Ok(error_result("frequency is required"));
    }
    if params.report_date.is_empty() {
        return Ok(error_result("report_date is required"));
    }

    let data = match c
        .get_sales_report(&params.vendor_number, &params.report_type, &params.report_sub_type, &params.frequency, &params.report_date)
        .await
    {
        Ok(d) => d,
        Err(e) => return Ok(error_result(format!("Failed to get sales report: {e}"))),
    };

    // Sales reports are typically gzip-compressed TSV files
    let mut text = format!(
        "Sales report downloaded ({} bytes). Data is gzip-compressed TSV format.\n\nFirst 1000 bytes:\n",
        data.len()
    )
    .into_bytes();
    text.extend_from_slice(&truncate_bytes(&data, 1000));
    Ok(CallToolResult::success(lossy_string(&text)))
}

/// Handles `get_finance_report`.
async fn handle_get_finance_report(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { VendorNumber string \"json:\\\"vendor_number\\\"\"; RegionCode string \"json:\\\"region_code\\\"\"; ReportType string \"json:\\\"report_type\\\"\"; ReportDate string \"json:\\\"report_date\\\"\" }", "") {
        vendor_number: String => "vendor_number";
        region_code: String => "region_code";
        report_type: String => "report_type";
        report_date: String => "report_date";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.vendor_number.is_empty() {
        return Ok(error_result("vendor_number is required"));
    }
    if params.region_code.is_empty() {
        return Ok(error_result("region_code is required"));
    }
    if params.report_type.is_empty() {
        return Ok(error_result("report_type is required"));
    }
    if params.report_date.is_empty() {
        return Ok(error_result("report_date is required"));
    }

    let data = match c
        .get_finance_report(&params.vendor_number, &params.region_code, &params.report_type, &params.report_date)
        .await
    {
        Ok(d) => d,
        Err(e) => return Ok(error_result(format!("Failed to get finance report: {e}"))),
    };

    // Finance reports are typically gzip-compressed TSV files
    let mut text = format!(
        "Finance report downloaded ({} bytes). Data is gzip-compressed TSV format.\n\nFirst 1000 bytes:\n",
        data.len()
    )
    .into_bytes();
    text.extend_from_slice(&truncate_bytes(&data, 1000));
    Ok(CallToolResult::success(lossy_string(&text)))
}

/// Truncates to `max_len` bytes plus `...`, cutting at a byte offset (which
/// may split a UTF-8 sequence).
fn truncate_bytes(s: &[u8], max_len: usize) -> Vec<u8> {
    if s.len() <= max_len {
        return s.to_vec();
    }
    let mut out = s[..max_len].to_vec();
    out.extend_from_slice(b"...");
    out
}

/// Converts raw bytes to the text emitted on the wire: every byte that
/// does not start a valid UTF-8 sequence becomes one U+FFFD, unlike
/// `String::from_utf8_lossy`, which folds a truncated sequence into a
/// single replacement character.
pub(super) fn lossy_string(b: &[u8]) -> String {
    let mut out = String::with_capacity(b.len());
    let mut rest = b;
    while !rest.is_empty() {
        match std::str::from_utf8(rest) {
            Ok(s) => {
                out.push_str(s);
                break;
            }
            Err(e) => {
                let valid = e.valid_up_to();
                out.push_str(std::str::from_utf8(&rest[..valid]).unwrap_or_default());
                out.push('\u{FFFD}');
                rest = &rest[valid + 1..];
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lossy_string_replaces_each_invalid_byte() {
        assert_eq!(lossy_string(b"ab"), "ab");
        assert_eq!(lossy_string(&[0x1f, 0x8b, 0x08]), "\u{1f}\u{FFFD}\u{8}");
        // A truncated 3-byte sequence is two replacement characters.
        assert_eq!(lossy_string(&[b'x', 0xe2, 0x82]), "x\u{FFFD}\u{FFFD}");
        assert_eq!(lossy_string("é".as_bytes()), "é");
    }

    #[test]
    fn truncate_bytes_splits_runes() {
        assert_eq!(truncate_bytes(b"abc", 3), b"abc");
        assert_eq!(truncate_bytes(b"abcd", 3), b"abc...");
    }
}
