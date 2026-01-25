#!/usr/bin/env bash
set -euo pipefail

if [[ -f Cargo.toml ]]; then
  version=$(yq '.package.version' Cargo.toml)
else
  version=$(gh release view --json name -q '.name' 2>/dev/null || echo '0.0.0')
fi
echo "version=$version" >> "$GITHUB_OUTPUT"

major_release="$(echo "$version" | cut -d. -f1)"
minor_release="$(echo "$version" | cut -d. -f2)"
patch_release="$(echo "$version" | cut -d. -f3)"
{
  echo "major-release=$major_release"
  echo "minor-release=$minor_release"
  echo "patch-release=$patch_release"
} >> "$GITHUB_OUTPUT"
echo "current=$version(as-is)=$major_release.$minor_release.$patch_release(parsed)"
