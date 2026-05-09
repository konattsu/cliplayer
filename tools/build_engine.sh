#!/usr/bin/env bash

CRATE_NAME=engine_wasm
WASM_OPT_VERSION=129

set -euo pipefail
TEMP_DIR=/workspaces/temp
OUTPUT_DIR=/workspaces/public

mkdir -p "$TEMP_DIR/wasm-bindgen"
mkdir -p "$OUTPUT_DIR"


echo "Build wasm"
cd /workspaces/search/engine-wasm
cargo build --release --lib --target wasm32-unknown-unknown

echo "Generate wasm-bindgen output"
# 0.2.120でないと`wasm-opt`と内部スキーマが合わないらしくエラーになる
# `wasm-opt`のバージョンを調節しようとしたがむずかったのでwasm-bindgen側をダウングレード
cargo install -f wasm-bindgen-cli --version 0.2.120
wasm-bindgen --out-dir /workspaces/temp/wasm-bindgen ./target/wasm32-unknown-unknown/release/$CRATE_NAME.wasm

echo "Optimize wasm with wasm-opt"
if [ -x "$TEMP_DIR/wasm-opt" ] && "$TEMP_DIR/wasm-opt" --version | grep -q "version $WASM_OPT_VERSION"; then
  echo "wasm-opt version is already $WASM_OPT_VERSION, skipping download"
else
  echo "Downloading wasm-opt version $WASM_OPT_VERSION"
  curl -fsSL "https://github.com/WebAssembly/binaryen/releases/download/version_$WASM_OPT_VERSION/binaryen-version_$WASM_OPT_VERSION-x86_64-linux.tar.gz" \
    -o "$TEMP_DIR/wasm-bindgen/binaryen.tar.gz"
  tar -xvf "$TEMP_DIR/wasm-bindgen/binaryen.tar.gz" -C "$TEMP_DIR"
  mv "$TEMP_DIR/binaryen-version_$WASM_OPT_VERSION/bin/wasm-opt" "$TEMP_DIR/wasm-opt"
fi
"$TEMP_DIR/wasm-opt" -O "$TEMP_DIR/wasm-bindgen/${CRATE_NAME}_bg.wasm" -o "$OUTPUT_DIR/engine.wasm"
