#!/usr/bin/env bash
set -euo pipefail

if command -v compare-changes >/dev/null 2>&1; then
  echo 'compare-changes is already installed'
  echo "installed=true" >> "$GITHUB_OUTPUT"
fi
