#!/usr/bin/env bash
set -euo pipefail
repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

tag="$1"
target="$2"
echo "$0 $tag $target"

rm -rf "tmp/$target"
repo="$(yq -p toml -oy '.package.name' "$repo_root/Cargo.toml")"
TARGET="$target" CC="./.release/zcc.sh" cargo build --target "$target" --release

cd "target/$target/release"
tar -czf "$repo_root/$repo-$target.tar.gz" "$repo"
