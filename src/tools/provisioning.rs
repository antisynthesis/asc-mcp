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
    todo!("port handleListBundleIDs from internal/asc/tools/provisioning.go")
}

/// Ported from `Registry.handleGetBundleID`.
async fn handle_get_bundle_id(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BundleIDID string \"json:\\\"bundle_id_id\\\"\" }", "") {
        bundle_id_id: String => "bundle_id_id";
    }
    }
    todo!("port handleGetBundleID from internal/asc/tools/provisioning.go")
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
    todo!("port handleListCertificates from internal/asc/tools/provisioning.go")
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
    todo!("port handleListProfiles from internal/asc/tools/provisioning.go")
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
    todo!("port handleListDevices from internal/asc/tools/provisioning.go")
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
    todo!("port handleRegisterDevice from internal/asc/tools/provisioning.go")
}

