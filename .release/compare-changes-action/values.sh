#!/usr/bin/env bash
set -euo pipefail

capture() {
  eval "export $1=\"$2\""
  echo "export $1=\"$2\""
}

capture PKG_FILENAME action
capture PKG_EXTENSION yml
capture PKG_OUTPUT "* ../.actions"
capture PKG_VERSION "${TAG#v}"

if [[ "$TAG" = "v0.0.0" ]] || ! gh api "repos/{owner}/{repo}/git/ref/tags/$TAG" &>/dev/null; then
  capture PKG_MACOS_ARM_SHA TBD
  capture PKG_LINUX_ARM_SHA TBD
  capture PKG_LINUX_X64_SHA TBD
  exit 0
fi

repo="${GITHUB_REPOSITORY##*/}"
for target in aarch64-apple-darwin aarch64-unknown-linux-musl x86_64-unknown-linux-musl; do
  checksum="$(gh release download "$TAG" --repo "$GITHUB_REPOSITORY" --pattern "$repo-$target.tar.gz" --output - |
    tar -xzO "$repo" | sha256sum | cut -d ' ' -f1)"
  case "$target" in
    aarch64-apple-darwin) capture PKG_MACOS_ARM_SHA "$checksum" ;;
    aarch64-unknown-linux-musl) capture PKG_LINUX_ARM_SHA "$checksum" ;;
    x86_64-unknown-linux-musl) capture PKG_LINUX_X64_SHA "$checksum" ;;
  esac
done
