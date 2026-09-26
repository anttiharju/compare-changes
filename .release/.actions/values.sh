#!/usr/bin/env bash
set -euo pipefail

capture() {
  eval "export $1=\"$2\""
  echo "export $1=\"$2\""
}

capture PKG_FILENAME action
capture PKG_EXTENSION yml
capture PKG_OUTPUT "* ../.actions/bash"
capture PKG_VERSION "$VERSION"

if [[ "$MODE" = bootstrap ]]; then
  capture PKG_MACOS_ARM_SHA TBD
  capture PKG_LINUX_ARM_SHA TBD
  capture PKG_LINUX_X64_SHA TBD
  exit 0
fi

repo="${GITHUB_REPOSITORY##*/}"

if [[ "$MODE" = release ]]; then
  repo_root="$(git rev-parse --show-toplevel)"
  directory="$repo_root/build/$VERSION"
  macos_arm_sha="$(sha256sum "$directory/aarch64-apple-darwin/$repo" | cut -d ' ' -f1)"
  linux_arm_sha="$(sha256sum "$directory/aarch64-unknown-linux-musl/$repo" | cut -d ' ' -f1)"
  linux_x64_sha="$(sha256sum "$directory/x86_64-unknown-linux-musl/$repo" | cut -d ' ' -f1)"
  capture PKG_MACOS_ARM_SHA "$macos_arm_sha"
  capture PKG_LINUX_ARM_SHA "$linux_arm_sha"
  capture PKG_LINUX_X64_SHA "$linux_x64_sha"
  exit 0
fi

gh auth status >&2

bin_checksum() {
  gh release download "$TAG" --repo "$GITHUB_REPOSITORY" --pattern "$repo-$1.tar.gz" --output - |
    tar -xzO "$repo" | sha256sum | cut -d ' ' -f1
}

macos_arm_sha="$(bin_checksum aarch64-apple-darwin)"
linux_arm_sha="$(bin_checksum aarch64-unknown-linux-musl)"
linux_x64_sha="$(bin_checksum x86_64-unknown-linux-musl)"
capture PKG_MACOS_ARM_SHA "$macos_arm_sha"
capture PKG_LINUX_ARM_SHA "$linux_arm_sha"
capture PKG_LINUX_X64_SHA "$linux_x64_sha"
