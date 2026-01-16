#!/usr/bin/env zsh
set -euo pipefail

echo "Running lint checks..."

# Format check
echo "Checking formatting..."
if [[ -n "$(gofmt -l .)" ]]; then
    echo "The following files are not properly formatted:"
    gofmt -l .
    exit 1
fi
echo "  Formatting OK"

# Vet
echo "Running go vet..."
go vet ./...
echo "  Vet OK"

# Static analysis (if staticcheck is available)
if command -v staticcheck &> /dev/null; then
    echo "Running staticcheck..."
    staticcheck ./...
    echo "  Staticcheck OK"
fi

echo "Lint checks complete"
