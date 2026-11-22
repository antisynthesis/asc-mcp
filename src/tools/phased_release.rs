//! Ported from internal/asc/tools/phased_release.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerPhasedReleaseTools`.
pub(super) fn register_phased_release_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "get_phased_release",
            "description": "Get phased release info for an App Store version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "version_id": {
                        "type": "string",
                        "description": "The App Store version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Get Phased Release",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_phased_release),
    );
    r.register(
        tool(r##"{
            "name": "create_phased_release",
            "description": "Enable phased release for an App Store version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "state": {
                        "type": "string",
                        "description": "Initial state (INACTIVE, ACTIVE)"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The App Store version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Create Phased Release",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_phased_release),
    );
    r.register(
        tool(r##"{
            "name": "update_phased_release",
            "description": "Update phased release (pause/resume)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "phased_release_id": {
                        "type": "string",
                        "description": "The phased release ID"
                    },
                    "state": {
                        "type": "string",
                        "description": "New state (INACTIVE, ACTIVE, PAUSED, COMPLETE)"
                    }
                },
                "required": [
                    "phased_release_id",
                    "state"
                ]
            },
            "annotations": {
                "title": "Update Phased Release",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_phased_release),
    );
    r.register(
        tool(r##"{
            "name": "delete_phased_release",
            "description": "Delete phased release (release to all users immediately)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "phased_release_id": {
                        "type": "string",
                        "description": "The phased release ID"
                    }
                },
                "required": [
                    "phased_release_id"
                ]
            },
            "annotations": {
                "title": "Delete Phased Release",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_phased_release),
    );
}

/// Ported from `Registry.handleGetPhasedRelease`.
async fn handle_get_phased_release(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    todo!("port handleGetPhasedRelease from internal/asc/tools/phased_release.go")
}

/// Ported from `Registry.handleCreatePhasedRelease`.
async fn handle_create_phased_release(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; State string \"json:\\\"state\\\"\" }", "") {
        version_id: String => "version_id";
        state: String => "state";
    }
    }
    todo!("port handleCreatePhasedRelease from internal/asc/tools/phased_release.go")
}

/// Ported from `Registry.handleUpdatePhasedRelease`.
async fn handle_update_phased_release(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PhasedReleaseID string \"json:\\\"phased_release_id\\\"\"; State string \"json:\\\"state\\\"\" }", "") {
        phased_release_id: String => "phased_release_id";
        state: String => "state";
    }
    }
    todo!("port handleUpdatePhasedRelease from internal/asc/tools/phased_release.go")
}

/// Ported from `Registry.handleDeletePhasedRelease`.
async fn handle_delete_phased_release(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PhasedReleaseID string \"json:\\\"phased_release_id\\\"\" }", "") {
        phased_release_id: String => "phased_release_id";
    }
    }
    todo!("port handleDeletePhasedRelease from internal/asc/tools/phased_release.go")
}

