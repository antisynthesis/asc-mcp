package api

import (
	"net/url"
	"testing"
)

func TestListOptions_Apply_Empty(t *testing.T) {
	q := url.Values{}
	(*ListOptions)(nil).Apply(q)
	if len(q) != 0 {
		t.Errorf("nil opts should write nothing, got %v", q)
	}
	(&ListOptions{}).Apply(q)
	if len(q) != 0 {
		t.Errorf("empty opts should write nothing, got %v", q)
	}
}

func TestListOptions_Apply_AllFields(t *testing.T) {
	q := url.Values{}
	opts := &ListOptions{
		Limit: 25,
		Filter: map[string][]string{
			"platform":      {"IOS", "MAC_OS"},
			"appStoreState": {"READY_FOR_SALE"},
		},
		Sort: []string{"-uploadedDate", "name"},
		Fields: map[string][]string{
			"apps": {"name", "bundleId"},
		},
		Include: []string{"builds", "appStoreVersions"},
	}
	opts.Apply(q)

	if got := q.Get("limit"); got != "25" {
		t.Errorf("limit = %q, want 25", got)
	}
	if got := q.Get("filter[platform]"); got != "IOS,MAC_OS" {
		t.Errorf("filter[platform] = %q, want IOS,MAC_OS", got)
	}
	if got := q.Get("filter[appStoreState]"); got != "READY_FOR_SALE" {
		t.Errorf("filter[appStoreState] = %q, want READY_FOR_SALE", got)
	}
	if got := q.Get("sort"); got != "-uploadedDate,name" {
		t.Errorf("sort = %q, want -uploadedDate,name", got)
	}
	if got := q.Get("fields[apps]"); got != "name,bundleId" {
		t.Errorf("fields[apps] = %q, want name,bundleId", got)
	}
	if got := q.Get("include"); got != "builds,appStoreVersions" {
		t.Errorf("include = %q, want builds,appStoreVersions", got)
	}
}

func TestListOptions_StripsEmptyValues(t *testing.T) {
	q := url.Values{}
	opts := &ListOptions{
		Filter: map[string][]string{
			"x": {"", "  ", "real"},
			"y": {""},
		},
		Sort:    []string{"", " ", "field"},
		Include: []string{""},
	}
	opts.Apply(q)
	if got := q.Get("filter[x]"); got != "real" {
		t.Errorf("filter[x] = %q, want real", got)
	}
	if got, ok := q["filter[y]"]; ok {
		t.Errorf("filter[y] should be absent when all values empty, got %v", got)
	}
	if got := q.Get("sort"); got != "field" {
		t.Errorf("sort = %q, want field", got)
	}
	if _, ok := q["include"]; ok {
		t.Errorf("include should be absent when all values empty")
	}
}
