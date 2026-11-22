//! Ported from internal/asc/tools/background_assets.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerBackgroundAssetTools`.
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

/// Ported from `Registry.handleListBackgroundAssets`.
async fn handle_list_background_assets(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListBackgroundAssets from internal/asc/tools/background_assets.go")
}

/// Ported from `Registry.handleGetBackgroundAsset`.
async fn handle_get_background_asset(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BackgroundAssetID string \"json:\\\"background_asset_id\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        background_asset_id: String => "background_asset_id";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleGetBackgroundAsset from internal/asc/tools/background_assets.go")
}

/// Ported from `Registry.handleCreateBackgroundAsset`.
async fn handle_create_background_asset(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; AssetPackIdentifier string \"json:\\\"asset_pack_identifier\\\"\" }", "") {
        app_id: String => "app_id";
        asset_pack_identifier: String => "asset_pack_identifier";
    }
    }
    todo!("port handleCreateBackgroundAsset from internal/asc/tools/background_assets.go")
}

/// Ported from `Registry.handleUpdateBackgroundAsset`.
async fn handle_update_background_asset(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BackgroundAssetID string \"json:\\\"background_asset_id\\\"\"; Archived *bool \"json:\\\"archived\\\"\" }", "") {
        background_asset_id: String => "background_asset_id";
        archived: Option<bool> => "archived";
    }
    }
    todo!("port handleUpdateBackgroundAsset from internal/asc/tools/background_assets.go")
}

/// Ported from `Registry.handleListBackgroundAssetVersions`.
async fn handle_list_background_asset_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BackgroundAssetID string \"json:\\\"background_asset_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        background_asset_id: String => "background_asset_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListBackgroundAssetVersions from internal/asc/tools/background_assets.go")
}

/// Ported from `Registry.handleGetBackgroundAssetVersion`.
async fn handle_get_background_asset_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        version_id: String => "version_id";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleGetBackgroundAssetVersion from internal/asc/tools/background_assets.go")
}

/// Ported from `Registry.handleCreateBackgroundAssetVersion`.
async fn handle_create_background_asset_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BackgroundAssetID string \"json:\\\"background_asset_id\\\"\" }", "") {
        background_asset_id: String => "background_asset_id";
    }
    }
    todo!("port handleCreateBackgroundAssetVersion from internal/asc/tools/background_assets.go")
}

/// Ported from `Registry.handleUploadBackgroundAssetFile`.
async fn handle_upload_background_asset_file(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; FileName string \"json:\\\"file_name\\\"\"; AssetType string \"json:\\\"asset_type\\\"\"; FilePath string \"json:\\\"file_path\\\"\"; FileDataBase64 string \"json:\\\"file_data_base64\\\"\" }", "") {
        version_id: String => "version_id";
        file_name: String => "file_name";
        asset_type: String => "asset_type";
        file_path: String => "file_path";
        file_data_base64: String => "file_data_base64";
    }
    }
    todo!("port handleUploadBackgroundAssetFile from internal/asc/tools/background_assets.go")
}

/// Ported from `Registry.handleListBackgroundAssetUploadFiles`.
async fn handle_list_background_asset_upload_files(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\" }", "") {
        version_id: String => "version_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
    }
    }
    todo!("port handleListBackgroundAssetUploadFiles from internal/asc/tools/background_assets.go")
}

/// Ported from `Registry.handleGetBackgroundAssetVersionRelease`.
async fn handle_get_background_asset_version_release(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ReleaseType string \"json:\\\"release_type\\\"\"; ReleaseID string \"json:\\\"release_id\\\"\" }", "") {
        release_type: String => "release_type";
        release_id: String => "release_id";
    }
    }
    todo!("port handleGetBackgroundAssetVersionRelease from internal/asc/tools/background_assets.go")
}

