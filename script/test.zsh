#!/usr/bin/env zsh
set -euo pipefail

echo "Running tests..."

cargo test --locked

echo "Tests complete"
