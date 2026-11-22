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
/// PagedDocumentLinks contains pagination links.
pub struct PagedDocumentLinks ("api.PagedDocumentLinks", "PagedDocumentLinks") {
    pub self_: String => "self", omitempty;
    pub first: String => "first", omitempty;
    pub next: String => "next", omitempty;
}
}

json_struct! {
/// PagingInformation contains pagination metadata.
pub struct PagingInformation ("api.PagingInformation", "PagingInformation") {
    pub paging: PagingInformationPaging => "paging";
}
}

json_struct! {
/// ErrorResponse represents an API error response.
pub struct ErrorResponse ("api.ErrorResponse", "ErrorResponse") {
    pub errors: JsonSlice<APIErrorDetail> => "errors";
}
}

json_struct! {
/// APIErrorDetail represents a single error entry in an API error response.
/// The wrapper *APIError exposed to callers carries the HTTP status code
/// alongside one or more of these details.
pub struct APIErrorDetail ("api.APIErrorDetail", "APIErrorDetail") {
    pub id: String => "id", omitempty;
    pub status: String => "status";
    pub code: String => "code";
    pub title: String => "title";
    pub detail: String => "detail";
}
}

json_struct! {
/// AppsResponse represents a list of apps response.
pub struct AppsResponse ("api.AppsResponse", "AppsResponse") {
    pub data: JsonSlice<App> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppResponse represents a single app response.
pub struct AppResponse ("api.AppResponse", "AppResponse") {
    pub data: App => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// App represents an App Store Connect app.
pub struct App ("api.App", "App") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppAttributes => "attributes";
}
}

json_struct! {
/// AppAttributes contains app attributes.
pub struct AppAttributes ("api.AppAttributes", "AppAttributes") {
    pub name: String => "name", omitempty;
    pub bundle_id: String => "bundleId", omitempty;
    pub sku: String => "sku", omitempty;
    pub primary_locale: String => "primaryLocale", omitempty;
    pub is_or_ever_was_made_for_kids: bool => "isOrEverWasMadeForKids", omitempty;
    pub content_rights_declaration: String => "contentRightsDeclaration", omitempty;
    pub streamlined_purchasing_enabled: bool => "streamlinedPurchasingEnabled", omitempty;
}
}

json_struct! {
/// BuildsResponse represents a list of builds response.
pub struct BuildsResponse ("api.BuildsResponse", "BuildsResponse") {
    pub data: JsonSlice<Build> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BuildResponse represents a single build response.
pub struct BuildResponse ("api.BuildResponse", "BuildResponse") {
    pub data: Build => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// Build represents an App Store Connect build.
pub struct Build ("api.Build", "Build") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BuildAttributes => "attributes";
}
}

json_struct! {
/// BuildAttributes contains build attributes.
pub struct BuildAttributes ("api.BuildAttributes", "BuildAttributes") {
    pub version: String => "version", omitempty;
    pub uploaded_date: Option<Timestamp> => "uploadedDate", omitempty;
    pub expiration_date: Option<Timestamp> => "expirationDate", omitempty;
    pub expired: bool => "expired", omitempty;
    pub min_os_version: String => "minOsVersion", omitempty;
    pub ls_minimum_system_version: String => "lsMinimumSystemVersion", omitempty;
    pub computed_min_mac_os_version: String => "computedMinMacOsVersion", omitempty;
    pub icon_asset_token: Value => "iconAssetToken", omitempty;
    pub processing_state: String => "processingState", omitempty;
    pub build_audience_type: String => "buildAudienceType", omitempty;
    pub uses_non_exempt_encryption: bool => "usesNonExemptEncryption", omitempty;
}
}

json_struct! {
/// AppStoreVersionsResponse represents a list of app store versions.
pub struct AppStoreVersionsResponse ("api.AppStoreVersionsResponse", "AppStoreVersionsResponse") {
    pub data: JsonSlice<AppStoreVersion> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppStoreVersionResponse represents a single app store version.
pub struct AppStoreVersionResponse ("api.AppStoreVersionResponse", "AppStoreVersionResponse") {
    pub data: AppStoreVersion => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppStoreVersion represents an App Store version.
pub struct AppStoreVersion ("api.AppStoreVersion", "AppStoreVersion") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppStoreVersionAttributes => "attributes";
}
}

json_struct! {
/// AppStoreVersionAttributes contains app store version attributes.
pub struct AppStoreVersionAttributes ("api.AppStoreVersionAttributes", "AppStoreVersionAttributes") {
    pub platform: String => "platform", omitempty;
    pub version_string: String => "versionString", omitempty;
    pub app_store_state: String => "appStoreState", omitempty;
    pub copyright: String => "copyright", omitempty;
    pub release_type: String => "releaseType", omitempty;
    pub earliest_release_date: Option<Timestamp> => "earliestReleaseDate", omitempty;
    pub downloadable: bool => "downloadable", omitempty;
    pub created_date: Option<Timestamp> => "createdDate", omitempty;
}
}

json_struct! {
/// BetaGroupsResponse represents a list of beta groups.
pub struct BetaGroupsResponse ("api.BetaGroupsResponse", "BetaGroupsResponse") {
    pub data: JsonSlice<BetaGroup> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaGroupResponse represents a single beta group.
pub struct BetaGroupResponse ("api.BetaGroupResponse", "BetaGroupResponse") {
    pub data: BetaGroup => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaGroup represents a TestFlight beta group.
pub struct BetaGroup ("api.BetaGroup", "BetaGroup") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BetaGroupAttributes => "attributes";
}
}

json_struct! {
/// BetaGroupAttributes contains beta group attributes.
pub struct BetaGroupAttributes ("api.BetaGroupAttributes", "BetaGroupAttributes") {
    pub name: String => "name", omitempty;
    pub created_date: Option<Timestamp> => "createdDate", omitempty;
    pub is_internal_group: bool => "isInternalGroup", omitempty;
    pub has_access_to_all_builds: bool => "hasAccessToAllBuilds", omitempty;
    pub public_link_enabled: bool => "publicLinkEnabled", omitempty;
    pub public_link_id: String => "publicLinkId", omitempty;
    pub public_link_limit_enabled: bool => "publicLinkLimitEnabled", omitempty;
    pub public_link_limit: i64 => "publicLinkLimit", omitempty;
    pub public_link: String => "publicLink", omitempty;
    pub feedback_enabled: bool => "feedbackEnabled", omitempty;
    pub ios_builds_available_for_tester_count: i64 => "iosBuildsAvailableForTesterCount", omitempty;
}
}

json_struct! {
/// BetaTestersResponse represents a list of beta testers.
pub struct BetaTestersResponse ("api.BetaTestersResponse", "BetaTestersResponse") {
    pub data: JsonSlice<BetaTester> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaTesterResponse represents a single beta tester.
pub struct BetaTesterResponse ("api.BetaTesterResponse", "BetaTesterResponse") {
    pub data: BetaTester => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaTester represents a TestFlight beta tester.
pub struct BetaTester ("api.BetaTester", "BetaTester") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BetaTesterAttributes => "attributes";
}
}

json_struct! {
/// BetaTesterAttributes contains beta tester attributes.
pub struct BetaTesterAttributes ("api.BetaTesterAttributes", "BetaTesterAttributes") {
    pub first_name: String => "firstName", omitempty;
    pub last_name: String => "lastName", omitempty;
    pub email: String => "email", omitempty;
    pub invite_type: String => "inviteType", omitempty;
    pub state: String => "state", omitempty;
}
}

json_struct! {
/// BundleIDsResponse represents a list of bundle IDs.
pub struct BundleIDsResponse ("api.BundleIDsResponse", "BundleIDsResponse") {
    pub data: JsonSlice<BundleID> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BundleIDResponse represents a single bundle ID.
pub struct BundleIDResponse ("api.BundleIDResponse", "BundleIDResponse") {
    pub data: BundleID => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BundleID represents a registered bundle identifier.
pub struct BundleID ("api.BundleID", "BundleID") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BundleIDAttributes => "attributes";
}
}

json_struct! {
/// BundleIDAttributes contains bundle ID attributes.
pub struct BundleIDAttributes ("api.BundleIDAttributes", "BundleIDAttributes") {
    pub name: String => "name", omitempty;
    pub identifier: String => "identifier", omitempty;
    pub platform: String => "platform", omitempty;
    pub seed_id: String => "seedId", omitempty;
}
}

json_struct! {
/// DevicesResponse represents a list of devices.
pub struct DevicesResponse ("api.DevicesResponse", "DevicesResponse") {
    pub data: JsonSlice<Device> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// DeviceResponse represents a single device.
pub struct DeviceResponse ("api.DeviceResponse", "DeviceResponse") {
    pub data: Device => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// Device represents a registered device.
pub struct Device ("api.Device", "Device") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: DeviceAttributes => "attributes";
}
}

json_struct! {
/// DeviceAttributes contains device attributes.
pub struct DeviceAttributes ("api.DeviceAttributes", "DeviceAttributes") {
    pub name: String => "name", omitempty;
    pub device_class: String => "deviceClass", omitempty;
    pub model: String => "model", omitempty;
    pub udid: String => "udid", omitempty;
    pub platform: String => "platform", omitempty;
    pub status: String => "status", omitempty;
    pub added_date: Option<Timestamp> => "addedDate", omitempty;
}
}

json_struct! {
/// CertificatesResponse represents a list of certificates.
pub struct CertificatesResponse ("api.CertificatesResponse", "CertificatesResponse") {
    pub data: JsonSlice<Certificate> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// CertificateResponse represents a single certificate.
pub struct CertificateResponse ("api.CertificateResponse", "CertificateResponse") {
    pub data: Certificate => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// Certificate represents a signing certificate.
pub struct Certificate ("api.Certificate", "Certificate") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: CertificateAttributes => "attributes";
}
}

json_struct! {
/// CertificateAttributes contains certificate attributes.
pub struct CertificateAttributes ("api.CertificateAttributes", "CertificateAttributes") {
    pub name: String => "name", omitempty;
    pub certificate_type: String => "certificateType", omitempty;
    pub display_name: String => "displayName", omitempty;
    pub serial_number: String => "serialNumber", omitempty;
    pub platform: String => "platform", omitempty;
    pub expiration_date: Option<Timestamp> => "expirationDate", omitempty;
    pub certificate_content: String => "certificateContent", omitempty;
}
}

json_struct! {
/// ProfilesResponse represents a list of provisioning profiles.
pub struct ProfilesResponse ("api.ProfilesResponse", "ProfilesResponse") {
    pub data: JsonSlice<Profile> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// ProfileResponse represents a single provisioning profile.
pub struct ProfileResponse ("api.ProfileResponse", "ProfileResponse") {
    pub data: Profile => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// Profile represents a provisioning profile.
pub struct Profile ("api.Profile", "Profile") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: ProfileAttributes => "attributes";
}
}

json_struct! {
/// ProfileAttributes contains provisioning profile attributes.
pub struct ProfileAttributes ("api.ProfileAttributes", "ProfileAttributes") {
    pub name: String => "name", omitempty;
    pub platform: String => "platform", omitempty;
    pub profile_type: String => "profileType", omitempty;
    pub profile_state: String => "profileState", omitempty;
    pub profile_content: String => "profileContent", omitempty;
    pub uuid: String => "uuid", omitempty;
    pub created_date: Option<Timestamp> => "createdDate", omitempty;
    pub expiration_date: Option<Timestamp> => "expirationDate", omitempty;
}
}

json_struct! {
/// BetaGroupCreateRequest represents a request to create a beta group.
pub struct BetaGroupCreateRequest ("api.BetaGroupCreateRequest", "BetaGroupCreateRequest") {
    pub data: BetaGroupCreateData => "data";
}
}

json_struct! {
/// BetaGroupCreateData contains the data for creating a beta group.
pub struct BetaGroupCreateData ("api.BetaGroupCreateData", "BetaGroupCreateData") {
    pub type_: String => "type";
    pub attributes: BetaGroupCreateAttributes => "attributes";
    pub relationships: BetaGroupCreateRelationships => "relationships";
}
}

json_struct! {
/// BetaGroupCreateAttributes contains attributes for creating a beta group.
pub struct BetaGroupCreateAttributes ("api.BetaGroupCreateAttributes", "BetaGroupCreateAttributes") {
    pub name: String => "name";
    pub is_internal_group: bool => "isInternalGroup", omitempty;
    pub has_access_to_all_builds: bool => "hasAccessToAllBuilds", omitempty;
    pub public_link_enabled: bool => "publicLinkEnabled", omitempty;
    pub public_link_limit_enabled: bool => "publicLinkLimitEnabled", omitempty;
    pub public_link_limit: i64 => "publicLinkLimit", omitempty;
    pub feedback_enabled: bool => "feedbackEnabled", omitempty;
}
}

json_struct! {
/// BetaGroupCreateRelationships contains relationships for creating a beta group.
pub struct BetaGroupCreateRelationships ("api.BetaGroupCreateRelationships", "BetaGroupCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// RelationshipData contains relationship data.
pub struct RelationshipData ("api.RelationshipData", "RelationshipData") {
    pub data: ResourceIdentifier => "data";
}
}

json_struct! {
/// ResourceIdentifier identifies a resource by type and ID.
pub struct ResourceIdentifier ("api.ResourceIdentifier", "ResourceIdentifier") {
    pub type_: String => "type";
    pub id: String => "id";
}
}

json_struct! {
/// BetaTesterCreateRequest represents a request to create/invite a beta tester.
pub struct BetaTesterCreateRequest ("api.BetaTesterCreateRequest", "BetaTesterCreateRequest") {
    pub data: BetaTesterCreateData => "data";
}
}

json_struct! {
/// BetaTesterCreateData contains the data for creating a beta tester.
pub struct BetaTesterCreateData ("api.BetaTesterCreateData", "BetaTesterCreateData") {
    pub type_: String => "type";
    pub attributes: BetaTesterCreateAttributes => "attributes";
    pub relationships: Option<BetaTesterCreateRelationships> => "relationships", omitempty;
}
}

json_struct! {
/// BetaTesterCreateAttributes contains attributes for creating a beta tester.
pub struct BetaTesterCreateAttributes ("api.BetaTesterCreateAttributes", "BetaTesterCreateAttributes") {
    pub email: String => "email";
    pub first_name: String => "firstName", omitempty;
    pub last_name: String => "lastName", omitempty;
}
}

json_struct! {
/// BetaTesterCreateRelationships contains relationships for creating a beta tester.
pub struct BetaTesterCreateRelationships ("api.BetaTesterCreateRelationships", "BetaTesterCreateRelationships") {
    pub beta_groups: Option<RelationshipDataList> => "betaGroups", omitempty;
    pub builds: Option<RelationshipDataList> => "builds", omitempty;
}
}

json_struct! {
/// RelationshipDataList contains a list of relationship data.
pub struct RelationshipDataList ("api.RelationshipDataList", "RelationshipDataList") {
    pub data: JsonSlice<ResourceIdentifier> => "data";
}
}

json_struct! {
/// DeviceCreateRequest represents a request to register a device.
pub struct DeviceCreateRequest ("api.DeviceCreateRequest", "DeviceCreateRequest") {
    pub data: DeviceCreateData => "data";
}
}

json_struct! {
/// DeviceCreateData contains the data for registering a device.
pub struct DeviceCreateData ("api.DeviceCreateData", "DeviceCreateData") {
    pub type_: String => "type";
    pub attributes: DeviceCreateAttributes => "attributes";
}
}

json_struct! {
/// DeviceCreateAttributes contains attributes for registering a device.
pub struct DeviceCreateAttributes ("api.DeviceCreateAttributes", "DeviceCreateAttributes") {
    pub name: String => "name";
    pub udid: String => "udid";
    pub platform: String => "platform";
}
}

json_struct! {
/// AppInfosResponse represents a list of app infos.
pub struct AppInfosResponse ("api.AppInfosResponse", "AppInfosResponse") {
    pub data: JsonSlice<AppInfo> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppInfoResponse represents a single app info.
pub struct AppInfoResponse ("api.AppInfoResponse", "AppInfoResponse") {
    pub data: AppInfo => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppInfo represents app information.
pub struct AppInfo ("api.AppInfo", "AppInfo") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppInfoAttributes => "attributes";
}
}

json_struct! {
/// AppInfoAttributes contains app info attributes.
pub struct AppInfoAttributes ("api.AppInfoAttributes", "AppInfoAttributes") {
    pub app_store_state: String => "appStoreState", omitempty;
    pub app_store_age_rating: String => "appStoreAgeRating", omitempty;
    pub brazil_age_rating: String => "brazilAgeRating", omitempty;
    pub kids_age_band: String => "kidsAgeBand", omitempty;
    pub brazil_age_rating_v2: String => "brazilAgeRatingV2", omitempty;
    pub state: String => "state", omitempty;
    pub primary_category: String => "primaryCategory", omitempty;
    pub secondary_category: String => "secondaryCategory", omitempty;
}
}

json_struct! {
/// AppInfoLocalizationsResponse represents a list of app info localizations.
pub struct AppInfoLocalizationsResponse ("api.AppInfoLocalizationsResponse", "AppInfoLocalizationsResponse") {
    pub data: JsonSlice<AppInfoLocalization> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppInfoLocalizationResponse represents a single app info localization.
pub struct AppInfoLocalizationResponse ("api.AppInfoLocalizationResponse", "AppInfoLocalizationResponse") {
    pub data: AppInfoLocalization => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppInfoLocalization represents localized app information.
pub struct AppInfoLocalization ("api.AppInfoLocalization", "AppInfoLocalization") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppInfoLocalizationAttributes => "attributes";
}
}

json_struct! {
/// AppInfoLocalizationAttributes contains app info localization attributes.
pub struct AppInfoLocalizationAttributes ("api.AppInfoLocalizationAttributes", "AppInfoLocalizationAttributes") {
    pub locale: String => "locale", omitempty;
    pub name: String => "name", omitempty;
    pub subtitle: String => "subtitle", omitempty;
    pub privacy_policy_url: String => "privacyPolicyUrl", omitempty;
    pub privacy_choices_url: String => "privacyChoicesUrl", omitempty;
    pub privacy_policy_text: String => "privacyPolicyText", omitempty;
}
}

json_struct! {
/// AppInfoLocalizationCreateRequest represents a request to create an app info localization.
pub struct AppInfoLocalizationCreateRequest ("api.AppInfoLocalizationCreateRequest", "AppInfoLocalizationCreateRequest") {
    pub data: AppInfoLocalizationCreateData => "data";
}
}

json_struct! {
/// AppInfoLocalizationCreateData contains the data for creating an app info localization.
pub struct AppInfoLocalizationCreateData ("api.AppInfoLocalizationCreateData", "AppInfoLocalizationCreateData") {
    pub type_: String => "type";
    pub attributes: AppInfoLocalizationCreateAttributes => "attributes";
    pub relationships: AppInfoLocalizationCreateRelationships => "relationships";
}
}

json_struct! {
/// AppInfoLocalizationCreateAttributes contains attributes for creating an app info localization.
pub struct AppInfoLocalizationCreateAttributes ("api.AppInfoLocalizationCreateAttributes", "AppInfoLocalizationCreateAttributes") {
    pub locale: String => "locale";
    pub name: String => "name";
    pub subtitle: String => "subtitle", omitempty;
    pub privacy_policy_url: String => "privacyPolicyUrl", omitempty;
    pub privacy_choices_url: String => "privacyChoicesUrl", omitempty;
    pub privacy_policy_text: String => "privacyPolicyText", omitempty;
}
}

json_struct! {
/// AppInfoLocalizationCreateRelationships contains relationships for creating an app info localization.
pub struct AppInfoLocalizationCreateRelationships ("api.AppInfoLocalizationCreateRelationships", "AppInfoLocalizationCreateRelationships") {
    pub app_info: RelationshipData => "appInfo";
}
}

json_struct! {
/// AppInfoLocalizationUpdateRequest represents a request to update an app info localization.
pub struct AppInfoLocalizationUpdateRequest ("api.AppInfoLocalizationUpdateRequest", "AppInfoLocalizationUpdateRequest") {
    pub data: AppInfoLocalizationUpdateData => "data";
}
}

json_struct! {
/// AppInfoLocalizationUpdateData contains the data for updating an app info localization.
pub struct AppInfoLocalizationUpdateData ("api.AppInfoLocalizationUpdateData", "AppInfoLocalizationUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppInfoLocalizationUpdateAttributes => "attributes";
}
}

json_struct! {
/// AppInfoLocalizationUpdateAttributes contains attributes for updating an app info localization.
pub struct AppInfoLocalizationUpdateAttributes ("api.AppInfoLocalizationUpdateAttributes", "AppInfoLocalizationUpdateAttributes") {
    pub name: String => "name", omitempty;
    pub subtitle: String => "subtitle", omitempty;
    pub privacy_policy_url: String => "privacyPolicyUrl", omitempty;
    pub privacy_choices_url: String => "privacyChoicesUrl", omitempty;
    pub privacy_policy_text: String => "privacyPolicyText", omitempty;
}
}

json_struct! {
/// AppStoreVersionLocalizationsResponse represents a list of version localizations.
pub struct AppStoreVersionLocalizationsResponse ("api.AppStoreVersionLocalizationsResponse", "AppStoreVersionLocalizationsResponse") {
    pub data: JsonSlice<AppStoreVersionLocalization> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppStoreVersionLocalizationResponse represents a single version localization.
pub struct AppStoreVersionLocalizationResponse ("api.AppStoreVersionLocalizationResponse", "AppStoreVersionLocalizationResponse") {
    pub data: AppStoreVersionLocalization => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppStoreVersionLocalization represents a localized app store version.
pub struct AppStoreVersionLocalization ("api.AppStoreVersionLocalization", "AppStoreVersionLocalization") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppStoreVersionLocalizationAttributes => "attributes";
}
}

json_struct! {
/// AppStoreVersionLocalizationAttributes contains version localization attributes.
pub struct AppStoreVersionLocalizationAttributes ("api.AppStoreVersionLocalizationAttributes", "AppStoreVersionLocalizationAttributes") {
    pub locale: String => "locale", omitempty;
    pub description: String => "description", omitempty;
    pub keywords: String => "keywords", omitempty;
    pub whats_new: String => "whatsNew", omitempty;
    pub promotional_text: String => "promotionalText", omitempty;
    pub marketing_url: String => "marketingUrl", omitempty;
    pub support_url: String => "supportUrl", omitempty;
}
}

json_struct! {
/// AppStoreVersionLocalizationCreateRequest represents a request to create a version localization.
pub struct AppStoreVersionLocalizationCreateRequest ("api.AppStoreVersionLocalizationCreateRequest", "AppStoreVersionLocalizationCreateRequest") {
    pub data: AppStoreVersionLocalizationCreateData => "data";
}
}

json_struct! {
/// AppStoreVersionLocalizationCreateData contains the data for creating a version localization.
pub struct AppStoreVersionLocalizationCreateData ("api.AppStoreVersionLocalizationCreateData", "AppStoreVersionLocalizationCreateData") {
    pub type_: String => "type";
    pub attributes: AppStoreVersionLocalizationCreateAttributes => "attributes";
    pub relationships: AppStoreVersionLocalizationCreateRelationships => "relationships";
}
}

json_struct! {
/// AppStoreVersionLocalizationCreateAttributes contains attributes for creating a version localization.
pub struct AppStoreVersionLocalizationCreateAttributes ("api.AppStoreVersionLocalizationCreateAttributes", "AppStoreVersionLocalizationCreateAttributes") {
    pub locale: String => "locale";
    pub description: String => "description", omitempty;
    pub keywords: String => "keywords", omitempty;
    pub whats_new: String => "whatsNew", omitempty;
    pub promotional_text: String => "promotionalText", omitempty;
    pub marketing_url: String => "marketingUrl", omitempty;
    pub support_url: String => "supportUrl", omitempty;
}
}

json_struct! {
/// AppStoreVersionLocalizationCreateRelationships contains relationships for creating a version localization.
pub struct AppStoreVersionLocalizationCreateRelationships ("api.AppStoreVersionLocalizationCreateRelationships", "AppStoreVersionLocalizationCreateRelationships") {
    pub app_store_version: RelationshipData => "appStoreVersion";
}
}

json_struct! {
/// AppStoreVersionLocalizationUpdateRequest represents a request to update a version localization.
pub struct AppStoreVersionLocalizationUpdateRequest ("api.AppStoreVersionLocalizationUpdateRequest", "AppStoreVersionLocalizationUpdateRequest") {
    pub data: AppStoreVersionLocalizationUpdateData => "data";
}
}

json_struct! {
/// AppStoreVersionLocalizationUpdateData contains the data for updating a version localization.
pub struct AppStoreVersionLocalizationUpdateData ("api.AppStoreVersionLocalizationUpdateData", "AppStoreVersionLocalizationUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppStoreVersionLocalizationUpdateAttributes => "attributes";
}
}

json_struct! {
/// AppStoreVersionLocalizationUpdateAttributes contains attributes for updating a version localization.
pub struct AppStoreVersionLocalizationUpdateAttributes ("api.AppStoreVersionLocalizationUpdateAttributes", "AppStoreVersionLocalizationUpdateAttributes") {
    pub description: String => "description", omitempty;
    pub keywords: String => "keywords", omitempty;
    pub whats_new: String => "whatsNew", omitempty;
    pub promotional_text: String => "promotionalText", omitempty;
    pub marketing_url: String => "marketingUrl", omitempty;
    pub support_url: String => "supportUrl", omitempty;
}
}

json_struct! {
/// CustomerReviewsResponse represents a list of customer reviews.
pub struct CustomerReviewsResponse ("api.CustomerReviewsResponse", "CustomerReviewsResponse") {
    pub data: JsonSlice<CustomerReview> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// CustomerReviewResponse represents a single customer review.
pub struct CustomerReviewResponse ("api.CustomerReviewResponse", "CustomerReviewResponse") {
    pub data: CustomerReview => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// CustomerReview represents a customer review.
pub struct CustomerReview ("api.CustomerReview", "CustomerReview") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: CustomerReviewAttributes => "attributes";
}
}

json_struct! {
/// CustomerReviewAttributes contains customer review attributes.
pub struct CustomerReviewAttributes ("api.CustomerReviewAttributes", "CustomerReviewAttributes") {
    pub rating: i64 => "rating", omitempty;
    pub title: String => "title", omitempty;
    pub body: String => "body", omitempty;
    pub reviewer_name: String => "reviewerNickname", omitempty;
    pub created_date: Option<Timestamp> => "createdDate", omitempty;
    pub territory: String => "territory", omitempty;
}
}

json_struct! {
/// CustomerReviewResponseV1 represents a response to a customer review.
pub struct CustomerReviewResponseV1 ("api.CustomerReviewResponseV1", "CustomerReviewResponseV1") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: CustomerReviewResponseV1Attributes => "attributes";
}
}

json_struct! {
/// CustomerReviewResponseV1Attributes contains review response attributes.
pub struct CustomerReviewResponseV1Attributes ("api.CustomerReviewResponseV1Attributes", "CustomerReviewResponseV1Attributes") {
    pub response_body: String => "responseBody", omitempty;
    pub last_modified: Option<Timestamp> => "lastModifiedDate", omitempty;
    pub state: String => "state", omitempty;
}
}

json_struct! {
/// CustomerReviewResponseV1Response represents a single review response.
pub struct CustomerReviewResponseV1Response ("api.CustomerReviewResponseV1Response", "CustomerReviewResponseV1Response") {
    pub data: CustomerReviewResponseV1 => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// CustomerReviewResponseCreateRequest represents a request to create a review response.
pub struct CustomerReviewResponseCreateRequest ("api.CustomerReviewResponseCreateRequest", "CustomerReviewResponseCreateRequest") {
    pub data: CustomerReviewResponseCreateData => "data";
}
}

json_struct! {
/// CustomerReviewResponseCreateData contains the data for creating a review response.
pub struct CustomerReviewResponseCreateData ("api.CustomerReviewResponseCreateData", "CustomerReviewResponseCreateData") {
    pub type_: String => "type";
    pub attributes: CustomerReviewResponseCreateAttributes => "attributes";
    pub relationships: CustomerReviewResponseCreateRelationships => "relationships";
}
}

json_struct! {
/// CustomerReviewResponseCreateAttributes contains attributes for creating a review response.
pub struct CustomerReviewResponseCreateAttributes ("api.CustomerReviewResponseCreateAttributes", "CustomerReviewResponseCreateAttributes") {
    pub response_body: String => "responseBody";
}
}

json_struct! {
/// CustomerReviewResponseCreateRelationships contains relationships for creating a review response.
pub struct CustomerReviewResponseCreateRelationships ("api.CustomerReviewResponseCreateRelationships", "CustomerReviewResponseCreateRelationships") {
    pub review: RelationshipData => "review";
}
}

json_struct! {
/// InAppPurchasesResponse represents a list of in-app purchases.
pub struct InAppPurchasesResponse ("api.InAppPurchasesResponse", "InAppPurchasesResponse") {
    pub data: JsonSlice<InAppPurchase> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// InAppPurchaseResponse represents a single in-app purchase.
pub struct InAppPurchaseResponse ("api.InAppPurchaseResponse", "InAppPurchaseResponse") {
    pub data: InAppPurchase => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// InAppPurchase represents an in-app purchase.
pub struct InAppPurchase ("api.InAppPurchase", "InAppPurchase") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: InAppPurchaseAttributes => "attributes";
}
}

json_struct! {
/// InAppPurchaseAttributes contains in-app purchase attributes.
pub struct InAppPurchaseAttributes ("api.InAppPurchaseAttributes", "InAppPurchaseAttributes") {
    pub name: String => "name", omitempty;
    pub product_id: String => "productId", omitempty;
    pub in_app_purchase_type: String => "inAppPurchaseType", omitempty;
    pub state: String => "state", omitempty;
    pub review_note: String => "reviewNote", omitempty;
    pub family_sharable: bool => "familySharable", omitempty;
    pub content_hosting: bool => "contentHosting", omitempty;
    pub available_in_all_territories: bool => "availableInAllTerritories", omitempty;
}
}

json_struct! {
/// InAppPurchaseCreateRequest represents a request to create an in-app purchase.
pub struct InAppPurchaseCreateRequest ("api.InAppPurchaseCreateRequest", "InAppPurchaseCreateRequest") {
    pub data: InAppPurchaseCreateData => "data";
}
}

json_struct! {
/// InAppPurchaseCreateData contains the data for creating an in-app purchase.
pub struct InAppPurchaseCreateData ("api.InAppPurchaseCreateData", "InAppPurchaseCreateData") {
    pub type_: String => "type";
    pub attributes: InAppPurchaseCreateAttributes => "attributes";
    pub relationships: InAppPurchaseCreateRelationships => "relationships";
}
}

json_struct! {
/// InAppPurchaseCreateAttributes contains attributes for creating an in-app purchase.
pub struct InAppPurchaseCreateAttributes ("api.InAppPurchaseCreateAttributes", "InAppPurchaseCreateAttributes") {
    pub name: String => "name";
    pub product_id: String => "productId";
    pub in_app_purchase_type: String => "inAppPurchaseType";
    pub review_note: String => "reviewNote", omitempty;
    pub family_sharable: bool => "familySharable", omitempty;
    pub available_in_all_territories: bool => "availableInAllTerritories", omitempty;
}
}

json_struct! {
/// InAppPurchaseCreateRelationships contains relationships for creating an in-app purchase.
pub struct InAppPurchaseCreateRelationships ("api.InAppPurchaseCreateRelationships", "InAppPurchaseCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// InAppPurchaseUpdateRequest represents a request to update an in-app purchase.
pub struct InAppPurchaseUpdateRequest ("api.InAppPurchaseUpdateRequest", "InAppPurchaseUpdateRequest") {
    pub data: InAppPurchaseUpdateData => "data";
}
}

json_struct! {
/// InAppPurchaseUpdateData contains the data for updating an in-app purchase.
pub struct InAppPurchaseUpdateData ("api.InAppPurchaseUpdateData", "InAppPurchaseUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: InAppPurchaseUpdateAttributes => "attributes";
}
}

json_struct! {
/// InAppPurchaseUpdateAttributes contains attributes for updating an in-app purchase.
pub struct InAppPurchaseUpdateAttributes ("api.InAppPurchaseUpdateAttributes", "InAppPurchaseUpdateAttributes") {
    pub name: String => "name", omitempty;
    pub review_note: String => "reviewNote", omitempty;
    pub family_sharable: Option<bool> => "familySharable", omitempty;
    pub available_in_all_territories: Option<bool> => "availableInAllTerritories", omitempty;
}
}

json_struct! {
/// SubscriptionsResponse represents a list of subscriptions.
pub struct SubscriptionsResponse ("api.SubscriptionsResponse", "SubscriptionsResponse") {
    pub data: JsonSlice<Subscription> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// SubscriptionResponse represents a single subscription.
pub struct SubscriptionResponse ("api.SubscriptionResponse", "SubscriptionResponse") {
    pub data: Subscription => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// Subscription represents an auto-renewable subscription.
pub struct Subscription ("api.Subscription", "Subscription") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: SubscriptionAttributes => "attributes";
}
}

json_struct! {
/// SubscriptionAttributes contains subscription attributes.
pub struct SubscriptionAttributes ("api.SubscriptionAttributes", "SubscriptionAttributes") {
    pub name: String => "name", omitempty;
    pub product_id: String => "productId", omitempty;
    pub family_sharable: bool => "familySharable", omitempty;
    pub state: String => "state", omitempty;
    pub subscription_period: String => "subscriptionPeriod", omitempty;
    pub review_note: String => "reviewNote", omitempty;
    pub group_level: i64 => "groupLevel", omitempty;
    pub available_in_all_territories: bool => "availableInAllTerritories", omitempty;
}
}

json_struct! {
/// SubscriptionGroupsResponse represents a list of subscription groups.
pub struct SubscriptionGroupsResponse ("api.SubscriptionGroupsResponse", "SubscriptionGroupsResponse") {
    pub data: JsonSlice<SubscriptionGroup> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// SubscriptionGroupResponse represents a single subscription group.
pub struct SubscriptionGroupResponse ("api.SubscriptionGroupResponse", "SubscriptionGroupResponse") {
    pub data: SubscriptionGroup => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// SubscriptionGroup represents a subscription group.
pub struct SubscriptionGroup ("api.SubscriptionGroup", "SubscriptionGroup") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: SubscriptionGroupAttributes => "attributes";
}
}

json_struct! {
/// SubscriptionGroupAttributes contains subscription group attributes.
pub struct SubscriptionGroupAttributes ("api.SubscriptionGroupAttributes", "SubscriptionGroupAttributes") {
    pub reference_name: String => "referenceName", omitempty;
}
}

json_struct! {
/// ReviewSubmissionsResponse represents a list of review submissions.
pub struct ReviewSubmissionsResponse ("api.ReviewSubmissionsResponse", "ReviewSubmissionsResponse") {
    pub data: JsonSlice<ReviewSubmission> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// ReviewSubmissionResponse represents a single review submission.
pub struct ReviewSubmissionResponse ("api.ReviewSubmissionResponse", "ReviewSubmissionResponse") {
    pub data: ReviewSubmission => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// ReviewSubmission represents a review submission.
pub struct ReviewSubmission ("api.ReviewSubmission", "ReviewSubmission") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: ReviewSubmissionAttributes => "attributes";
}
}

json_struct! {
/// ReviewSubmissionAttributes contains review submission attributes.
pub struct ReviewSubmissionAttributes ("api.ReviewSubmissionAttributes", "ReviewSubmissionAttributes") {
    pub platform: String => "platform", omitempty;
    pub submitted_date: Option<Timestamp> => "submittedDate", omitempty;
    pub state: String => "state", omitempty;
}
}

json_struct! {
/// ReviewSubmissionCreateRequest represents a request to create a review submission.
pub struct ReviewSubmissionCreateRequest ("api.ReviewSubmissionCreateRequest", "ReviewSubmissionCreateRequest") {
    pub data: ReviewSubmissionCreateData => "data";
}
}

json_struct! {
/// ReviewSubmissionCreateData contains the data for creating a review submission.
pub struct ReviewSubmissionCreateData ("api.ReviewSubmissionCreateData", "ReviewSubmissionCreateData") {
    pub type_: String => "type";
    pub attributes: Option<ReviewSubmissionCreateAttributes> => "attributes", omitempty;
    pub relationships: ReviewSubmissionCreateRelationships => "relationships";
}
}

json_struct! {
/// ReviewSubmissionCreateAttributes contains attributes for creating a review submission.
pub struct ReviewSubmissionCreateAttributes ("api.ReviewSubmissionCreateAttributes", "ReviewSubmissionCreateAttributes") {
    pub platform: String => "platform", omitempty;
}
}

json_struct! {
/// ReviewSubmissionCreateRelationships contains relationships for creating a review submission.
pub struct ReviewSubmissionCreateRelationships ("api.ReviewSubmissionCreateRelationships", "ReviewSubmissionCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// ReviewSubmissionUpdateRequest represents a request to update a review submission.
pub struct ReviewSubmissionUpdateRequest ("api.ReviewSubmissionUpdateRequest", "ReviewSubmissionUpdateRequest") {
    pub data: ReviewSubmissionUpdateData => "data";
}
}

json_struct! {
/// ReviewSubmissionUpdateData contains the data for updating a review submission.
pub struct ReviewSubmissionUpdateData ("api.ReviewSubmissionUpdateData", "ReviewSubmissionUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: ReviewSubmissionUpdateAttributes => "attributes";
}
}

json_struct! {
/// ReviewSubmissionUpdateAttributes contains attributes for updating a review submission.
pub struct ReviewSubmissionUpdateAttributes ("api.ReviewSubmissionUpdateAttributes", "ReviewSubmissionUpdateAttributes") {
    pub submitted: Option<bool> => "submitted", omitempty;
    pub canceled: Option<bool> => "canceled", omitempty;
}
}

json_struct! {
/// ReviewSubmissionItemsResponse represents a list of review submission items.
pub struct ReviewSubmissionItemsResponse ("api.ReviewSubmissionItemsResponse", "ReviewSubmissionItemsResponse") {
    pub data: JsonSlice<ReviewSubmissionItem> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// ReviewSubmissionItemResponse represents a single review submission item.
pub struct ReviewSubmissionItemResponse ("api.ReviewSubmissionItemResponse", "ReviewSubmissionItemResponse") {
    pub data: ReviewSubmissionItem => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// ReviewSubmissionItem represents an item attached to a review submission.
pub struct ReviewSubmissionItem ("api.ReviewSubmissionItem", "ReviewSubmissionItem") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: ReviewSubmissionItemAttributes => "attributes";
}
}

json_struct! {
/// ReviewSubmissionItemAttributes contains review submission item attributes.
pub struct ReviewSubmissionItemAttributes ("api.ReviewSubmissionItemAttributes", "ReviewSubmissionItemAttributes") {
    pub state: String => "state", omitempty;
}
}

json_struct! {
/// ReviewSubmissionItemCreateRequest represents a request to attach an item to a review submission.
pub struct ReviewSubmissionItemCreateRequest ("api.ReviewSubmissionItemCreateRequest", "ReviewSubmissionItemCreateRequest") {
    pub data: ReviewSubmissionItemCreateData => "data";
}
}

json_struct! {
/// ReviewSubmissionItemCreateData contains the data for creating a review submission item.
pub struct ReviewSubmissionItemCreateData ("api.ReviewSubmissionItemCreateData", "ReviewSubmissionItemCreateData") {
    pub type_: String => "type";
    pub relationships: ReviewSubmissionItemCreateRelationships => "relationships";
}
}

json_struct! {
/// ReviewSubmissionItemCreateRelationships contains relationships for creating a review submission item.
/// The gameCenter* pointers carry Game Center content to App Review
/// (App Store Connect API 4.2+): each names a content version rather than
/// the content resource itself.
pub struct ReviewSubmissionItemCreateRelationships ("api.ReviewSubmissionItemCreateRelationships", "ReviewSubmissionItemCreateRelationships") {
    pub review_submission: RelationshipData => "reviewSubmission";
    pub app_store_version: Option<RelationshipData> => "appStoreVersion", omitempty;
    pub app_custom_product_page_version: Option<RelationshipData> => "appCustomProductPageVersion", omitempty;
    pub app_store_version_experiment_v2: Option<RelationshipData> => "appStoreVersionExperimentV2", omitempty;
    pub app_event: Option<RelationshipData> => "appEvent", omitempty;
    pub game_center_achievement_version: Option<RelationshipData> => "gameCenterAchievementVersion", omitempty;
    pub game_center_leaderboard_version: Option<RelationshipData> => "gameCenterLeaderboardVersion", omitempty;
    pub game_center_leaderboard_set_version: Option<RelationshipData> => "gameCenterLeaderboardSetVersion", omitempty;
    pub game_center_activity_version: Option<RelationshipData> => "gameCenterActivityVersion", omitempty;
    pub game_center_challenge_version: Option<RelationshipData> => "gameCenterChallengeVersion", omitempty;
    pub in_app_purchase_version: Option<RelationshipData> => "inAppPurchaseVersion", omitempty;
    pub subscription_version: Option<RelationshipData> => "subscriptionVersion", omitempty;
    pub subscription_group_version: Option<RelationshipData> => "subscriptionGroupVersion", omitempty;
}
}

json_struct! {
/// AppStoreVersionCreateRequest represents a request to create a version.
pub struct AppStoreVersionCreateRequest ("api.AppStoreVersionCreateRequest", "AppStoreVersionCreateRequest") {
    pub data: AppStoreVersionCreateData => "data";
}
}

json_struct! {
/// AppStoreVersionCreateData contains the data for creating a version.
pub struct AppStoreVersionCreateData ("api.AppStoreVersionCreateData", "AppStoreVersionCreateData") {
    pub type_: String => "type";
    pub attributes: AppStoreVersionCreateAttributes => "attributes";
    pub relationships: AppStoreVersionCreateRelationships => "relationships";
}
}

json_struct! {
/// AppStoreVersionCreateAttributes contains attributes for creating a version.
pub struct AppStoreVersionCreateAttributes ("api.AppStoreVersionCreateAttributes", "AppStoreVersionCreateAttributes") {
    pub platform: String => "platform";
    pub version_string: String => "versionString";
    pub copyright: String => "copyright", omitempty;
    pub release_type: String => "releaseType", omitempty;
    pub earliest_release_date: Option<Timestamp> => "earliestReleaseDate", omitempty;
}
}

json_struct! {
/// AppStoreVersionCreateRelationships contains relationships for creating a version.
pub struct AppStoreVersionCreateRelationships ("api.AppStoreVersionCreateRelationships", "AppStoreVersionCreateRelationships") {
    pub app: RelationshipData => "app";
    pub build: Option<RelationshipData> => "build", omitempty;
}
}

json_struct! {
/// AppStoreVersionUpdateRequest represents a request to update a version.
pub struct AppStoreVersionUpdateRequest ("api.AppStoreVersionUpdateRequest", "AppStoreVersionUpdateRequest") {
    pub data: AppStoreVersionUpdateData => "data";
}
}

json_struct! {
/// AppStoreVersionUpdateData contains the data for updating a version.
pub struct AppStoreVersionUpdateData ("api.AppStoreVersionUpdateData", "AppStoreVersionUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppStoreVersionUpdateAttributes => "attributes";
}
}

json_struct! {
/// AppStoreVersionUpdateAttributes contains attributes for updating a version.
pub struct AppStoreVersionUpdateAttributes ("api.AppStoreVersionUpdateAttributes", "AppStoreVersionUpdateAttributes") {
    pub version_string: String => "versionString", omitempty;
    pub copyright: String => "copyright", omitempty;
    pub release_type: String => "releaseType", omitempty;
    pub earliest_release_date: Option<Timestamp> => "earliestReleaseDate", omitempty;
    pub downloadable: Option<bool> => "downloadable", omitempty;
}
}

json_struct! {
/// AppStoreReviewDetailResponse represents app store review detail.
pub struct AppStoreReviewDetailResponse ("api.AppStoreReviewDetailResponse", "AppStoreReviewDetailResponse") {
    pub data: AppStoreReviewDetail => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppStoreReviewDetail represents review details for submission.
pub struct AppStoreReviewDetail ("api.AppStoreReviewDetail", "AppStoreReviewDetail") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppStoreReviewDetailAttributes => "attributes";
}
}

json_struct! {
/// AppStoreReviewDetailAttributes contains review detail attributes.
pub struct AppStoreReviewDetailAttributes ("api.AppStoreReviewDetailAttributes", "AppStoreReviewDetailAttributes") {
    pub contact_first_name: String => "contactFirstName", omitempty;
    pub contact_last_name: String => "contactLastName", omitempty;
    pub contact_phone: String => "contactPhone", omitempty;
    pub contact_email: String => "contactEmail", omitempty;
    pub demo_account_name: String => "demoAccountName", omitempty;
    pub demo_account_password: String => "demoAccountPassword", omitempty;
    pub demo_account_required: bool => "demoAccountRequired", omitempty;
    pub notes: String => "notes", omitempty;
}
}

json_struct! {
/// AppStoreReviewDetailCreateRequest represents a request to create review details.
pub struct AppStoreReviewDetailCreateRequest ("api.AppStoreReviewDetailCreateRequest", "AppStoreReviewDetailCreateRequest") {
    pub data: AppStoreReviewDetailCreateData => "data";
}
}

json_struct! {
/// AppStoreReviewDetailCreateData contains the data for creating review details.
pub struct AppStoreReviewDetailCreateData ("api.AppStoreReviewDetailCreateData", "AppStoreReviewDetailCreateData") {
    pub type_: String => "type";
    pub attributes: AppStoreReviewDetailCreateAttributes => "attributes";
    pub relationships: AppStoreReviewDetailCreateRelationships => "relationships";
}
}

json_struct! {
/// AppStoreReviewDetailCreateAttributes contains attributes for creating review details.
pub struct AppStoreReviewDetailCreateAttributes ("api.AppStoreReviewDetailCreateAttributes", "AppStoreReviewDetailCreateAttributes") {
    pub contact_first_name: String => "contactFirstName", omitempty;
    pub contact_last_name: String => "contactLastName", omitempty;
    pub contact_phone: String => "contactPhone", omitempty;
    pub contact_email: String => "contactEmail", omitempty;
    pub demo_account_name: String => "demoAccountName", omitempty;
    pub demo_account_password: String => "demoAccountPassword", omitempty;
    pub demo_account_required: Option<bool> => "demoAccountRequired", omitempty;
    pub notes: String => "notes", omitempty;
}
}

json_struct! {
/// AppStoreReviewDetailCreateRelationships contains relationships for creating review details.
pub struct AppStoreReviewDetailCreateRelationships ("api.AppStoreReviewDetailCreateRelationships", "AppStoreReviewDetailCreateRelationships") {
    pub app_store_version: RelationshipData => "appStoreVersion";
}
}

json_struct! {
/// AppStoreReviewDetailUpdateRequest represents a request to update review details.
pub struct AppStoreReviewDetailUpdateRequest ("api.AppStoreReviewDetailUpdateRequest", "AppStoreReviewDetailUpdateRequest") {
    pub data: AppStoreReviewDetailUpdateData => "data";
}
}

json_struct! {
/// AppStoreReviewDetailUpdateData contains the data for updating review details.
pub struct AppStoreReviewDetailUpdateData ("api.AppStoreReviewDetailUpdateData", "AppStoreReviewDetailUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppStoreReviewDetailUpdateAttributes => "attributes";
}
}

json_struct! {
/// AppStoreReviewDetailUpdateAttributes contains attributes for updating review details.
pub struct AppStoreReviewDetailUpdateAttributes ("api.AppStoreReviewDetailUpdateAttributes", "AppStoreReviewDetailUpdateAttributes") {
    pub contact_first_name: String => "contactFirstName", omitempty;
    pub contact_last_name: String => "contactLastName", omitempty;
    pub contact_phone: String => "contactPhone", omitempty;
    pub contact_email: String => "contactEmail", omitempty;
    pub demo_account_name: String => "demoAccountName", omitempty;
    pub demo_account_password: String => "demoAccountPassword", omitempty;
    pub demo_account_required: Option<bool> => "demoAccountRequired", omitempty;
    pub notes: String => "notes", omitempty;
}
}

json_struct! {
/// AppStoreVersionPhasedReleaseResponse represents a phased release response.
pub struct AppStoreVersionPhasedReleaseResponse ("api.AppStoreVersionPhasedReleaseResponse", "AppStoreVersionPhasedReleaseResponse") {
    pub data: AppStoreVersionPhasedRelease => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppStoreVersionPhasedRelease represents a phased release.
pub struct AppStoreVersionPhasedRelease ("api.AppStoreVersionPhasedRelease", "AppStoreVersionPhasedRelease") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppStoreVersionPhasedReleaseAttributes => "attributes";
}
}

json_struct! {
/// AppStoreVersionPhasedReleaseAttributes contains phased release attributes.
pub struct AppStoreVersionPhasedReleaseAttributes ("api.AppStoreVersionPhasedReleaseAttributes", "AppStoreVersionPhasedReleaseAttributes") {
    pub phased_release_state: String => "phasedReleaseState", omitempty;
    pub start_date: Option<Timestamp> => "startDate", omitempty;
    pub total_pause_duration: i64 => "totalPauseDuration", omitempty;
    pub current_day_number: i64 => "currentDayNumber", omitempty;
}
}

json_struct! {
/// AppStoreVersionPhasedReleaseCreateRequest represents a request to create a phased release.
pub struct AppStoreVersionPhasedReleaseCreateRequest ("api.AppStoreVersionPhasedReleaseCreateRequest", "AppStoreVersionPhasedReleaseCreateRequest") {
    pub data: AppStoreVersionPhasedReleaseCreateData => "data";
}
}

json_struct! {
/// AppStoreVersionPhasedReleaseCreateData contains the data for creating a phased release.
pub struct AppStoreVersionPhasedReleaseCreateData ("api.AppStoreVersionPhasedReleaseCreateData", "AppStoreVersionPhasedReleaseCreateData") {
    pub type_: String => "type";
    pub attributes: AppStoreVersionPhasedReleaseCreateAttributes => "attributes";
    pub relationships: AppStoreVersionPhasedReleaseCreateRelationships => "relationships";
}
}

json_struct! {
/// AppStoreVersionPhasedReleaseCreateAttributes contains attributes for creating a phased release.
pub struct AppStoreVersionPhasedReleaseCreateAttributes ("api.AppStoreVersionPhasedReleaseCreateAttributes", "AppStoreVersionPhasedReleaseCreateAttributes") {
    pub phased_release_state: String => "phasedReleaseState", omitempty;
}
}

json_struct! {
/// AppStoreVersionPhasedReleaseCreateRelationships contains relationships for creating a phased release.
pub struct AppStoreVersionPhasedReleaseCreateRelationships ("api.AppStoreVersionPhasedReleaseCreateRelationships", "AppStoreVersionPhasedReleaseCreateRelationships") {
    pub app_store_version: RelationshipData => "appStoreVersion";
}
}

json_struct! {
/// AppStoreVersionPhasedReleaseUpdateRequest represents a request to update a phased release.
pub struct AppStoreVersionPhasedReleaseUpdateRequest ("api.AppStoreVersionPhasedReleaseUpdateRequest", "AppStoreVersionPhasedReleaseUpdateRequest") {
    pub data: AppStoreVersionPhasedReleaseUpdateData => "data";
}
}

json_struct! {
/// AppStoreVersionPhasedReleaseUpdateData contains the data for updating a phased release.
pub struct AppStoreVersionPhasedReleaseUpdateData ("api.AppStoreVersionPhasedReleaseUpdateData", "AppStoreVersionPhasedReleaseUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppStoreVersionPhasedReleaseUpdateAttributes => "attributes";
}
}

json_struct! {
/// AppStoreVersionPhasedReleaseUpdateAttributes contains attributes for updating a phased release.
pub struct AppStoreVersionPhasedReleaseUpdateAttributes ("api.AppStoreVersionPhasedReleaseUpdateAttributes", "AppStoreVersionPhasedReleaseUpdateAttributes") {
    pub phased_release_state: String => "phasedReleaseState", omitempty;
}
}

json_struct! {
/// AppScreenshotSetsResponse represents a list of screenshot sets.
pub struct AppScreenshotSetsResponse ("api.AppScreenshotSetsResponse", "AppScreenshotSetsResponse") {
    pub data: JsonSlice<AppScreenshotSet> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppScreenshotSetResponse represents a single screenshot set.
pub struct AppScreenshotSetResponse ("api.AppScreenshotSetResponse", "AppScreenshotSetResponse") {
    pub data: AppScreenshotSet => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppScreenshotSet represents a screenshot set.
pub struct AppScreenshotSet ("api.AppScreenshotSet", "AppScreenshotSet") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppScreenshotSetAttributes => "attributes";
}
}

json_struct! {
/// AppScreenshotSetAttributes contains screenshot set attributes.
pub struct AppScreenshotSetAttributes ("api.AppScreenshotSetAttributes", "AppScreenshotSetAttributes") {
    pub screenshot_display_type: String => "screenshotDisplayType", omitempty;
}
}

json_struct! {
/// AppScreenshotsResponse represents a list of screenshots.
pub struct AppScreenshotsResponse ("api.AppScreenshotsResponse", "AppScreenshotsResponse") {
    pub data: JsonSlice<AppScreenshot> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppScreenshotResponse represents a single screenshot.
pub struct AppScreenshotResponse ("api.AppScreenshotResponse", "AppScreenshotResponse") {
    pub data: AppScreenshot => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppScreenshot represents an app screenshot.
pub struct AppScreenshot ("api.AppScreenshot", "AppScreenshot") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppScreenshotAttributes => "attributes";
}
}

json_struct! {
/// AppScreenshotAttributes contains screenshot attributes.
pub struct AppScreenshotAttributes ("api.AppScreenshotAttributes", "AppScreenshotAttributes") {
    pub file_size: i64 => "fileSize", omitempty;
    pub file_name: String => "fileName", omitempty;
    pub source_file_checksum: String => "sourceFileChecksum", omitempty;
    pub image_asset: Option<ImageAsset> => "imageAsset", omitempty;
    pub asset_token: String => "assetToken", omitempty;
    pub asset_type: String => "assetType", omitempty;
    pub upload_operations: JsonSlice<UploadOperation> => "uploadOperations", omitempty;
    pub asset_delivery_state: Option<AssetDeliveryState> => "assetDeliveryState", omitempty;
}
}

json_struct! {
/// ImageAsset represents an image asset.
pub struct ImageAsset ("api.ImageAsset", "ImageAsset") {
    pub template_url: String => "templateUrl", omitempty;
    pub width: i64 => "width", omitempty;
    pub height: i64 => "height", omitempty;
}
}

json_struct! {
/// UploadOperation represents an upload operation.
pub struct UploadOperation ("api.UploadOperation", "UploadOperation") {
    pub method: String => "method", omitempty;
    pub url: String => "url", omitempty;
    pub length: i64 => "length", omitempty;
    pub offset: i64 => "offset", omitempty;
    pub request_headers: JsonSlice<RequestHeader> => "requestHeaders", omitempty;
}
}

json_struct! {
/// RequestHeader represents a request header.
pub struct RequestHeader ("api.RequestHeader", "RequestHeader") {
    pub name: String => "name", omitempty;
    pub value: String => "value", omitempty;
}
}

json_struct! {
/// AssetDeliveryState represents asset delivery state for an upload.
/// Errors and Warnings carry the same per-error detail shape as the
/// top-level ErrorResponse.
pub struct AssetDeliveryState ("api.AssetDeliveryState", "AssetDeliveryState") {
    pub errors: JsonSlice<APIErrorDetail> => "errors", omitempty;
    pub warnings: JsonSlice<APIErrorDetail> => "warnings", omitempty;
    pub state: String => "state", omitempty;
}
}

json_struct! {
/// AppScreenshotCreateRequest represents a request to create a screenshot.
pub struct AppScreenshotCreateRequest ("api.AppScreenshotCreateRequest", "AppScreenshotCreateRequest") {
    pub data: AppScreenshotCreateData => "data";
}
}

json_struct! {
/// AppScreenshotCreateData contains the data for creating a screenshot.
pub struct AppScreenshotCreateData ("api.AppScreenshotCreateData", "AppScreenshotCreateData") {
    pub type_: String => "type";
    pub attributes: AppScreenshotCreateAttributes => "attributes";
    pub relationships: AppScreenshotCreateRelationships => "relationships";
}
}

json_struct! {
/// AppScreenshotCreateAttributes contains attributes for creating a screenshot.
pub struct AppScreenshotCreateAttributes ("api.AppScreenshotCreateAttributes", "AppScreenshotCreateAttributes") {
    pub file_size: i64 => "fileSize";
    pub file_name: String => "fileName";
}
}

json_struct! {
/// AppScreenshotCreateRelationships contains relationships for creating a screenshot.
pub struct AppScreenshotCreateRelationships ("api.AppScreenshotCreateRelationships", "AppScreenshotCreateRelationships") {
    pub app_screenshot_set: RelationshipData => "appScreenshotSet";
}
}

json_struct! {
/// AppScreenshotUpdateRequest represents a request to update a screenshot.
pub struct AppScreenshotUpdateRequest ("api.AppScreenshotUpdateRequest", "AppScreenshotUpdateRequest") {
    pub data: AppScreenshotUpdateData => "data";
}
}

json_struct! {
/// AppScreenshotUpdateData contains the data for updating a screenshot.
pub struct AppScreenshotUpdateData ("api.AppScreenshotUpdateData", "AppScreenshotUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppScreenshotUpdateAttributes => "attributes";
}
}

json_struct! {
/// AppScreenshotUpdateAttributes contains attributes for updating a screenshot.
pub struct AppScreenshotUpdateAttributes ("api.AppScreenshotUpdateAttributes", "AppScreenshotUpdateAttributes") {
    pub source_file_checksum: String => "sourceFileChecksum", omitempty;
    pub uploaded: Option<bool> => "uploaded", omitempty;
}
}

json_struct! {
/// AppPreviewSetsResponse represents a list of preview sets.
pub struct AppPreviewSetsResponse ("api.AppPreviewSetsResponse", "AppPreviewSetsResponse") {
    pub data: JsonSlice<AppPreviewSet> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppPreviewSetResponse represents a single preview set.
pub struct AppPreviewSetResponse ("api.AppPreviewSetResponse", "AppPreviewSetResponse") {
    pub data: AppPreviewSet => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppPreviewSet represents a preview set.
pub struct AppPreviewSet ("api.AppPreviewSet", "AppPreviewSet") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppPreviewSetAttributes => "attributes";
}
}

json_struct! {
/// AppPreviewSetAttributes contains preview set attributes.
pub struct AppPreviewSetAttributes ("api.AppPreviewSetAttributes", "AppPreviewSetAttributes") {
    pub preview_type: String => "previewType", omitempty;
}
}

json_struct! {
/// AppPreviewsResponse represents a list of previews.
pub struct AppPreviewsResponse ("api.AppPreviewsResponse", "AppPreviewsResponse") {
    pub data: JsonSlice<AppPreview> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppPreviewResponse represents a single preview.
pub struct AppPreviewResponse ("api.AppPreviewResponse", "AppPreviewResponse") {
    pub data: AppPreview => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppPreview represents an app preview.
pub struct AppPreview ("api.AppPreview", "AppPreview") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppPreviewAttributes => "attributes";
}
}

json_struct! {
/// AppPreviewAttributes contains preview attributes.
pub struct AppPreviewAttributes ("api.AppPreviewAttributes", "AppPreviewAttributes") {
    pub file_size: i64 => "fileSize", omitempty;
    pub file_name: String => "fileName", omitempty;
    pub source_file_checksum: String => "sourceFileChecksum", omitempty;
    pub preview_frame_time_code: String => "previewFrameTimeCode", omitempty;
    pub mime_type: String => "mimeType", omitempty;
    pub video_url: String => "videoUrl", omitempty;
    pub preview_image: Option<ImageAsset> => "previewImage", omitempty;
    pub upload_operations: JsonSlice<UploadOperation> => "uploadOperations", omitempty;
    pub asset_delivery_state: Option<AssetDeliveryState> => "assetDeliveryState", omitempty;
}
}

json_struct! {
/// AppPreviewCreateRequest represents a request to create a preview.
pub struct AppPreviewCreateRequest ("api.AppPreviewCreateRequest", "AppPreviewCreateRequest") {
    pub data: AppPreviewCreateData => "data";
}
}

json_struct! {
/// AppPreviewCreateData contains the data for creating a preview.
pub struct AppPreviewCreateData ("api.AppPreviewCreateData", "AppPreviewCreateData") {
    pub type_: String => "type";
    pub attributes: AppPreviewCreateAttributes => "attributes";
    pub relationships: AppPreviewCreateRelationships => "relationships";
}
}

json_struct! {
/// AppPreviewCreateAttributes contains attributes for creating a preview.
pub struct AppPreviewCreateAttributes ("api.AppPreviewCreateAttributes", "AppPreviewCreateAttributes") {
    pub file_size: i64 => "fileSize";
    pub file_name: String => "fileName";
    pub preview_frame_time_code: String => "previewFrameTimeCode", omitempty;
    pub mime_type: String => "mimeType", omitempty;
}
}

json_struct! {
/// AppPreviewCreateRelationships contains relationships for creating a preview.
pub struct AppPreviewCreateRelationships ("api.AppPreviewCreateRelationships", "AppPreviewCreateRelationships") {
    pub app_preview_set: RelationshipData => "appPreviewSet";
}
}

json_struct! {
/// AppPreviewUpdateRequest represents a commit/update for a preview upload.
pub struct AppPreviewUpdateRequest ("api.AppPreviewUpdateRequest", "AppPreviewUpdateRequest") {
    pub data: AppPreviewUpdateData => "data";
}
}

json_struct! {
/// AppPreviewUpdateData carries the type, id, and commit attributes.
pub struct AppPreviewUpdateData ("api.AppPreviewUpdateData", "AppPreviewUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppPreviewUpdateAttributes => "attributes";
}
}

json_struct! {
/// AppPreviewUpdateAttributes carries the SHA-256 checksum and uploaded
/// flag that Apple uses to verify a finished upload.
pub struct AppPreviewUpdateAttributes ("api.AppPreviewUpdateAttributes", "AppPreviewUpdateAttributes") {
    pub source_file_checksum: String => "sourceFileChecksum", omitempty;
    pub uploaded: Option<bool> => "uploaded", omitempty;
    pub preview_frame_time_code: String => "previewFrameTimeCode", omitempty;
}
}

json_struct! {
/// EndAppAvailabilityPreOrderCreateRequest represents a request to end pre-order availability.
pub struct EndAppAvailabilityPreOrderCreateRequest ("api.EndAppAvailabilityPreOrderCreateRequest", "EndAppAvailabilityPreOrderCreateRequest") {
    pub data: EndAppAvailabilityPreOrderCreateData => "data";
}
}

json_struct! {
/// EndAppAvailabilityPreOrderCreateData contains the data for ending pre-order availability.
pub struct EndAppAvailabilityPreOrderCreateData ("api.EndAppAvailabilityPreOrderCreateData", "EndAppAvailabilityPreOrderCreateData") {
    pub type_: String => "type";
    pub relationships: EndAppAvailabilityPreOrderCreateRelationships => "relationships";
}
}

json_struct! {
/// EndAppAvailabilityPreOrderCreateRelationships contains relationships for ending pre-order availability.
pub struct EndAppAvailabilityPreOrderCreateRelationships ("api.EndAppAvailabilityPreOrderCreateRelationships", "EndAppAvailabilityPreOrderCreateRelationships") {
    pub territory_availabilities: RelationshipDataList => "territoryAvailabilities";
}
}

json_struct! {
/// EndAppAvailabilityPreOrderResponse represents an end pre-order response.
pub struct EndAppAvailabilityPreOrderResponse ("api.EndAppAvailabilityPreOrderResponse", "EndAppAvailabilityPreOrderResponse") {
    pub data: EndAppAvailabilityPreOrder => "data";
}
}

json_struct! {
/// EndAppAvailabilityPreOrder represents an end pre-order resource.
pub struct EndAppAvailabilityPreOrder ("api.EndAppAvailabilityPreOrder", "EndAppAvailabilityPreOrder") {
    pub type_: String => "type";
    pub id: String => "id";
}
}

json_struct! {
/// AppEventsResponse represents a list of app events.
pub struct AppEventsResponse ("api.AppEventsResponse", "AppEventsResponse") {
    pub data: JsonSlice<AppEvent> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppEventResponse represents a single app event.
pub struct AppEventResponse ("api.AppEventResponse", "AppEventResponse") {
    pub data: AppEvent => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppEvent represents an app event.
pub struct AppEvent ("api.AppEvent", "AppEvent") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppEventAttributes => "attributes";
}
}

json_struct! {
/// AppEventAttributes contains app event attributes.
pub struct AppEventAttributes ("api.AppEventAttributes", "AppEventAttributes") {
    pub reference_name: String => "referenceName", omitempty;
    pub badge: String => "badge", omitempty;
    pub event_state: String => "eventState", omitempty;
    pub deep_link: String => "deepLink", omitempty;
    pub purchase_requirement: String => "purchaseRequirement", omitempty;
    pub primary_locale: String => "primaryLocale", omitempty;
    pub priority: String => "priority", omitempty;
    pub purpose: String => "purpose", omitempty;
    pub territory_schedules: JsonSlice<TerritorySchedule> => "territorySchedules", omitempty;
    pub archived_territory_schedules: JsonSlice<TerritorySchedule> => "archivedTerritorySchedules", omitempty;
}
}

json_struct! {
/// TerritorySchedule represents a territory schedule for an event.
pub struct TerritorySchedule ("api.TerritorySchedule", "TerritorySchedule") {
    pub territories: JsonSlice<String> => "territories", omitempty;
    pub publish_start: Option<Timestamp> => "publishStart", omitempty;
    pub event_start: Option<Timestamp> => "eventStart", omitempty;
    pub event_end: Option<Timestamp> => "eventEnd", omitempty;
}
}

json_struct! {
/// AppEventCreateRequest represents a request to create an app event.
pub struct AppEventCreateRequest ("api.AppEventCreateRequest", "AppEventCreateRequest") {
    pub data: AppEventCreateData => "data";
}
}

json_struct! {
/// AppEventCreateData contains the data for creating an app event.
pub struct AppEventCreateData ("api.AppEventCreateData", "AppEventCreateData") {
    pub type_: String => "type";
    pub attributes: AppEventCreateAttributes => "attributes";
    pub relationships: AppEventCreateRelationships => "relationships";
}
}

json_struct! {
/// AppEventCreateAttributes contains attributes for creating an app event.
pub struct AppEventCreateAttributes ("api.AppEventCreateAttributes", "AppEventCreateAttributes") {
    pub reference_name: String => "referenceName";
    pub badge: String => "badge", omitempty;
    pub deep_link: String => "deepLink", omitempty;
    pub purchase_requirement: String => "purchaseRequirement", omitempty;
    pub primary_locale: String => "primaryLocale", omitempty;
    pub priority: String => "priority", omitempty;
    pub purpose: String => "purpose", omitempty;
    pub territory_schedules: JsonSlice<TerritorySchedule> => "territorySchedules", omitempty;
}
}

json_struct! {
/// AppEventCreateRelationships contains relationships for creating an app event.
pub struct AppEventCreateRelationships ("api.AppEventCreateRelationships", "AppEventCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// AppEventUpdateRequest represents a request to update an app event.
pub struct AppEventUpdateRequest ("api.AppEventUpdateRequest", "AppEventUpdateRequest") {
    pub data: AppEventUpdateData => "data";
}
}

json_struct! {
/// AppEventUpdateData contains the data for updating an app event.
pub struct AppEventUpdateData ("api.AppEventUpdateData", "AppEventUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppEventUpdateAttributes => "attributes";
}
}

json_struct! {
/// AppEventUpdateAttributes contains attributes for updating an app event.
pub struct AppEventUpdateAttributes ("api.AppEventUpdateAttributes", "AppEventUpdateAttributes") {
    pub reference_name: String => "referenceName", omitempty;
    pub badge: String => "badge", omitempty;
    pub deep_link: String => "deepLink", omitempty;
    pub purchase_requirement: String => "purchaseRequirement", omitempty;
    pub primary_locale: String => "primaryLocale", omitempty;
    pub priority: String => "priority", omitempty;
    pub purpose: String => "purpose", omitempty;
    pub territory_schedules: JsonSlice<TerritorySchedule> => "territorySchedules", omitempty;
}
}

json_struct! {
/// AnalyticsReportRequestsResponse represents a list of analytics report requests.
pub struct AnalyticsReportRequestsResponse ("api.AnalyticsReportRequestsResponse", "AnalyticsReportRequestsResponse") {
    pub data: JsonSlice<AnalyticsReportRequest> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AnalyticsReportRequestResponse represents a single analytics report request.
pub struct AnalyticsReportRequestResponse ("api.AnalyticsReportRequestResponse", "AnalyticsReportRequestResponse") {
    pub data: AnalyticsReportRequest => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AnalyticsReportRequest represents an analytics report request.
pub struct AnalyticsReportRequest ("api.AnalyticsReportRequest", "AnalyticsReportRequest") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AnalyticsReportRequestAttributes => "attributes";
}
}

json_struct! {
/// AnalyticsReportRequestAttributes contains analytics report request attributes.
pub struct AnalyticsReportRequestAttributes ("api.AnalyticsReportRequestAttributes", "AnalyticsReportRequestAttributes") {
    pub access_type: String => "accessType", omitempty;
    pub stoppable: bool => "stoppable", omitempty;
}
}

json_struct! {
/// AnalyticsReportRequestCreateRequest represents a request to create an analytics report request.
pub struct AnalyticsReportRequestCreateRequest ("api.AnalyticsReportRequestCreateRequest", "AnalyticsReportRequestCreateRequest") {
    pub data: AnalyticsReportRequestCreateData => "data";
}
}

json_struct! {
/// AnalyticsReportRequestCreateData contains the data for creating an analytics report request.
pub struct AnalyticsReportRequestCreateData ("api.AnalyticsReportRequestCreateData", "AnalyticsReportRequestCreateData") {
    pub type_: String => "type";
    pub attributes: AnalyticsReportRequestCreateAttributes => "attributes";
    pub relationships: AnalyticsReportRequestCreateRelationships => "relationships";
}
}

json_struct! {
/// AnalyticsReportRequestCreateAttributes contains attributes for creating an analytics report request.
pub struct AnalyticsReportRequestCreateAttributes ("api.AnalyticsReportRequestCreateAttributes", "AnalyticsReportRequestCreateAttributes") {
    pub access_type: String => "accessType";
}
}

json_struct! {
/// AnalyticsReportRequestCreateRelationships contains relationships for creating an analytics report request.
pub struct AnalyticsReportRequestCreateRelationships ("api.AnalyticsReportRequestCreateRelationships", "AnalyticsReportRequestCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// AnalyticsReportsResponse represents a list of analytics reports.
pub struct AnalyticsReportsResponse ("api.AnalyticsReportsResponse", "AnalyticsReportsResponse") {
    pub data: JsonSlice<AnalyticsReport> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AnalyticsReportResponse represents a single analytics report.
pub struct AnalyticsReportResponse ("api.AnalyticsReportResponse", "AnalyticsReportResponse") {
    pub data: AnalyticsReport => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AnalyticsReport represents an analytics report.
pub struct AnalyticsReport ("api.AnalyticsReport", "AnalyticsReport") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AnalyticsReportAttributes => "attributes";
}
}

json_struct! {
/// AnalyticsReportAttributes contains analytics report attributes.
pub struct AnalyticsReportAttributes ("api.AnalyticsReportAttributes", "AnalyticsReportAttributes") {
    pub category: String => "category", omitempty;
    pub name: String => "name", omitempty;
}
}

json_struct! {
/// AnalyticsReportInstancesResponse represents a list of report instances.
pub struct AnalyticsReportInstancesResponse ("api.AnalyticsReportInstancesResponse", "AnalyticsReportInstancesResponse") {
    pub data: JsonSlice<AnalyticsReportInstance> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AnalyticsReportInstance represents an analytics report instance.
pub struct AnalyticsReportInstance ("api.AnalyticsReportInstance", "AnalyticsReportInstance") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AnalyticsReportInstanceAttributes => "attributes";
}
}

json_struct! {
/// AnalyticsReportInstanceAttributes contains report instance attributes.
pub struct AnalyticsReportInstanceAttributes ("api.AnalyticsReportInstanceAttributes", "AnalyticsReportInstanceAttributes") {
    pub granularity: String => "granularity", omitempty;
    pub processing_date: String => "processingDate", omitempty;
}
}

json_struct! {
/// AnalyticsReportSegmentsResponse represents a list of report segments.
pub struct AnalyticsReportSegmentsResponse ("api.AnalyticsReportSegmentsResponse", "AnalyticsReportSegmentsResponse") {
    pub data: JsonSlice<AnalyticsReportSegment> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AnalyticsReportSegment represents an analytics report segment.
pub struct AnalyticsReportSegment ("api.AnalyticsReportSegment", "AnalyticsReportSegment") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AnalyticsReportSegmentAttributes => "attributes";
}
}

json_struct! {
/// AnalyticsReportSegmentAttributes contains report segment attributes.
pub struct AnalyticsReportSegmentAttributes ("api.AnalyticsReportSegmentAttributes", "AnalyticsReportSegmentAttributes") {
    pub checksum: String => "checksum", omitempty;
    pub size_in_bytes: i64 => "sizeInBytes", omitempty;
    pub url: String => "url", omitempty;
}
}

json_struct! {
/// AppClipsResponse represents a list of app clips.
pub struct AppClipsResponse ("api.AppClipsResponse", "AppClipsResponse") {
    pub data: JsonSlice<AppClip> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppClipResponse represents a single app clip.
pub struct AppClipResponse ("api.AppClipResponse", "AppClipResponse") {
    pub data: AppClip => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppClip represents an app clip.
pub struct AppClip ("api.AppClip", "AppClip") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppClipAttributes => "attributes";
}
}

json_struct! {
/// AppClipAttributes contains app clip attributes.
pub struct AppClipAttributes ("api.AppClipAttributes", "AppClipAttributes") {
    pub bundle_id: String => "bundleId", omitempty;
}
}

json_struct! {
/// AppClipDefaultExperiencesResponse represents a list of default experiences.
pub struct AppClipDefaultExperiencesResponse ("api.AppClipDefaultExperiencesResponse", "AppClipDefaultExperiencesResponse") {
    pub data: JsonSlice<AppClipDefaultExperience> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppClipDefaultExperienceResponse represents a single default experience.
pub struct AppClipDefaultExperienceResponse ("api.AppClipDefaultExperienceResponse", "AppClipDefaultExperienceResponse") {
    pub data: AppClipDefaultExperience => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppClipDefaultExperience represents an app clip default experience.
pub struct AppClipDefaultExperience ("api.AppClipDefaultExperience", "AppClipDefaultExperience") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppClipDefaultExperienceAttributes => "attributes";
}
}

json_struct! {
/// AppClipDefaultExperienceAttributes contains default experience attributes.
pub struct AppClipDefaultExperienceAttributes ("api.AppClipDefaultExperienceAttributes", "AppClipDefaultExperienceAttributes") {
    pub action: String => "action", omitempty;
}
}

json_struct! {
/// AppClipAdvancedExperiencesResponse represents a list of advanced experiences.
pub struct AppClipAdvancedExperiencesResponse ("api.AppClipAdvancedExperiencesResponse", "AppClipAdvancedExperiencesResponse") {
    pub data: JsonSlice<AppClipAdvancedExperience> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppClipAdvancedExperienceResponse represents a single advanced experience.
pub struct AppClipAdvancedExperienceResponse ("api.AppClipAdvancedExperienceResponse", "AppClipAdvancedExperienceResponse") {
    pub data: AppClipAdvancedExperience => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppClipAdvancedExperience represents an app clip advanced experience.
pub struct AppClipAdvancedExperience ("api.AppClipAdvancedExperience", "AppClipAdvancedExperience") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppClipAdvancedExperienceAttributes => "attributes";
}
}

json_struct! {
/// AppClipAdvancedExperienceAttributes contains advanced experience attributes.
pub struct AppClipAdvancedExperienceAttributes ("api.AppClipAdvancedExperienceAttributes", "AppClipAdvancedExperienceAttributes") {
    pub action: String => "action", omitempty;
    pub is_powered_by: bool => "isPoweredBy", omitempty;
    pub place: Option<Place> => "place", omitempty;
    pub place_status: String => "placeStatus", omitempty;
    pub business_category: String => "businessCategory", omitempty;
    pub default_language: String => "defaultLanguage", omitempty;
    pub removed: bool => "removed", omitempty;
    pub link: String => "link", omitempty;
    pub version: i64 => "version", omitempty;
    pub status: String => "status", omitempty;
}
}

json_struct! {
/// Place represents a place for an app clip experience.
pub struct Place ("api.Place", "Place") {
    pub place_id: String => "placeId", omitempty;
    pub names: JsonSlice<String> => "names", omitempty;
    pub main_address: Option<Address> => "mainAddress", omitempty;
    pub display_point: Option<Point> => "displayPoint", omitempty;
    pub map_action: String => "mapAction", omitempty;
    pub relationship: String => "relationship", omitempty;
    pub phone_number: Option<PhoneNumber> => "phoneNumber", omitempty;
    pub homepage_url: String => "homepageUrl", omitempty;
    pub categories: JsonSlice<String> => "categories", omitempty;
}
}

json_struct! {
/// Address represents an address.
pub struct Address ("api.Address", "Address") {
    pub street_address: JsonSlice<String> => "streetAddress", omitempty;
    pub floor: String => "floor", omitempty;
    pub neighborhood: String => "neighborhood", omitempty;
    pub locality: String => "locality", omitempty;
    pub state_province: String => "stateProvince", omitempty;
    pub postal_code: String => "postalCode", omitempty;
    pub country_code: String => "countryCode", omitempty;
}
}

json_struct! {
/// Point represents a geographic point.
pub struct Point ("api.Point", "Point") {
    pub coordinates: Option<Coordinates> => "coordinates", omitempty;
    pub source: String => "source", omitempty;
}
}

json_struct! {
/// Coordinates represents geographic coordinates.
pub struct Coordinates ("api.Coordinates", "Coordinates") {
    pub latitude: f64 => "latitude", omitempty;
    pub longitude: f64 => "longitude", omitempty;
}
}

json_struct! {
/// PhoneNumber represents a phone number.
pub struct PhoneNumber ("api.PhoneNumber", "PhoneNumber") {
    pub number: String => "number", omitempty;
    pub type_: String => "type", omitempty;
    pub intent: String => "intent", omitempty;
}
}

json_struct! {
/// GameCenterAchievementsResponse represents a list of achievements.
pub struct GameCenterAchievementsResponse ("api.GameCenterAchievementsResponse", "GameCenterAchievementsResponse") {
    pub data: JsonSlice<GameCenterAchievement> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterAchievementResponse represents a single achievement.
pub struct GameCenterAchievementResponse ("api.GameCenterAchievementResponse", "GameCenterAchievementResponse") {
    pub data: GameCenterAchievement => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterAchievement represents a Game Center achievement.
pub struct GameCenterAchievement ("api.GameCenterAchievement", "GameCenterAchievement") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterAchievementAttributes => "attributes";
}
}

json_struct! {
/// GameCenterAchievementAttributes contains achievement attributes.
pub struct GameCenterAchievementAttributes ("api.GameCenterAchievementAttributes", "GameCenterAchievementAttributes") {
    pub reference_name: String => "referenceName", omitempty;
    pub vendor_identifier: String => "vendorIdentifier", omitempty;
    pub points: i64 => "points", omitempty;
    pub show_before_earned: bool => "showBeforeEarned", omitempty;
    pub repeatable: bool => "repeatable", omitempty;
    pub archived: bool => "archived", omitempty;
}
}

json_struct! {
/// GameCenterAchievementCreateRequest represents a request to create an achievement
/// via the v2 Game Center API. The v2 API requires an initial version resource,
/// declared inline through the versions relationship and the included array.
pub struct GameCenterAchievementCreateRequest ("api.GameCenterAchievementCreateRequest", "GameCenterAchievementCreateRequest") {
    pub data: GameCenterAchievementCreateData => "data";
    pub included: JsonSlice<GameCenterVersionInlineCreate> => "included", omitempty;
}
}

json_struct! {
/// GameCenterVersionInlineCreate declares a version resource created inline with a
/// v2 Game Center create request. The ID is a client-chosen temporary identifier
/// referenced from the versions relationship.
pub struct GameCenterVersionInlineCreate ("api.GameCenterVersionInlineCreate", "GameCenterVersionInlineCreate") {
    pub type_: String => "type";
    pub id: String => "id";
}
}

json_struct! {
/// GameCenterAchievementCreateData contains the data for creating an achievement.
pub struct GameCenterAchievementCreateData ("api.GameCenterAchievementCreateData", "GameCenterAchievementCreateData") {
    pub type_: String => "type";
    pub attributes: GameCenterAchievementCreateAttributes => "attributes";
    pub relationships: GameCenterAchievementCreateRelationships => "relationships";
}
}

json_struct! {
/// GameCenterAchievementCreateAttributes contains attributes for creating an achievement.
/// The v2 API requires showBeforeEarned and repeatable to always be present.
pub struct GameCenterAchievementCreateAttributes ("api.GameCenterAchievementCreateAttributes", "GameCenterAchievementCreateAttributes") {
    pub reference_name: String => "referenceName";
    pub vendor_identifier: String => "vendorIdentifier";
    pub points: i64 => "points";
    pub show_before_earned: bool => "showBeforeEarned";
    pub repeatable: bool => "repeatable";
}
}

json_struct! {
/// GameCenterAchievementCreateRelationships contains relationships for creating an achievement.
pub struct GameCenterAchievementCreateRelationships ("api.GameCenterAchievementCreateRelationships", "GameCenterAchievementCreateRelationships") {
    pub game_center_detail: RelationshipData => "gameCenterDetail";
    pub versions: RelationshipDataList => "versions";
}
}

json_struct! {
/// GameCenterAchievementUpdateRequest represents a request to update an achievement.
pub struct GameCenterAchievementUpdateRequest ("api.GameCenterAchievementUpdateRequest", "GameCenterAchievementUpdateRequest") {
    pub data: GameCenterAchievementUpdateData => "data";
}
}

json_struct! {
/// GameCenterAchievementUpdateData contains the data for updating an achievement.
pub struct GameCenterAchievementUpdateData ("api.GameCenterAchievementUpdateData", "GameCenterAchievementUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterAchievementUpdateAttributes => "attributes";
}
}

json_struct! {
/// GameCenterAchievementUpdateAttributes contains attributes for updating an achievement.
pub struct GameCenterAchievementUpdateAttributes ("api.GameCenterAchievementUpdateAttributes", "GameCenterAchievementUpdateAttributes") {
    pub reference_name: String => "referenceName", omitempty;
    pub points: Option<i64> => "points", omitempty;
    pub show_before_earned: Option<bool> => "showBeforeEarned", omitempty;
    pub repeatable: Option<bool> => "repeatable", omitempty;
    pub archived: Option<bool> => "archived", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardsResponse represents a list of leaderboards.
pub struct GameCenterLeaderboardsResponse ("api.GameCenterLeaderboardsResponse", "GameCenterLeaderboardsResponse") {
    pub data: JsonSlice<GameCenterLeaderboard> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardResponse represents a single leaderboard.
pub struct GameCenterLeaderboardResponse ("api.GameCenterLeaderboardResponse", "GameCenterLeaderboardResponse") {
    pub data: GameCenterLeaderboard => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboard represents a Game Center leaderboard.
pub struct GameCenterLeaderboard ("api.GameCenterLeaderboard", "GameCenterLeaderboard") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterLeaderboardAttributes => "attributes";
}
}

json_struct! {
/// GameCenterLeaderboardAttributes contains leaderboard attributes.
pub struct GameCenterLeaderboardAttributes ("api.GameCenterLeaderboardAttributes", "GameCenterLeaderboardAttributes") {
    pub default_formatter: String => "defaultFormatter", omitempty;
    pub reference_name: String => "referenceName", omitempty;
    pub vendor_identifier: String => "vendorIdentifier", omitempty;
    pub submission_type: String => "submissionType", omitempty;
    pub score_sort_type: String => "scoreSortType", omitempty;
    pub score_range_start: String => "scoreRangeStart", omitempty;
    pub score_range_end: String => "scoreRangeEnd", omitempty;
    pub recurrence_start_date: Option<Timestamp> => "recurrenceStartDate", omitempty;
    pub recurrence_duration: String => "recurrenceDuration", omitempty;
    pub recurrence_rule: String => "recurrenceRule", omitempty;
    pub archived: bool => "archived", omitempty;
    pub visibility: String => "visibility", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardCreateRequest represents a request to create a leaderboard
/// via the v2 Game Center API. The v2 API requires an initial version resource,
/// declared inline through the versions relationship and the included array.
pub struct GameCenterLeaderboardCreateRequest ("api.GameCenterLeaderboardCreateRequest", "GameCenterLeaderboardCreateRequest") {
    pub data: GameCenterLeaderboardCreateData => "data";
    pub included: JsonSlice<GameCenterVersionInlineCreate> => "included", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardCreateData contains the data for creating a leaderboard.
pub struct GameCenterLeaderboardCreateData ("api.GameCenterLeaderboardCreateData", "GameCenterLeaderboardCreateData") {
    pub type_: String => "type";
    pub attributes: GameCenterLeaderboardCreateAttributes => "attributes";
    pub relationships: GameCenterLeaderboardCreateRelationships => "relationships";
}
}

json_struct! {
/// GameCenterLeaderboardCreateAttributes contains attributes for creating a leaderboard.
/// The v2 API requires defaultFormatter.
pub struct GameCenterLeaderboardCreateAttributes ("api.GameCenterLeaderboardCreateAttributes", "GameCenterLeaderboardCreateAttributes") {
    pub default_formatter: String => "defaultFormatter";
    pub reference_name: String => "referenceName";
    pub vendor_identifier: String => "vendorIdentifier";
    pub submission_type: String => "submissionType";
    pub score_sort_type: String => "scoreSortType";
    pub score_range_start: String => "scoreRangeStart", omitempty;
    pub score_range_end: String => "scoreRangeEnd", omitempty;
    pub recurrence_start_date: Option<Timestamp> => "recurrenceStartDate", omitempty;
    pub recurrence_duration: String => "recurrenceDuration", omitempty;
    pub recurrence_rule: String => "recurrenceRule", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardCreateRelationships contains relationships for creating a leaderboard.
pub struct GameCenterLeaderboardCreateRelationships ("api.GameCenterLeaderboardCreateRelationships", "GameCenterLeaderboardCreateRelationships") {
    pub game_center_detail: RelationshipData => "gameCenterDetail";
    pub versions: RelationshipDataList => "versions";
}
}

json_struct! {
/// GameCenterLeaderboardUpdateRequest represents a request to update a leaderboard.
pub struct GameCenterLeaderboardUpdateRequest ("api.GameCenterLeaderboardUpdateRequest", "GameCenterLeaderboardUpdateRequest") {
    pub data: GameCenterLeaderboardUpdateData => "data";
}
}

json_struct! {
/// GameCenterLeaderboardUpdateData contains the data for updating a leaderboard.
pub struct GameCenterLeaderboardUpdateData ("api.GameCenterLeaderboardUpdateData", "GameCenterLeaderboardUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterLeaderboardUpdateAttributes => "attributes";
}
}

json_struct! {
/// GameCenterLeaderboardUpdateAttributes contains attributes for updating a leaderboard.
pub struct GameCenterLeaderboardUpdateAttributes ("api.GameCenterLeaderboardUpdateAttributes", "GameCenterLeaderboardUpdateAttributes") {
    pub reference_name: String => "referenceName", omitempty;
    pub submission_type: String => "submissionType", omitempty;
    pub score_sort_type: String => "scoreSortType", omitempty;
    pub score_range_start: String => "scoreRangeStart", omitempty;
    pub score_range_end: String => "scoreRangeEnd", omitempty;
    pub recurrence_start_date: Option<Timestamp> => "recurrenceStartDate", omitempty;
    pub recurrence_duration: String => "recurrenceDuration", omitempty;
    pub recurrence_rule: String => "recurrenceRule", omitempty;
    pub archived: Option<bool> => "archived", omitempty;
}
}

json_struct! {
/// GameCenterDetailsResponse represents game center details.
pub struct GameCenterDetailsResponse ("api.GameCenterDetailsResponse", "GameCenterDetailsResponse") {
    pub data: JsonSlice<GameCenterDetail> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterDetailResponse represents a single game center detail.
pub struct GameCenterDetailResponse ("api.GameCenterDetailResponse", "GameCenterDetailResponse") {
    pub data: GameCenterDetail => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterDetail represents game center details for an app.
pub struct GameCenterDetail ("api.GameCenterDetail", "GameCenterDetail") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterDetailAttributes => "attributes";
}
}

json_struct! {
/// GameCenterDetailAttributes contains game center detail attributes.
pub struct GameCenterDetailAttributes ("api.GameCenterDetailAttributes", "GameCenterDetailAttributes") {
    pub arcade_enabled: bool => "arcadeEnabled", omitempty;
    pub challenge_enabled: bool => "challengeEnabled", omitempty;
}
}

json_struct! {
/// CiBuildRunsResponse represents a list of build runs.
pub struct CiBuildRunsResponse ("api.CiBuildRunsResponse", "CiBuildRunsResponse") {
    pub data: JsonSlice<CiBuildRun> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// CiBuildRunResponse represents a single build run.
pub struct CiBuildRunResponse ("api.CiBuildRunResponse", "CiBuildRunResponse") {
    pub data: CiBuildRun => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// CiBuildRun represents an Xcode Cloud build run.
pub struct CiBuildRun ("api.CiBuildRun", "CiBuildRun") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: CiBuildRunAttributes => "attributes";
}
}

json_struct! {
/// CiBuildRunAttributes contains build run attributes.
pub struct CiBuildRunAttributes ("api.CiBuildRunAttributes", "CiBuildRunAttributes") {
    pub number: i64 => "number", omitempty;
    pub created_date: Option<Timestamp> => "createdDate", omitempty;
    pub started_date: Option<Timestamp> => "startedDate", omitempty;
    pub finished_date: Option<Timestamp> => "finishedDate", omitempty;
    pub source_commit: Option<SourceCommit> => "sourceCommit", omitempty;
    pub destination_commit: Option<SourceCommit> => "destinationCommit", omitempty;
    pub is_pull_request_build: bool => "isPullRequestBuild", omitempty;
    pub execution_progress: String => "executionProgress", omitempty;
    pub completion_status: String => "completionStatus", omitempty;
    pub start_reason: String => "startReason", omitempty;
    pub cancel_reason: String => "cancelReason", omitempty;
}
}

json_struct! {
/// SourceCommit represents a source commit.
pub struct SourceCommit ("api.SourceCommit", "SourceCommit") {
    pub commit_sha: String => "commitSha", omitempty;
    pub author: Option<Author> => "author", omitempty;
    pub committer: Option<Author> => "committer", omitempty;
    pub message: String => "message", omitempty;
    pub web_url: String => "webUrl", omitempty;
}
}

json_struct! {
/// Author represents a commit author.
pub struct Author ("api.Author", "Author") {
    pub display_name: String => "displayName", omitempty;
}
}

json_struct! {
/// CiWorkflowsResponse represents a list of workflows.
pub struct CiWorkflowsResponse ("api.CiWorkflowsResponse", "CiWorkflowsResponse") {
    pub data: JsonSlice<CiWorkflow> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// CiWorkflowResponse represents a single workflow.
pub struct CiWorkflowResponse ("api.CiWorkflowResponse", "CiWorkflowResponse") {
    pub data: CiWorkflow => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// CiWorkflow represents an Xcode Cloud workflow.
pub struct CiWorkflow ("api.CiWorkflow", "CiWorkflow") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: CiWorkflowAttributes => "attributes";
}
}

json_struct! {
/// CiWorkflowAttributes contains workflow attributes.
pub struct CiWorkflowAttributes ("api.CiWorkflowAttributes", "CiWorkflowAttributes") {
    pub name: String => "name", omitempty;
    pub description: String => "description", omitempty;
    pub branch_start_condition: Option<BranchStartCondition> => "branchStartCondition", omitempty;
    pub tag_start_condition: Option<TagStartCondition> => "tagStartCondition", omitempty;
    pub pull_request_start_condition: Option<PullRequestStartCondition> => "pullRequestStartCondition", omitempty;
    pub scheduled_start_condition: Option<ScheduledStartCondition> => "scheduledStartCondition", omitempty;
    pub manual_branch_start_condition: Option<ManualBranchStartCondition> => "manualBranchStartCondition", omitempty;
    pub actions: JsonSlice<WorkflowAction> => "actions", omitempty;
    pub is_enabled: bool => "isEnabled", omitempty;
    pub is_locked_for_editing: bool => "isLockedForEditing", omitempty;
    pub clean: bool => "clean", omitempty;
    pub container_file_path: String => "containerFilePath", omitempty;
    pub last_modified_date: Option<Timestamp> => "lastModifiedDate", omitempty;
}
}

json_struct! {
/// BranchStartCondition represents a branch start condition.
pub struct BranchStartCondition ("api.BranchStartCondition", "BranchStartCondition") {
    pub source: Option<PatternCondition> => "source", omitempty;
    pub files_and_folders_rule: Option<FilesAndFoldersRule> => "filesAndFoldersRule", omitempty;
    pub auto_cancel: bool => "autoCancel", omitempty;
}
}

json_struct! {
/// TagStartCondition represents a tag start condition.
pub struct TagStartCondition ("api.TagStartCondition", "TagStartCondition") {
    pub source: Option<PatternCondition> => "source", omitempty;
    pub files_and_folders_rule: Option<FilesAndFoldersRule> => "filesAndFoldersRule", omitempty;
    pub auto_cancel: bool => "autoCancel", omitempty;
}
}

json_struct! {
/// PullRequestStartCondition represents a pull request start condition.
pub struct PullRequestStartCondition ("api.PullRequestStartCondition", "PullRequestStartCondition") {
    pub source: Option<PatternCondition> => "source", omitempty;
    pub destination: Option<PatternCondition> => "destination", omitempty;
    pub files_and_folders_rule: Option<FilesAndFoldersRule> => "filesAndFoldersRule", omitempty;
    pub auto_cancel: bool => "autoCancel", omitempty;
}
}

json_struct! {
/// ScheduledStartCondition represents a scheduled start condition.
pub struct ScheduledStartCondition ("api.ScheduledStartCondition", "ScheduledStartCondition") {
    pub source: Option<PatternCondition> => "source", omitempty;
    pub schedule: Option<Schedule> => "schedule", omitempty;
}
}

json_struct! {
/// ManualBranchStartCondition represents a manual branch start condition.
pub struct ManualBranchStartCondition ("api.ManualBranchStartCondition", "ManualBranchStartCondition") {
    pub source: Option<PatternCondition> => "source", omitempty;
}
}

json_struct! {
/// PatternCondition represents a pattern condition.
pub struct PatternCondition ("api.PatternCondition", "PatternCondition") {
    pub patterns: JsonSlice<Pattern> => "patterns", omitempty;
    pub is_all_match: bool => "isAllMatch", omitempty;
}
}

json_struct! {
/// Pattern represents a pattern.
pub struct Pattern ("api.Pattern", "Pattern") {
    pub pattern: String => "pattern", omitempty;
    pub is_prefix: bool => "isPrefix", omitempty;
}
}

json_struct! {
/// FilesAndFoldersRule represents a files and folders rule.
pub struct FilesAndFoldersRule ("api.FilesAndFoldersRule", "FilesAndFoldersRule") {
    pub mode: String => "mode", omitempty;
    pub paths: JsonSlice<String> => "paths", omitempty;
}
}

json_struct! {
/// Schedule represents a schedule.
pub struct Schedule ("api.Schedule", "Schedule") {
    pub frequency: String => "frequency", omitempty;
    pub days: JsonSlice<String> => "days", omitempty;
    pub hour: i64 => "hour", omitempty;
    pub minute: i64 => "minute", omitempty;
    pub timezone: String => "timezone", omitempty;
}
}

json_struct! {
/// WorkflowAction represents a workflow action.
pub struct WorkflowAction ("api.WorkflowAction", "WorkflowAction") {
    pub name: String => "name", omitempty;
    pub action_type: String => "actionType", omitempty;
    pub destination: String => "destination", omitempty;
    pub build_distribution_audience: String => "buildDistributionAudience", omitempty;
    pub test_configuration: Option<TestConfiguration> => "testConfiguration", omitempty;
    pub scheme: String => "scheme", omitempty;
    pub platform: String => "platform", omitempty;
    pub is_required_to_pass: bool => "isRequiredToPass", omitempty;
}
}

json_struct! {
/// TestConfiguration represents a test configuration.
pub struct TestConfiguration ("api.TestConfiguration", "TestConfiguration") {
    pub kind: String => "kind", omitempty;
    pub test_plan_name: String => "testPlanName", omitempty;
    pub test_destinations: JsonSlice<TestDestination> => "testDestinations", omitempty;
}
}

json_struct! {
/// TestDestination represents a test destination.
pub struct TestDestination ("api.TestDestination", "TestDestination") {
    pub device_type_name: String => "deviceTypeName", omitempty;
    pub device_type_identifier: String => "deviceTypeIdentifier", omitempty;
    pub runtime_name: String => "runtimeName", omitempty;
    pub runtime_identifier: String => "runtimeIdentifier", omitempty;
    pub kind: String => "kind", omitempty;
}
}

json_struct! {
/// CiProductsResponse represents a list of products.
pub struct CiProductsResponse ("api.CiProductsResponse", "CiProductsResponse") {
    pub data: JsonSlice<CiProduct> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// CiProductResponse represents a single product.
pub struct CiProductResponse ("api.CiProductResponse", "CiProductResponse") {
    pub data: CiProduct => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// CiProduct represents an Xcode Cloud product.
pub struct CiProduct ("api.CiProduct", "CiProduct") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: CiProductAttributes => "attributes";
}
}

json_struct! {
/// CiProductAttributes contains product attributes.
pub struct CiProductAttributes ("api.CiProductAttributes", "CiProductAttributes") {
    pub name: String => "name", omitempty;
    pub created_date: Option<Timestamp> => "createdDate", omitempty;
    pub product_type: String => "productType", omitempty;
}
}

json_struct! {
/// SalesReportsResponse represents a list of sales reports.
pub struct SalesReportsResponse ("api.SalesReportsResponse", "SalesReportsResponse") {
    pub data: Base64Bytes => "data", omitempty;
}
}

json_struct! {
/// FinanceReportsResponse represents a list of finance reports.
pub struct FinanceReportsResponse ("api.FinanceReportsResponse", "FinanceReportsResponse") {
    pub data: Base64Bytes => "data", omitempty;
}
}

json_struct! {
/// AppEncryptionDeclarationsResponse represents a list of encryption declarations.
pub struct AppEncryptionDeclarationsResponse ("api.AppEncryptionDeclarationsResponse", "AppEncryptionDeclarationsResponse") {
    pub data: JsonSlice<AppEncryptionDeclaration> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppEncryptionDeclarationResponse represents a single encryption declaration.
pub struct AppEncryptionDeclarationResponse ("api.AppEncryptionDeclarationResponse", "AppEncryptionDeclarationResponse") {
    pub data: AppEncryptionDeclaration => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppEncryptionDeclaration represents an encryption declaration.
pub struct AppEncryptionDeclaration ("api.AppEncryptionDeclaration", "AppEncryptionDeclaration") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppEncryptionDeclarationAttributes => "attributes";
}
}

json_struct! {
/// AppEncryptionDeclarationAttributes contains encryption declaration attributes.
pub struct AppEncryptionDeclarationAttributes ("api.AppEncryptionDeclarationAttributes", "AppEncryptionDeclarationAttributes") {
    pub app_description: String => "appDescription", omitempty;
    pub created_date: String => "createdDate", omitempty;
    pub uses_encryption: bool => "usesEncryption", omitempty;
    pub exempt: bool => "exempt", omitempty;
    pub contains_proprietary_cryptography: bool => "containsProprietaryCryptography", omitempty;
    pub contains_third_party_cryptography: bool => "containsThirdPartyCryptography", omitempty;
    pub available_on_french_store: bool => "availableOnFrenchStore", omitempty;
    pub platform: String => "platform", omitempty;
    pub uploaded_date: String => "uploadedDate", omitempty;
    pub document_url: String => "documentUrl", omitempty;
    pub document_name: String => "documentName", omitempty;
    pub document_type: String => "documentType", omitempty;
    pub app_encryption_declaration_state: String => "appEncryptionDeclarationState", omitempty;
    pub code_value: String => "codeValue", omitempty;
}
}

json_struct! {
/// AppEncryptionDeclarationCreateRequest represents a request to create an encryption declaration.
pub struct AppEncryptionDeclarationCreateRequest ("api.AppEncryptionDeclarationCreateRequest", "AppEncryptionDeclarationCreateRequest") {
    pub data: AppEncryptionDeclarationCreateData => "data";
}
}

json_struct! {
/// AppEncryptionDeclarationCreateData contains the data for creating an encryption declaration.
pub struct AppEncryptionDeclarationCreateData ("api.AppEncryptionDeclarationCreateData", "AppEncryptionDeclarationCreateData") {
    pub type_: String => "type";
    pub attributes: AppEncryptionDeclarationCreateAttributes => "attributes";
    pub relationships: AppEncryptionDeclarationCreateRelationships => "relationships";
}
}

json_struct! {
/// AppEncryptionDeclarationCreateAttributes contains attributes for creating an encryption declaration.
pub struct AppEncryptionDeclarationCreateAttributes ("api.AppEncryptionDeclarationCreateAttributes", "AppEncryptionDeclarationCreateAttributes") {
    pub app_description: String => "appDescription", omitempty;
    pub uses_encryption: bool => "usesEncryption";
    pub exempt: bool => "exempt", omitempty;
    pub contains_proprietary_cryptography: bool => "containsProprietaryCryptography", omitempty;
    pub contains_third_party_cryptography: bool => "containsThirdPartyCryptography", omitempty;
    pub available_on_french_store: bool => "availableOnFrenchStore", omitempty;
    pub code_value: String => "codeValue", omitempty;
}
}

json_struct! {
/// AppEncryptionDeclarationCreateRelationships contains relationships for creating an encryption declaration.
pub struct AppEncryptionDeclarationCreateRelationships ("api.AppEncryptionDeclarationCreateRelationships", "AppEncryptionDeclarationCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// UsersResponse represents a list of users.
pub struct UsersResponse ("api.UsersResponse", "UsersResponse") {
    pub data: JsonSlice<User> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// UserResponse represents a single user.
pub struct UserResponse ("api.UserResponse", "UserResponse") {
    pub data: User => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// User represents an App Store Connect user.
pub struct User ("api.User", "User") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: UserAttributes => "attributes";
}
}

json_struct! {
/// UserAttributes contains user attributes.
pub struct UserAttributes ("api.UserAttributes", "UserAttributes") {
    pub username: String => "username", omitempty;
    pub first_name: String => "firstName", omitempty;
    pub last_name: String => "lastName", omitempty;
    pub email: String => "email", omitempty;
    pub preferred_currency_territory: String => "preferredCurrencyTerritory", omitempty;
    pub agreed_to_terms: bool => "agreedToTerms", omitempty;
    pub roles: JsonSlice<String> => "roles", omitempty;
    pub all_apps_visible: bool => "allAppsVisible", omitempty;
    pub provisioning_allowed: bool => "provisioningAllowed", omitempty;
    pub expiration_date: Option<Timestamp> => "expirationDate", omitempty;
}
}

json_struct! {
/// UserUpdateRequest represents a request to update a user.
pub struct UserUpdateRequest ("api.UserUpdateRequest", "UserUpdateRequest") {
    pub data: UserUpdateData => "data";
}
}

json_struct! {
/// UserUpdateData contains the data for updating a user.
pub struct UserUpdateData ("api.UserUpdateData", "UserUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: UserUpdateAttributes => "attributes";
    pub relationships: Option<UserUpdateRelationships> => "relationships", omitempty;
}
}

json_struct! {
/// UserUpdateAttributes contains attributes for updating a user.
pub struct UserUpdateAttributes ("api.UserUpdateAttributes", "UserUpdateAttributes") {
    pub roles: JsonSlice<String> => "roles", omitempty;
    pub all_apps_visible: Option<bool> => "allAppsVisible", omitempty;
    pub provisioning_allowed: Option<bool> => "provisioningAllowed", omitempty;
}
}

json_struct! {
/// UserUpdateRelationships contains relationships for updating a user.
pub struct UserUpdateRelationships ("api.UserUpdateRelationships", "UserUpdateRelationships") {
    pub visible_apps: Option<RelationshipDataList> => "visibleApps", omitempty;
}
}

json_struct! {
/// UserInvitationsResponse represents a list of user invitations.
pub struct UserInvitationsResponse ("api.UserInvitationsResponse", "UserInvitationsResponse") {
    pub data: JsonSlice<UserInvitation> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// UserInvitationResponse represents a single user invitation.
pub struct UserInvitationResponse ("api.UserInvitationResponse", "UserInvitationResponse") {
    pub data: UserInvitation => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// UserInvitation represents a user invitation.
pub struct UserInvitation ("api.UserInvitation", "UserInvitation") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: UserInvitationAttributes => "attributes";
}
}

json_struct! {
/// UserInvitationAttributes contains user invitation attributes.
pub struct UserInvitationAttributes ("api.UserInvitationAttributes", "UserInvitationAttributes") {
    pub email: String => "email", omitempty;
    pub first_name: String => "firstName", omitempty;
    pub last_name: String => "lastName", omitempty;
    pub expiration_date: Option<Timestamp> => "expirationDate", omitempty;
    pub roles: JsonSlice<String> => "roles", omitempty;
    pub all_apps_visible: bool => "allAppsVisible", omitempty;
    pub provisioning_allowed: bool => "provisioningAllowed", omitempty;
}
}

json_struct! {
/// UserInvitationCreateRequest represents a request to create a user invitation.
pub struct UserInvitationCreateRequest ("api.UserInvitationCreateRequest", "UserInvitationCreateRequest") {
    pub data: UserInvitationCreateData => "data";
}
}

json_struct! {
/// UserInvitationCreateData contains the data for creating a user invitation.
pub struct UserInvitationCreateData ("api.UserInvitationCreateData", "UserInvitationCreateData") {
    pub type_: String => "type";
    pub attributes: UserInvitationCreateAttributes => "attributes";
    pub relationships: Option<UserInvitationCreateRelationships> => "relationships", omitempty;
}
}

json_struct! {
/// UserInvitationCreateAttributes contains attributes for creating a user invitation.
pub struct UserInvitationCreateAttributes ("api.UserInvitationCreateAttributes", "UserInvitationCreateAttributes") {
    pub email: String => "email";
    pub first_name: String => "firstName";
    pub last_name: String => "lastName";
    pub roles: JsonSlice<String> => "roles";
    pub all_apps_visible: bool => "allAppsVisible", omitempty;
    pub provisioning_allowed: bool => "provisioningAllowed", omitempty;
}
}

json_struct! {
/// UserInvitationCreateRelationships contains relationships for creating a user invitation.
pub struct UserInvitationCreateRelationships ("api.UserInvitationCreateRelationships", "UserInvitationCreateRelationships") {
    pub visible_apps: Option<RelationshipDataList> => "visibleApps", omitempty;
}
}

json_struct! {
/// AppPriceSchedulesResponse represents app price schedules.
pub struct AppPriceSchedulesResponse ("api.AppPriceSchedulesResponse", "AppPriceSchedulesResponse") {
    pub data: JsonSlice<AppPriceSchedule> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppPriceScheduleResponse represents a single app price schedule.
pub struct AppPriceScheduleResponse ("api.AppPriceScheduleResponse", "AppPriceScheduleResponse") {
    pub data: AppPriceSchedule => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppPriceSchedule represents an app price schedule.
pub struct AppPriceSchedule ("api.AppPriceSchedule", "AppPriceSchedule") {
    pub type_: String => "type";
    pub id: String => "id";
}
}

json_struct! {
/// AppPricePointsResponse represents a list of app price points.
pub struct AppPricePointsResponse ("api.AppPricePointsResponse", "AppPricePointsResponse") {
    pub data: JsonSlice<AppPricePoint> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppPricePointResponse represents a single app price point.
pub struct AppPricePointResponse ("api.AppPricePointResponse", "AppPricePointResponse") {
    pub data: AppPricePoint => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppPricePoint represents an app price point.
pub struct AppPricePoint ("api.AppPricePoint", "AppPricePoint") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppPricePointAttributes => "attributes";
}
}

json_struct! {
/// AppPricePointAttributes contains app price point attributes.
pub struct AppPricePointAttributes ("api.AppPricePointAttributes", "AppPricePointAttributes") {
    pub customer_price: String => "customerPrice", omitempty;
    pub proceeds: String => "proceeds", omitempty;
}
}

json_struct! {
/// TerritoryResponse represents a territory.
pub struct TerritoryResponse ("api.TerritoryResponse", "TerritoryResponse") {
    pub data: Territory => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// TerritoriesResponse represents a list of territories.
pub struct TerritoriesResponse ("api.TerritoriesResponse", "TerritoriesResponse") {
    pub data: JsonSlice<Territory> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// Territory represents a territory.
pub struct Territory ("api.Territory", "Territory") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: TerritoryAttributes => "attributes";
}
}

json_struct! {
/// TerritoryAttributes contains territory attributes.
pub struct TerritoryAttributes ("api.TerritoryAttributes", "TerritoryAttributes") {
    pub currency: String => "currency", omitempty;
}
}

json_struct! {
/// AppAvailabilityResponse represents app availability.
pub struct AppAvailabilityResponse ("api.AppAvailabilityResponse", "AppAvailabilityResponse") {
    pub data: AppAvailability => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppAvailability represents app availability.
pub struct AppAvailability ("api.AppAvailability", "AppAvailability") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppAvailabilityAttributes => "attributes";
}
}

json_struct! {
/// AppAvailabilityAttributes contains app availability attributes.
pub struct AppAvailabilityAttributes ("api.AppAvailabilityAttributes", "AppAvailabilityAttributes") {
    pub available_in_new_territories: bool => "availableInNewTerritories", omitempty;
}
}

json_struct! {
/// AppAvailabilityCreateRequest represents a request to set app availability.
pub struct AppAvailabilityCreateRequest ("api.AppAvailabilityCreateRequest", "AppAvailabilityCreateRequest") {
    pub data: AppAvailabilityCreateData => "data";
    pub included: JsonSlice<TerritoryAvailabilityInlineCreate> => "included", omitempty;
}
}

json_struct! {
/// AppAvailabilityCreateData contains the data for setting app availability.
pub struct AppAvailabilityCreateData ("api.AppAvailabilityCreateData", "AppAvailabilityCreateData") {
    pub type_: String => "type";
    pub attributes: AppAvailabilityCreateAttributes => "attributes";
    pub relationships: AppAvailabilityCreateRelationships => "relationships";
}
}

json_struct! {
/// AppAvailabilityCreateAttributes contains attributes for setting app availability.
pub struct AppAvailabilityCreateAttributes ("api.AppAvailabilityCreateAttributes", "AppAvailabilityCreateAttributes") {
    pub available_in_new_territories: bool => "availableInNewTerritories";
}
}

json_struct! {
/// AppAvailabilityCreateRelationships contains relationships for setting app availability.
pub struct AppAvailabilityCreateRelationships ("api.AppAvailabilityCreateRelationships", "AppAvailabilityCreateRelationships") {
    pub app: RelationshipData => "app";
    pub territory_availabilities: RelationshipDataList => "territoryAvailabilities";
}
}

json_struct! {
/// TerritoryAvailabilityInlineCreate represents an inline territory availability
/// included in an app availability create request.
pub struct TerritoryAvailabilityInlineCreate ("api.TerritoryAvailabilityInlineCreate", "TerritoryAvailabilityInlineCreate") {
    pub type_: String => "type";
    pub id: String => "id", omitempty;
    pub attributes: Option<TerritoryAvailabilityInlineAttributes> => "attributes", omitempty;
    pub relationships: Option<TerritoryAvailabilityInlineCreateRelationships> => "relationships", omitempty;
}
}

json_struct! {
/// TerritoryAvailabilityInlineAttributes contains attributes for an inline territory availability.
pub struct TerritoryAvailabilityInlineAttributes ("api.TerritoryAvailabilityInlineAttributes", "TerritoryAvailabilityInlineAttributes") {
    pub available: Option<bool> => "available", omitempty;
    pub release_date: String => "releaseDate", omitempty;
    pub pre_order_enabled: Option<bool> => "preOrderEnabled", omitempty;
}
}

json_struct! {
/// TerritoryAvailabilityInlineCreateRelationships contains relationships for an inline territory availability.
pub struct TerritoryAvailabilityInlineCreateRelationships ("api.TerritoryAvailabilityInlineCreateRelationships", "TerritoryAvailabilityInlineCreateRelationships") {
    pub territory: RelationshipData => "territory";
}
}

json_struct! {
/// TerritoryAvailabilitiesResponse represents territory availabilities.
pub struct TerritoryAvailabilitiesResponse ("api.TerritoryAvailabilitiesResponse", "TerritoryAvailabilitiesResponse") {
    pub data: JsonSlice<TerritoryAvailability> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// TerritoryAvailability represents territory availability.
pub struct TerritoryAvailability ("api.TerritoryAvailability", "TerritoryAvailability") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: TerritoryAvailabilityAttributes => "attributes";
}
}

json_struct! {
/// TerritoryAvailabilityAttributes contains territory availability attributes.
pub struct TerritoryAvailabilityAttributes ("api.TerritoryAvailabilityAttributes", "TerritoryAvailabilityAttributes") {
    pub available: bool => "available", omitempty;
    pub release_date: Option<Timestamp> => "releaseDate", omitempty;
    pub pre_order_enabled: bool => "preOrderEnabled", omitempty;
    pub pre_order_publish_date: Option<Timestamp> => "preOrderPublishDate", omitempty;
}
}

json_struct! {
/// AgeRatingDeclarationResponse represents an age rating declaration.
pub struct AgeRatingDeclarationResponse ("api.AgeRatingDeclarationResponse", "AgeRatingDeclarationResponse") {
    pub data: AgeRatingDeclaration => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AgeRatingDeclaration represents an age rating declaration.
pub struct AgeRatingDeclaration ("api.AgeRatingDeclaration", "AgeRatingDeclaration") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AgeRatingDeclarationAttributes => "attributes";
}
}

json_struct! {
/// AgeRatingDeclarationAttributes contains age rating declaration attributes.
pub struct AgeRatingDeclarationAttributes ("api.AgeRatingDeclarationAttributes", "AgeRatingDeclarationAttributes") {
    pub alcohol_tobacco_or_drug_use_or_references: String => "alcoholTobaccoOrDrugUseOrReferences", omitempty;
    pub contests: String => "contests", omitempty;
    pub gambling: bool => "gambling", omitempty;
    pub gambling_simulated: String => "gamblingSimulated", omitempty;
    pub kids_age_band: String => "kidsAgeBand", omitempty;
    pub mature_or_suggestive_themes: String => "matureOrSuggestiveThemes", omitempty;
    pub medical_or_treatment_information: String => "medicalOrTreatmentInformation", omitempty;
    pub profanity_or_crude_humor: String => "profanityOrCrudeHumor", omitempty;
    pub sexual_content_graphic_and_nudity: String => "sexualContentGraphicAndNudity", omitempty;
    pub sexual_content_or_nudity: String => "sexualContentOrNudity", omitempty;
    pub horror_or_fear_themes: String => "horrorOrFearThemes", omitempty;
    pub unrestricted_web_access: bool => "unrestrictedWebAccess", omitempty;
    pub violence_cartoon_or_fantasy: String => "violenceCartoonOrFantasy", omitempty;
    pub violence_realistic: String => "violenceRealistic", omitempty;
    pub violence_realistic_prolonged_graphic_or_sadistic: String => "violenceRealisticProlongedGraphicOrSadistic", omitempty;
    pub seventeen_plus: bool => "seventeenPlus", omitempty;
}
}

json_struct! {
/// AgeRatingDeclarationUpdateRequest represents a request to update an age rating declaration.
pub struct AgeRatingDeclarationUpdateRequest ("api.AgeRatingDeclarationUpdateRequest", "AgeRatingDeclarationUpdateRequest") {
    pub data: AgeRatingDeclarationUpdateData => "data";
}
}

json_struct! {
/// AgeRatingDeclarationUpdateData contains the data for updating an age rating declaration.
pub struct AgeRatingDeclarationUpdateData ("api.AgeRatingDeclarationUpdateData", "AgeRatingDeclarationUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AgeRatingDeclarationUpdateAttributes => "attributes";
}
}

json_struct! {
/// AgeRatingDeclarationUpdateAttributes contains attributes for updating an age rating declaration.
pub struct AgeRatingDeclarationUpdateAttributes ("api.AgeRatingDeclarationUpdateAttributes", "AgeRatingDeclarationUpdateAttributes") {
    pub alcohol_tobacco_or_drug_use_or_references: String => "alcoholTobaccoOrDrugUseOrReferences", omitempty;
    pub contests: String => "contests", omitempty;
    pub gambling: Option<bool> => "gambling", omitempty;
    pub gambling_simulated: String => "gamblingSimulated", omitempty;
    pub kids_age_band: String => "kidsAgeBand", omitempty;
    pub mature_or_suggestive_themes: String => "matureOrSuggestiveThemes", omitempty;
    pub medical_or_treatment_information: String => "medicalOrTreatmentInformation", omitempty;
    pub profanity_or_crude_humor: String => "profanityOrCrudeHumor", omitempty;
    pub sexual_content_graphic_and_nudity: String => "sexualContentGraphicAndNudity", omitempty;
    pub sexual_content_or_nudity: String => "sexualContentOrNudity", omitempty;
    pub horror_or_fear_themes: String => "horrorOrFearThemes", omitempty;
    pub unrestricted_web_access: Option<bool> => "unrestrictedWebAccess", omitempty;
    pub violence_cartoon_or_fantasy: String => "violenceCartoonOrFantasy", omitempty;
    pub violence_realistic: String => "violenceRealistic", omitempty;
    pub violence_realistic_prolonged_graphic_or_sadistic: String => "violenceRealisticProlongedGraphicOrSadistic", omitempty;
    pub seventeen_plus: Option<bool> => "seventeenPlus", omitempty;
}
}

json_struct! {
/// EndUserLicenseAgreementResponse represents an EULA.
pub struct EndUserLicenseAgreementResponse ("api.EndUserLicenseAgreementResponse", "EndUserLicenseAgreementResponse") {
    pub data: EndUserLicenseAgreement => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// EndUserLicenseAgreement represents an end user license agreement.
pub struct EndUserLicenseAgreement ("api.EndUserLicenseAgreement", "EndUserLicenseAgreement") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: EndUserLicenseAgreementAttributes => "attributes";
}
}

json_struct! {
/// EndUserLicenseAgreementAttributes contains EULA attributes.
pub struct EndUserLicenseAgreementAttributes ("api.EndUserLicenseAgreementAttributes", "EndUserLicenseAgreementAttributes") {
    pub agreement_text: String => "agreementText", omitempty;
}
}

json_struct! {
/// EndUserLicenseAgreementCreateRequest represents a request to create an EULA.
pub struct EndUserLicenseAgreementCreateRequest ("api.EndUserLicenseAgreementCreateRequest", "EndUserLicenseAgreementCreateRequest") {
    pub data: EndUserLicenseAgreementCreateData => "data";
}
}

json_struct! {
/// EndUserLicenseAgreementCreateData contains the data for creating an EULA.
pub struct EndUserLicenseAgreementCreateData ("api.EndUserLicenseAgreementCreateData", "EndUserLicenseAgreementCreateData") {
    pub type_: String => "type";
    pub attributes: EndUserLicenseAgreementCreateAttributes => "attributes";
    pub relationships: EndUserLicenseAgreementCreateRelationships => "relationships";
}
}

json_struct! {
/// EndUserLicenseAgreementCreateAttributes contains attributes for creating an EULA.
pub struct EndUserLicenseAgreementCreateAttributes ("api.EndUserLicenseAgreementCreateAttributes", "EndUserLicenseAgreementCreateAttributes") {
    pub agreement_text: String => "agreementText";
}
}

json_struct! {
/// EndUserLicenseAgreementCreateRelationships contains relationships for creating an EULA.
pub struct EndUserLicenseAgreementCreateRelationships ("api.EndUserLicenseAgreementCreateRelationships", "EndUserLicenseAgreementCreateRelationships") {
    pub app: RelationshipData => "app";
    pub territories: RelationshipDataList => "territories";
}
}

json_struct! {
/// EndUserLicenseAgreementUpdateRequest represents a request to update an EULA.
pub struct EndUserLicenseAgreementUpdateRequest ("api.EndUserLicenseAgreementUpdateRequest", "EndUserLicenseAgreementUpdateRequest") {
    pub data: EndUserLicenseAgreementUpdateData => "data";
}
}

json_struct! {
/// EndUserLicenseAgreementUpdateData contains the data for updating an EULA.
pub struct EndUserLicenseAgreementUpdateData ("api.EndUserLicenseAgreementUpdateData", "EndUserLicenseAgreementUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: EndUserLicenseAgreementUpdateAttributes => "attributes";
    pub relationships: Option<EndUserLicenseAgreementUpdateRelationships> => "relationships", omitempty;
}
}

json_struct! {
/// EndUserLicenseAgreementUpdateAttributes contains attributes for updating an EULA.
pub struct EndUserLicenseAgreementUpdateAttributes ("api.EndUserLicenseAgreementUpdateAttributes", "EndUserLicenseAgreementUpdateAttributes") {
    pub agreement_text: String => "agreementText", omitempty;
}
}

json_struct! {
/// EndUserLicenseAgreementUpdateRelationships contains relationships for updating an EULA.
pub struct EndUserLicenseAgreementUpdateRelationships ("api.EndUserLicenseAgreementUpdateRelationships", "EndUserLicenseAgreementUpdateRelationships") {
    pub territories: Option<RelationshipDataList> => "territories", omitempty;
}
}

json_struct! {
/// BetaAppReviewSubmissionsResponse represents a list of beta app review submissions.
pub struct BetaAppReviewSubmissionsResponse ("api.BetaAppReviewSubmissionsResponse", "BetaAppReviewSubmissionsResponse") {
    pub data: JsonSlice<BetaAppReviewSubmission> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaAppReviewSubmissionResponse represents a single beta app review submission.
pub struct BetaAppReviewSubmissionResponse ("api.BetaAppReviewSubmissionResponse", "BetaAppReviewSubmissionResponse") {
    pub data: BetaAppReviewSubmission => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaAppReviewSubmission represents a beta app review submission.
pub struct BetaAppReviewSubmission ("api.BetaAppReviewSubmission", "BetaAppReviewSubmission") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BetaAppReviewSubmissionAttributes => "attributes";
}
}

json_struct! {
/// BetaAppReviewSubmissionAttributes contains beta app review submission attributes.
pub struct BetaAppReviewSubmissionAttributes ("api.BetaAppReviewSubmissionAttributes", "BetaAppReviewSubmissionAttributes") {
    pub beta_review_state: String => "betaReviewState", omitempty;
    pub submitted_date: Option<Timestamp> => "submittedDate", omitempty;
}
}

json_struct! {
/// BetaAppReviewSubmissionCreateRequest represents a request to create a beta app review submission.
pub struct BetaAppReviewSubmissionCreateRequest ("api.BetaAppReviewSubmissionCreateRequest", "BetaAppReviewSubmissionCreateRequest") {
    pub data: BetaAppReviewSubmissionCreateData => "data";
}
}

json_struct! {
/// BetaAppReviewSubmissionCreateData contains the data for creating a beta app review submission.
pub struct BetaAppReviewSubmissionCreateData ("api.BetaAppReviewSubmissionCreateData", "BetaAppReviewSubmissionCreateData") {
    pub type_: String => "type";
    pub relationships: BetaAppReviewSubmissionCreateRelationships => "relationships";
}
}

json_struct! {
/// BetaAppReviewSubmissionCreateRelationships contains relationships for creating a beta app review submission.
pub struct BetaAppReviewSubmissionCreateRelationships ("api.BetaAppReviewSubmissionCreateRelationships", "BetaAppReviewSubmissionCreateRelationships") {
    pub build: RelationshipData => "build";
}
}

json_struct! {
/// BetaLicenseAgreementResponse represents a beta license agreement.
pub struct BetaLicenseAgreementResponse ("api.BetaLicenseAgreementResponse", "BetaLicenseAgreementResponse") {
    pub data: BetaLicenseAgreement => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaLicenseAgreementsResponse represents a list of beta license agreements.
pub struct BetaLicenseAgreementsResponse ("api.BetaLicenseAgreementsResponse", "BetaLicenseAgreementsResponse") {
    pub data: JsonSlice<BetaLicenseAgreement> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaLicenseAgreement represents a beta license agreement.
pub struct BetaLicenseAgreement ("api.BetaLicenseAgreement", "BetaLicenseAgreement") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BetaLicenseAgreementAttributes => "attributes";
}
}

json_struct! {
/// BetaLicenseAgreementAttributes contains beta license agreement attributes.
pub struct BetaLicenseAgreementAttributes ("api.BetaLicenseAgreementAttributes", "BetaLicenseAgreementAttributes") {
    pub agreement_text: String => "agreementText", omitempty;
}
}

json_struct! {
/// BetaLicenseAgreementUpdateRequest represents a request to update a beta license agreement.
pub struct BetaLicenseAgreementUpdateRequest ("api.BetaLicenseAgreementUpdateRequest", "BetaLicenseAgreementUpdateRequest") {
    pub data: BetaLicenseAgreementUpdateData => "data";
}
}

json_struct! {
/// BetaLicenseAgreementUpdateData contains the data for updating a beta license agreement.
pub struct BetaLicenseAgreementUpdateData ("api.BetaLicenseAgreementUpdateData", "BetaLicenseAgreementUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BetaLicenseAgreementUpdateAttributes => "attributes";
}
}

json_struct! {
/// BetaLicenseAgreementUpdateAttributes contains attributes for updating a beta license agreement.
pub struct BetaLicenseAgreementUpdateAttributes ("api.BetaLicenseAgreementUpdateAttributes", "BetaLicenseAgreementUpdateAttributes") {
    pub agreement_text: String => "agreementText", omitempty;
}
}

json_struct! {
/// SandboxTestersResponse represents a list of sandbox testers.
pub struct SandboxTestersResponse ("api.SandboxTestersResponse", "SandboxTestersResponse") {
    pub data: JsonSlice<SandboxTester> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// SandboxTesterResponse represents a single sandbox tester.
pub struct SandboxTesterResponse ("api.SandboxTesterResponse", "SandboxTesterResponse") {
    pub data: SandboxTester => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// SandboxTester represents a sandbox tester.
pub struct SandboxTester ("api.SandboxTester", "SandboxTester") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: SandboxTesterAttributes => "attributes";
}
}

json_struct! {
/// SandboxTesterAttributes contains sandbox tester attributes.
pub struct SandboxTesterAttributes ("api.SandboxTesterAttributes", "SandboxTesterAttributes") {
    pub first_name: String => "firstName", omitempty;
    pub last_name: String => "lastName", omitempty;
    pub email: String => "email", omitempty;
    pub password: String => "password", omitempty;
    pub confirm_password: String => "confirmPassword", omitempty;
    pub secret_question: String => "secretQuestion", omitempty;
    pub secret_answer: String => "secretAnswer", omitempty;
    pub birth_date: String => "birthDate", omitempty;
    pub app_store_territory: String => "appStoreTerritory", omitempty;
    pub interruptable: bool => "interruptable", omitempty;
    pub subscription_renewal_rate: String => "subscriptionRenewalRate", omitempty;
}
}

json_struct! {
/// SandboxTesterClearPurchaseHistoryRequest represents a request to clear
/// purchase history for sandbox testers.
pub struct SandboxTesterClearPurchaseHistoryRequest ("api.SandboxTesterClearPurchaseHistoryRequest", "SandboxTesterClearPurchaseHistoryRequest") {
    pub data: SandboxTesterClearPurchaseHistoryData => "data";
}
}

json_struct! {
/// SandboxTesterClearPurchaseHistoryData contains the data for clearing purchase history.
pub struct SandboxTesterClearPurchaseHistoryData ("api.SandboxTesterClearPurchaseHistoryData", "SandboxTesterClearPurchaseHistoryData") {
    pub type_: String => "type";
    pub relationships: SandboxTesterClearPurchaseHistoryRelationships => "relationships";
}
}

json_struct! {
/// SandboxTesterClearPurchaseHistoryRelationships contains relationships for clearing purchase history.
pub struct SandboxTesterClearPurchaseHistoryRelationships ("api.SandboxTesterClearPurchaseHistoryRelationships", "SandboxTesterClearPurchaseHistoryRelationships") {
    pub sandbox_testers: RelationshipDataList => "sandboxTesters";
}
}

json_struct! {
/// SandboxTesterClearPurchaseHistoryResponse represents a clear purchase history response.
pub struct SandboxTesterClearPurchaseHistoryResponse ("api.SandboxTesterClearPurchaseHistoryResponse", "SandboxTesterClearPurchaseHistoryResponse") {
    pub data: SandboxTesterClearPurchaseHistory => "data";
}
}

json_struct! {
/// SandboxTesterClearPurchaseHistory represents a clear purchase history resource.
pub struct SandboxTesterClearPurchaseHistory ("api.SandboxTesterClearPurchaseHistory", "SandboxTesterClearPurchaseHistory") {
    pub type_: String => "type";
    pub id: String => "id";
}
}

json_struct! {
/// SandboxTesterUpdateRequest represents a request to update a sandbox tester.
pub struct SandboxTesterUpdateRequest ("api.SandboxTesterUpdateRequest", "SandboxTesterUpdateRequest") {
    pub data: SandboxTesterUpdateData => "data";
}
}

json_struct! {
/// SandboxTesterUpdateData contains the data for updating a sandbox tester.
pub struct SandboxTesterUpdateData ("api.SandboxTesterUpdateData", "SandboxTesterUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: SandboxTesterUpdateAttributes => "attributes";
}
}

json_struct! {
/// SandboxTesterUpdateAttributes contains attributes for updating a sandbox tester.
pub struct SandboxTesterUpdateAttributes ("api.SandboxTesterUpdateAttributes", "SandboxTesterUpdateAttributes") {
    pub interruptable: Option<bool> => "interruptable", omitempty;
    pub subscription_renewal_rate: String => "subscriptionRenewalRate", omitempty;
    pub territory: String => "territory", omitempty;
}
}

json_struct! {
/// PromotedPurchasesResponse represents a list of promoted purchases.
pub struct PromotedPurchasesResponse ("api.PromotedPurchasesResponse", "PromotedPurchasesResponse") {
    pub data: JsonSlice<PromotedPurchase> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// PromotedPurchaseResponse represents a single promoted purchase.
pub struct PromotedPurchaseResponse ("api.PromotedPurchaseResponse", "PromotedPurchaseResponse") {
    pub data: PromotedPurchase => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// PromotedPurchase represents a promoted purchase.
pub struct PromotedPurchase ("api.PromotedPurchase", "PromotedPurchase") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: PromotedPurchaseAttributes => "attributes";
}
}

json_struct! {
/// PromotedPurchaseAttributes contains promoted purchase attributes.
pub struct PromotedPurchaseAttributes ("api.PromotedPurchaseAttributes", "PromotedPurchaseAttributes") {
    pub visible_for_all_users: bool => "visibleForAllUsers", omitempty;
    pub enabled: bool => "enabled", omitempty;
    pub state: String => "state", omitempty;
}
}

json_struct! {
/// PromotedPurchaseCreateRequest represents a request to create a promoted purchase.
pub struct PromotedPurchaseCreateRequest ("api.PromotedPurchaseCreateRequest", "PromotedPurchaseCreateRequest") {
    pub data: PromotedPurchaseCreateData => "data";
}
}

json_struct! {
/// PromotedPurchaseCreateData contains the data for creating a promoted purchase.
pub struct PromotedPurchaseCreateData ("api.PromotedPurchaseCreateData", "PromotedPurchaseCreateData") {
    pub type_: String => "type";
    pub attributes: PromotedPurchaseCreateAttributes => "attributes";
    pub relationships: PromotedPurchaseCreateRelationships => "relationships";
}
}

json_struct! {
/// PromotedPurchaseCreateAttributes contains attributes for creating a promoted purchase.
pub struct PromotedPurchaseCreateAttributes ("api.PromotedPurchaseCreateAttributes", "PromotedPurchaseCreateAttributes") {
    pub visible_for_all_users: bool => "visibleForAllUsers";
    pub enabled: bool => "enabled";
}
}

json_struct! {
/// PromotedPurchaseCreateRelationships contains relationships for creating a promoted purchase.
pub struct PromotedPurchaseCreateRelationships ("api.PromotedPurchaseCreateRelationships", "PromotedPurchaseCreateRelationships") {
    pub app: RelationshipData => "app";
    pub in_app_purchase: RelationshipData => "inAppPurchaseV2";
}
}

json_struct! {
/// PromotedPurchaseUpdateRequest represents a request to update a promoted purchase.
pub struct PromotedPurchaseUpdateRequest ("api.PromotedPurchaseUpdateRequest", "PromotedPurchaseUpdateRequest") {
    pub data: PromotedPurchaseUpdateData => "data";
}
}

json_struct! {
/// PromotedPurchaseUpdateData contains the data for updating a promoted purchase.
pub struct PromotedPurchaseUpdateData ("api.PromotedPurchaseUpdateData", "PromotedPurchaseUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: PromotedPurchaseUpdateAttributes => "attributes";
}
}

json_struct! {
/// PromotedPurchaseUpdateAttributes contains attributes for updating a promoted purchase.
pub struct PromotedPurchaseUpdateAttributes ("api.PromotedPurchaseUpdateAttributes", "PromotedPurchaseUpdateAttributes") {
    pub visible_for_all_users: Option<bool> => "visibleForAllUsers", omitempty;
    pub enabled: Option<bool> => "enabled", omitempty;
}
}

json_struct! {
/// SubscriptionOfferCodesResponse represents a list of subscription offer codes.
pub struct SubscriptionOfferCodesResponse ("api.SubscriptionOfferCodesResponse", "SubscriptionOfferCodesResponse") {
    pub data: JsonSlice<SubscriptionOfferCode> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// SubscriptionOfferCodeResponse represents a single subscription offer code.
pub struct SubscriptionOfferCodeResponse ("api.SubscriptionOfferCodeResponse", "SubscriptionOfferCodeResponse") {
    pub data: SubscriptionOfferCode => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// SubscriptionOfferCode represents a subscription offer code.
pub struct SubscriptionOfferCode ("api.SubscriptionOfferCode", "SubscriptionOfferCode") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: SubscriptionOfferCodeAttributes => "attributes";
}
}

json_struct! {
/// SubscriptionOfferCodeAttributes contains subscription offer code attributes.
pub struct SubscriptionOfferCodeAttributes ("api.SubscriptionOfferCodeAttributes", "SubscriptionOfferCodeAttributes") {
    pub name: String => "name", omitempty;
    pub customer_eligibilities: JsonSlice<String> => "customerEligibilities", omitempty;
    pub offer_eligibility: String => "offerEligibility", omitempty;
    pub duration: String => "duration", omitempty;
    pub offer_mode: String => "offerMode", omitempty;
    pub number_of_periods: i64 => "numberOfPeriods", omitempty;
    pub total_number_of_codes: i64 => "totalNumberOfCodes", omitempty;
    pub active: bool => "active", omitempty;
}
}

json_struct! {
/// SubscriptionOfferCodeCreateRequest represents a request to create a subscription offer code.
pub struct SubscriptionOfferCodeCreateRequest ("api.SubscriptionOfferCodeCreateRequest", "SubscriptionOfferCodeCreateRequest") {
    pub data: SubscriptionOfferCodeCreateData => "data";
}
}

json_struct! {
/// SubscriptionOfferCodeCreateData contains the data for creating a subscription offer code.
pub struct SubscriptionOfferCodeCreateData ("api.SubscriptionOfferCodeCreateData", "SubscriptionOfferCodeCreateData") {
    pub type_: String => "type";
    pub attributes: SubscriptionOfferCodeCreateAttributes => "attributes";
    pub relationships: SubscriptionOfferCodeCreateRelationships => "relationships";
}
}

json_struct! {
/// SubscriptionOfferCodeCreateAttributes contains attributes for creating a subscription offer code.
pub struct SubscriptionOfferCodeCreateAttributes ("api.SubscriptionOfferCodeCreateAttributes", "SubscriptionOfferCodeCreateAttributes") {
    pub name: String => "name";
    pub customer_eligibilities: JsonSlice<String> => "customerEligibilities";
    pub offer_eligibility: String => "offerEligibility";
    pub duration: String => "duration";
    pub offer_mode: String => "offerMode";
    pub number_of_periods: i64 => "numberOfPeriods";
}
}

json_struct! {
/// SubscriptionOfferCodeCreateRelationships contains relationships for creating a subscription offer code.
pub struct SubscriptionOfferCodeCreateRelationships ("api.SubscriptionOfferCodeCreateRelationships", "SubscriptionOfferCodeCreateRelationships") {
    pub subscription: RelationshipData => "subscription";
}
}

json_struct! {
/// SubscriptionOfferCodeUpdateRequest represents a request to update a subscription offer code.
pub struct SubscriptionOfferCodeUpdateRequest ("api.SubscriptionOfferCodeUpdateRequest", "SubscriptionOfferCodeUpdateRequest") {
    pub data: SubscriptionOfferCodeUpdateData => "data";
}
}

json_struct! {
/// SubscriptionOfferCodeUpdateData contains the data for updating a subscription offer code.
pub struct SubscriptionOfferCodeUpdateData ("api.SubscriptionOfferCodeUpdateData", "SubscriptionOfferCodeUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: SubscriptionOfferCodeUpdateAttributes => "attributes";
}
}

json_struct! {
/// SubscriptionOfferCodeUpdateAttributes contains attributes for updating a subscription offer code.
pub struct SubscriptionOfferCodeUpdateAttributes ("api.SubscriptionOfferCodeUpdateAttributes", "SubscriptionOfferCodeUpdateAttributes") {
    pub active: Option<bool> => "active", omitempty;
}
}

json_struct! {
/// SubscriptionPricePointsResponse represents a list of subscription price points.
pub struct SubscriptionPricePointsResponse ("api.SubscriptionPricePointsResponse", "SubscriptionPricePointsResponse") {
    pub data: JsonSlice<SubscriptionPricePoint> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// SubscriptionPricePointResponse represents a single subscription price point.
pub struct SubscriptionPricePointResponse ("api.SubscriptionPricePointResponse", "SubscriptionPricePointResponse") {
    pub data: SubscriptionPricePoint => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// SubscriptionPricePoint represents a subscription price point.
pub struct SubscriptionPricePoint ("api.SubscriptionPricePoint", "SubscriptionPricePoint") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: SubscriptionPricePointAttributes => "attributes";
}
}

json_struct! {
/// SubscriptionPricePointAttributes contains subscription price point attributes.
pub struct SubscriptionPricePointAttributes ("api.SubscriptionPricePointAttributes", "SubscriptionPricePointAttributes") {
    pub customer_price: String => "customerPrice", omitempty;
    pub proceeds: String => "proceeds", omitempty;
    pub proceeds_year2: String => "proceedsYear2", omitempty;
}
}

json_struct! {
/// WinBackOffersResponse represents a list of win-back offers.
pub struct WinBackOffersResponse ("api.WinBackOffersResponse", "WinBackOffersResponse") {
    pub data: JsonSlice<WinBackOffer> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// WinBackOfferResponse represents a single win-back offer.
pub struct WinBackOfferResponse ("api.WinBackOfferResponse", "WinBackOfferResponse") {
    pub data: WinBackOffer => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// WinBackOffer represents a win-back offer.
pub struct WinBackOffer ("api.WinBackOffer", "WinBackOffer") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: WinBackOfferAttributes => "attributes";
}
}

json_struct! {
/// WinBackOfferAttributes contains win-back offer attributes.
pub struct WinBackOfferAttributes ("api.WinBackOfferAttributes", "WinBackOfferAttributes") {
    pub reference_name: String => "referenceName", omitempty;
    pub offer_id: String => "offerId", omitempty;
    pub duration: String => "duration", omitempty;
    pub offer_mode: String => "offerMode", omitempty;
    pub period_count: i64 => "periodCount", omitempty;
    pub customer_eligibility_paid_subscription_duration_in_months: i64 => "customerEligibilityPaidSubscriptionDurationInMonths", omitempty;
    pub customer_eligibility_time_since_last_subscribed_in_months: Option<IntegerRange> => "customerEligibilityTimeSinceLastSubscribedInMonths", omitempty;
    pub customer_eligibility_wait_between_offers_in_months: i64 => "customerEligibilityWaitBetweenOffersInMonths", omitempty;
    pub start_date: Option<Timestamp> => "startDate", omitempty;
    pub end_date: Option<Timestamp> => "endDate", omitempty;
    pub priority: String => "priority", omitempty;
    pub promotion_intent: String => "promotionIntent", omitempty;
}
}

json_struct! {
/// IntegerRange represents an integer range.
pub struct IntegerRange ("api.IntegerRange", "IntegerRange") {
    pub minimum: i64 => "minimum", omitempty;
    pub maximum: i64 => "maximum", omitempty;
}
}

json_struct! {
/// WinBackOfferCreateRequest represents a request to create a win-back offer.
pub struct WinBackOfferCreateRequest ("api.WinBackOfferCreateRequest", "WinBackOfferCreateRequest") {
    pub data: WinBackOfferCreateData => "data";
}
}

json_struct! {
/// WinBackOfferCreateData contains the data for creating a win-back offer.
pub struct WinBackOfferCreateData ("api.WinBackOfferCreateData", "WinBackOfferCreateData") {
    pub type_: String => "type";
    pub attributes: WinBackOfferCreateAttributes => "attributes";
    pub relationships: WinBackOfferCreateRelationships => "relationships";
}
}

json_struct! {
/// WinBackOfferCreateAttributes contains attributes for creating a win-back offer.
pub struct WinBackOfferCreateAttributes ("api.WinBackOfferCreateAttributes", "WinBackOfferCreateAttributes") {
    pub reference_name: String => "referenceName";
    pub offer_id: String => "offerId";
    pub duration: String => "duration";
    pub offer_mode: String => "offerMode";
    pub period_count: i64 => "periodCount";
    pub customer_eligibility_paid_subscription_duration_in_months: i64 => "customerEligibilityPaidSubscriptionDurationInMonths";
    pub customer_eligibility_time_since_last_subscribed_in_months: Option<IntegerRange> => "customerEligibilityTimeSinceLastSubscribedInMonths", omitempty;
    pub customer_eligibility_wait_between_offers_in_months: i64 => "customerEligibilityWaitBetweenOffersInMonths", omitempty;
    pub start_date: Option<Timestamp> => "startDate", omitempty;
    pub end_date: Option<Timestamp> => "endDate", omitempty;
    pub priority: String => "priority";
    pub promotion_intent: String => "promotionIntent", omitempty;
}
}

json_struct! {
/// WinBackOfferCreateRelationships contains relationships for creating a win-back offer.
pub struct WinBackOfferCreateRelationships ("api.WinBackOfferCreateRelationships", "WinBackOfferCreateRelationships") {
    pub subscription: RelationshipData => "subscription";
    pub prices: RelationshipDataList => "prices";
}
}

json_struct! {
/// WinBackOfferUpdateRequest represents a request to update a win-back offer.
pub struct WinBackOfferUpdateRequest ("api.WinBackOfferUpdateRequest", "WinBackOfferUpdateRequest") {
    pub data: WinBackOfferUpdateData => "data";
}
}

json_struct! {
/// WinBackOfferUpdateData contains the data for updating a win-back offer.
pub struct WinBackOfferUpdateData ("api.WinBackOfferUpdateData", "WinBackOfferUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: WinBackOfferUpdateAttributes => "attributes";
}
}

json_struct! {
/// WinBackOfferUpdateAttributes contains attributes for updating a win-back offer.
pub struct WinBackOfferUpdateAttributes ("api.WinBackOfferUpdateAttributes", "WinBackOfferUpdateAttributes") {
    pub customer_eligibility_paid_subscription_duration_in_months: Option<i64> => "customerEligibilityPaidSubscriptionDurationInMonths", omitempty;
    pub customer_eligibility_time_since_last_subscribed_in_months: Option<IntegerRange> => "customerEligibilityTimeSinceLastSubscribedInMonths", omitempty;
    pub customer_eligibility_wait_between_offers_in_months: Option<i64> => "customerEligibilityWaitBetweenOffersInMonths", omitempty;
    pub start_date: Option<Timestamp> => "startDate", omitempty;
    pub end_date: Option<Timestamp> => "endDate", omitempty;
    pub priority: String => "priority", omitempty;
    pub promotion_intent: String => "promotionIntent", omitempty;
}
}

json_struct! {
/// AppStoreVersionExperimentsResponse represents a list of experiments.
pub struct AppStoreVersionExperimentsResponse ("api.AppStoreVersionExperimentsResponse", "AppStoreVersionExperimentsResponse") {
    pub data: JsonSlice<AppStoreVersionExperiment> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppStoreVersionExperimentResponse represents a single experiment.
pub struct AppStoreVersionExperimentResponse ("api.AppStoreVersionExperimentResponse", "AppStoreVersionExperimentResponse") {
    pub data: AppStoreVersionExperiment => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppStoreVersionExperiment represents an App Store version experiment.
pub struct AppStoreVersionExperiment ("api.AppStoreVersionExperiment", "AppStoreVersionExperiment") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppStoreVersionExperimentAttributes => "attributes";
}
}

json_struct! {
/// AppStoreVersionExperimentAttributes contains experiment attributes.
pub struct AppStoreVersionExperimentAttributes ("api.AppStoreVersionExperimentAttributes", "AppStoreVersionExperimentAttributes") {
    pub name: String => "name", omitempty;
    pub platform: String => "platform", omitempty;
    pub traffic_proportion: i64 => "trafficProportion", omitempty;
    pub state: String => "state", omitempty;
    pub review_required: bool => "reviewRequired", omitempty;
    pub start_date: Option<Timestamp> => "startDate", omitempty;
    pub end_date: Option<Timestamp> => "endDate", omitempty;
    pub latest_control_version: String => "latestControlVersion", omitempty;
    pub control_version_start_date: Option<Timestamp> => "controlVersionStartDate", omitempty;
}
}

json_struct! {
/// AppStoreVersionExperimentCreateRequest represents a request to create an experiment.
pub struct AppStoreVersionExperimentCreateRequest ("api.AppStoreVersionExperimentCreateRequest", "AppStoreVersionExperimentCreateRequest") {
    pub data: AppStoreVersionExperimentCreateData => "data";
}
}

json_struct! {
/// AppStoreVersionExperimentCreateData contains the data for creating an experiment.
pub struct AppStoreVersionExperimentCreateData ("api.AppStoreVersionExperimentCreateData", "AppStoreVersionExperimentCreateData") {
    pub type_: String => "type";
    pub attributes: AppStoreVersionExperimentCreateAttributes => "attributes";
    pub relationships: AppStoreVersionExperimentCreateRelationships => "relationships";
}
}

json_struct! {
/// AppStoreVersionExperimentCreateAttributes contains attributes for creating an experiment.
/// The v2 experiments API requires name, platform, and trafficProportion.
pub struct AppStoreVersionExperimentCreateAttributes ("api.AppStoreVersionExperimentCreateAttributes", "AppStoreVersionExperimentCreateAttributes") {
    pub name: String => "name";
    pub platform: String => "platform";
    pub traffic_proportion: i64 => "trafficProportion";
}
}

json_struct! {
/// AppStoreVersionExperimentCreateRelationships contains relationships for creating an experiment.
/// The v2 experiments API attaches experiments to an app rather than a version.
pub struct AppStoreVersionExperimentCreateRelationships ("api.AppStoreVersionExperimentCreateRelationships", "AppStoreVersionExperimentCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// AppStoreVersionExperimentUpdateRequest represents a request to update an experiment.
pub struct AppStoreVersionExperimentUpdateRequest ("api.AppStoreVersionExperimentUpdateRequest", "AppStoreVersionExperimentUpdateRequest") {
    pub data: AppStoreVersionExperimentUpdateData => "data";
}
}

json_struct! {
/// AppStoreVersionExperimentUpdateData contains the data for updating an experiment.
pub struct AppStoreVersionExperimentUpdateData ("api.AppStoreVersionExperimentUpdateData", "AppStoreVersionExperimentUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppStoreVersionExperimentUpdateAttributes => "attributes";
}
}

json_struct! {
/// AppStoreVersionExperimentUpdateAttributes contains attributes for updating an experiment.
pub struct AppStoreVersionExperimentUpdateAttributes ("api.AppStoreVersionExperimentUpdateAttributes", "AppStoreVersionExperimentUpdateAttributes") {
    pub name: String => "name", omitempty;
    pub traffic_proportion: Option<i64> => "trafficProportion", omitempty;
    pub started: Option<bool> => "started", omitempty;
}
}

json_struct! {
/// AppCustomProductPagesResponse represents a list of custom product pages.
pub struct AppCustomProductPagesResponse ("api.AppCustomProductPagesResponse", "AppCustomProductPagesResponse") {
    pub data: JsonSlice<AppCustomProductPage> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppCustomProductPageResponse represents a single custom product page.
pub struct AppCustomProductPageResponse ("api.AppCustomProductPageResponse", "AppCustomProductPageResponse") {
    pub data: AppCustomProductPage => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppCustomProductPage represents a custom product page.
pub struct AppCustomProductPage ("api.AppCustomProductPage", "AppCustomProductPage") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppCustomProductPageAttributes => "attributes";
}
}

json_struct! {
/// AppCustomProductPageAttributes contains custom product page attributes.
pub struct AppCustomProductPageAttributes ("api.AppCustomProductPageAttributes", "AppCustomProductPageAttributes") {
    pub name: String => "name", omitempty;
    pub url: String => "url", omitempty;
    pub visible: bool => "visible", omitempty;
}
}

json_struct! {
/// AppCustomProductPageCreateRequest represents a request to create a custom product page.
pub struct AppCustomProductPageCreateRequest ("api.AppCustomProductPageCreateRequest", "AppCustomProductPageCreateRequest") {
    pub data: AppCustomProductPageCreateData => "data";
}
}

json_struct! {
/// AppCustomProductPageCreateData contains the data for creating a custom product page.
pub struct AppCustomProductPageCreateData ("api.AppCustomProductPageCreateData", "AppCustomProductPageCreateData") {
    pub type_: String => "type";
    pub attributes: AppCustomProductPageCreateAttributes => "attributes";
    pub relationships: AppCustomProductPageCreateRelationships => "relationships";
}
}

json_struct! {
/// AppCustomProductPageCreateAttributes contains attributes for creating a custom product page.
pub struct AppCustomProductPageCreateAttributes ("api.AppCustomProductPageCreateAttributes", "AppCustomProductPageCreateAttributes") {
    pub name: String => "name";
}
}

json_struct! {
/// AppCustomProductPageCreateRelationships contains relationships for creating a custom product page.
pub struct AppCustomProductPageCreateRelationships ("api.AppCustomProductPageCreateRelationships", "AppCustomProductPageCreateRelationships") {
    pub app: RelationshipData => "app";
    pub app_store_version_template: Option<RelationshipData> => "appStoreVersionTemplate", omitempty;
}
}

json_struct! {
/// AppCustomProductPageUpdateRequest represents a request to update a custom product page.
pub struct AppCustomProductPageUpdateRequest ("api.AppCustomProductPageUpdateRequest", "AppCustomProductPageUpdateRequest") {
    pub data: AppCustomProductPageUpdateData => "data";
}
}

json_struct! {
/// AppCustomProductPageUpdateData contains the data for updating a custom product page.
pub struct AppCustomProductPageUpdateData ("api.AppCustomProductPageUpdateData", "AppCustomProductPageUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppCustomProductPageUpdateAttributes => "attributes";
}
}

json_struct! {
/// AppCustomProductPageUpdateAttributes contains attributes for updating a custom product page.
pub struct AppCustomProductPageUpdateAttributes ("api.AppCustomProductPageUpdateAttributes", "AppCustomProductPageUpdateAttributes") {
    pub name: String => "name", omitempty;
    pub visible: Option<bool> => "visible", omitempty;
}
}

json_struct! {
/// RoutingAppCoverageResponse represents a routing app coverage.
pub struct RoutingAppCoverageResponse ("api.RoutingAppCoverageResponse", "RoutingAppCoverageResponse") {
    pub data: RoutingAppCoverage => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// RoutingAppCoverage represents routing app coverage.
pub struct RoutingAppCoverage ("api.RoutingAppCoverage", "RoutingAppCoverage") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: RoutingAppCoverageAttributes => "attributes";
}
}

json_struct! {
/// RoutingAppCoverageAttributes contains routing app coverage attributes.
pub struct RoutingAppCoverageAttributes ("api.RoutingAppCoverageAttributes", "RoutingAppCoverageAttributes") {
    pub file_size: i64 => "fileSize", omitempty;
    pub file_name: String => "fileName", omitempty;
    pub source_file_checksum: String => "sourceFileChecksum", omitempty;
    pub upload_operations: JsonSlice<UploadOperation> => "uploadOperations", omitempty;
    pub asset_delivery_state: Option<AssetDeliveryState> => "assetDeliveryState", omitempty;
}
}

json_struct! {
/// RoutingAppCoverageCreateRequest represents a request to create routing app coverage.
pub struct RoutingAppCoverageCreateRequest ("api.RoutingAppCoverageCreateRequest", "RoutingAppCoverageCreateRequest") {
    pub data: RoutingAppCoverageCreateData => "data";
}
}

json_struct! {
/// RoutingAppCoverageCreateData contains the data for creating routing app coverage.
pub struct RoutingAppCoverageCreateData ("api.RoutingAppCoverageCreateData", "RoutingAppCoverageCreateData") {
    pub type_: String => "type";
    pub attributes: RoutingAppCoverageCreateAttributes => "attributes";
    pub relationships: RoutingAppCoverageCreateRelationships => "relationships";
}
}

json_struct! {
/// RoutingAppCoverageCreateAttributes contains attributes for creating routing app coverage.
pub struct RoutingAppCoverageCreateAttributes ("api.RoutingAppCoverageCreateAttributes", "RoutingAppCoverageCreateAttributes") {
    pub file_size: i64 => "fileSize";
    pub file_name: String => "fileName";
}
}

json_struct! {
/// RoutingAppCoverageCreateRelationships contains relationships for creating routing app coverage.
pub struct RoutingAppCoverageCreateRelationships ("api.RoutingAppCoverageCreateRelationships", "RoutingAppCoverageCreateRelationships") {
    pub app_store_version: RelationshipData => "appStoreVersion";
}
}

json_struct! {
/// RoutingAppCoverageUpdateRequest represents a request to update routing app coverage.
pub struct RoutingAppCoverageUpdateRequest ("api.RoutingAppCoverageUpdateRequest", "RoutingAppCoverageUpdateRequest") {
    pub data: RoutingAppCoverageUpdateData => "data";
}
}

json_struct! {
/// RoutingAppCoverageUpdateData contains the data for updating routing app coverage.
pub struct RoutingAppCoverageUpdateData ("api.RoutingAppCoverageUpdateData", "RoutingAppCoverageUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: RoutingAppCoverageUpdateAttributes => "attributes";
}
}

json_struct! {
/// RoutingAppCoverageUpdateAttributes contains attributes for updating routing app coverage.
pub struct RoutingAppCoverageUpdateAttributes ("api.RoutingAppCoverageUpdateAttributes", "RoutingAppCoverageUpdateAttributes") {
    pub source_file_checksum: String => "sourceFileChecksum", omitempty;
    pub uploaded: Option<bool> => "uploaded", omitempty;
}
}

json_struct! {
/// PerfPowerMetricsResponse represents power and performance metrics.
pub struct PerfPowerMetricsResponse ("api.PerfPowerMetricsResponse", "PerfPowerMetricsResponse") {
    pub data: JsonSlice<PerfPowerMetric> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// PerfPowerMetric represents a performance/power metric.
pub struct PerfPowerMetric ("api.PerfPowerMetric", "PerfPowerMetric") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: PerfPowerMetricAttributes => "attributes";
}
}

json_struct! {
/// PerfPowerMetricAttributes contains metric attributes.
pub struct PerfPowerMetricAttributes ("api.PerfPowerMetricAttributes", "PerfPowerMetricAttributes") {
    pub metric_type: String => "metricType", omitempty;
    pub device_type: String => "deviceType", omitempty;
    pub platform: String => "platform", omitempty;
}
}

json_struct! {
/// DiagnosticLogsResponse represents diagnostic logs.
pub struct DiagnosticLogsResponse ("api.DiagnosticLogsResponse", "DiagnosticLogsResponse") {
    pub data: JsonSlice<DiagnosticLog> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// DiagnosticLog represents a diagnostic log.
pub struct DiagnosticLog ("api.DiagnosticLog", "DiagnosticLog") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: DiagnosticLogAttributes => "attributes";
}
}

json_struct! {
/// DiagnosticLogAttributes contains diagnostic log attributes.
pub struct DiagnosticLogAttributes ("api.DiagnosticLogAttributes", "DiagnosticLogAttributes") {
    pub diagnostic_type: String => "diagnosticType", omitempty;
}
}

json_struct! {
/// DiagnosticSignaturesResponse represents diagnostic signatures.
pub struct DiagnosticSignaturesResponse ("api.DiagnosticSignaturesResponse", "DiagnosticSignaturesResponse") {
    pub data: JsonSlice<DiagnosticSignature> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// DiagnosticSignature represents a diagnostic signature.
pub struct DiagnosticSignature ("api.DiagnosticSignature", "DiagnosticSignature") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: DiagnosticSignatureAttributes => "attributes";
}
}

json_struct! {
/// DiagnosticSignatureAttributes contains diagnostic signature attributes.
pub struct DiagnosticSignatureAttributes ("api.DiagnosticSignatureAttributes", "DiagnosticSignatureAttributes") {
    pub diagnostic_type: String => "diagnosticType", omitempty;
    pub signature: String => "signature", omitempty;
    pub weight: f64 => "weight", omitempty;
}
}

json_struct! {
/// AppStoreReviewAttachmentsResponse represents a list of review attachments.
pub struct AppStoreReviewAttachmentsResponse ("api.AppStoreReviewAttachmentsResponse", "AppStoreReviewAttachmentsResponse") {
    pub data: JsonSlice<AppStoreReviewAttachment> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppStoreReviewAttachmentResponse represents a single review attachment.
pub struct AppStoreReviewAttachmentResponse ("api.AppStoreReviewAttachmentResponse", "AppStoreReviewAttachmentResponse") {
    pub data: AppStoreReviewAttachment => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppStoreReviewAttachment represents a review attachment.
pub struct AppStoreReviewAttachment ("api.AppStoreReviewAttachment", "AppStoreReviewAttachment") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppStoreReviewAttachmentAttributes => "attributes";
}
}

json_struct! {
/// AppStoreReviewAttachmentAttributes contains review attachment attributes.
pub struct AppStoreReviewAttachmentAttributes ("api.AppStoreReviewAttachmentAttributes", "AppStoreReviewAttachmentAttributes") {
    pub file_size: i64 => "fileSize", omitempty;
    pub file_name: String => "fileName", omitempty;
    pub source_file_checksum: String => "sourceFileChecksum", omitempty;
    pub upload_operations: JsonSlice<UploadOperation> => "uploadOperations", omitempty;
    pub asset_delivery_state: Option<AssetDeliveryState> => "assetDeliveryState", omitempty;
}
}

json_struct! {
/// AppStoreReviewAttachmentCreateRequest represents a request to create a review attachment.
pub struct AppStoreReviewAttachmentCreateRequest ("api.AppStoreReviewAttachmentCreateRequest", "AppStoreReviewAttachmentCreateRequest") {
    pub data: AppStoreReviewAttachmentCreateData => "data";
}
}

json_struct! {
/// AppStoreReviewAttachmentCreateData contains the data for creating a review attachment.
pub struct AppStoreReviewAttachmentCreateData ("api.AppStoreReviewAttachmentCreateData", "AppStoreReviewAttachmentCreateData") {
    pub type_: String => "type";
    pub attributes: AppStoreReviewAttachmentCreateAttributes => "attributes";
    pub relationships: AppStoreReviewAttachmentCreateRelationships => "relationships";
}
}

json_struct! {
/// AppStoreReviewAttachmentCreateAttributes contains attributes for creating a review attachment.
pub struct AppStoreReviewAttachmentCreateAttributes ("api.AppStoreReviewAttachmentCreateAttributes", "AppStoreReviewAttachmentCreateAttributes") {
    pub file_size: i64 => "fileSize";
    pub file_name: String => "fileName";
}
}

json_struct! {
/// AppStoreReviewAttachmentCreateRelationships contains relationships for creating a review attachment.
pub struct AppStoreReviewAttachmentCreateRelationships ("api.AppStoreReviewAttachmentCreateRelationships", "AppStoreReviewAttachmentCreateRelationships") {
    pub app_store_review_detail: RelationshipData => "appStoreReviewDetail";
}
}

json_struct! {
/// AppStoreReviewAttachmentUpdateRequest represents a request to update a review attachment.
pub struct AppStoreReviewAttachmentUpdateRequest ("api.AppStoreReviewAttachmentUpdateRequest", "AppStoreReviewAttachmentUpdateRequest") {
    pub data: AppStoreReviewAttachmentUpdateData => "data";
}
}

json_struct! {
/// AppStoreReviewAttachmentUpdateData contains the data for updating a review attachment.
pub struct AppStoreReviewAttachmentUpdateData ("api.AppStoreReviewAttachmentUpdateData", "AppStoreReviewAttachmentUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppStoreReviewAttachmentUpdateAttributes => "attributes";
}
}

json_struct! {
/// AppStoreReviewAttachmentUpdateAttributes contains attributes for updating a review attachment.
pub struct AppStoreReviewAttachmentUpdateAttributes ("api.AppStoreReviewAttachmentUpdateAttributes", "AppStoreReviewAttachmentUpdateAttributes") {
    pub source_file_checksum: String => "sourceFileChecksum", omitempty;
    pub uploaded: Option<bool> => "uploaded", omitempty;
}
}

json_struct! {
/// AppCategoriesResponse represents a list of app categories.
pub struct AppCategoriesResponse ("api.AppCategoriesResponse", "AppCategoriesResponse") {
    pub data: JsonSlice<AppCategory> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppCategoryResponse represents a single app category.
pub struct AppCategoryResponse ("api.AppCategoryResponse", "AppCategoryResponse") {
    pub data: AppCategory => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AppCategory represents an app category.
pub struct AppCategory ("api.AppCategory", "AppCategory") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AppCategoryAttributes => "attributes";
}
}

json_struct! {
/// AppCategoryAttributes contains app category attributes.
pub struct AppCategoryAttributes ("api.AppCategoryAttributes", "AppCategoryAttributes") {
    pub platforms: JsonSlice<String> => "platforms", omitempty;
}
}

json_struct! {
/// BetaAppLocalizationsResponse represents a list of beta app localizations.
pub struct BetaAppLocalizationsResponse ("api.BetaAppLocalizationsResponse", "BetaAppLocalizationsResponse") {
    pub data: JsonSlice<BetaAppLocalization> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaAppLocalizationResponse represents a single beta app localization.
pub struct BetaAppLocalizationResponse ("api.BetaAppLocalizationResponse", "BetaAppLocalizationResponse") {
    pub data: BetaAppLocalization => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaAppLocalization represents a beta app localization.
pub struct BetaAppLocalization ("api.BetaAppLocalization", "BetaAppLocalization") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BetaAppLocalizationAttributes => "attributes";
}
}

json_struct! {
/// BetaAppLocalizationAttributes contains beta app localization attributes.
pub struct BetaAppLocalizationAttributes ("api.BetaAppLocalizationAttributes", "BetaAppLocalizationAttributes") {
    pub feedback_email: String => "feedbackEmail", omitempty;
    pub marketing_url: String => "marketingUrl", omitempty;
    pub privacy_policy_url: String => "privacyPolicyUrl", omitempty;
    pub tv_os_privacy_policy: String => "tvOsPrivacyPolicy", omitempty;
    pub description: String => "description", omitempty;
    pub locale: String => "locale", omitempty;
}
}

json_struct! {
/// BetaAppLocalizationCreateRequest represents a request to create a beta app localization.
pub struct BetaAppLocalizationCreateRequest ("api.BetaAppLocalizationCreateRequest", "BetaAppLocalizationCreateRequest") {
    pub data: BetaAppLocalizationCreateData => "data";
}
}

json_struct! {
/// BetaAppLocalizationCreateData contains the data for creating a beta app localization.
pub struct BetaAppLocalizationCreateData ("api.BetaAppLocalizationCreateData", "BetaAppLocalizationCreateData") {
    pub type_: String => "type";
    pub attributes: BetaAppLocalizationCreateAttributes => "attributes";
    pub relationships: BetaAppLocalizationCreateRelationships => "relationships";
}
}

json_struct! {
/// BetaAppLocalizationCreateAttributes contains attributes for creating a beta app localization.
pub struct BetaAppLocalizationCreateAttributes ("api.BetaAppLocalizationCreateAttributes", "BetaAppLocalizationCreateAttributes") {
    pub feedback_email: String => "feedbackEmail", omitempty;
    pub marketing_url: String => "marketingUrl", omitempty;
    pub privacy_policy_url: String => "privacyPolicyUrl", omitempty;
    pub tv_os_privacy_policy: String => "tvOsPrivacyPolicy", omitempty;
    pub description: String => "description", omitempty;
    pub locale: String => "locale";
}
}

json_struct! {
/// BetaAppLocalizationCreateRelationships contains relationships for creating a beta app localization.
pub struct BetaAppLocalizationCreateRelationships ("api.BetaAppLocalizationCreateRelationships", "BetaAppLocalizationCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// BetaAppLocalizationUpdateRequest represents a request to update a beta app localization.
pub struct BetaAppLocalizationUpdateRequest ("api.BetaAppLocalizationUpdateRequest", "BetaAppLocalizationUpdateRequest") {
    pub data: BetaAppLocalizationUpdateData => "data";
}
}

json_struct! {
/// BetaAppLocalizationUpdateData contains the data for updating a beta app localization.
pub struct BetaAppLocalizationUpdateData ("api.BetaAppLocalizationUpdateData", "BetaAppLocalizationUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BetaAppLocalizationUpdateAttributes => "attributes";
}
}

json_struct! {
/// BetaAppLocalizationUpdateAttributes contains attributes for updating a beta app localization.
pub struct BetaAppLocalizationUpdateAttributes ("api.BetaAppLocalizationUpdateAttributes", "BetaAppLocalizationUpdateAttributes") {
    pub feedback_email: String => "feedbackEmail", omitempty;
    pub marketing_url: String => "marketingUrl", omitempty;
    pub privacy_policy_url: String => "privacyPolicyUrl", omitempty;
    pub tv_os_privacy_policy: String => "tvOsPrivacyPolicy", omitempty;
    pub description: String => "description", omitempty;
}
}

json_struct! {
/// BetaBuildLocalizationsResponse represents a list of beta build localizations.
pub struct BetaBuildLocalizationsResponse ("api.BetaBuildLocalizationsResponse", "BetaBuildLocalizationsResponse") {
    pub data: JsonSlice<BetaBuildLocalization> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaBuildLocalizationResponse represents a single beta build localization.
pub struct BetaBuildLocalizationResponse ("api.BetaBuildLocalizationResponse", "BetaBuildLocalizationResponse") {
    pub data: BetaBuildLocalization => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BetaBuildLocalization represents a beta build localization.
pub struct BetaBuildLocalization ("api.BetaBuildLocalization", "BetaBuildLocalization") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BetaBuildLocalizationAttributes => "attributes";
}
}

json_struct! {
/// BetaBuildLocalizationAttributes contains beta build localization attributes.
pub struct BetaBuildLocalizationAttributes ("api.BetaBuildLocalizationAttributes", "BetaBuildLocalizationAttributes") {
    pub whats_new: String => "whatsNew", omitempty;
    pub locale: String => "locale", omitempty;
}
}

json_struct! {
/// BetaBuildLocalizationCreateRequest represents a request to create a beta build localization.
pub struct BetaBuildLocalizationCreateRequest ("api.BetaBuildLocalizationCreateRequest", "BetaBuildLocalizationCreateRequest") {
    pub data: BetaBuildLocalizationCreateData => "data";
}
}

json_struct! {
/// BetaBuildLocalizationCreateData contains the data for creating a beta build localization.
pub struct BetaBuildLocalizationCreateData ("api.BetaBuildLocalizationCreateData", "BetaBuildLocalizationCreateData") {
    pub type_: String => "type";
    pub attributes: BetaBuildLocalizationCreateAttributes => "attributes";
    pub relationships: BetaBuildLocalizationCreateRelationships => "relationships";
}
}

json_struct! {
/// BetaBuildLocalizationCreateAttributes contains attributes for creating a beta build localization.
pub struct BetaBuildLocalizationCreateAttributes ("api.BetaBuildLocalizationCreateAttributes", "BetaBuildLocalizationCreateAttributes") {
    pub whats_new: String => "whatsNew", omitempty;
    pub locale: String => "locale";
}
}

json_struct! {
/// BetaBuildLocalizationCreateRelationships contains relationships for creating a beta build localization.
pub struct BetaBuildLocalizationCreateRelationships ("api.BetaBuildLocalizationCreateRelationships", "BetaBuildLocalizationCreateRelationships") {
    pub build: RelationshipData => "build";
}
}

json_struct! {
/// BetaBuildLocalizationUpdateRequest represents a request to update a beta build localization.
pub struct BetaBuildLocalizationUpdateRequest ("api.BetaBuildLocalizationUpdateRequest", "BetaBuildLocalizationUpdateRequest") {
    pub data: BetaBuildLocalizationUpdateData => "data";
}
}

json_struct! {
/// BetaBuildLocalizationUpdateData contains the data for updating a beta build localization.
pub struct BetaBuildLocalizationUpdateData ("api.BetaBuildLocalizationUpdateData", "BetaBuildLocalizationUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BetaBuildLocalizationUpdateAttributes => "attributes";
}
}

json_struct! {
/// BetaBuildLocalizationUpdateAttributes contains attributes for updating a beta build localization.
pub struct BetaBuildLocalizationUpdateAttributes ("api.BetaBuildLocalizationUpdateAttributes", "BetaBuildLocalizationUpdateAttributes") {
    pub whats_new: String => "whatsNew", omitempty;
}
}

json_struct! {
/// BuildBetaDetailResponse represents a build beta detail.
pub struct BuildBetaDetailResponse ("api.BuildBetaDetailResponse", "BuildBetaDetailResponse") {
    pub data: BuildBetaDetail => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BuildBetaDetailsResponse represents a list of build beta details.
pub struct BuildBetaDetailsResponse ("api.BuildBetaDetailsResponse", "BuildBetaDetailsResponse") {
    pub data: JsonSlice<BuildBetaDetail> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// BuildBetaDetail represents build beta details.
pub struct BuildBetaDetail ("api.BuildBetaDetail", "BuildBetaDetail") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BuildBetaDetailAttributes => "attributes";
}
}

json_struct! {
/// BuildBetaDetailAttributes contains build beta detail attributes.
pub struct BuildBetaDetailAttributes ("api.BuildBetaDetailAttributes", "BuildBetaDetailAttributes") {
    pub auto_notify_enabled: bool => "autoNotifyEnabled", omitempty;
    pub internal_build_state: String => "internalBuildState", omitempty;
    pub external_build_state: String => "externalBuildState", omitempty;
}
}

json_struct! {
/// BuildBetaDetailUpdateRequest represents a request to update a build beta detail.
pub struct BuildBetaDetailUpdateRequest ("api.BuildBetaDetailUpdateRequest", "BuildBetaDetailUpdateRequest") {
    pub data: BuildBetaDetailUpdateData => "data";
}
}

json_struct! {
/// BuildBetaDetailUpdateData contains the data for updating a build beta detail.
pub struct BuildBetaDetailUpdateData ("api.BuildBetaDetailUpdateData", "BuildBetaDetailUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: BuildBetaDetailUpdateAttributes => "attributes";
}
}

json_struct! {
/// BuildBetaDetailUpdateAttributes contains attributes for updating a build beta detail.
pub struct BuildBetaDetailUpdateAttributes ("api.BuildBetaDetailUpdateAttributes", "BuildBetaDetailUpdateAttributes") {
    pub auto_notify_enabled: Option<bool> => "autoNotifyEnabled", omitempty;
}
}

json_struct! {
/// AlternativeDistributionPackageResponse represents an alternative distribution package.
pub struct AlternativeDistributionPackageResponse ("api.AlternativeDistributionPackageResponse", "AlternativeDistributionPackageResponse") {
    pub data: AlternativeDistributionPackage => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AlternativeDistributionPackage represents an alternative distribution package.
pub struct AlternativeDistributionPackage ("api.AlternativeDistributionPackage", "AlternativeDistributionPackage") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AlternativeDistributionPackageAttributes => "attributes";
}
}

json_struct! {
/// AlternativeDistributionPackageAttributes contains alternative distribution package attributes.
pub struct AlternativeDistributionPackageAttributes ("api.AlternativeDistributionPackageAttributes", "AlternativeDistributionPackageAttributes") {
}
}

json_struct! {
/// AlternativeDistributionKeyResponse represents an alternative distribution key.
pub struct AlternativeDistributionKeyResponse ("api.AlternativeDistributionKeyResponse", "AlternativeDistributionKeyResponse") {
    pub data: AlternativeDistributionKey => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AlternativeDistributionKeysResponse represents a list of alternative distribution keys.
pub struct AlternativeDistributionKeysResponse ("api.AlternativeDistributionKeysResponse", "AlternativeDistributionKeysResponse") {
    pub data: JsonSlice<AlternativeDistributionKey> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// AlternativeDistributionKey represents an alternative distribution key.
pub struct AlternativeDistributionKey ("api.AlternativeDistributionKey", "AlternativeDistributionKey") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: AlternativeDistributionKeyAttributes => "attributes";
}
}

json_struct! {
/// AlternativeDistributionKeyAttributes contains alternative distribution key attributes.
pub struct AlternativeDistributionKeyAttributes ("api.AlternativeDistributionKeyAttributes", "AlternativeDistributionKeyAttributes") {
    pub public_key: String => "publicKey", omitempty;
}
}

json_struct! {
/// AlternativeDistributionKeyCreateRequest represents a request to create an alternative distribution key.
pub struct AlternativeDistributionKeyCreateRequest ("api.AlternativeDistributionKeyCreateRequest", "AlternativeDistributionKeyCreateRequest") {
    pub data: AlternativeDistributionKeyCreateData => "data";
}
}

json_struct! {
/// AlternativeDistributionKeyCreateData contains the data for creating an alternative distribution key.
pub struct AlternativeDistributionKeyCreateData ("api.AlternativeDistributionKeyCreateData", "AlternativeDistributionKeyCreateData") {
    pub type_: String => "type";
    pub attributes: AlternativeDistributionKeyCreateAttributes => "attributes";
    pub relationships: AlternativeDistributionKeyCreateRelationships => "relationships";
}
}

json_struct! {
/// AlternativeDistributionKeyCreateAttributes contains attributes for creating an alternative distribution key.
pub struct AlternativeDistributionKeyCreateAttributes ("api.AlternativeDistributionKeyCreateAttributes", "AlternativeDistributionKeyCreateAttributes") {
    pub public_key: String => "publicKey";
}
}

json_struct! {
/// AlternativeDistributionKeyCreateRelationships contains relationships for creating an alternative distribution key.
pub struct AlternativeDistributionKeyCreateRelationships ("api.AlternativeDistributionKeyCreateRelationships", "AlternativeDistributionKeyCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// MarketplaceSearchDetailResponse represents marketplace search detail.
pub struct MarketplaceSearchDetailResponse ("api.MarketplaceSearchDetailResponse", "MarketplaceSearchDetailResponse") {
    pub data: MarketplaceSearchDetail => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// MarketplaceSearchDetail represents marketplace search details.
pub struct MarketplaceSearchDetail ("api.MarketplaceSearchDetail", "MarketplaceSearchDetail") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: MarketplaceSearchDetailAttributes => "attributes";
}
}

json_struct! {
/// MarketplaceSearchDetailAttributes contains marketplace search detail attributes.
pub struct MarketplaceSearchDetailAttributes ("api.MarketplaceSearchDetailAttributes", "MarketplaceSearchDetailAttributes") {
    pub catalog_url: String => "catalogUrl", omitempty;
}
}

json_struct! {
/// MarketplaceSearchDetailCreateRequest represents a request to create marketplace search details.
pub struct MarketplaceSearchDetailCreateRequest ("api.MarketplaceSearchDetailCreateRequest", "MarketplaceSearchDetailCreateRequest") {
    pub data: MarketplaceSearchDetailCreateData => "data";
}
}

json_struct! {
/// MarketplaceSearchDetailCreateData contains the data for creating marketplace search details.
pub struct MarketplaceSearchDetailCreateData ("api.MarketplaceSearchDetailCreateData", "MarketplaceSearchDetailCreateData") {
    pub type_: String => "type";
    pub attributes: MarketplaceSearchDetailCreateAttributes => "attributes";
    pub relationships: MarketplaceSearchDetailCreateRelationships => "relationships";
}
}

json_struct! {
/// MarketplaceSearchDetailCreateAttributes contains attributes for creating marketplace search details.
pub struct MarketplaceSearchDetailCreateAttributes ("api.MarketplaceSearchDetailCreateAttributes", "MarketplaceSearchDetailCreateAttributes") {
    pub catalog_url: String => "catalogUrl";
}
}

json_struct! {
/// MarketplaceSearchDetailCreateRelationships contains relationships for creating marketplace search details.
pub struct MarketplaceSearchDetailCreateRelationships ("api.MarketplaceSearchDetailCreateRelationships", "MarketplaceSearchDetailCreateRelationships") {
    pub app: RelationshipData => "app";
}
}

json_struct! {
/// MarketplaceSearchDetailUpdateRequest represents a request to update marketplace search details.
pub struct MarketplaceSearchDetailUpdateRequest ("api.MarketplaceSearchDetailUpdateRequest", "MarketplaceSearchDetailUpdateRequest") {
    pub data: MarketplaceSearchDetailUpdateData => "data";
}
}

json_struct! {
/// MarketplaceSearchDetailUpdateData contains the data for updating marketplace search details.
pub struct MarketplaceSearchDetailUpdateData ("api.MarketplaceSearchDetailUpdateData", "MarketplaceSearchDetailUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: MarketplaceSearchDetailUpdateAttributes => "attributes";
}
}

json_struct! {
/// MarketplaceSearchDetailUpdateAttributes contains attributes for updating marketplace search details.
pub struct MarketplaceSearchDetailUpdateAttributes ("api.MarketplaceSearchDetailUpdateAttributes", "MarketplaceSearchDetailUpdateAttributes") {
    pub catalog_url: String => "catalogUrl", omitempty;
}
}

json_struct! {
pub struct PagingInformationPaging ("struct { Total int \"json:\\\"total\\\"\"; Limit int \"json:\\\"limit\\\"\" }", "") {
    pub total: i64 => "total";
    pub limit: i64 => "limit";
}
}
