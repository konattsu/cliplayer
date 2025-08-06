#!/bin/sh

LOG_DIR="./logs"

set -- "$LOG_DIR"/*
[ -e "$1" ] || exit 0

for file in "$LOG_DIR"/*; do
  out="${file}.json"
  echo "[" > "$out"
  sed '$!s/$/,/' "$file" >> "$out"
  echo "]" >> "$out"
done
