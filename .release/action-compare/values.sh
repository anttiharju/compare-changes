#!/usr/bin/env bash
set -euo pipefail

capture() {
  eval "export $1=\"$2\""
  echo "export $1=\"$2\""
}

capture PKG_FILENAME action
capture PKG_EXTENSION yml
capture PKG_VERSION "${TAG#v}"
