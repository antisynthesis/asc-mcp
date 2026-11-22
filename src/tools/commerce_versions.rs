//! Ported from internal/asc/tools/commerce_versions.go.
//!
//! Tool definitions were generated from the Go registry's runtime output so
//! names, descriptions, schemas and annotations match exactly.

#![allow(unused_imports)]

use super::prelude::*;

/// Ported from `Registry.registerCommerceVersionTools`.
pub(super) fn register_commerce_version_tools(r: &mut Registry) {
    r.register(
        tool(r##"{
            "name": "create_in_app_purchase_version",
            "description": "Create a new version of an in-app purchase. The version snapshots the product's current metadata; edit its localizations and images, then submit it with add_review_submission_item.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "iap_id": {
                        "type": "string",
                        "description": "The in-app purchase ID"
                    }
                },
                "required": [
                    "iap_id"
                ]
            },
            "annotations": {
                "title": "Create In App Purchase Version",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_in_app_purchase_version),
    );
    r.register(
        tool(r##"{
            "name": "get_in_app_purchase_version",
            "description": "Get an in-app purchase version and its review state",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "version_id": {
                        "type": "string",
                        "description": "The in-app purchase version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Get In App Purchase Version",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_in_app_purchase_version),
    );
    r.register(
        tool(r##"{
            "name": "list_in_app_purchase_versions",
            "description": "List the versions of an in-app purchase",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"APPROVED\"]} becomes filter[state]=APPROVED."
                    },
                    "iap_id": {
                        "type": "string",
                        "description": "The in-app purchase ID"
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
                        "description": "Maximum number of results to return (default 50)"
                    }
                },
                "required": [
                    "iap_id"
                ]
            },
            "annotations": {
                "title": "List In App Purchase Versions",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_in_app_purchase_versions),
    );
    r.register(
        tool(r##"{
            "name": "list_in_app_purchase_version_localizations",
            "description": "List the localizations of an in-app purchase version",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"APPROVED\"]} becomes filter[state]=APPROVED."
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
                        "description": "Maximum number of results to return (default 50)"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The in-app purchase version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "List In App Purchase Version Localizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_in_app_purchase_version_localizations),
    );
    r.register(
        tool(r##"{
            "name": "create_in_app_purchase_localization",
            "description": "Add a localization to an in-app purchase version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "description": {
                        "type": "string",
                        "description": "The description shown to customers"
                    },
                    "locale": {
                        "type": "string",
                        "description": "The locale, e.g. en-US"
                    },
                    "name": {
                        "type": "string",
                        "description": "The display name shown to customers"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The in-app purchase version ID"
                    }
                },
                "required": [
                    "version_id",
                    "locale",
                    "name"
                ]
            },
            "annotations": {
                "title": "Create In App Purchase Localization",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_in_app_purchase_localization),
    );
    r.register(
        tool(r##"{
            "name": "update_in_app_purchase_localization",
            "description": "Update the name or description of an in-app purchase localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "description": {
                        "type": "string",
                        "description": "The updated description"
                    },
                    "localization_id": {
                        "type": "string",
                        "description": "The in-app purchase localization ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "The updated display name"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Update In App Purchase Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_in_app_purchase_localization),
    );
    r.register(
        tool(r##"{
            "name": "delete_in_app_purchase_localization",
            "description": "Delete an in-app purchase localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The in-app purchase localization ID"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Delete In App Purchase Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_in_app_purchase_localization),
    );
    r.register(
        tool(r##"{
            "name": "list_in_app_purchase_version_images",
            "description": "List the promotional images of an in-app purchase version",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"APPROVED\"]} becomes filter[state]=APPROVED."
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
                        "description": "Maximum number of results to return (default 50)"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The in-app purchase version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "List In App Purchase Version Images",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_in_app_purchase_version_images),
    );
    r.register(
        tool(r##"{
            "name": "create_in_app_purchase_image",
            "description": "Reserve a promotional image upload on an in-app purchase version. Returns the upload operations to perform, then commit with update_in_app_purchase_image.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "file_name": {
                        "type": "string",
                        "description": "The image file name"
                    },
                    "file_size": {
                        "type": "integer",
                        "description": "The image file size in bytes"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The in-app purchase version ID"
                    }
                },
                "required": [
                    "version_id",
                    "file_name",
                    "file_size"
                ]
            },
            "annotations": {
                "title": "Create In App Purchase Image",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_in_app_purchase_image),
    );
    r.register(
        tool(r##"{
            "name": "update_in_app_purchase_image",
            "description": "Mark an in-app purchase image upload complete",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "image_id": {
                        "type": "string",
                        "description": "The in-app purchase image ID"
                    },
                    "uploaded": {
                        "type": "boolean",
                        "description": "Set true once every upload operation has completed (default true)"
                    }
                },
                "required": [
                    "image_id"
                ]
            },
            "annotations": {
                "title": "Update In App Purchase Image",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_in_app_purchase_image),
    );
    r.register(
        tool(r##"{
            "name": "delete_in_app_purchase_image",
            "description": "Delete an in-app purchase promotional image",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "image_id": {
                        "type": "string",
                        "description": "The in-app purchase image ID"
                    }
                },
                "required": [
                    "image_id"
                ]
            },
            "annotations": {
                "title": "Delete In App Purchase Image",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_in_app_purchase_image),
    );
    r.register(
        tool(r##"{
            "name": "create_subscription_version",
            "description": "Create a new version of a subscription. Edit its localizations and images, then submit it with add_review_submission_item.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "subscription_id": {
                        "type": "string",
                        "description": "The subscription ID"
                    }
                },
                "required": [
                    "subscription_id"
                ]
            },
            "annotations": {
                "title": "Create Subscription Version",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_subscription_version),
    );
    r.register(
        tool(r##"{
            "name": "get_subscription_version",
            "description": "Get a subscription version and its review state",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "version_id": {
                        "type": "string",
                        "description": "The subscription version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Get Subscription Version",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_subscription_version),
    );
    r.register(
        tool(r##"{
            "name": "list_subscription_versions",
            "description": "List the versions of a subscription",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"APPROVED\"]} becomes filter[state]=APPROVED."
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
                        "description": "Maximum number of results to return (default 50)"
                    },
                    "subscription_id": {
                        "type": "string",
                        "description": "The subscription ID"
                    }
                },
                "required": [
                    "subscription_id"
                ]
            },
            "annotations": {
                "title": "List Subscription Versions",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_subscription_versions),
    );
    r.register(
        tool(r##"{
            "name": "list_subscription_version_localizations",
            "description": "List the localizations of a subscription version",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"APPROVED\"]} becomes filter[state]=APPROVED."
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
                        "description": "Maximum number of results to return (default 50)"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The subscription version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "List Subscription Version Localizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_subscription_version_localizations),
    );
    r.register(
        tool(r##"{
            "name": "create_subscription_localization",
            "description": "Add a localization to a subscription version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "description": {
                        "type": "string",
                        "description": "The description shown to customers"
                    },
                    "locale": {
                        "type": "string",
                        "description": "The locale, e.g. en-US"
                    },
                    "name": {
                        "type": "string",
                        "description": "The display name shown to customers"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The subscription version ID"
                    }
                },
                "required": [
                    "version_id",
                    "locale",
                    "name"
                ]
            },
            "annotations": {
                "title": "Create Subscription Localization",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_subscription_localization),
    );
    r.register(
        tool(r##"{
            "name": "update_subscription_localization",
            "description": "Update the name or description of a subscription localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "description": {
                        "type": "string",
                        "description": "The updated description"
                    },
                    "localization_id": {
                        "type": "string",
                        "description": "The subscription localization ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "The updated display name"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Update Subscription Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_subscription_localization),
    );
    r.register(
        tool(r##"{
            "name": "delete_subscription_localization",
            "description": "Delete a subscription localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The subscription localization ID"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Delete Subscription Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_subscription_localization),
    );
    r.register(
        tool(r##"{
            "name": "list_subscription_version_images",
            "description": "List the promotional images of a subscription version",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"APPROVED\"]} becomes filter[state]=APPROVED."
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
                        "description": "Maximum number of results to return (default 50)"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The subscription version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "List Subscription Version Images",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_subscription_version_images),
    );
    r.register(
        tool(r##"{
            "name": "create_subscription_image",
            "description": "Reserve a promotional image upload on a subscription version. Returns the upload operations to perform, then commit with update_subscription_image.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "file_name": {
                        "type": "string",
                        "description": "The image file name"
                    },
                    "file_size": {
                        "type": "integer",
                        "description": "The image file size in bytes"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The subscription version ID"
                    }
                },
                "required": [
                    "version_id",
                    "file_name",
                    "file_size"
                ]
            },
            "annotations": {
                "title": "Create Subscription Image",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_subscription_image),
    );
    r.register(
        tool(r##"{
            "name": "update_subscription_image",
            "description": "Mark a subscription image upload complete",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "image_id": {
                        "type": "string",
                        "description": "The subscription image ID"
                    },
                    "uploaded": {
                        "type": "boolean",
                        "description": "Set true once every upload operation has completed (default true)"
                    }
                },
                "required": [
                    "image_id"
                ]
            },
            "annotations": {
                "title": "Update Subscription Image",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_subscription_image),
    );
    r.register(
        tool(r##"{
            "name": "delete_subscription_image",
            "description": "Delete a subscription promotional image",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "image_id": {
                        "type": "string",
                        "description": "The subscription image ID"
                    }
                },
                "required": [
                    "image_id"
                ]
            },
            "annotations": {
                "title": "Delete Subscription Image",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_subscription_image),
    );
    r.register(
        tool(r##"{
            "name": "create_subscription_group_version",
            "description": "Create a new version of a subscription group. Edit its localizations, then submit it with add_review_submission_item.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "group_id": {
                        "type": "string",
                        "description": "The subscription group ID"
                    }
                },
                "required": [
                    "group_id"
                ]
            },
            "annotations": {
                "title": "Create Subscription Group Version",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_subscription_group_version),
    );
    r.register(
        tool(r##"{
            "name": "get_subscription_group_version",
            "description": "Get a subscription group version and its review state",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "version_id": {
                        "type": "string",
                        "description": "The subscription group version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "Get Subscription Group Version",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_get_subscription_group_version),
    );
    r.register(
        tool(r##"{
            "name": "list_subscription_group_versions",
            "description": "List the versions of a subscription group",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"APPROVED\"]} becomes filter[state]=APPROVED."
                    },
                    "group_id": {
                        "type": "string",
                        "description": "The subscription group ID"
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
                        "description": "Maximum number of results to return (default 50)"
                    }
                },
                "required": [
                    "group_id"
                ]
            },
            "annotations": {
                "title": "List Subscription Group Versions",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_subscription_group_versions),
    );
    r.register(
        tool(r##"{
            "name": "list_subscription_group_version_localizations",
            "description": "List the localizations of a subscription group version",
            "inputSchema": {
                "type": "object",
                "properties": {
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
                        "description": "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"state\": [\"APPROVED\"]} becomes filter[state]=APPROVED."
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
                        "description": "Maximum number of results to return (default 50)"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The subscription group version ID"
                    }
                },
                "required": [
                    "version_id"
                ]
            },
            "annotations": {
                "title": "List Subscription Group Version Localizations",
                "readOnlyHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_list_subscription_group_version_localizations),
    );
    r.register(
        tool(r##"{
            "name": "create_subscription_group_localization",
            "description": "Add a localization to a subscription group version",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "custom_app_name": {
                        "type": "string",
                        "description": "An alternate app name to show on the subscription management screen"
                    },
                    "locale": {
                        "type": "string",
                        "description": "The locale, e.g. en-US"
                    },
                    "name": {
                        "type": "string",
                        "description": "The subscription group display name"
                    },
                    "version_id": {
                        "type": "string",
                        "description": "The subscription group version ID"
                    }
                },
                "required": [
                    "version_id",
                    "locale",
                    "name"
                ]
            },
            "annotations": {
                "title": "Create Subscription Group Localization",
                "readOnlyHint": false,
                "destructiveHint": false,
                "idempotentHint": false,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_create_subscription_group_localization),
    );
    r.register(
        tool(r##"{
            "name": "update_subscription_group_localization",
            "description": "Update the name or custom app name of a subscription group localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "custom_app_name": {
                        "type": "string",
                        "description": "The updated alternate app name"
                    },
                    "localization_id": {
                        "type": "string",
                        "description": "The subscription group localization ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "The updated subscription group display name"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Update Subscription Group Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_update_subscription_group_localization),
    );
    r.register(
        tool(r##"{
            "name": "delete_subscription_group_localization",
            "description": "Delete a subscription group localization",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "localization_id": {
                        "type": "string",
                        "description": "The subscription group localization ID"
                    }
                },
                "required": [
                    "localization_id"
                ]
            },
            "annotations": {
                "title": "Delete Subscription Group Localization",
                "readOnlyHint": false,
                "destructiveHint": true,
                "idempotentHint": true,
                "openWorldHint": true
            }
        }"##),
        handler!(handle_delete_subscription_group_localization),
    );
}

go_struct! {
pub(crate) struct CommerceListParams ("tools.commerceListParams", "commerceListParams") {
    pub(crate) limit: i64 => "limit";
    pub(crate) cursor: String => "cursor";
    pub(crate) filter: GoMap<GoSlice<String>> => "filter";
    pub(crate) fields: GoMap<GoSlice<String>> => "fields";
    pub(crate) include: GoSlice<String> => "include";
}
}

go_struct! {
pub(crate) struct CommerceImageCreateParams ("tools.commerceImageCreateParams", "commerceImageCreateParams") {
    pub(crate) version_id: String => "version_id";
    pub(crate) file_name: String => "file_name";
    pub(crate) file_size: i64 => "file_size";
}
}

go_struct! {
pub(crate) struct CommerceVersionRow ("tools.commerceVersionRow", "commerceVersionRow") {
    pub(crate) id: String => "ID";
    pub(crate) attributes: CommerceVersionAttributes => "Attributes";
}
}

go_struct! {
pub(crate) struct CommerceImageRow ("tools.commerceImageRow", "commerceImageRow") {
    pub(crate) id: String => "ID";
    pub(crate) attributes: CommerceImageAttributes => "Attributes";
}
}

/// Ported from `Registry.handleCreateInAppPurchaseVersion`.
async fn handle_create_in_app_purchase_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\" }", "") {
        iap_id: String => "iap_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.iap_id.is_empty() {
        return Ok(error_result("iap_id is required"));
    }
    let req = InAppPurchaseVersionCreateRequest {
        data: InAppPurchaseVersionCreateData {
            type_: "inAppPurchaseVersions".into(),
            relationships: InAppPurchaseVersionCreateRelationships { in_app_purchase: rel("inAppPurchases", &params.iap_id) },
        },
    };
    let resp = match c.create_in_app_purchase_version(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create in-app purchase version: {e}"))),
    };
    Ok(data_result(&format!("In-app purchase version created:\n{}", format_commerce_version(&resp.data.id, &resp.data.attributes)), &resp.data))
}

/// Ported from `Registry.handleGetInAppPurchaseVersion`.
async fn handle_get_in_app_purchase_version(c: Arc<Client>, args: Args) -> HandlerResult {
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
    let resp = match c.get_in_app_purchase_version(&params.version_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get in-app purchase version: {e}"))),
    };
    Ok(data_result(&format_commerce_version(&resp.data.id, &resp.data.attributes), &resp.data))
}

/// Ported from `Registry.handleListInAppPurchaseVersions`.
async fn handle_list_in_app_purchase_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\"; tools.commerceListParams }", "") {
        iap_id: String => "iap_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.iap_id.is_empty() {
        return Ok(error_result("iap_id is required"));
    }
    let opts = params.commerce_list_params.opts(50);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_in_app_purchase_versions(&params.iap_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list in-app purchase versions: {e}"))),
    };
    let versions = version_rows(resp.data.iter().map(|v| (&v.id, &v.attributes)));
    Ok(list_result(&format_commerce_versions(&versions), &resp.data, &resp.links))
}

/// Ported from `Registry.handleListInAppPurchaseVersionLocalizations`.
async fn handle_list_in_app_purchase_version_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; tools.commerceListParams }", "") {
        version_id: String => "version_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }
    let opts = params.commerce_list_params.opts(50);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_in_app_purchase_version_localizations(&params.version_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list in-app purchase version localizations: {e}"))),
    };
    Ok(list_result(&format_commerce_localizations(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleCreateInAppPurchaseLocalization`.
async fn handle_create_in_app_purchase_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; Locale string \"json:\\\"locale\\\"\"; Name string \"json:\\\"name\\\"\"; Description string \"json:\\\"description\\\"\" }", "") {
        version_id: String => "version_id";
        locale: String => "locale";
        name: String => "name";
        description: String => "description";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }
    if params.locale.is_empty() {
        return Ok(error_result("locale is required"));
    }
    if params.name.is_empty() {
        return Ok(error_result("name is required"));
    }
    let req = InAppPurchaseLocalizationCreateRequest {
        data: InAppPurchaseLocalizationCreateData {
            type_: "inAppPurchaseLocalizations".into(),
            attributes: InAppPurchaseLocalizationCreateAttributes {
                name: params.name.clone(),
                locale: params.locale.clone(),
                description: params.description.clone(),
            },
            relationships: InAppPurchaseLocalizationCreateRelationships { version: rel("inAppPurchaseVersions", &params.version_id) },
        },
    };
    let resp = match c.create_in_app_purchase_localization(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create in-app purchase localization: {e}"))),
    };
    Ok(data_result(&format!("In-app purchase localization created:\n{}", format_commerce_localization(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleUpdateInAppPurchaseLocalization`.
async fn handle_update_in_app_purchase_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Name string \"json:\\\"name\\\"\"; Description string \"json:\\\"description\\\"\" }", "") {
        localization_id: String => "localization_id";
        name: String => "name";
        description: String => "description";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }
    let req = InAppPurchaseLocalizationUpdateRequest {
        data: InAppPurchaseLocalizationUpdateData {
            type_: "inAppPurchaseLocalizations".into(),
            id: params.localization_id.clone(),
            attributes: InAppPurchaseLocalizationUpdateAttributes { name: params.name.clone(), description: params.description.clone() },
        },
    };
    let resp = match c.update_in_app_purchase_localization(&params.localization_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update in-app purchase localization: {e}"))),
    };
    Ok(data_result(&format!("In-app purchase localization updated:\n{}", format_commerce_localization(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleDeleteInAppPurchaseLocalization`.
async fn handle_delete_in_app_purchase_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }
    if let Err(e) = c.delete_in_app_purchase_localization(&params.localization_id).await {
        return Ok(error_result(format!("Failed to delete in-app purchase localization: {e}")));
    }
    Ok(CallToolResult::success("In-app purchase localization deleted successfully"))
}

/// Ported from `Registry.handleListInAppPurchaseVersionImages`.
async fn handle_list_in_app_purchase_version_images(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; tools.commerceListParams }", "") {
        version_id: String => "version_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }
    let opts = params.commerce_list_params.opts(50);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_in_app_purchase_version_images(&params.version_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list in-app purchase version images: {e}"))),
    };
    let images: Vec<CommerceImageRow> = resp.data.iter().map(|img| image_row(&img.id, &img.attributes)).collect();
    Ok(list_result(&format_commerce_images(&images), &resp.data, &resp.links))
}

/// Ported from `Registry.handleCreateInAppPurchaseImage`.
async fn handle_create_in_app_purchase_image(c: Arc<Client>, args: Args) -> HandlerResult {
    let params = match parse_commerce_image_create(&args) {
        Ok(p) => p,
        Err(r) => return Ok(r),
    };
    let req = InAppPurchaseImageCreateRequest {
        data: InAppPurchaseImageCreateData {
            type_: "inAppPurchaseImages".into(),
            attributes: CommerceImageCreateAttributes { file_name: params.file_name.clone(), file_size: params.file_size },
            relationships: InAppPurchaseImageCreateRelationships { version: rel("inAppPurchaseVersions", &params.version_id) },
        },
    };
    let resp = match c.create_in_app_purchase_image(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create in-app purchase image: {e}"))),
    };
    Ok(data_result(&format!("In-app purchase image reserved:\n{}", format_commerce_image(&image_row(&resp.data.id, &resp.data.attributes))), &resp.data))
}

/// Ported from `Registry.handleUpdateInAppPurchaseImage`.
async fn handle_update_in_app_purchase_image(c: Arc<Client>, args: Args) -> HandlerResult {
    let (image_id, uploaded) = match parse_commerce_image_update(&args) {
        Ok(p) => p,
        Err(r) => return Ok(r),
    };
    let req = InAppPurchaseImageUpdateRequest {
        data: InAppPurchaseImageUpdateData {
            type_: "inAppPurchaseImages".into(),
            id: image_id.clone(),
            attributes: CommerceImageUpdateAttributes { uploaded },
        },
    };
    let resp = match c.update_in_app_purchase_image(&image_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update in-app purchase image: {e}"))),
    };
    Ok(data_result(&format!("In-app purchase image updated:\n{}", format_commerce_image(&image_row(&resp.data.id, &resp.data.attributes))), &resp.data))
}

/// Ported from `Registry.handleDeleteInAppPurchaseImage`.
async fn handle_delete_in_app_purchase_image(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ImageID string \"json:\\\"image_id\\\"\" }", "") {
        image_id: String => "image_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.image_id.is_empty() {
        return Ok(error_result("image_id is required"));
    }
    if let Err(e) = c.delete_in_app_purchase_image(&params.image_id).await {
        return Ok(error_result(format!("Failed to delete in-app purchase image: {e}")));
    }
    Ok(CallToolResult::success("In-app purchase image deleted successfully"))
}

/// Ported from `Registry.handleCreateSubscriptionVersion`.
async fn handle_create_subscription_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubscriptionID string \"json:\\\"subscription_id\\\"\" }", "") {
        subscription_id: String => "subscription_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.subscription_id.is_empty() {
        return Ok(error_result("subscription_id is required"));
    }
    let req = SubscriptionVersionCreateRequest {
        data: SubscriptionVersionCreateData {
            type_: "subscriptionVersions".into(),
            relationships: SubscriptionVersionCreateRelationships { subscription: rel("subscriptions", &params.subscription_id) },
        },
    };
    let resp = match c.create_subscription_version(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create subscription version: {e}"))),
    };
    Ok(data_result(&format!("Subscription version created:\n{}", format_commerce_version(&resp.data.id, &resp.data.attributes)), &resp.data))
}

/// Ported from `Registry.handleGetSubscriptionVersion`.
async fn handle_get_subscription_version(c: Arc<Client>, args: Args) -> HandlerResult {
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
    let resp = match c.get_subscription_version(&params.version_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get subscription version: {e}"))),
    };
    Ok(data_result(&format_commerce_version(&resp.data.id, &resp.data.attributes), &resp.data))
}

/// Ported from `Registry.handleListSubscriptionVersions`.
async fn handle_list_subscription_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubscriptionID string \"json:\\\"subscription_id\\\"\"; tools.commerceListParams }", "") {
        subscription_id: String => "subscription_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.subscription_id.is_empty() {
        return Ok(error_result("subscription_id is required"));
    }
    let opts = params.commerce_list_params.opts(50);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_subscription_versions(&params.subscription_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list subscription versions: {e}"))),
    };
    let versions = version_rows(resp.data.iter().map(|v| (&v.id, &v.attributes)));
    Ok(list_result(&format_commerce_versions(&versions), &resp.data, &resp.links))
}

/// Ported from `Registry.handleListSubscriptionVersionLocalizations`.
async fn handle_list_subscription_version_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; tools.commerceListParams }", "") {
        version_id: String => "version_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }
    let opts = params.commerce_list_params.opts(50);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_subscription_version_localizations(&params.version_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list subscription version localizations: {e}"))),
    };
    let locs: Vec<InAppPurchaseLocalization> = resp
        .data
        .iter()
        .map(|loc| InAppPurchaseLocalization { id: loc.id.clone(), attributes: loc.attributes.clone(), ..Default::default() })
        .collect();
    Ok(list_result(&format_commerce_localizations(&locs), &resp.data, &resp.links))
}

/// Ported from `Registry.handleCreateSubscriptionLocalization`.
async fn handle_create_subscription_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; Locale string \"json:\\\"locale\\\"\"; Name string \"json:\\\"name\\\"\"; Description string \"json:\\\"description\\\"\" }", "") {
        version_id: String => "version_id";
        locale: String => "locale";
        name: String => "name";
        description: String => "description";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }
    if params.locale.is_empty() {
        return Ok(error_result("locale is required"));
    }
    if params.name.is_empty() {
        return Ok(error_result("name is required"));
    }
    let req = SubscriptionLocalizationCreateRequest {
        data: SubscriptionLocalizationCreateData {
            type_: "subscriptionLocalizations".into(),
            attributes: InAppPurchaseLocalizationCreateAttributes {
                name: params.name.clone(),
                locale: params.locale.clone(),
                description: params.description.clone(),
            },
            relationships: SubscriptionLocalizationRelationships { version: rel("subscriptionVersions", &params.version_id) },
        },
    };
    let resp = match c.create_subscription_localization(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create subscription localization: {e}"))),
    };
    let row = InAppPurchaseLocalization { id: resp.data.id.clone(), attributes: resp.data.attributes.clone(), ..Default::default() };
    Ok(data_result(&format!("Subscription localization created:\n{}", format_commerce_localization(&row)), &resp.data))
}

/// Ported from `Registry.handleUpdateSubscriptionLocalization`.
async fn handle_update_subscription_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Name string \"json:\\\"name\\\"\"; Description string \"json:\\\"description\\\"\" }", "") {
        localization_id: String => "localization_id";
        name: String => "name";
        description: String => "description";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }
    let req = SubscriptionLocalizationUpdateRequest {
        data: SubscriptionLocalizationUpdateData {
            type_: "subscriptionLocalizations".into(),
            id: params.localization_id.clone(),
            attributes: InAppPurchaseLocalizationUpdateAttributes { name: params.name.clone(), description: params.description.clone() },
        },
    };
    let resp = match c.update_subscription_localization(&params.localization_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update subscription localization: {e}"))),
    };
    let row = InAppPurchaseLocalization { id: resp.data.id.clone(), attributes: resp.data.attributes.clone(), ..Default::default() };
    Ok(data_result(&format!("Subscription localization updated:\n{}", format_commerce_localization(&row)), &resp.data))
}

/// Ported from `Registry.handleDeleteSubscriptionLocalization`.
async fn handle_delete_subscription_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }
    if let Err(e) = c.delete_subscription_localization(&params.localization_id).await {
        return Ok(error_result(format!("Failed to delete subscription localization: {e}")));
    }
    Ok(CallToolResult::success("Subscription localization deleted successfully"))
}

/// Ported from `Registry.handleListSubscriptionVersionImages`.
async fn handle_list_subscription_version_images(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; tools.commerceListParams }", "") {
        version_id: String => "version_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }
    let opts = params.commerce_list_params.opts(50);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_subscription_version_images(&params.version_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list subscription version images: {e}"))),
    };
    let images: Vec<CommerceImageRow> = resp.data.iter().map(|img| image_row(&img.id, &img.attributes)).collect();
    Ok(list_result(&format_commerce_images(&images), &resp.data, &resp.links))
}

/// Ported from `Registry.handleCreateSubscriptionImage`.
async fn handle_create_subscription_image(c: Arc<Client>, args: Args) -> HandlerResult {
    let params = match parse_commerce_image_create(&args) {
        Ok(p) => p,
        Err(r) => return Ok(r),
    };
    let req = SubscriptionImageCreateRequest {
        data: SubscriptionImageCreateData {
            type_: "subscriptionImages".into(),
            attributes: CommerceImageCreateAttributes { file_name: params.file_name.clone(), file_size: params.file_size },
            relationships: SubscriptionLocalizationRelationships { version: rel("subscriptionVersions", &params.version_id) },
        },
    };
    let resp = match c.create_subscription_image(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create subscription image: {e}"))),
    };
    Ok(data_result(&format!("Subscription image reserved:\n{}", format_commerce_image(&image_row(&resp.data.id, &resp.data.attributes))), &resp.data))
}

/// Ported from `Registry.handleUpdateSubscriptionImage`.
async fn handle_update_subscription_image(c: Arc<Client>, args: Args) -> HandlerResult {
    let (image_id, uploaded) = match parse_commerce_image_update(&args) {
        Ok(p) => p,
        Err(r) => return Ok(r),
    };
    let req = SubscriptionImageUpdateRequest {
        data: SubscriptionImageUpdateData {
            type_: "subscriptionImages".into(),
            id: image_id.clone(),
            attributes: CommerceImageUpdateAttributes { uploaded },
        },
    };
    let resp = match c.update_subscription_image(&image_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update subscription image: {e}"))),
    };
    Ok(data_result(&format!("Subscription image updated:\n{}", format_commerce_image(&image_row(&resp.data.id, &resp.data.attributes))), &resp.data))
}

/// Ported from `Registry.handleDeleteSubscriptionImage`.
async fn handle_delete_subscription_image(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ImageID string \"json:\\\"image_id\\\"\" }", "") {
        image_id: String => "image_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.image_id.is_empty() {
        return Ok(error_result("image_id is required"));
    }
    if let Err(e) = c.delete_subscription_image(&params.image_id).await {
        return Ok(error_result(format!("Failed to delete subscription image: {e}")));
    }
    Ok(CallToolResult::success("Subscription image deleted successfully"))
}

/// Ported from `Registry.handleCreateSubscriptionGroupVersion`.
async fn handle_create_subscription_group_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GroupID string \"json:\\\"group_id\\\"\" }", "") {
        group_id: String => "group_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.group_id.is_empty() {
        return Ok(error_result("group_id is required"));
    }
    let req = SubscriptionGroupVersionCreateRequest {
        data: SubscriptionGroupVersionCreateData {
            type_: "subscriptionGroupVersions".into(),
            relationships: SubscriptionGroupVersionCreateRelationships { subscription_group: rel("subscriptionGroups", &params.group_id) },
        },
    };
    let resp = match c.create_subscription_group_version(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create subscription group version: {e}"))),
    };
    Ok(data_result(&format!("Subscription group version created:\n{}", format_commerce_version(&resp.data.id, &resp.data.attributes)), &resp.data))
}

/// Ported from `Registry.handleGetSubscriptionGroupVersion`.
async fn handle_get_subscription_group_version(c: Arc<Client>, args: Args) -> HandlerResult {
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
    let resp = match c.get_subscription_group_version(&params.version_id).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to get subscription group version: {e}"))),
    };
    Ok(data_result(&format_commerce_version(&resp.data.id, &resp.data.attributes), &resp.data))
}

/// Ported from `Registry.handleListSubscriptionGroupVersions`.
async fn handle_list_subscription_group_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GroupID string \"json:\\\"group_id\\\"\"; tools.commerceListParams }", "") {
        group_id: String => "group_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.group_id.is_empty() {
        return Ok(error_result("group_id is required"));
    }
    let opts = params.commerce_list_params.opts(50);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_subscription_group_versions(&params.group_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list subscription group versions: {e}"))),
    };
    let versions = version_rows(resp.data.iter().map(|v| (&v.id, &v.attributes)));
    Ok(list_result(&format_commerce_versions(&versions), &resp.data, &resp.links))
}

/// Ported from `Registry.handleListSubscriptionGroupVersionLocalizations`.
async fn handle_list_subscription_group_version_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; tools.commerceListParams }", "") {
        version_id: String => "version_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }
    let opts = params.commerce_list_params.opts(50);
    let resp = match paginated_fetch(&c, &params.commerce_list_params.cursor, || c.list_subscription_group_version_localizations(&params.version_id, opts.as_ref())).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to list subscription group version localizations: {e}"))),
    };
    Ok(list_result(&format_subscription_group_localizations(&resp.data), &resp.data, &resp.links))
}

/// Ported from `Registry.handleCreateSubscriptionGroupLocalization`.
async fn handle_create_subscription_group_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; Locale string \"json:\\\"locale\\\"\"; Name string \"json:\\\"name\\\"\"; CustomAppName string \"json:\\\"custom_app_name\\\"\" }", "") {
        version_id: String => "version_id";
        locale: String => "locale";
        name: String => "name";
        custom_app_name: String => "custom_app_name";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.version_id.is_empty() {
        return Ok(error_result("version_id is required"));
    }
    if params.locale.is_empty() {
        return Ok(error_result("locale is required"));
    }
    if params.name.is_empty() {
        return Ok(error_result("name is required"));
    }
    let req = SubscriptionGroupLocalizationCreateRequest {
        data: SubscriptionGroupLocalizationCreateData {
            type_: "subscriptionGroupLocalizations".into(),
            attributes: SubscriptionGroupLocalizationCreateAttributes {
                name: params.name.clone(),
                locale: params.locale.clone(),
                custom_app_name: params.custom_app_name.clone(),
            },
            relationships: SubscriptionLocalizationRelationships { version: rel("subscriptionGroupVersions", &params.version_id) },
        },
    };
    let resp = match c.create_subscription_group_localization(&req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to create subscription group localization: {e}"))),
    };
    Ok(data_result(&format!("Subscription group localization created:\n{}", format_subscription_group_localization(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleUpdateSubscriptionGroupLocalization`.
async fn handle_update_subscription_group_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\"; Name string \"json:\\\"name\\\"\"; CustomAppName string \"json:\\\"custom_app_name\\\"\" }", "") {
        localization_id: String => "localization_id";
        name: String => "name";
        custom_app_name: String => "custom_app_name";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }
    let req = SubscriptionGroupLocalizationUpdateRequest {
        data: SubscriptionGroupLocalizationUpdateData {
            type_: "subscriptionGroupLocalizations".into(),
            id: params.localization_id.clone(),
            attributes: SubscriptionGroupLocalizationUpdateAttributes {
                name: params.name.clone(),
                custom_app_name: params.custom_app_name.clone(),
            },
        },
    };
    let resp = match c.update_subscription_group_localization(&params.localization_id, &req).await {
        Ok(r) => r,
        Err(e) => return Ok(error_result(format!("Failed to update subscription group localization: {e}"))),
    };
    Ok(data_result(&format!("Subscription group localization updated:\n{}", format_subscription_group_localization(&resp.data)), &resp.data))
}

/// Ported from `Registry.handleDeleteSubscriptionGroupLocalization`.
async fn handle_delete_subscription_group_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(format!("invalid arguments: {e}"));
    }
    if params.localization_id.is_empty() {
        return Ok(error_result("localization_id is required"));
    }
    if let Err(e) = c.delete_subscription_group_localization(&params.localization_id).await {
        return Ok(error_result(format!("Failed to delete subscription group localization: {e}")));
    }
    Ok(CallToolResult::success("Subscription group localization deleted successfully"))
}

impl CommerceListParams {
    /// Go `commerceListParams.opts`: clamps the limit the same way every
    /// other list tool does and assembles the query options.
    pub(crate) fn opts(&self, default_limit: i64) -> Option<ListOptions> {
        let mut limit = self.limit;
        if limit <= 0 {
            limit = default_limit;
        }
        if limit > 200 {
            limit = 200;
        }
        list_opts(limit, &self.filter, &GoSlice::nil(), &self.fields, &self.include)
    }
}

/// `api.RelationshipData{Data: api.ResourceIdentifier{Type: t, ID: id}}`.
pub(crate) fn rel(t: &str, id: &str) -> RelationshipData {
    RelationshipData { data: ResourceIdentifier { type_: t.to_string(), id: id.to_string() } }
}

/// Go `parseCommerceImageCreate`.
// Mirrors Go's (value, *mcp.ToolsCallResult) return pair.
#[allow(clippy::result_large_err)]
fn parse_commerce_image_create(args: &Args) -> Result<CommerceImageCreateParams, CallToolResult> {
    let mut params = CommerceImageCreateParams::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(error_result(format!("invalid arguments: {e}")));
    }
    if params.version_id.is_empty() {
        return Err(error_result("version_id is required"));
    }
    if params.file_name.is_empty() {
        return Err(error_result("file_name is required"));
    }
    if params.file_size <= 0 {
        return Err(error_result("file_size is required"));
    }
    Ok(params)
}

/// Go `parseCommerceImageUpdate`.
// Mirrors Go's (value, *mcp.ToolsCallResult) return pair.
#[allow(clippy::result_large_err)]
fn parse_commerce_image_update(args: &Args) -> Result<(String, Option<bool>), CallToolResult> {
    go_struct! {
    struct Params ("struct { ImageID string \"json:\\\"image_id\\\"\"; Uploaded *bool \"json:\\\"uploaded\\\"\" }", "") {
        image_id: String => "image_id";
        uploaded: Option<bool> => "uploaded";
    }
    }
    let mut params = Params::default();
    if let Err(e) = unmarshal_args(args.as_ref(), &mut params) {
        return Err(error_result(format!("invalid arguments: {e}")));
    }
    if params.image_id.is_empty() {
        return Err(error_result("image_id is required"));
    }
    let uploaded = params.uploaded.or(Some(true));
    Ok((params.image_id, uploaded))
}

/// Go `formatCommerceVersions`.
fn format_commerce_versions(versions: &[CommerceVersionRow]) -> String {
    if versions.is_empty() {
        return "No versions found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} versions:\n\n", versions.len());
    for v in versions {
        sb.push_str(&format_commerce_version(&v.id, &v.attributes));
        sb.push_str("\n---\n");
    }
    sb
}

/// Go `formatCommerceVersion`.
fn format_commerce_version(id: &str, attrs: &CommerceVersionAttributes) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", id);
    if attrs.version > 0 {
        let _ = writeln!(sb, "Version: {}", attrs.version);
    }
    if !attrs.state.is_empty() {
        let _ = writeln!(sb, "State: {}", attrs.state);
    }
    sb
}

/// Go `formatCommerceLocalizations`.
fn format_commerce_localizations(locs: &[InAppPurchaseLocalization]) -> String {
    if locs.is_empty() {
        return "No localizations found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} localizations:\n\n", locs.len());
    for loc in locs {
        sb.push_str(&format_commerce_localization(loc));
        sb.push_str("\n---\n");
    }
    sb
}

/// Go `formatCommerceLocalization`.
fn format_commerce_localization(loc: &InAppPurchaseLocalization) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", loc.id);
    let _ = writeln!(sb, "Locale: {}", loc.attributes.locale);
    let _ = writeln!(sb, "Name: {}", loc.attributes.name);
    if !loc.attributes.description.is_empty() {
        let _ = writeln!(sb, "Description: {}", loc.attributes.description);
    }
    if !loc.attributes.state.is_empty() {
        let _ = writeln!(sb, "State: {}", loc.attributes.state);
    }
    sb
}

/// Go `formatSubscriptionGroupLocalizations`.
fn format_subscription_group_localizations(locs: &[SubscriptionGroupLocalization]) -> String {
    if locs.is_empty() {
        return "No subscription group localizations found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} subscription group localizations:\n\n", locs.len());
    for loc in locs {
        sb.push_str(&format_subscription_group_localization(loc));
        sb.push_str("\n---\n");
    }
    sb
}

/// Go `formatSubscriptionGroupLocalization`.
fn format_subscription_group_localization(loc: &SubscriptionGroupLocalization) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", loc.id);
    let _ = writeln!(sb, "Locale: {}", loc.attributes.locale);
    let _ = writeln!(sb, "Name: {}", loc.attributes.name);
    if !loc.attributes.custom_app_name.is_empty() {
        let _ = writeln!(sb, "Custom App Name: {}", loc.attributes.custom_app_name);
    }
    if !loc.attributes.state.is_empty() {
        let _ = writeln!(sb, "State: {}", loc.attributes.state);
    }
    sb
}

/// Go `formatCommerceImages`.
fn format_commerce_images(images: &[CommerceImageRow]) -> String {
    if images.is_empty() {
        return "No images found".to_string();
    }
    let mut sb = String::new();
    let _ = write!(sb, "Found {} images:\n\n", images.len());
    for img in images {
        sb.push_str(&format_commerce_image(img));
        sb.push_str("\n---\n");
    }
    sb
}

/// Go `formatCommerceImage`.
fn format_commerce_image(img: &CommerceImageRow) -> String {
    let mut sb = String::new();
    let _ = writeln!(sb, "ID: {}", img.id);
    if !img.attributes.file_name.is_empty() {
        let _ = writeln!(sb, "File Name: {}", img.attributes.file_name);
    }
    if img.attributes.file_size > 0 {
        let _ = writeln!(sb, "File Size: {} bytes", img.attributes.file_size);
    }
    if !img.attributes.state.is_empty() {
        let _ = writeln!(sb, "State: {}", img.attributes.state);
    }
    if !img.attributes.upload_operations.is_empty() {
        let _ = writeln!(sb, "Upload Operations: {}", img.attributes.upload_operations.len());
    }
    sb
}

fn image_row(id: &str, attributes: &CommerceImageAttributes) -> CommerceImageRow {
    CommerceImageRow { id: id.to_string(), attributes: attributes.clone() }
}

fn version_rows<'a>(it: impl Iterator<Item = (&'a String, &'a CommerceVersionAttributes)>) -> Vec<CommerceVersionRow> {
    it.map(|(id, a)| CommerceVersionRow { id: id.clone(), attributes: a.clone() }).collect()
}
