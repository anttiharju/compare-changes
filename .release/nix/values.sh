#!/usr/bin/env bash
set -euo pipefail

capture() {
  eval "export $1=\"$2\""
  echo "export $1=\"$2\""
}

repo="${GITHUB_REPOSITORY##*/}"
capture PKG_FILENAME "default"
capture PKG_EXTENSION nix
capture PKG_REPO "$repo"
repo_root="$(git rev-parse --show-toplevel)"
version="$(toml get "$repo_root/Cargo.toml" package.version --raw)"
capture PKG_VERSION "$version"
capture PKG_OWNER "${GITHUB_REPOSITORY%%/*}"
capture PKG_REV "$GITHUB_SHA"
sha256="$(nix-prefetch-url --quiet --unpack "https://github.com/$GITHUB_REPOSITORY/archive/$GITHUB_SHA.tar.gz")"
hash="$(nix hash convert --hash-algo sha256 --to sri "$sha256")"
capture PKG_HASH "$hash"
time=$(TZ=UTC git show --quiet --date=format-local:%Y-%m-%dT%H:%M:%SZ --format=%cd)
capture PKG_TIME "$time"
homepage="$(gh api "repos/{owner}/{repo}" --jq .homepage)"
capture PKG_HOMEPAGE "$homepage"
desc="$(gh repo view --json description --jq .description)"
capture PKG_DESC "$desc"
