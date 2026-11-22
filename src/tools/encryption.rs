//! Ported from internal/asc/tools/encryption.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerEncryptionTools`.
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

/// Ported from `Registry.handleListEncryptionDeclarations`.
async fn handle_list_encryption_declarations(c: Arc<Client>, args: Args) -> HandlerResult {
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
    todo!("port handleListEncryptionDeclarations from internal/asc/tools/encryption.go")
}

/// Ported from `Registry.handleGetEncryptionDeclaration`.
async fn handle_get_encryption_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { DeclarationID string \"json:\\\"declaration_id\\\"\" }", "") {
        declaration_id: String => "declaration_id";
    }
    }
    todo!("port handleGetEncryptionDeclaration from internal/asc/tools/encryption.go")
}

/// Ported from `Registry.handleCreateEncryptionDeclaration`.
async fn handle_create_encryption_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
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
    todo!("port handleCreateEncryptionDeclaration from internal/asc/tools/encryption.go")
}

/// Ported from `Registry.handleAssignBuildToEncryptionDeclaration`.
async fn handle_assign_build_to_encryption_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { DeclarationID string \"json:\\\"declaration_id\\\"\"; BuildID string \"json:\\\"build_id\\\"\" }", "") {
        declaration_id: String => "declaration_id";
        build_id: String => "build_id";
    }
    }
    todo!("port handleAssignBuildToEncryptionDeclaration from internal/asc/tools/encryption.go")
}

