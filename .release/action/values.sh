#!/usr/bin/env bash
set -euo pipefail

capture() {
  eval "export $1=\"$2\""
  echo "export $1=\"$2\""
}

capture PKG_EXTENSION yml
repo_root="$(git rev-parse --show-toplevel)"
version="$(yq -p toml -oy '.package.version' "$repo_root/Cargo.toml")"
capture PKG_VERSION "$version"
