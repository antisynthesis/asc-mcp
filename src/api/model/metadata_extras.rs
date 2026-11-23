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
/// AccessibilityDeclarationsResponse represents a list of accessibility declarations.
pub struct AccessibilityDeclarationsResponse ("api.AccessibilityDeclarationsResponse", "AccessibilityDeclarationsResponse") {
    pub data: JsonSlice<AccessibilityDeclaration> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AccessibilityDeclarationResponse represents a single accessibility declaration.
pub struct AccessibilityDeclarationResponse ("api.AccessibilityDeclarationResponse", "AccessibilityDeclarationResponse") {
    pub data: AccessibilityDeclaration => "data";
}
}

json_struct! {
/// AccessibilityDeclaration represents a per-device-family accessibility declaration.
pub struct AccessibilityDeclaration ("api.AccessibilityDeclaration", "AccessibilityDeclaration") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AccessibilityDeclarationAttributes => "attributes";
}
}

json_struct! {
/// AccessibilityDeclarationAttributes contains accessibility declaration attributes.
pub struct AccessibilityDeclarationAttributes ("api.AccessibilityDeclarationAttributes", "AccessibilityDeclarationAttributes") {
    pub device_family: String => "deviceFamily", omitempty;
    pub state: String => "state", omitempty;
    pub supports_audio_descriptions: Option<bool> => "supportsAudioDescriptions", omitempty;
    pub supports_captions: Option<bool> => "supportsCaptions", omitempty;
    pub supports_dark_interface: Option<bool> => "supportsDarkInterface", omitempty;
    pub supports_differentiate_without_color_alone: Option<bool> => "supportsDifferentiateWithoutColorAlone", omitempty;
    pub supports_larger_text: Option<bool> => "supportsLargerText", omitempty;
    pub supports_reduced_motion: Option<bool> => "supportsReducedMotion", omitempty;
    pub supports_sufficient_contrast: Option<bool> => "supportsSufficientContrast", omitempty;
    pub supports_voice_control: Option<bool> => "supportsVoiceControl", omitempty;
    pub supports_voiceover: Option<bool> => "supportsVoiceover", omitempty;
}
}

json_struct! {
/// AccessibilityDeclarationCreateRequest represents a request to create an
/// accessibility declaration.
pub struct AccessibilityDeclarationCreateRequest ("api.AccessibilityDeclarationCreateRequest", "AccessibilityDeclarationCreateRequest") {
    pub data: AccessibilityDeclarationCreateData => "data";
}
}

json_struct! {
/// AccessibilityDeclarationCreateData contains the data for creating an
/// accessibility declaration.
pub struct AccessibilityDeclarationCreateData ("api.AccessibilityDeclarationCreateData", "AccessibilityDeclarationCreateData") {
    pub type_: String => "type";
    pub attributes: AccessibilityDeclarationCreateAttributes => "attributes";
    pub relationships: AccessibilityDeclarationCreateRelationships => "relationships";
}
}

json_struct! {
/// AccessibilityDeclarationCreateAttributes contains attributes for creating
/// an accessibility declaration. DeviceFamily is required.
pub struct AccessibilityDeclarationCreateAttributes ("api.AccessibilityDeclarationCreateAttributes", "AccessibilityDeclarationCreateAttributes") {
    pub device_family: String => "deviceFamily";
    pub supports_audio_descriptions: Option<bool> => "supportsAudioDescriptions", omitempty;
    pub supports_captions: Option<bool> => "supportsCaptions", omitempty;
    pub supports_dark_interface: Option<bool> => "supportsDarkInterface", omitempty;
    pub supports_differentiate_without_color_alone: Option<bool> => "supportsDifferentiateWithoutColorAlone", omitempty;
    pub supports_larger_text: Option<bool> => "supportsLargerText", omitempty;
    pub supports_reduced_motion: Option<bool> => "supportsReducedMotion", omitempty;
    pub supports_sufficient_contrast: Option<bool> => "supportsSufficientContrast", omitempty;
    pub supports_voice_control: Option<bool> => "supportsVoiceControl", omitempty;
    pub supports_voiceover: Option<bool> => "supportsVoiceover", omitempty;
}
}

json_struct! {
/// AccessibilityDeclarationCreateRelationships contains relationships for
/// creating an accessibility declaration.
pub struct AccessibilityDeclarationCreateRelationships ("api.AccessibilityDeclarationCreateRelationships", "AccessibilityDeclarationCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// AccessibilityDeclarationUpdateRequest represents a request to update an
/// accessibility declaration.
pub struct AccessibilityDeclarationUpdateRequest ("api.AccessibilityDeclarationUpdateRequest", "AccessibilityDeclarationUpdateRequest") {
    pub data: AccessibilityDeclarationUpdateData => "data";
}
}

json_struct! {
/// AccessibilityDeclarationUpdateData contains the data for updating an
/// accessibility declaration.
pub struct AccessibilityDeclarationUpdateData ("api.AccessibilityDeclarationUpdateData", "AccessibilityDeclarationUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: Option<AccessibilityDeclarationUpdateAttributes> => "attributes", omitempty;
}
}

json_struct! {
/// AccessibilityDeclarationUpdateAttributes contains attributes for updating
/// an accessibility declaration. Setting Publish to true publishes the draft
/// declaration to the App Store.
pub struct AccessibilityDeclarationUpdateAttributes ("api.AccessibilityDeclarationUpdateAttributes", "AccessibilityDeclarationUpdateAttributes") {
    pub publish: Option<bool> => "publish", omitempty;
    pub supports_audio_descriptions: Option<bool> => "supportsAudioDescriptions", omitempty;
    pub supports_captions: Option<bool> => "supportsCaptions", omitempty;
    pub supports_dark_interface: Option<bool> => "supportsDarkInterface", omitempty;
    pub supports_differentiate_without_color_alone: Option<bool> => "supportsDifferentiateWithoutColorAlone", omitempty;
    pub supports_larger_text: Option<bool> => "supportsLargerText", omitempty;
    pub supports_reduced_motion: Option<bool> => "supportsReducedMotion", omitempty;
    pub supports_sufficient_contrast: Option<bool> => "supportsSufficientContrast", omitempty;
    pub supports_voice_control: Option<bool> => "supportsVoiceControl", omitempty;
    pub supports_voiceover: Option<bool> => "supportsVoiceover", omitempty;
}
}

json_struct! {
/// CustomerReviewSummarizationsResponse represents a list of customer review
/// summarizations.
pub struct CustomerReviewSummarizationsResponse ("api.CustomerReviewSummarizationsResponse", "CustomerReviewSummarizationsResponse") {
    pub data: JsonSlice<CustomerReviewSummarization> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// CustomerReviewSummarization represents an Apple-generated summary of
/// customer reviews for a platform, locale, and territory.
pub struct CustomerReviewSummarization ("api.CustomerReviewSummarization", "CustomerReviewSummarization") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: CustomerReviewSummarizationAttributes => "attributes";
}
}

json_struct! {
/// CustomerReviewSummarizationAttributes contains review summarization attributes.
pub struct CustomerReviewSummarizationAttributes ("api.CustomerReviewSummarizationAttributes", "CustomerReviewSummarizationAttributes") {
    pub created_date: Option<Timestamp> => "createdDate", omitempty;
    pub locale: String => "locale", omitempty;
    pub platform: String => "platform", omitempty;
    pub text: String => "text", omitempty;
}
}

json_struct! {
/// AppTagsResponse represents a list of app tags.
pub struct AppTagsResponse ("api.AppTagsResponse", "AppTagsResponse") {
    pub data: JsonSlice<AppTag> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppTagResponse represents a single app tag.
pub struct AppTagResponse ("api.AppTagResponse", "AppTagResponse") {
    pub data: AppTag => "data";
}
}

json_struct! {
/// AppTag represents an Apple-created tag applied to an app.
pub struct AppTag ("api.AppTag", "AppTag") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppTagAttributes => "attributes";
}
}

json_struct! {
/// AppTagAttributes contains app tag attributes.
pub struct AppTagAttributes ("api.AppTagAttributes", "AppTagAttributes") {
    pub name: String => "name", omitempty;
    pub visible_in_app_store: Option<bool> => "visibleInAppStore", omitempty;
}
}

json_struct! {
/// AppTagUpdateRequest represents a request to update an app tag's visibility.
pub struct AppTagUpdateRequest ("api.AppTagUpdateRequest", "AppTagUpdateRequest") {
    pub data: AppTagUpdateData => "data";
}
}

json_struct! {
/// AppTagUpdateData contains the data for updating an app tag.
pub struct AppTagUpdateData ("api.AppTagUpdateData", "AppTagUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppTagUpdateAttributes => "attributes";
}
}

json_struct! {
/// AppTagUpdateAttributes contains attributes for updating an app tag.
/// Setting VisibleInAppStore to false opts the app out of the tag.
pub struct AppTagUpdateAttributes ("api.AppTagUpdateAttributes", "AppTagUpdateAttributes") {
    pub visible_in_app_store: Option<bool> => "visibleInAppStore", omitempty;
}
}

json_struct! {
/// TerritoryAgeRatingsResponse represents a list of territory age ratings.
pub struct TerritoryAgeRatingsResponse ("api.TerritoryAgeRatingsResponse", "TerritoryAgeRatingsResponse") {
    pub data: JsonSlice<TerritoryAgeRating> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// TerritoryAgeRating represents an app's age rating in a specific territory.
pub struct TerritoryAgeRating ("api.TerritoryAgeRating", "TerritoryAgeRating") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: TerritoryAgeRatingAttributes => "attributes";
}
}

json_struct! {
/// TerritoryAgeRatingAttributes contains territory age rating attributes.
pub struct TerritoryAgeRatingAttributes ("api.TerritoryAgeRatingAttributes", "TerritoryAgeRatingAttributes") {
    pub app_store_age_rating: String => "appStoreAgeRating", omitempty;
}
}

json_struct! {
/// AndroidToIosAppMappingDetailsResponse represents a list of Android-to-iOS
/// app mapping details.
pub struct AndroidToIosAppMappingDetailsResponse ("api.AndroidToIosAppMappingDetailsResponse", "AndroidToIosAppMappingDetailsResponse") {
    pub data: JsonSlice<AndroidToIosAppMappingDetail> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AndroidToIosAppMappingDetailResponse represents a single Android-to-iOS
/// app mapping detail.
pub struct AndroidToIosAppMappingDetailResponse ("api.AndroidToIosAppMappingDetailResponse", "AndroidToIosAppMappingDetailResponse") {
    pub data: AndroidToIosAppMappingDetail => "data";
}
}

json_struct! {
/// AndroidToIosAppMappingDetail maps an Android package to an iOS app.
pub struct AndroidToIosAppMappingDetail ("api.AndroidToIosAppMappingDetail", "AndroidToIosAppMappingDetail") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AndroidToIosAppMappingDetailAttributes => "attributes";
}
}

json_struct! {
/// AndroidToIosAppMappingDetailAttributes contains mapping detail attributes.
pub struct AndroidToIosAppMappingDetailAttributes ("api.AndroidToIosAppMappingDetailAttributes", "AndroidToIosAppMappingDetailAttributes") {
    pub package_name: String => "packageName", omitempty;
    pub app_signing_key_public_certificate_sha256_fingerprints: JsonSlice<String> => "appSigningKeyPublicCertificateSha256Fingerprints", omitempty;
}
}

json_struct! {
/// AndroidToIosAppMappingDetailCreateRequest represents a request to create an
/// Android-to-iOS app mapping.
pub struct AndroidToIosAppMappingDetailCreateRequest ("api.AndroidToIosAppMappingDetailCreateRequest", "AndroidToIosAppMappingDetailCreateRequest") {
    pub data: AndroidToIosAppMappingDetailCreateData => "data";
}
}

json_struct! {
/// AndroidToIosAppMappingDetailCreateData contains the data for creating a mapping.
pub struct AndroidToIosAppMappingDetailCreateData ("api.AndroidToIosAppMappingDetailCreateData", "AndroidToIosAppMappingDetailCreateData") {
    pub type_: String => "type";
    pub attributes: AndroidToIosAppMappingDetailCreateAttributes => "attributes";
    pub relationships: AndroidToIosAppMappingDetailCreateRelationships => "relationships";
}
}

json_struct! {
/// AndroidToIosAppMappingDetailCreateAttributes contains attributes for
/// creating a mapping. Both fields are required by the API.
pub struct AndroidToIosAppMappingDetailCreateAttributes ("api.AndroidToIosAppMappingDetailCreateAttributes", "AndroidToIosAppMappingDetailCreateAttributes") {
    pub package_name: String => "packageName";
    pub app_signing_key_public_certificate_sha256_fingerprints: JsonSlice<String> => "appSigningKeyPublicCertificateSha256Fingerprints";
}
}

json_struct! {
/// AndroidToIosAppMappingDetailCreateRelationships contains relationships for
/// creating a mapping.
pub struct AndroidToIosAppMappingDetailCreateRelationships ("api.AndroidToIosAppMappingDetailCreateRelationships", "AndroidToIosAppMappingDetailCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// AndroidToIosAppMappingDetailUpdateRequest represents a request to update an
/// Android-to-iOS app mapping.
pub struct AndroidToIosAppMappingDetailUpdateRequest ("api.AndroidToIosAppMappingDetailUpdateRequest", "AndroidToIosAppMappingDetailUpdateRequest") {
    pub data: AndroidToIosAppMappingDetailUpdateData => "data";
}
}

json_struct! {
/// AndroidToIosAppMappingDetailUpdateData contains the data for updating a mapping.
pub struct AndroidToIosAppMappingDetailUpdateData ("api.AndroidToIosAppMappingDetailUpdateData", "AndroidToIosAppMappingDetailUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: Option<AndroidToIosAppMappingDetailUpdateAttributes> => "attributes", omitempty;
}
}

json_struct! {
/// AndroidToIosAppMappingDetailUpdateAttributes contains attributes for
/// updating a mapping.
pub struct AndroidToIosAppMappingDetailUpdateAttributes ("api.AndroidToIosAppMappingDetailUpdateAttributes", "AndroidToIosAppMappingDetailUpdateAttributes") {
    pub package_name: String => "packageName", omitempty;
    pub app_signing_key_public_certificate_sha256_fingerprints: JsonSlice<String> => "appSigningKeyPublicCertificateSha256Fingerprints", omitempty;
}
}
