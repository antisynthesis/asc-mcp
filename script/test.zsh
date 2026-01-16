#!/usr/bin/env zsh
set -euo pipefail

echo "Running tests..."

# Run unit tests with race detection
go test -v -race ./...

echo "Tests complete"
