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
output=".release/$pkg/.output"
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

  if [[ "$TAG" = "v0.0.0" ]]; then
    rev="$(gh api "repos/$GITHUB_REPOSITORY/commits/HEAD" --jq '.sha')"
  else
    rev="$(gh api "repos/$GITHUB_REPOSITORY/git/ref/tags/$TAG" --jq '.object.sha')"
  fi
  export GITHUB_SHA="$rev"
}

# Setup env
tag="$(git tag --sort=-creatordate | head -n1)"
tag="${tag:-v0.0.0}"
export TAG="$tag"

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
if [[ -f "$cache" && -z "${NO_CACHE:-}" && ! "$pkg/values.sh" -nt "$cache" ]]; then
  cat "$cache"
else
  # shellcheck source=/dev/null
  source "$pkg/values.sh" | tee "$cache"
fi

cd -P "$pkg"
# shellcheck source=/dev/null
source "values.cache"
filename="$PKG_FILENAME"
ext="$PKG_EXTENSION"
[[ "$output" == /* ]] || output="$repo_root/$output"
mkdir -p "$output"
output="$(cd "$output" && pwd -P)"
read -r -a output_sources <<< "${PKG_OUTPUT:-*}"
output_patterns=()
for path in "${output_sources[@]}"; do
  [[ "$path" == ../* ]] && path="${path##*/}"
  output_patterns+=("${path%/}")
  find "$output" -mindepth 1 -name .git -prune -o -path "$output/${path%/}" -prune -exec rm -rf -- {} +
done

write_output() {
  local source="$1" path="$2" pattern
  for pattern in "${output_patterns[@]}"; do
    case "$path/" in
      $pattern/*)
        mkdir -p "$output/$(dirname "./$path")"
        if [[ "$source" == "template.$ext" ]]; then
          envsubst -i "$source" -no-unset -no-empty > "$output/$path"
        else
          cp -p "$source" "$output/$path"
        fi
        return
        ;;
    esac
  done
}

write_output "template.$ext" "$filename.$ext"
write_output "$repo_root/LICENSE" LICENSE
git ls-files -z --cached --others --exclude-standard -- . |
  while IFS= read -r -d '' path; do
    case "$path" in
      .*|*/.*|values.sh|"template.$ext"|"${output#"$PWD"/}"/*) continue ;;
    esac
    [[ -f "$path" ]] || continue
    write_output "./$path" "$path"
  done
for path in "${output_sources[@]}"; do
  if [[ "$path" == ../* ]]; then
    write_output "$path" "${path##*/}"
  fi
done
