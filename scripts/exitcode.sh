#!/usr/bin/env bash
set -euo pipefail

EXITCODE_FILE="src/exitcode/mod.rs"
errors=0

if [[ ! -f $EXITCODE_FILE ]]; then
  echo "exitcode file not found: $EXITCODE_FILE"
  exit 1
fi

# Extract lines between 'define_exitcodes! {' and the closing '}'
exitcode_lines=$(awk '/define_exitcodes! *\{/{flag=1; next} /\}/{flag=0} flag' "$EXITCODE_FILE")

# Extract function names using rg, then mapfile
fn_lines=$(echo "$exitcode_lines" | rg '^\s*([a-zA-Z0-9_]+)\s*=>' -or '$1')
mapfile -t fn_names <<< "$fn_lines"

# Check each function name for exactly one call in the src directory
for fn in "${fn_names[@]}"; do
  num=$(rg -o "exitcode::${fn}\\(" src | wc -l || true)
  if [[ "$num" -ne 1 ]]; then
    echo "expected exactly 1 call to exitcode::${fn}, found ${num}"
    errors=$((errors+1))
  fi
done

if [[ "$errors" -ne 0 ]]; then
  exit 1
fi
