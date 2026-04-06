#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")" # normalize working directory so caller wd does not matter

# Validate pkg as enum
pkg="${1:-}"
shift
if [[ -z "$pkg" ]] || [[ ! -d "$pkg" ]]; then
  pkgs=(*/)
  pkgs=("${pkgs[@]%/}")
  echo "Usage: $0 <package> [--no-cache] [--output|-o <path>]"
  echo "Valid packages: ${pkgs[*]}"
  exit 1
fi

# Parse flags
output=".release/$pkg"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --no-cache) export NO_CACHE=1; shift ;;
    --output|-o) output="$2"; shift 2 ;;
    *) echo "Error: Unknown param: $1" >&2; exit 1 ;;
  esac
done

mock_github_actions_env() {
  #remote_url=https://example.com/owner/repository.git
  #remote_url=git@example.com:owner/repository.git
  remote_url="$(git remote get-url origin)"

  local normalized_url="${remote_url/://}"
  local temp="${normalized_url%/*}"
  owner="$(basename "$temp")"

  repo="$(basename --suffix .git "$remote_url")"
  export GITHUB_REPOSITORY="$owner/$repo"

  repo_root="$(git rev-parse --show-toplevel)"
  tag="v$(toml get "$repo_root/Cargo.toml" package.version --raw)"
  if gh api "repos/$GITHUB_REPOSITORY/git/ref/tags/$tag" &>/dev/null; then
    rev="$(gh api "repos/$GITHUB_REPOSITORY/git/ref/tags/$tag" --jq '.object.sha')"
  else
    rev="$(gh api "repos/$GITHUB_REPOSITORY/commits/HEAD" --jq '.sha')"
  fi
  export GITHUB_SHA="$rev"
}

# Setup env
[[ -z "${GITHUB_REPOSITORY:-}" ]] && mock_github_actions_env

# Paths
cache="$pkg/values.cache"
cache_key="$pkg/template.cache"
repo_root="$(git rev-parse --show-toplevel)"

# Check if values.sh changed
calculate_key() {
  local pkg="$1"
  content=$(git log -1 --format=%H -- "$repo_root/.release/$pkg" "$repo_root/.release/render.sh")
  tag=$(git describe --tags --abbrev=0 2>/dev/null || echo "no_tag")
  echo "$tag-$content"
}

if [[ -f "$cache_key" ]]; then
  current_key=$(calculate_key "$pkg")
  previous_key=$(cat "$cache_key")
  [[ "$current_key" != "$previous_key" ]] && export NO_CACHE=1
else
  export NO_CACHE=1
fi

# Render
calculate_key "$pkg" > "$cache_key"
if [[ -f "$cache" && -z "${NO_CACHE:-}" ]]; then
  cat "$cache"
else
  # shellcheck source=/dev/null
  source "$pkg/values.sh" | tee "$cache"
fi

cd "$pkg"
# shellcheck source=/dev/null
source "values.cache"
filename="$PKG_FILENAME"
ext="$PKG_EXTENSION"
mkdir -p "$repo_root/$output"
envsubst -i "template.$ext" -no-unset -no-empty > "$repo_root/$output/$filename.$ext"
if [[ "$output" == ".release/$pkg" ]]; then
  cp "$repo_root/$output/template.$ext" "$repo_root/$output/$filename.tpl.$ext" # easier to visually diff two gitignored files
fi
