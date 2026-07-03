#!/usr/bin/env bash
set -euo pipefail

# Build the WASM package and install it into the web app's public directory.
# Requires: wasm-pack (https://rustwasm.github.io/wasm-pack/installer/)

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/i8080-web/public/wasm"

echo "Building WASM package..."
wasm-pack build --target bundler --out-dir "$OUT_DIR" -- --features wasm

# wasm-pack drops a .gitignore that would hide the output from git
rm -f "$OUT_DIR/.gitignore"

echo "Done. WASM output: $OUT_DIR"
