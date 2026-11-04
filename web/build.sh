#!/bin/sh
# Construit la version web (WebAssembly) dans web/dist, puis la sert avec `./web/build.sh serve`.
# Prérequis : rustup target add wasm32-unknown-unknown
set -e
cd "$(dirname "$0")/.."
cargo build --release --target wasm32-unknown-unknown -p afk-game --bin all-for-kingdom
mkdir -p web/dist
cp target/wasm32-unknown-unknown/release/all-for-kingdom.wasm web/dist/
cp web/index.html web/dist/
# le chargeur JS de miniquad (la version utilisée par macroquad)
GL=$(ls -d "${CARGO_HOME:-$HOME/.cargo}"/registry/src/*/miniquad-0.4.*/js/gl.js | tail -n 1)
cp "$GL" web/dist/mq_js_bundle.js
echo "Version web prête dans web/dist ($(du -h web/dist/all-for-kingdom.wasm | cut -f1))"
if [ "$1" = "serve" ]; then
  echo "http://localhost:8000"
  cd web/dist && python3 -m http.server 8000
fi
