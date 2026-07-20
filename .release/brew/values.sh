#!/usr/bin/env bash
set -euo pipefail

capture() {
  eval "export $1=\"$2\""
  echo "export $1=\"$2\""
}

repo="${GITHUB_REPOSITORY##*/}"
capture PKG_FILENAME "$repo"
capture PKG_EXTENSION rb
capture PKG_REPO "$repo"
class="$(echo "$repo" | gawk -F'-' '{for(i=1;i<=NF;i++) printf "%s%s", toupper(substr($i,1,1)), substr($i,2)}')"
capture PKG_CLASS "$class"
desc="$(gh repo view --json description --jq .description)"
capture PKG_DESC "$desc"
homepage="$(gh api "repos/{owner}/{repo}" --jq .homepage)"
capture PKG_HOMEPAGE "$homepage"
capture PKG_VERSION "${TAG#v}"
capture PKG_OWNER "${GITHUB_REPOSITORY%%/*}"

if [[ "$TAG" = "v0.0.0" ]] || ! gh api "repos/{owner}/{repo}/git/ref/tags/$TAG" &>/dev/null; then
  capture PKG_MAC_INTEL_SHA TBD
  capture PKG_MAC_ARM_SHA TBD
  capture PKG_LINUX_ARM_SHA TBD
  capture PKG_LINUX_INTEL_SHA TBD
  exit 0
fi

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root/.release/brew"
pattern="$repo-*.tar.gz"
gh release download "$TAG" --pattern "$pattern" --clobber
for archive in $pattern; do
  echo "# $archive"
done
mac_arm_sha="$([[ -f "$repo-aarch64-apple-darwin.tar.gz" ]] && sha256sum "$repo-aarch64-apple-darwin.tar.gz" | cut -d ' ' -f1 || echo "TBD")"
capture PKG_MAC_ARM_SHA "$mac_arm_sha"
linux_arm_sha="$([[ -f "$repo-aarch64-unknown-linux-musl.tar.gz" ]] && sha256sum "$repo-aarch64-unknown-linux-musl.tar.gz" | cut -d ' ' -f1 || echo "TBD")"
capture PKG_LINUX_ARM_SHA "$linux_arm_sha"
linux_intel_sha="$([[ -f "$repo-x86_64-unknown-linux-musl.tar.gz" ]] && sha256sum "$repo-x86_64-unknown-linux-musl.tar.gz" | cut -d ' ' -f1 || echo "TBD")"
capture PKG_LINUX_INTEL_SHA "$linux_intel_sha"
