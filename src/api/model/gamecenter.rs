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
/// GameCenterLeaderboardSetsResponse represents a list of leaderboard sets.
pub struct GameCenterLeaderboardSetsResponse ("api.GameCenterLeaderboardSetsResponse", "GameCenterLeaderboardSetsResponse") {
    pub data: JsonSlice<GameCenterLeaderboardSet> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetResponse represents a single leaderboard set.
pub struct GameCenterLeaderboardSetResponse ("api.GameCenterLeaderboardSetResponse", "GameCenterLeaderboardSetResponse") {
    pub data: GameCenterLeaderboardSet => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSet represents a Game Center leaderboard set: a
/// named grouping of leaderboards presented together in the Game Center UI.
pub struct GameCenterLeaderboardSet ("api.GameCenterLeaderboardSet", "GameCenterLeaderboardSet") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterLeaderboardSetAttributes => "attributes";
}
}

json_struct! {
/// GameCenterLeaderboardSetAttributes contains leaderboard set attributes.
pub struct GameCenterLeaderboardSetAttributes ("api.GameCenterLeaderboardSetAttributes", "GameCenterLeaderboardSetAttributes") {
    pub reference_name: String => "referenceName", omitempty;
    pub vendor_identifier: String => "vendorIdentifier", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetCreateRequest represents a request to create a
/// leaderboard set via the v2 Game Center API. The v2 API requires an
/// initial version resource, declared inline through the versions
/// relationship and the included array.
pub struct GameCenterLeaderboardSetCreateRequest ("api.GameCenterLeaderboardSetCreateRequest", "GameCenterLeaderboardSetCreateRequest") {
    pub data: GameCenterLeaderboardSetCreateData => "data";
    pub included: JsonSlice<GameCenterVersionInlineCreate> => "included", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetCreateData contains the data for creating a leaderboard set.
pub struct GameCenterLeaderboardSetCreateData ("api.GameCenterLeaderboardSetCreateData", "GameCenterLeaderboardSetCreateData") {
    pub type_: String => "type";
    pub attributes: GameCenterLeaderboardSetCreateAttributes => "attributes";
    pub relationships: GameCenterLeaderboardSetCreateRelationships => "relationships";
}
}

json_struct! {
/// GameCenterLeaderboardSetCreateAttributes contains attributes for creating a leaderboard set.
pub struct GameCenterLeaderboardSetCreateAttributes ("api.GameCenterLeaderboardSetCreateAttributes", "GameCenterLeaderboardSetCreateAttributes") {
    pub reference_name: String => "referenceName";
    pub vendor_identifier: String => "vendorIdentifier";
}
}

json_struct! {
/// GameCenterLeaderboardSetCreateRelationships contains relationships for
/// creating a leaderboard set. A set belongs to either an app's Game
/// Center detail or a Game Center group, never both.
pub struct GameCenterLeaderboardSetCreateRelationships ("api.GameCenterLeaderboardSetCreateRelationships", "GameCenterLeaderboardSetCreateRelationships") {
    pub game_center_detail: Option<RelationshipData> => "gameCenterDetail", omitempty;
    pub game_center_group: Option<RelationshipData> => "gameCenterGroup", omitempty;
    pub versions: RelationshipDataList => "versions";
}
}

json_struct! {
/// GameCenterLeaderboardSetUpdateRequest represents a request to update a leaderboard set.
pub struct GameCenterLeaderboardSetUpdateRequest ("api.GameCenterLeaderboardSetUpdateRequest", "GameCenterLeaderboardSetUpdateRequest") {
    pub data: GameCenterLeaderboardSetUpdateData => "data";
}
}

json_struct! {
/// GameCenterLeaderboardSetUpdateData contains the data for updating a leaderboard set.
pub struct GameCenterLeaderboardSetUpdateData ("api.GameCenterLeaderboardSetUpdateData", "GameCenterLeaderboardSetUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterLeaderboardSetUpdateAttributes => "attributes";
}
}

json_struct! {
/// GameCenterLeaderboardSetUpdateAttributes contains attributes for updating a leaderboard set.
/// The vendor identifier is immutable once the set exists.
pub struct GameCenterLeaderboardSetUpdateAttributes ("api.GameCenterLeaderboardSetUpdateAttributes", "GameCenterLeaderboardSetUpdateAttributes") {
    pub reference_name: String => "referenceName", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetVersionsResponse represents a list of leaderboard set versions.
pub struct GameCenterLeaderboardSetVersionsResponse ("api.GameCenterLeaderboardSetVersionsResponse", "GameCenterLeaderboardSetVersionsResponse") {
    pub data: JsonSlice<GameCenterLeaderboardSetVersion> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetVersionResponse represents a single leaderboard set version.
pub struct GameCenterLeaderboardSetVersionResponse ("api.GameCenterLeaderboardSetVersionResponse", "GameCenterLeaderboardSetVersionResponse") {
    pub data: GameCenterLeaderboardSetVersion => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetVersion represents one editable revision of a
/// leaderboard set. Localizations hang off the version, and the version's
/// state tracks it through App Review.
pub struct GameCenterLeaderboardSetVersion ("api.GameCenterLeaderboardSetVersion", "GameCenterLeaderboardSetVersion") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterLeaderboardSetVersionAttributes => "attributes";
}
}

json_struct! {
/// GameCenterLeaderboardSetVersionAttributes contains leaderboard set version attributes.
pub struct GameCenterLeaderboardSetVersionAttributes ("api.GameCenterLeaderboardSetVersionAttributes", "GameCenterLeaderboardSetVersionAttributes") {
    pub version: i64 => "version", omitempty;
    pub state: String => "state", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetVersionCreateRequest represents a request to open
/// a new editable version of a leaderboard set.
pub struct GameCenterLeaderboardSetVersionCreateRequest ("api.GameCenterLeaderboardSetVersionCreateRequest", "GameCenterLeaderboardSetVersionCreateRequest") {
    pub data: GameCenterLeaderboardSetVersionCreateData => "data";
}
}

json_struct! {
/// GameCenterLeaderboardSetVersionCreateData contains the data for creating a leaderboard set version.
pub struct GameCenterLeaderboardSetVersionCreateData ("api.GameCenterLeaderboardSetVersionCreateData", "GameCenterLeaderboardSetVersionCreateData") {
    pub type_: String => "type";
    pub relationships: GameCenterLeaderboardSetVersionCreateRelationships => "relationships";
}
}

json_struct! {
/// GameCenterLeaderboardSetVersionCreateRelationships contains relationships
/// for creating a leaderboard set version.
pub struct GameCenterLeaderboardSetVersionCreateRelationships ("api.GameCenterLeaderboardSetVersionCreateRelationships", "GameCenterLeaderboardSetVersionCreateRelationships") {
    pub leaderboard_set: RelationshipData => "leaderboardSet";
}
}

json_struct! {
/// GameCenterLeaderboardSetLocalizationsResponse represents a list of leaderboard set localizations.
pub struct GameCenterLeaderboardSetLocalizationsResponse ("api.GameCenterLeaderboardSetLocalizationsResponse", "GameCenterLeaderboardSetLocalizationsResponse") {
    pub data: JsonSlice<GameCenterLeaderboardSetLocalization> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetLocalizationResponse represents a single leaderboard set localization.
pub struct GameCenterLeaderboardSetLocalizationResponse ("api.GameCenterLeaderboardSetLocalizationResponse", "GameCenterLeaderboardSetLocalizationResponse") {
    pub data: GameCenterLeaderboardSetLocalization => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetLocalization represents the localized name of a
/// leaderboard set in one locale.
pub struct GameCenterLeaderboardSetLocalization ("api.GameCenterLeaderboardSetLocalization", "GameCenterLeaderboardSetLocalization") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterLeaderboardSetLocalizationAttributes => "attributes";
}
}

json_struct! {
/// GameCenterLeaderboardSetLocalizationAttributes contains leaderboard set localization attributes.
pub struct GameCenterLeaderboardSetLocalizationAttributes ("api.GameCenterLeaderboardSetLocalizationAttributes", "GameCenterLeaderboardSetLocalizationAttributes") {
    pub locale: String => "locale", omitempty;
    pub name: String => "name", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetLocalizationCreateRequest represents a request to
/// create a leaderboard set localization against a version.
pub struct GameCenterLeaderboardSetLocalizationCreateRequest ("api.GameCenterLeaderboardSetLocalizationCreateRequest", "GameCenterLeaderboardSetLocalizationCreateRequest") {
    pub data: GameCenterLeaderboardSetLocalizationCreateData => "data";
}
}

json_struct! {
/// GameCenterLeaderboardSetLocalizationCreateData contains the data for creating a localization.
pub struct GameCenterLeaderboardSetLocalizationCreateData ("api.GameCenterLeaderboardSetLocalizationCreateData", "GameCenterLeaderboardSetLocalizationCreateData") {
    pub type_: String => "type";
    pub attributes: GameCenterLeaderboardSetLocalizationCreateAttributes => "attributes";
    pub relationships: GameCenterLeaderboardSetLocalizationCreateRelationships => "relationships";
}
}

json_struct! {
/// GameCenterLeaderboardSetLocalizationCreateAttributes contains attributes for creating a localization.
pub struct GameCenterLeaderboardSetLocalizationCreateAttributes ("api.GameCenterLeaderboardSetLocalizationCreateAttributes", "GameCenterLeaderboardSetLocalizationCreateAttributes") {
    pub locale: String => "locale";
    pub name: String => "name";
}
}

json_struct! {
/// GameCenterLeaderboardSetLocalizationCreateRelationships contains relationships for creating a localization.
pub struct GameCenterLeaderboardSetLocalizationCreateRelationships ("api.GameCenterLeaderboardSetLocalizationCreateRelationships", "GameCenterLeaderboardSetLocalizationCreateRelationships") {
    pub version: RelationshipData => "version";
}
}

json_struct! {
/// GameCenterLeaderboardSetLocalizationUpdateRequest represents a request to update a localization.
pub struct GameCenterLeaderboardSetLocalizationUpdateRequest ("api.GameCenterLeaderboardSetLocalizationUpdateRequest", "GameCenterLeaderboardSetLocalizationUpdateRequest") {
    pub data: GameCenterLeaderboardSetLocalizationUpdateData => "data";
}
}

json_struct! {
/// GameCenterLeaderboardSetLocalizationUpdateData contains the data for updating a localization.
pub struct GameCenterLeaderboardSetLocalizationUpdateData ("api.GameCenterLeaderboardSetLocalizationUpdateData", "GameCenterLeaderboardSetLocalizationUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterLeaderboardSetLocalizationUpdateAttributes => "attributes";
}
}

json_struct! {
/// GameCenterLeaderboardSetLocalizationUpdateAttributes contains attributes for updating a localization.
pub struct GameCenterLeaderboardSetLocalizationUpdateAttributes ("api.GameCenterLeaderboardSetLocalizationUpdateAttributes", "GameCenterLeaderboardSetLocalizationUpdateAttributes") {
    pub name: String => "name", omitempty;
}
}

json_struct! {
/// GameCenterImageResponse represents a single Game Center image.
pub struct GameCenterImageResponse ("api.GameCenterImageResponse", "GameCenterImageResponse") {
    pub data: GameCenterImage => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterImage represents an uploaded Game Center image asset.
pub struct GameCenterImage ("api.GameCenterImage", "GameCenterImage") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterImageAttributes => "attributes";
}
}

json_struct! {
/// GameCenterImageAttributes contains Game Center image attributes.
pub struct GameCenterImageAttributes ("api.GameCenterImageAttributes", "GameCenterImageAttributes") {
    pub file_size: i64 => "fileSize", omitempty;
    pub file_name: String => "fileName", omitempty;
    pub image_asset: Option<ImageAsset> => "imageAsset", omitempty;
    pub upload_operations: JsonSlice<UploadOperation> => "uploadOperations", omitempty;
    pub asset_delivery_state: Option<AssetDeliveryState> => "assetDeliveryState", omitempty;
}
}

json_struct! {
/// GameCenterImageCreateRequest reserves a Game Center image upload. The
/// resource type and the relationship key differ per content type, so the
/// caller supplies both.
pub struct GameCenterImageCreateRequest ("api.GameCenterImageCreateRequest", "GameCenterImageCreateRequest") {
    pub data: GameCenterImageCreateData => "data";
}
}

json_struct! {
/// GameCenterImageCreateData contains the data for reserving an image upload.
pub struct GameCenterImageCreateData ("api.GameCenterImageCreateData", "GameCenterImageCreateData") {
    pub type_: String => "type";
    pub attributes: GameCenterImageCreateAttrs => "attributes";
    pub relationships: JsonMap<Option<RelationshipData>> => "relationships", omitempty;
}
}

json_struct! {
/// GameCenterImageCreateAttrs contains attributes for reserving an image upload.
pub struct GameCenterImageCreateAttrs ("api.GameCenterImageCreateAttrs", "GameCenterImageCreateAttrs") {
    pub file_name: String => "fileName";
    pub file_size: i64 => "fileSize";
}
}

json_struct! {
/// GameCenterImageUpdateRequest commits a reserved Game Center image.
pub struct GameCenterImageUpdateRequest ("api.GameCenterImageUpdateRequest", "GameCenterImageUpdateRequest") {
    pub data: GameCenterImageUpdateData => "data";
}
}

json_struct! {
/// GameCenterImageUpdateData contains the data for committing an image upload.
pub struct GameCenterImageUpdateData ("api.GameCenterImageUpdateData", "GameCenterImageUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterImageUpdateAttrs => "attributes";
}
}

json_struct! {
/// GameCenterImageUpdateAttrs contains attributes for committing an image upload.
pub struct GameCenterImageUpdateAttrs ("api.GameCenterImageUpdateAttrs", "GameCenterImageUpdateAttrs") {
    pub uploaded: Option<bool> => "uploaded", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetMemberLocalizationsResponse represents a list of member localizations.
pub struct GameCenterLeaderboardSetMemberLocalizationsResponse ("api.GameCenterLeaderboardSetMemberLocalizationsResponse", "GameCenterLeaderboardSetMemberLocalizationsResponse") {
    pub data: JsonSlice<GameCenterLeaderboardSetMemberLocalization> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetMemberLocalizationResponse represents a single member localization.
pub struct GameCenterLeaderboardSetMemberLocalizationResponse ("api.GameCenterLeaderboardSetMemberLocalizationResponse", "GameCenterLeaderboardSetMemberLocalizationResponse") {
    pub data: GameCenterLeaderboardSetMemberLocalization => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetMemberLocalization names one leaderboard as it
/// appears inside a specific leaderboard set, which can differ from the
/// leaderboard's own localized name.
pub struct GameCenterLeaderboardSetMemberLocalization ("api.GameCenterLeaderboardSetMemberLocalization", "GameCenterLeaderboardSetMemberLocalization") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterLeaderboardSetMemberLocalizationAttributes => "attributes";
}
}

json_struct! {
/// GameCenterLeaderboardSetMemberLocalizationAttributes contains member localization attributes.
pub struct GameCenterLeaderboardSetMemberLocalizationAttributes ("api.GameCenterLeaderboardSetMemberLocalizationAttributes", "GameCenterLeaderboardSetMemberLocalizationAttributes") {
    pub locale: String => "locale", omitempty;
    pub name: String => "name", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetMemberLocalizationCreateRequest represents a request
/// to name a leaderboard within a leaderboard set for one locale.
pub struct GameCenterLeaderboardSetMemberLocalizationCreateRequest ("api.GameCenterLeaderboardSetMemberLocalizationCreateRequest", "GameCenterLeaderboardSetMemberLocalizationCreateRequest") {
    pub data: GameCenterLeaderboardSetMemberLocalizationCreateData => "data";
}
}

json_struct! {
/// GameCenterLeaderboardSetMemberLocalizationCreateData contains the data for creating a member localization.
pub struct GameCenterLeaderboardSetMemberLocalizationCreateData ("api.GameCenterLeaderboardSetMemberLocalizationCreateData", "GameCenterLeaderboardSetMemberLocalizationCreateData") {
    pub type_: String => "type";
    pub attributes: GameCenterLeaderboardSetMemberLocalizationCreateAttributes => "attributes";
    pub relationships: GameCenterLeaderboardSetMemberLocalizationCreateRelationships => "relationships";
}
}

json_struct! {
/// GameCenterLeaderboardSetMemberLocalizationCreateAttributes contains attributes for creating a member localization.
pub struct GameCenterLeaderboardSetMemberLocalizationCreateAttributes ("api.GameCenterLeaderboardSetMemberLocalizationCreateAttributes", "GameCenterLeaderboardSetMemberLocalizationCreateAttributes") {
    pub locale: String => "locale", omitempty;
    pub name: String => "name", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardSetMemberLocalizationCreateRelationships contains relationships for creating a member localization.
pub struct GameCenterLeaderboardSetMemberLocalizationCreateRelationships ("api.GameCenterLeaderboardSetMemberLocalizationCreateRelationships", "GameCenterLeaderboardSetMemberLocalizationCreateRelationships") {
    pub game_center_leaderboard_set: RelationshipData => "gameCenterLeaderboardSet";
    pub game_center_leaderboard: RelationshipData => "gameCenterLeaderboard";
}
}

json_struct! {
/// GameCenterLeaderboardSetMemberLocalizationUpdateRequest represents a request to update a member localization.
pub struct GameCenterLeaderboardSetMemberLocalizationUpdateRequest ("api.GameCenterLeaderboardSetMemberLocalizationUpdateRequest", "GameCenterLeaderboardSetMemberLocalizationUpdateRequest") {
    pub data: GameCenterLeaderboardSetMemberLocalizationUpdateData => "data";
}
}

json_struct! {
/// GameCenterLeaderboardSetMemberLocalizationUpdateData contains the data for updating a member localization.
pub struct GameCenterLeaderboardSetMemberLocalizationUpdateData ("api.GameCenterLeaderboardSetMemberLocalizationUpdateData", "GameCenterLeaderboardSetMemberLocalizationUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterLeaderboardSetMemberLocalizationUpdateAttributes => "attributes";
}
}

json_struct! {
/// GameCenterLeaderboardSetMemberLocalizationUpdateAttributes contains attributes for updating a member localization.
pub struct GameCenterLeaderboardSetMemberLocalizationUpdateAttributes ("api.GameCenterLeaderboardSetMemberLocalizationUpdateAttributes", "GameCenterLeaderboardSetMemberLocalizationUpdateAttributes") {
    pub name: String => "name", omitempty;
}
}

json_struct! {
/// GameCenterActivitiesResponse represents a list of activities.
pub struct GameCenterActivitiesResponse ("api.GameCenterActivitiesResponse", "GameCenterActivitiesResponse") {
    pub data: JsonSlice<GameCenterActivity> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterActivityResponse represents a single activity.
pub struct GameCenterActivityResponse ("api.GameCenterActivityResponse", "GameCenterActivityResponse") {
    pub data: GameCenterActivity => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterActivity represents a Game Center activity: a multiplayer or
/// solo play mode surfaced in the Game Center UI.
pub struct GameCenterActivity ("api.GameCenterActivity", "GameCenterActivity") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterActivityAttributes => "attributes";
}
}

json_struct! {
/// GameCenterActivityAttributes contains activity attributes. Properties is
/// an arbitrary string map passed through to the game at launch.
pub struct GameCenterActivityAttributes ("api.GameCenterActivityAttributes", "GameCenterActivityAttributes") {
    pub reference_name: String => "referenceName", omitempty;
    pub vendor_identifier: String => "vendorIdentifier", omitempty;
    pub play_style: String => "playStyle", omitempty;
    pub minimum_players_count: i64 => "minimumPlayersCount", omitempty;
    pub maximum_players_count: i64 => "maximumPlayersCount", omitempty;
    pub supports_party_code: bool => "supportsPartyCode", omitempty;
    pub archived: bool => "archived", omitempty;
    pub properties: JsonMap<String> => "properties", omitempty;
}
}

json_struct! {
/// GameCenterActivityCreateRequest represents a request to create an
/// activity. The initial version is declared inline through the versions
/// relationship and the included array; the inline version is the only
/// place a fallback URL can be set at creation time.
pub struct GameCenterActivityCreateRequest ("api.GameCenterActivityCreateRequest", "GameCenterActivityCreateRequest") {
    pub data: GameCenterActivityCreateData => "data";
    pub included: JsonSlice<GameCenterActivityVersionInlineCreate> => "included", omitempty;
}
}

json_struct! {
/// GameCenterActivityCreateData contains the data for creating an activity.
pub struct GameCenterActivityCreateData ("api.GameCenterActivityCreateData", "GameCenterActivityCreateData") {
    pub type_: String => "type";
    pub attributes: GameCenterActivityCreateAttributes => "attributes";
    pub relationships: GameCenterActivityCreateRelationships => "relationships";
}
}

json_struct! {
/// GameCenterActivityCreateAttributes contains attributes for creating an activity.
pub struct GameCenterActivityCreateAttributes ("api.GameCenterActivityCreateAttributes", "GameCenterActivityCreateAttributes") {
    pub reference_name: String => "referenceName";
    pub vendor_identifier: String => "vendorIdentifier";
    pub play_style: String => "playStyle", omitempty;
    pub minimum_players_count: Option<i64> => "minimumPlayersCount", omitempty;
    pub maximum_players_count: Option<i64> => "maximumPlayersCount", omitempty;
    pub supports_party_code: Option<bool> => "supportsPartyCode", omitempty;
    pub properties: JsonMap<String> => "properties", omitempty;
}
}

json_struct! {
/// GameCenterActivityCreateRelationships contains relationships for creating
/// an activity. An activity belongs to either an app's Game Center detail
/// or a Game Center group, never both.
pub struct GameCenterActivityCreateRelationships ("api.GameCenterActivityCreateRelationships", "GameCenterActivityCreateRelationships") {
    pub game_center_detail: Option<RelationshipData> => "gameCenterDetail", omitempty;
    pub game_center_group: Option<RelationshipData> => "gameCenterGroup", omitempty;
    pub versions: Option<RelationshipDataList> => "versions", omitempty;
}
}

json_struct! {
/// GameCenterActivityVersionInlineCreate declares the activity version
/// created inline with an activity create request. The ID is a
/// client-chosen temporary identifier referenced from the versions
/// relationship.
pub struct GameCenterActivityVersionInlineCreate ("api.GameCenterActivityVersionInlineCreate", "GameCenterActivityVersionInlineCreate") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: Option<GameCenterActivityVersionInlineCreateAttributes> => "attributes", omitempty;
}
}

json_struct! {
/// GameCenterActivityVersionInlineCreateAttributes contains the attributes
/// settable on an inline activity version.
pub struct GameCenterActivityVersionInlineCreateAttributes ("api.GameCenterActivityVersionInlineCreateAttributes", "GameCenterActivityVersionInlineCreateAttributes") {
    pub fallback_url: String => "fallbackUrl", omitempty;
}
}

json_struct! {
/// GameCenterActivityUpdateRequest represents a request to update an activity.
pub struct GameCenterActivityUpdateRequest ("api.GameCenterActivityUpdateRequest", "GameCenterActivityUpdateRequest") {
    pub data: GameCenterActivityUpdateData => "data";
}
}

json_struct! {
/// GameCenterActivityUpdateData contains the data for updating an activity.
pub struct GameCenterActivityUpdateData ("api.GameCenterActivityUpdateData", "GameCenterActivityUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterActivityUpdateAttributes => "attributes";
}
}

json_struct! {
/// GameCenterActivityUpdateAttributes contains attributes for updating an activity.
pub struct GameCenterActivityUpdateAttributes ("api.GameCenterActivityUpdateAttributes", "GameCenterActivityUpdateAttributes") {
    pub reference_name: String => "referenceName", omitempty;
    pub play_style: String => "playStyle", omitempty;
    pub minimum_players_count: Option<i64> => "minimumPlayersCount", omitempty;
    pub maximum_players_count: Option<i64> => "maximumPlayersCount", omitempty;
    pub supports_party_code: Option<bool> => "supportsPartyCode", omitempty;
    pub archived: Option<bool> => "archived", omitempty;
    pub properties: JsonMap<String> => "properties", omitempty;
}
}

json_struct! {
/// GameCenterActivityVersionsResponse represents a list of activity versions.
pub struct GameCenterActivityVersionsResponse ("api.GameCenterActivityVersionsResponse", "GameCenterActivityVersionsResponse") {
    pub data: JsonSlice<GameCenterActivityVersion> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterActivityVersionResponse represents a single activity version.
pub struct GameCenterActivityVersionResponse ("api.GameCenterActivityVersionResponse", "GameCenterActivityVersionResponse") {
    pub data: GameCenterActivityVersion => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterActivityVersion represents one editable revision of an activity.
pub struct GameCenterActivityVersion ("api.GameCenterActivityVersion", "GameCenterActivityVersion") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterActivityVersionAttributes => "attributes";
}
}

json_struct! {
/// GameCenterActivityVersionAttributes contains activity version attributes.
/// FallbackURL is opened when the player's device cannot launch the activity.
pub struct GameCenterActivityVersionAttributes ("api.GameCenterActivityVersionAttributes", "GameCenterActivityVersionAttributes") {
    pub version: i64 => "version", omitempty;
    pub state: String => "state", omitempty;
    pub fallback_url: String => "fallbackUrl", omitempty;
}
}

json_struct! {
/// GameCenterActivityVersionCreateRequest represents a request to open a new
/// editable version of an activity.
pub struct GameCenterActivityVersionCreateRequest ("api.GameCenterActivityVersionCreateRequest", "GameCenterActivityVersionCreateRequest") {
    pub data: GameCenterActivityVersionCreateData => "data";
}
}

json_struct! {
/// GameCenterActivityVersionCreateData contains the data for creating an activity version.
pub struct GameCenterActivityVersionCreateData ("api.GameCenterActivityVersionCreateData", "GameCenterActivityVersionCreateData") {
    pub type_: String => "type";
    pub attributes: Option<GameCenterActivityVersionCreateAttributes> => "attributes", omitempty;
    pub relationships: GameCenterActivityVersionCreateRelationships => "relationships";
}
}

json_struct! {
/// GameCenterActivityVersionCreateAttributes contains attributes for creating an activity version.
pub struct GameCenterActivityVersionCreateAttributes ("api.GameCenterActivityVersionCreateAttributes", "GameCenterActivityVersionCreateAttributes") {
    pub fallback_url: String => "fallbackUrl", omitempty;
}
}

json_struct! {
/// GameCenterActivityVersionCreateRelationships contains relationships for creating an activity version.
pub struct GameCenterActivityVersionCreateRelationships ("api.GameCenterActivityVersionCreateRelationships", "GameCenterActivityVersionCreateRelationships") {
    pub activity: RelationshipData => "activity";
}
}

json_struct! {
/// GameCenterActivityVersionUpdateRequest represents a request to update an activity version.
pub struct GameCenterActivityVersionUpdateRequest ("api.GameCenterActivityVersionUpdateRequest", "GameCenterActivityVersionUpdateRequest") {
    pub data: GameCenterActivityVersionUpdateData => "data";
}
}

json_struct! {
/// GameCenterActivityVersionUpdateData contains the data for updating an activity version.
pub struct GameCenterActivityVersionUpdateData ("api.GameCenterActivityVersionUpdateData", "GameCenterActivityVersionUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterActivityVersionUpdateAttributes => "attributes";
}
}

json_struct! {
/// GameCenterActivityVersionUpdateAttributes contains attributes for updating an activity version.
pub struct GameCenterActivityVersionUpdateAttributes ("api.GameCenterActivityVersionUpdateAttributes", "GameCenterActivityVersionUpdateAttributes") {
    pub fallback_url: String => "fallbackUrl", omitempty;
}
}

json_struct! {
/// GameCenterActivityLocalizationsResponse represents a list of activity localizations.
pub struct GameCenterActivityLocalizationsResponse ("api.GameCenterActivityLocalizationsResponse", "GameCenterActivityLocalizationsResponse") {
    pub data: JsonSlice<GameCenterActivityLocalization> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterActivityLocalizationResponse represents a single activity localization.
pub struct GameCenterActivityLocalizationResponse ("api.GameCenterActivityLocalizationResponse", "GameCenterActivityLocalizationResponse") {
    pub data: GameCenterActivityLocalization => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterActivityLocalization represents the localized name and
/// description of an activity in one locale.
pub struct GameCenterActivityLocalization ("api.GameCenterActivityLocalization", "GameCenterActivityLocalization") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterActivityLocalizationAttributes => "attributes";
}
}

json_struct! {
/// GameCenterActivityLocalizationAttributes contains activity localization attributes.
pub struct GameCenterActivityLocalizationAttributes ("api.GameCenterActivityLocalizationAttributes", "GameCenterActivityLocalizationAttributes") {
    pub locale: String => "locale", omitempty;
    pub name: String => "name", omitempty;
    pub description: String => "description", omitempty;
}
}

json_struct! {
/// GameCenterActivityLocalizationCreateRequest represents a request to create
/// an activity localization against a version.
pub struct GameCenterActivityLocalizationCreateRequest ("api.GameCenterActivityLocalizationCreateRequest", "GameCenterActivityLocalizationCreateRequest") {
    pub data: GameCenterActivityLocalizationCreateData => "data";
}
}

json_struct! {
/// GameCenterActivityLocalizationCreateData contains the data for creating an activity localization.
pub struct GameCenterActivityLocalizationCreateData ("api.GameCenterActivityLocalizationCreateData", "GameCenterActivityLocalizationCreateData") {
    pub type_: String => "type";
    pub attributes: GameCenterActivityLocalizationCreateAttributes => "attributes";
    pub relationships: GameCenterActivityLocalizationCreateRelationships => "relationships";
}
}

json_struct! {
/// GameCenterActivityLocalizationCreateAttributes contains attributes for creating an activity localization.
pub struct GameCenterActivityLocalizationCreateAttributes ("api.GameCenterActivityLocalizationCreateAttributes", "GameCenterActivityLocalizationCreateAttributes") {
    pub locale: String => "locale";
    pub name: String => "name";
    pub description: String => "description", omitempty;
}
}

json_struct! {
/// GameCenterActivityLocalizationCreateRelationships contains relationships for creating an activity localization.
pub struct GameCenterActivityLocalizationCreateRelationships ("api.GameCenterActivityLocalizationCreateRelationships", "GameCenterActivityLocalizationCreateRelationships") {
    pub version: RelationshipData => "version";
}
}

json_struct! {
/// GameCenterActivityLocalizationUpdateRequest represents a request to update an activity localization.
pub struct GameCenterActivityLocalizationUpdateRequest ("api.GameCenterActivityLocalizationUpdateRequest", "GameCenterActivityLocalizationUpdateRequest") {
    pub data: GameCenterActivityLocalizationUpdateData => "data";
}
}

json_struct! {
/// GameCenterActivityLocalizationUpdateData contains the data for updating an activity localization.
pub struct GameCenterActivityLocalizationUpdateData ("api.GameCenterActivityLocalizationUpdateData", "GameCenterActivityLocalizationUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterActivityLocalizationUpdateAttributes => "attributes";
}
}

json_struct! {
/// GameCenterActivityLocalizationUpdateAttributes contains attributes for updating an activity localization.
pub struct GameCenterActivityLocalizationUpdateAttributes ("api.GameCenterActivityLocalizationUpdateAttributes", "GameCenterActivityLocalizationUpdateAttributes") {
    pub name: String => "name", omitempty;
    pub description: String => "description", omitempty;
}
}

json_struct! {
/// GameCenterChallengesResponse represents a list of challenges.
pub struct GameCenterChallengesResponse ("api.GameCenterChallengesResponse", "GameCenterChallengesResponse") {
    pub data: JsonSlice<GameCenterChallenge> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterChallengeResponse represents a single challenge.
pub struct GameCenterChallengeResponse ("api.GameCenterChallengeResponse", "GameCenterChallengeResponse") {
    pub data: GameCenterChallenge => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterChallenge represents a Game Center challenge: a competition
/// between players built on top of a leaderboard.
pub struct GameCenterChallenge ("api.GameCenterChallenge", "GameCenterChallenge") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterChallengeAttributes => "attributes";
}
}

json_struct! {
/// GameCenterChallengeAttributes contains challenge attributes. The
/// allowedDurations attribute was removed in App Store Connect API 4.1 and
/// is intentionally absent.
pub struct GameCenterChallengeAttributes ("api.GameCenterChallengeAttributes", "GameCenterChallengeAttributes") {
    pub reference_name: String => "referenceName", omitempty;
    pub vendor_identifier: String => "vendorIdentifier", omitempty;
    pub challenge_type: String => "challengeType", omitempty;
    pub repeatable: bool => "repeatable", omitempty;
    pub archived: bool => "archived", omitempty;
}
}

json_struct! {
/// GameCenterChallengeCreateRequest represents a request to create a
/// challenge. Like every versioned Game Center resource, the initial
/// version is declared inline through the versions relationship and the
/// included array.
pub struct GameCenterChallengeCreateRequest ("api.GameCenterChallengeCreateRequest", "GameCenterChallengeCreateRequest") {
    pub data: GameCenterChallengeCreateData => "data";
    pub included: JsonSlice<GameCenterVersionInlineCreate> => "included", omitempty;
}
}

json_struct! {
/// GameCenterChallengeCreateData contains the data for creating a challenge.
pub struct GameCenterChallengeCreateData ("api.GameCenterChallengeCreateData", "GameCenterChallengeCreateData") {
    pub type_: String => "type";
    pub attributes: GameCenterChallengeCreateAttributes => "attributes";
    pub relationships: GameCenterChallengeCreateRelationships => "relationships";
}
}

json_struct! {
/// GameCenterChallengeCreateAttributes contains attributes for creating a challenge.
pub struct GameCenterChallengeCreateAttributes ("api.GameCenterChallengeCreateAttributes", "GameCenterChallengeCreateAttributes") {
    pub reference_name: String => "referenceName";
    pub vendor_identifier: String => "vendorIdentifier";
    pub challenge_type: String => "challengeType";
    pub repeatable: Option<bool> => "repeatable", omitempty;
}
}

json_struct! {
/// GameCenterChallengeCreateRelationships contains relationships for creating
/// a challenge. LeaderboardV2 points at the v2 leaderboard the challenge
/// scores against; the deprecated v1 `leaderboard` key is not exposed.
pub struct GameCenterChallengeCreateRelationships ("api.GameCenterChallengeCreateRelationships", "GameCenterChallengeCreateRelationships") {
    pub game_center_detail: Option<RelationshipData> => "gameCenterDetail", omitempty;
    pub game_center_group: Option<RelationshipData> => "gameCenterGroup", omitempty;
    pub leaderboard_v2: Option<RelationshipData> => "leaderboardV2", omitempty;
    pub versions: Option<RelationshipDataList> => "versions", omitempty;
}
}

json_struct! {
/// GameCenterChallengeUpdateRequest represents a request to update a challenge.
pub struct GameCenterChallengeUpdateRequest ("api.GameCenterChallengeUpdateRequest", "GameCenterChallengeUpdateRequest") {
    pub data: GameCenterChallengeUpdateData => "data";
}
}

json_struct! {
/// GameCenterChallengeUpdateData contains the data for updating a challenge.
pub struct GameCenterChallengeUpdateData ("api.GameCenterChallengeUpdateData", "GameCenterChallengeUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterChallengeUpdateAttributes => "attributes";
    pub relationships: Option<GameCenterChallengeUpdateRelationships> => "relationships", omitempty;
}
}

json_struct! {
/// GameCenterChallengeUpdateAttributes contains attributes for updating a challenge.
pub struct GameCenterChallengeUpdateAttributes ("api.GameCenterChallengeUpdateAttributes", "GameCenterChallengeUpdateAttributes") {
    pub reference_name: String => "referenceName", omitempty;
    pub repeatable: Option<bool> => "repeatable", omitempty;
    pub archived: Option<bool> => "archived", omitempty;
}
}

json_struct! {
/// GameCenterChallengeUpdateRelationships contains relationships for updating a challenge.
pub struct GameCenterChallengeUpdateRelationships ("api.GameCenterChallengeUpdateRelationships", "GameCenterChallengeUpdateRelationships") {
    pub leaderboard_v2: Option<RelationshipData> => "leaderboardV2", omitempty;
}
}

json_struct! {
/// GameCenterChallengeVersionsResponse represents a list of challenge versions.
pub struct GameCenterChallengeVersionsResponse ("api.GameCenterChallengeVersionsResponse", "GameCenterChallengeVersionsResponse") {
    pub data: JsonSlice<GameCenterChallengeVersion> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterChallengeVersionResponse represents a single challenge version.
pub struct GameCenterChallengeVersionResponse ("api.GameCenterChallengeVersionResponse", "GameCenterChallengeVersionResponse") {
    pub data: GameCenterChallengeVersion => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterChallengeVersion represents one editable revision of a challenge.
pub struct GameCenterChallengeVersion ("api.GameCenterChallengeVersion", "GameCenterChallengeVersion") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterChallengeVersionAttributes => "attributes";
}
}

json_struct! {
/// GameCenterChallengeVersionAttributes contains challenge version attributes.
pub struct GameCenterChallengeVersionAttributes ("api.GameCenterChallengeVersionAttributes", "GameCenterChallengeVersionAttributes") {
    pub version: i64 => "version", omitempty;
    pub state: String => "state", omitempty;
}
}

json_struct! {
/// GameCenterChallengeVersionCreateRequest represents a request to open a new
/// editable version of a challenge.
pub struct GameCenterChallengeVersionCreateRequest ("api.GameCenterChallengeVersionCreateRequest", "GameCenterChallengeVersionCreateRequest") {
    pub data: GameCenterChallengeVersionCreateData => "data";
}
}

json_struct! {
/// GameCenterChallengeVersionCreateData contains the data for creating a challenge version.
pub struct GameCenterChallengeVersionCreateData ("api.GameCenterChallengeVersionCreateData", "GameCenterChallengeVersionCreateData") {
    pub type_: String => "type";
    pub relationships: GameCenterChallengeVersionCreateRelationships => "relationships";
}
}

json_struct! {
/// GameCenterChallengeVersionCreateRelationships contains relationships for creating a challenge version.
pub struct GameCenterChallengeVersionCreateRelationships ("api.GameCenterChallengeVersionCreateRelationships", "GameCenterChallengeVersionCreateRelationships") {
    pub challenge: RelationshipData => "challenge";
}
}

json_struct! {
/// GameCenterChallengeLocalizationsResponse represents a list of challenge localizations.
pub struct GameCenterChallengeLocalizationsResponse ("api.GameCenterChallengeLocalizationsResponse", "GameCenterChallengeLocalizationsResponse") {
    pub data: JsonSlice<GameCenterChallengeLocalization> => "data";
    pub links: PagedDocumentLinks => "links";
    pub meta: Option<PagingInformation> => "meta", omitempty;
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterChallengeLocalizationResponse represents a single challenge localization.
pub struct GameCenterChallengeLocalizationResponse ("api.GameCenterChallengeLocalizationResponse", "GameCenterChallengeLocalizationResponse") {
    pub data: GameCenterChallengeLocalization => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterChallengeLocalization represents the localized name and
/// description of a challenge in one locale.
pub struct GameCenterChallengeLocalization ("api.GameCenterChallengeLocalization", "GameCenterChallengeLocalization") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterChallengeLocalizationAttributes => "attributes";
}
}

json_struct! {
/// GameCenterChallengeLocalizationAttributes contains challenge localization attributes.
pub struct GameCenterChallengeLocalizationAttributes ("api.GameCenterChallengeLocalizationAttributes", "GameCenterChallengeLocalizationAttributes") {
    pub locale: String => "locale", omitempty;
    pub name: String => "name", omitempty;
    pub description: String => "description", omitempty;
}
}

json_struct! {
/// GameCenterChallengeLocalizationCreateRequest represents a request to create
/// a challenge localization against a version.
pub struct GameCenterChallengeLocalizationCreateRequest ("api.GameCenterChallengeLocalizationCreateRequest", "GameCenterChallengeLocalizationCreateRequest") {
    pub data: GameCenterChallengeLocalizationCreateData => "data";
}
}

json_struct! {
/// GameCenterChallengeLocalizationCreateData contains the data for creating a challenge localization.
pub struct GameCenterChallengeLocalizationCreateData ("api.GameCenterChallengeLocalizationCreateData", "GameCenterChallengeLocalizationCreateData") {
    pub type_: String => "type";
    pub attributes: GameCenterChallengeLocalizationCreateAttributes => "attributes";
    pub relationships: GameCenterChallengeLocalizationCreateRelationships => "relationships";
}
}

json_struct! {
/// GameCenterChallengeLocalizationCreateAttributes contains attributes for creating a challenge localization.
pub struct GameCenterChallengeLocalizationCreateAttributes ("api.GameCenterChallengeLocalizationCreateAttributes", "GameCenterChallengeLocalizationCreateAttributes") {
    pub locale: String => "locale";
    pub name: String => "name";
    pub description: String => "description", omitempty;
}
}

json_struct! {
/// GameCenterChallengeLocalizationCreateRelationships contains relationships for creating a challenge localization.
pub struct GameCenterChallengeLocalizationCreateRelationships ("api.GameCenterChallengeLocalizationCreateRelationships", "GameCenterChallengeLocalizationCreateRelationships") {
    pub version: RelationshipData => "version";
}
}

json_struct! {
/// GameCenterChallengeLocalizationUpdateRequest represents a request to update a challenge localization.
pub struct GameCenterChallengeLocalizationUpdateRequest ("api.GameCenterChallengeLocalizationUpdateRequest", "GameCenterChallengeLocalizationUpdateRequest") {
    pub data: GameCenterChallengeLocalizationUpdateData => "data";
}
}

json_struct! {
/// GameCenterChallengeLocalizationUpdateData contains the data for updating a challenge localization.
pub struct GameCenterChallengeLocalizationUpdateData ("api.GameCenterChallengeLocalizationUpdateData", "GameCenterChallengeLocalizationUpdateData") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterChallengeLocalizationUpdateAttributes => "attributes";
}
}

json_struct! {
/// GameCenterChallengeLocalizationUpdateAttributes contains attributes for updating a challenge localization.
pub struct GameCenterChallengeLocalizationUpdateAttributes ("api.GameCenterChallengeLocalizationUpdateAttributes", "GameCenterChallengeLocalizationUpdateAttributes") {
    pub name: String => "name", omitempty;
    pub description: String => "description", omitempty;
}
}

json_struct! {
/// GameCenterPlayerAchievementSubmissionResponse represents a submitted
/// achievement progress record.
pub struct GameCenterPlayerAchievementSubmissionResponse ("api.GameCenterPlayerAchievementSubmissionResponse", "GameCenterPlayerAchievementSubmissionResponse") {
    pub data: GameCenterPlayerAchievementSubmission => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterPlayerAchievementSubmission represents achievement progress a
/// server posted on a player's behalf.
pub struct GameCenterPlayerAchievementSubmission ("api.GameCenterPlayerAchievementSubmission", "GameCenterPlayerAchievementSubmission") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterPlayerAchievementSubmissionAttributes => "attributes";
}
}

json_struct! {
/// GameCenterPlayerAchievementSubmissionAttributes contains achievement
/// submission attributes. PreReleased marks progress earned in a build that
/// has not shipped to the App Store yet.
pub struct GameCenterPlayerAchievementSubmissionAttributes ("api.GameCenterPlayerAchievementSubmissionAttributes", "GameCenterPlayerAchievementSubmissionAttributes") {
    pub bundle_id: String => "bundleId", omitempty;
    pub challenge_ids: JsonSlice<String> => "challengeIds", omitempty;
    pub percentage_achieved: i64 => "percentageAchieved", omitempty;
    pub scoped_player_id: String => "scopedPlayerId", omitempty;
    pub submitted_date: String => "submittedDate", omitempty;
    pub vendor_identifier: String => "vendorIdentifier", omitempty;
    pub pre_released: bool => "preReleased", omitempty;
}
}

json_struct! {
/// GameCenterPlayerAchievementSubmissionCreateRequest represents a request to
/// submit achievement progress for a player.
pub struct GameCenterPlayerAchievementSubmissionCreateRequest ("api.GameCenterPlayerAchievementSubmissionCreateRequest", "GameCenterPlayerAchievementSubmissionCreateRequest") {
    pub data: GameCenterPlayerAchievementSubmissionCreateData => "data";
}
}

json_struct! {
/// GameCenterPlayerAchievementSubmissionCreateData contains the data for submitting achievement progress.
pub struct GameCenterPlayerAchievementSubmissionCreateData ("api.GameCenterPlayerAchievementSubmissionCreateData", "GameCenterPlayerAchievementSubmissionCreateData") {
    pub type_: String => "type";
    pub attributes: GameCenterPlayerAchievementSubmissionCreateAttributes => "attributes";
}
}

json_struct! {
/// GameCenterPlayerAchievementSubmissionCreateAttributes contains attributes
/// for submitting achievement progress.
pub struct GameCenterPlayerAchievementSubmissionCreateAttributes ("api.GameCenterPlayerAchievementSubmissionCreateAttributes", "GameCenterPlayerAchievementSubmissionCreateAttributes") {
    pub bundle_id: String => "bundleId";
    pub vendor_identifier: String => "vendorIdentifier";
    pub scoped_player_id: String => "scopedPlayerId";
    pub percentage_achieved: i64 => "percentageAchieved";
    pub challenge_ids: JsonSlice<String> => "challengeIds", omitempty;
    pub submitted_date: String => "submittedDate", omitempty;
    pub pre_released: Option<bool> => "preReleased", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardEntrySubmissionResponse represents a submitted
/// leaderboard score.
pub struct GameCenterLeaderboardEntrySubmissionResponse ("api.GameCenterLeaderboardEntrySubmissionResponse", "GameCenterLeaderboardEntrySubmissionResponse") {
    pub data: GameCenterLeaderboardEntrySubmission => "data";
    pub included: JsonSlice<Value> => "included", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardEntrySubmission represents a leaderboard score a
/// server posted on a player's behalf.
pub struct GameCenterLeaderboardEntrySubmission ("api.GameCenterLeaderboardEntrySubmission", "GameCenterLeaderboardEntrySubmission") {
    pub type_: String => "type";
    pub id: String => "id";
    pub attributes: GameCenterLeaderboardEntrySubmissionAttributes => "attributes";
}
}

json_struct! {
/// GameCenterLeaderboardEntrySubmissionAttributes contains leaderboard entry
/// submission attributes. Score and Context are decimal strings because the
/// API models them as arbitrary-precision numbers.
pub struct GameCenterLeaderboardEntrySubmissionAttributes ("api.GameCenterLeaderboardEntrySubmissionAttributes", "GameCenterLeaderboardEntrySubmissionAttributes") {
    pub bundle_id: String => "bundleId", omitempty;
    pub challenge_ids: JsonSlice<String> => "challengeIds", omitempty;
    pub context: String => "context", omitempty;
    pub scoped_player_id: String => "scopedPlayerId", omitempty;
    pub score: String => "score", omitempty;
    pub submitted_date: String => "submittedDate", omitempty;
    pub vendor_identifier: String => "vendorIdentifier", omitempty;
    pub pre_released: bool => "preReleased", omitempty;
}
}

json_struct! {
/// GameCenterLeaderboardEntrySubmissionCreateRequest represents a request to
/// submit a leaderboard score for a player.
pub struct GameCenterLeaderboardEntrySubmissionCreateRequest ("api.GameCenterLeaderboardEntrySubmissionCreateRequest", "GameCenterLeaderboardEntrySubmissionCreateRequest") {
    pub data: GameCenterLeaderboardEntrySubmissionCreateData => "data";
}
}

json_struct! {
/// GameCenterLeaderboardEntrySubmissionCreateData contains the data for submitting a score.
pub struct GameCenterLeaderboardEntrySubmissionCreateData ("api.GameCenterLeaderboardEntrySubmissionCreateData", "GameCenterLeaderboardEntrySubmissionCreateData") {
    pub type_: String => "type";
    pub attributes: GameCenterLeaderboardEntrySubmissionCreateAttributes => "attributes";
}
}

json_struct! {
/// GameCenterLeaderboardEntrySubmissionCreateAttributes contains attributes for submitting a score.
pub struct GameCenterLeaderboardEntrySubmissionCreateAttributes ("api.GameCenterLeaderboardEntrySubmissionCreateAttributes", "GameCenterLeaderboardEntrySubmissionCreateAttributes") {
    pub bundle_id: String => "bundleId";
    pub vendor_identifier: String => "vendorIdentifier";
    pub scoped_player_id: String => "scopedPlayerId";
    pub score: String => "score";
    pub context: String => "context", omitempty;
    pub challenge_ids: JsonSlice<String> => "challengeIds", omitempty;
    pub submitted_date: String => "submittedDate", omitempty;
    pub pre_released: Option<bool> => "preReleased", omitempty;
}
}
