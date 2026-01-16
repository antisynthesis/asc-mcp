#!/usr/bin/env zsh
set -euo pipefail

BINARY_NAME="asc-mcp"
BUILD_DIR="bin"
VERSION="${VERSION:-dev}"
COMMIT="${COMMIT:-$(git rev-parse --short HEAD 2>/dev/null || echo 'unknown')}"
BUILD_DATE="${BUILD_DATE:-$(date -u +%Y-%m-%dT%H:%M:%SZ)}"

LDFLAGS="-w -s"
LDFLAGS="${LDFLAGS} -X github.com/antisynthesis/asc-mcp/internal/asc/cmd.Version=${VERSION}"
LDFLAGS="${LDFLAGS} -X github.com/antisynthesis/asc-mcp/internal/asc/cmd.Commit=${COMMIT}"
LDFLAGS="${LDFLAGS} -X github.com/antisynthesis/asc-mcp/internal/asc/cmd.BuildDate=${BUILD_DATE}"

mkdir -p "${BUILD_DIR}"

echo "Building ${BINARY_NAME}..."
echo "  Version:    ${VERSION}"
echo "  Commit:     ${COMMIT}"
echo "  Build Date: ${BUILD_DATE}"

go build -ldflags="${LDFLAGS}" -o "${BUILD_DIR}/${BINARY_NAME}" ./cmd/asc-mcp

echo "Build complete: ${BUILD_DIR}/${BINARY_NAME}"
