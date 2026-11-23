//! User and invitation tools.

#![allow(unused_imports)]

use super::prelude::*;

/// Registers this module's tools in definition order.
pub(super) fn register_user_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "list_users",
            "description": "List all users in the App Store Connect team",
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
                        "description": "Maximum number of users to return (default 50)"
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
                "title": "List Users",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_users),
    );
    r.register(
        tool(r##"{
            "name": "get_user",
            "description": "Get details of a specific user",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "string",
                        "description": "The user ID"
                    }
                },
                "required": [
                    "user_id"
                ]
            },
            "annotations": {
                "title": "Get User",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_user),
    );
    r.register(
        tool(r##"{
            "name": "update_user",
            "description": "Update a user's roles or all apps visibility",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "all_apps_visible": {
                        "type": "boolean",
                        "description": "Whether user can see all apps"
                    },
                    "roles": {
                        "type": "array",
                        "description": "List of roles: ADMIN, FINANCE, ACCOUNT_HOLDER, SALES, MARKETING, APP_MANAGER, DEVELOPER, ACCESS_TO_REPORTS, CUSTOMER_SUPPORT, CREATE_APPS, CLOUD_MANAGED_DEVELOPER_ID, CLOUD_MANAGED_APP_DISTRIBUTION",
                        "items": {
                            "type": "string"
                        }
                    },
                    "user_id": {
                        "type": "string",
                        "description": "The user ID"
                    }
                },
                "required": [
                    "user_id"
                ]
            },
            "annotations": {
                "title": "Update User",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_user),
    );
    r.register(
        tool(r##"{
            "name": "delete_user",
            "description": "Remove a user from the App Store Connect team",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "user_id": {
                        "type": "string",
                        "description": "The user ID to remove"
                    }
                },
                "required": [
                    "user_id"
                ]
            },
            "annotations": {
                "title": "Delete User",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_user),
    );
    r.register(
        tool(r##"{
            "name": "list_user_invitations",
            "description": "List pending user invitations",
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
                        "description": "Maximum number of invitations to return (default 50)"
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
                "title": "List User Invitations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_user_invitations),
    );
    r.register(
        tool(r##"{
            "name": "get_user_invitation",
            "description": "Get details of a specific user invitation",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "invitation_id": {
                        "type": "string",
                        "description": "The user invitation ID"
                    }
                },
                "required": [
                    "invitation_id"
                ]
            },
            "annotations": {
                "title": "Get User Invitation",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_user_invitation),
    );
    r.register(
        tool(r##"{
            "name": "create_user_invitation",
            "description": "Invite a new user to the App Store Connect team",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "all_apps_visible": {
                        "type": "boolean",
                        "description": "Whether user can see all apps (default true)"
                    },
                    "email": {
                        "type": "string",
                        "description": "Email address of the user to invite"
                    },
                    "first_name": {
                        "type": "string",
                        "description": "First name of the user"
                    },
                    "last_name": {
                        "type": "string",
                        "description": "Last name of the user"
                    },
                    "roles": {
                        "type": "array",
                        "description": "List of roles to assign: ADMIN, FINANCE, ACCOUNT_HOLDER, SALES, MARKETING, APP_MANAGER, DEVELOPER, ACCESS_TO_REPORTS, CUSTOMER_SUPPORT, CREATE_APPS, CLOUD_MANAGED_DEVELOPER_ID, CLOUD_MANAGED_APP_DISTRIBUTION",
                        "items": {
                            "type": "string"
                        }
                    }
                },
                "required": [
                    "email",
                    "first_name",
                    "last_name",
                    "roles"
                ]
            },
            "annotations": {
                "title": "Create User Invitation",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_user_invitation),
    );
    r.register(
        tool(r##"{
            "name": "delete_user_invitation",
            "description": "Delete (cancel) a pending user invitation",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "invitation_id": {
                        "type": "string",
                        "description": "The user invitation ID to delete"
                    }
                },
                "required": [
                    "invitation_id"
                ]
            },
            "annotations": {
                "title": "Delete User Invitation",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_user_invitation),
    );
}

/// Handles `list_users`.
async fn handle_list_users(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
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
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_users(opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list users: {e}"))),
    };

    Ok(list_result(&format_users(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_user`.
async fn handle_get_user(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { UserID string \"json:\\\"user_id\\\"\" }", "") {
        user_id: String => "user_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.user_id.is_empty() {
        return Ok(error_result("user_id is required"));
    }

    let resp = match c.get_user(&params.user_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get user: {e}"))),
    };

    Ok(data_result(&format_user(&resp.data), &resp.data))
}

/// Handles `update_user`.
async fn handle_update_user(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { UserID string \"json:\\\"user_id\\\"\"; Roles []string \"json:\\\"roles\\\"\"; AllAppsVisible *bool \"json:\\\"all_apps_visible\\\"\" }", "") {
        user_id: String => "user_id";
        roles: JsonSlice<String> => "roles";
        all_apps_visible: Option<bool> => "all_apps_visible";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.user_id.is_empty() {
        return Ok(error_result("user_id is required"));
    }

    let req = UserUpdateRequest {
        data: UserUpdateData {
            type_: "users".into(),
            id: params.user_id.clone(),
            attributes: UserUpdateAttributes {
                roles: params.roles.clone(),
                all_apps_visible: params.all_apps_visible,
                ..Default::default()
            },
            relationships: None,
        },
    };

    let resp = match c.update_user(&params.user_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update user: {e}"))),
    };

    Ok(data_result(&format!("User updated successfully:\n{}", format_user(&resp.data)), &resp.data))
}

/// Handles `delete_user`.
async fn handle_delete_user(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { UserID string \"json:\\\"user_id\\\"\" }", "") {
        user_id: String => "user_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.user_id.is_empty() {
        return Ok(error_result("user_id is required"));
    }

    if let Err(e) = c.delete_user(&params.user_id).await {
        return Ok(error_result(format!("Failed to delete user: {e}")));
    }

    Ok(CallToolResult::success("User removed successfully"))
}

/// Handles `list_user_invitations`.
async fn handle_list_user_invitations(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
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
    let resp = match paginated_fetch(&c, &params.cursor, || c.list_user_invitations(opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list user invitations: {e}"))),
    };

    Ok(list_result(&format_user_invitations(&resp.data), &resp.data, &resp.links))
}

/// Handles `get_user_invitation`.
async fn handle_get_user_invitation(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { InvitationID string \"json:\\\"invitation_id\\\"\" }", "") {
        invitation_id: String => "invitation_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.invitation_id.is_empty() {
        return Ok(error_result("invitation_id is required"));
    }

    let resp = match c.get_user_invitation(&params.invitation_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get user invitation: {e}"))),
    };

    Ok(data_result(&format_user_invitation(&resp.data), &resp.data))
}

/// Handles `create_user_invitation`.
async fn handle_create_user_invitation(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { Email string \"json:\\\"email\\\"\"; FirstName string \"json:\\\"first_name\\\"\"; LastName string \"json:\\\"last_name\\\"\"; Roles []string \"json:\\\"roles\\\"\"; AllAppsVisible *bool \"json:\\\"all_apps_visible\\\"\" }", "") {
        email: String => "email";
        first_name: String => "first_name";
        last_name: String => "last_name";
        roles: JsonSlice<String> => "roles";
        all_apps_visible: Option<bool> => "all_apps_visible";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.email.is_empty() || params.first_name.is_empty() || params.last_name.is_empty() {
        return Ok(error_result("email, first_name, and last_name are required"));
    }

    if params.roles.is_empty() {
        return Ok(error_result("at least one role is required"));
    }

    let all_apps_visible = params.all_apps_visible.unwrap_or(true);

    let req = UserInvitationCreateRequest {
        data: UserInvitationCreateData {
            type_: "userInvitations".into(),
            attributes: UserInvitationCreateAttributes {
                email: params.email.clone(),
                first_name: params.first_name.clone(),
                last_name: params.last_name.clone(),
                roles: params.roles.clone(),
                all_apps_visible,
                ..Default::default()
            },
            relationships: None,
        },
    };

    let resp = match c.create_user_invitation(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create user invitation: {e}"))),
    };

    Ok(data_result(&format!("User invitation sent:\n{}", format_user_invitation(&resp.data)), &resp.data))
}

/// Handles `delete_user_invitation`.
async fn handle_delete_user_invitation(c: Arc<Client>, args: Args) -> HandlerResult {
    json_struct! {
    struct Params ("struct { InvitationID string \"json:\\\"invitation_id\\\"\" }", "") {
        invitation_id: String => "invitation_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }

    if params.invitation_id.is_empty() {
        return Ok(error_result("invitation_id is required"));
    }

    if let Err(e) = c.delete_user_invitation(&params.invitation_id).await {
        return Ok(error_result(format!("Failed to delete user invitation: {e}")));
    }

    Ok(CallToolResult::success("User invitation deleted"))
}

fn format_users(users: &[User]) -> String {
    if users.is_empty() {
        return "No users found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} users:\n\n", users.len());

    for user in users {
        sb.push_str(&format_user(user));
        sb.push_str("\n---\n");
    }

    sb
}

fn format_user(user: &User) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", user.id);
    let _ = writeln!(sb, "Username: {}", user.attributes.username);
    let _ = writeln!(sb, "Name: {} {}", user.attributes.first_name, user.attributes.last_name);
    if !user.attributes.roles.is_empty() {
        let _ = writeln!(sb, "Roles: {}", user.attributes.roles.join(", "));
    }
    let _ = writeln!(sb, "All Apps Visible: {}", user.attributes.all_apps_visible);
    let _ = writeln!(sb, "Provisioning Allowed: {}", user.attributes.provisioning_allowed);
    sb
}

fn format_user_invitations(invitations: &[UserInvitation]) -> String {
    if invitations.is_empty() {
        return "No pending user invitations found".to_string();
    }

    let mut sb = String::new();
    let _ = write!(sb, "Found {} pending invitations:\n\n", invitations.len());

    for inv in invitations {
        sb.push_str(&format_user_invitation(inv));
        sb.push_str("\n---\n");
    }

    sb
}

fn format_user_invitation(inv: &UserInvitation) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", inv.id);
    let _ = writeln!(sb, "Email: {}", inv.attributes.email);
    let _ = writeln!(sb, "Name: {} {}", inv.attributes.first_name, inv.attributes.last_name);
    if !inv.attributes.roles.is_empty() {
        let _ = writeln!(sb, "Roles: {}", inv.attributes.roles.join(", "));
    }
    let _ = writeln!(sb, "All Apps Visible: {}", inv.attributes.all_apps_visible);
    if let Some(d) = &inv.attributes.expiration_date {
        let _ = writeln!(sb, "Expires: {}", d.format("2006-01-02 15:04"));
    }
    sb
}
