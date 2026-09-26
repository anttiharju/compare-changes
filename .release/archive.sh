#!/usr/bin/env bash
set -euo pipefail
repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

target="$1"
echo "$0 $target"

remote_url="$(git remote get-url origin)"
repo="$(basename --suffix .git "$remote_url")"
cargo build --locked --all-features --target "$target" --release

cd "target/$target/release"
tar -czf "$repo_root/$repo-$target.tar.gz" "$repo"

directory="$repo_root/build/$VERSION/$target"
mkdir -p "$directory"
cp "$repo" "$directory/$repo"
mv "$repo-$target.tar.gz" "$directory/$repo"
