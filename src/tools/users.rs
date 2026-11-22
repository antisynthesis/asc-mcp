//! Ported from internal/asc/tools/users.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerUserTools`.
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

/// Ported from `Registry.handleListUsers`.
async fn handle_list_users(c: Arc<Client>, args: Args) -> HandlerResult {
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
    todo!("port handleListUsers from internal/asc/tools/users.go")
}

/// Ported from `Registry.handleGetUser`.
async fn handle_get_user(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { UserID string \"json:\\\"user_id\\\"\" }", "") {
        user_id: String => "user_id";
    }
    }
    todo!("port handleGetUser from internal/asc/tools/users.go")
}

/// Ported from `Registry.handleUpdateUser`.
async fn handle_update_user(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { UserID string \"json:\\\"user_id\\\"\"; Roles []string \"json:\\\"roles\\\"\"; AllAppsVisible *bool \"json:\\\"all_apps_visible\\\"\" }", "") {
        user_id: String => "user_id";
        roles: GoSlice<String> => "roles";
        all_apps_visible: Option<bool> => "all_apps_visible";
    }
    }
    todo!("port handleUpdateUser from internal/asc/tools/users.go")
}

/// Ported from `Registry.handleDeleteUser`.
async fn handle_delete_user(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { UserID string \"json:\\\"user_id\\\"\" }", "") {
        user_id: String => "user_id";
    }
    }
    todo!("port handleDeleteUser from internal/asc/tools/users.go")
}

/// Ported from `Registry.handleListUserInvitations`.
async fn handle_list_user_invitations(c: Arc<Client>, args: Args) -> HandlerResult {
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
    todo!("port handleListUserInvitations from internal/asc/tools/users.go")
}

/// Ported from `Registry.handleGetUserInvitation`.
async fn handle_get_user_invitation(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { InvitationID string \"json:\\\"invitation_id\\\"\" }", "") {
        invitation_id: String => "invitation_id";
    }
    }
    todo!("port handleGetUserInvitation from internal/asc/tools/users.go")
}

/// Ported from `Registry.handleCreateUserInvitation`.
async fn handle_create_user_invitation(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { Email string \"json:\\\"email\\\"\"; FirstName string \"json:\\\"first_name\\\"\"; LastName string \"json:\\\"last_name\\\"\"; Roles []string \"json:\\\"roles\\\"\"; AllAppsVisible *bool \"json:\\\"all_apps_visible\\\"\" }", "") {
        email: String => "email";
        first_name: String => "first_name";
        last_name: String => "last_name";
        roles: GoSlice<String> => "roles";
        all_apps_visible: Option<bool> => "all_apps_visible";
    }
    }
    todo!("port handleCreateUserInvitation from internal/asc/tools/users.go")
}

/// Ported from `Registry.handleDeleteUserInvitation`.
async fn handle_delete_user_invitation(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { InvitationID string \"json:\\\"invitation_id\\\"\" }", "") {
        invitation_id: String => "invitation_id";
    }
    }
    todo!("port handleDeleteUserInvitation from internal/asc/tools/users.go")
}

