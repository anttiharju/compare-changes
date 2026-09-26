#!/usr/bin/env bash
set -euo pipefail

triple="$OS-$ARCH"
case "$triple" in
  Linux-ARM64) triple=aarch64-unknown-linux-musl ;;
  Linux-X64) triple=x86_64-unknown-linux-musl ;;
  macOS-ARM64) triple=aarch64-apple-darwin ;;
  *) echo "Unsupported os-arch target: $triple" >&2; exit 1 ;;
esac
echo "triple=$triple"
echo "triple=$triple" >> "$GITHUB_OUTPUT"
