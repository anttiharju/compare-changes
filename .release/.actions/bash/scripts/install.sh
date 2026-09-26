#!/usr/bin/env bash
set -euo pipefail

printf 'gh release download "v%s" --repo anttiharju/compare-changes --pattern "compare-changes-%s.tar.gz"\n' "$VERSION" "$TARGET"
gh release download "v$VERSION" --repo anttiharju/compare-changes --pattern "compare-changes-$TARGET.tar.gz"
tar -xzf "compare-changes-$TARGET.tar.gz" compare-changes
rm "compare-changes-$TARGET.tar.gz"
