#!/usr/bin/env bash
set -euo pipefail

echo "Formatting workspace..."
cargo fmt

echo "Running tests..."
cargo test --all
