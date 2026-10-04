#!/usr/bin/env bash
set -euo pipefail

ARGS=()
for arg in "$@"; do
  if [[ "$arg" == "-lmsvcrt" ]]; then
    ARGS+=("-lc")
  else
    ARGS+=("$arg")
  fi
done

zig cc "${ARGS[@]}" -target x86_64-windows-gnu
