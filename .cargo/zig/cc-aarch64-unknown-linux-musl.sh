#!/usr/bin/env bash
set -euo pipefail

# https://codeberg.org/ziglang/zig/issues/36624
ARGS=()
for arg in "$@"; do
  if [[ "$arg" != "-Wl,--fix-cortex-a53-843419" ]]; then
    ARGS+=("$arg")
  fi
done

exec zig cc "${ARGS[@]}" -target aarch64-linux-musl

