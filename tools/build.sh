#!/usr/bin/env bash

TARGET_DIR=target/x86_64-unknown-linux-gnu/release

set -euo pipefail

echo "Build rust tools"
cargo build --release

ARTIST_INPUT_HASH="$("$TARGET_DIR/metadata" --quiet artist hash-inputs)"
TAG_INPUT_HASH="$("$TARGET_DIR/metadata" --quiet tag hash-inputs)"
MUSIC_INPUT_HASH="$("$TARGET_DIR/musictl" --quiet build hash-inputs)"


DATASET_BUILD_ID_SOURCE="$(
  printf 'cliplayer:dataset-build-id\nartist:%s\ntag:%s\nmusic:%s\n' \
    "$ARTIST_INPUT_HASH" \
    "$TAG_INPUT_HASH" \
    "$MUSIC_INPUT_HASH"
)"
DATASET_BUILD_ID="$(printf '%s' "$DATASET_BUILD_ID_SOURCE" | sha256sum | cut -d ' ' -f 1)"

echo "Dataset build id source: ===$DATASET_BUILD_ID_SOURCE==="
echo "Dataset build id: ===$DATASET_BUILD_ID==="

echo "Generate minified json files"
# build.sh は 1 回だけ dataset build id を決め、それを全生成コマンドに渡す
"$TARGET_DIR/metadata" artist minify --dataset-build-id "$DATASET_BUILD_ID"
"$TARGET_DIR/metadata" tag minify --dataset-build-id "$DATASET_BUILD_ID"
"$TARGET_DIR/musictl" build minify --dataset-build-id "$DATASET_BUILD_ID"

echo "Build search index"
"$TARGET_DIR/index-builder" --dataset-build-id "$DATASET_BUILD_ID"

echo "Build web site"
# TODO
