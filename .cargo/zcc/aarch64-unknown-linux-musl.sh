#!/usr/bin/env bash
set -euo pipefail

zig cc "$@" -target aarch64-linux-musl
