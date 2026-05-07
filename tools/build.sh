#!/usr/bin/env bash

TARGET_DIR=target/x86_64-unknown-linux-gnu/release

set -euo pipefail

echo "Build rust tools"
cargo build --release

echo "Generate minified json files"
"$TARGET_DIR/metadata" artist --operation minify
"$TARGET_DIR/metadata" tag --operation minify
"$TARGET_DIR/musictl" minify

echo "Build search index"
"$TARGET_DIR/index-builder"

echo "Build web site"
# TODO
