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
output=".release"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --no-cache) export NO_CACHE=1; shift ;;
    --output|-o) output="$2"; shift 2 ;;
    *) echo "Error: Unknown param: $1" >&2; exit 1 ;;
  esac
done

# Setup env
[[ -z "${GITHUB_REPOSITORY:-}" ]] && source actions_env_mock.sh

# Paths
cache="$pkg/values.cache"
cache_key="$pkg.cache"

# Check if values.sh changed
calculate_key() {
  local file="$1"
  branch=$(git rev-parse --abbrev-ref HEAD)
  hash=$(hashsum --sha256 "$file" | cut -d ' ' -f1)
  echo "$branch-$hash"
}

if [[ -f "$cache_key" ]]; then
  current_key=$(calculate_key "$pkg/values.sh")
  previous_key=$(cat "$cache_key")
  [[ "$current_key" != "$previous_key" ]] && export NO_CACHE=1
else
  export NO_CACHE=1
fi

# Render
calculate_key "$pkg/values.sh" > "$cache_key"
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
repo_root="$(git rev-parse --show-toplevel)"
envsubst -i "template.$ext" -no-unset -no-empty > "$repo_root/$output/$filename.$ext"
cp "template.$ext" "$filename.tpl.$ext" # easier to visually diff two gitignored files
