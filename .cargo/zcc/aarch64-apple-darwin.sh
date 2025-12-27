#!/usr/bin/env bash
set -euo pipefail

# Filter out -liconv since it being included is "kind of an accident", see https://github.com/rust-lang/rust/issues/112501#issuecomment-1616996273
ARGS=()
for arg in "$@"; do
  if [[ "$arg" != "-liconv" ]]; then
    ARGS+=("$arg")
  fi
done

exec zig cc "${ARGS[@]}" -target aarch64-macos
