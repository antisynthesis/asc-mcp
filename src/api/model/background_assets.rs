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
/// BackgroundAssetsResponse represents a list of background assets.
pub struct BackgroundAssetsResponse ("api.BackgroundAssetsResponse", "BackgroundAssetsResponse") {
    pub data: JsonSlice<BackgroundAsset> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BackgroundAssetResponse represents a single background asset.
pub struct BackgroundAssetResponse ("api.BackgroundAssetResponse", "BackgroundAssetResponse") {
    pub data: BackgroundAsset => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BackgroundAsset represents an Apple-hosted background asset pack.
pub struct BackgroundAsset ("api.BackgroundAsset", "BackgroundAsset") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BackgroundAssetAttributes => "attributes";
    pub relationships: Option<BackgroundAssetRelationships> => "relationships", omitempty;
}
}

json_struct! {
/// BackgroundAssetAttributes contains background asset attributes.
/// UsedBytes reports the storage consumed by the asset's versions
/// against the app's Apple-hosted quota.
pub struct BackgroundAssetAttributes ("api.BackgroundAssetAttributes", "BackgroundAssetAttributes") {
    pub archived: bool => "archived", omitempty;
    pub asset_pack_identifier: String => "assetPackIdentifier", omitempty;
    pub created_date: Option<Timestamp> => "createdDate", omitempty;
    pub used_bytes: i64 => "usedBytes" as "int64", omitempty;
}
}

json_struct! {
/// BackgroundAssetRelationships exposes the resources linked to a
/// background asset; the three version pointers identify which version
/// is currently live in each release channel.
pub struct BackgroundAssetRelationships ("api.BackgroundAssetRelationships", "BackgroundAssetRelationships") {
    pub app: Option<RelationshipData> => "app", omitempty;
    pub app_store_version: Option<RelationshipData> => "appStoreVersion", omitempty;
    pub internal_beta_version: Option<RelationshipData> => "internalBetaVersion", omitempty;
    pub external_beta_version: Option<RelationshipData> => "externalBetaVersion", omitempty;
}
}

json_struct! {
/// BackgroundAssetCreateRequest represents a request to create a
/// background asset.
pub struct BackgroundAssetCreateRequest ("api.BackgroundAssetCreateRequest", "BackgroundAssetCreateRequest") {
    pub data: BackgroundAssetCreateData => "data";
}
}

json_struct! {
/// BackgroundAssetCreateData contains the data for creating a background
/// asset.
pub struct BackgroundAssetCreateData ("api.BackgroundAssetCreateData", "BackgroundAssetCreateData") {
    pub type_: String => "type";
    pub attributes: BackgroundAssetCreateAttributes => "attributes";
    pub relationships: BackgroundAssetCreateRelationships => "relationships";
}
}

json_struct! {
/// BackgroundAssetCreateAttributes contains attributes for creating a
/// background asset. AssetPackIdentifier is required by the API.
pub struct BackgroundAssetCreateAttributes ("api.BackgroundAssetCreateAttributes", "BackgroundAssetCreateAttributes") {
    pub asset_pack_identifier: String => "assetPackIdentifier";
}
}

json_struct! {
/// BackgroundAssetCreateRelationships contains relationships for
/// creating a background asset.
pub struct BackgroundAssetCreateRelationships ("api.BackgroundAssetCreateRelationships", "BackgroundAssetCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// BackgroundAssetUpdateRequest represents a request to update a
/// background asset (archive or unarchive it).
pub struct BackgroundAssetUpdateRequest ("api.BackgroundAssetUpdateRequest", "BackgroundAssetUpdateRequest") {
    pub data: BackgroundAssetUpdateData => "data";
}
}

json_struct! {
/// BackgroundAssetUpdateData contains the data for updating a background
/// asset.
pub struct BackgroundAssetUpdateData ("api.BackgroundAssetUpdateData", "BackgroundAssetUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: Option<BackgroundAssetUpdateAttributes> => "attributes", omitempty;
}
}

json_struct! {
/// BackgroundAssetUpdateAttributes contains attributes for updating a
/// background asset.
pub struct BackgroundAssetUpdateAttributes ("api.BackgroundAssetUpdateAttributes", "BackgroundAssetUpdateAttributes") {
    pub archived: Option<bool> => "archived", omitempty;
}
}

json_struct! {
/// BackgroundAssetVersionsResponse represents a list of background asset
/// versions.
pub struct BackgroundAssetVersionsResponse ("api.BackgroundAssetVersionsResponse", "BackgroundAssetVersionsResponse") {
    pub data: JsonSlice<BackgroundAssetVersion> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BackgroundAssetVersionResponse represents a single background asset
/// version.
pub struct BackgroundAssetVersionResponse ("api.BackgroundAssetVersionResponse", "BackgroundAssetVersionResponse") {
    pub data: BackgroundAssetVersion => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BackgroundAssetVersion represents one uploadable version of a
/// background asset pack.
pub struct BackgroundAssetVersion ("api.BackgroundAssetVersion", "BackgroundAssetVersion") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BackgroundAssetVersionAttributes => "attributes";
    pub relationships: Option<BackgroundAssetVersionRelationships> => "relationships", omitempty;
}
}

json_struct! {
/// BackgroundAssetVersionAttributes contains background asset version
/// attributes. State is one of AWAITING_UPLOAD, PROCESSING, FAILED, or
/// COMPLETE; StateDetails carries Apple's per-issue codes and
/// descriptions.
pub struct BackgroundAssetVersionAttributes ("api.BackgroundAssetVersionAttributes", "BackgroundAssetVersionAttributes") {
    pub created_date: Option<Timestamp> => "createdDate", omitempty;
    pub platforms: JsonSlice<String> => "platforms", omitempty;
    pub state: String => "state", omitempty;
    pub state_details: Option<BackgroundAssetVersionStateDetail> => "stateDetails", omitempty;
    pub version: String => "version", omitempty;
    pub locale: String => "locale", omitempty;
}
}

json_struct! {
/// BackgroundAssetVersionStateDetail carries the coded messages Apple
/// attaches to a version's processing state.
pub struct BackgroundAssetVersionStateDetail ("api.BackgroundAssetVersionStateDetail", "BackgroundAssetVersionStateDetail") {
    pub errors: JsonSlice<StateDetail> => "errors", omitempty;
    pub warnings: JsonSlice<StateDetail> => "warnings", omitempty;
    pub infos: JsonSlice<StateDetail> => "infos", omitempty;
}
}

json_struct! {
/// BackgroundAssetVersionRelationships exposes the resources linked to a
/// background asset version, including its release records per channel
/// and its asset/manifest upload files.
pub struct BackgroundAssetVersionRelationships ("api.BackgroundAssetVersionRelationships", "BackgroundAssetVersionRelationships") {
    pub background_asset: Option<RelationshipData> => "backgroundAsset", omitempty;
    pub internal_beta_release: Option<RelationshipData> => "internalBetaRelease", omitempty;
    pub external_beta_release: Option<RelationshipData> => "externalBetaRelease", omitempty;
    pub app_store_release: Option<RelationshipData> => "appStoreRelease", omitempty;
    pub asset_file: Option<RelationshipData> => "assetFile", omitempty;
    pub manifest_file: Option<RelationshipData> => "manifestFile", omitempty;
}
}

json_struct! {
/// BackgroundAssetVersionCreateRequest represents a request to create a
/// background asset version.
pub struct BackgroundAssetVersionCreateRequest ("api.BackgroundAssetVersionCreateRequest", "BackgroundAssetVersionCreateRequest") {
    pub data: BackgroundAssetVersionCreateData => "data";
}
}

json_struct! {
/// BackgroundAssetVersionCreateData contains the data for creating a
/// background asset version.
pub struct BackgroundAssetVersionCreateData ("api.BackgroundAssetVersionCreateData", "BackgroundAssetVersionCreateData") {
    pub type_: String => "type";
    pub relationships: BackgroundAssetVersionCreateRelationships => "relationships";
}
}

json_struct! {
/// BackgroundAssetVersionCreateRelationships contains relationships for
/// creating a background asset version.
pub struct BackgroundAssetVersionCreateRelationships ("api.BackgroundAssetVersionCreateRelationships", "BackgroundAssetVersionCreateRelationships") {
    pub background_asset: RelationshipData => "backgroundAsset";
}
}

json_struct! {
/// BackgroundAssetUploadFilesResponse represents a list of background
/// asset upload files.
pub struct BackgroundAssetUploadFilesResponse ("api.BackgroundAssetUploadFilesResponse", "BackgroundAssetUploadFilesResponse") {
    pub data: JsonSlice<BackgroundAssetUploadFile> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
}
}

json_struct! {
/// BackgroundAssetUploadFileResponse represents a single background
/// asset upload file.
pub struct BackgroundAssetUploadFileResponse ("api.BackgroundAssetUploadFileResponse", "BackgroundAssetUploadFileResponse") {
    pub data: BackgroundAssetUploadFile => "data";
}
}

json_struct! {
/// BackgroundAssetUploadFile represents one file (asset pack or
/// manifest) on a background asset version.
pub struct BackgroundAssetUploadFile ("api.BackgroundAssetUploadFile", "BackgroundAssetUploadFile") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BackgroundAssetUploadFileAttributes => "attributes";
}
}

json_struct! {
/// BackgroundAssetUploadFileAttributes contains background asset upload
/// file attributes, including the upload operations issued at
/// reservation time.
pub struct BackgroundAssetUploadFileAttributes ("api.BackgroundAssetUploadFileAttributes", "BackgroundAssetUploadFileAttributes") {
    pub asset_delivery_state: Option<AssetDeliveryState> => "assetDeliveryState", omitempty;
    pub asset_token: String => "assetToken", omitempty;
    pub asset_type: String => "assetType", omitempty;
    pub file_name: String => "fileName", omitempty;
    pub file_size: i64 => "fileSize" as "int64", omitempty;
    pub source_file_checksums: Option<Checksums> => "sourceFileChecksums", omitempty;
    pub upload_operations: JsonSlice<UploadOperation> => "uploadOperations", omitempty;
}
}

json_struct! {
/// BackgroundAssetUploadFileCreateRequest represents a request to
/// reserve a background asset upload file.
pub struct BackgroundAssetUploadFileCreateRequest ("api.BackgroundAssetUploadFileCreateRequest", "BackgroundAssetUploadFileCreateRequest") {
    pub data: BackgroundAssetUploadFileCreateData => "data";
}
}

json_struct! {
/// BackgroundAssetUploadFileCreateData contains the data for reserving a
/// background asset upload file.
pub struct BackgroundAssetUploadFileCreateData ("api.BackgroundAssetUploadFileCreateData", "BackgroundAssetUploadFileCreateData") {
    pub type_: String => "type";
    pub attributes: BackgroundAssetUploadFileCreateAttributes => "attributes";
    pub relationships: BackgroundAssetUploadFileCreateRelationships => "relationships";
}
}

json_struct! {
/// BackgroundAssetUploadFileCreateAttributes contains attributes for
/// reserving a background asset upload file. All fields are required by
/// the API; AssetType is ASSET or MANIFEST.
pub struct BackgroundAssetUploadFileCreateAttributes ("api.BackgroundAssetUploadFileCreateAttributes", "BackgroundAssetUploadFileCreateAttributes") {
    pub asset_type: String => "assetType";
    pub file_name: String => "fileName";
    pub file_size: i64 => "fileSize" as "int64";
}
}

json_struct! {
/// BackgroundAssetUploadFileCreateRelationships contains relationships
/// for reserving a background asset upload file.
pub struct BackgroundAssetUploadFileCreateRelationships ("api.BackgroundAssetUploadFileCreateRelationships", "BackgroundAssetUploadFileCreateRelationships") {
    pub background_asset_version: RelationshipData => "backgroundAssetVersion";
}
}

json_struct! {
/// BackgroundAssetUploadFileUpdateRequest represents the commit PATCH
/// for a background asset upload file.
pub struct BackgroundAssetUploadFileUpdateRequest ("api.BackgroundAssetUploadFileUpdateRequest", "BackgroundAssetUploadFileUpdateRequest") {
    pub data: BackgroundAssetUploadFileUpdateData => "data";
}
}

json_struct! {
/// BackgroundAssetUploadFileUpdateData contains the data for committing
/// a background asset upload file.
pub struct BackgroundAssetUploadFileUpdateData ("api.BackgroundAssetUploadFileUpdateData", "BackgroundAssetUploadFileUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: Option<BackgroundAssetUploadFileUpdateAttributes> => "attributes", omitempty;
}
}

json_struct! {
/// BackgroundAssetUploadFileUpdateAttributes contains attributes for
/// committing a background asset upload file.
pub struct BackgroundAssetUploadFileUpdateAttributes ("api.BackgroundAssetUploadFileUpdateAttributes", "BackgroundAssetUploadFileUpdateAttributes") {
    pub source_file_checksums: Option<Checksums> => "sourceFileChecksums", omitempty;
    pub uploaded: Option<bool> => "uploaded", omitempty;
}
}

json_struct! {
/// BackgroundAssetVersionReleaseResponse represents a single release
/// record (internal beta, external beta, or App Store) for a background
/// asset version.
pub struct BackgroundAssetVersionReleaseResponse ("api.BackgroundAssetVersionReleaseResponse", "BackgroundAssetVersionReleaseResponse") {
    pub data: BackgroundAssetVersionRelease => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BackgroundAssetVersionRelease represents the release state of a
/// background asset version in one channel. The three release resource
/// types share this shape; Type distinguishes them.
pub struct BackgroundAssetVersionRelease ("api.BackgroundAssetVersionRelease", "BackgroundAssetVersionRelease") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BackgroundAssetVersionReleaseAttributes => "attributes";
    pub relationships: Option<BackgroundAssetVersionReleaseRelationships> => "relationships", omitempty;
}
}

json_struct! {
/// BackgroundAssetVersionReleaseAttributes contains the release state.
/// Internal beta releases use READY_FOR_TESTING or SUPERSEDED; external
/// beta and App Store releases add review states such as
/// WAITING_FOR_REVIEW, IN_REVIEW, REJECTED, and READY_FOR_DISTRIBUTION.
pub struct BackgroundAssetVersionReleaseAttributes ("api.BackgroundAssetVersionReleaseAttributes", "BackgroundAssetVersionReleaseAttributes") {
    pub state: String => "state", omitempty;
}
}

json_struct! {
/// BackgroundAssetVersionReleaseRelationships exposes the version a
/// release record belongs to.
pub struct BackgroundAssetVersionReleaseRelationships ("api.BackgroundAssetVersionReleaseRelationships", "BackgroundAssetVersionReleaseRelationships") {
    pub background_asset_version: Option<RelationshipData> => "backgroundAssetVersion", omitempty;
}
}
