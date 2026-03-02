#!/usr/bin/env bash

# Generic policy renderer: substitute <VAR> placeholders from env vars

set -euo pipefail

if [ $# -lt 1 ]; then
  echo "Usage: $0 TEMPLATE_FILE [additional aws args]" >&2
  exit 1
fi

TEMPLATE_PATH="$1"
shift

if [ ! -f "$TEMPLATE_PATH" ]; then
  echo "template not found: $TEMPLATE_PATH" >&2
  exit 1
fi

template=$(<"$TEMPLATE_PATH")
vars=$(grep -o '<[A-Z0-9_]*>' "$TEMPLATE_PATH" | tr -d '<>' | sort -u)
for v in $vars; do
  if [ -z "${!v-}" ]; then
    echo "required environment variable $v is not set" >&2
    exit 1
  fi
  export "$v"="${!v}"
done

perl -pe 's/<([A-Z0-9_]+)>/defined $ENV{$1} ? $ENV{$1} : die "env $1 not set\n"/ge' <<<"$template"
