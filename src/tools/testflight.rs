//! Ported from internal/asc/tools/testflight.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerTestFlightTools`.
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

/// Ported from `Registry.handleListBetaGroups`.
async fn handle_list_beta_groups(c: Arc<Client>, args: Args) -> HandlerResult {
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
    todo!("port handleListBetaGroups from internal/asc/tools/testflight.go")
}

/// Ported from `Registry.handleCreateBetaGroup`.
async fn handle_create_beta_group(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { AppID string \"json:\\\"app_id\\\"\"; Name string \"json:\\\"name\\\"\"; PublicLinkEnabled bool \"json:\\\"public_link_enabled\\\"\"; FeedbackEnabled bool \"json:\\\"feedback_enabled\\\"\" }", "") {
        app_id: String => "app_id";
        name: String => "name";
        public_link_enabled: bool => "public_link_enabled";
        feedback_enabled: bool => "feedback_enabled";
    }
    }
    todo!("port handleCreateBetaGroup from internal/asc/tools/testflight.go")
}

/// Ported from `Registry.handleDeleteBetaGroup`.
async fn handle_delete_beta_group(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BetaGroupID string \"json:\\\"beta_group_id\\\"\" }", "") {
        beta_group_id: String => "beta_group_id";
    }
    }
    todo!("port handleDeleteBetaGroup from internal/asc/tools/testflight.go")
}

/// Ported from `Registry.handleListBetaTesters`.
async fn handle_list_beta_testers(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BetaGroupID string \"json:\\\"beta_group_id\\\"\"; Limit int \"json:\\\"limit\\\"\"; Cursor string \"json:\\\"cursor\\\"\"; Filter map[string][]string \"json:\\\"filter\\\"\"; Sort []string \"json:\\\"sort\\\"\"; Fields map[string][]string \"json:\\\"fields\\\"\"; Include []string \"json:\\\"include\\\"\" }", "") {
        beta_group_id: String => "beta_group_id";
        limit: i64 => "limit";
        cursor: String => "cursor";
        filter: GoMap<GoSlice<String>> => "filter";
        sort: GoSlice<String> => "sort";
        fields: GoMap<GoSlice<String>> => "fields";
        include: GoSlice<String> => "include";
    }
    }
    todo!("port handleListBetaTesters from internal/asc/tools/testflight.go")
}

/// Ported from `Registry.handleInviteBetaTester`.
async fn handle_invite_beta_tester(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { Email string \"json:\\\"email\\\"\"; FirstName string \"json:\\\"first_name\\\"\"; LastName string \"json:\\\"last_name\\\"\"; BetaGroupIDs []string \"json:\\\"beta_group_ids\\\"\" }", "") {
        email: String => "email";
        first_name: String => "first_name";
        last_name: String => "last_name";
        beta_group_ids: GoSlice<String> => "beta_group_ids";
    }
    }
    todo!("port handleInviteBetaTester from internal/asc/tools/testflight.go")
}

/// Ported from `Registry.handleRemoveBetaTester`.
async fn handle_remove_beta_tester(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BetaTesterID string \"json:\\\"beta_tester_id\\\"\" }", "") {
        beta_tester_id: String => "beta_tester_id";
    }
    }
    todo!("port handleRemoveBetaTester from internal/asc/tools/testflight.go")
}

/// Ported from `Registry.handleAddTesterToGroup`.
async fn handle_add_tester_to_group(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { BetaGroupID string \"json:\\\"beta_group_id\\\"\"; BetaTesterID string \"json:\\\"beta_tester_id\\\"\" }", "") {
        beta_group_id: String => "beta_group_id";
        beta_tester_id: String => "beta_tester_id";
    }
    }
    todo!("port handleAddTesterToGroup from internal/asc/tools/testflight.go")
}

