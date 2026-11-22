//! Background asset tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
pub(super) fn register_background_asset_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_background_assets",
            "description": "List the Apple-hosted background assets for an app, including each asset pack's identifier, archived state, and storage used (usedBytes). Filter versions.locale to read per-locale asset packs.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to list background assets for"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Supported keys: archived (true/false), assetPackIdentifier, versions.locale (e.g. en-US), versions.platforms (IOS, MAC_OS, TV_OS, VISION_OS). Values are arrays, e.g. {\"versions.locale\": [\"en-US\"]}."
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response (supported: appStoreVersion, internalBetaVersion, externalBetaVersion).",
                        "items": {
                            "type": "string"
                        }
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of background assets to return (default 50, max 200)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort order, e.g. assetPackIdentifier, -createdDate",
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
                "title": "List Background Assets",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_background_assets),
    );
    r.register(
        tool(r##"{
            "name": "get_background_asset",
            "description": "Get details of a background asset, including its asset pack identifier, archived state, storage used (usedBytes), and the version currently live in each release channel",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "background_asset_id": {
                        "type": "string",
                        "description": "The Background Asset ID"
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response (supported: appStoreVersion, internalBetaVersion, externalBetaVersion).",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "background_asset_id"
                ]
            },
            "annotations": {
                "title": "Get Background Asset",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_background_asset),
    );
    r.register(
        tool(r##"{
            "name": "create_background_asset",
            "description": "Create an Apple-hosted background asset for an app. The asset pack identifier must match the identifier your app requests at runtime. Next, call create_background_asset_version to open a version for upload.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to create the background asset for"
                    },
                    "asset_pack_identifier": {
                        "type": "string",
                        "description": "The asset pack identifier (reverse-DNS style, e.g. com.example.game.levels1)"
                    }
                },
                "required": [
                    "app_id",
                    "asset_pack_identifier"
                ]
            },
            "annotations": {
                "title": "Create Background Asset",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_background_asset),
    );
    r.register(
        tool(r##"{
            "name": "update_background_asset",
            "description": "Archive or unarchive a background asset. Archiving stops distribution of the asset pack without deleting it.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "archived": {
                        "type": "boolean",
                        "description": "true to archive the asset, false to unarchive it"
                    },
                    "background_asset_id": {
                        "type": "string",
                        "description": "The Background Asset ID"
                    }
                },
                "required": [
                    "background_asset_id",
                    "archived"
                ]
            },
            "annotations": {
                "title": "Update Background Asset",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_background_asset),
    );
    r.register(
        tool(r##"{
            "name": "list_background_asset_versions",
            "description": "List the versions of a background asset, including each version's processing state, state details, and per-channel release states (internal beta, external beta, App Store)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "background_asset_id": {
                        "type": "string",
                        "description": "The Background Asset ID to list versions for"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "filter": {
                        "type": "object",
                        "description": "JSON:API filter map. Supported keys: locale, platforms (IOS, MAC_OS, TV_OS, VISION_OS), state (AWAITING_UPLOAD, PROCESSING, FAILED, COMPLETE), version, internalBetaRelease.state, externalBetaRelease.state, appStoreRelease.state. Values are arrays, e.g. {\"state\": [\"COMPLETE\"]}."
                    },
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response (supported: internalBetaRelease, externalBetaRelease, appStoreRelease, assetFile, manifestFile).",
                        "items": {
                            "type": "string"
                        }
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of versions to return (default 50, max 200)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort order, e.g. -version",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "background_asset_id"
                ]
            },
            "annotations": {
                "title": "List Background Asset Versions",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_background_asset_versions),
    );
    r.register(
        tool(r##"{
            "name": "get_background_asset_version",
            "description": "Get a background asset version, including its processing state (AWAITING_UPLOAD, PROCESSING, FAILED, COMPLETE), state detail errors and warnings, and the IDs of its per-channel release records",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "include": {
                        "type": "array",
                        "description": "Related resource names to include in the response (supported: internalBetaRelease, externalBetaRelease, appStoreRelease, assetFile, manifestFile).",
                        "items": {
                            "type": "string"
                        }
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The Background Asset Version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Get Background Asset Version",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_background_asset_version),
    );
    r.register(
        tool(r##"{
            "name": "create_background_asset_version",
            "description": "Create a new version on a background asset. Apple assigns the version number. Next, call upload_background_asset_file to deliver the asset pack and its manifest.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "background_asset_id": {
                        "type": "string",
                        "description": "The Background Asset ID to create a version on"
                    }
                },
                "required": [
                    "background_asset_id"
                ]
            },
            "annotations": {
                "title": "Create Background Asset Version",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_background_asset_version),
    );
    r.register(
        tool(r##"{
            "name": "upload_background_asset_file",
            "description": "Reserve, upload, and commit one file on a background asset version created by create_background_asset_version. Upload the asset pack archive (asset_type ASSET) and its manifest plist (asset_type MANIFEST); both are required before Apple processes the version. Provide exactly one of file_path or file_data_base64. After committing both files, poll get_background_asset_version until the state is COMPLETE or FAILED.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "asset_type": {
                        "type": "string",
                        "description": "The file slot to fill: ASSET (the asset pack archive, default) or MANIFEST (its manifest plist)",
                        "enum": [
                            "ASSET",
                            "MANIFEST"
                        ]
                    },
                    "file_data_base64": {
                        "type": "string",
                        "description": "Standard base64 (RFC 4648) encoded bytes of the file. Use this when the client must transport the file across a remote transport such as HTTP."
                    },
                    "file_name": {
                        "type": "string",
                        "description": "The original file name (e.g. levels1.aar or Manifest.plist)"
                    },
                    "file_path": {
                        "type": "string",
                        "description": "Absolute path to the file on the server's local filesystem. Use this when the MCP server and the file are on the same host (most stdio scenarios)."
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The Background Asset Version ID returned by create_background_asset_version"
                    }
                },
                "required": [
                    "version_id",
                    "file_name"
                ]
            },
            "annotations": {
                "title": "Upload Background Asset File",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_upload_background_asset_file),
    );
    r.register(
        tool(r##"{
            "name": "list_background_asset_upload_files",
            "description": "List the upload files (asset pack and manifest) attached to a background asset version, including each file's asset delivery state",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of files to return (default 50, max 200)"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The Background Asset Version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "List Background Asset Upload Files",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_background_asset_upload_files),
    );
    r.register(
        tool(r##"{
            "name": "get_background_asset_version_release",
            "description": "Get the release state of a background asset version in one channel: internal beta, external beta (TestFlight), or App Store. Release IDs come from get_background_asset_version or list_background_asset_versions relationships/includes.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "release_id": {
                        "type": "string",
                        "description": "The release record ID for the chosen channel"
                    },
                    "release_type": {
                        "type": "string",
                        "description": "The release channel: INTERNAL_BETA (states READY_FOR_TESTING, SUPERSEDED), EXTERNAL_BETA (adds review states like WAITING_FOR_REVIEW, IN_REVIEW, REJECTED), or APP_STORE (adds PREPARE_FOR_SUBMISSION, ACCEPTED, READY_FOR_DISTRIBUTION)",
                        "enum": [
                            "INTERNAL_BETA",
                            "EXTERNAL_BETA",
                            "APP_STORE"
                        ]
                    }
                },
                "required": [
                    "release_type",
                    "release_id"
                ]
            },
            "annotations": {
                "title": "Get Background Asset Version Release",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_background_asset_version_release),
    );
}

/// Handles `list_background_assets`.
async fn handle_list_background_assets(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
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

    let limit = clamp_limit(params.limit);

    let opts = list_opts(limit, &params.filter, &params.sort, &JsonMap::nil(), &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_background_assets(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list background assets: {e}"))),
    };

    Ok(list_result(&format_background_assets(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_background_asset`.
async fn handle_get_background_asset(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { BackgroundAssetID string \"json:\\\"background_asset_id\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        background_asset_id: String => "background_asset_id";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.background_asset_id.is_empty() {
        return Ok(error_result("background_asset_id is required"));
    }

    let opts = list_opts(0, &JsonMap::nil(), &JsonSlice::nil(), &JsonMap::nil(), &params.include);
    let resp = match c.get_background_asset(&params.background_asset_id, opts.as_ref()).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get background asset: {e}"))),
    };

    Ok(data_result(&format_background_asset(&resp.data), &resp.data))
}

/// Handles `create_background_asset`.
async fn handle_create_background_asset(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; AssetPackIdentifier string \"json:\\\"asset_pack_identifier\\\"\" }", "") {
        app_id: String => "app_id";
        asset_pack_identifier: String => "asset_pack_identifier";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }

    if params.asset_pack_identifier.is_empty() {
        return Ok(error_result("asset_pack_identifier is required"));
    }

    let req = BackgroundAssetCreateRequest {
        data: BackgroundAssetCreateData {
            type_: "backgroundAssets".into(),
            attributes: BackgroundAssetCreateAttributes { asset_pack_identifier: params.asset_pack_identifier.clone() },
            relationships: BackgroundAssetCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
            },
        },
    };

    let resp = match c.create_background_asset(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create background asset: {e}"))),
    };

    Ok(data_result(
        &format!(
            "Created background asset:\n{}\nNext, call create_background_asset_version with background_asset_id={} to open a version for upload.",
            format_background_asset(&resp.data),
            resp.data.id
        ),
        &resp.data,
    ))
}

/// Handles `update_background_asset`.
async fn handle_update_background_asset(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { BackgroundAssetID string \"json:\\\"background_asset_id\\\"\"; Archived *bool \"json:\\\"archived\\\"\" }", "") {
        background_asset_id: String => "background_asset_id";
        archived: Option<bool> => "archived";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.background_asset_id.is_empty() {
        return Ok(error_result("background_asset_id is required"));
    }

    if params.archived.is_none() {
        return Ok(error_result("archived is required"));
    }

    let req = BackgroundAssetUpdateRequest {
        data: BackgroundAssetUpdateData {
            type_: "backgroundAssets".into(),
            id: params.background_asset_id.clone(),
            attributes: Some(BackgroundAssetUpdateAttributes { archived: params.archived }),
        },
    };

    let resp = match c.update_background_asset(&params.background_asset_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update background asset: {e}"))),
    };

    Ok(data_result(&format!("Background asset updated:\n{}", format_background_asset(&resp.data)), &resp.data))
}

/// Handles `list_background_asset_versions`.
async fn handle_list_background_asset_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { BackgroundAssetID string \"json:\\\"background_asset_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        background_asset_id: String => "background_asset_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.background_asset_id.is_empty() {
        return Ok(error_result("background_asset_id is required"));
    }

    let limit = clamp_limit(params.limit);

    let opts = list_opts(limit, &params.filter, &params.sort, &JsonMap::nil(), &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_background_asset_versions(&params.background_asset_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list background asset versions: {e}"))),
    };

    Ok(list_result(&format_background_asset_versions(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_background_asset_version`.
async fn handle_get_background_asset_version(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        version_id: String => "version_id";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    let opts = list_opts(0, &JsonMap::nil(), &JsonSlice::nil(), &JsonMap::nil(), &params.include);
    let resp = match c.get_background_asset_version(&params.version_id, opts.as_ref()).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get background asset version: {e}"))),
    };

    Ok(data_result(&format_background_asset_version(&resp.data), &resp.data))
}

/// Handles `create_background_asset_version`.
async fn handle_create_background_asset_version(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { BackgroundAssetID string \"json:\\\"background_asset_id\\\"\" }", "") {
        background_asset_id: String => "background_asset_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.background_asset_id.is_empty() {
        return Ok(error_result("background_asset_id is required"));
    }

    let req = BackgroundAssetVersionCreateRequest {
        data: BackgroundAssetVersionCreateData {
            type_: "backgroundAssetVersions".into(),
            relationships: BackgroundAssetVersionCreateRelationships {
                background_asset: RelationshipData {
                    data: ResourceIdentifier { type_: "backgroundAssets".into(), id: params.background_asset_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_background_asset_version(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create background asset version: {e}"))),
    };

    Ok(data_result(
        &format!(
            "Created background asset version:\n{}\nNext, call upload_background_asset_file with version_id={} to deliver the asset pack (ASSET) and its manifest (MANIFEST).",
            format_background_asset_version(&resp.data),
            resp.data.id
        ),
        &resp.data,
    ))
}

/// Handles `upload_background_asset_file`.
async fn handle_upload_background_asset_file(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; FileName string \"json:\\\"file_name\\\"\"; AssetType string \"json:\\\"asset_type\\\"\"; FilePath string \"json:\\\"file_path\\\"\"; FileDataBase64 string \"json:\\\"file_data_base64\\\"\" }", "") {
        version_id: String => "version_id";
        file_name: String => "file_name";
        asset_type: String => "asset_type";
        file_path: String => "file_path";
        file_data_base64: String => "file_data_base64";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Ok(error_result(format!("invalid arguments: {e}")));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    if params.file_name.is_empty() {
        return Ok(error_result("file_name is required"));
    }
    let mut asset_type = params.asset_type.clone();
    if asset_type.is_empty() {
        asset_type = "ASSET".into();
    }
    let body = match super::uploads::load_upload_body(&params.file_path, &params.file_data_base64) {
        Ok(b) => b,
        Err(e) => return Ok(error_result(e)),
    };

    let resp = match c.upload_background_asset_file(&params.version_id, &params.file_name, &asset_type, &body).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to upload background asset file: {e}"))),
    };

    let text = format!(
        "Uploaded background asset file {} ({}, {}, {} bytes) to version {}. Poll get_background_asset_version for processing status.",
        resp.data.id,
        params.file_name,
        asset_type,
        body.len(),
        params.version_id
    );
    Ok(data_result(&text, &resp.data))
}

/// Handles `list_background_asset_upload_files`.
async fn handle_list_background_asset_upload_files(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\" }", "") {
        version_id: String => "version_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    let limit = clamp_limit(params.limit);

    let opts = limit_opts(limit);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_background_asset_upload_files(&params.version_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list background asset upload files: {e}"))),
    };

    Ok(list_result(&format_background_asset_upload_files(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_background_asset_version_release`.
async fn handle_get_background_asset_version_release(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { ReleaseType string \"json:\\\"release_type\\\"\"; ReleaseID string \"json:\\\"release_id\\\"\" }", "") {
        release_type: String => "release_type";
        release_id: String => "release_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.release_type.is_empty() {
        return Ok(error_result("release_type is required"));
    }

    if params.release_id.is_empty() {
        return Ok(error_result("release_id is required"));
    }

    let result = match params.release_type.as_str() {
        "INTERNAL_BETA" => c.get_background_asset_version_internal_beta_release(&params.release_id).await,
        "EXTERNAL_BETA" => c.get_background_asset_version_external_beta_release(&params.release_id).await,
        "APP_STORE" => c.get_background_asset_version_app_store_release(&params.release_id).await,
        _ => return Ok(error_result("release_type must be INTERNAL_BETA, EXTERNAL_BETA, or APP_STORE")),
    };
    let resp = match result {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get background asset version release: {e}"))),
    };

    Ok(data_result(&format_background_asset_version_release(&resp.data), &resp.data))
}

/// Clamps a page size: `limit <= 0 → 50; limit > 200 → 200`.
fn clamp_limit(limit: i64) -> i64 {
    let mut limit = limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }
    limit
}

fn format_background_assets(assets: &[BackgroundAsset]) -> String {
    if assets.is_empty() {
        return "No background assets found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} background assets:\n\n", assets.len());

    for asset in assets {
        sb.push_str(&format_background_asset(asset));
        sb.push_str("\n---\n");
    }

    sb
}

/// The related resource's ID, or `None` when the optional relationship is
/// absent or carries an empty ID.
fn rel_id(rel: &Option<RelationshipData>) -> Option<&str> {
    rel.as_ref().map(|r| r.data.id.as_str()).filter(|id| !id.is_empty())
}

fn format_background_asset(asset: &BackgroundAsset) -> String {
    let a = &asset.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "Background Asset ID: {}", asset.id);
    if !a.asset_pack_identifier.is_empty() {
        let _ = writeln!(sb, "Asset Pack Identifier: {}", a.asset_pack_identifier);
    }
    let _ = writeln!(sb, "Archived: {}", a.archived);
    if a.used_bytes > 0 {
        let _ = writeln!(sb, "Used Bytes: {}", a.used_bytes);
    }
    if let Some(d) = &a.created_date {
        let _ = writeln!(sb, "Created: {}", d.format("2006-01-02 15:04:05"));
    }
    if let Some(rel) = &asset.relationships {
        if let Some(id) = rel_id(&rel.app_store_version) {
            let _ = writeln!(sb, "App Store Version ID: {}", id);
        }
        if let Some(id) = rel_id(&rel.internal_beta_version) {
            let _ = writeln!(sb, "Internal Beta Version ID: {}", id);
        }
        if let Some(id) = rel_id(&rel.external_beta_version) {
            let _ = writeln!(sb, "External Beta Version ID: {}", id);
        }
    }
    sb
}

fn format_background_asset_versions(versions: &[BackgroundAssetVersion]) -> String {
    if versions.is_empty() {
        return "No background asset versions found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} background asset versions:\n\n", versions.len());

    for version in versions {
        sb.push_str(&format_background_asset_version(version));
        sb.push_str("\n---\n");
    }

    sb
}

fn format_background_asset_version(version: &BackgroundAssetVersion) -> String {
    let a = &version.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "Background Asset Version ID: {}", version.id);
    if !a.version.is_empty() {
        let _ = writeln!(sb, "Version: {}", a.version);
    }
    if !a.locale.is_empty() {
        let _ = writeln!(sb, "Locale: {}", a.locale);
    }
    if !a.platforms.is_empty() {
        let _ = writeln!(sb, "Platforms: {}", a.platforms.join(", "));
    }
    if !a.state.is_empty() {
        let _ = writeln!(sb, "State: {}", a.state);
    }
    if let Some(details) = &a.state_details {
        for detail in details.errors.iter() {
            let _ = writeln!(sb, "Error [{}]: {}", detail.code, detail.description);
        }
        for detail in details.warnings.iter() {
            let _ = writeln!(sb, "Warning [{}]: {}", detail.code, detail.description);
        }
        for detail in details.infos.iter() {
            let _ = writeln!(sb, "Info [{}]: {}", detail.code, detail.description);
        }
    }
    if let Some(d) = &a.created_date {
        let _ = writeln!(sb, "Created: {}", d.format("2006-01-02 15:04:05"));
    }
    if let Some(rel) = &version.relationships {
        if let Some(id) = rel_id(&rel.internal_beta_release) {
            let _ = writeln!(sb, "Internal Beta Release ID: {}", id);
        }
        if let Some(id) = rel_id(&rel.external_beta_release) {
            let _ = writeln!(sb, "External Beta Release ID: {}", id);
        }
        if let Some(id) = rel_id(&rel.app_store_release) {
            let _ = writeln!(sb, "App Store Release ID: {}", id);
        }
        if let Some(id) = rel_id(&rel.asset_file) {
            let _ = writeln!(sb, "Asset File ID: {}", id);
        }
        if let Some(id) = rel_id(&rel.manifest_file) {
            let _ = writeln!(sb, "Manifest File ID: {}", id);
        }
    }
    sb
}

fn format_background_asset_upload_files(files: &[BackgroundAssetUploadFile]) -> String {
    if files.is_empty() {
        return "No background asset upload files found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} background asset upload files:\n\n", files.len());

    for file in files {
        let a = &file.attributes;
        let _ = writeln!(sb, "File ID: {}", file.id);
        if !a.file_name.is_empty() {
            let _ = writeln!(sb, "Name: {} ({} bytes)", a.file_name, a.file_size);
        }
        if !a.asset_type.is_empty() {
            let _ = writeln!(sb, "Asset Type: {}", a.asset_type);
        }
        if let Some(st) = a.asset_delivery_state.as_ref().filter(|s| !s.state.is_empty()) {
            let _ = writeln!(sb, "Delivery State: {}", st.state);
        }
        sb.push_str("\n---\n");
    }

    sb
}

fn format_background_asset_version_release(release: &BackgroundAssetVersionRelease) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "Release ID: {}", release.id);
    if !release.type_.is_empty() {
        let _ = writeln!(sb, "Type: {}", release.type_);
    }
    if !release.attributes.state.is_empty() {
        let _ = writeln!(sb, "State: {}", release.attributes.state);
    }
    if let Some(id) = release.relationships.as_ref().and_then(|r| rel_id(&r.background_asset_version)) {
        let _ = writeln!(sb, "Background Asset Version ID: {}", id);
    }
    sb
}
