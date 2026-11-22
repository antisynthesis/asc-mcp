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
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    let resp = match c.get_app_store_version_phased_release(&params.version_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get phased release: {e}"))),
    };

    Ok(data_result(&format_phased_release(&resp.data), &resp.data))
}

/// Ported from `Registry.handleCreatePhasedRelease`.
async fn handle_create_phased_release(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; State string \"json:\\\"state\\\"\" }", "") {
        version_id: String => "version_id";
        state: String => "state";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }

    let req = AppStoreVersionPhasedReleaseCreateRequest {
        data: AppStoreVersionPhasedReleaseCreateData {
            type_: "appStoreVersionPhasedReleases".into(),
            attributes: AppStoreVersionPhasedReleaseCreateAttributes { phased_release_state: params.state.clone() },
            relationships: AppStoreVersionPhasedReleaseCreateRelationships {
                app_store_version: RelationshipData {
                    data: ResourceIdentifier { type_: "appStoreVersions".into(), id: params.version_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_app_store_version_phased_release(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create phased release: {e}"))),
    };

    Ok(data_result(
        &format!("Created phased release: {} (state: {})", resp.data.id, resp.data.attributes.phased_release_state),
        &resp.data,
    ))
}

/// Ported from `Registry.handleUpdatePhasedRelease`.
async fn handle_update_phased_release(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PhasedReleaseID string \"json:\\\"phased_release_id\\\"\"; State string \"json:\\\"state\\\"\" }", "") {
        phased_release_id: String => "phased_release_id";
        state: String => "state";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.phased_release_id.is_empty() {
        return Ok(error_result("phased_release_id is required"));
    }
    if params.state.is_empty() {
        return Ok(error_result("state is required"));
    }

    let req = AppStoreVersionPhasedReleaseUpdateRequest {
        data: AppStoreVersionPhasedReleaseUpdateData {
            type_: "appStoreVersionPhasedReleases".into(),
            id: params.phased_release_id.clone(),
            attributes: AppStoreVersionPhasedReleaseUpdateAttributes { phased_release_state: params.state.clone() },
        },
    };

    let resp = match c.update_app_store_version_phased_release(&params.phased_release_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update phased release: {e}"))),
    };

    Ok(data_result(
        &format!("Updated phased release: {} (state: {})", resp.data.id, resp.data.attributes.phased_release_state),
        &resp.data,
    ))
}

/// Ported from `Registry.handleDeletePhasedRelease`.
async fn handle_delete_phased_release(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { PhasedReleaseID string \"json:\\\"phased_release_id\\\"\" }", "") {
        phased_release_id: String => "phased_release_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.phased_release_id.is_empty() {
        return Ok(error_result("phased_release_id is required"));
    }

    if let Err(e) = c.delete_app_store_version_phased_release(&params.phased_release_id).await {
        return Ok(error_result(format!("Failed to delete phased release: {e}")));
    }

    Ok(CallToolResult::success("Phased release deleted - app will release to all users"))
}

/// Ported from `formatPhasedRelease`.
fn format_phased_release(pr: &AppStoreVersionPhasedRelease) -> String {
    let mut result = format!("Phased Release ID: {}\n", pr.id);
    let _ = writeln!(result, "State: {}", pr.attributes.phased_release_state);
    if let Some(d) = &pr.attributes.start_date {
        let _ = writeln!(result, "Start Date: {}", d.format("2006-01-02"));
    }
    let _ = writeln!(result, "Current Day: {}", pr.attributes.current_day_number);
    let _ = writeln!(result, "Total Pause Duration: {} days", pr.attributes.total_pause_duration);
    result
}
