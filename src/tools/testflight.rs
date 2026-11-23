//! TestFlight group and tester tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
pub(super) fn register_test_flight_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_beta_groups",
            "description": "List TestFlight beta groups. Can filter by app ID. Returns group name, tester counts, and public link information.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "Optional: Filter beta groups by app ID"
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
                        "description": "Maximum number of beta groups to return (default: 50)",
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
                "title": "List Beta Groups",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_beta_groups),
    );
    r.register(
        tool(r##"{
            "name": "create_beta_group",
            "description": "Create a new TestFlight beta group for an app.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "app_id": {
                        "type": "string",
                        "description": "The App Store Connect ID of the app"
                    },
                    "feedback_enabled": {
                        "type": "boolean",
                        "description": "Whether to enable feedback for the group (default: true)"
                    },
                    "name": {
                        "type": "string",
                        "description": "Name for the beta group"
                    },
                    "public_link_enabled": {
                        "type": "boolean",
                        "description": "Whether to enable public link for the group (default: false)"
                    }
                },
                "required": [
                    "app_id",
                    "name"
                ]
            },
            "annotations": {
                "title": "Create Beta Group",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_beta_group),
    );
    r.register(
        tool(r##"{
            "name": "delete_beta_group",
            "description": "Delete a TestFlight beta group.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "beta_group_id": {
                        "type": "string",
                        "description": "The App Store Connect ID of the beta group to delete"
                    }
                },
                "required": [
                    "beta_group_id"
                ]
            },
            "annotations": {
                "title": "Delete Beta Group",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_beta_group),
    );
    r.register(
        tool(r##"{
            "name": "list_beta_testers",
            "description": "List TestFlight beta testers. Can filter by beta group ID. Returns tester email, name, invite status, and state.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "beta_group_id": {
                        "type": "string",
                        "description": "Optional: Filter testers by beta group ID"
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
                        "description": "Maximum number of testers to return (default: 50)",
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
                "title": "List Beta Testers",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_beta_testers),
    );
    r.register(
        tool(r##"{
            "name": "invite_beta_tester",
            "description": "Invite a new beta tester to TestFlight, optionally adding them to specific beta groups.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "beta_group_ids": {
                        "type": "array",
                        "description": "Optional: IDs of beta groups to add the tester to",
                        "items": {
                            "type": "string"
                        }
                    },
                    "email": {
                        "type": "string",
                        "description": "Email address of the tester to invite"
                    },
                    "first_name": {
                        "type": "string",
                        "description": "First name of the tester (optional)"
                    },
                    "last_name": {
                        "type": "string",
                        "description": "Last name of the tester (optional)"
                    }
                },
                "required": [
                    "email"
                ]
            },
            "annotations": {
                "title": "Invite Beta Tester",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_invite_beta_tester),
    );
    r.register(
        tool(r##"{
            "name": "remove_beta_tester",
            "description": "Remove a beta tester from TestFlight.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "beta_tester_id": {
                        "type": "string",
                        "description": "The App Store Connect ID of the beta tester to remove"
                    }
                },
                "required": [
                    "beta_tester_id"
                ]
            },
            "annotations": {
                "title": "Remove Beta Tester",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_remove_beta_tester),
    );
    r.register(
        tool(r##"{
            "name": "add_tester_to_group",
            "description": "Add an existing beta tester to a beta group.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "beta_group_id": {
                        "type": "string",
                        "description": "The App Store Connect ID of the beta group"
                    },
                    "beta_tester_id": {
                        "type": "string",
                        "description": "The App Store Connect ID of the beta tester"
                    }
                },
                "required": [
                    "beta_group_id",
                    "beta_tester_id"
                ]
            },
            "annotations": {
                "title": "Add Tester To Group",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_add_tester_to_group),
    );
}

/// Handles `list_beta_groups`.
async fn handle_list_beta_groups(c: Arc<Client>, args: Args) -> HandlerResult {
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
    let mut params = Params { limit: 50, ..Default::default() };
    if args.is_some()
        && let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
            return Err(format!("invalid arguments: {e}"));
        }

    let opts = list_opts(params.limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_beta_groups(&params.app_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list beta groups: {e}"))),
    };

    if resp.data.is_empty() {
        return Ok(list_result("No beta groups found.", &resp.data, &resp.links));
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} beta groups:\n\n", resp.data.len());
    for group in resp.data.iter() {
        let a = &group.attributes;
        let _ = writeln!(sb, "**{}**", a.name);
        let _ = writeln!(sb, "  - ID: {}", group.id);
        let _ = writeln!(sb, "  - Internal Group: {}", a.is_internal_group);
        let _ = writeln!(sb, "  - Has Access to All Builds: {}", a.has_access_to_all_builds);
        let _ = writeln!(sb, "  - Feedback Enabled: {}", a.feedback_enabled);
        let _ = writeln!(sb, "  - Public Link Enabled: {}", a.public_link_enabled);
        if !a.public_link.is_empty() {
            let _ = writeln!(sb, "  - Public Link: {}", a.public_link);
        }
        if let Some(d) = &a.created_date {
            let _ = writeln!(sb, "  - Created: {}", d.format("2006-01-02"));
        }
        sb.push('\n');
    }

    Ok(list_result(&sb, &resp.data, &resp.links))
}

/// Handles `create_beta_group`.
async fn handle_create_beta_group(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Name string \"json:\\\"name\\\"\"; PublicLinkEnabled bool \"json:\\\"public_link_enabled\\\"\"; FeedbackEnabled bool \"json:\\\"feedback_enabled\\\"\" }", "") {
        app_id: String => "app_id";
        name: String => "name";
        public_link_enabled: bool => "public_link_enabled";
        feedback_enabled: bool => "feedback_enabled";
    }
    }
    let mut params = Params { feedback_enabled: true, ..Default::default() };
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.app_id.is_empty() {
        return Ok(error_result("app_id is required"));
    }
    if params.name.is_empty() {
        return Ok(error_result("name is required"));
    }

    let req = BetaGroupCreateRequest {
        data: BetaGroupCreateData {
            type_: "betaGroups".into(),
            attributes: BetaGroupCreateAttributes {
                name: params.name.clone(),
                public_link_enabled: params.public_link_enabled,
                feedback_enabled: params.feedback_enabled,
                ..Default::default()
            },
            relationships: BetaGroupCreateRelationships {
                app: RelationshipData {
                    data: ResourceIdentifier { type_: "apps".into(), id: params.app_id.clone() },
                },
            },
        },
    };

    let resp = match c.create_beta_group(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create beta group: {e}"))),
    };

    let mut sb = String::new();
    let _ = write!(sb, "Successfully created beta group **{}**\n\n", resp.data.attributes.name);
    let _ = writeln!(sb, "- ID: {}", resp.data.id);
    let _ = writeln!(sb, "- Public Link Enabled: {}", resp.data.attributes.public_link_enabled);
    let _ = writeln!(sb, "- Feedback Enabled: {}", resp.data.attributes.feedback_enabled);

    Ok(data_result(&sb, &resp.data))
}

/// Handles `delete_beta_group`.
async fn handle_delete_beta_group(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { BetaGroupID string \"json:\\\"beta_group_id\\\"\" }", "") {
        beta_group_id: String => "beta_group_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.beta_group_id.is_empty() {
        return Ok(error_result("beta_group_id is required"));
    }

    if let Err(e) = c.delete_beta_group(&params.beta_group_id).await {
        return Ok(error_result(format!("Failed to delete beta group: {e}")));
    }

    Ok(CallToolResult::success(format!("Successfully deleted beta group {}", params.beta_group_id)))
}

/// Handles `list_beta_testers`.
async fn handle_list_beta_testers(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { BetaGroupID string \"json:\\\"beta_group_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        beta_group_id: String => "beta_group_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: JsonMap<JsonSlice<String>> => "filter";
        sort: JsonSlice<String> => "sort";
        fields: JsonMap<JsonSlice<String>> => "fields";
        include: JsonSlice<String> => "include";
    }
    }
    let mut params = Params { limit: 50, ..Default::default() };
    if args.is_some()
        && let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
            return Err(format!("invalid arguments: {e}"));
        }

    let opts = list_opts(params.limit, &params.filter, &params.sort, &params.fields, &params.include);
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_beta_testers(&params.beta_group_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list beta testers: {e}"))),
    };

    if resp.data.is_empty() {
        return Ok(list_result("No beta testers found.", &resp.data, &resp.links));
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} beta testers:\n\n", resp.data.len());
    for tester in resp.data.iter() {
        let a = &tester.attributes;
        let mut name = a.email.clone();
        if !a.first_name.is_empty() || !a.last_name.is_empty() {
            name = format!("{} {} ({})", a.first_name, a.last_name, a.email);
        }
        let _ = writeln!(sb, "**{}**", name);
        let _ = writeln!(sb, "  - ID: {}", tester.id);
        let _ = writeln!(sb, "  - State: {}", a.state);
        let _ = writeln!(sb, "  - Invite Type: {}", a.invite_type);
        sb.push('\n');
    }

    Ok(list_result(&sb, &resp.data, &resp.links))
}

/// Handles `invite_beta_tester`.
async fn handle_invite_beta_tester(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { Email string \"json:\\\"email\\\"\"; FirstName string \"json:\\\"first_name\\\"\"; LastName string \"json:\\\"last_name\\\"\"; BetaGroupIDs []string \"json:\\\"beta_group_ids\\\"\" }", "") {
        email: String => "email";
        first_name: String => "first_name";
        last_name: String => "last_name";
        beta_group_ids: JsonSlice<String> => "beta_group_ids";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.email.is_empty() {
        return Ok(error_result("email is required"));
    }

    let mut req = BetaTesterCreateRequest {
        data: BetaTesterCreateData {
            type_: "betaTesters".into(),
            attributes: BetaTesterCreateAttributes {
                email: params.email.clone(),
                first_name: params.first_name.clone(),
                last_name: params.last_name.clone(),
            },
            relationships: None,
        },
    };

    if !params.beta_group_ids.is_empty() {
        let mut groups: Vec<ResourceIdentifier> = Vec::with_capacity(params.beta_group_ids.len());
        for id in params.beta_group_ids.iter() {
            groups.push(ResourceIdentifier { type_: "betaGroups".into(), id: id.clone() });
        }
        req.data.relationships = Some(BetaTesterCreateRelationships {
            beta_groups: Some(RelationshipDataList { data: JsonSlice::from(groups) }),
            ..Default::default()
        });
    }

    let resp = match c.create_beta_tester(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to invite beta tester: {e}"))),
    };

    let mut sb = String::new();
    let _ = write!(sb, "Successfully invited beta tester **{}**\n\n", resp.data.attributes.email);
    let _ = writeln!(sb, "- ID: {}", resp.data.id);
    let _ = writeln!(sb, "- State: {}", resp.data.attributes.state);

    Ok(data_result(&sb, &resp.data))
}

/// Handles `remove_beta_tester`.
async fn handle_remove_beta_tester(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { BetaTesterID string \"json:\\\"beta_tester_id\\\"\" }", "") {
        beta_tester_id: String => "beta_tester_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.beta_tester_id.is_empty() {
        return Ok(error_result("beta_tester_id is required"));
    }

    if let Err(e) = c.delete_beta_tester(&params.beta_tester_id).await {
        return Ok(error_result(format!("Failed to remove beta tester: {e}")));
    }

    Ok(CallToolResult::success(format!("Successfully removed beta tester {}", params.beta_tester_id)))
}

/// Handles `add_tester_to_group`.
async fn handle_add_tester_to_group(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { BetaGroupID string \"json:\\\"beta_group_id\\\"\"; BetaTesterID string \"json:\\\"beta_tester_id\\\"\" }", "") {
        beta_group_id: String => "beta_group_id";
        beta_tester_id: String => "beta_tester_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.beta_group_id.is_empty() {
        return Ok(error_result("beta_group_id is required"));
    }
    if params.beta_tester_id.is_empty() {
        return Ok(error_result("beta_tester_id is required"));
    }

    if let Err(e) = c.add_beta_tester_to_group(&params.beta_group_id, &params.beta_tester_id).await {
        return Ok(error_result(format!("Failed to add tester to group: {e}")));
    }

    Ok(CallToolResult::success(format!(
        "Successfully added beta tester {} to group {}",
        params.beta_tester_id, params.beta_group_id
    )))
}

