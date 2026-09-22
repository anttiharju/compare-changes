# find-changes-action

[![Build](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml/badge.svg)](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml)

People tend to start crafting custom scripts and setups to run logic in GitHub Actions conditionally. What they usually fail at are:

1. Accuracy
2. Performance
3. Composability

across `push`, `pull_request`, `merge_group` events with the `rebase`, `merge commit`, and `squash` merge strategies. Leading to various sorts of annoyances.

They also have a tendency to couple the change detection logic with the custom need at hand, and when that need changes, it's nontrivial to get things right again if one managed to work them out over time.

**`find-changes-action` gets all of this right:**

1. The pesky corner cases of having an accurate list of changes across the different scenarios have been worked out.
2. It runs in seconds
3. It outputs simple JSON, so you're left free to script together the logic **you** need for your use case.
   - You also only ever need to run `find-changes-action` once per (a chain of) workflow(s), and pass around the already-found changes output as a GitHub Actions input.

## Trivial example

```yml
name: find-changes
on: [pull_request]

jobs:
  example:
    runs-on: ubuntu-latest
    steps:
      - name: Find changes
        id: changes
        uses: anttiharju/find-changes-action@v0 # handles checkout

      - name: Echo changed files
        shell: sh
        run: |
          echo ${{ steps.changes.outputs.array }}
        # ["foo/bar", "baz"]
```

In case you are looking for pre-made change comparison action, check out the this action's sibling, [`compare-changes-action`](https://github.com/anttiharju/find-changes-action).

## More information

Refer to https://github.com/anttiharju/compare-changes
