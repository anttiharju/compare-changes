#!/usr/bin/env bash
set -euo pipefail
repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

tag="$1"
target="$2"
echo "$0 $tag $target"

rm -rf "tmp/$target"
remote_url="$(git remote get-url origin)"
repo="$(basename --suffix .git "$remote_url")"
cargo build --target "$target" --release

cd "target/$target/release"
tar -czf "$repo_root/$repo-$target.tar.gz" "$repo"
