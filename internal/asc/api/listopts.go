// Package api provides query option helpers for App Store Connect API
// list endpoints.
package api

import (
	"fmt"
	"net/url"
	"sort"
	"strings"
)

// ListOptions captures the JSON:API query parameters that every list
// endpoint accepts on the App Store Connect API:
//
//   - Limit          → ?limit=N (Apple caps at 200; we cap callers too)
//   - Filter[<k>=v]  → ?filter[<k>]=v1,v2,...
//   - Sort           → ?sort=field1,-field2
//   - Fields[<k>=v]  → ?fields[<k>]=f1,f2 (sparse fieldsets)
//   - Include        → ?include=rel1,rel2 (related resources)
//
// All fields are optional; a zero ListOptions produces an empty query.
type ListOptions struct {
	Limit   int
	Filter  map[string][]string
	Sort    []string
	Fields  map[string][]string
	Include []string
}

// Apply writes the option's query parameters into q. Keys with empty
// values are skipped so we never emit malformed `filter[x]=`. Apply
// sorts keys alphabetically so the resulting URL is deterministic,
// which makes integration tests cheaper to write.
func (o *ListOptions) Apply(q url.Values) {
	if o == nil {
		return
	}
	if o.Limit > 0 {
		q.Set("limit", fmt.Sprintf("%d", o.Limit))
	}
	writeBracketedMap(q, "filter", o.Filter)
	if v := strings.Join(stripEmpty(o.Sort), ","); v != "" {
		q.Set("sort", v)
	}
	writeBracketedMap(q, "fields", o.Fields)
	if v := strings.Join(stripEmpty(o.Include), ","); v != "" {
		q.Set("include", v)
	}
}

// writeBracketedMap renders a map like Filter or Fields as repeated
// ?prefix[key]=v1,v2 query params.
func writeBracketedMap(q url.Values, prefix string, m map[string][]string) {
	if len(m) == 0 {
		return
	}
	keys := make([]string, 0, len(m))
	for k := range m {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	for _, k := range keys {
		vs := stripEmpty(m[k])
		if len(vs) == 0 {
			continue
		}
		q.Set(prefix+"["+k+"]", strings.Join(vs, ","))
	}
}

func stripEmpty(in []string) []string {
	out := make([]string, 0, len(in))
	for _, s := range in {
		s = strings.TrimSpace(s)
		if s != "" {
			out = append(out, s)
		}
	}
	return out
}

// withLimit is a convenience for callers that only need to set Limit.
// It avoids constructing a one-field ListOptions inline at every call site.
func withLimit(limit int) *ListOptions {
	if limit <= 0 {
		return nil
	}
	return &ListOptions{Limit: limit}
}
