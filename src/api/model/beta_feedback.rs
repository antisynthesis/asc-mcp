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
/// BetaFeedbackCrashSubmissionsResponse represents a list of crash
/// feedback submissions.
pub struct BetaFeedbackCrashSubmissionsResponse ("api.BetaFeedbackCrashSubmissionsResponse", "BetaFeedbackCrashSubmissionsResponse") {
    pub data: JsonSlice<BetaFeedbackCrashSubmission> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaFeedbackCrashSubmissionResponse represents a single crash
/// feedback submission.
pub struct BetaFeedbackCrashSubmissionResponse ("api.BetaFeedbackCrashSubmissionResponse", "BetaFeedbackCrashSubmissionResponse") {
    pub data: BetaFeedbackCrashSubmission => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaFeedbackCrashSubmission represents one crash report submitted by
/// a TestFlight tester.
pub struct BetaFeedbackCrashSubmission ("api.BetaFeedbackCrashSubmission", "BetaFeedbackCrashSubmission") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BetaFeedbackDeviceAttributes => "attributes";
    pub relationships: Option<BetaFeedbackSubmissionLinkage> => "relationships", omitempty;
}
}

json_struct! {
/// BetaFeedbackScreenshotSubmissionsResponse represents a list of
/// screenshot feedback submissions.
pub struct BetaFeedbackScreenshotSubmissionsResponse ("api.BetaFeedbackScreenshotSubmissionsResponse", "BetaFeedbackScreenshotSubmissionsResponse") {
    pub data: JsonSlice<BetaFeedbackScreenshotSubmission> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaFeedbackScreenshotSubmissionResponse represents a single
/// screenshot feedback submission.
pub struct BetaFeedbackScreenshotSubmissionResponse ("api.BetaFeedbackScreenshotSubmissionResponse", "BetaFeedbackScreenshotSubmissionResponse") {
    pub data: BetaFeedbackScreenshotSubmission => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaFeedbackScreenshotSubmission represents one screenshot feedback
/// item submitted by a TestFlight tester. The Screenshots slice carries
/// time-limited download URLs for the annotated images.
pub struct BetaFeedbackScreenshotSubmission ("api.BetaFeedbackScreenshotSubmission", "BetaFeedbackScreenshotSubmission") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BetaFeedbackDeviceAttributes => "attributes";
    pub relationships: Option<BetaFeedbackSubmissionLinkage> => "relationships", omitempty;
}
}

json_struct! {
/// BetaFeedbackDeviceAttributes contains the tester comment plus the
/// device and environment snapshot Apple captures with each feedback
/// submission. Crash and screenshot submissions share this shape; only
/// screenshot submissions populate Screenshots.
pub struct BetaFeedbackDeviceAttributes ("api.BetaFeedbackDeviceAttributes", "BetaFeedbackDeviceAttributes") {
    pub created_date: Option<Timestamp> => "createdDate", omitempty;
    pub comment: String => "comment", omitempty;
    pub email: String => "email", omitempty;
    pub device_model: String => "deviceModel", omitempty;
    pub os_version: String => "osVersion", omitempty;
    pub locale: String => "locale", omitempty;
    pub time_zone: String => "timeZone", omitempty;
    pub architecture: String => "architecture", omitempty;
    pub connection_type: String => "connectionType", omitempty;
    pub paired_apple_watch: String => "pairedAppleWatch", omitempty;
    pub app_uptime_in_milliseconds: i64 => "appUptimeInMilliseconds" as "int64", omitempty;
    pub disk_bytes_available: i64 => "diskBytesAvailable" as "int64", omitempty;
    pub disk_bytes_total: i64 => "diskBytesTotal" as "int64", omitempty;
    pub battery_percentage: i64 => "batteryPercentage", omitempty;
    pub screen_width_in_points: i64 => "screenWidthInPoints", omitempty;
    pub screen_height_in_points: i64 => "screenHeightInPoints", omitempty;
    pub app_platform: String => "appPlatform", omitempty;
    pub device_platform: String => "devicePlatform", omitempty;
    pub device_family: String => "deviceFamily", omitempty;
    pub build_bundle_id: String => "buildBundleId", omitempty;
    pub screenshots: JsonSlice<BetaFeedbackScreenshotImage> => "screenshots", omitempty;
}
}

json_struct! {
/// BetaFeedbackScreenshotImage is one downloadable screenshot image with
/// its dimensions and URL expiration.
pub struct BetaFeedbackScreenshotImage ("api.BetaFeedbackScreenshotImage", "BetaFeedbackScreenshotImage") {
    pub url: String => "url", omitempty;
    pub width: i64 => "width", omitempty;
    pub height: i64 => "height", omitempty;
    pub expiration_date: Option<Timestamp> => "expirationDate", omitempty;
}
}

json_struct! {
/// BetaFeedbackSubmissionLinkage exposes the build and tester a feedback
/// submission belongs to.
pub struct BetaFeedbackSubmissionLinkage ("api.BetaFeedbackSubmissionLinkage", "BetaFeedbackSubmissionLinkage") {
    pub build: Option<RelationshipData> => "build", omitempty;
    pub tester: Option<RelationshipData> => "tester", omitempty;
}
}

json_struct! {
/// BetaCrashLogResponse represents the crash log attached to a crash
/// feedback submission.
pub struct BetaCrashLogResponse ("api.BetaCrashLogResponse", "BetaCrashLogResponse") {
    pub data: BetaCrashLog => "data";
}
}

json_struct! {
/// BetaCrashLog carries the symbolicated crash log text.
pub struct BetaCrashLog ("api.BetaCrashLog", "BetaCrashLog") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BetaCrashLogAttributes => "attributes";
}
}

json_struct! {
/// BetaCrashLogAttributes contains the crash log text.
pub struct BetaCrashLogAttributes ("api.BetaCrashLogAttributes", "BetaCrashLogAttributes") {
    pub log_text: String => "logText", omitempty;
}
}

json_struct! {
/// BetaBuildUsagesResponse represents TestFlight usage metrics for one
/// build (install, session, crash, feedback, and invite counts).
pub struct BetaBuildUsagesResponse ("api.BetaBuildUsagesResponse", "BetaBuildUsagesResponse") {
    pub data: JsonSlice<BetaBuildUsageGroup> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
}
}

json_struct! {
/// BetaBuildUsageGroup is one group of usage data points.
pub struct BetaBuildUsageGroup ("api.BetaBuildUsageGroup", "BetaBuildUsageGroup") {
    pub data_points: JsonSlice<BetaBuildUsageDataPoint> => "dataPoints", omitempty;
}
}

json_struct! {
/// BetaBuildUsageDataPoint is one measured interval of build usage.
pub struct BetaBuildUsageDataPoint ("api.BetaBuildUsageDataPoint", "BetaBuildUsageDataPoint") {
    pub start: Option<Timestamp> => "start", omitempty;
    pub end: Option<Timestamp> => "end", omitempty;
    pub values: BetaBuildUsageValues => "values";
}
}

json_struct! {
/// BetaBuildUsageValues carries the usage counters for a data point.
pub struct BetaBuildUsageValues ("api.BetaBuildUsageValues", "BetaBuildUsageValues") {
    pub crash_count: i64 => "crashCount";
    pub install_count: i64 => "installCount";
    pub session_count: i64 => "sessionCount";
    pub feedback_count: i64 => "feedbackCount";
    pub invite_count: i64 => "inviteCount";
}
}
