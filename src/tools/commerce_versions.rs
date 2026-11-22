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
    todo!("port handleCreateInAppPurchaseVersion from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleGetInAppPurchaseVersion`.
async fn handle_get_in_app_purchase_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    todo!("port handleGetInAppPurchaseVersion from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleListInAppPurchaseVersions`.
async fn handle_list_in_app_purchase_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { IAPID string \"json:\\\"iap_id\\\"\"; tools.commerceListParams }", "") {
        iap_id: String => "iap_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListInAppPurchaseVersions from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleListInAppPurchaseVersionLocalizations`.
async fn handle_list_in_app_purchase_version_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; tools.commerceListParams }", "") {
        version_id: String => "version_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListInAppPurchaseVersionLocalizations from internal/asc/tools/commerce_versions.go")
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
    todo!("port handleCreateInAppPurchaseLocalization from internal/asc/tools/commerce_versions.go")
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
    todo!("port handleUpdateInAppPurchaseLocalization from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleDeleteInAppPurchaseLocalization`.
async fn handle_delete_in_app_purchase_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    todo!("port handleDeleteInAppPurchaseLocalization from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleListInAppPurchaseVersionImages`.
async fn handle_list_in_app_purchase_version_images(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; tools.commerceListParams }", "") {
        version_id: String => "version_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListInAppPurchaseVersionImages from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleCreateInAppPurchaseImage`.
async fn handle_create_in_app_purchase_image(c: Arc<Client>, args: Args) -> HandlerResult {
    todo!("port handleCreateInAppPurchaseImage from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleUpdateInAppPurchaseImage`.
async fn handle_update_in_app_purchase_image(c: Arc<Client>, args: Args) -> HandlerResult {
    todo!("port handleUpdateInAppPurchaseImage from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleDeleteInAppPurchaseImage`.
async fn handle_delete_in_app_purchase_image(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ImageID string \"json:\\\"image_id\\\"\" }", "") {
        image_id: String => "image_id";
    }
    }
    todo!("port handleDeleteInAppPurchaseImage from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleCreateSubscriptionVersion`.
async fn handle_create_subscription_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubscriptionID string \"json:\\\"subscription_id\\\"\" }", "") {
        subscription_id: String => "subscription_id";
    }
    }
    todo!("port handleCreateSubscriptionVersion from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleGetSubscriptionVersion`.
async fn handle_get_subscription_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    todo!("port handleGetSubscriptionVersion from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleListSubscriptionVersions`.
async fn handle_list_subscription_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { SubscriptionID string \"json:\\\"subscription_id\\\"\"; tools.commerceListParams }", "") {
        subscription_id: String => "subscription_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListSubscriptionVersions from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleListSubscriptionVersionLocalizations`.
async fn handle_list_subscription_version_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; tools.commerceListParams }", "") {
        version_id: String => "version_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListSubscriptionVersionLocalizations from internal/asc/tools/commerce_versions.go")
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
    todo!("port handleCreateSubscriptionLocalization from internal/asc/tools/commerce_versions.go")
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
    todo!("port handleUpdateSubscriptionLocalization from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleDeleteSubscriptionLocalization`.
async fn handle_delete_subscription_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    todo!("port handleDeleteSubscriptionLocalization from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleListSubscriptionVersionImages`.
async fn handle_list_subscription_version_images(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; tools.commerceListParams }", "") {
        version_id: String => "version_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListSubscriptionVersionImages from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleCreateSubscriptionImage`.
async fn handle_create_subscription_image(c: Arc<Client>, args: Args) -> HandlerResult {
    todo!("port handleCreateSubscriptionImage from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleUpdateSubscriptionImage`.
async fn handle_update_subscription_image(c: Arc<Client>, args: Args) -> HandlerResult {
    todo!("port handleUpdateSubscriptionImage from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleDeleteSubscriptionImage`.
async fn handle_delete_subscription_image(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { ImageID string \"json:\\\"image_id\\\"\" }", "") {
        image_id: String => "image_id";
    }
    }
    todo!("port handleDeleteSubscriptionImage from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleCreateSubscriptionGroupVersion`.
async fn handle_create_subscription_group_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GroupID string \"json:\\\"group_id\\\"\" }", "") {
        group_id: String => "group_id";
    }
    }
    todo!("port handleCreateSubscriptionGroupVersion from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleGetSubscriptionGroupVersion`.
async fn handle_get_subscription_group_version(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\" }", "") {
        version_id: String => "version_id";
    }
    }
    todo!("port handleGetSubscriptionGroupVersion from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleListSubscriptionGroupVersions`.
async fn handle_list_subscription_group_versions(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { GroupID string \"json:\\\"group_id\\\"\"; tools.commerceListParams }", "") {
        group_id: String => "group_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListSubscriptionGroupVersions from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleListSubscriptionGroupVersionLocalizations`.
async fn handle_list_subscription_group_version_localizations(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { VersionID string \"json:\\\"version_id\\\"\"; tools.commerceListParams }", "") {
        version_id: String => "version_id";
        commerce_list_params: CommerceListParams => "commerceListParams", flatten;
    }
    }
    todo!("port handleListSubscriptionGroupVersionLocalizations from internal/asc/tools/commerce_versions.go")
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
    todo!("port handleCreateSubscriptionGroupLocalization from internal/asc/tools/commerce_versions.go")
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
    todo!("port handleUpdateSubscriptionGroupLocalization from internal/asc/tools/commerce_versions.go")
}

/// Ported from `Registry.handleDeleteSubscriptionGroupLocalization`.
async fn handle_delete_subscription_group_localization(c: Arc<Client>, args: Args) -> HandlerResult {
    go_struct! {
    struct Params ("struct { LocalizationID string \"json:\\\"localization_id\\\"\" }", "") {
        localization_id: String => "localization_id";
    }
    }
    todo!("port handleDeleteSubscriptionGroupLocalization from internal/asc/tools/commerce_versions.go")
}

