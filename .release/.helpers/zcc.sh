#!/usr/bin/env bash
"$(which zig)" cc -target "$ZIG_TARGET" "$@"
