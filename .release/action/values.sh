#!/usr/bin/env bash
set -euo pipefail

capture() {
  eval "export $1=\"$2\""
  echo "export $1=\"$2\""
}

capture PKG_FILENAME action
capture PKG_EXTENSION yml
repo_root="$(git rev-parse --show-toplevel)"
version="$(toml get "$repo_root/Cargo.toml" package.version --raw)"
capture PKG_VERSION "$version"
