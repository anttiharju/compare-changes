#!/usr/bin/env bash
set -euo pipefail

workflow_event="${WORKFLOW_EVENT-push}"
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

if [[ -z "$WORKFLOW" && "$workflow_event" != "push" ]]; then
  echo "compare-changes: 'workflow-event' requires 'workflow'" >&2
  exit 1
fi

if [[ -n "$WORKFLOW" ]]; then
  printf 'compare-changes --workflow "%s" --workflow-event "%s" --changes "%s"\n' "$WORKFLOW" "$workflow_event" "$CHANGES"
  "$BINARY" --workflow "$WORKFLOW" --workflow-event "$workflow_event" --changes "$CHANGES" $debug_flag
else
  printf 'compare-changes --paths "<inline>" --changes "%s"\n' "$CHANGES"
  "$BINARY" --paths "$PATHS" --changes "$CHANGES" $debug_flag
fi
