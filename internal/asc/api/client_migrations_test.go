package api

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"strings"
	"testing"
)

// TestClient_MigratedEndpointPaths verifies that operations deprecated in the
// App Store Connect API (v1 Game Center CRUD, v1 experiments, encryption
// declaration build linkage) now call their current replacements.
func TestClient_MigratedEndpointPaths(t *testing.T) {
	tests := []struct {
		name       string
		call       func(c *Client) error
		wantMethod string
		wantPath   string
		checkBody  func(t *testing.T, body []byte)
	}{
		{
			name: "list achievements uses gameCenterAchievementsV2",
			call: func(c *Client) error {
				_, err := c.ListGameCenterAchievements(context.Background(), "GC1", nil)
				return err
			},
			wantMethod: http.MethodGet,
			wantPath:   "/v1/gameCenterDetails/GC1/gameCenterAchievementsV2",
		},
		{
			name: "get achievement uses v2",
			call: func(c *Client) error {
				_, err := c.GetGameCenterAchievement(context.Background(), "A1")
				return err
			},
			wantMethod: http.MethodGet,
			wantPath:   "/v2/gameCenterAchievements/A1",
		},
		{
			name: "create achievement uses v2 with inline version",
			call: func(c *Client) error {
				req := &GameCenterAchievementCreateRequest{
					Data: GameCenterAchievementCreateData{
						Type: "gameCenterAchievements",
						Relationships: GameCenterAchievementCreateRelationships{
							GameCenterDetail: RelationshipData{Data: ResourceIdentifier{Type: "gameCenterDetails", ID: "GC1"}},
							Versions: RelationshipDataList{
								Data: []ResourceIdentifier{{Type: "gameCenterAchievementVersions", ID: "${new-version}"}},
							},
						},
					},
					Included: []GameCenterVersionInlineCreate{{Type: "gameCenterAchievementVersions", ID: "${new-version}"}},
				}
				_, err := c.CreateGameCenterAchievement(context.Background(), req)
				return err
			},
			wantMethod: http.MethodPost,
			wantPath:   "/v2/gameCenterAchievements",
			checkBody: func(t *testing.T, body []byte) {
				var req struct {
					Data struct {
						Relationships struct {
							Versions struct {
								Data []ResourceIdentifier `json:"data"`
							} `json:"versions"`
						} `json:"relationships"`
					} `json:"data"`
					Included []GameCenterVersionInlineCreate `json:"included"`
				}
				if err := json.Unmarshal(body, &req); err != nil {
					t.Fatalf("failed to unmarshal request body: %v", err)
				}
				if len(req.Data.Relationships.Versions.Data) != 1 {
					t.Fatalf("expected 1 versions relationship, got %d", len(req.Data.Relationships.Versions.Data))
				}
				if len(req.Included) != 1 || req.Included[0].Type != "gameCenterAchievementVersions" {
					t.Errorf("expected inline gameCenterAchievementVersions include, got %+v", req.Included)
				}
			},
		},
		{
			name: "update achievement uses v2",
			call: func(c *Client) error {
				req := &GameCenterAchievementUpdateRequest{
					Data: GameCenterAchievementUpdateData{Type: "gameCenterAchievements", ID: "A1"},
				}
				_, err := c.UpdateGameCenterAchievement(context.Background(), "A1", req)
				return err
			},
			wantMethod: http.MethodPatch,
			wantPath:   "/v2/gameCenterAchievements/A1",
		},
		{
			name: "delete achievement uses v2",
			call: func(c *Client) error {
				return c.DeleteGameCenterAchievement(context.Background(), "A1")
			},
			wantMethod: http.MethodDelete,
			wantPath:   "/v2/gameCenterAchievements/A1",
		},
		{
			name: "list leaderboards uses gameCenterLeaderboardsV2",
			call: func(c *Client) error {
				_, err := c.ListGameCenterLeaderboards(context.Background(), "GC1", nil)
				return err
			},
			wantMethod: http.MethodGet,
			wantPath:   "/v1/gameCenterDetails/GC1/gameCenterLeaderboardsV2",
		},
		{
			name: "get leaderboard uses v2",
			call: func(c *Client) error {
				_, err := c.GetGameCenterLeaderboard(context.Background(), "L1")
				return err
			},
			wantMethod: http.MethodGet,
			wantPath:   "/v2/gameCenterLeaderboards/L1",
		},
		{
			name: "create leaderboard uses v2 with default formatter",
			call: func(c *Client) error {
				req := &GameCenterLeaderboardCreateRequest{
					Data: GameCenterLeaderboardCreateData{
						Type: "gameCenterLeaderboards",
						Attributes: GameCenterLeaderboardCreateAttributes{
							DefaultFormatter: "INTEGER",
						},
						Relationships: GameCenterLeaderboardCreateRelationships{
							GameCenterDetail: RelationshipData{Data: ResourceIdentifier{Type: "gameCenterDetails", ID: "GC1"}},
							Versions: RelationshipDataList{
								Data: []ResourceIdentifier{{Type: "gameCenterLeaderboardVersions", ID: "${new-version}"}},
							},
						},
					},
					Included: []GameCenterVersionInlineCreate{{Type: "gameCenterLeaderboardVersions", ID: "${new-version}"}},
				}
				_, err := c.CreateGameCenterLeaderboard(context.Background(), req)
				return err
			},
			wantMethod: http.MethodPost,
			wantPath:   "/v2/gameCenterLeaderboards",
			checkBody: func(t *testing.T, body []byte) {
				var req struct {
					Data struct {
						Attributes struct {
							DefaultFormatter string `json:"defaultFormatter"`
						} `json:"attributes"`
					} `json:"data"`
				}
				if err := json.Unmarshal(body, &req); err != nil {
					t.Fatalf("failed to unmarshal request body: %v", err)
				}
				if req.Data.Attributes.DefaultFormatter != "INTEGER" {
					t.Errorf("defaultFormatter = %q, want INTEGER", req.Data.Attributes.DefaultFormatter)
				}
			},
		},
		{
			name: "update leaderboard uses v2",
			call: func(c *Client) error {
				req := &GameCenterLeaderboardUpdateRequest{
					Data: GameCenterLeaderboardUpdateData{Type: "gameCenterLeaderboards", ID: "L1"},
				}
				_, err := c.UpdateGameCenterLeaderboard(context.Background(), "L1", req)
				return err
			},
			wantMethod: http.MethodPatch,
			wantPath:   "/v2/gameCenterLeaderboards/L1",
		},
		{
			name: "delete leaderboard uses v2",
			call: func(c *Client) error {
				return c.DeleteGameCenterLeaderboard(context.Background(), "L1")
			},
			wantMethod: http.MethodDelete,
			wantPath:   "/v2/gameCenterLeaderboards/L1",
		},
		{
			name: "list experiments for version uses V2 relationship",
			call: func(c *Client) error {
				_, err := c.ListAppStoreVersionExperiments(context.Background(), "V1", nil)
				return err
			},
			wantMethod: http.MethodGet,
			wantPath:   "/v1/appStoreVersions/V1/appStoreVersionExperimentsV2",
		},
		{
			name: "list experiments for app uses V2 relationship",
			call: func(c *Client) error {
				_, err := c.ListAppStoreVersionExperimentsForApp(context.Background(), "APP1", nil)
				return err
			},
			wantMethod: http.MethodGet,
			wantPath:   "/v1/apps/APP1/appStoreVersionExperimentsV2",
		},
		{
			name: "get experiment uses v2",
			call: func(c *Client) error {
				_, err := c.GetAppStoreVersionExperiment(context.Background(), "E1")
				return err
			},
			wantMethod: http.MethodGet,
			wantPath:   "/v2/appStoreVersionExperiments/E1",
		},
		{
			name: "create experiment uses v2 with app relationship",
			call: func(c *Client) error {
				req := &AppStoreVersionExperimentCreateRequest{
					Data: AppStoreVersionExperimentCreateData{
						Type: "appStoreVersionExperiments",
						Attributes: AppStoreVersionExperimentCreateAttributes{
							Name:              "Test",
							Platform:          "IOS",
							TrafficProportion: 50,
						},
						Relationships: AppStoreVersionExperimentCreateRelationships{
							App: RelationshipData{Data: ResourceIdentifier{Type: "apps", ID: "APP1"}},
						},
					},
				}
				_, err := c.CreateAppStoreVersionExperiment(context.Background(), req)
				return err
			},
			wantMethod: http.MethodPost,
			wantPath:   "/v2/appStoreVersionExperiments",
			checkBody: func(t *testing.T, body []byte) {
				var req struct {
					Data struct {
						Attributes struct {
							Platform string `json:"platform"`
						} `json:"attributes"`
						Relationships struct {
							App RelationshipData `json:"app"`
						} `json:"relationships"`
					} `json:"data"`
				}
				if err := json.Unmarshal(body, &req); err != nil {
					t.Fatalf("failed to unmarshal request body: %v", err)
				}
				if req.Data.Attributes.Platform != "IOS" {
					t.Errorf("platform = %q, want IOS", req.Data.Attributes.Platform)
				}
				if req.Data.Relationships.App.Data.Type != "apps" || req.Data.Relationships.App.Data.ID != "APP1" {
					t.Errorf("app relationship = %+v, want apps/APP1", req.Data.Relationships.App.Data)
				}
			},
		},
		{
			name: "update experiment uses v2",
			call: func(c *Client) error {
				req := &AppStoreVersionExperimentUpdateRequest{
					Data: AppStoreVersionExperimentUpdateData{Type: "appStoreVersionExperiments", ID: "E1"},
				}
				_, err := c.UpdateAppStoreVersionExperiment(context.Background(), "E1", req)
				return err
			},
			wantMethod: http.MethodPatch,
			wantPath:   "/v2/appStoreVersionExperiments/E1",
		},
		{
			name: "delete experiment uses v2",
			call: func(c *Client) error {
				return c.DeleteAppStoreVersionExperiment(context.Background(), "E1")
			},
			wantMethod: http.MethodDelete,
			wantPath:   "/v2/appStoreVersionExperiments/E1",
		},
		{
			name: "assign build patches build encryption declaration relationship",
			call: func(c *Client) error {
				return c.AssignBuildToEncryptionDeclaration(context.Background(), "D1", "B1")
			},
			wantMethod: http.MethodPatch,
			wantPath:   "/v1/builds/B1/relationships/appEncryptionDeclaration",
			checkBody: func(t *testing.T, body []byte) {
				var req struct {
					Data ResourceIdentifier `json:"data"`
				}
				if err := json.Unmarshal(body, &req); err != nil {
					t.Fatalf("failed to unmarshal request body: %v", err)
				}
				if req.Data.Type != "appEncryptionDeclarations" || req.Data.ID != "D1" {
					t.Errorf("linkage = %+v, want appEncryptionDeclarations/D1", req.Data)
				}
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var gotMethod, gotPath string
			var gotBody []byte
			handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				gotMethod = r.Method
				gotPath = r.URL.Path
				gotBody, _ = io.ReadAll(r.Body)
				w.Header().Set("Content-Type", "application/json")
				if r.Method == http.MethodDelete || r.URL.Path == "/v1/builds/B1/relationships/appEncryptionDeclaration" {
					w.WriteHeader(http.StatusNoContent)
					return
				}
				w.WriteHeader(http.StatusOK)
				// List endpoints (the *V2 to-many relationships) return arrays;
				// everything else returns a single resource.
				if strings.HasSuffix(r.URL.Path, "V2") {
					w.Write([]byte(`{"data":[],"links":{}}`))
					return
				}
				w.Write([]byte(`{"data":{"type":"test","id":"1","attributes":{}}}`))
			})

			client, server := newTestClient(t, handler)
			defer server.Close()

			if err := tt.call(client); err != nil {
				t.Fatalf("unexpected error: %v", err)
			}

			if gotMethod != tt.wantMethod {
				t.Errorf("method = %s, want %s", gotMethod, tt.wantMethod)
			}
			if gotPath != tt.wantPath {
				t.Errorf("path = %s, want %s", gotPath, tt.wantPath)
			}
			if tt.checkBody != nil {
				tt.checkBody(t, gotBody)
			}
		})
	}
}
