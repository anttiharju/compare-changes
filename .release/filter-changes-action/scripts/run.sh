#!/usr/bin/env bash
set -euo pipefail

args=(--filter --paths "$FILTER" --changes "$CHANGES")
if [[ "$DEBUG" = "true" ]]; then
  args+=(--debug)
fi

"$BINARY" "${args[@]}"
