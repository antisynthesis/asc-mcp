//! Ported from internal/asc/tools/uploads.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerUploadTools`.
pub(super) fn register_upload_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "upload_app_screenshot",
            "description": "Reserve, upload, and commit an App Store screenshot to an existing screenshot set. Provide exactly one of file_path or file_data_base64. Returns the final screenshot record once Apple has confirmed the bytes.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "file_data_base64": {
                        "type": "string",
                        "description": "Standard base64 (RFC 4648) encoded bytes of the file. Use this when the client must transport the file across a remote transport such as HTTP."
                    },
                    "file_name": {
                        "type": "string",
                        "description": "The original file name (e.g. iphone6.5-1.png). Apple uses this verbatim in the asset URL."
                    },
                    "file_path": {
                        "type": "string",
                        "description": "Absolute path to the file on the server's local filesystem. Use this when the MCP server and the file are on the same host (most stdio scenarios)."
                    },
                    "screenshot_set_id": {
                        "type": "string",
                        "description": "The App Screenshot Set ID that the new screenshot will belong to."
                    }
                },
                "required": [
                    "screenshot_set_id",
                    "file_name"
                ]
            },
            "annotations": {
                "title": "Upload App Screenshot",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_upload_app_screenshot),
    );
    r.register(
        tool(r##"{
            "name": "upload_app_preview",
            "description": "Reserve, upload, and commit an App Store preview video to an existing preview set. Provide exactly one of file_path or file_data_base64. mime_type defaults to video/mp4 if omitted. Returns the final preview record once Apple has confirmed the bytes.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "file_data_base64": {
                        "type": "string",
                        "description": "Standard base64 (RFC 4648) encoded bytes of the file. Use this when the client must transport the file across a remote transport such as HTTP."
                    },
                    "file_name": {
                        "type": "string",
                        "description": "The original file name (e.g. iphone6.5-1.mp4)."
                    },
                    "file_path": {
                        "type": "string",
                        "description": "Absolute path to the file on the server's local filesystem. Use this when the MCP server and the file are on the same host (most stdio scenarios)."
                    },
                    "mime_type": {
                        "type": "string",
                        "description": "MIME type of the preview file. Defaults to video/mp4."
                    },
                    "preview_set_id": {
                        "type": "string",
                        "description": "The App Preview Set ID that the new preview will belong to."
                    }
                },
                "required": [
                    "preview_set_id",
                    "file_name"
                ]
            },
            "annotations": {
                "title": "Upload App Preview",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_upload_app_preview),
    );
    r.register(
        tool(r##"{
            "name": "upload_review_attachment",
            "description": "Reserve, upload, and commit an App Store review attachment (PDF, screenshot, or other supporting material) to an App Store Review Detail. Provide exactly one of file_path or file_data_base64.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "file_data_base64": {
                        "type": "string",
                        "description": "Standard base64 (RFC 4648) encoded bytes of the file. Use this when the client must transport the file across a remote transport such as HTTP."
                    },
                    "file_name": {
                        "type": "string",
                        "description": "The original file name."
                    },
                    "file_path": {
                        "type": "string",
                        "description": "Absolute path to the file on the server's local filesystem. Use this when the MCP server and the file are on the same host (most stdio scenarios)."
                    },
                    "review_detail_id": {
                        "type": "string",
                        "description": "The App Store Review Detail ID that the attachment will belong to."
                    }
                },
                "required": [
                    "review_detail_id",
                    "file_name"
                ]
            },
            "annotations": {
                "title": "Upload Review Attachment",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_upload_review_attachment),
    );
}

/// Go `MaxLoadedFileSize`: the largest payload loaded from disk or decoded
/// from base64 before it reaches the API client.
pub(crate) const MAX_LOADED_FILE_SIZE: usize = api::MAX_UPLOAD_SIZE;

/// Go `loadUploadBody`: resolves exactly one of `file_path` (local stdio
/// sessions) or `file_data_base64` (remote transports) to the bytes to
/// upload. Error strings match the Go server's.
pub(crate) fn load_upload_body(file_path: &str, file_data_base64: &str) -> Result<Vec<u8>, String> {
    let has_path = !file_path.is_empty();
    let has_data = !file_data_base64.is_empty();
    if has_path == has_data {
        return Err("provide exactly one of file_path or file_data_base64".into());
    }
    if has_data {
        let b = decode_base64_flexible(file_data_base64)
            .map_err(|e| format!("file_data_base64 is not valid base64: {e}"))?;
        if b.len() > MAX_LOADED_FILE_SIZE {
            return Err(format!("file_data exceeds max size of {MAX_LOADED_FILE_SIZE} bytes"));
        }
        return Ok(b);
    }
    let abs = go_abs(file_path).map_err(|e| format!("resolve file_path: {e}"))?;
    let meta = std::fs::metadata(&abs)
        .map_err(|e| format!("stat file_path: {}", crate::go::os::path_error("stat", &abs, &e)))?;
    if meta.is_dir() {
        return Err("file_path is a directory".into());
    }
    if meta.len() as usize > MAX_LOADED_FILE_SIZE {
        return Err(format!("file at {abs} exceeds max size of {MAX_LOADED_FILE_SIZE} bytes"));
    }
    std::fs::read(&abs).map_err(|e| crate::go::os::path_error("open", &abs, &e))
}

/// Go `decodeBase64Flexible`: standard or URL-safe alphabet, with or
/// without padding. Like Go's decoders, CR and LF are ignored and
/// non-canonical trailing bits are accepted.
pub(crate) fn decode_base64_flexible(s: &str) -> Result<Vec<u8>, String> {
    use base64::alphabet::{STANDARD, URL_SAFE};
    use base64::engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig};
    use base64::Engine;
    let cleaned: String = s.chars().filter(|c| *c != '\r' && *c != '\n').collect();
    let cfg = |pad: DecodePaddingMode| {
        GeneralPurposeConfig::new().with_decode_allow_trailing_bits(true).with_decode_padding_mode(pad)
    };
    let engines = [
        GeneralPurpose::new(&STANDARD, cfg(DecodePaddingMode::RequireCanonical)),
        GeneralPurpose::new(&STANDARD, cfg(DecodePaddingMode::RequireNone)),
        GeneralPurpose::new(&URL_SAFE, cfg(DecodePaddingMode::RequireCanonical)),
        GeneralPurpose::new(&URL_SAFE, cfg(DecodePaddingMode::RequireNone)),
    ];
    for e in &engines {
        if let Ok(b) = e.decode(&cleaned) {
            return Ok(b);
        }
    }
    Err("unrecognized base64 encoding".into())
}

/// Go `filepath.Abs`: joins relative paths onto the working directory and
/// cleans the result lexically.
pub(crate) fn go_abs(path: &str) -> Result<String, String> {
    let joined = if path.starts_with('/') {
        path.to_string()
    } else {
        let cwd = std::env::current_dir().map_err(|e| crate::go::os::errno_text(&e))?;
        format!("{}/{}", cwd.display(), path)
    };
    Ok(go_clean(&joined))
}

/// Go `filepath.Clean` for Unix paths.
pub(crate) fn go_clean(path: &str) -> String {
    if path.is_empty() {
        return ".".into();
    }
    let rooted = path.starts_with('/');
    let mut out: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                if out.last().is_some_and(|p| *p != "..") {
                    out.pop();
                } else if !rooted {
                    out.push("..");
                }
            }
            p => out.push(p),
        }
    }
    let body = out.join("/");
    match (rooted, body.is_empty()) {
        (true, _) => format!("/{body}"),
        (false, true) => ".".into(),
        (false, false) => body,
    }
}

/// Ported from `Registry.handleUploadAppScreenshot`.
async fn handle_upload_app_screenshot(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ScreenshotSetID string \"json:\\\"screenshot_set_id\\\"\"; FileName string \"json:\\\"file_name\\\"\"; FilePath string \"json:\\\"file_path\\\"\"; FileDataBase64 string \"json:\\\"file_data_base64\\\"\" }", "") {
        screenshot_set_id: String => "screenshot_set_id";
        file_name: String => "file_name";
        file_path: String => "file_path";
        file_data_base64: String => "file_data_base64";
    }
    }
    todo!("port handleUploadAppScreenshot from internal/asc/tools/uploads.go")
}

/// Ported from `Registry.handleUploadAppPreview`.
async fn handle_upload_app_preview(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PreviewSetID string \"json:\\\"preview_set_id\\\"\"; FileName string \"json:\\\"file_name\\\"\"; MimeType string \"json:\\\"mime_type\\\"\"; FilePath string \"json:\\\"file_path\\\"\"; FileDataBase64 string \"json:\\\"file_data_base64\\\"\" }", "") {
        preview_set_id: String => "preview_set_id";
        file_name: String => "file_name";
        mime_type: String => "mime_type";
        file_path: String => "file_path";
        file_data_base64: String => "file_data_base64";
    }
    }
    todo!("port handleUploadAppPreview from internal/asc/tools/uploads.go")
}

/// Ported from `Registry.handleUploadReviewAttachment`.
async fn handle_upload_review_attachment(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ReviewDetailID string \"json:\\\"review_detail_id\\\"\"; FileName string \"json:\\\"file_name\\\"\"; FilePath string \"json:\\\"file_path\\\"\"; FileDataBase64 string \"json:\\\"file_data_base64\\\"\" }", "") {
        review_detail_id: String => "review_detail_id";
        file_name: String => "file_name";
        file_path: String => "file_path";
        file_data_base64: String => "file_data_base64";
    }
    }
    todo!("port handleUploadReviewAttachment from internal/asc/tools/uploads.go")
}

