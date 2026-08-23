#!/usr/bin/env bash
set -euo pipefail

# TODO: Dig into this
ARGS=()
for arg in "$@"; do
  if [[ "$arg" != "-Wl,--fix-cortex-a53-843419" ]]; then
    ARGS+=("$arg")
  fi
done

exec zig cc "${ARGS[@]}" -target aarch64-linux-musl

