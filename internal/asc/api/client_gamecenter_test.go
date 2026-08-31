package api

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"testing"
)

func TestClient_ListGameCenterLeaderboardSets(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/gameCenterDetails/gc-1/gameCenterLeaderboardSetsV2" {
			t.Errorf("path = %q, want /v1/gameCenterDetails/gc-1/gameCenterLeaderboardSetsV2", r.URL.Path)
		}
		if r.Method != http.MethodGet {
			t.Errorf("method = %q, want GET", r.Method)
		}
		if got := r.URL.Query().Get("limit"); got != "25" {
			t.Errorf("limit = %q, want 25", got)
		}

		json.NewEncoder(w).Encode(GameCenterLeaderboardSetsResponse{
			Data: []GameCenterLeaderboardSet{
				{
					Type: "gameCenterLeaderboardSets",
					ID:   "set-1",
					Attributes: GameCenterLeaderboardSetAttributes{
						ReferenceName:    "Season Ladders",
						VendorIdentifier: "com.example.sets.season",
					},
				},
			},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	resp, err := client.ListGameCenterLeaderboardSets(context.Background(), "gc-1", &ListOptions{Limit: 25})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 leaderboard set, got %d", len(resp.Data))
	}
	if resp.Data[0].Attributes.ReferenceName != "Season Ladders" {
		t.Errorf("reference name = %q, want Season Ladders", resp.Data[0].Attributes.ReferenceName)
	}
}

func TestClient_CreateGameCenterLeaderboardSet(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v2/gameCenterLeaderboardSets" {
			t.Errorf("path = %q, want /v2/gameCenterLeaderboardSets", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req GameCenterLeaderboardSetCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Type != "gameCenterLeaderboardSets" {
			t.Errorf("type = %q, want gameCenterLeaderboardSets", req.Data.Type)
		}
		if req.Data.Relationships.GameCenterDetail == nil || req.Data.Relationships.GameCenterDetail.Data.ID != "gc-1" {
			t.Error("expected gameCenterDetail relationship with id gc-1")
		}
		if len(req.Data.Relationships.Versions.Data) != 1 {
			t.Fatalf("expected 1 inline version linkage, got %d", len(req.Data.Relationships.Versions.Data))
		}
		if len(req.Included) != 1 || req.Included[0].Type != "gameCenterLeaderboardSetVersions" {
			t.Error("expected an included gameCenterLeaderboardSetVersions resource")
		}
		if req.Included[0].ID != req.Data.Relationships.Versions.Data[0].ID {
			t.Error("included version ID must match the versions linkage ID")
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(GameCenterLeaderboardSetResponse{
			Data: GameCenterLeaderboardSet{Type: "gameCenterLeaderboardSets", ID: "set-1"},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &GameCenterLeaderboardSetCreateRequest{
		Data: GameCenterLeaderboardSetCreateData{
			Type: "gameCenterLeaderboardSets",
			Attributes: GameCenterLeaderboardSetCreateAttributes{
				ReferenceName:    "Season Ladders",
				VendorIdentifier: "com.example.sets.season",
			},
			Relationships: GameCenterLeaderboardSetCreateRelationships{
				GameCenterDetail: &RelationshipData{
					Data: ResourceIdentifier{Type: "gameCenterDetails", ID: "gc-1"},
				},
				Versions: RelationshipDataList{
					Data: []ResourceIdentifier{{Type: "gameCenterLeaderboardSetVersions", ID: "${new-version}"}},
				},
			},
		},
		Included: []GameCenterVersionInlineCreate{
			{Type: "gameCenterLeaderboardSetVersions", ID: "${new-version}"},
		},
	}

	resp, err := client.CreateGameCenterLeaderboardSet(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp.Data.ID != "set-1" {
		t.Errorf("id = %q, want set-1", resp.Data.ID)
	}
}

func TestClient_GameCenterLeaderboardSetMembership(t *testing.T) {
	tests := []struct {
		name       string
		wantMethod string
		call       func(c *Client, linkages *RelationshipDataList) error
	}{
		{
			name:       "add",
			wantMethod: http.MethodPost,
			call: func(c *Client, linkages *RelationshipDataList) error {
				return c.AddGameCenterLeaderboardSetMembers(context.Background(), "set-1", linkages)
			},
		},
		{
			name:       "remove",
			wantMethod: http.MethodDelete,
			call: func(c *Client, linkages *RelationshipDataList) error {
				return c.RemoveGameCenterLeaderboardSetMembers(context.Background(), "set-1", linkages)
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				const want = "/v2/gameCenterLeaderboardSets/set-1/relationships/gameCenterLeaderboards"
				if r.URL.Path != want {
					t.Errorf("path = %q, want %s", r.URL.Path, want)
				}
				if r.Method != tt.wantMethod {
					t.Errorf("method = %q, want %s", r.Method, tt.wantMethod)
				}

				body, _ := io.ReadAll(r.Body)
				var req RelationshipDataList
				if err := json.Unmarshal(body, &req); err != nil {
					t.Fatalf("failed to decode request: %v", err)
				}
				if len(req.Data) != 2 {
					t.Fatalf("expected 2 linkages, got %d", len(req.Data))
				}
				if req.Data[0].Type != "gameCenterLeaderboards" {
					t.Errorf("linkage type = %q, want gameCenterLeaderboards", req.Data[0].Type)
				}

				w.WriteHeader(http.StatusNoContent)
			})

			client, server := newTestClient(t, handler)
			defer server.Close()

			linkages := &RelationshipDataList{
				Data: []ResourceIdentifier{
					{Type: "gameCenterLeaderboards", ID: "lb-1"},
					{Type: "gameCenterLeaderboards", ID: "lb-2"},
				},
			}
			if err := tt.call(client, linkages); err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
		})
	}
}

func TestClient_CreateGameCenterLeaderboardSetLocalization(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v2/gameCenterLeaderboardSetLocalizations" {
			t.Errorf("path = %q, want /v2/gameCenterLeaderboardSetLocalizations", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req GameCenterLeaderboardSetLocalizationCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Attributes.Locale != "en-US" {
			t.Errorf("locale = %q, want en-US", req.Data.Attributes.Locale)
		}
		if req.Data.Relationships.Version.Data.Type != "gameCenterLeaderboardSetVersions" {
			t.Errorf("version type = %q, want gameCenterLeaderboardSetVersions", req.Data.Relationships.Version.Data.Type)
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(GameCenterLeaderboardSetLocalizationResponse{
			Data: GameCenterLeaderboardSetLocalization{Type: "gameCenterLeaderboardSetLocalizations", ID: "loc-1"},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &GameCenterLeaderboardSetLocalizationCreateRequest{
		Data: GameCenterLeaderboardSetLocalizationCreateData{
			Type: "gameCenterLeaderboardSetLocalizations",
			Attributes: GameCenterLeaderboardSetLocalizationCreateAttributes{
				Locale: "en-US",
				Name:   "Season Ladders",
			},
			Relationships: GameCenterLeaderboardSetLocalizationCreateRelationships{
				Version: RelationshipData{
					Data: ResourceIdentifier{Type: "gameCenterLeaderboardSetVersions", ID: "ver-1"},
				},
			},
		},
	}

	resp, err := client.CreateGameCenterLeaderboardSetLocalization(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp.Data.ID != "loc-1" {
		t.Errorf("id = %q, want loc-1", resp.Data.ID)
	}
}

func TestClient_CreateGameCenterLeaderboardSetMemberLocalization(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/gameCenterLeaderboardSetMemberLocalizations" {
			t.Errorf("path = %q, want /v1/gameCenterLeaderboardSetMemberLocalizations", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req GameCenterLeaderboardSetMemberLocalizationCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Relationships.GameCenterLeaderboardSet.Data.ID != "set-1" {
			t.Errorf("set id = %q, want set-1", req.Data.Relationships.GameCenterLeaderboardSet.Data.ID)
		}
		if req.Data.Relationships.GameCenterLeaderboard.Data.ID != "lb-1" {
			t.Errorf("leaderboard id = %q, want lb-1", req.Data.Relationships.GameCenterLeaderboard.Data.ID)
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(GameCenterLeaderboardSetMemberLocalizationResponse{
			Data: GameCenterLeaderboardSetMemberLocalization{
				Type: "gameCenterLeaderboardSetMemberLocalizations",
				ID:   "mem-1",
			},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &GameCenterLeaderboardSetMemberLocalizationCreateRequest{
		Data: GameCenterLeaderboardSetMemberLocalizationCreateData{
			Type: "gameCenterLeaderboardSetMemberLocalizations",
			Attributes: GameCenterLeaderboardSetMemberLocalizationCreateAttributes{
				Locale: "en-US",
				Name:   "Weekly",
			},
			Relationships: GameCenterLeaderboardSetMemberLocalizationCreateRelationships{
				GameCenterLeaderboardSet: RelationshipData{
					Data: ResourceIdentifier{Type: "gameCenterLeaderboardSets", ID: "set-1"},
				},
				GameCenterLeaderboard: RelationshipData{
					Data: ResourceIdentifier{Type: "gameCenterLeaderboards", ID: "lb-1"},
				},
			},
		},
	}

	resp, err := client.CreateGameCenterLeaderboardSetMemberLocalization(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp.Data.ID != "mem-1" {
		t.Errorf("id = %q, want mem-1", resp.Data.ID)
	}
}

func TestClient_CreateGameCenterActivity(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/gameCenterActivities" {
			t.Errorf("path = %q, want /v1/gameCenterActivities", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req GameCenterActivityCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Attributes.PlayStyle != "SYNCHRONOUS" {
			t.Errorf("play style = %q, want SYNCHRONOUS", req.Data.Attributes.PlayStyle)
		}
		if req.Data.Relationships.Versions == nil || len(req.Data.Relationships.Versions.Data) != 1 {
			t.Fatal("expected one inline version linkage")
		}
		if len(req.Included) != 1 {
			t.Fatalf("expected 1 included version, got %d", len(req.Included))
		}
		if req.Included[0].Attributes == nil || req.Included[0].Attributes.FallbackURL != "https://example.com/play" {
			t.Error("expected fallbackUrl on the inline initial version")
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(GameCenterActivityResponse{
			Data: GameCenterActivity{Type: "gameCenterActivities", ID: "act-1"},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	minPlayers := 2
	req := &GameCenterActivityCreateRequest{
		Data: GameCenterActivityCreateData{
			Type: "gameCenterActivities",
			Attributes: GameCenterActivityCreateAttributes{
				ReferenceName:       "Co-op Raid",
				VendorIdentifier:    "com.example.activities.raid",
				PlayStyle:           "SYNCHRONOUS",
				MinimumPlayersCount: &minPlayers,
			},
			Relationships: GameCenterActivityCreateRelationships{
				GameCenterDetail: &RelationshipData{
					Data: ResourceIdentifier{Type: "gameCenterDetails", ID: "gc-1"},
				},
				Versions: &RelationshipDataList{
					Data: []ResourceIdentifier{{Type: "gameCenterActivityVersions", ID: "${new-version}"}},
				},
			},
		},
		Included: []GameCenterActivityVersionInlineCreate{
			{
				Type:       "gameCenterActivityVersions",
				ID:         "${new-version}",
				Attributes: &GameCenterActivityVersionInlineCreateAttributes{FallbackURL: "https://example.com/play"},
			},
		},
	}

	resp, err := client.CreateGameCenterActivity(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp.Data.ID != "act-1" {
		t.Errorf("id = %q, want act-1", resp.Data.ID)
	}
}

func TestClient_UpdateGameCenterActivityVersion(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/gameCenterActivityVersions/ver-1" {
			t.Errorf("path = %q, want /v1/gameCenterActivityVersions/ver-1", r.URL.Path)
		}
		if r.Method != http.MethodPatch {
			t.Errorf("method = %q, want PATCH", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req GameCenterActivityVersionUpdateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Attributes.FallbackURL != "https://example.com/next" {
			t.Errorf("fallbackUrl = %q, want https://example.com/next", req.Data.Attributes.FallbackURL)
		}

		json.NewEncoder(w).Encode(GameCenterActivityVersionResponse{
			Data: GameCenterActivityVersion{Type: "gameCenterActivityVersions", ID: "ver-1"},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &GameCenterActivityVersionUpdateRequest{
		Data: GameCenterActivityVersionUpdateData{
			Type: "gameCenterActivityVersions",
			ID:   "ver-1",
			Attributes: GameCenterActivityVersionUpdateAttributes{
				FallbackURL: "https://example.com/next",
			},
		},
	}

	resp, err := client.UpdateGameCenterActivityVersion(context.Background(), "ver-1", req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp.Data.ID != "ver-1" {
		t.Errorf("id = %q, want ver-1", resp.Data.ID)
	}
}

func TestClient_CreateGameCenterChallenge(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/gameCenterChallenges" {
			t.Errorf("path = %q, want /v1/gameCenterChallenges", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)

		// allowedDurations was removed from the API in 4.1; make sure the
		// request never reintroduces it.
		var raw map[string]any
		if err := json.Unmarshal(body, &raw); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		data := raw["data"].(map[string]any)
		attrs := data["attributes"].(map[string]any)
		if _, ok := attrs["allowedDurations"]; ok {
			t.Error("request must not contain allowedDurations")
		}

		var req GameCenterChallengeCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Attributes.ChallengeType != "LEADERBOARD" {
			t.Errorf("challenge type = %q, want LEADERBOARD", req.Data.Attributes.ChallengeType)
		}
		if req.Data.Relationships.LeaderboardV2 == nil || req.Data.Relationships.LeaderboardV2.Data.ID != "lb-1" {
			t.Error("expected leaderboardV2 relationship with id lb-1")
		}
		if len(req.Included) != 1 || req.Included[0].Type != "gameCenterChallengeVersions" {
			t.Error("expected an included gameCenterChallengeVersions resource")
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(GameCenterChallengeResponse{
			Data: GameCenterChallenge{Type: "gameCenterChallenges", ID: "chl-1"},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	repeatable := true
	req := &GameCenterChallengeCreateRequest{
		Data: GameCenterChallengeCreateData{
			Type: "gameCenterChallenges",
			Attributes: GameCenterChallengeCreateAttributes{
				ReferenceName:    "Weekend Sprint",
				VendorIdentifier: "com.example.challenges.sprint",
				ChallengeType:    "LEADERBOARD",
				Repeatable:       &repeatable,
			},
			Relationships: GameCenterChallengeCreateRelationships{
				GameCenterDetail: &RelationshipData{
					Data: ResourceIdentifier{Type: "gameCenterDetails", ID: "gc-1"},
				},
				LeaderboardV2: &RelationshipData{
					Data: ResourceIdentifier{Type: "gameCenterLeaderboards", ID: "lb-1"},
				},
				Versions: &RelationshipDataList{
					Data: []ResourceIdentifier{{Type: "gameCenterChallengeVersions", ID: "${new-version}"}},
				},
			},
		},
		Included: []GameCenterVersionInlineCreate{
			{Type: "gameCenterChallengeVersions", ID: "${new-version}"},
		},
	}

	resp, err := client.CreateGameCenterChallenge(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp.Data.ID != "chl-1" {
		t.Errorf("id = %q, want chl-1", resp.Data.ID)
	}
}

func TestClient_CreateGameCenterPlayerAchievementSubmission(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/gameCenterPlayerAchievementSubmissions" {
			t.Errorf("path = %q, want /v1/gameCenterPlayerAchievementSubmissions", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req GameCenterPlayerAchievementSubmissionCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Attributes.PercentageAchieved != 100 {
			t.Errorf("percentageAchieved = %d, want 100", req.Data.Attributes.PercentageAchieved)
		}
		if req.Data.Attributes.PreReleased == nil || !*req.Data.Attributes.PreReleased {
			t.Error("expected preReleased=true in request body")
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(GameCenterPlayerAchievementSubmissionResponse{
			Data: GameCenterPlayerAchievementSubmission{
				Type: "gameCenterPlayerAchievementSubmissions",
				ID:   "sub-1",
				Attributes: GameCenterPlayerAchievementSubmissionAttributes{
					PercentageAchieved: 100,
					PreReleased:        true,
				},
			},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &GameCenterPlayerAchievementSubmissionCreateRequest{
		Data: GameCenterPlayerAchievementSubmissionCreateData{
			Type: "gameCenterPlayerAchievementSubmissions",
			Attributes: GameCenterPlayerAchievementSubmissionCreateAttributes{
				BundleID:           "com.example.game",
				VendorIdentifier:   "com.example.achievements.first",
				ScopedPlayerID:     "player-1",
				PercentageAchieved: 100,
				PreReleased:        boolPtrTest(true),
			},
		},
	}

	resp, err := client.CreateGameCenterPlayerAchievementSubmission(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !resp.Data.Attributes.PreReleased {
		t.Error("expected preReleased submission in response")
	}
}

func TestClient_CreateGameCenterLeaderboardEntrySubmission(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/gameCenterLeaderboardEntrySubmissions" {
			t.Errorf("path = %q, want /v1/gameCenterLeaderboardEntrySubmissions", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req GameCenterLeaderboardEntrySubmissionCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Attributes.Score != "12345" {
			t.Errorf("score = %q, want 12345", req.Data.Attributes.Score)
		}
		if req.Data.Attributes.ScopedPlayerID != "player-1" {
			t.Errorf("scopedPlayerId = %q, want player-1", req.Data.Attributes.ScopedPlayerID)
		}
		if req.Data.Attributes.PreReleased == nil || *req.Data.Attributes.PreReleased {
			t.Error("expected preReleased=false in request body")
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(GameCenterLeaderboardEntrySubmissionResponse{
			Data: GameCenterLeaderboardEntrySubmission{
				Type:       "gameCenterLeaderboardEntrySubmissions",
				ID:         "entry-1",
				Attributes: GameCenterLeaderboardEntrySubmissionAttributes{Score: "12345"},
			},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &GameCenterLeaderboardEntrySubmissionCreateRequest{
		Data: GameCenterLeaderboardEntrySubmissionCreateData{
			Type: "gameCenterLeaderboardEntrySubmissions",
			Attributes: GameCenterLeaderboardEntrySubmissionCreateAttributes{
				BundleID:         "com.example.game",
				VendorIdentifier: "com.example.leaderboards.high",
				ScopedPlayerID:   "player-1",
				Score:            "12345",
				PreReleased:      boolPtrTest(false),
			},
		},
	}

	resp, err := client.CreateGameCenterLeaderboardEntrySubmission(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp.Data.ID != "entry-1" {
		t.Errorf("id = %q, want entry-1", resp.Data.ID)
	}
}

func TestClient_UploadGameCenterImages(t *testing.T) {
	tests := []struct {
		name         string
		collection   string
		resourceType string
		call         func(c *Client, body []byte) (*GameCenterImageResponse, error)
	}{
		{
			name:         "leaderboard set",
			collection:   "/v2/gameCenterLeaderboardSetImages",
			resourceType: "gameCenterLeaderboardSetImages",
			call: func(c *Client, body []byte) (*GameCenterImageResponse, error) {
				return c.UploadGameCenterLeaderboardSetImage(context.Background(), "loc-1", "set.png", body)
			},
		},
		{
			name:         "activity",
			collection:   "/v1/gameCenterActivityImages",
			resourceType: "gameCenterActivityImages",
			call: func(c *Client, body []byte) (*GameCenterImageResponse, error) {
				return c.UploadGameCenterActivityImage(context.Background(), "loc-1", "", "activity.png", body)
			},
		},
		{
			name:         "challenge",
			collection:   "/v1/gameCenterChallengeImages",
			resourceType: "gameCenterChallengeImages",
			call: func(c *Client, body []byte) (*GameCenterImageResponse, error) {
				return c.UploadGameCenterChallengeImage(context.Background(), "", "ver-1", "challenge.png", body)
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			payload := []byte("image-bytes")

			// uploadURL is filled in once httptest hands us its address;
			// the reservation response points the upload step back at the
			// same server.
			var uploadURL string
			var uploadedChunk []byte
			var committed bool

			handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				switch {
				case r.URL.Path == tt.collection && r.Method == http.MethodPost:
					body, _ := io.ReadAll(r.Body)
					var req GameCenterImageCreateRequest
					if err := json.Unmarshal(body, &req); err != nil {
						t.Fatalf("failed to decode reservation: %v", err)
					}
					if req.Data.Type != tt.resourceType {
						t.Errorf("type = %q, want %s", req.Data.Type, tt.resourceType)
					}
					if req.Data.Attributes.FileSize != len(payload) {
						t.Errorf("fileSize = %d, want %d", req.Data.Attributes.FileSize, len(payload))
					}
					if len(req.Data.Relationships) != 1 {
						t.Errorf("expected exactly 1 relationship, got %d", len(req.Data.Relationships))
					}

					w.WriteHeader(http.StatusCreated)
					json.NewEncoder(w).Encode(GameCenterImageResponse{
						Data: GameCenterImage{
							Type: tt.resourceType,
							ID:   "img-1",
							Attributes: GameCenterImageAttributes{
								UploadOperations: []UploadOperation{{
									Method: http.MethodPut,
									URL:    uploadURL,
									Offset: 0,
									Length: len(payload),
								}},
							},
						},
					})

				case r.URL.Path == "/upload":
					uploadedChunk, _ = io.ReadAll(r.Body)
					w.WriteHeader(http.StatusOK)

				case r.URL.Path == tt.collection+"/img-1" && r.Method == http.MethodPatch:
					body, _ := io.ReadAll(r.Body)
					var req GameCenterImageUpdateRequest
					if err := json.Unmarshal(body, &req); err != nil {
						t.Fatalf("failed to decode commit: %v", err)
					}
					if req.Data.Attributes.Uploaded == nil || !*req.Data.Attributes.Uploaded {
						t.Error("expected uploaded=true on commit")
					}
					committed = true

					json.NewEncoder(w).Encode(GameCenterImageResponse{
						Data: GameCenterImage{Type: tt.resourceType, ID: "img-1"},
					})

				default:
					t.Errorf("unexpected request: %s %s", r.Method, r.URL.Path)
					w.WriteHeader(http.StatusNotFound)
				}
			})

			client, server := newTestClient(t, handler)
			defer server.Close()
			uploadURL = server.URL + "/upload"

			resp, err := tt.call(client, payload)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if resp.Data.ID != "img-1" {
				t.Errorf("id = %q, want img-1", resp.Data.ID)
			}
			if string(uploadedChunk) != string(payload) {
				t.Errorf("uploaded chunk = %q, want %q", uploadedChunk, payload)
			}
			if !committed {
				t.Error("expected the commit PATCH to be issued")
			}
		})
	}
}

func TestClient_DeleteGameCenterContent(t *testing.T) {
	tests := []struct {
		name     string
		wantPath string
		call     func(c *Client) error
	}{
		{
			name:     "leaderboard set",
			wantPath: "/v2/gameCenterLeaderboardSets/set-1",
			call:     func(c *Client) error { return c.DeleteGameCenterLeaderboardSet(context.Background(), "set-1") },
		},
		{
			name:     "leaderboard set localization",
			wantPath: "/v2/gameCenterLeaderboardSetLocalizations/loc-1",
			call: func(c *Client) error {
				return c.DeleteGameCenterLeaderboardSetLocalization(context.Background(), "loc-1")
			},
		},
		{
			name:     "member localization",
			wantPath: "/v1/gameCenterLeaderboardSetMemberLocalizations/mem-1",
			call: func(c *Client) error {
				return c.DeleteGameCenterLeaderboardSetMemberLocalization(context.Background(), "mem-1")
			},
		},
		{
			name:     "activity",
			wantPath: "/v1/gameCenterActivities/act-1",
			call:     func(c *Client) error { return c.DeleteGameCenterActivity(context.Background(), "act-1") },
		},
		{
			name:     "activity localization",
			wantPath: "/v1/gameCenterActivityLocalizations/loc-1",
			call: func(c *Client) error {
				return c.DeleteGameCenterActivityLocalization(context.Background(), "loc-1")
			},
		},
		{
			name:     "challenge",
			wantPath: "/v1/gameCenterChallenges/chl-1",
			call:     func(c *Client) error { return c.DeleteGameCenterChallenge(context.Background(), "chl-1") },
		},
		{
			name:     "challenge localization",
			wantPath: "/v1/gameCenterChallengeLocalizations/loc-1",
			call: func(c *Client) error {
				return c.DeleteGameCenterChallengeLocalization(context.Background(), "loc-1")
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				if r.URL.Path != tt.wantPath {
					t.Errorf("path = %q, want %s", r.URL.Path, tt.wantPath)
				}
				if r.Method != http.MethodDelete {
					t.Errorf("method = %q, want DELETE", r.Method)
				}
				w.WriteHeader(http.StatusNoContent)
			})

			client, server := newTestClient(t, handler)
			defer server.Close()

			if err := tt.call(client); err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
		})
	}
}

func TestClient_GameCenterVersionCollections(t *testing.T) {
	tests := []struct {
		name     string
		wantPath string
		call     func(c *Client) error
	}{
		{
			name:     "leaderboard set versions",
			wantPath: "/v2/gameCenterLeaderboardSets/set-1/versions",
			call: func(c *Client) error {
				_, err := c.ListGameCenterLeaderboardSetVersions(context.Background(), "set-1", nil)
				return err
			},
		},
		{
			name:     "leaderboard set localizations",
			wantPath: "/v2/gameCenterLeaderboardSetVersions/ver-1/localizations",
			call: func(c *Client) error {
				_, err := c.ListGameCenterLeaderboardSetLocalizations(context.Background(), "ver-1", nil)
				return err
			},
		},
		{
			name:     "leaderboard set members",
			wantPath: "/v2/gameCenterLeaderboardSets/set-1/gameCenterLeaderboards",
			call: func(c *Client) error {
				_, err := c.ListGameCenterLeaderboardSetMembers(context.Background(), "set-1", nil)
				return err
			},
		},
		{
			name:     "activities",
			wantPath: "/v1/gameCenterDetails/gc-1/gameCenterActivities",
			call: func(c *Client) error {
				_, err := c.ListGameCenterActivities(context.Background(), "gc-1", nil)
				return err
			},
		},
		{
			name:     "activity versions",
			wantPath: "/v1/gameCenterActivities/act-1/versions",
			call: func(c *Client) error {
				_, err := c.ListGameCenterActivityVersions(context.Background(), "act-1", nil)
				return err
			},
		},
		{
			name:     "activity localizations",
			wantPath: "/v1/gameCenterActivityVersions/ver-1/localizations",
			call: func(c *Client) error {
				_, err := c.ListGameCenterActivityLocalizations(context.Background(), "ver-1", nil)
				return err
			},
		},
		{
			name:     "challenges",
			wantPath: "/v1/gameCenterDetails/gc-1/gameCenterChallenges",
			call: func(c *Client) error {
				_, err := c.ListGameCenterChallenges(context.Background(), "gc-1", nil)
				return err
			},
		},
		{
			name:     "challenge versions",
			wantPath: "/v1/gameCenterChallenges/chl-1/versions",
			call: func(c *Client) error {
				_, err := c.ListGameCenterChallengeVersions(context.Background(), "chl-1", nil)
				return err
			},
		},
		{
			name:     "challenge localizations",
			wantPath: "/v1/gameCenterChallengeVersions/ver-1/localizations",
			call: func(c *Client) error {
				_, err := c.ListGameCenterChallengeLocalizations(context.Background(), "ver-1", nil)
				return err
			},
		},
		{
			name:     "member localizations",
			wantPath: "/v1/gameCenterLeaderboardSetMemberLocalizations",
			call: func(c *Client) error {
				_, err := c.ListGameCenterLeaderboardSetMemberLocalizations(context.Background(), &ListOptions{
					Filter: map[string][]string{"gameCenterLeaderboardSet": {"set-1"}},
				})
				return err
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				if r.URL.Path != tt.wantPath {
					t.Errorf("path = %q, want %s", r.URL.Path, tt.wantPath)
				}
				if r.Method != http.MethodGet {
					t.Errorf("method = %q, want GET", r.Method)
				}
				w.Write([]byte(`{"data":[]}`))
			})

			client, server := newTestClient(t, handler)
			defer server.Close()

			if err := tt.call(client); err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
		})
	}
}
