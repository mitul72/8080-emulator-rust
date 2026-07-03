#!/usr/bin/env bash
set -euo pipefail

# Build WASM and start the Vite dev server.
# Requires: wasm-pack, Node.js, npm

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

"$ROOT/scripts/build-wasm.sh"

echo "Starting Vite dev server..."
cd "$ROOT/i8080-web" && npm install && npm run dev
