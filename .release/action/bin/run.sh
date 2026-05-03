#!/usr/bin/env bash

printf 'compare-changes --wildcard "%s" --changes "%s"\n' "$WORKFLOW" "$CHANGES"
debug_flag=""
if [[ "$DEBUG" = "true" ]]; then
  debug_flag="--debug"
fi
compare-changes --wildcard "$WORKFLOW" --changes "$CHANGES" $debug_flag
