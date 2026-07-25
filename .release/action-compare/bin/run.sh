#!/usr/bin/env bash

debug_flag=""
if [[ "$DEBUG" = "true" ]]; then
  debug_flag="--debug"
fi

if [[ -n "$WORKFLOW" && -n "$PATHS" ]]; then
  echo "compare-changes: only one of 'workflow' or 'paths' input may be set" >&2
  exit 1
fi

if [[ -z "$WORKFLOW" && -z "$PATHS" ]]; then
  echo "compare-changes: one of 'workflow' or 'paths' input must be set" >&2
  exit 1
fi

if [[ -n "$WORKFLOW" ]]; then
  printf 'compare-changes --workflow "%s" --changes "%s"\n' "$WORKFLOW" "$CHANGES"
  compare-changes --workflow "$WORKFLOW" --changes "$CHANGES" $debug_flag
else
  printf 'compare-changes --paths "<inline>" --changes "%s"\n' "$CHANGES"
  compare-changes --paths "$PATHS" --changes "$CHANGES" $debug_flag
fi
