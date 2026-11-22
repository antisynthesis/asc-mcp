//! Ported from internal/asc/tools/build_uploads.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerBuildUploadTools`.
pub(super) fn register_build_upload_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "start_build_upload",
            "description": "Start a build upload for an app. Creates the buildUploads record that upload_build_file delivers the binary into. The version attributes must match the binary's Info.plist values.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to upload a build for"
                    },
                    "cf_bundle_short_version_string": {
                        "type": "string",
                        "description": "The binary's CFBundleShortVersionString (marketing version, e.g. 1.2.3)"
                    },
                    "cf_bundle_version": {
                        "type": "string",
                        "description": "The binary's CFBundleVersion (build number, e.g. 42)"
                    },
                    "platform": {
                        "type": "string",
                        "description": "The platform: IOS, MAC_OS, TV_OS, or VISION_OS",
                        "enum": [
                            "IOS",
                            "MAC_OS",
                            "TV_OS",
                            "VISION_OS"
                        ]
                    }
                },
                "required": [
                    "app_id",
                    "cf_bundle_short_version_string",
                    "cf_bundle_version",
                    "platform"
                ]
            },
            "annotations": {
                "title": "Start Build Upload",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_start_build_upload),
    );
    r.register(
        tool(r##"{
            "name": "upload_build_file",
            "description": "Reserve, upload, and commit a build binary (or auxiliary asset) on an existing build upload created by start_build_upload. Provide exactly one of file_path or file_data_base64. After committing, poll get_build_upload until the state is COMPLETE or FAILED.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "asset_type": {
                        "type": "string",
                        "description": "The asset slot to fill: ASSET (the binary itself, default), ASSET_DESCRIPTION, or ASSET_SPI",
                        "enum": [
                            "ASSET",
                            "ASSET_DESCRIPTION",
                            "ASSET_SPI"
                        ]
                    },
                    "build_upload_id": {
                        "type": "string",
                        "description": "The Build Upload ID returned by start_build_upload"
                    },
                    "file_data_base64": {
                        "type": "string",
                        "description": "Standard base64 (RFC 4648) encoded bytes of the file. Use this when the client must transport the file across a remote transport such as HTTP."
                    },
                    "file_name": {
                        "type": "string",
                        "description": "The original file name (e.g. MyApp.ipa)"
                    },
                    "file_path": {
                        "type": "string",
                        "description": "Absolute path to the file on the server's local filesystem. Use this when the MCP server and the file are on the same host (most stdio scenarios)."
                    },
                    "uti": {
                        "type": "string",
                        "description": "Uniform type identifier of the file. Defaults to com.apple.ipa.",
                        "enum": [
                            "com.apple.ipa",
                            "com.apple.pkg",
                            "com.pkware.zip-archive",
                            "com.apple.binary-property-list",
                            "com.apple.xml-property-list"
                        ]
                    }
                },
                "required": [
                    "build_upload_id",
                    "file_name"
                ]
            },
            "annotations": {
                "title": "Upload Build File",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_upload_build_file),
    );
    r.register(
        tool(r##"{
            "name": "get_build_upload",
            "description": "Get the status of a build upload, including its processing state (AWAITING_UPLOAD, PROCESSING, FAILED, COMPLETE), any state errors or warnings, and the resulting build ID once processing completes",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "build_upload_id": {
                        "type": "string",
                        "description": "The Build Upload ID"
                    }
                },
                "required": [
                    "build_upload_id"
                ]
            },
            "annotations": {
                "title": "Get Build Upload",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_build_upload),
    );
    r.register(
        tool(r##"{
            "name": "list_build_uploads",
            "description": "List the build uploads for an app, including their processing states",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to list build uploads for"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Supported keys: cfBundleShortVersionString, cfBundleVersion, platform (IOS, MAC_OS, TV_OS, VISION_OS), state. Values are arrays, e.g. {\"state\": [\"PROCESSING\"]}."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of build uploads to return (default 50, max 200)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort order, e.g. -uploadedDate, cfBundleVersion",
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
                "title": "List Build Uploads",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_build_uploads),
    );
    r.register(
        tool(r##"{
            "name": "list_build_upload_files",
            "description": "List the asset files attached to a build upload, including each file's asset delivery state",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "build_upload_id": {
                        "type": "string",
                        "description": "The Build Upload ID"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of files to return (default 50, max 200)"
                    }
                },
                "required": [
                    "build_upload_id"
                ]
            },
            "annotations": {
                "title": "List Build Upload Files",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_build_upload_files),
    );
    r.register(
        tool(r##"{
            "name": "delete_build_upload",
            "description": "Delete a build upload (e.g. an abandoned or failed delivery)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "build_upload_id": {
                        "type": "string",
                        "description": "The Build Upload ID"
                    }
                },
                "required": [
                    "build_upload_id"
                ]
            },
            "annotations": {
                "title": "Delete Build Upload",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_build_upload),
    );
}

/// Ported from `Registry.handleStartBuildUpload`.
async fn handle_start_build_upload(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; CFBundleShortVersionString string \"json:\\\"cf_bundle_short_version_string\\\"\"; CFBundleVersion string \"json:\\\"cf_bundle_version\\\"\"; Platform string \"json:\\\"platform\\\"\" }", "") {
        app_id: String => "app_id";
        cf_bundle_short_version_string: String => "cf_bundle_short_version_string";
        cf_bundle_version: String => "cf_bundle_version";
        platform: String => "platform";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }
    if params.cf_bundle_short_version_string.is_empty() {
        return Ok(error_result("cf_bundle_short_version_string is required"));
    }
    if params.cf_bundle_version.is_empty() {
        return Ok(error_result("cf_bundle_version is required"));
    }
    if params.platform.is_empty() {
        return Ok(error_result("platform is required"));
    }

    let req = BuildUploadCreateRequest {
        data: BuildUploadCreateData {
            type_: "buildUploads".into(),
            attributes: BuildUploadCreateAttributes {
                cf_bundle_short_version_string: params.cf_bundle_short_version_string.clone(),
                cf_bundle_version: params.cf_bundle_version.clone(),
                platform: params.platform.clone(),
            },
            relationships: BuildUploadCreateRelationships {
                app: RelationshipData {
                    data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_build_upload(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to start build upload: {e}"))),
    };

    Ok(data_result(
        &format!(
            "Started build upload:\n{}\nNext, call upload_build_file with build_upload_id={} to deliver the binary.",
            format_build_upload(&resp.data),
            resp.data.id
        ),
        &resp.data,
    ))
}

/// Ported from `Registry.handleUploadBuildFile`.
async fn handle_upload_build_file(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildUploadID string \"json:\\\"build_upload_id\\\"\"; FileName string \"json:\\\"file_name\\\"\"; AssetType string \"json:\\\"asset_type\\\"\"; UTI string \"json:\\\"uti\\\"\"; FilePath string \"json:\\\"file_path\\\"\"; FileDataBase64 string \"json:\\\"file_data_base64\\\"\" }", "") {
        build_upload_id: String => "build_upload_id";
        file_name: String => "file_name";
        asset_type: String => "asset_type";
        uti: String => "uti";
        file_path: String => "file_path";
        file_data_base64: String => "file_data_base64";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Ok(error_result(format!("invalid arguments: {e}")));
    }

    if params.build_upload_id.is_empty() {
        return Ok(error_result("build_upload_id is required"));
    }
    if params.file_name.is_empty() {
        return Ok(error_result("file_name is required"));
    }
    let mut asset_type = params.asset_type.clone();
    if asset_type.is_empty() {
        asset_type = "ASSET".into();
    }
    let mut uti = params.uti.clone();
    if uti.is_empty() {
        uti = "com.apple.ipa".into();
    }
    let body = match super::uploads::load_upload_body(&params.file_path, &params.file_data_base64) {
        Ok(b) => b,
        Err(e) => return Ok(error_result(e)),
    };

    let resp = match c.upload_build_file(&params.build_upload_id, &params.file_name, &asset_type, &uti, &body).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to upload build file: {e}"))),
    };

    let text = format!(
        "Uploaded build file {} ({}, {} bytes) to build upload {}. Poll get_build_upload for processing status.",
        resp.data.id,
        params.file_name,
        body.len(),
        params.build_upload_id
    );
    Ok(data_result(&text, &resp.data))
}

/// Ported from `Registry.handleGetBuildUpload`.
async fn handle_get_build_upload(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildUploadID string \"json:\\\"build_upload_id\\\"\" }", "") {
        build_upload_id: String => "build_upload_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.build_upload_id.is_empty() {
        return Ok(error_result("build_upload_id is required"));
    }

    let resp = match c.get_build_upload(&params.build_upload_id, None).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get build upload: {e}"))),
    };

    Ok(data_result(&format_build_upload(&resp.data), &resp.data))
}

/// Ported from `Registry.handleListBuildUploads`.
async fn handle_list_build_uploads(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
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

    let opts = list_opts(limit, &params.filter, &params.sort, &GoMap::nil(), &GoSlice::nil());
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_build_uploads(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list build uploads: {e}"))),
    };

    Ok(list_result(&format_build_uploads(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleListBuildUploadFiles`.
async fn handle_list_build_upload_files(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildUploadID string \"json:\\\"build_upload_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\" }", "") {
        build_upload_id: String => "build_upload_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.build_upload_id.is_empty() {
        return Ok(error_result("build_upload_id is required"));
    }

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = limit_opts(limit);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_build_upload_files(&params.build_upload_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list build upload files: {e}"))),
    };

    Ok(list_result(&format_build_upload_files(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleDeleteBuildUpload`.
async fn handle_delete_build_upload(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BuildUploadID string \"json:\\\"build_upload_id\\\"\" }", "") {
        build_upload_id: String => "build_upload_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.build_upload_id.is_empty() {
        return Ok(error_result("build_upload_id is required"));
    }

    if let Err(e) = c.delete_build_upload(&params.build_upload_id).await {
        return Ok(error_result(format!("Failed to delete build upload: {e}")));
    }

    Ok(CallToolResult::success("Build upload deleted successfully"))
}

/// Ported from `formatBuildUploads`.
fn format_build_uploads(uploads: &[BuildUpload]) -> String {
    if uploads.is_empty() {
        return "No build uploads found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} build uploads:\n\n", uploads.len());
    for upload in uploads {
        sb.push_str(&format_build_upload(upload));
        sb.push_str("\n---\n");
    }
    sb
}

/// Ported from `formatBuildUpload`.
fn format_build_upload(upload: &BuildUpload) -> String {
    let a = &upload.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "Build Upload ID: {}", upload.id);
    if !a.cf_bundle_short_version_string.is_empty() {
        let _ = writeln!(sb, "Version: {} ({})", a.cf_bundle_short_version_string, a.cf_bundle_version);
    }
    if !a.platform.is_empty() {
        let _ = writeln!(sb, "Platform: {}", a.platform);
    }
    if let Some(state) = &a.state {
        let _ = writeln!(sb, "State: {}", state.state);
        for detail in state.errors.iter() {
            let _ = writeln!(sb, "Error [{}]: {}", detail.code, detail.description);
        }
        for detail in state.warnings.iter() {
            let _ = writeln!(sb, "Warning [{}]: {}", detail.code, detail.description);
        }
    }
    if let Some(d) = &a.created_date {
        let _ = writeln!(sb, "Created: {}", d.format("2006-01-02 15:04:05"));
    }
    if let Some(d) = &a.uploaded_date {
        let _ = writeln!(sb, "Uploaded: {}", d.format("2006-01-02 15:04:05"));
    }
    if let Some(rel) = &upload.relationships {
        if let Some(build) = &rel.build {
            if !build.data.id.is_empty() {
                let _ = writeln!(sb, "Build ID: {}", build.data.id);
            }
        }
    }
    sb
}

/// Ported from `formatBuildUploadFiles`.
fn format_build_upload_files(files: &[BuildUploadFile]) -> String {
    if files.is_empty() {
        return "No build upload files found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} build upload files:\n\n", files.len());
    for file in files {
        let a = &file.attributes;
        let _ = writeln!(sb, "File ID: {}", file.id);
        if !a.file_name.is_empty() {
            let _ = writeln!(sb, "Name: {} ({} bytes)", a.file_name, a.file_size);
        }
        if !a.asset_type.is_empty() {
            let _ = writeln!(sb, "Asset Type: {}", a.asset_type);
        }
        if !a.uti.is_empty() {
            let _ = writeln!(sb, "UTI: {}", a.uti);
        }
        if let Some(ds) = &a.asset_delivery_state {
            if !ds.state.is_empty() {
                let _ = writeln!(sb, "Delivery State: {}", ds.state);
            }
        }
        sb.push_str("\n---\n");
    }
    sb
}
