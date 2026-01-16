#!/usr/bin/env zsh
set -euo pipefail

BINARY_NAME="asc-mcp"
INSTALL_DIR="${INSTALL_DIR:-${GOPATH:-$HOME/go}/bin}"

echo "Building ${BINARY_NAME}..."
./script/build.zsh

echo "Installing to ${INSTALL_DIR}..."
mkdir -p "${INSTALL_DIR}"
cp "bin/${BINARY_NAME}" "${INSTALL_DIR}/"

echo "Installation complete"
echo "Make sure ${INSTALL_DIR} is in your PATH"
