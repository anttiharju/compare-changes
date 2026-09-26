#!/usr/bin/env bash
set -euo pipefail

case "$TARGET" in
  aarch64-apple-darwin) checksum="$MACOS_ARM_SHA" ;;
  aarch64-unknown-linux-musl) checksum="$LINUX_ARM_SHA" ;;
  x86_64-unknown-linux-musl) checksum="$LINUX_X64_SHA" ;;
  *) echo "Unsupported target: $TARGET" >&2; exit 1 ;;
esac

if command -v sha256sum >/dev/null 2>&1; then
  printf '%s  %s\n' "$checksum" "$BINARY" | sha256sum --check
else
  printf '%s  %s\n' "$checksum" "$BINARY" | shasum -a 256 --check
fi
