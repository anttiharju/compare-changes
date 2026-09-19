#!/usr/bin/env bash
set -euo pipefail

# -liconv: see cc-aarch64-apple-darwin.sh
# --fix-cortex-a53-843419: see cc-aarch64-unknown-linux-musl.sh
ARGS=()
for arg in "$@"; do
  if [[ "$arg" != "-liconv" && "$arg" != "-Wl,--fix-cortex-a53-843419" ]]; then
    ARGS+=("$arg")
  fi
done

zig cc "${ARGS[@]}"
