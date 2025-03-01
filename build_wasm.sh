#!/usr/bin/bash
cargo build --release
$HOME/.cargo/bin/wasm-bindgen  ./target/wasm32-unknown-unknown/release/froguelike.wasm --out-dir ./wasm_help/staging --no-modules --no-typescript
cp ./wasm_help/index.html ./wasm_help/staging/index.html
miniserve ./wasm_help/staging/
start http://127.0.0.1:8080/index.html