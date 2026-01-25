#!/usr/bin/env bash
set -euo pipefail

if [[ "$REQUIRE" = "true" ]]; then
  echo "Error: $MSG"
  exit 1
else
  echo "skipped=true"
  echo "skipped=true" >> "$GITHUB_OUTPUT"
fi
