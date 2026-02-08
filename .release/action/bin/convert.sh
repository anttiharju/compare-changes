#!/usr/bin/env sh

target="$OS-$ARCH"
case "$target" in
  Linux-ARM64) echo "target=aarch64-unknown-linux-musl" >> "$GITHUB_OUTPUT" ;;
  Linux-X64) echo "target=x86_64-unknown-linux-musl" >> "$GITHUB_OUTPUT" ;;
  *) echo "Unsupported os-arch target: $target" >&2; exit 1 ;;
esac
