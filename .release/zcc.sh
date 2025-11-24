#!/usr/bin/env bash
set -euo pipefail

# Convert Rust target to Zig target
case "$TARGET" in
  aarch64-apple-darwin)
    ZIG_TARGET="aarch64-macos"
    ;;
  x86_64-unknown-linux-gnu)
    ZIG_TARGET="x86_64-linux-gnu"
    ;;
  aarch64-unknown-linux-gnu)
    ZIG_TARGET="aarch64-linux-gnu"
    ;;
  *)
    echo "$0: unsupported target: $TARGET" >&2
    exit 1
    ;;
esac

zig cc -target "$ZIG_TARGET" "$@"
