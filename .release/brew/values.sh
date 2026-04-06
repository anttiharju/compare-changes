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
homepage="$(gh api "repos/$GITHUB_REPOSITORY" --jq .homepage)"
capture PKG_HOMEPAGE "$homepage"
repo_root="$(git rev-parse --show-toplevel)"
version="$(toml get "$repo_root/Cargo.toml" package.version --raw)"
capture PKG_VERSION "$version"
capture PKG_OWNER "${GITHUB_REPOSITORY%%/*}"

tag="v$version"
if [[ "$version" = "0.0.0" ]] || ! gh api "repos/$GITHUB_REPOSITORY/git/ref/tags/$tag" &>/dev/null; then
  capture PKG_MAC_INTEL_SHA TBD
  capture PKG_MAC_ARM_SHA TBD
  capture PKG_LINUX_ARM_SHA TBD
  capture PKG_LINUX_INTEL_SHA TBD
  exit 0
fi

cd "$repo_root/.release/brew"
pattern="$repo-*.tar.gz"
gh release download "$tag" --pattern "$pattern" --clobber
for archive in $pattern; do
  echo "# $archive"
done
mac_arm_sha="$([[ -f "$repo-aarch64-apple-darwin.tar.gz" ]] && sha256sum "$repo-aarch64-apple-darwin.tar.gz" | cut -d ' ' -f1 || echo "TBD")"
capture PKG_MAC_ARM_SHA "$mac_arm_sha"
linux_arm_sha="$([[ -f "$repo-aarch64-unknown-linux-musl.tar.gz" ]] && sha256sum "$repo-aarch64-unknown-linux-musl.tar.gz" | cut -d ' ' -f1 || echo "TBD")"
capture PKG_LINUX_ARM_SHA "$linux_arm_sha"
linux_intel_sha="$([[ -f "$repo-x86_64-unknown-linux-musl.tar.gz" ]] && sha256sum "$repo-x86_64-unknown-linux-musl.tar.gz" | cut -d ' ' -f1 || echo "TBD")"
capture PKG_LINUX_INTEL_SHA "$linux_intel_sha"
