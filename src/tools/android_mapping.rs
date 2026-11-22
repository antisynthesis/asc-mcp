//! Ported from internal/asc/tools/android_mapping.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerAndroidMappingTools`.
pub(super) fn register_android_mapping_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_android_to_ios_app_mappings",
            "description": "List the Android-to-iOS app mappings for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to list mappings for"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page."
                    },
                    "fields": {
                        "type": "object",
                        "description": "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of mappings to return (default 50, max 200)"
                    }
                },
                "required": [
                    "app_id"
                ]
            },
            "annotations": {
                "title": "List Android-to-iOS App Mappings",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_android_to_ios_app_mappings),
    );
    r.register(
        tool(r##"{
            "name": "get_android_to_ios_app_mapping",
            "description": "Get details of a specific Android-to-iOS app mapping",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "mapping_id": {
                        "type": "string",
                        "description": "The Android-to-iOS app mapping detail ID"
                    }
                },
                "required": [
                    "mapping_id"
                ]
            },
            "annotations": {
                "title": "Get Android-to-iOS App Mapping",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_android_to_ios_app_mapping),
    );
    r.register(
        tool(r##"{
            "name": "create_android_to_ios_app_mapping",
            "description": "Map an Android package to an iOS app so users migrating platforms are directed to it",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to map the Android package to"
                    },
                    "package_name": {
                        "type": "string",
                        "description": "The Android package name, e.g. com.example.app"
                    },
                    "sha256_fingerprints": {
                        "type": "array",
                        "description": "SHA-256 fingerprints of the Android app signing key's public certificates",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "app_id",
                    "package_name",
                    "sha256_fingerprints"
                ]
            },
            "annotations": {
                "title": "Create Android-to-iOS App Mapping",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_android_to_ios_app_mapping),
    );
    r.register(
        tool(r##"{
            "name": "update_android_to_ios_app_mapping",
            "description": "Update an Android-to-iOS app mapping's package name or signing key fingerprints",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "mapping_id": {
                        "type": "string",
                        "description": "The Android-to-iOS app mapping detail ID"
                    },
                    "package_name": {
                        "type": "string",
                        "description": "New Android package name"
                    },
                    "sha256_fingerprints": {
                        "type": "array",
                        "description": "New SHA-256 fingerprints of the Android app signing key's public certificates (replaces the existing list)",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "mapping_id"
                ]
            },
            "annotations": {
                "title": "Update Android-to-iOS App Mapping",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_android_to_ios_app_mapping),
    );
    r.register(
        tool(r##"{
            "name": "delete_android_to_ios_app_mapping",
            "description": "Delete an Android-to-iOS app mapping",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "mapping_id": {
                        "type": "string",
                        "description": "The Android-to-iOS app mapping detail ID"
                    }
                },
                "required": [
                    "mapping_id"
                ]
            },
            "annotations": {
                "title": "Delete Android-to-iOS App Mapping",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_android_to_ios_app_mapping),
    );
}

/// Ported from `Registry.handleListAndroidToIosAppMappings`.
async fn handle_list_android_to_ios_app_mappings(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        fields: GoMap<GoSlice<String>> => "fields";
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

    let opts = list_opts(limit, &GoMap::nil(), &GoSlice::nil(), &params.fields, &GoSlice::nil());
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_android_to_ios_app_mapping_details(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list Android-to-iOS app mappings: {e}"))),
    };

    Ok(list_result(&format_android_to_ios_app_mappings(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetAndroidToIosAppMapping`.
async fn handle_get_android_to_ios_app_mapping(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { MappingID string \"json:\\\"mapping_id\\\"\" }", "") {
        mapping_id: String => "mapping_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.mapping_id.is_empty() {
        return Ok(error_result("mapping_id is required"));
    }

    let resp = match c.get_android_to_ios_app_mapping_detail(&params.mapping_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get Android-to-iOS app mapping: {e}"))),
    };

    Ok(data_result(&format_android_to_ios_app_mapping(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreateAndroidToIosAppMapping`.
async fn handle_create_android_to_ios_app_mapping(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; PackageName string \"json:\\\"package_name\\\"\"; Sha256Fingerprints []string \"json:\\\"sha256_fingerprints\\\"\" }", "") {
        app_id: String => "app_id";
        package_name: String => "package_name";
        sha256_fingerprints: GoSlice<String> => "sha256_fingerprints";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }
    if params.package_name.is_empty() {
        return Ok(error_result("package_name is required"));
    }
    if params.sha256_fingerprints.is_empty() {
        return Ok(error_result("sha256_fingerprints is required"));
    }

    let req = AndroidToIosAppMappingDetailCreateRequest {
        data: AndroidToIosAppMappingDetailCreateData {
            type_: "androidToIosAppMappingDetails".into(),
            attributes: AndroidToIosAppMappingDetailCreateAttributes {
                package_name: params.package_name.clone(),
                app_signing_key_public_certificate_sha256_fingerprints: params.sha256_fingerprints.clone(),
            },
            relationships: AndroidToIosAppMappingDetailCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
            },
        },
    };

    let resp = match c.create_android_to_ios_app_mapping_detail(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create Android-to-iOS app mapping: {e}"))),
    };

    Ok(data_result(&format!("Created Android-to-iOS app mapping:\n{}", format_android_to_ios_app_mapping(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleUpdateAndroidToIosAppMapping`.
async fn handle_update_android_to_ios_app_mapping(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { MappingID string \"json:\\\"mapping_id\\\"\"; PackageName string \"json:\\\"package_name\\\"\"; Sha256Fingerprints []string \"json:\\\"sha256_fingerprints\\\"\" }", "") {
        mapping_id: String => "mapping_id";
        package_name: String => "package_name";
        sha256_fingerprints: GoSlice<String> => "sha256_fingerprints";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.mapping_id.is_empty() {
        return Ok(error_result("mapping_id is required"));
    }

    let req = AndroidToIosAppMappingDetailUpdateRequest {
        data: AndroidToIosAppMappingDetailUpdateData {
            type_: "androidToIosAppMappingDetails".into(),
            id: params.mapping_id.clone(),
            attributes: Some(AndroidToIosAppMappingDetailUpdateAttributes {
                package_name: params.package_name.clone(),
                app_signing_key_public_certificate_sha256_fingerprints: params.sha256_fingerprints.clone(),
            }),
        },
    };

    let resp = match c.update_android_to_ios_app_mapping_detail(&params.mapping_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update Android-to-iOS app mapping: {e}"))),
    };

    Ok(data_result(&format!("Android-to-iOS app mapping updated:\n{}", format_android_to_ios_app_mapping(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleDeleteAndroidToIosAppMapping`.
async fn handle_delete_android_to_ios_app_mapping(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { MappingID string \"json:\\\"mapping_id\\\"\" }", "") {
        mapping_id: String => "mapping_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.mapping_id.is_empty() {
        return Ok(error_result("mapping_id is required"));
    }

    if let Err(e) = c.delete_android_to_ios_app_mapping_detail(&params.mapping_id).await {
        return Ok(error_result(format!("Failed to delete Android-to-iOS app mapping: {e}")));
    }

    Ok(CallToolResult::success("Android-to-iOS app mapping deleted successfully"))
}

/// Ported from `formatAndroidToIosAppMappings`.
fn format_android_to_ios_app_mappings(mappings: &[AndroidToIosAppMappingDetail]) -> String {
    if mappings.is_empty() {
        return "No Android-to-iOS app mappings found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} Android-to-iOS app mappings:\n\n", mappings.len());

    for mapping in mappings {
        sb.push_str(&format_android_to_ios_app_mapping(mapping));
        sb.push_str("\n---\n");
    }

    sb
}

/// Ported from `formatAndroidToIosAppMapping`.
fn format_android_to_ios_app_mapping(mapping: &AndroidToIosAppMappingDetail) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "Mapping ID: {}", mapping.id);
    if !mapping.attributes.package_name.is_empty() {
        let _ = writeln!(sb, "Package Name: {}", mapping.attributes.package_name);
    }
    if !mapping.attributes.app_signing_key_public_certificate_sha256_fingerprints.is_empty() {
        let _ = writeln!(
            sb,
            "SHA-256 Fingerprints: {}",
            mapping.attributes.app_signing_key_public_certificate_sha256_fingerprints.join(", ")
        );
    }
    sb
}
