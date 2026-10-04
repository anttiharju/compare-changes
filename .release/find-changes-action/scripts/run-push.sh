#!/usr/bin/env bash
set -euo pipefail

authorization="$(printf 'x-access-token:%s' "$GITHUB_TOKEN" | base64 | tr -d '\n')"
printf '::add-mask::%s\n' "$authorization"
GIT_CONFIG_VALUE_0="AUTHORIZATION: basic $authorization" "$BINARY" --find
