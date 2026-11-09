#!/usr/bin/env bash
# Rust -> WebAssembly -> single self-contained dist/index.html
set -euo pipefail
cd "$(dirname "$0")/.."
source "$HOME/.cargo/env" 2>/dev/null || true
cargo build --release --target wasm32-unknown-unknown -p smart-egui --lib
rm -rf target/web && mkdir -p target/web
wasm-bindgen --target web --no-typescript --out-dir target/web --out-name smart_egui target/wasm32-unknown-unknown/release/smart_egui.wasm
if command -v wasm-opt >/dev/null 2>&1; then
  wasm-opt -Oz --strip-debug --all-features target/web/smart_egui_bg.wasm -o target/web/opt.wasm
  mv target/web/opt.wasm target/web/smart_egui_bg.wasm
fi
node scripts/pack.mjs
npm run build
