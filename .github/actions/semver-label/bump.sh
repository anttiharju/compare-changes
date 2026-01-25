#!/usr/bin/env bash
set -eo pipefail

if [[ "$TYPE" == "major-release" ]]; then
  echo "version=$((MAJOR_RELEASE + 1)).0.0" >> "$GITHUB_OUTPUT"
elif [[ "$TYPE" == "minor-release" ]]; then
  echo "version=$MAJOR_RELEASE.$((MINOR_RELEASE + 1)).0" >> "$GITHUB_OUTPUT"
elif [[ "$TYPE" == "patch-release" ]]; then
  echo "version=$MAJOR_RELEASE.$MINOR_RELASE.$((PATCH_RELEASE + 1))" >> "$GITHUB_OUTPUT"
else
  echo "Invalid type: $TYPE" >&2
  exit 1
fi
