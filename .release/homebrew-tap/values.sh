#!/usr/bin/env bash
set -euo pipefail

capture() {
  eval "export $1=\"$2\""
  echo "export $1=\"$2\""
}

repo="${GITHUB_REPOSITORY##*/}"
capture PKG_FILENAME "$repo"
capture PKG_EXTENSION rb
capture PKG_OUTPUT "$repo.rb"
capture PKG_REPO "$repo"
class="$(echo "$repo" | gawk -F'-' '{for(i=1;i<=NF;i++) printf "%s%s", toupper(substr($i,1,1)), substr($i,2)}')"
capture PKG_CLASS "$class"
capture PKG_VERSION "$VERSION"
capture PKG_OWNER "${GITHUB_REPOSITORY%%/*}"
gh auth status >&2
desc="$(gh repo view --json description --jq .description)"
capture PKG_DESC "$desc"
homepage="$(gh api "repos/{owner}/{repo}" --jq .homepage)"
capture PKG_HOMEPAGE "$homepage"

if [[ "$MODE" = bootstrap ]]; then
  capture PKG_MACOS_ARM_SHA TBD
  capture PKG_LINUX_ARM_SHA TBD
  capture PKG_LINUX_X64_SHA TBD
  exit 0
fi

if [[ "$MODE" = release ]]; then
  repo_root="$(git rev-parse --show-toplevel)"
  macos_arm_sha="$(sha256sum "$repo_root/$repo-aarch64-apple-darwin.tar.gz" | cut -d ' ' -f1)"
  linux_arm_sha="$(sha256sum "$repo_root/$repo-aarch64-unknown-linux-musl.tar.gz" | cut -d ' ' -f1)"
  linux_x64_sha="$(sha256sum "$repo_root/$repo-x86_64-unknown-linux-musl.tar.gz" | cut -d ' ' -f1)"
  capture PKG_MACOS_ARM_SHA "$macos_arm_sha"
  capture PKG_LINUX_ARM_SHA "$linux_arm_sha"
  capture PKG_LINUX_X64_SHA "$linux_x64_sha"
  exit 0
fi

tar_checksum() {
  gh release download "$TAG" --repo "$GITHUB_REPOSITORY" --pattern "$repo-$1.tar.gz" --output - |
    sha256sum | cut -d ' ' -f1
}

macos_arm_sha="$(tar_checksum aarch64-apple-darwin)"
linux_arm_sha="$(tar_checksum aarch64-unknown-linux-musl)"
linux_x64_sha="$(tar_checksum x86_64-unknown-linux-musl)"
capture PKG_MACOS_ARM_SHA "$macos_arm_sha"
capture PKG_LINUX_ARM_SHA "$linux_arm_sha"
capture PKG_LINUX_X64_SHA "$linux_x64_sha"
