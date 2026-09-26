#!/usr/bin/env bash
set -euo pipefail

directory="$RUNNER_TEMP/anttiharju/compare-changes/${VERSION#v}"
mkdir -p "$directory"
echo "directory=$directory" >> "$GITHUB_OUTPUT"
cd "$directory"

if [[ -f ./compare-changes ]]; then
  echo 'File check passed: compare-changes exists'
else
  echo 'File check failed: no compare-changes found'
fi

if [[ -x ./compare-changes ]]; then
  echo 'Execute permission check passed: compare-changes is executable'
else
  echo 'Execute permission check failed: compare-changes is not executable'
fi

if [[ -f ./compare-changes && -x ./compare-changes ]]; then
  echo "available=true" >> "$GITHUB_OUTPUT"
fi
