//! Ported from internal/asc/tools/provisioning.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerProvisioningTools`.
pub(super) fn register_provisioning_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_bundle_ids",
            "description": "List all registered bundle IDs in your App Store Connect account. Returns bundle identifier, name, platform, and seed ID.",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                        "description": "Maximum number of bundle IDs to return (default: 50)",
                        "default": 50
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
                "title": "List Bundle Ids",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_bundle_ids),
    );
    r.register(
        tool(r##"{
            "name": "get_bundle_id",
            "description": "Get detailed information about a specific bundle ID.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "bundle_id_id": {
                        "type": "string",
                        "description": "The App Store Connect ID of the bundle ID resource (not the bundle identifier string)"
                    }
                },
                "required": [
                    "bundle_id_id"
                ]
            },
            "annotations": {
                "title": "Get Bundle Id",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_bundle_id),
    );
    r.register(
        tool(r##"{
            "name": "list_certificates",
            "description": "List all signing certificates in your App Store Connect account. Returns certificate name, type, serial number, and expiration date.",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                        "description": "Maximum number of certificates to return (default: 50)",
                        "default": 50
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
                "title": "List Certificates",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_certificates),
    );
    r.register(
        tool(r##"{
            "name": "list_profiles",
            "description": "List all provisioning profiles in your App Store Connect account. Returns profile name, type, state, UUID, and expiration date.",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                        "description": "Maximum number of profiles to return (default: 50)",
                        "default": 50
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
                "title": "List Profiles",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_profiles),
    );
    r.register(
        tool(r##"{
            "name": "list_devices",
            "description": "List all registered devices in your App Store Connect account. Returns device name, UDID, model, platform, and status.",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                        "description": "Maximum number of devices to return (default: 50)",
                        "default": 50
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
                "title": "List Devices",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_devices),
    );
    r.register(
        tool(r##"{
            "name": "register_device",
            "description": "Register a new device for development or ad hoc distribution.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "A name for the device (e.g., 'John's iPhone 15')"
                    },
                    "platform": {
                        "type": "string",
                        "description": "The device platform",
                        "enum": [
                            "IOS",
                            "MAC_OS"
                        ]
                    },
                    "udid": {
                        "type": "string",
                        "description": "The device's UDID"
                    }
                },
                "required": [
                    "name",
                    "udid",
                    "platform"
                ]
            },
            "annotations": {
                "title": "Register Device",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_register_device),
    );
}

/// Ported from `Registry.handleListBundleIDs`.
async fn handle_list_bundle_ids(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    let mut params = Params { limit: 50, ..Default::default() };
    if args.is_some() {
        if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
            return Err(format!("invalid arguments: {e}"));
        }
    }

    let opts = list_opts(params.limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_bundle_ids(opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list bundle IDs: {e}"))),
    };

    if resp.data.is_empty() {
        return Ok(list_result("No bundle IDs found.", &resp.data, &resp.links));
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} bundle IDs:\n\n", resp.data.len());
    for bundle_id in resp.data.iter() {
        let a = &bundle_id.attributes;
        let _ = write!(sb, "**{}**\n", a.name);
        let _ = write!(sb, "  - ID: {}\n", bundle_id.id);
        let _ = write!(sb, "  - Identifier: {}\n", a.identifier);
        let _ = write!(sb, "  - Platform: {}\n", a.platform);
        if !a.seed_id.is_empty() {
            let _ = write!(sb, "  - Seed ID: {}\n", a.seed_id);
        }
        sb.push('\n');
    }

    Ok(list_result(&sb, &resp.data, &resp.links))
}

/// Ported from `Registry.handleGetBundleID`.
async fn handle_get_bundle_id(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BundleIDID string \"json:\\\"bundle_id_id\\\"\" }", "") {
        bundle_id_id: String => "bundle_id_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.bundle_id_id.is_empty() {
        return Ok(error_result("bundle_id_id is required"));
    }

    let resp = match c.get_bundle_id(&params.bundle_id_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get bundle ID: {e}"))),
    };

    let bundle_id = &resp.data;
    let a = &bundle_id.attributes;
    let mut sb = String::new();
    let _ = write!(sb, "**{}**\n\n", a.name);
    let _ = write!(sb, "- ID: {}\n", bundle_id.id);
    let _ = write!(sb, "- Identifier: {}\n", a.identifier);
    let _ = write!(sb, "- Platform: {}\n", a.platform);
    if !a.seed_id.is_empty() {
        let _ = write!(sb, "- Seed ID: {}\n", a.seed_id);
    }

    Ok(data_result(&sb, &resp.data))
}

/// Ported from `Registry.handleListCertificates`.
async fn handle_list_certificates(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    let mut params = Params { limit: 50, ..Default::default() };
    if args.is_some() {
        if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
            return Err(format!("invalid arguments: {e}"));
        }
    }

    let opts = list_opts(params.limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_certificates(opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list certificates: {e}"))),
    };

    if resp.data.is_empty() {
        return Ok(list_result("No certificates found.", &resp.data, &resp.links));
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} certificates:\n\n", resp.data.len());
    for cert in resp.data.iter() {
        let a = &cert.attributes;
        let mut display_name = a.display_name.clone();
        if display_name.is_empty() {
            display_name = a.name.clone();
        }
        let _ = write!(sb, "**{}**\n", display_name);
        let _ = write!(sb, "  - ID: {}\n", cert.id);
        let _ = write!(sb, "  - Type: {}\n", a.certificate_type);
        let _ = write!(sb, "  - Serial Number: {}\n", a.serial_number);
        if !a.platform.is_empty() {
            let _ = write!(sb, "  - Platform: {}\n", a.platform);
        }
        if let Some(d) = &a.expiration_date {
            let _ = write!(sb, "  - Expires: {}\n", d.format("2006-01-02"));
        }
        sb.push('\n');
    }

    Ok(list_result(&sb, &resp.data, &resp.links))
}

/// Ported from `Registry.handleListProfiles`.
async fn handle_list_profiles(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    let mut params = Params { limit: 50, ..Default::default() };
    if args.is_some() {
        if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
            return Err(format!("invalid arguments: {e}"));
        }
    }

    let opts = list_opts(params.limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_profiles(opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list profiles: {e}"))),
    };

    if resp.data.is_empty() {
        return Ok(list_result("No provisioning profiles found.", &resp.data, &resp.links));
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} provisioning profiles:\n\n", resp.data.len());
    for profile in resp.data.iter() {
        let a = &profile.attributes;
        let _ = write!(sb, "**{}**\n", a.name);
        let _ = write!(sb, "  - ID: {}\n", profile.id);
        let _ = write!(sb, "  - UUID: {}\n", a.uuid);
        let _ = write!(sb, "  - Type: {}\n", a.profile_type);
        let _ = write!(sb, "  - State: {}\n", a.profile_state);
        let _ = write!(sb, "  - Platform: {}\n", a.platform);
        if let Some(d) = &a.created_date {
            let _ = write!(sb, "  - Created: {}\n", d.format("2006-01-02"));
        }
        if let Some(d) = &a.expiration_date {
            let _ = write!(sb, "  - Expires: {}\n", d.format("2006-01-02"));
        }
        sb.push('\n');
    }

    Ok(list_result(&sb, &resp.data, &resp.links))
}

/// Ported from `Registry.handleListDevices`.
async fn handle_list_devices(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    let mut params = Params { limit: 50, ..Default::default() };
    if args.is_some() {
        if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
            return Err(format!("invalid arguments: {e}"));
        }
    }

    let opts = list_opts(params.limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_devices(opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list devices: {e}"))),
    };

    if resp.data.is_empty() {
        return Ok(list_result("No devices found.", &resp.data, &resp.links));
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} devices:\n\n", resp.data.len());
    for device in resp.data.iter() {
        let a = &device.attributes;
        let _ = write!(sb, "**{}**\n", a.name);
        let _ = write!(sb, "  - ID: {}\n", device.id);
        let _ = write!(sb, "  - UDID: {}\n", a.udid);
        let _ = write!(sb, "  - Model: {}\n", a.model);
        let _ = write!(sb, "  - Device Class: {}\n", a.device_class);
        let _ = write!(sb, "  - Platform: {}\n", a.platform);
        let _ = write!(sb, "  - Status: {}\n", a.status);
        if let Some(d) = &a.added_date {
            let _ = write!(sb, "  - Added: {}\n", d.format("2006-01-02"));
        }
        sb.push('\n');
    }

    Ok(list_result(&sb, &resp.data, &resp.links))
}

/// Ported from `Registry.handleRegisterDevice`.
async fn handle_register_device(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { Name string \"json:\\\"name\\\"\"; UDID string \"json:\\\"udid\\\"\"; Platform string \"json:\\\"platform\\\"\" }", "") {
        name: String => "name";
        udid: String => "udid";
        platform: String => "platform";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.name.is_empty() {
        return Ok(error_result("name is required"));
    }
    if params.udid.is_empty() {
        return Ok(error_result("udid is required"));
    }
    if params.platform.is_empty() {
        return Ok(error_result("platform is required"));
    }

    let req = DeviceCreateRequest {
        data: DeviceCreateData {
            type_: "devices".into(),
            attributes: DeviceCreateAttributes {
                name: params.name.clone(),
                udid: params.udid.clone(),
                platform: params.platform.clone(),
            },
        },
    };

    let resp = match c.register_device(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to register device: {e}"))),
    };

    let a = &resp.data.attributes;
    let mut sb = String::new();
    let _ = write!(sb, "Successfully registered device **{}**\n\n", a.name);
    let _ = write!(sb, "- ID: {}\n", resp.data.id);
    let _ = write!(sb, "- UDID: {}\n", a.udid);
    let _ = write!(sb, "- Model: {}\n", a.model);
    let _ = write!(sb, "- Platform: {}\n", a.platform);
    let _ = write!(sb, "- Status: {}\n", a.status);

    Ok(data_result(&sb, &resp.data))
}

