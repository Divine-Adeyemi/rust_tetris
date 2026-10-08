#!/usr/bin/env bash
# Build the game for the web and serve it at http://localhost:8080
set -euo pipefail
cd "$(dirname "$0")"

rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/rust_tetris.wasm web/

PORT="${1:-8080}"
echo "Serving at http://localhost:${PORT}"
python3 -m http.server "$PORT" --directory web
