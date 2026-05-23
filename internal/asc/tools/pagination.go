// Package tools provides MCP tool implementations for App Store Connect.
package tools

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/antisynthesis/asc-mcp/internal/asc/api"
	"github.com/antisynthesis/asc-mcp/internal/asc/mcp"
)

// cursorProperty returns the JSON Schema entry used by every list tool
// to opt into pagination. Clients pass back the opaque URL surfaced in
// the previous response's footer to fetch the next page.
func cursorProperty() mcp.Property {
	return mcp.Property{
		Type:        "string",
		Description: "Opaque pagination cursor. Pass the URL surfaced as 'Next cursor' in the previous response to fetch the next page.",
	}
}

// paginatedFetch returns the next page of a list response, either by
// fetching the first page (when cursor is empty) or by following the
// cursor URL returned by a previous call.
//
// The cursor URL is validated against the client's base host by
// Client.GetURL so a tampered response cannot redirect the client.
func paginatedFetch[T any](
	ctx context.Context,
	client *api.Client,
	cursor string,
	fetchFirst func() (*T, error),
) (*T, error) {
	if cursor == "" {
		return fetchFirst()
	}
	data, err := client.GetURL(ctx, cursor)
	if err != nil {
		return nil, err
	}
	var resp T
	if err := json.Unmarshal(data, &resp); err != nil {
		return nil, fmt.Errorf("failed to parse paginated response: %w", err)
	}
	return &resp, nil
}

// paginationFooter formats the "next cursor" hint appended to list-tool
// responses. The model sees the verbatim URL and can pass it back
// through the tool's `cursor` argument to fetch the next page.
func paginationFooter(nextURL string) string {
	if nextURL == "" {
		return ""
	}
	return fmt.Sprintf("\n\n_More results available. Call again with `cursor=%q` to fetch the next page._", nextURL)
}

// newListResult builds a successful tool result for a list endpoint,
// appending the next-page footer when the response carries a
// `links.next`.
func newListResult(content string, links api.PagedDocumentLinks) *mcp.ToolsCallResult {
	return mcp.NewSuccessResult(content + paginationFooter(links.Next))
}
