#!/usr/bin/env bash

printf 'compare-changes --workflow "%s" --changes "%s"\n' "$WORKFLOW" "$CHANGES"
debug_flag=""
if [[ "$DEBUG" = "true" ]]; then
  debug_flag="--debug"
fi
compare-changes --workflow "$WORKFLOW" --changes "$CHANGES" $debug_flag
