#!/usr/bin/env bash
# DEPRECATED: scheduled for removal with the handshake protocol revisions.
# Builds the original Go server from git history as the parity reference.
#
#   tests/parity/build-go-reference.sh [GIT_REV] [OUT_BINARY]
#
# GIT_REV defaults to the last commit that still contained the Go sources.
# The build is patched so the Go client honours ASC_API_BASE_URL (as the
# Rust client does), which lets the harness point both at the mock API.
set -euo pipefail
repo="$(git rev-parse --show-toplevel)"
rev="${1:-$(git -C "$repo" log -n1 --format=%H -- go.mod)}"
if ! git -C "$repo" cat-file -e "$rev:go.mod" 2>/dev/null; then
  rev="$(git -C "$repo" rev-list -n1 "$rev"^ -- go.mod)"
fi
out="${2:-$repo/target/asc-mcp-go}"
case "$out" in /*) ;; *) out="$PWD/$out" ;; esac
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
git -C "$repo" archive "$rev" go.mod go.sum internal cmd | tar -x -C "$work"
python3 - "$work/internal/asc/api/client.go" <<'PY'
import sys
p = sys.argv[1]
s = open(p).read()
s = s.replace("baseURL:       BaseURL,", "baseURL:       baseURLFromEnv(),")
s = s.replace("import (\n", "import (\n\t\"os\"\n", 1)
s += '''
func baseURLFromEnv() string {
	if v := os.Getenv("ASC_API_BASE_URL"); v != "" {
		return strings.TrimRight(v, "/")
	}
	return BaseURL
}
'''
open(p, "w").write(s)
PY
mkdir -p "$(dirname "$out")"
(cd "$work" && go build -o "$out" ./cmd/asc-mcp)
echo "built Go reference from $rev -> $out"
