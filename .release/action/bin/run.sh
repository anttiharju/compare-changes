#!/usr/bin/env sh

printf 'compare-changes --wildcard "%s" --changes "%s"\n' "$WILDCARD" "$CHANGES"
debug_flag=""
if [ "$DEBUG" = "true" ]; then
  debug_flag="--debug"
fi
compare-changes --wildcard "$WILDCARD" --changes "$CHANGES" $debug_flag
