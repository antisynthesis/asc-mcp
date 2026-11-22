//! Ported from internal/asc/tools/accessibility.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerAccessibilityTools`.
pub(super) fn register_accessibility_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_accessibility_declarations",
            "description": "List the per-device-family accessibility declarations for an app",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to list accessibility declarations for"
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
                        "description": "JSON:API filter map. Supported keys: deviceFamily (IPHONE, IPAD, APPLE_TV, APPLE_WATCH, MAC, VISION), state (DRAFT, PUBLISHED, REPLACED). Values are arrays, e.g. {\"state\": [\"PUBLISHED\"]}."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of declarations to return (default 50, max 200)"
                    }
                },
                "required": [
                    "app_id"
                ]
            },
            "annotations": {
                "title": "List Accessibility Declarations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_accessibility_declarations),
    );
    r.register(
        tool(r##"{
            "name": "get_accessibility_declaration",
            "description": "Get details of a specific accessibility declaration",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "declaration_id": {
                        "type": "string",
                        "description": "The accessibility declaration ID"
                    }
                },
                "required": [
                    "declaration_id"
                ]
            },
            "annotations": {
                "title": "Get Accessibility Declaration",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_accessibility_declaration),
    );
    r.register(
        tool(r##"{
            "name": "create_accessibility_declaration",
            "description": "Create a draft accessibility declaration for an app and device family",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App ID to create the declaration for"
                    },
                    "device_family": {
                        "type": "string",
                        "description": "Device family: IPHONE, IPAD, APPLE_TV, APPLE_WATCH, MAC, or VISION"
                    },
                    "supports_audio_descriptions": {
                        "type": "boolean",
                        "description": "Whether the app supports audio descriptions"
                    },
                    "supports_captions": {
                        "type": "boolean",
                        "description": "Whether the app supports captions"
                    },
                    "supports_dark_interface": {
                        "type": "boolean",
                        "description": "Whether the app supports a dark interface"
                    },
                    "supports_differentiate_without_color_alone": {
                        "type": "boolean",
                        "description": "Whether the app differentiates content without relying on color alone"
                    },
                    "supports_larger_text": {
                        "type": "boolean",
                        "description": "Whether the app supports larger text"
                    },
                    "supports_reduced_motion": {
                        "type": "boolean",
                        "description": "Whether the app supports reduced motion"
                    },
                    "supports_sufficient_contrast": {
                        "type": "boolean",
                        "description": "Whether the app provides sufficient contrast"
                    },
                    "supports_voice_control": {
                        "type": "boolean",
                        "description": "Whether the app supports Voice Control"
                    },
                    "supports_voiceover": {
                        "type": "boolean",
                        "description": "Whether the app supports VoiceOver"
                    }
                },
                "required": [
                    "app_id",
                    "device_family"
                ]
            },
            "annotations": {
                "title": "Create Accessibility Declaration",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_accessibility_declaration),
    );
    r.register(
        tool(r##"{
            "name": "update_accessibility_declaration",
            "description": "Update a draft accessibility declaration; set publish=true to publish it to the App Store",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "declaration_id": {
                        "type": "string",
                        "description": "The accessibility declaration ID"
                    },
                    "publish": {
                        "type": "boolean",
                        "description": "Set true to publish the draft declaration to the App Store"
                    },
                    "supports_audio_descriptions": {
                        "type": "boolean",
                        "description": "Whether the app supports audio descriptions"
                    },
                    "supports_captions": {
                        "type": "boolean",
                        "description": "Whether the app supports captions"
                    },
                    "supports_dark_interface": {
                        "type": "boolean",
                        "description": "Whether the app supports a dark interface"
                    },
                    "supports_differentiate_without_color_alone": {
                        "type": "boolean",
                        "description": "Whether the app differentiates content without relying on color alone"
                    },
                    "supports_larger_text": {
                        "type": "boolean",
                        "description": "Whether the app supports larger text"
                    },
                    "supports_reduced_motion": {
                        "type": "boolean",
                        "description": "Whether the app supports reduced motion"
                    },
                    "supports_sufficient_contrast": {
                        "type": "boolean",
                        "description": "Whether the app provides sufficient contrast"
                    },
                    "supports_voice_control": {
                        "type": "boolean",
                        "description": "Whether the app supports Voice Control"
                    },
                    "supports_voiceover": {
                        "type": "boolean",
                        "description": "Whether the app supports VoiceOver"
                    }
                },
                "required": [
                    "declaration_id"
                ]
            },
            "annotations": {
                "title": "Update Accessibility Declaration",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_accessibility_declaration),
    );
    r.register(
        tool(r##"{
            "name": "delete_accessibility_declaration",
            "description": "Delete a draft accessibility declaration",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "declaration_id": {
                        "type": "string",
                        "description": "The accessibility declaration ID"
                    }
                },
                "required": [
                    "declaration_id"
                ]
            },
            "annotations": {
                "title": "Delete Accessibility Declaration",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_accessibility_declaration),
    );
}

go_struct! {
pub(crate) struct AccessibilitySupportParams ("tools.accessibilitySupportParams", "accessibilitySupportParams") {
    pub(crate) supports_audio_descriptions: Option<bool> => "supports_audio_descriptions";
    pub(crate) supports_captions: Option<bool> => "supports_captions";
    pub(crate) supports_dark_interface: Option<bool> => "supports_dark_interface";
    pub(crate) supports_differentiate_without_color_alone: Option<bool> => "supports_differentiate_without_color_alone";
    pub(crate) supports_larger_text: Option<bool> => "supports_larger_text";
    pub(crate) supports_reduced_motion: Option<bool> => "supports_reduced_motion";
    pub(crate) supports_sufficient_contrast: Option<bool> => "supports_sufficient_contrast";
    pub(crate) supports_voice_control: Option<bool> => "supports_voice_control";
    pub(crate) supports_voiceover: Option<bool> => "supports_voiceover";
}
}

/// Ported from `Registry.handleListAccessibilityDeclarations`.
async fn handle_list_accessibility_declarations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        fields: GoMap<GoSlice<String>> => "fields";
    }
    }
    todo!("port handleListAccessibilityDeclarations from internal/asc/tools/accessibility.go")
}

/// Ported from `Registry.handleGetAccessibilityDeclaration`.
async fn handle_get_accessibility_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { DeclarationID string \"json:\\\"declaration_id\\\"\" }", "") {
        declaration_id: String => "declaration_id";
    }
    }
    todo!("port handleGetAccessibilityDeclaration from internal/asc/tools/accessibility.go")
}

/// Ported from `Registry.handleCreateAccessibilityDeclaration`.
async fn handle_create_accessibility_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; DeviceFamily string \"json:\\\"device_family\\\"\"; tools.accessibilitySupportParams }", "") {
        app_id: String => "app_id";
        device_family: String => "device_family";
        accessibility_support_params: AccessibilitySupportParams => "accessibilitySupportParams", flatten;
    }
    }
    todo!("port handleCreateAccessibilityDeclaration from internal/asc/tools/accessibility.go")
}

/// Ported from `Registry.handleUpdateAccessibilityDeclaration`.
async fn handle_update_accessibility_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { DeclarationID string \"json:\\\"declaration_id\\\"\"; Publish *bool \"json:\\\"publish\\\"\"; tools.accessibilitySupportParams }", "") {
        declaration_id: String => "declaration_id";
        publish: Option<bool> => "publish";
        accessibility_support_params: AccessibilitySupportParams => "accessibilitySupportParams", flatten;
    }
    }
    todo!("port handleUpdateAccessibilityDeclaration from internal/asc/tools/accessibility.go")
}

/// Ported from `Registry.handleDeleteAccessibilityDeclaration`.
async fn handle_delete_accessibility_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { DeclarationID string \"json:\\\"declaration_id\\\"\" }", "") {
        declaration_id: String => "declaration_id";
    }
    }
    todo!("port handleDeleteAccessibilityDeclaration from internal/asc/tools/accessibility.go")
}

