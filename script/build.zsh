#!/usr/bin/env zsh
set -euo pipefail

BINARY_NAME="asc-mcp"
BUILD_DIR="bin"
export ASC_MCP_VERSION="${VERSION:-dev}"
export ASC_MCP_COMMIT="${COMMIT:-$(git rev-parse --short HEAD 2>/dev/null || echo 'unknown')}"
export ASC_MCP_BUILD_DATE="${BUILD_DATE:-$(date -u +%Y-%m-%dT%H:%M:%SZ)}"

mkdir -p "${BUILD_DIR}"

echo "Building ${BINARY_NAME}..."
echo "  Version:    ${ASC_MCP_VERSION}"
echo "  Commit:     ${ASC_MCP_COMMIT}"
echo "  Build Date: ${ASC_MCP_BUILD_DATE}"

cargo build --release --locked
cp "target/release/${BINARY_NAME}" "${BUILD_DIR}/${BINARY_NAME}"

echo "Build complete: ${BUILD_DIR}/${BINARY_NAME}"
