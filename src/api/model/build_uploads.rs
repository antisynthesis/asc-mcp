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
/// BuildUploadsResponse represents a list of build uploads.
pub struct BuildUploadsResponse ("api.BuildUploadsResponse", "BuildUploadsResponse") {
    pub data: JsonSlice<BuildUpload> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BuildUploadResponse represents a single build upload.
pub struct BuildUploadResponse ("api.BuildUploadResponse", "BuildUploadResponse") {
    pub data: BuildUpload => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BuildUpload represents an in-progress or completed build delivery.
pub struct BuildUpload ("api.BuildUpload", "BuildUpload") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BuildUploadAttributes => "attributes";
    pub relationships: Option<BuildUploadRelationships> => "relationships", omitempty;
}
}

json_struct! {
/// BuildUploadAttributes contains build upload attributes.
pub struct BuildUploadAttributes ("api.BuildUploadAttributes", "BuildUploadAttributes") {
    pub cf_bundle_short_version_string: String => "cfBundleShortVersionString", omitempty;
    pub cf_bundle_version: String => "cfBundleVersion", omitempty;
    pub created_date: Option<Timestamp> => "createdDate", omitempty;
    pub state: Option<BuildUploadState> => "state", omitempty;
    pub platform: String => "platform", omitempty;
    pub uploaded_date: Option<Timestamp> => "uploadedDate", omitempty;
}
}

json_struct! {
/// BuildUploadState describes upload processing progress. State is one of
/// AWAITING_UPLOAD, PROCESSING, FAILED, or COMPLETE; the detail slices
/// carry Apple's per-issue codes and descriptions.
pub struct BuildUploadState ("api.BuildUploadState", "BuildUploadState") {
    pub errors: JsonSlice<StateDetail> => "errors", omitempty;
    pub warnings: JsonSlice<StateDetail> => "warnings", omitempty;
    pub infos: JsonSlice<StateDetail> => "infos", omitempty;
    pub state: String => "state", omitempty;
}
}

json_struct! {
/// StateDetail is a single coded message inside a BuildUploadState.
pub struct StateDetail ("api.StateDetail", "StateDetail") {
    pub code: String => "code", omitempty;
    pub description: String => "description", omitempty;
}
}

json_struct! {
/// BuildUploadRelationships exposes the resources linked to a build
/// upload; Build is populated once Apple has processed the binary.
pub struct BuildUploadRelationships ("api.BuildUploadRelationships", "BuildUploadRelationships") {
    pub build: Option<RelationshipData> => "build", omitempty;
    pub asset_file: Option<RelationshipData> => "assetFile", omitempty;
}
}

json_struct! {
/// BuildUploadCreateRequest represents a request to create a build upload.
pub struct BuildUploadCreateRequest ("api.BuildUploadCreateRequest", "BuildUploadCreateRequest") {
    pub data: BuildUploadCreateData => "data";
}
}

json_struct! {
/// BuildUploadCreateData contains the data for creating a build upload.
pub struct BuildUploadCreateData ("api.BuildUploadCreateData", "BuildUploadCreateData") {
    pub type_: String => "type";
    pub attributes: BuildUploadCreateAttributes => "attributes";
    pub relationships: BuildUploadCreateRelationships => "relationships";
}
}

json_struct! {
/// BuildUploadCreateAttributes contains attributes for creating a build
/// upload. All fields are required by the API.
pub struct BuildUploadCreateAttributes ("api.BuildUploadCreateAttributes", "BuildUploadCreateAttributes") {
    pub cf_bundle_short_version_string: String => "cfBundleShortVersionString";
    pub cf_bundle_version: String => "cfBundleVersion";
    pub platform: String => "platform";
}
}

json_struct! {
/// BuildUploadCreateRelationships contains relationships for creating a
/// build upload.
pub struct BuildUploadCreateRelationships ("api.BuildUploadCreateRelationships", "BuildUploadCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// BuildUploadFilesResponse represents a list of build upload files.
pub struct BuildUploadFilesResponse ("api.BuildUploadFilesResponse", "BuildUploadFilesResponse") {
    pub data: JsonSlice<BuildUploadFile> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
}
}

json_struct! {
/// BuildUploadFileResponse represents a single build upload file.
pub struct BuildUploadFileResponse ("api.BuildUploadFileResponse", "BuildUploadFileResponse") {
    pub data: BuildUploadFile => "data";
}
}

json_struct! {
/// BuildUploadFile represents one asset file inside a build upload.
pub struct BuildUploadFile ("api.BuildUploadFile", "BuildUploadFile") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BuildUploadFileAttributes => "attributes";
}
}

json_struct! {
/// BuildUploadFileAttributes contains build upload file attributes,
/// including the upload operations issued at reservation time.
pub struct BuildUploadFileAttributes ("api.BuildUploadFileAttributes", "BuildUploadFileAttributes") {
    pub asset_delivery_state: Option<AssetDeliveryState> => "assetDeliveryState", omitempty;
    pub asset_token: String => "assetToken", omitempty;
    pub asset_type: String => "assetType", omitempty;
    pub file_name: String => "fileName", omitempty;
    pub file_size: i64 => "fileSize" as "int64", omitempty;
    pub source_file_checksums: Option<Checksums> => "sourceFileChecksums", omitempty;
    pub upload_operations: JsonSlice<UploadOperation> => "uploadOperations", omitempty;
    pub uti: String => "uti", omitempty;
}
}

json_struct! {
/// Checksums carries the file (and optional composite) checksum used to
/// commit a build upload file.
pub struct Checksums ("api.Checksums", "Checksums") {
    pub file: Option<Checksum> => "file", omitempty;
    pub composite: Option<Checksum> => "composite", omitempty;
}
}

json_struct! {
/// Checksum is a single hash value with its algorithm (MD5 or SHA_256).
pub struct Checksum ("api.Checksum", "Checksum") {
    pub hash: String => "hash", omitempty;
    pub algorithm: String => "algorithm", omitempty;
}
}

json_struct! {
/// BuildUploadFileCreateRequest represents a request to reserve a build
/// upload file.
pub struct BuildUploadFileCreateRequest ("api.BuildUploadFileCreateRequest", "BuildUploadFileCreateRequest") {
    pub data: BuildUploadFileCreateData => "data";
}
}

json_struct! {
/// BuildUploadFileCreateData contains the data for reserving a build
/// upload file.
pub struct BuildUploadFileCreateData ("api.BuildUploadFileCreateData", "BuildUploadFileCreateData") {
    pub type_: String => "type";
    pub attributes: BuildUploadFileCreateAttributes => "attributes";
    pub relationships: BuildUploadFileCreateRelationships => "relationships";
}
}

json_struct! {
/// BuildUploadFileCreateAttributes contains attributes for reserving a
/// build upload file. All fields are required by the API.
pub struct BuildUploadFileCreateAttributes ("api.BuildUploadFileCreateAttributes", "BuildUploadFileCreateAttributes") {
    pub asset_type: String => "assetType";
    pub file_name: String => "fileName";
    pub file_size: i64 => "fileSize" as "int64";
    pub uti: String => "uti";
}
}

json_struct! {
/// BuildUploadFileCreateRelationships contains relationships for
/// reserving a build upload file.
pub struct BuildUploadFileCreateRelationships ("api.BuildUploadFileCreateRelationships", "BuildUploadFileCreateRelationships") {
    pub build_upload: RelationshipData => "buildUpload";
}
}

json_struct! {
/// BuildUploadFileUpdateRequest represents the commit PATCH for a build
/// upload file.
pub struct BuildUploadFileUpdateRequest ("api.BuildUploadFileUpdateRequest", "BuildUploadFileUpdateRequest") {
    pub data: BuildUploadFileUpdateData => "data";
}
}

json_struct! {
/// BuildUploadFileUpdateData contains the data for committing a build
/// upload file.
pub struct BuildUploadFileUpdateData ("api.BuildUploadFileUpdateData", "BuildUploadFileUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: Option<BuildUploadFileUpdateAttributes> => "attributes", omitempty;
}
}

json_struct! {
/// BuildUploadFileUpdateAttributes contains attributes for committing a
/// build upload file.
pub struct BuildUploadFileUpdateAttributes ("api.BuildUploadFileUpdateAttributes", "BuildUploadFileUpdateAttributes") {
    pub source_file_checksums: Option<Checksums> => "sourceFileChecksums", omitempty;
    pub uploaded: Option<bool> => "uploaded", omitempty;
}
}
