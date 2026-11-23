#!/usr/bin/env zsh
set -euo pipefail

echo "Running lint checks..."

echo "Checking formatting..."
cargo fmt --all -- --check
echo "  Formatting OK"

echo "Running clippy..."
cargo clippy --all-targets --locked -- -D warnings
echo "  Clippy OK"

echo "Lint checks complete"
