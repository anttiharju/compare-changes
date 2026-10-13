#!/usr/bin/env bash
set -euo pipefail

tag="$1"

jq --join-output '
  (.head_commit.message // "" | split("\n") | .[2:] | join("\n")) as $notes
  | if ($notes | test("\\S")) then $notes else "" end
' "$GITHUB_EVENT_PATH" > "$RUNNER_TEMP/handwritten-release-notes.md"

if [[ ! -s "$RUNNER_TEMP/handwritten-release-notes.md" ]]; then
  exit 0
fi

gh api "repos/$GITHUB_REPOSITORY/commits/$GITHUB_SHA/pulls" > "$RUNNER_TEMP/release-pull-requests.json"
jq --join-output --rawfile notes "$RUNNER_TEMP/handwritten-release-notes.md" '
  if any(.[]; .merge_commit_sha == env.GITHUB_SHA and ($notes | sub("\\s+$"; "")) == .title)
  then "" else $notes end
' "$RUNNER_TEMP/release-pull-requests.json" > "$RUNNER_TEMP/release-note-body.md"

if [[ ! -s "$RUNNER_TEMP/release-note-body.md" ]]; then
  exit 0
fi

gh release view "$tag" --json body > "$RUNNER_TEMP/generated-release-notes.json"
jq --join-output --rawfile notes "$RUNNER_TEMP/release-note-body.md" '
  .body | split("\n") | map(select(startswith("**Full Changelog**: "))) | last
  | if . == null then error("Generated release notes have no Full Changelog link")
    else $notes + "\n\n---\n" + . + "\n" end
' "$RUNNER_TEMP/generated-release-notes.json" > "$RUNNER_TEMP/release-notes.md"

gh release edit "$tag" --notes-file "$RUNNER_TEMP/release-notes.md"
