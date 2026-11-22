// App Store Connect API resource and request models. Fields encode in
// declaration order and carry their JSON key and omitempty flag; do not
// reorder fields.

#![allow(missing_docs)]

#[allow(unused_imports)]
use super::*;
use crate::json_struct;
#[allow(unused_imports)]
use crate::wire::{Base64Bytes, JsonMap, JsonSlice, RawMessage, Timestamp};
#[allow(unused_imports)]
use serde_json::Value;

json_struct! {
/// WebhooksResponse represents a list of webhooks.
pub struct WebhooksResponse ("api.WebhooksResponse", "WebhooksResponse") {
    pub data: JsonSlice<Webhook> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// WebhookResponse represents a single webhook.
pub struct WebhookResponse ("api.WebhookResponse", "WebhookResponse") {
    pub data: Webhook => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// Webhook represents a webhook configured for an app.
pub struct Webhook ("api.Webhook", "Webhook") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: WebhookAttributes => "attributes";
}
}

json_struct! {
/// WebhookAttributes contains webhook attributes.
pub struct WebhookAttributes ("api.WebhookAttributes", "WebhookAttributes") {
    pub enabled: bool => "enabled", omitempty;
    pub event_types: JsonSlice<String> => "eventTypes", omitempty;
    pub name: String => "name", omitempty;
    pub url: String => "url", omitempty;
}
}

json_struct! {
/// WebhookCreateRequest represents a request to create a webhook.
pub struct WebhookCreateRequest ("api.WebhookCreateRequest", "WebhookCreateRequest") {
    pub data: WebhookCreateData => "data";
}
}

json_struct! {
/// WebhookCreateData contains the data for creating a webhook.
pub struct WebhookCreateData ("api.WebhookCreateData", "WebhookCreateData") {
    pub type_: String => "type";
    pub attributes: WebhookCreateAttributes => "attributes";
    pub relationships: WebhookCreateRelationships => "relationships";
}
}

json_struct! {
/// WebhookCreateAttributes contains attributes for creating a webhook.
/// All fields are required by the API.
pub struct WebhookCreateAttributes ("api.WebhookCreateAttributes", "WebhookCreateAttributes") {
    pub enabled: bool => "enabled";
    pub event_types: JsonSlice<String> => "eventTypes";
    pub name: String => "name";
    pub secret: String => "secret";
    pub url: String => "url";
}
}

json_struct! {
/// WebhookCreateRelationships contains relationships for creating a webhook.
pub struct WebhookCreateRelationships ("api.WebhookCreateRelationships", "WebhookCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// WebhookUpdateRequest represents a request to update a webhook.
pub struct WebhookUpdateRequest ("api.WebhookUpdateRequest", "WebhookUpdateRequest") {
    pub data: WebhookUpdateData => "data";
}
}

json_struct! {
/// WebhookUpdateData contains the data for updating a webhook.
pub struct WebhookUpdateData ("api.WebhookUpdateData", "WebhookUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: Option<WebhookUpdateAttributes> => "attributes", omitempty;
}
}

json_struct! {
/// WebhookUpdateAttributes contains attributes for updating a webhook.
pub struct WebhookUpdateAttributes ("api.WebhookUpdateAttributes", "WebhookUpdateAttributes") {
    pub enabled: Option<bool> => "enabled", omitempty;
    pub event_types: JsonSlice<String> => "eventTypes", omitempty;
    pub name: String => "name", omitempty;
    pub secret: String => "secret", omitempty;
    pub url: String => "url", omitempty;
}
}

json_struct! {
/// WebhookDeliveriesResponse represents a list of webhook deliveries.
pub struct WebhookDeliveriesResponse ("api.WebhookDeliveriesResponse", "WebhookDeliveriesResponse") {
    pub data: JsonSlice<WebhookDelivery> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// WebhookDeliveryResponse represents a single webhook delivery.
pub struct WebhookDeliveryResponse ("api.WebhookDeliveryResponse", "WebhookDeliveryResponse") {
    pub data: WebhookDelivery => "data";
}
}

json_struct! {
/// WebhookDelivery represents a delivery attempt for a webhook event.
pub struct WebhookDelivery ("api.WebhookDelivery", "WebhookDelivery") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: WebhookDeliveryAttributes => "attributes";
}
}

json_struct! {
/// WebhookDeliveryAttributes contains webhook delivery attributes.
pub struct WebhookDeliveryAttributes ("api.WebhookDeliveryAttributes", "WebhookDeliveryAttributes") {
    pub created_date: Option<Timestamp> => "createdDate", omitempty;
    pub delivery_state: String => "deliveryState", omitempty;
    pub error_message: String => "errorMessage", omitempty;
    pub redelivery: bool => "redelivery", omitempty;
    pub sent_date: Option<Timestamp> => "sentDate", omitempty;
    pub request: Option<WebhookDeliveryRequestInfo> => "request", omitempty;
    pub response: Option<WebhookDeliveryResponseInfo> => "response", omitempty;
}
}

json_struct! {
/// WebhookDeliveryRequestInfo describes the outbound request of a delivery.
pub struct WebhookDeliveryRequestInfo ("api.WebhookDeliveryRequestInfo", "WebhookDeliveryRequestInfo") {
    pub url: String => "url", omitempty;
}
}

json_struct! {
/// WebhookDeliveryResponseInfo describes the endpoint's response to a delivery.
pub struct WebhookDeliveryResponseInfo ("api.WebhookDeliveryResponseInfo", "WebhookDeliveryResponseInfo") {
    pub http_status_code: i64 => "httpStatusCode", omitempty;
    pub body: String => "body", omitempty;
}
}

json_struct! {
/// WebhookDeliveryCreateRequest represents a request to redeliver a webhook
/// event using a previous delivery as the template.
pub struct WebhookDeliveryCreateRequest ("api.WebhookDeliveryCreateRequest", "WebhookDeliveryCreateRequest") {
    pub data: WebhookDeliveryCreateData => "data";
}
}

json_struct! {
/// WebhookDeliveryCreateData contains the data for creating a redelivery.
pub struct WebhookDeliveryCreateData ("api.WebhookDeliveryCreateData", "WebhookDeliveryCreateData") {
    pub type_: String => "type";
    pub relationships: WebhookDeliveryCreateRelationships => "relationships";
}
}

json_struct! {
/// WebhookDeliveryCreateRelationships contains relationships for a redelivery.
pub struct WebhookDeliveryCreateRelationships ("api.WebhookDeliveryCreateRelationships", "WebhookDeliveryCreateRelationships") {
    pub template: RelationshipData => "template";
}
}

json_struct! {
/// WebhookPingResponse represents a single webhook ping.
pub struct WebhookPingResponse ("api.WebhookPingResponse", "WebhookPingResponse") {
    pub data: WebhookPing => "data";
}
}

json_struct! {
/// WebhookPing represents a test event sent to a webhook endpoint.
pub struct WebhookPing ("api.WebhookPing", "WebhookPing") {
    pub type_: String => "type";
    pub id: String => "id";
}
}

json_struct! {
/// WebhookPingCreateRequest represents a request to send a test ping event.
pub struct WebhookPingCreateRequest ("api.WebhookPingCreateRequest", "WebhookPingCreateRequest") {
    pub data: WebhookPingCreateData => "data";
}
}

json_struct! {
/// WebhookPingCreateData contains the data for creating a webhook ping.
pub struct WebhookPingCreateData ("api.WebhookPingCreateData", "WebhookPingCreateData") {
    pub type_: String => "type";
    pub relationships: WebhookPingCreateRelationships => "relationships";
}
}

json_struct! {
/// WebhookPingCreateRelationships contains relationships for a webhook ping.
pub struct WebhookPingCreateRelationships ("api.WebhookPingCreateRelationships", "WebhookPingCreateRelationships") {
    pub webhook: RelationshipData => "webhook";
}
}
