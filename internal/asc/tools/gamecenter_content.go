package tools

import (
	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// registerGameCenterContentTools registers the Game Center content tools
// added by App Store Connect API 4.0-4.3: leaderboard sets on the v2
// version tree, activities and challenges with their versions,
// localizations, images and releases, and the player submission
// endpoints a game server uses to post progress on a player's behalf.
func (r *Registry) registerGameCenterContentTools() {
	r.registerGameCenterLeaderboardSetTools()
	r.registerGameCenterActivityTools()
	r.registerGameCenterChallengeTools()
	r.registerGameCenterSubmissionTools()
}

// gcListParams captures the JSON:API query knobs every Game Center list
// tool accepts. Handlers embed it next to their own parent ID field;
// encoding/json flattens the embedded struct.
type gcListParams struct {
	Limit   int                 `json:"limit"`
	Cursor  string              `json:"cursor"`
	Filter  map[string][]string `json:"filter"`
	Fields  map[string][]string `json:"fields"`
	Include []string            `json:"include"`
}

// gcLimit clamps a caller-supplied page size to the range the App Store
// Connect API accepts, defaulting to 50.
func gcLimit(limit int) int {
	if limit <= 0 {
		return 50
	}
	if limit > 200 {
		return 200
	}
	return limit
}

// opts returns the *api.ListOptions for these parameters with the limit
// already clamped.
func (p gcListParams) opts() *api.ListOptions {
	return listOpts(gcLimit(p.Limit), p.Filter, nil, p.Fields, p.Include)
}

// gcListSchema builds the input schema for a Game Center list tool: one
// required parent ID plus the JSON:API knobs shared by every collection
// endpoint. noun names the listed resource in the limit description.
func gcListSchema(idKey, idDescription, noun string) mcp.JSONSchema {
	return mcp.JSONSchema{
		Type: "object",
		Properties: map[string]mcp.Property{
			idKey: {
				Type:        "string",
				Description: idDescription,
			},
			"limit": {
				Type:        "integer",
				Description: "Maximum number of " + noun + " to return (default 50, max 200)",
			},
			"cursor": cursorProperty(),
			"filter": {
				Type:        "object",
				Description: "JSON:API filter map. Keys are attribute names; values are arrays of allowed values, e.g. {\"referenceName\": [\"Weekly Ladder\"]} becomes filter[referenceName]=Weekly%20Ladder.",
			},
			"fields": {
				Type:        "object",
				Description: "Sparse fieldsets. Keys are resource type names; values are arrays of attribute names to return.",
			},
			"include": {
				Type:        "array",
				Description: "Related resource names to include in the response.",
				Items:       &mcp.Property{Type: "string"},
			},
		},
		Required: []string{idKey},
	}
}

// readOnlyGameCenterTool returns the annotations shared by every Game
// Center content read tool.
func readOnlyGameCenterTool(title string) *mcp.ToolAnnotations {
	return &mcp.ToolAnnotations{
		Title:         title,
		ReadOnlyHint:  mcp.BoolPtr(true),
		OpenWorldHint: mcp.BoolPtr(true),
	}
}

// writeGameCenterTool returns the annotations shared by Game Center
// content create tools, which are neither destructive nor idempotent.
func writeGameCenterTool(title string) *mcp.ToolAnnotations {
	return &mcp.ToolAnnotations{
		Title:           title,
		ReadOnlyHint:    mcp.BoolPtr(false),
		DestructiveHint: mcp.BoolPtr(false),
		IdempotentHint:  mcp.BoolPtr(false),
		OpenWorldHint:   mcp.BoolPtr(true),
	}
}

// mutateGameCenterTool returns the annotations shared by Game Center
// content update and delete tools, which overwrite or remove state and
// can be retried safely.
func mutateGameCenterTool(title string) *mcp.ToolAnnotations {
	return &mcp.ToolAnnotations{
		Title:           title,
		ReadOnlyHint:    mcp.BoolPtr(false),
		DestructiveHint: mcp.BoolPtr(true),
		IdempotentHint:  mcp.BoolPtr(true),
		OpenWorldHint:   mcp.BoolPtr(true),
	}
}
