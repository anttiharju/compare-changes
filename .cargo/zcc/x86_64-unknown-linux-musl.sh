#!/usr/bin/env bash
set -euo pipefail

zig cc "$@" -target x86_64-linux-musl
