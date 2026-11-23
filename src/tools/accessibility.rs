//! Accessibility declaration tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
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

json_struct! {
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

/// Handles `list_accessibility_declarations`.
async fn handle_list_accessibility_declarations(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\" }", "") {
        app_id: String => "app_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        fields: JsonMap<JsonSlice<String>> => "fields";
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

    let opts = list_opts(limit, &params.filter, &JsonSlice::nil(), &params.fields, &JsonSlice::nil());
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_accessibility_declarations(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list accessibility declarations: {e}"))),
    };

    Ok(list_result(&format_accessibility_declarations(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_accessibility_declaration`.
async fn handle_get_accessibility_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
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

    let resp = match c.get_accessibility_declaration(&params.declaration_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get accessibility declaration: {e}"))),
    };

    Ok(data_result(&format_accessibility_declaration(&resp.data), &resp.data))
}

/// Handles `create_accessibility_declaration`.
async fn handle_create_accessibility_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; DeviceFamily string \"json:\\\"device_family\\\"\"; tools.accessibilitySupportParams }", "") {
        app_id: String => "app_id";
        device_family: String => "device_family";
        accessibility_support_params: AccessibilitySupportParams => "accessibilitySupportParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }

    if params.device_family.is_empty() {
        return Ok(error_result("device_family is required"));
    }

    let s = &params.accessibility_support_params;
    let req = AccessibilityDeclarationCreateRequest {
        data: AccessibilityDeclarationCreateData {
            type_: "accessibilityDeclarations".into(),
            attributes: AccessibilityDeclarationCreateAttributes {
                device_family: params.device_family.clone(),
                supports_audio_descriptions: s.supports_audio_descriptions,
                supports_captions: s.supports_captions,
                supports_dark_interface: s.supports_dark_interface,
                supports_differentiate_without_color_alone: s.supports_differentiate_without_color_alone,
                supports_larger_text: s.supports_larger_text,
                supports_reduced_motion: s.supports_reduced_motion,
                supports_sufficient_contrast: s.supports_sufficient_contrast,
                supports_voice_control: s.supports_voice_control,
                supports_voiceover: s.supports_voiceover,
            },
            relationships: AccessibilityDeclarationCreateRelationships {
                app: RelationshipData { data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() } },
            },
        },
    };

    let resp = match c.create_accessibility_declaration(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create accessibility declaration: {e}"))),
    };

    Ok(data_result(&format!("Created accessibility declaration:\n{}", format_accessibility_declaration(&resp.data)), &resp.data))
}

/// Handles `update_accessibility_declaration`.
async fn handle_update_accessibility_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { DeclarationID string \"json:\\\"declaration_id\\\"\"; Publish *bool \"json:\\\"publish\\\"\"; tools.accessibilitySupportParams }", "") {
        declaration_id: String => "declaration_id";
        publish: Option<bool> => "publish";
        accessibility_support_params: AccessibilitySupportParams => "accessibilitySupportParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.declaration_id.is_empty() {
        return Ok(error_result("declaration_id is required"));
    }

    let s = &params.accessibility_support_params;
    let req = AccessibilityDeclarationUpdateRequest {
        data: AccessibilityDeclarationUpdateData {
            type_: "accessibilityDeclarations".into(),
            id: params.declaration_id.clone(),
            attributes: Some(AccessibilityDeclarationUpdateAttributes {
                publish: params.publish,
                supports_audio_descriptions: s.supports_audio_descriptions,
                supports_captions: s.supports_captions,
                supports_dark_interface: s.supports_dark_interface,
                supports_differentiate_without_color_alone: s.supports_differentiate_without_color_alone,
                supports_larger_text: s.supports_larger_text,
                supports_reduced_motion: s.supports_reduced_motion,
                supports_sufficient_contrast: s.supports_sufficient_contrast,
                supports_voice_control: s.supports_voice_control,
                supports_voiceover: s.supports_voiceover,
            }),
        },
    };

    let resp = match c.update_accessibility_declaration(&params.declaration_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update accessibility declaration: {e}"))),
    };

    Ok(data_result(&format!("Accessibility declaration updated:\n{}", format_accessibility_declaration(&resp.data)), &resp.data))
}

/// Handles `delete_accessibility_declaration`.
async fn handle_delete_accessibility_declaration(c: Arc<Client>, args: Args) -> HandlerResult {
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

    if let Err(e) = c.delete_accessibility_declaration(&params.declaration_id).await {
        return Ok(error_result(format!("Failed to delete accessibility declaration: {e}")));
    }

    Ok(CallToolResult::success("Accessibility declaration deleted successfully"))
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

fn format_accessibility_declarations(declarations: &[AccessibilityDeclaration]) -> String {
    if declarations.is_empty() {
        return "No accessibility declarations found".into();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} accessibility declarations:\n\n", declarations.len());

    for decl in declarations {
        sb.push_str(&format_accessibility_declaration(decl));
        sb.push_str("\n---\n");
    }

    sb
}

fn format_accessibility_declaration(decl: &AccessibilityDeclaration) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "Declaration ID: {}", decl.id);

    let attrs = &decl.attributes;
    if !attrs.device_family.is_empty() {
        let _ = writeln!(sb, "Device Family: {}", attrs.device_family);
    }
    if !attrs.state.is_empty() {
        let _ = writeln!(sb, "State: {}", attrs.state);
    }

    let features: [(&str, Option<bool>); 9] = [
        ("Audio Descriptions", attrs.supports_audio_descriptions),
        ("Captions", attrs.supports_captions),
        ("Dark Interface", attrs.supports_dark_interface),
        ("Differentiate Without Color Alone", attrs.supports_differentiate_without_color_alone),
        ("Larger Text", attrs.supports_larger_text),
        ("Reduced Motion", attrs.supports_reduced_motion),
        ("Sufficient Contrast", attrs.supports_sufficient_contrast),
        ("Voice Control", attrs.supports_voice_control),
        ("VoiceOver", attrs.supports_voiceover),
    ];
    for (label, value) in features {
        if let Some(v) = value {
            let _ = writeln!(sb, "{}: {}", label, v);
        }
    }

    sb
}
