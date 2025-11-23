#!/usr/bin/env bash
set -euo pipefail

#remote_url=https://example.com/owner/repository.git
#remote_url=git@example.com:owner/repository.git
remote_url="$(git remote get-url origin)"

normalized_url="${remote_url/://}"
temp="${normalized_url%/*}"
owner="$(basename "$temp")"

repo_root="$(git rev-parse --show-toplevel)"
repo="$(yq -p toml -oy '.package.name' "$repo_root/Cargo.toml")"

export GITHUB_REPOSITORY="$owner/$repo"

tag="v$(yq -p toml -oy '.package.version' "$repo_root/Cargo.toml")"
if gh api "repos/$GITHUB_REPOSITORY/git/ref/tags/$tag" &>/dev/null; then
  rev="$(gh api "repos/$GITHUB_REPOSITORY/git/ref/tags/$tag" --jq '.object.sha')"
else
  rev="$(gh api "repos/$GITHUB_REPOSITORY/commits/HEAD" --jq '.sha')"
fi
export GITHUB_SHA="$rev"
