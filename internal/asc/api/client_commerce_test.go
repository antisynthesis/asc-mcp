package api

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"testing"
)

// TestCommerceReadPaths exercises every commerce read endpoint, asserting
// the client hits the path and method the App Store Connect OpenAPI spec
// documents for it.
func TestCommerceReadPaths(t *testing.T) {
	tests := []struct {
		name string
		path string
		list bool
		call func(*Client) error
	}{
		{
			name: "get in-app purchase version",
			path: "/v1/inAppPurchaseVersions/ver-1",
			call: func(c *Client) error {
				_, err := c.GetInAppPurchaseVersion(context.Background(), "ver-1")
				return err
			},
		},
		{
			name: "list in-app purchase versions",
			list: true,
			path: "/v2/inAppPurchases/iap-1/versions",
			call: func(c *Client) error {
				_, err := c.ListInAppPurchaseVersions(context.Background(), "iap-1", nil)
				return err
			},
		},
		{
			name: "list in-app purchase version localizations",
			list: true,
			path: "/v1/inAppPurchaseVersions/ver-1/localizations",
			call: func(c *Client) error {
				_, err := c.ListInAppPurchaseVersionLocalizations(context.Background(), "ver-1", nil)
				return err
			},
		},
		{
			name: "list in-app purchase version images",
			list: true,
			path: "/v1/inAppPurchaseVersions/ver-1/images",
			call: func(c *Client) error {
				_, err := c.ListInAppPurchaseVersionImages(context.Background(), "ver-1", nil)
				return err
			},
		},
		{
			name: "get subscription version",
			path: "/v1/subscriptionVersions/ver-2",
			call: func(c *Client) error {
				_, err := c.GetSubscriptionVersion(context.Background(), "ver-2")
				return err
			},
		},
		{
			name: "list subscription versions",
			list: true,
			path: "/v1/subscriptions/sub-1/versions",
			call: func(c *Client) error {
				_, err := c.ListSubscriptionVersions(context.Background(), "sub-1", nil)
				return err
			},
		},
		{
			name: "list subscription version localizations",
			list: true,
			path: "/v1/subscriptionVersions/ver-2/localizations",
			call: func(c *Client) error {
				_, err := c.ListSubscriptionVersionLocalizations(context.Background(), "ver-2", nil)
				return err
			},
		},
		{
			name: "list subscription version images",
			list: true,
			path: "/v1/subscriptionVersions/ver-2/images",
			call: func(c *Client) error {
				_, err := c.ListSubscriptionVersionImages(context.Background(), "ver-2", nil)
				return err
			},
		},
		{
			name: "get subscription group version",
			path: "/v1/subscriptionGroupVersions/ver-3",
			call: func(c *Client) error {
				_, err := c.GetSubscriptionGroupVersion(context.Background(), "ver-3")
				return err
			},
		},
		{
			name: "list subscription group versions",
			list: true,
			path: "/v1/subscriptionGroups/group-1/versions",
			call: func(c *Client) error {
				_, err := c.ListSubscriptionGroupVersions(context.Background(), "group-1", nil)
				return err
			},
		},
		{
			name: "list subscription group version localizations",
			list: true,
			path: "/v1/subscriptionGroupVersions/ver-3/localizations",
			call: func(c *Client) error {
				_, err := c.ListSubscriptionGroupVersionLocalizations(context.Background(), "ver-3", nil)
				return err
			},
		},
		{
			name: "list in-app purchase offer codes",
			list: true,
			path: "/v2/inAppPurchases/iap-1/offerCodes",
			call: func(c *Client) error {
				_, err := c.ListInAppPurchaseOfferCodes(context.Background(), "iap-1", nil)
				return err
			},
		},
		{
			name: "get in-app purchase offer code",
			path: "/v1/inAppPurchaseOfferCodes/offer-1",
			call: func(c *Client) error {
				_, err := c.GetInAppPurchaseOfferCode(context.Background(), "offer-1")
				return err
			},
		},
		{
			name: "list in-app purchase offer code prices",
			list: true,
			path: "/v1/inAppPurchaseOfferCodes/offer-1/prices",
			call: func(c *Client) error {
				_, err := c.ListInAppPurchaseOfferCodePrices(context.Background(), "offer-1", nil)
				return err
			},
		},
		{
			name: "list in-app purchase offer code custom codes",
			list: true,
			path: "/v1/inAppPurchaseOfferCodes/offer-1/customCodes",
			call: func(c *Client) error {
				_, err := c.ListInAppPurchaseOfferCodeCustomCodes(context.Background(), "offer-1", nil)
				return err
			},
		},
		{
			name: "list in-app purchase offer code one-time-use codes",
			list: true,
			path: "/v1/inAppPurchaseOfferCodes/offer-1/oneTimeUseCodes",
			call: func(c *Client) error {
				_, err := c.ListInAppPurchaseOfferCodeOneTimeUseCodes(context.Background(), "offer-1", nil)
				return err
			},
		},
		{
			name: "list in-app purchase price points",
			list: true,
			path: "/v2/inAppPurchases/iap-1/pricePoints",
			call: func(c *Client) error {
				_, err := c.ListInAppPurchasePricePoints(context.Background(), "iap-1", nil)
				return err
			},
		},
		{
			name: "get in-app purchase price schedule",
			path: "/v2/inAppPurchases/iap-1/iapPriceSchedule",
			call: func(c *Client) error {
				_, err := c.GetInAppPurchasePriceSchedule(context.Background(), "iap-1")
				return err
			},
		},
		{
			name: "list in-app purchase price schedule manual prices",
			list: true,
			path: "/v1/inAppPurchasePriceSchedules/sched-1/manualPrices",
			call: func(c *Client) error {
				_, err := c.ListInAppPurchasePriceSchedulePrices(context.Background(), "sched-1", false, nil)
				return err
			},
		},
		{
			name: "list in-app purchase price schedule automatic prices",
			list: true,
			path: "/v1/inAppPurchasePriceSchedules/sched-1/automaticPrices",
			call: func(c *Client) error {
				_, err := c.ListInAppPurchasePriceSchedulePrices(context.Background(), "sched-1", true, nil)
				return err
			},
		},
		{
			name: "get in-app purchase availability",
			path: "/v2/inAppPurchases/iap-1/inAppPurchaseAvailability",
			call: func(c *Client) error {
				_, err := c.GetInAppPurchaseAvailability(context.Background(), "iap-1")
				return err
			},
		},
		{
			name: "list in-app purchase available territories",
			list: true,
			path: "/v1/inAppPurchaseAvailabilities/avail-1/availableTerritories",
			call: func(c *Client) error {
				_, err := c.ListInAppPurchaseAvailableTerritories(context.Background(), "avail-1", nil)
				return err
			},
		},
		{
			name: "list subscription plan availabilities",
			list: true,
			path: "/v1/subscriptions/sub-1/planAvailabilities",
			call: func(c *Client) error {
				_, err := c.ListSubscriptionPlanAvailabilities(context.Background(), "sub-1", nil)
				return err
			},
		},
		{
			name: "get subscription plan availability",
			path: "/v1/subscriptionPlanAvailabilities/plan-1",
			call: func(c *Client) error {
				_, err := c.GetSubscriptionPlanAvailability(context.Background(), "plan-1")
				return err
			},
		},
		{
			name: "list subscription plan available territories",
			list: true,
			path: "/v1/subscriptionPlanAvailabilities/plan-1/availableTerritories",
			call: func(c *Client) error {
				_, err := c.ListSubscriptionPlanAvailableTerritories(context.Background(), "plan-1", nil)
				return err
			},
		},
		{
			name: "get subscription price point",
			path: "/v1/subscriptionPricePoints/pp-1",
			call: func(c *Client) error {
				_, err := c.GetSubscriptionPricePoint(context.Background(), "pp-1")
				return err
			},
		},
		{
			name: "list subscription price point equalizations",
			list: true,
			path: "/v1/subscriptionPricePoints/pp-1/equalizations",
			call: func(c *Client) error {
				_, err := c.ListSubscriptionPricePointEqualizations(context.Background(), "pp-1", nil)
				return err
			},
		},
		{
			name: "get app price point",
			path: "/v3/appPricePoints/pp-2",
			call: func(c *Client) error {
				_, err := c.GetAppPricePoint(context.Background(), "pp-2")
				return err
			},
		},
		{
			name: "list app price point equalizations",
			list: true,
			path: "/v3/appPricePoints/pp-2/equalizations",
			call: func(c *Client) error {
				_, err := c.ListAppPricePointEqualizations(context.Background(), "pp-2", nil)
				return err
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				if r.URL.Path != tt.path {
					t.Errorf("path = %q, want %s", r.URL.Path, tt.path)
				}
				if r.Method != http.MethodGet {
					t.Errorf("method = %q, want GET", r.Method)
				}
				if tt.list {
					w.Write([]byte(`{"data":[],"links":{}}`))
					return
				}
				w.Write([]byte(`{"data":{"type":"x","id":"1"}}`))
			})

			client, server := newTestClient(t, handler)
			defer server.Close()

			if err := tt.call(client); err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
		})
	}
}

func TestClient_CreateInAppPurchaseVersion(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/inAppPurchaseVersions" {
			t.Errorf("path = %q, want /v1/inAppPurchaseVersions", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req InAppPurchaseVersionCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Type != "inAppPurchaseVersions" {
			t.Errorf("type = %q, want inAppPurchaseVersions", req.Data.Type)
		}
		if req.Data.Relationships.InAppPurchase.Data.Type != "inAppPurchases" {
			t.Errorf("relationship type = %q, want inAppPurchases", req.Data.Relationships.InAppPurchase.Data.Type)
		}
		if req.Data.Relationships.InAppPurchase.Data.ID != "iap-1" {
			t.Errorf("in-app purchase id = %q, want iap-1", req.Data.Relationships.InAppPurchase.Data.ID)
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(InAppPurchaseVersionResponse{
			Data: InAppPurchaseVersion{
				Type:       "inAppPurchaseVersions",
				ID:         "ver-1",
				Attributes: CommerceVersionAttributes{Version: 2, State: "PREPARE_FOR_SUBMISSION"},
			},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &InAppPurchaseVersionCreateRequest{
		Data: InAppPurchaseVersionCreateData{
			Type: "inAppPurchaseVersions",
			Relationships: InAppPurchaseVersionCreateRelationships{
				InAppPurchase: RelationshipData{
					Data: ResourceIdentifier{Type: "inAppPurchases", ID: "iap-1"},
				},
			},
		},
	}

	resp, err := client.CreateInAppPurchaseVersion(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "ver-1" {
		t.Errorf("id = %q, want ver-1", resp.Data.ID)
	}
	if resp.Data.Attributes.Version != 2 {
		t.Errorf("version = %d, want 2", resp.Data.Attributes.Version)
	}
}

func TestClient_CreateSubscriptionVersion(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/subscriptionVersions" {
			t.Errorf("path = %q, want /v1/subscriptionVersions", r.URL.Path)
		}

		body, _ := io.ReadAll(r.Body)
		var req SubscriptionVersionCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Relationships.Subscription.Data.ID != "sub-1" {
			t.Errorf("subscription id = %q, want sub-1", req.Data.Relationships.Subscription.Data.ID)
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(SubscriptionVersionResponse{
			Data: SubscriptionVersion{Type: "subscriptionVersions", ID: "ver-2"},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &SubscriptionVersionCreateRequest{
		Data: SubscriptionVersionCreateData{
			Type: "subscriptionVersions",
			Relationships: SubscriptionVersionCreateRelationships{
				Subscription: RelationshipData{
					Data: ResourceIdentifier{Type: "subscriptions", ID: "sub-1"},
				},
			},
		},
	}

	resp, err := client.CreateSubscriptionVersion(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "ver-2" {
		t.Errorf("id = %q, want ver-2", resp.Data.ID)
	}
}

func TestClient_CreateSubscriptionGroupVersion(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/subscriptionGroupVersions" {
			t.Errorf("path = %q, want /v1/subscriptionGroupVersions", r.URL.Path)
		}

		body, _ := io.ReadAll(r.Body)
		var req SubscriptionGroupVersionCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Relationships.SubscriptionGroup.Data.Type != "subscriptionGroups" {
			t.Errorf("relationship type = %q, want subscriptionGroups", req.Data.Relationships.SubscriptionGroup.Data.Type)
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(SubscriptionGroupVersionResponse{
			Data: SubscriptionGroupVersion{Type: "subscriptionGroupVersions", ID: "ver-3"},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &SubscriptionGroupVersionCreateRequest{
		Data: SubscriptionGroupVersionCreateData{
			Type: "subscriptionGroupVersions",
			Relationships: SubscriptionGroupVersionCreateRelationships{
				SubscriptionGroup: RelationshipData{
					Data: ResourceIdentifier{Type: "subscriptionGroups", ID: "group-1"},
				},
			},
		},
	}

	resp, err := client.CreateSubscriptionGroupVersion(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "ver-3" {
		t.Errorf("id = %q, want ver-3", resp.Data.ID)
	}
}

func TestClient_CreateInAppPurchaseLocalization(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v2/inAppPurchaseLocalizations" {
			t.Errorf("path = %q, want /v2/inAppPurchaseLocalizations", r.URL.Path)
		}

		body, _ := io.ReadAll(r.Body)
		var req InAppPurchaseLocalizationCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Attributes.Locale != "en-US" {
			t.Errorf("locale = %q, want en-US", req.Data.Attributes.Locale)
		}
		// The v2 collection points at a version, not at the product.
		if req.Data.Relationships.Version.Data.Type != "inAppPurchaseVersions" {
			t.Errorf("relationship type = %q, want inAppPurchaseVersions", req.Data.Relationships.Version.Data.Type)
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(InAppPurchaseLocalizationResponse{
			Data: InAppPurchaseLocalization{
				Type: "inAppPurchaseLocalizations",
				ID:   "loc-1",
				Attributes: InAppPurchaseLocalizationAttributes{
					Name:   "Gold Coins",
					Locale: "en-US",
				},
			},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &InAppPurchaseLocalizationCreateRequest{
		Data: InAppPurchaseLocalizationCreateData{
			Type: "inAppPurchaseLocalizations",
			Attributes: InAppPurchaseLocalizationCreateAttributes{
				Name:   "Gold Coins",
				Locale: "en-US",
			},
			Relationships: InAppPurchaseLocalizationCreateRelationships{
				Version: RelationshipData{
					Data: ResourceIdentifier{Type: "inAppPurchaseVersions", ID: "ver-1"},
				},
			},
		},
	}

	resp, err := client.CreateInAppPurchaseLocalization(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.Attributes.Name != "Gold Coins" {
		t.Errorf("name = %q, want Gold Coins", resp.Data.Attributes.Name)
	}
}

func TestClient_UpdateSubscriptionImage(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v2/subscriptionImages/img-1" {
			t.Errorf("path = %q, want /v2/subscriptionImages/img-1", r.URL.Path)
		}
		if r.Method != http.MethodPatch {
			t.Errorf("method = %q, want PATCH", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req SubscriptionImageUpdateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Attributes.Uploaded == nil || !*req.Data.Attributes.Uploaded {
			t.Error("expected uploaded=true in the request body")
		}

		json.NewEncoder(w).Encode(SubscriptionImageResponse{
			Data: SubscriptionImage{Type: "subscriptionImages", ID: "img-1"},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &SubscriptionImageUpdateRequest{
		Data: SubscriptionImageUpdateData{
			Type:       "subscriptionImages",
			ID:         "img-1",
			Attributes: CommerceImageUpdateAttributes{Uploaded: boolPtrTest(true)},
		},
	}

	if _, err := client.UpdateSubscriptionImage(context.Background(), "img-1", req); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestClient_CreateInAppPurchaseOfferCode(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/inAppPurchaseOfferCodes" {
			t.Errorf("path = %q, want /v1/inAppPurchaseOfferCodes", r.URL.Path)
		}

		body, _ := io.ReadAll(r.Body)
		var req InAppPurchaseOfferCodeCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if len(req.Data.Attributes.CustomerEligibilities) != 1 {
			t.Fatalf("expected 1 customer eligibility, got %d", len(req.Data.Attributes.CustomerEligibilities))
		}
		if len(req.Data.Relationships.Prices.Data) != 1 {
			t.Fatalf("expected 1 price linkage, got %d", len(req.Data.Relationships.Prices.Data))
		}
		if len(req.Included) != 1 {
			t.Fatalf("expected 1 included price, got %d", len(req.Included))
		}
		// The linkage and the inline resource must agree on the ID.
		if req.Data.Relationships.Prices.Data[0].ID != req.Included[0].ID {
			t.Errorf("price linkage id = %q, included id = %q", req.Data.Relationships.Prices.Data[0].ID, req.Included[0].ID)
		}
		if req.Included[0].Relationships.PricePoint.Data.Type != "inAppPurchasePricePoints" {
			t.Errorf("price point type = %q, want inAppPurchasePricePoints", req.Included[0].Relationships.PricePoint.Data.Type)
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(InAppPurchaseOfferCodeResponse{
			Data: InAppPurchaseOfferCode{
				Type:       "inAppPurchaseOfferCodes",
				ID:         "offer-1",
				Attributes: InAppPurchaseOfferCodeAttributes{Name: "Launch", Active: true},
			},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &InAppPurchaseOfferCodeCreateRequest{
		Data: InAppPurchaseOfferCodeCreateData{
			Type: "inAppPurchaseOfferCodes",
			Attributes: InAppPurchaseOfferCodeCreateAttributes{
				Name:                  "Launch",
				CustomerEligibilities: []string{"NEW"},
			},
			Relationships: InAppPurchaseOfferCodeCreateRelationships{
				InAppPurchase: RelationshipData{
					Data: ResourceIdentifier{Type: "inAppPurchases", ID: "iap-1"},
				},
				Prices: RelationshipDataList{
					Data: []ResourceIdentifier{{Type: "inAppPurchaseOfferPrices", ID: "price-1"}},
				},
			},
		},
		Included: []InAppPurchaseOfferPriceInlineCreate{
			{
				Type: "inAppPurchaseOfferPrices",
				ID:   "price-1",
				Relationships: &InAppPurchaseOfferPriceInlineCreateRelationships{
					Territory: RelationshipData{
						Data: ResourceIdentifier{Type: "territories", ID: "USA"},
					},
					PricePoint: RelationshipData{
						Data: ResourceIdentifier{Type: "inAppPurchasePricePoints", ID: "pp-1"},
					},
				},
			},
		},
	}

	resp, err := client.CreateInAppPurchaseOfferCode(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "offer-1" {
		t.Errorf("id = %q, want offer-1", resp.Data.ID)
	}
}

func TestClient_CreateInAppPurchaseOfferCodeOneTimeUseCode(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/inAppPurchaseOfferCodeOneTimeUseCodes" {
			t.Errorf("path = %q, want /v1/inAppPurchaseOfferCodeOneTimeUseCodes", r.URL.Path)
		}

		body, _ := io.ReadAll(r.Body)
		var req InAppPurchaseOfferCodeOneTimeUseCodeCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Attributes.NumberOfCodes != 500 {
			t.Errorf("number of codes = %d, want 500", req.Data.Attributes.NumberOfCodes)
		}
		if req.Data.Attributes.ExpirationDate != "2026-12-31" {
			t.Errorf("expiration = %q, want 2026-12-31", req.Data.Attributes.ExpirationDate)
		}
		if req.Data.Relationships.OfferCode.Data.ID != "offer-1" {
			t.Errorf("offer code id = %q, want offer-1", req.Data.Relationships.OfferCode.Data.ID)
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(InAppPurchaseOfferCodeOneTimeUseCodeResponse{
			Data: InAppPurchaseOfferCodeOneTimeUseCode{
				Type: "inAppPurchaseOfferCodeOneTimeUseCodes",
				ID:   "batch-1",
			},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &InAppPurchaseOfferCodeOneTimeUseCodeCreateRequest{
		Data: InAppPurchaseOfferCodeOneTimeUseCodeCreateData{
			Type: "inAppPurchaseOfferCodeOneTimeUseCodes",
			Attributes: InAppPurchaseOfferCodeOneTimeUseCodeCreateAttributes{
				NumberOfCodes:  500,
				ExpirationDate: "2026-12-31",
			},
			Relationships: OfferCodeRelationship{
				OfferCode: RelationshipData{
					Data: ResourceIdentifier{Type: "inAppPurchaseOfferCodes", ID: "offer-1"},
				},
			},
		},
	}

	if _, err := client.CreateInAppPurchaseOfferCodeOneTimeUseCode(context.Background(), req); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestClient_GetInAppPurchaseOfferCodeOneTimeUseCodeValues(t *testing.T) {
	const csv = "code\nABC123\nDEF456\n"

	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/inAppPurchaseOfferCodeOneTimeUseCodes/batch-1/values" {
			t.Errorf("path = %q, want /v1/inAppPurchaseOfferCodeOneTimeUseCodes/batch-1/values", r.URL.Path)
		}
		w.Header().Set("Content-Type", "text/csv")
		w.Write([]byte(csv))
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	got, err := client.GetInAppPurchaseOfferCodeOneTimeUseCodeValues(context.Background(), "batch-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if got != csv {
		t.Errorf("csv = %q, want %q", got, csv)
	}
}

func TestClient_CreateInAppPurchasePriceSchedule(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/inAppPurchasePriceSchedules" {
			t.Errorf("path = %q, want /v1/inAppPurchasePriceSchedules", r.URL.Path)
		}

		body, _ := io.ReadAll(r.Body)
		var req InAppPurchasePriceScheduleCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Relationships.BaseTerritory.Data.ID != "USA" {
			t.Errorf("base territory = %q, want USA", req.Data.Relationships.BaseTerritory.Data.ID)
		}
		if len(req.Included) != 1 {
			t.Fatalf("expected 1 inline price, got %d", len(req.Included))
		}
		if req.Included[0].Relationships.InAppPurchasePricePoint.Data.ID != "pp-1" {
			t.Errorf("price point = %q, want pp-1", req.Included[0].Relationships.InAppPurchasePricePoint.Data.ID)
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(InAppPurchasePriceScheduleResponse{
			Data: InAppPurchasePriceSchedule{Type: "inAppPurchasePriceSchedules", ID: "sched-1"},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &InAppPurchasePriceScheduleCreateRequest{
		Data: InAppPurchasePriceScheduleCreateData{
			Type: "inAppPurchasePriceSchedules",
			Relationships: InAppPurchasePriceScheduleCreateRelationships{
				InAppPurchase: RelationshipData{
					Data: ResourceIdentifier{Type: "inAppPurchases", ID: "iap-1"},
				},
				BaseTerritory: RelationshipData{
					Data: ResourceIdentifier{Type: "territories", ID: "USA"},
				},
				ManualPrices: RelationshipDataList{
					Data: []ResourceIdentifier{{Type: "inAppPurchasePrices", ID: "price-1"}},
				},
			},
		},
		Included: []InAppPurchasePriceInlineCreate{
			{
				Type: "inAppPurchasePrices",
				ID:   "price-1",
				Relationships: &InAppPurchasePriceInlineCreateRelationships{
					InAppPurchasePricePoint: RelationshipData{
						Data: ResourceIdentifier{Type: "inAppPurchasePricePoints", ID: "pp-1"},
					},
				},
			},
		},
	}

	resp, err := client.CreateInAppPurchasePriceSchedule(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "sched-1" {
		t.Errorf("id = %q, want sched-1", resp.Data.ID)
	}
}

func TestClient_CreateAppPriceSchedule(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/appPriceSchedules" {
			t.Errorf("path = %q, want /v1/appPriceSchedules", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("method = %q, want POST", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req AppPriceScheduleCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Relationships.App.Data.ID != "app-1" {
			t.Errorf("app id = %q, want app-1", req.Data.Relationships.App.Data.ID)
		}
		if req.Data.Relationships.BaseTerritory.Data.Type != "territories" {
			t.Errorf("base territory type = %q, want territories", req.Data.Relationships.BaseTerritory.Data.Type)
		}
		if len(req.Included) != 1 {
			t.Fatalf("expected 1 inline price, got %d", len(req.Included))
		}
		if req.Included[0].Attributes == nil || req.Included[0].Attributes.StartDate != "2026-01-01" {
			t.Error("expected inline price to carry startDate 2026-01-01")
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(AppPriceScheduleResponse{
			Data: AppPriceSchedule{Type: "appPriceSchedules", ID: "app-sched-1"},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &AppPriceScheduleCreateRequest{
		Data: AppPriceScheduleCreateData{
			Type: "appPriceSchedules",
			Relationships: AppPriceScheduleCreateRelationships{
				App: RelationshipData{
					Data: ResourceIdentifier{Type: "apps", ID: "app-1"},
				},
				BaseTerritory: RelationshipData{
					Data: ResourceIdentifier{Type: "territories", ID: "USA"},
				},
				ManualPrices: RelationshipDataList{
					Data: []ResourceIdentifier{{Type: "appPrices", ID: "price-1"}},
				},
			},
		},
		Included: []AppPriceInlineCreate{
			{
				Type:       "appPrices",
				ID:         "price-1",
				Attributes: &CommercePriceInlineAttributes{StartDate: "2026-01-01"},
				Relationships: &AppPriceInlineCreateRelationships{
					AppPricePoint: RelationshipData{
						Data: ResourceIdentifier{Type: "appPricePoints", ID: "pp-2"},
					},
				},
			},
		},
	}

	resp, err := client.CreateAppPriceSchedule(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.ID != "app-sched-1" {
		t.Errorf("id = %q, want app-sched-1", resp.Data.ID)
	}
}

func TestClient_CreateInAppPurchaseAvailability(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/inAppPurchaseAvailabilities" {
			t.Errorf("path = %q, want /v1/inAppPurchaseAvailabilities", r.URL.Path)
		}

		body, _ := io.ReadAll(r.Body)
		var req InAppPurchaseAvailabilityCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if !req.Data.Attributes.AvailableInNewTerritories {
			t.Error("expected availableInNewTerritories=true")
		}
		if len(req.Data.Relationships.AvailableTerritories.Data) != 2 {
			t.Fatalf("expected 2 territories, got %d", len(req.Data.Relationships.AvailableTerritories.Data))
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(InAppPurchaseAvailabilityResponse{
			Data: InAppPurchaseAvailability{Type: "inAppPurchaseAvailabilities", ID: "avail-1"},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &InAppPurchaseAvailabilityCreateRequest{
		Data: InAppPurchaseAvailabilityCreateData{
			Type:       "inAppPurchaseAvailabilities",
			Attributes: InAppPurchaseAvailabilityAttributes{AvailableInNewTerritories: true},
			Relationships: InAppPurchaseAvailabilityCreateRelationships{
				InAppPurchase: RelationshipData{
					Data: ResourceIdentifier{Type: "inAppPurchases", ID: "iap-1"},
				},
				AvailableTerritories: RelationshipDataList{
					Data: []ResourceIdentifier{
						{Type: "territories", ID: "USA"},
						{Type: "territories", ID: "CAN"},
					},
				},
			},
		},
	}

	if _, err := client.CreateInAppPurchaseAvailability(context.Background(), req); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestClient_CreateSubscriptionPlanAvailability(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/subscriptionPlanAvailabilities" {
			t.Errorf("path = %q, want /v1/subscriptionPlanAvailabilities", r.URL.Path)
		}

		body, _ := io.ReadAll(r.Body)
		var req SubscriptionPlanAvailabilityCreateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Attributes.PlanType != "UPFRONT" {
			t.Errorf("plan type = %q, want UPFRONT", req.Data.Attributes.PlanType)
		}
		if req.Data.Relationships.Subscription.Data.ID != "sub-1" {
			t.Errorf("subscription id = %q, want sub-1", req.Data.Relationships.Subscription.Data.ID)
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(SubscriptionPlanAvailabilityResponse{
			Data: SubscriptionPlanAvailability{
				Type:       "subscriptionPlanAvailabilities",
				ID:         "plan-1",
				Attributes: SubscriptionPlanAvailabilityAttributes{PlanType: "UPFRONT"},
			},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &SubscriptionPlanAvailabilityCreateRequest{
		Data: SubscriptionPlanAvailabilityCreateData{
			Type: "subscriptionPlanAvailabilities",
			Attributes: SubscriptionPlanAvailabilityAttributes{
				PlanType:                  "UPFRONT",
				AvailableInNewTerritories: boolPtrTest(true),
			},
			Relationships: SubscriptionPlanAvailabilityCreateRelationships{
				Subscription: RelationshipData{
					Data: ResourceIdentifier{Type: "subscriptions", ID: "sub-1"},
				},
				AvailableTerritories: RelationshipDataList{
					Data: []ResourceIdentifier{{Type: "territories", ID: "USA"}},
				},
			},
		},
	}

	resp, err := client.CreateSubscriptionPlanAvailability(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if resp.Data.Attributes.PlanType != "UPFRONT" {
		t.Errorf("plan type = %q, want UPFRONT", resp.Data.Attributes.PlanType)
	}
}

func TestClient_UpdateSubscriptionPlanAvailability(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/subscriptionPlanAvailabilities/plan-1" {
			t.Errorf("path = %q, want /v1/subscriptionPlanAvailabilities/plan-1", r.URL.Path)
		}
		if r.Method != http.MethodPatch {
			t.Errorf("method = %q, want PATCH", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var req SubscriptionPlanAvailabilityUpdateRequest
		if err := json.Unmarshal(body, &req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}
		if req.Data.Relationships == nil || req.Data.Relationships.AvailableTerritories == nil {
			t.Fatal("expected availableTerritories relationship in the request body")
		}
		if len(req.Data.Relationships.AvailableTerritories.Data) != 1 {
			t.Errorf("expected 1 territory, got %d", len(req.Data.Relationships.AvailableTerritories.Data))
		}

		json.NewEncoder(w).Encode(SubscriptionPlanAvailabilityResponse{
			Data: SubscriptionPlanAvailability{Type: "subscriptionPlanAvailabilities", ID: "plan-1"},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	req := &SubscriptionPlanAvailabilityUpdateRequest{
		Data: SubscriptionPlanAvailabilityUpdateData{
			Type: "subscriptionPlanAvailabilities",
			ID:   "plan-1",
			Relationships: &SubscriptionPlanAvailabilityUpdateRelationships{
				AvailableTerritories: &RelationshipDataList{
					Data: []ResourceIdentifier{{Type: "territories", ID: "JPN"}},
				},
			},
		},
	}

	if _, err := client.UpdateSubscriptionPlanAvailability(context.Background(), "plan-1", req); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestClient_ListSubscriptionPricePointAdjustedEqualizations(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/subscriptionPricePoints/pp-1/adjustedEqualizations" {
			t.Errorf("path = %q, want /v1/subscriptionPricePoints/pp-1/adjustedEqualizations", r.URL.Path)
		}
		if got := r.URL.Query().Get("filter[upfrontPricePointId]"); got != "pp-upfront" {
			t.Errorf("filter[upfrontPricePointId] = %q, want pp-upfront", got)
		}
		if got := r.URL.Query().Get("filter[planType]"); got != "UPFRONT" {
			t.Errorf("filter[planType] = %q, want UPFRONT", got)
		}

		json.NewEncoder(w).Encode(SubscriptionPricePointsResponse{
			Data: []SubscriptionPricePoint{
				{
					Type:       "subscriptionPricePoints",
					ID:         "pp-2",
					Attributes: SubscriptionPricePointAttributes{CustomerPrice: "9.99"},
				},
			},
		})
	})

	client, server := newTestClient(t, handler)
	defer server.Close()

	opts := &ListOptions{
		Filter: map[string][]string{
			"upfrontPricePointId": {"pp-upfront"},
			"planType":            {"UPFRONT"},
		},
	}

	resp, err := client.ListSubscriptionPricePointAdjustedEqualizations(context.Background(), "pp-1", opts)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(resp.Data) != 1 {
		t.Fatalf("expected 1 price point, got %d", len(resp.Data))
	}
	if resp.Data[0].Attributes.CustomerPrice != "9.99" {
		t.Errorf("customer price = %q, want 9.99", resp.Data[0].Attributes.CustomerPrice)
	}
}

// TestCommerceDeletePaths checks that the version-scoped v2 delete
// endpoints target the v2 tree, not the v1 one.
func TestCommerceDeletePaths(t *testing.T) {
	tests := []struct {
		name string
		path string
		call func(*Client) error
	}{
		{
			name: "delete in-app purchase localization",
			path: "/v2/inAppPurchaseLocalizations/loc-1",
			call: func(c *Client) error {
				return c.DeleteInAppPurchaseLocalization(context.Background(), "loc-1")
			},
		},
		{
			name: "delete in-app purchase image",
			path: "/v2/inAppPurchaseImages/img-1",
			call: func(c *Client) error {
				return c.DeleteInAppPurchaseImage(context.Background(), "img-1")
			},
		},
		{
			name: "delete subscription localization",
			path: "/v2/subscriptionLocalizations/loc-2",
			call: func(c *Client) error {
				return c.DeleteSubscriptionLocalization(context.Background(), "loc-2")
			},
		},
		{
			name: "delete subscription image",
			path: "/v2/subscriptionImages/img-2",
			call: func(c *Client) error {
				return c.DeleteSubscriptionImage(context.Background(), "img-2")
			},
		},
		{
			name: "delete subscription group localization",
			path: "/v2/subscriptionGroupLocalizations/loc-3",
			call: func(c *Client) error {
				return c.DeleteSubscriptionGroupLocalization(context.Background(), "loc-3")
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				if r.URL.Path != tt.path {
					t.Errorf("path = %q, want %s", r.URL.Path, tt.path)
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

// TestReviewSubmissionItemCommerceRelationships checks that the commerce
// version relationships added in 4.4.1 serialize under the member names
// the review submission item endpoint expects.
func TestReviewSubmissionItemCommerceRelationships(t *testing.T) {
	req := ReviewSubmissionItemCreateRequest{
		Data: ReviewSubmissionItemCreateData{
			Type: "reviewSubmissionItems",
			Relationships: ReviewSubmissionItemCreateRelationships{
				ReviewSubmission: RelationshipData{
					Data: ResourceIdentifier{Type: "reviewSubmissions", ID: "sub-1"},
				},
				InAppPurchaseVersion: &RelationshipData{
					Data: ResourceIdentifier{Type: "inAppPurchaseVersions", ID: "ver-1"},
				},
				SubscriptionVersion: &RelationshipData{
					Data: ResourceIdentifier{Type: "subscriptionVersions", ID: "ver-2"},
				},
				SubscriptionGroupVersion: &RelationshipData{
					Data: ResourceIdentifier{Type: "subscriptionGroupVersions", ID: "ver-3"},
				},
			},
		},
	}

	body, err := json.Marshal(req)
	if err != nil {
		t.Fatalf("failed to marshal request: %v", err)
	}

	var decoded struct {
		Data struct {
			Relationships map[string]struct {
				Data ResourceIdentifier `json:"data"`
			} `json:"relationships"`
		} `json:"data"`
	}
	if err := json.Unmarshal(body, &decoded); err != nil {
		t.Fatalf("failed to decode request: %v", err)
	}

	want := map[string]string{
		"inAppPurchaseVersion":     "inAppPurchaseVersions",
		"subscriptionVersion":      "subscriptionVersions",
		"subscriptionGroupVersion": "subscriptionGroupVersions",
	}
	for member, resourceType := range want {
		rel, ok := decoded.Data.Relationships[member]
		if !ok {
			t.Errorf("missing relationship member %q", member)
			continue
		}
		if rel.Data.Type != resourceType {
			t.Errorf("%s type = %q, want %s", member, rel.Data.Type, resourceType)
		}
	}
}
