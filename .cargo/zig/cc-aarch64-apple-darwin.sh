#!/usr/bin/env bash
set -euo pipefail

# TODO: The workaround filtering out `-liconv` can be removed once rust-lang/libc 1.0 is released https://github.com/rust-lang/libc/issues/3248
ARGS=()
for arg in "$@"; do
  if [[ "$arg" != "-liconv" ]]; then
    ARGS+=("$arg")
  fi
done

exec zig cc "${ARGS[@]}" -target aarch64-macos
