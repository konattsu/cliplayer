#!/bin/sh

LOG_DIR="./logs"

files=$(find "$LOG_DIR" -type f | sort)
count=$(echo "$files" | wc -l)
i=1

echo "$files" | while read -r file; do
  echo "|$(basename "$file")|------------------------------------------------------------------"
  echo
  if [ -n "$YOUTUBE_API_KEY" ] && grep -q "$YOUTUBE_API_KEY" "$file"; then
    echo "Error: YOUTUBE_API_KEY found in $file"
    exit 1
  fi
  cat "$file"
  if [ "$i" -lt "$count" ]; then
    echo
    echo
  fi
  i=$((i+1))
done
