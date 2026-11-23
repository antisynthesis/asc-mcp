//! Export compliance (encryption declaration) tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
pub(super) fn register_encryption_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_encryption_declarations",
            "description": "List app encryption declarations",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "Filter by App ID (optional)"
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
                        "description": "Maximum number of declarations to return (default 50)"
                    },
                    "sort": {
                        "type": "array",
                        "description": "Sort fields; prefix with - for descending. Joined comma-separated for the sort query param.",
                        "items": {
                            "type": "string"
                        }
                    }
                }
            },
            "annotations": {
                "title": "List Encryption Declarations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_encryption_declarations),
    );
    r.register(
        tool(r##"{
            "name": "get_encryption_declaration",
            "description": "Get details of a specific encryption declaration",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "declaration_id": {
                        "type": "string",
                        "description": "The encryption declaration ID"
                    }
                },
                "required": [
                    "declaration_id"
                ]
            },
            "annotations": {
                "title": "Get Encryption Declaration",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_encryption_declaration),
    );
    r.register(
        tool(r##"{
            "name": "create_encryption_declaration",
            "description": "Create an encryption declaration for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_description": {
                        "type": "string",
                        "description": "Description of how the app uses encryption"
                    },
                    "app_id": {
                        "type": "string",
                        "description": "The App ID"
                    },
                    "available_on_french_store": {
                        "type": "boolean",
                        "description": "Whether the app is available on the French store"
                    },
                    "code_value": {
                        "type": "string",
                        "description": "CCATS code value if applicable"
                    },
                    "contains_proprietary_cryptography": {
                        "type": "boolean",
                        "description": "Whether the app contains proprietary cryptography"
                    },
                    "contains_third_party_cryptography": {
                        "type": "boolean",
                        "description": "Whether the app contains third-party cryptography"
                    },
                    "exempt": {
                        "type": "boolean",
                        "description": "Whether the app is exempt from export regulations"
                    },
                    "uses_encryption": {
                        "type": "boolean",
                        "description": "Whether the app uses encryption"
                    }
                },
                "required": [
                    "app_id",
                    "uses_encryption"
                ]
            },
            "annotations": {
                "title": "Create Encryption Declaration",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_encryption_declaration),
    );
    r.register(
        tool(r##"{
            "name": "assign_build_to_encryption_declaration",
            "description": "Assign a build to an encryption declaration",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "build_id": {
                        "type": "string",
                        "description": "The build ID to assign"
                    },
                    "declaration_id": {
                        "type": "string",
                        "description": "The encryption declaration ID"
                    }
                },
                "required": [
                    "declaration_id",
                    "build_id"
                ]
            },
            "annotations": {
                "title": "Assign Build To Encryption Declaration",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_assign_build_to_encryption_declaration),
    );
}

/// Handles `list_encryption_declarations`.
async fn handle_list_encryption_declarations(c: Arc<Client>, args: Args) -> HandlerResult {
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

    let mut limit = params.limit;
    if limit <= 0 {
        limit = 50;
    }
    if limit > 200 {
        limit = 200;
    }

    let opts = list_opts(limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_app_encryption_declarations(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list encryption declarations: {e}"))),
    };

    Ok(list_result(&format_encryption_declarations(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_encryption_declaration`.
async fn handle_get_encryption_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { DeclarationID string \"json:\\\"declaration_id\\\"\" }", "") {
        declaration_id: String => "declaration_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.declaration_id.is_empty() {
        return Ok(error_result("declaration_id is required"));
    }

    let resp = match c.get_app_encryption_declaration(&params.declaration_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get encryption declaration: {e}"))),
    };

    Ok(data_result(&format_encryption_declaration(&resp.data), &resp.data))
}

/// Handles `create_encryption_declaration`.
async fn handle_create_encryption_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; UsesEncryption bool \"json:\\\"uses_encryption\\\"\"; Exempt bool \"json:\\\"exempt\\\"\"; ContainsProprietaryCryptography bool \"json:\\\"contains_proprietary_cryptography\\\"\"; ContainsThirdPartyCryptography bool \"json:\\\"contains_third_party_cryptography\\\"\"; AvailableOnFrenchStore bool \"json:\\\"available_on_french_store\\\"\"; AppDescription string \"json:\\\"app_description\\\"\"; CodeValue string \"json:\\\"code_value\\\"\" }", "") {
        app_id: String => "app_id";
        uses_encryption: bool => "uses_encryption";
        exempt: bool => "exempt";
        contains_proprietary_cryptography: bool => "contains_proprietary_cryptography";
        contains_third_party_cryptography: bool => "contains_third_party_cryptography";
        available_on_french_store: bool => "available_on_french_store";
        app_description: String => "app_description";
        code_value: String => "code_value";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }

    let req = AppEncryptionDeclarationCreateRequest {
        data: AppEncryptionDeclarationCreateData {
            type_: "appEncryptionDeclarations".into(),
            attributes: AppEncryptionDeclarationCreateAttributes {
                uses_encryption: params.uses_encryption,
                exempt: params.exempt,
                contains_proprietary_cryptography: params.contains_proprietary_cryptography,
                contains_third_party_cryptography: params.contains_third_party_cryptography,
                available_on_french_store: params.available_on_french_store,
                app_description: params.app_description.clone(),
                code_value: params.code_value.clone(),
            },
            relationships: AppEncryptionDeclarationCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
            },
        },
    };

    let resp = match c.create_app_encryption_declaration(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create encryption declaration: {e}"))),
    };

    Ok(data_result(&format!("Created encryption declaration: {}", resp.data.id), &resp.data))
}

/// Handles `assign_build_to_encryption_declaration`.
async fn handle_assign_build_to_encryption_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { DeclarationID string \"json:\\\"declaration_id\\\"\"; BuildID string \"json:\\\"build_id\\\"\" }", "") {
        declaration_id: String => "declaration_id";
        build_id: String => "build_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.declaration_id.is_empty() {
        return Ok(error_result("declaration_id is required"));
    }
    if params.build_id.is_empty() {
        return Ok(error_result("build_id is required"));
    }

    if let Err(e) = c.assign_build_to_encryption_declaration(&params.declaration_id, &params.build_id).await {
        return Ok(error_result(format!("Failed to assign build to encryption declaration: {e}")));
    }

    Ok(CallToolResult::success("Build assigned to encryption declaration successfully"))
}

fn format_encryption_declarations(declarations: &[AppEncryptionDeclaration]) -> String {
    if declarations.is_empty() {
        return "No encryption declarations found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} encryption declarations:\n\n", declarations.len());

    for decl in declarations {
        sb.push_str(&format_encryption_declaration(decl));
        sb.push_str("\n---\n");
    }

    sb
}

fn format_encryption_declaration(decl: &AppEncryptionDeclaration) -> String {
    let a = &decl.attributes;
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", decl.id);
    let _ = writeln!(sb, "Uses Encryption: {}", a.uses_encryption);
    let _ = writeln!(sb, "Exempt: {}", a.exempt);
    let _ = writeln!(sb, "Contains Proprietary Cryptography: {}", a.contains_proprietary_cryptography);
    let _ = writeln!(sb, "Contains Third-Party Cryptography: {}", a.contains_third_party_cryptography);
    let _ = writeln!(sb, "Available on French Store: {}", a.available_on_french_store);
    let _ = writeln!(sb, "Platform: {}", a.platform);
    let _ = writeln!(sb, "State: {}", a.app_encryption_declaration_state);
    if !a.app_description.is_empty() {
        let _ = writeln!(sb, "Description: {}", a.app_description);
    }
    if !a.code_value.is_empty() {
        let _ = writeln!(sb, "CCATS Code: {}", a.code_value);
    }
    if !a.document_url.is_empty() {
        let _ = writeln!(sb, "Document URL: {}", a.document_url);
    }
    sb
}
