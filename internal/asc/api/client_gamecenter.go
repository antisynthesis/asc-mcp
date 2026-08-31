package api

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
)

// Game Center content API methods (App Store Connect API 4.0-4.3).
//
// Leaderboard sets moved to the versioned v2 tree alongside achievements
// and leaderboards: a set owns versions, a version owns localizations,
// and a localization owns the set's image. Activities and challenges are
// the two content types introduced in 4.0; both follow the same
// resource/version/localization shape and both are created with an
// initial version declared inline. Player achievement and leaderboard
// entry submissions (4.3) let a server post progress on a player's
// behalf, including for pre-release builds.

// deleteWithBody performs a DELETE carrying a JSON:API linkages
// document. The shared Client.Delete helper sends no body, and removing
// leaderboards from a leaderboard set requires one.
func (c *Client) deleteWithBody(ctx context.Context, path string, body any) error {
	_, err := c.doRequest(ctx, http.MethodDelete, path, nil, body)
	return err
}

// Game Center leaderboard set types

// GameCenterLeaderboardSetsResponse represents a list of leaderboard sets.
type GameCenterLeaderboardSetsResponse struct {
	Data     []GameCenterLeaderboardSet `json:"data"`
	Links    PagedDocumentLinks         `json:"links"`
	Meta     *PagingInformation         `json:"meta,omitempty"`
	Included []any                      `json:"included,omitempty"`
}

// GameCenterLeaderboardSetResponse represents a single leaderboard set.
type GameCenterLeaderboardSetResponse struct {
	Data     GameCenterLeaderboardSet `json:"data"`
	Included []any                    `json:"included,omitempty"`
}

// GameCenterLeaderboardSet represents a Game Center leaderboard set: a
// named grouping of leaderboards presented together in the Game Center UI.
type GameCenterLeaderboardSet struct {
	Type       string                             `json:"type"`
	ID         string                             `json:"id"`
	Attributes GameCenterLeaderboardSetAttributes `json:"attributes"`
}

// GameCenterLeaderboardSetAttributes contains leaderboard set attributes.
type GameCenterLeaderboardSetAttributes struct {
	ReferenceName    string `json:"referenceName,omitempty"`
	VendorIdentifier string `json:"vendorIdentifier,omitempty"`
}

// GameCenterLeaderboardSetCreateRequest represents a request to create a
// leaderboard set via the v2 Game Center API. The v2 API requires an
// initial version resource, declared inline through the versions
// relationship and the included array.
type GameCenterLeaderboardSetCreateRequest struct {
	Data     GameCenterLeaderboardSetCreateData `json:"data"`
	Included []GameCenterVersionInlineCreate    `json:"included,omitempty"`
}

// GameCenterLeaderboardSetCreateData contains the data for creating a leaderboard set.
type GameCenterLeaderboardSetCreateData struct {
	Type          string                                      `json:"type"`
	Attributes    GameCenterLeaderboardSetCreateAttributes    `json:"attributes"`
	Relationships GameCenterLeaderboardSetCreateRelationships `json:"relationships"`
}

// GameCenterLeaderboardSetCreateAttributes contains attributes for creating a leaderboard set.
type GameCenterLeaderboardSetCreateAttributes struct {
	ReferenceName    string `json:"referenceName"`
	VendorIdentifier string `json:"vendorIdentifier"`
}

// GameCenterLeaderboardSetCreateRelationships contains relationships for
// creating a leaderboard set. A set belongs to either an app's Game
// Center detail or a Game Center group, never both.
type GameCenterLeaderboardSetCreateRelationships struct {
	GameCenterDetail *RelationshipData    `json:"gameCenterDetail,omitempty"`
	GameCenterGroup  *RelationshipData    `json:"gameCenterGroup,omitempty"`
	Versions         RelationshipDataList `json:"versions"`
}

// GameCenterLeaderboardSetUpdateRequest represents a request to update a leaderboard set.
type GameCenterLeaderboardSetUpdateRequest struct {
	Data GameCenterLeaderboardSetUpdateData `json:"data"`
}

// GameCenterLeaderboardSetUpdateData contains the data for updating a leaderboard set.
type GameCenterLeaderboardSetUpdateData struct {
	Type       string                                   `json:"type"`
	ID         string                                   `json:"id"`
	Attributes GameCenterLeaderboardSetUpdateAttributes `json:"attributes"`
}

// GameCenterLeaderboardSetUpdateAttributes contains attributes for updating a leaderboard set.
// The vendor identifier is immutable once the set exists.
type GameCenterLeaderboardSetUpdateAttributes struct {
	ReferenceName string `json:"referenceName,omitempty"`
}

// GameCenterLeaderboardSetVersionsResponse represents a list of leaderboard set versions.
type GameCenterLeaderboardSetVersionsResponse struct {
	Data     []GameCenterLeaderboardSetVersion `json:"data"`
	Links    PagedDocumentLinks                `json:"links"`
	Meta     *PagingInformation                `json:"meta,omitempty"`
	Included []any                             `json:"included,omitempty"`
}

// GameCenterLeaderboardSetVersionResponse represents a single leaderboard set version.
type GameCenterLeaderboardSetVersionResponse struct {
	Data     GameCenterLeaderboardSetVersion `json:"data"`
	Included []any                           `json:"included,omitempty"`
}

// GameCenterLeaderboardSetVersion represents one editable revision of a
// leaderboard set. Localizations hang off the version, and the version's
// state tracks it through App Review.
type GameCenterLeaderboardSetVersion struct {
	Type       string                                    `json:"type"`
	ID         string                                    `json:"id"`
	Attributes GameCenterLeaderboardSetVersionAttributes `json:"attributes"`
}

// GameCenterLeaderboardSetVersionAttributes contains leaderboard set version attributes.
type GameCenterLeaderboardSetVersionAttributes struct {
	Version int    `json:"version,omitempty"`
	State   string `json:"state,omitempty"`
}

// GameCenterLeaderboardSetVersionCreateRequest represents a request to open
// a new editable version of a leaderboard set.
type GameCenterLeaderboardSetVersionCreateRequest struct {
	Data GameCenterLeaderboardSetVersionCreateData `json:"data"`
}

// GameCenterLeaderboardSetVersionCreateData contains the data for creating a leaderboard set version.
type GameCenterLeaderboardSetVersionCreateData struct {
	Type          string                                             `json:"type"`
	Relationships GameCenterLeaderboardSetVersionCreateRelationships `json:"relationships"`
}

// GameCenterLeaderboardSetVersionCreateRelationships contains relationships
// for creating a leaderboard set version.
type GameCenterLeaderboardSetVersionCreateRelationships struct {
	LeaderboardSet RelationshipData `json:"leaderboardSet"`
}

// GameCenterLeaderboardSetLocalizationsResponse represents a list of leaderboard set localizations.
type GameCenterLeaderboardSetLocalizationsResponse struct {
	Data     []GameCenterLeaderboardSetLocalization `json:"data"`
	Links    PagedDocumentLinks                     `json:"links"`
	Meta     *PagingInformation                     `json:"meta,omitempty"`
	Included []any                                  `json:"included,omitempty"`
}

// GameCenterLeaderboardSetLocalizationResponse represents a single leaderboard set localization.
type GameCenterLeaderboardSetLocalizationResponse struct {
	Data     GameCenterLeaderboardSetLocalization `json:"data"`
	Included []any                                `json:"included,omitempty"`
}

// GameCenterLeaderboardSetLocalization represents the localized name of a
// leaderboard set in one locale.
type GameCenterLeaderboardSetLocalization struct {
	Type       string                                         `json:"type"`
	ID         string                                         `json:"id"`
	Attributes GameCenterLeaderboardSetLocalizationAttributes `json:"attributes"`
}

// GameCenterLeaderboardSetLocalizationAttributes contains leaderboard set localization attributes.
type GameCenterLeaderboardSetLocalizationAttributes struct {
	Locale string `json:"locale,omitempty"`
	Name   string `json:"name,omitempty"`
}

// GameCenterLeaderboardSetLocalizationCreateRequest represents a request to
// create a leaderboard set localization against a version.
type GameCenterLeaderboardSetLocalizationCreateRequest struct {
	Data GameCenterLeaderboardSetLocalizationCreateData `json:"data"`
}

// GameCenterLeaderboardSetLocalizationCreateData contains the data for creating a localization.
type GameCenterLeaderboardSetLocalizationCreateData struct {
	Type          string                                                  `json:"type"`
	Attributes    GameCenterLeaderboardSetLocalizationCreateAttributes    `json:"attributes"`
	Relationships GameCenterLeaderboardSetLocalizationCreateRelationships `json:"relationships"`
}

// GameCenterLeaderboardSetLocalizationCreateAttributes contains attributes for creating a localization.
type GameCenterLeaderboardSetLocalizationCreateAttributes struct {
	Locale string `json:"locale"`
	Name   string `json:"name"`
}

// GameCenterLeaderboardSetLocalizationCreateRelationships contains relationships for creating a localization.
type GameCenterLeaderboardSetLocalizationCreateRelationships struct {
	Version RelationshipData `json:"version"`
}

// GameCenterLeaderboardSetLocalizationUpdateRequest represents a request to update a localization.
type GameCenterLeaderboardSetLocalizationUpdateRequest struct {
	Data GameCenterLeaderboardSetLocalizationUpdateData `json:"data"`
}

// GameCenterLeaderboardSetLocalizationUpdateData contains the data for updating a localization.
type GameCenterLeaderboardSetLocalizationUpdateData struct {
	Type       string                                               `json:"type"`
	ID         string                                               `json:"id"`
	Attributes GameCenterLeaderboardSetLocalizationUpdateAttributes `json:"attributes"`
}

// GameCenterLeaderboardSetLocalizationUpdateAttributes contains attributes for updating a localization.
type GameCenterLeaderboardSetLocalizationUpdateAttributes struct {
	Name string `json:"name,omitempty"`
}

// Game Center image types
//
// Every Game Center image (leaderboard set, activity, challenge) is
// reserved with a file name and size, uploaded through the returned
// upload operations, and committed with uploaded=true. Unlike App Store
// screenshots, the commit carries no source file checksum.

// GameCenterImageResponse represents a single Game Center image.
type GameCenterImageResponse struct {
	Data     GameCenterImage `json:"data"`
	Included []any           `json:"included,omitempty"`
}

// GameCenterImage represents an uploaded Game Center image asset.
type GameCenterImage struct {
	Type       string                    `json:"type"`
	ID         string                    `json:"id"`
	Attributes GameCenterImageAttributes `json:"attributes"`
}

// GameCenterImageAttributes contains Game Center image attributes.
type GameCenterImageAttributes struct {
	FileSize           int                 `json:"fileSize,omitempty"`
	FileName           string              `json:"fileName,omitempty"`
	ImageAsset         *ImageAsset         `json:"imageAsset,omitempty"`
	UploadOperations   []UploadOperation   `json:"uploadOperations,omitempty"`
	AssetDeliveryState *AssetDeliveryState `json:"assetDeliveryState,omitempty"`
}

// GameCenterImageCreateRequest reserves a Game Center image upload. The
// resource type and the relationship key differ per content type, so the
// caller supplies both.
type GameCenterImageCreateRequest struct {
	Data GameCenterImageCreateData `json:"data"`
}

// GameCenterImageCreateData contains the data for reserving an image upload.
type GameCenterImageCreateData struct {
	Type          string                       `json:"type"`
	Attributes    GameCenterImageCreateAttrs   `json:"attributes"`
	Relationships map[string]*RelationshipData `json:"relationships,omitempty"`
}

// GameCenterImageCreateAttrs contains attributes for reserving an image upload.
type GameCenterImageCreateAttrs struct {
	FileName string `json:"fileName"`
	FileSize int    `json:"fileSize"`
}

// GameCenterImageUpdateRequest commits a reserved Game Center image.
type GameCenterImageUpdateRequest struct {
	Data GameCenterImageUpdateData `json:"data"`
}

// GameCenterImageUpdateData contains the data for committing an image upload.
type GameCenterImageUpdateData struct {
	Type       string                     `json:"type"`
	ID         string                     `json:"id"`
	Attributes GameCenterImageUpdateAttrs `json:"attributes"`
}

// GameCenterImageUpdateAttrs contains attributes for committing an image upload.
type GameCenterImageUpdateAttrs struct {
	Uploaded *bool `json:"uploaded,omitempty"`
}

// Game Center leaderboard set member localization types

// GameCenterLeaderboardSetMemberLocalizationsResponse represents a list of member localizations.
type GameCenterLeaderboardSetMemberLocalizationsResponse struct {
	Data     []GameCenterLeaderboardSetMemberLocalization `json:"data"`
	Links    PagedDocumentLinks                           `json:"links"`
	Meta     *PagingInformation                           `json:"meta,omitempty"`
	Included []any                                        `json:"included,omitempty"`
}

// GameCenterLeaderboardSetMemberLocalizationResponse represents a single member localization.
type GameCenterLeaderboardSetMemberLocalizationResponse struct {
	Data     GameCenterLeaderboardSetMemberLocalization `json:"data"`
	Included []any                                      `json:"included,omitempty"`
}

// GameCenterLeaderboardSetMemberLocalization names one leaderboard as it
// appears inside a specific leaderboard set, which can differ from the
// leaderboard's own localized name.
type GameCenterLeaderboardSetMemberLocalization struct {
	Type       string                                               `json:"type"`
	ID         string                                               `json:"id"`
	Attributes GameCenterLeaderboardSetMemberLocalizationAttributes `json:"attributes"`
}

// GameCenterLeaderboardSetMemberLocalizationAttributes contains member localization attributes.
type GameCenterLeaderboardSetMemberLocalizationAttributes struct {
	Locale string `json:"locale,omitempty"`
	Name   string `json:"name,omitempty"`
}

// GameCenterLeaderboardSetMemberLocalizationCreateRequest represents a request
// to name a leaderboard within a leaderboard set for one locale.
type GameCenterLeaderboardSetMemberLocalizationCreateRequest struct {
	Data GameCenterLeaderboardSetMemberLocalizationCreateData `json:"data"`
}

// GameCenterLeaderboardSetMemberLocalizationCreateData contains the data for creating a member localization.
type GameCenterLeaderboardSetMemberLocalizationCreateData struct {
	Type          string                                                        `json:"type"`
	Attributes    GameCenterLeaderboardSetMemberLocalizationCreateAttributes    `json:"attributes"`
	Relationships GameCenterLeaderboardSetMemberLocalizationCreateRelationships `json:"relationships"`
}

// GameCenterLeaderboardSetMemberLocalizationCreateAttributes contains attributes for creating a member localization.
type GameCenterLeaderboardSetMemberLocalizationCreateAttributes struct {
	Locale string `json:"locale,omitempty"`
	Name   string `json:"name,omitempty"`
}

// GameCenterLeaderboardSetMemberLocalizationCreateRelationships contains relationships for creating a member localization.
type GameCenterLeaderboardSetMemberLocalizationCreateRelationships struct {
	GameCenterLeaderboardSet RelationshipData `json:"gameCenterLeaderboardSet"`
	GameCenterLeaderboard    RelationshipData `json:"gameCenterLeaderboard"`
}

// GameCenterLeaderboardSetMemberLocalizationUpdateRequest represents a request to update a member localization.
type GameCenterLeaderboardSetMemberLocalizationUpdateRequest struct {
	Data GameCenterLeaderboardSetMemberLocalizationUpdateData `json:"data"`
}

// GameCenterLeaderboardSetMemberLocalizationUpdateData contains the data for updating a member localization.
type GameCenterLeaderboardSetMemberLocalizationUpdateData struct {
	Type       string                                                     `json:"type"`
	ID         string                                                     `json:"id"`
	Attributes GameCenterLeaderboardSetMemberLocalizationUpdateAttributes `json:"attributes"`
}

// GameCenterLeaderboardSetMemberLocalizationUpdateAttributes contains attributes for updating a member localization.
type GameCenterLeaderboardSetMemberLocalizationUpdateAttributes struct {
	Name string `json:"name,omitempty"`
}

// Game Center activity types

// GameCenterActivitiesResponse represents a list of activities.
type GameCenterActivitiesResponse struct {
	Data     []GameCenterActivity `json:"data"`
	Links    PagedDocumentLinks   `json:"links"`
	Meta     *PagingInformation   `json:"meta,omitempty"`
	Included []any                `json:"included,omitempty"`
}

// GameCenterActivityResponse represents a single activity.
type GameCenterActivityResponse struct {
	Data     GameCenterActivity `json:"data"`
	Included []any              `json:"included,omitempty"`
}

// GameCenterActivity represents a Game Center activity: a multiplayer or
// solo play mode surfaced in the Game Center UI.
type GameCenterActivity struct {
	Type       string                       `json:"type"`
	ID         string                       `json:"id"`
	Attributes GameCenterActivityAttributes `json:"attributes"`
}

// GameCenterActivityAttributes contains activity attributes. Properties is
// an arbitrary string map passed through to the game at launch.
type GameCenterActivityAttributes struct {
	ReferenceName       string            `json:"referenceName,omitempty"`
	VendorIdentifier    string            `json:"vendorIdentifier,omitempty"`
	PlayStyle           string            `json:"playStyle,omitempty"`
	MinimumPlayersCount int               `json:"minimumPlayersCount,omitempty"`
	MaximumPlayersCount int               `json:"maximumPlayersCount,omitempty"`
	SupportsPartyCode   bool              `json:"supportsPartyCode,omitempty"`
	Archived            bool              `json:"archived,omitempty"`
	Properties          map[string]string `json:"properties,omitempty"`
}

// GameCenterActivityCreateRequest represents a request to create an
// activity. The initial version is declared inline through the versions
// relationship and the included array; the inline version is the only
// place a fallback URL can be set at creation time.
type GameCenterActivityCreateRequest struct {
	Data     GameCenterActivityCreateData            `json:"data"`
	Included []GameCenterActivityVersionInlineCreate `json:"included,omitempty"`
}

// GameCenterActivityCreateData contains the data for creating an activity.
type GameCenterActivityCreateData struct {
	Type          string                                `json:"type"`
	Attributes    GameCenterActivityCreateAttributes    `json:"attributes"`
	Relationships GameCenterActivityCreateRelationships `json:"relationships"`
}

// GameCenterActivityCreateAttributes contains attributes for creating an activity.
type GameCenterActivityCreateAttributes struct {
	ReferenceName       string            `json:"referenceName"`
	VendorIdentifier    string            `json:"vendorIdentifier"`
	PlayStyle           string            `json:"playStyle,omitempty"`
	MinimumPlayersCount *int              `json:"minimumPlayersCount,omitempty"`
	MaximumPlayersCount *int              `json:"maximumPlayersCount,omitempty"`
	SupportsPartyCode   *bool             `json:"supportsPartyCode,omitempty"`
	Properties          map[string]string `json:"properties,omitempty"`
}

// GameCenterActivityCreateRelationships contains relationships for creating
// an activity. An activity belongs to either an app's Game Center detail
// or a Game Center group, never both.
type GameCenterActivityCreateRelationships struct {
	GameCenterDetail *RelationshipData     `json:"gameCenterDetail,omitempty"`
	GameCenterGroup  *RelationshipData     `json:"gameCenterGroup,omitempty"`
	Versions         *RelationshipDataList `json:"versions,omitempty"`
}

// GameCenterActivityVersionInlineCreate declares the activity version
// created inline with an activity create request. The ID is a
// client-chosen temporary identifier referenced from the versions
// relationship.
type GameCenterActivityVersionInlineCreate struct {
	Type       string                                           `json:"type"`
	ID         string                                           `json:"id"`
	Attributes *GameCenterActivityVersionInlineCreateAttributes `json:"attributes,omitempty"`
}

// GameCenterActivityVersionInlineCreateAttributes contains the attributes
// settable on an inline activity version.
type GameCenterActivityVersionInlineCreateAttributes struct {
	FallbackURL string `json:"fallbackUrl,omitempty"`
}

// GameCenterActivityUpdateRequest represents a request to update an activity.
type GameCenterActivityUpdateRequest struct {
	Data GameCenterActivityUpdateData `json:"data"`
}

// GameCenterActivityUpdateData contains the data for updating an activity.
type GameCenterActivityUpdateData struct {
	Type       string                             `json:"type"`
	ID         string                             `json:"id"`
	Attributes GameCenterActivityUpdateAttributes `json:"attributes"`
}

// GameCenterActivityUpdateAttributes contains attributes for updating an activity.
type GameCenterActivityUpdateAttributes struct {
	ReferenceName       string            `json:"referenceName,omitempty"`
	PlayStyle           string            `json:"playStyle,omitempty"`
	MinimumPlayersCount *int              `json:"minimumPlayersCount,omitempty"`
	MaximumPlayersCount *int              `json:"maximumPlayersCount,omitempty"`
	SupportsPartyCode   *bool             `json:"supportsPartyCode,omitempty"`
	Archived            *bool             `json:"archived,omitempty"`
	Properties          map[string]string `json:"properties,omitempty"`
}

// GameCenterActivityVersionsResponse represents a list of activity versions.
type GameCenterActivityVersionsResponse struct {
	Data     []GameCenterActivityVersion `json:"data"`
	Links    PagedDocumentLinks          `json:"links"`
	Meta     *PagingInformation          `json:"meta,omitempty"`
	Included []any                       `json:"included,omitempty"`
}

// GameCenterActivityVersionResponse represents a single activity version.
type GameCenterActivityVersionResponse struct {
	Data     GameCenterActivityVersion `json:"data"`
	Included []any                     `json:"included,omitempty"`
}

// GameCenterActivityVersion represents one editable revision of an activity.
type GameCenterActivityVersion struct {
	Type       string                              `json:"type"`
	ID         string                              `json:"id"`
	Attributes GameCenterActivityVersionAttributes `json:"attributes"`
}

// GameCenterActivityVersionAttributes contains activity version attributes.
// FallbackURL is opened when the player's device cannot launch the activity.
type GameCenterActivityVersionAttributes struct {
	Version     int    `json:"version,omitempty"`
	State       string `json:"state,omitempty"`
	FallbackURL string `json:"fallbackUrl,omitempty"`
}

// GameCenterActivityVersionCreateRequest represents a request to open a new
// editable version of an activity.
type GameCenterActivityVersionCreateRequest struct {
	Data GameCenterActivityVersionCreateData `json:"data"`
}

// GameCenterActivityVersionCreateData contains the data for creating an activity version.
type GameCenterActivityVersionCreateData struct {
	Type          string                                       `json:"type"`
	Attributes    *GameCenterActivityVersionCreateAttributes   `json:"attributes,omitempty"`
	Relationships GameCenterActivityVersionCreateRelationships `json:"relationships"`
}

// GameCenterActivityVersionCreateAttributes contains attributes for creating an activity version.
type GameCenterActivityVersionCreateAttributes struct {
	FallbackURL string `json:"fallbackUrl,omitempty"`
}

// GameCenterActivityVersionCreateRelationships contains relationships for creating an activity version.
type GameCenterActivityVersionCreateRelationships struct {
	Activity RelationshipData `json:"activity"`
}

// GameCenterActivityVersionUpdateRequest represents a request to update an activity version.
type GameCenterActivityVersionUpdateRequest struct {
	Data GameCenterActivityVersionUpdateData `json:"data"`
}

// GameCenterActivityVersionUpdateData contains the data for updating an activity version.
type GameCenterActivityVersionUpdateData struct {
	Type       string                                    `json:"type"`
	ID         string                                    `json:"id"`
	Attributes GameCenterActivityVersionUpdateAttributes `json:"attributes"`
}

// GameCenterActivityVersionUpdateAttributes contains attributes for updating an activity version.
type GameCenterActivityVersionUpdateAttributes struct {
	FallbackURL string `json:"fallbackUrl,omitempty"`
}

// GameCenterActivityLocalizationsResponse represents a list of activity localizations.
type GameCenterActivityLocalizationsResponse struct {
	Data     []GameCenterActivityLocalization `json:"data"`
	Links    PagedDocumentLinks               `json:"links"`
	Meta     *PagingInformation               `json:"meta,omitempty"`
	Included []any                            `json:"included,omitempty"`
}

// GameCenterActivityLocalizationResponse represents a single activity localization.
type GameCenterActivityLocalizationResponse struct {
	Data     GameCenterActivityLocalization `json:"data"`
	Included []any                          `json:"included,omitempty"`
}

// GameCenterActivityLocalization represents the localized name and
// description of an activity in one locale.
type GameCenterActivityLocalization struct {
	Type       string                                   `json:"type"`
	ID         string                                   `json:"id"`
	Attributes GameCenterActivityLocalizationAttributes `json:"attributes"`
}

// GameCenterActivityLocalizationAttributes contains activity localization attributes.
type GameCenterActivityLocalizationAttributes struct {
	Locale      string `json:"locale,omitempty"`
	Name        string `json:"name,omitempty"`
	Description string `json:"description,omitempty"`
}

// GameCenterActivityLocalizationCreateRequest represents a request to create
// an activity localization against a version.
type GameCenterActivityLocalizationCreateRequest struct {
	Data GameCenterActivityLocalizationCreateData `json:"data"`
}

// GameCenterActivityLocalizationCreateData contains the data for creating an activity localization.
type GameCenterActivityLocalizationCreateData struct {
	Type          string                                            `json:"type"`
	Attributes    GameCenterActivityLocalizationCreateAttributes    `json:"attributes"`
	Relationships GameCenterActivityLocalizationCreateRelationships `json:"relationships"`
}

// GameCenterActivityLocalizationCreateAttributes contains attributes for creating an activity localization.
type GameCenterActivityLocalizationCreateAttributes struct {
	Locale      string `json:"locale"`
	Name        string `json:"name"`
	Description string `json:"description,omitempty"`
}

// GameCenterActivityLocalizationCreateRelationships contains relationships for creating an activity localization.
type GameCenterActivityLocalizationCreateRelationships struct {
	Version RelationshipData `json:"version"`
}

// GameCenterActivityLocalizationUpdateRequest represents a request to update an activity localization.
type GameCenterActivityLocalizationUpdateRequest struct {
	Data GameCenterActivityLocalizationUpdateData `json:"data"`
}

// GameCenterActivityLocalizationUpdateData contains the data for updating an activity localization.
type GameCenterActivityLocalizationUpdateData struct {
	Type       string                                         `json:"type"`
	ID         string                                         `json:"id"`
	Attributes GameCenterActivityLocalizationUpdateAttributes `json:"attributes"`
}

// GameCenterActivityLocalizationUpdateAttributes contains attributes for updating an activity localization.
type GameCenterActivityLocalizationUpdateAttributes struct {
	Name        string `json:"name,omitempty"`
	Description string `json:"description,omitempty"`
}

// Game Center challenge types

// GameCenterChallengesResponse represents a list of challenges.
type GameCenterChallengesResponse struct {
	Data     []GameCenterChallenge `json:"data"`
	Links    PagedDocumentLinks    `json:"links"`
	Meta     *PagingInformation    `json:"meta,omitempty"`
	Included []any                 `json:"included,omitempty"`
}

// GameCenterChallengeResponse represents a single challenge.
type GameCenterChallengeResponse struct {
	Data     GameCenterChallenge `json:"data"`
	Included []any               `json:"included,omitempty"`
}

// GameCenterChallenge represents a Game Center challenge: a competition
// between players built on top of a leaderboard.
type GameCenterChallenge struct {
	Type       string                        `json:"type"`
	ID         string                        `json:"id"`
	Attributes GameCenterChallengeAttributes `json:"attributes"`
}

// GameCenterChallengeAttributes contains challenge attributes. The
// allowedDurations attribute was removed in App Store Connect API 4.1 and
// is intentionally absent.
type GameCenterChallengeAttributes struct {
	ReferenceName    string `json:"referenceName,omitempty"`
	VendorIdentifier string `json:"vendorIdentifier,omitempty"`
	ChallengeType    string `json:"challengeType,omitempty"`
	Repeatable       bool   `json:"repeatable,omitempty"`
	Archived         bool   `json:"archived,omitempty"`
}

// GameCenterChallengeCreateRequest represents a request to create a
// challenge. Like every versioned Game Center resource, the initial
// version is declared inline through the versions relationship and the
// included array.
type GameCenterChallengeCreateRequest struct {
	Data     GameCenterChallengeCreateData   `json:"data"`
	Included []GameCenterVersionInlineCreate `json:"included,omitempty"`
}

// GameCenterChallengeCreateData contains the data for creating a challenge.
type GameCenterChallengeCreateData struct {
	Type          string                                 `json:"type"`
	Attributes    GameCenterChallengeCreateAttributes    `json:"attributes"`
	Relationships GameCenterChallengeCreateRelationships `json:"relationships"`
}

// GameCenterChallengeCreateAttributes contains attributes for creating a challenge.
type GameCenterChallengeCreateAttributes struct {
	ReferenceName    string `json:"referenceName"`
	VendorIdentifier string `json:"vendorIdentifier"`
	ChallengeType    string `json:"challengeType"`
	Repeatable       *bool  `json:"repeatable,omitempty"`
}

// GameCenterChallengeCreateRelationships contains relationships for creating
// a challenge. LeaderboardV2 points at the v2 leaderboard the challenge
// scores against; the deprecated v1 `leaderboard` key is not exposed.
type GameCenterChallengeCreateRelationships struct {
	GameCenterDetail *RelationshipData     `json:"gameCenterDetail,omitempty"`
	GameCenterGroup  *RelationshipData     `json:"gameCenterGroup,omitempty"`
	LeaderboardV2    *RelationshipData     `json:"leaderboardV2,omitempty"`
	Versions         *RelationshipDataList `json:"versions,omitempty"`
}

// GameCenterChallengeUpdateRequest represents a request to update a challenge.
type GameCenterChallengeUpdateRequest struct {
	Data GameCenterChallengeUpdateData `json:"data"`
}

// GameCenterChallengeUpdateData contains the data for updating a challenge.
type GameCenterChallengeUpdateData struct {
	Type          string                                  `json:"type"`
	ID            string                                  `json:"id"`
	Attributes    GameCenterChallengeUpdateAttributes     `json:"attributes"`
	Relationships *GameCenterChallengeUpdateRelationships `json:"relationships,omitempty"`
}

// GameCenterChallengeUpdateAttributes contains attributes for updating a challenge.
type GameCenterChallengeUpdateAttributes struct {
	ReferenceName string `json:"referenceName,omitempty"`
	Repeatable    *bool  `json:"repeatable,omitempty"`
	Archived      *bool  `json:"archived,omitempty"`
}

// GameCenterChallengeUpdateRelationships contains relationships for updating a challenge.
type GameCenterChallengeUpdateRelationships struct {
	LeaderboardV2 *RelationshipData `json:"leaderboardV2,omitempty"`
}

// GameCenterChallengeVersionsResponse represents a list of challenge versions.
type GameCenterChallengeVersionsResponse struct {
	Data     []GameCenterChallengeVersion `json:"data"`
	Links    PagedDocumentLinks           `json:"links"`
	Meta     *PagingInformation           `json:"meta,omitempty"`
	Included []any                        `json:"included,omitempty"`
}

// GameCenterChallengeVersionResponse represents a single challenge version.
type GameCenterChallengeVersionResponse struct {
	Data     GameCenterChallengeVersion `json:"data"`
	Included []any                      `json:"included,omitempty"`
}

// GameCenterChallengeVersion represents one editable revision of a challenge.
type GameCenterChallengeVersion struct {
	Type       string                               `json:"type"`
	ID         string                               `json:"id"`
	Attributes GameCenterChallengeVersionAttributes `json:"attributes"`
}

// GameCenterChallengeVersionAttributes contains challenge version attributes.
type GameCenterChallengeVersionAttributes struct {
	Version int    `json:"version,omitempty"`
	State   string `json:"state,omitempty"`
}

// GameCenterChallengeVersionCreateRequest represents a request to open a new
// editable version of a challenge.
type GameCenterChallengeVersionCreateRequest struct {
	Data GameCenterChallengeVersionCreateData `json:"data"`
}

// GameCenterChallengeVersionCreateData contains the data for creating a challenge version.
type GameCenterChallengeVersionCreateData struct {
	Type          string                                        `json:"type"`
	Relationships GameCenterChallengeVersionCreateRelationships `json:"relationships"`
}

// GameCenterChallengeVersionCreateRelationships contains relationships for creating a challenge version.
type GameCenterChallengeVersionCreateRelationships struct {
	Challenge RelationshipData `json:"challenge"`
}

// GameCenterChallengeLocalizationsResponse represents a list of challenge localizations.
type GameCenterChallengeLocalizationsResponse struct {
	Data     []GameCenterChallengeLocalization `json:"data"`
	Links    PagedDocumentLinks                `json:"links"`
	Meta     *PagingInformation                `json:"meta,omitempty"`
	Included []any                             `json:"included,omitempty"`
}

// GameCenterChallengeLocalizationResponse represents a single challenge localization.
type GameCenterChallengeLocalizationResponse struct {
	Data     GameCenterChallengeLocalization `json:"data"`
	Included []any                           `json:"included,omitempty"`
}

// GameCenterChallengeLocalization represents the localized name and
// description of a challenge in one locale.
type GameCenterChallengeLocalization struct {
	Type       string                                    `json:"type"`
	ID         string                                    `json:"id"`
	Attributes GameCenterChallengeLocalizationAttributes `json:"attributes"`
}

// GameCenterChallengeLocalizationAttributes contains challenge localization attributes.
type GameCenterChallengeLocalizationAttributes struct {
	Locale      string `json:"locale,omitempty"`
	Name        string `json:"name,omitempty"`
	Description string `json:"description,omitempty"`
}

// GameCenterChallengeLocalizationCreateRequest represents a request to create
// a challenge localization against a version.
type GameCenterChallengeLocalizationCreateRequest struct {
	Data GameCenterChallengeLocalizationCreateData `json:"data"`
}

// GameCenterChallengeLocalizationCreateData contains the data for creating a challenge localization.
type GameCenterChallengeLocalizationCreateData struct {
	Type          string                                             `json:"type"`
	Attributes    GameCenterChallengeLocalizationCreateAttributes    `json:"attributes"`
	Relationships GameCenterChallengeLocalizationCreateRelationships `json:"relationships"`
}

// GameCenterChallengeLocalizationCreateAttributes contains attributes for creating a challenge localization.
type GameCenterChallengeLocalizationCreateAttributes struct {
	Locale      string `json:"locale"`
	Name        string `json:"name"`
	Description string `json:"description,omitempty"`
}

// GameCenterChallengeLocalizationCreateRelationships contains relationships for creating a challenge localization.
type GameCenterChallengeLocalizationCreateRelationships struct {
	Version RelationshipData `json:"version"`
}

// GameCenterChallengeLocalizationUpdateRequest represents a request to update a challenge localization.
type GameCenterChallengeLocalizationUpdateRequest struct {
	Data GameCenterChallengeLocalizationUpdateData `json:"data"`
}

// GameCenterChallengeLocalizationUpdateData contains the data for updating a challenge localization.
type GameCenterChallengeLocalizationUpdateData struct {
	Type       string                                          `json:"type"`
	ID         string                                          `json:"id"`
	Attributes GameCenterChallengeLocalizationUpdateAttributes `json:"attributes"`
}

// GameCenterChallengeLocalizationUpdateAttributes contains attributes for updating a challenge localization.
type GameCenterChallengeLocalizationUpdateAttributes struct {
	Name        string `json:"name,omitempty"`
	Description string `json:"description,omitempty"`
}

// Game Center player submission types (App Store Connect API 4.3)

// GameCenterPlayerAchievementSubmissionResponse represents a submitted
// achievement progress record.
type GameCenterPlayerAchievementSubmissionResponse struct {
	Data     GameCenterPlayerAchievementSubmission `json:"data"`
	Included []any                                 `json:"included,omitempty"`
}

// GameCenterPlayerAchievementSubmission represents achievement progress a
// server posted on a player's behalf.
type GameCenterPlayerAchievementSubmission struct {
	Type       string                                          `json:"type"`
	ID         string                                          `json:"id"`
	Attributes GameCenterPlayerAchievementSubmissionAttributes `json:"attributes"`
}

// GameCenterPlayerAchievementSubmissionAttributes contains achievement
// submission attributes. PreReleased marks progress earned in a build that
// has not shipped to the App Store yet.
type GameCenterPlayerAchievementSubmissionAttributes struct {
	BundleID           string   `json:"bundleId,omitempty"`
	ChallengeIDs       []string `json:"challengeIds,omitempty"`
	PercentageAchieved int      `json:"percentageAchieved,omitempty"`
	ScopedPlayerID     string   `json:"scopedPlayerId,omitempty"`
	SubmittedDate      string   `json:"submittedDate,omitempty"`
	VendorIdentifier   string   `json:"vendorIdentifier,omitempty"`
	PreReleased        bool     `json:"preReleased,omitempty"`
}

// GameCenterPlayerAchievementSubmissionCreateRequest represents a request to
// submit achievement progress for a player.
type GameCenterPlayerAchievementSubmissionCreateRequest struct {
	Data GameCenterPlayerAchievementSubmissionCreateData `json:"data"`
}

// GameCenterPlayerAchievementSubmissionCreateData contains the data for submitting achievement progress.
type GameCenterPlayerAchievementSubmissionCreateData struct {
	Type       string                                                `json:"type"`
	Attributes GameCenterPlayerAchievementSubmissionCreateAttributes `json:"attributes"`
}

// GameCenterPlayerAchievementSubmissionCreateAttributes contains attributes
// for submitting achievement progress.
type GameCenterPlayerAchievementSubmissionCreateAttributes struct {
	BundleID           string   `json:"bundleId"`
	VendorIdentifier   string   `json:"vendorIdentifier"`
	ScopedPlayerID     string   `json:"scopedPlayerId"`
	PercentageAchieved int      `json:"percentageAchieved"`
	ChallengeIDs       []string `json:"challengeIds,omitempty"`
	SubmittedDate      string   `json:"submittedDate,omitempty"`
	PreReleased        *bool    `json:"preReleased,omitempty"`
}

// GameCenterLeaderboardEntrySubmissionResponse represents a submitted
// leaderboard score.
type GameCenterLeaderboardEntrySubmissionResponse struct {
	Data     GameCenterLeaderboardEntrySubmission `json:"data"`
	Included []any                                `json:"included,omitempty"`
}

// GameCenterLeaderboardEntrySubmission represents a leaderboard score a
// server posted on a player's behalf.
type GameCenterLeaderboardEntrySubmission struct {
	Type       string                                         `json:"type"`
	ID         string                                         `json:"id"`
	Attributes GameCenterLeaderboardEntrySubmissionAttributes `json:"attributes"`
}

// GameCenterLeaderboardEntrySubmissionAttributes contains leaderboard entry
// submission attributes. Score and Context are decimal strings because the
// API models them as arbitrary-precision numbers.
type GameCenterLeaderboardEntrySubmissionAttributes struct {
	BundleID         string   `json:"bundleId,omitempty"`
	ChallengeIDs     []string `json:"challengeIds,omitempty"`
	Context          string   `json:"context,omitempty"`
	ScopedPlayerID   string   `json:"scopedPlayerId,omitempty"`
	Score            string   `json:"score,omitempty"`
	SubmittedDate    string   `json:"submittedDate,omitempty"`
	VendorIdentifier string   `json:"vendorIdentifier,omitempty"`
	PreReleased      bool     `json:"preReleased,omitempty"`
}

// GameCenterLeaderboardEntrySubmissionCreateRequest represents a request to
// submit a leaderboard score for a player.
type GameCenterLeaderboardEntrySubmissionCreateRequest struct {
	Data GameCenterLeaderboardEntrySubmissionCreateData `json:"data"`
}

// GameCenterLeaderboardEntrySubmissionCreateData contains the data for submitting a score.
type GameCenterLeaderboardEntrySubmissionCreateData struct {
	Type       string                                               `json:"type"`
	Attributes GameCenterLeaderboardEntrySubmissionCreateAttributes `json:"attributes"`
}

// GameCenterLeaderboardEntrySubmissionCreateAttributes contains attributes for submitting a score.
type GameCenterLeaderboardEntrySubmissionCreateAttributes struct {
	BundleID         string   `json:"bundleId"`
	VendorIdentifier string   `json:"vendorIdentifier"`
	ScopedPlayerID   string   `json:"scopedPlayerId"`
	Score            string   `json:"score"`
	Context          string   `json:"context,omitempty"`
	ChallengeIDs     []string `json:"challengeIds,omitempty"`
	SubmittedDate    string   `json:"submittedDate,omitempty"`
	PreReleased      *bool    `json:"preReleased,omitempty"`
}

// Game Center leaderboard set methods

// ListGameCenterLeaderboardSets returns the leaderboard sets for a game
// center detail via the v2 Game Center API.
func (c *Client) ListGameCenterLeaderboardSets(ctx context.Context, gameCenterDetailID string, opts *ListOptions) (*GameCenterLeaderboardSetsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/gameCenterDetails/"+url.PathEscape(gameCenterDetailID)+"/gameCenterLeaderboardSetsV2", query)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardSetsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetGameCenterLeaderboardSet returns a single leaderboard set.
func (c *Client) GetGameCenterLeaderboardSet(ctx context.Context, leaderboardSetID string) (*GameCenterLeaderboardSetResponse, error) {
	data, err := c.Get(ctx, "/v2/gameCenterLeaderboardSets/"+url.PathEscape(leaderboardSetID), nil)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardSetResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateGameCenterLeaderboardSet creates a leaderboard set via the v2 Game Center API.
func (c *Client) CreateGameCenterLeaderboardSet(ctx context.Context, req *GameCenterLeaderboardSetCreateRequest) (*GameCenterLeaderboardSetResponse, error) {
	data, err := c.Post(ctx, "/v2/gameCenterLeaderboardSets", req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardSetResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateGameCenterLeaderboardSet updates a leaderboard set.
func (c *Client) UpdateGameCenterLeaderboardSet(ctx context.Context, leaderboardSetID string, req *GameCenterLeaderboardSetUpdateRequest) (*GameCenterLeaderboardSetResponse, error) {
	data, err := c.Patch(ctx, "/v2/gameCenterLeaderboardSets/"+url.PathEscape(leaderboardSetID), req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardSetResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteGameCenterLeaderboardSet deletes a leaderboard set.
func (c *Client) DeleteGameCenterLeaderboardSet(ctx context.Context, leaderboardSetID string) error {
	return c.Delete(ctx, "/v2/gameCenterLeaderboardSets/"+url.PathEscape(leaderboardSetID))
}

// ListGameCenterLeaderboardSetMembers returns the leaderboards that belong
// to a leaderboard set.
func (c *Client) ListGameCenterLeaderboardSetMembers(ctx context.Context, leaderboardSetID string, opts *ListOptions) (*GameCenterLeaderboardsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v2/gameCenterLeaderboardSets/"+url.PathEscape(leaderboardSetID)+"/gameCenterLeaderboards", query)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// AddGameCenterLeaderboardSetMembers adds leaderboards to a leaderboard set.
func (c *Client) AddGameCenterLeaderboardSetMembers(ctx context.Context, leaderboardSetID string, req *RelationshipDataList) error {
	_, err := c.Post(ctx, "/v2/gameCenterLeaderboardSets/"+url.PathEscape(leaderboardSetID)+"/relationships/gameCenterLeaderboards", req)
	return err
}

// RemoveGameCenterLeaderboardSetMembers removes leaderboards from a
// leaderboard set. The linkages travel in the DELETE body.
func (c *Client) RemoveGameCenterLeaderboardSetMembers(ctx context.Context, leaderboardSetID string, req *RelationshipDataList) error {
	return c.deleteWithBody(ctx, "/v2/gameCenterLeaderboardSets/"+url.PathEscape(leaderboardSetID)+"/relationships/gameCenterLeaderboards", req)
}

// ListGameCenterLeaderboardSetVersions returns the versions of a leaderboard set.
func (c *Client) ListGameCenterLeaderboardSetVersions(ctx context.Context, leaderboardSetID string, opts *ListOptions) (*GameCenterLeaderboardSetVersionsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v2/gameCenterLeaderboardSets/"+url.PathEscape(leaderboardSetID)+"/versions", query)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardSetVersionsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetGameCenterLeaderboardSetVersion returns a single leaderboard set version.
func (c *Client) GetGameCenterLeaderboardSetVersion(ctx context.Context, versionID string) (*GameCenterLeaderboardSetVersionResponse, error) {
	data, err := c.Get(ctx, "/v2/gameCenterLeaderboardSetVersions/"+url.PathEscape(versionID), nil)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardSetVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateGameCenterLeaderboardSetVersion opens a new editable version of a leaderboard set.
func (c *Client) CreateGameCenterLeaderboardSetVersion(ctx context.Context, req *GameCenterLeaderboardSetVersionCreateRequest) (*GameCenterLeaderboardSetVersionResponse, error) {
	data, err := c.Post(ctx, "/v2/gameCenterLeaderboardSetVersions", req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardSetVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListGameCenterLeaderboardSetLocalizations returns the localizations of a
// leaderboard set version.
func (c *Client) ListGameCenterLeaderboardSetLocalizations(ctx context.Context, versionID string, opts *ListOptions) (*GameCenterLeaderboardSetLocalizationsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v2/gameCenterLeaderboardSetVersions/"+url.PathEscape(versionID)+"/localizations", query)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardSetLocalizationsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateGameCenterLeaderboardSetLocalization creates a leaderboard set localization.
func (c *Client) CreateGameCenterLeaderboardSetLocalization(ctx context.Context, req *GameCenterLeaderboardSetLocalizationCreateRequest) (*GameCenterLeaderboardSetLocalizationResponse, error) {
	data, err := c.Post(ctx, "/v2/gameCenterLeaderboardSetLocalizations", req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardSetLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateGameCenterLeaderboardSetLocalization updates a leaderboard set localization.
func (c *Client) UpdateGameCenterLeaderboardSetLocalization(ctx context.Context, localizationID string, req *GameCenterLeaderboardSetLocalizationUpdateRequest) (*GameCenterLeaderboardSetLocalizationResponse, error) {
	data, err := c.Patch(ctx, "/v2/gameCenterLeaderboardSetLocalizations/"+url.PathEscape(localizationID), req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardSetLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteGameCenterLeaderboardSetLocalization deletes a leaderboard set localization.
func (c *Client) DeleteGameCenterLeaderboardSetLocalization(ctx context.Context, localizationID string) error {
	return c.Delete(ctx, "/v2/gameCenterLeaderboardSetLocalizations/"+url.PathEscape(localizationID))
}

// Game Center leaderboard set member localization methods

// ListGameCenterLeaderboardSetMemberLocalizations returns the per-set names
// of leaderboards. The API requires at least one of the
// filter[gameCenterLeaderboardSet] / filter[gameCenterLeaderboard] filters,
// which the caller supplies through opts.
func (c *Client) ListGameCenterLeaderboardSetMemberLocalizations(ctx context.Context, opts *ListOptions) (*GameCenterLeaderboardSetMemberLocalizationsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/gameCenterLeaderboardSetMemberLocalizations", query)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardSetMemberLocalizationsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateGameCenterLeaderboardSetMemberLocalization names a leaderboard
// within a leaderboard set for one locale.
func (c *Client) CreateGameCenterLeaderboardSetMemberLocalization(ctx context.Context, req *GameCenterLeaderboardSetMemberLocalizationCreateRequest) (*GameCenterLeaderboardSetMemberLocalizationResponse, error) {
	data, err := c.Post(ctx, "/v1/gameCenterLeaderboardSetMemberLocalizations", req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardSetMemberLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateGameCenterLeaderboardSetMemberLocalization updates a member localization.
func (c *Client) UpdateGameCenterLeaderboardSetMemberLocalization(ctx context.Context, localizationID string, req *GameCenterLeaderboardSetMemberLocalizationUpdateRequest) (*GameCenterLeaderboardSetMemberLocalizationResponse, error) {
	data, err := c.Patch(ctx, "/v1/gameCenterLeaderboardSetMemberLocalizations/"+url.PathEscape(localizationID), req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardSetMemberLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteGameCenterLeaderboardSetMemberLocalization deletes a member localization.
func (c *Client) DeleteGameCenterLeaderboardSetMemberLocalization(ctx context.Context, localizationID string) error {
	return c.Delete(ctx, "/v1/gameCenterLeaderboardSetMemberLocalizations/"+url.PathEscape(localizationID))
}

// Game Center activity methods

// ListGameCenterActivities returns the activities for a game center detail.
func (c *Client) ListGameCenterActivities(ctx context.Context, gameCenterDetailID string, opts *ListOptions) (*GameCenterActivitiesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/gameCenterDetails/"+url.PathEscape(gameCenterDetailID)+"/gameCenterActivities", query)
	if err != nil {
		return nil, err
	}

	var resp GameCenterActivitiesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetGameCenterActivity returns a single activity.
func (c *Client) GetGameCenterActivity(ctx context.Context, activityID string) (*GameCenterActivityResponse, error) {
	data, err := c.Get(ctx, "/v1/gameCenterActivities/"+url.PathEscape(activityID), nil)
	if err != nil {
		return nil, err
	}

	var resp GameCenterActivityResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateGameCenterActivity creates an activity along with its initial version.
func (c *Client) CreateGameCenterActivity(ctx context.Context, req *GameCenterActivityCreateRequest) (*GameCenterActivityResponse, error) {
	data, err := c.Post(ctx, "/v1/gameCenterActivities", req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterActivityResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateGameCenterActivity updates an activity.
func (c *Client) UpdateGameCenterActivity(ctx context.Context, activityID string, req *GameCenterActivityUpdateRequest) (*GameCenterActivityResponse, error) {
	data, err := c.Patch(ctx, "/v1/gameCenterActivities/"+url.PathEscape(activityID), req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterActivityResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteGameCenterActivity deletes an activity.
func (c *Client) DeleteGameCenterActivity(ctx context.Context, activityID string) error {
	return c.Delete(ctx, "/v1/gameCenterActivities/"+url.PathEscape(activityID))
}

// ListGameCenterActivityVersions returns the versions of an activity.
func (c *Client) ListGameCenterActivityVersions(ctx context.Context, activityID string, opts *ListOptions) (*GameCenterActivityVersionsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/gameCenterActivities/"+url.PathEscape(activityID)+"/versions", query)
	if err != nil {
		return nil, err
	}

	var resp GameCenterActivityVersionsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetGameCenterActivityVersion returns a single activity version.
func (c *Client) GetGameCenterActivityVersion(ctx context.Context, versionID string) (*GameCenterActivityVersionResponse, error) {
	data, err := c.Get(ctx, "/v1/gameCenterActivityVersions/"+url.PathEscape(versionID), nil)
	if err != nil {
		return nil, err
	}

	var resp GameCenterActivityVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateGameCenterActivityVersion opens a new editable version of an activity.
func (c *Client) CreateGameCenterActivityVersion(ctx context.Context, req *GameCenterActivityVersionCreateRequest) (*GameCenterActivityVersionResponse, error) {
	data, err := c.Post(ctx, "/v1/gameCenterActivityVersions", req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterActivityVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateGameCenterActivityVersion updates an activity version's fallback URL.
func (c *Client) UpdateGameCenterActivityVersion(ctx context.Context, versionID string, req *GameCenterActivityVersionUpdateRequest) (*GameCenterActivityVersionResponse, error) {
	data, err := c.Patch(ctx, "/v1/gameCenterActivityVersions/"+url.PathEscape(versionID), req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterActivityVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListGameCenterActivityLocalizations returns the localizations of an activity version.
func (c *Client) ListGameCenterActivityLocalizations(ctx context.Context, versionID string, opts *ListOptions) (*GameCenterActivityLocalizationsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/gameCenterActivityVersions/"+url.PathEscape(versionID)+"/localizations", query)
	if err != nil {
		return nil, err
	}

	var resp GameCenterActivityLocalizationsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateGameCenterActivityLocalization creates an activity localization.
func (c *Client) CreateGameCenterActivityLocalization(ctx context.Context, req *GameCenterActivityLocalizationCreateRequest) (*GameCenterActivityLocalizationResponse, error) {
	data, err := c.Post(ctx, "/v1/gameCenterActivityLocalizations", req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterActivityLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateGameCenterActivityLocalization updates an activity localization.
func (c *Client) UpdateGameCenterActivityLocalization(ctx context.Context, localizationID string, req *GameCenterActivityLocalizationUpdateRequest) (*GameCenterActivityLocalizationResponse, error) {
	data, err := c.Patch(ctx, "/v1/gameCenterActivityLocalizations/"+url.PathEscape(localizationID), req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterActivityLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteGameCenterActivityLocalization deletes an activity localization.
func (c *Client) DeleteGameCenterActivityLocalization(ctx context.Context, localizationID string) error {
	return c.Delete(ctx, "/v1/gameCenterActivityLocalizations/"+url.PathEscape(localizationID))
}

// Game Center challenge methods

// ListGameCenterChallenges returns the challenges for a game center detail.
func (c *Client) ListGameCenterChallenges(ctx context.Context, gameCenterDetailID string, opts *ListOptions) (*GameCenterChallengesResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/gameCenterDetails/"+url.PathEscape(gameCenterDetailID)+"/gameCenterChallenges", query)
	if err != nil {
		return nil, err
	}

	var resp GameCenterChallengesResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetGameCenterChallenge returns a single challenge.
func (c *Client) GetGameCenterChallenge(ctx context.Context, challengeID string) (*GameCenterChallengeResponse, error) {
	data, err := c.Get(ctx, "/v1/gameCenterChallenges/"+url.PathEscape(challengeID), nil)
	if err != nil {
		return nil, err
	}

	var resp GameCenterChallengeResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateGameCenterChallenge creates a challenge along with its initial version.
func (c *Client) CreateGameCenterChallenge(ctx context.Context, req *GameCenterChallengeCreateRequest) (*GameCenterChallengeResponse, error) {
	data, err := c.Post(ctx, "/v1/gameCenterChallenges", req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterChallengeResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateGameCenterChallenge updates a challenge.
func (c *Client) UpdateGameCenterChallenge(ctx context.Context, challengeID string, req *GameCenterChallengeUpdateRequest) (*GameCenterChallengeResponse, error) {
	data, err := c.Patch(ctx, "/v1/gameCenterChallenges/"+url.PathEscape(challengeID), req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterChallengeResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteGameCenterChallenge deletes a challenge.
func (c *Client) DeleteGameCenterChallenge(ctx context.Context, challengeID string) error {
	return c.Delete(ctx, "/v1/gameCenterChallenges/"+url.PathEscape(challengeID))
}

// ListGameCenterChallengeVersions returns the versions of a challenge.
func (c *Client) ListGameCenterChallengeVersions(ctx context.Context, challengeID string, opts *ListOptions) (*GameCenterChallengeVersionsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/gameCenterChallenges/"+url.PathEscape(challengeID)+"/versions", query)
	if err != nil {
		return nil, err
	}

	var resp GameCenterChallengeVersionsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// GetGameCenterChallengeVersion returns a single challenge version.
func (c *Client) GetGameCenterChallengeVersion(ctx context.Context, versionID string) (*GameCenterChallengeVersionResponse, error) {
	data, err := c.Get(ctx, "/v1/gameCenterChallengeVersions/"+url.PathEscape(versionID), nil)
	if err != nil {
		return nil, err
	}

	var resp GameCenterChallengeVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateGameCenterChallengeVersion opens a new editable version of a challenge.
func (c *Client) CreateGameCenterChallengeVersion(ctx context.Context, req *GameCenterChallengeVersionCreateRequest) (*GameCenterChallengeVersionResponse, error) {
	data, err := c.Post(ctx, "/v1/gameCenterChallengeVersions", req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterChallengeVersionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// ListGameCenterChallengeLocalizations returns the localizations of a challenge version.
func (c *Client) ListGameCenterChallengeLocalizations(ctx context.Context, versionID string, opts *ListOptions) (*GameCenterChallengeLocalizationsResponse, error) {
	query := url.Values{}
	if opts != nil {
		opts.Apply(query)
	}

	data, err := c.Get(ctx, "/v1/gameCenterChallengeVersions/"+url.PathEscape(versionID)+"/localizations", query)
	if err != nil {
		return nil, err
	}

	var resp GameCenterChallengeLocalizationsResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateGameCenterChallengeLocalization creates a challenge localization.
func (c *Client) CreateGameCenterChallengeLocalization(ctx context.Context, req *GameCenterChallengeLocalizationCreateRequest) (*GameCenterChallengeLocalizationResponse, error) {
	data, err := c.Post(ctx, "/v1/gameCenterChallengeLocalizations", req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterChallengeLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// UpdateGameCenterChallengeLocalization updates a challenge localization.
func (c *Client) UpdateGameCenterChallengeLocalization(ctx context.Context, localizationID string, req *GameCenterChallengeLocalizationUpdateRequest) (*GameCenterChallengeLocalizationResponse, error) {
	data, err := c.Patch(ctx, "/v1/gameCenterChallengeLocalizations/"+url.PathEscape(localizationID), req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterChallengeLocalizationResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// DeleteGameCenterChallengeLocalization deletes a challenge localization.
func (c *Client) DeleteGameCenterChallengeLocalization(ctx context.Context, localizationID string) error {
	return c.Delete(ctx, "/v1/gameCenterChallengeLocalizations/"+url.PathEscape(localizationID))
}

// Game Center image upload methods

// createGameCenterImage reserves an image upload at the given collection path.
func (c *Client) createGameCenterImage(ctx context.Context, path string, req *GameCenterImageCreateRequest) (*GameCenterImageResponse, error) {
	data, err := c.Post(ctx, path, req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterImageResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// commitGameCenterImage marks a reserved image as uploaded.
func (c *Client) commitGameCenterImage(ctx context.Context, path string, req *GameCenterImageUpdateRequest) (*GameCenterImageResponse, error) {
	data, err := c.Patch(ctx, path, req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterImageResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// uploadGameCenterImage reserves, uploads, and commits a Game Center image
// in one call. collectionPath is the image collection endpoint,
// resourceType the JSON:API type, and relationships the parent linkage
// (localization or version) the image attaches to.
func (c *Client) uploadGameCenterImage(ctx context.Context, collectionPath, resourceType string, relationships map[string]*RelationshipData, fileName string, body []byte) (*GameCenterImageResponse, error) {
	if fileName == "" {
		return nil, fmt.Errorf("file_name is required")
	}
	if len(body) == 0 {
		return nil, fmt.Errorf("file body is empty")
	}
	if len(body) > MaxUploadSize {
		return nil, fmt.Errorf("file body exceeds max size of %d bytes", MaxUploadSize)
	}

	reservation, err := c.createGameCenterImage(ctx, collectionPath, &GameCenterImageCreateRequest{
		Data: GameCenterImageCreateData{
			Type: resourceType,
			Attributes: GameCenterImageCreateAttrs{
				FileName: fileName,
				FileSize: len(body),
			},
			Relationships: relationships,
		},
	})
	if err != nil {
		return nil, fmt.Errorf("reservation: %w", err)
	}

	if err := c.PerformUploadOperations(ctx, reservation.Data.Attributes.UploadOperations, body); err != nil {
		return nil, fmt.Errorf("upload: %w", err)
	}

	// Game Center images commit with uploaded=true only; unlike App Store
	// screenshots they carry no source file checksum.
	uploaded := true
	return c.commitGameCenterImage(ctx, collectionPath+"/"+url.PathEscape(reservation.Data.ID), &GameCenterImageUpdateRequest{
		Data: GameCenterImageUpdateData{
			Type:       resourceType,
			ID:         reservation.Data.ID,
			Attributes: GameCenterImageUpdateAttrs{Uploaded: &uploaded},
		},
	})
}

// UploadGameCenterLeaderboardSetImage uploads the image shown for a
// leaderboard set localization.
func (c *Client) UploadGameCenterLeaderboardSetImage(ctx context.Context, localizationID, fileName string, body []byte) (*GameCenterImageResponse, error) {
	return c.uploadGameCenterImage(ctx, "/v2/gameCenterLeaderboardSetImages", "gameCenterLeaderboardSetImages", map[string]*RelationshipData{
		"localization": {Data: ResourceIdentifier{Type: "gameCenterLeaderboardSetLocalizations", ID: localizationID}},
	}, fileName, body)
}

// UploadGameCenterActivityImage uploads an activity image. Passing a
// localization ID sets that locale's image; passing a version ID instead
// sets the version's default image.
func (c *Client) UploadGameCenterActivityImage(ctx context.Context, localizationID, versionID, fileName string, body []byte) (*GameCenterImageResponse, error) {
	rels := map[string]*RelationshipData{}
	if localizationID != "" {
		rels["localization"] = &RelationshipData{Data: ResourceIdentifier{Type: "gameCenterActivityLocalizations", ID: localizationID}}
	}
	if versionID != "" {
		rels["version"] = &RelationshipData{Data: ResourceIdentifier{Type: "gameCenterActivityVersions", ID: versionID}}
	}
	return c.uploadGameCenterImage(ctx, "/v1/gameCenterActivityImages", "gameCenterActivityImages", rels, fileName, body)
}

// UploadGameCenterChallengeImage uploads a challenge image. Passing a
// localization ID sets that locale's image; passing a version ID instead
// sets the version's default image.
func (c *Client) UploadGameCenterChallengeImage(ctx context.Context, localizationID, versionID, fileName string, body []byte) (*GameCenterImageResponse, error) {
	rels := map[string]*RelationshipData{}
	if localizationID != "" {
		rels["localization"] = &RelationshipData{Data: ResourceIdentifier{Type: "gameCenterChallengeLocalizations", ID: localizationID}}
	}
	if versionID != "" {
		rels["version"] = &RelationshipData{Data: ResourceIdentifier{Type: "gameCenterChallengeVersions", ID: versionID}}
	}
	return c.uploadGameCenterImage(ctx, "/v1/gameCenterChallengeImages", "gameCenterChallengeImages", rels, fileName, body)
}

// Game Center player submission methods

// CreateGameCenterPlayerAchievementSubmission submits achievement progress
// for a player on the game server's behalf.
func (c *Client) CreateGameCenterPlayerAchievementSubmission(ctx context.Context, req *GameCenterPlayerAchievementSubmissionCreateRequest) (*GameCenterPlayerAchievementSubmissionResponse, error) {
	data, err := c.Post(ctx, "/v1/gameCenterPlayerAchievementSubmissions", req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterPlayerAchievementSubmissionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}

// CreateGameCenterLeaderboardEntrySubmission submits a leaderboard score for
// a player on the game server's behalf.
func (c *Client) CreateGameCenterLeaderboardEntrySubmission(ctx context.Context, req *GameCenterLeaderboardEntrySubmissionCreateRequest) (*GameCenterLeaderboardEntrySubmissionResponse, error) {
	data, err := c.Post(ctx, "/v1/gameCenterLeaderboardEntrySubmissions", req)
	if err != nil {
		return nil, err
	}

	var resp GameCenterLeaderboardEntrySubmissionResponse
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return &resp, nil
}
