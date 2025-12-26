#!/usr/bin/env sh

if command -v compare-changes >/dev/null 2>&1; then
  echo 'compare-changes is already installed'
  echo "installed=true" >> "$GITHUB_OUTPUT"
fi
