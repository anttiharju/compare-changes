# find-changes-action

[![Build](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml/badge.svg)](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml)

People tend to start crafting custom scripts and setups to run logic in GitHub Actions conditionally. What they usually fail at are:

1. Accuracy
2. Performance
3. Maintainability
4. Composability

All of this across `push`, `pull_request`, `merge_group` events with the `rebase`, `merge commit`, and `squash` merge strategies. Leading to various sorts of annoyances.

They also have a tendency to couple the change detection logic with the custom need at hand, and when that need changes, it's nontrivial to get things right again if one managed to work them out over time.

**`find-changes-action` gets all of this right:**

1. The pesky corner cases of having an accurate list of changes across the different scenarios have been worked out.
2. It runs in seconds, even on big repositories! (because it doesn't clone all of the git history)
3. It has a built-in validation mode (see [validation example](#validation-example) below)
   - It supports _re-use_ of conditional workflow patterns across compare-changes-action calls and native GitHub Actions by being able to ingest workflows that set `on.push.paths`!
     - Run either `compare-changes --validate` as part of your CI to ensure your patterns stay up-to-date over time and run https://github.com/mpalmer/action-validator (which uses compare-changes the Rust library!) to validate your workflow stubs (see [workflow stub example](#workflow-stub-example) below).
4. It outputs simple JSON, so you're left free to script together the logic **you** need for your use case.
   - You also only ever need to run `find-changes-action` once per (a chain of) workflow(s), and pass around the already-found changes output as a GitHub Actions input.

## Trivial example

```yml
name: find-changes
on: [pull_request]

jobs:
  example:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v7
        with:
          persist-credentials: false

      - name: Find changes
        id: changes
        uses: anttiharju/find-changes-action@v0.12.19

      - name: Echo changed files
        shell: sh
        run: |
          echo ${{ steps.changes.outputs.array }}
        # ["foo/bar", "baz"]
```

In case you are looking for pre-made change comparison action, check out the this action's sibling, [`compare-changes-action`](https://github.com/anttiharju/find-changes-action).

## Validation example

```sh
$ compare-changes --validate
Validating patterns:
✓ 5 in .github/workflows/wildcard/shellcheck.yml
✓ 3 in .github/actions/detect-changes/action.yml:11
✓ 4 in .github/actions/detect-changes/action.yml:19
✓ 3 in .github/actions/detect-changes/action.yml:28
✓ 3 in .github/actions/detect-changes/action.yml:36
✓ 3 in .github/actions/detect-changes/action.yml:44
✓ 3 in .github/actions/detect-changes/action.yml:52
✓ 3 in .github/actions/detect-changes/action.yml:60
✓ 5 in .github/actions/detect-changes/action.yml:68
✓ 7 in .github/actions/detect-changes/action.yml:78
✓ 3 in .github/actions/detect-changes/action.yml:95
✓ 4 in .github/actions/detect-changes/action.yml:103
✓ 4 in .github/actions/detect-changes/action.yml:112
✓ 3 in .github/workflows/plan.yml:34
✓ 53 patterns across 14 files match at least one file!
```

## Workflow stub example

```yml
permissions: {}
on:
  push:
    branches:
      - wildcard
    paths:
      - "**.sh"
      - ".shellcheckrc"
      - "flake.nix"
      - "flake.lock"
      - ".github/workflows/wildcard/shellcheck.yml"
jobs:
  wildcard:
    runs-on: ubuntu-latest
    steps:
      - run: |
          true
```

```yml
name: GitHub Action documentation
permissions: {}
on:
  pull_request:
    paths:
      - ".github/workflows/example.yml"
      - ".github/workflows/wildcard/shellcheck.yml"
      - "**.sh"
      - ".shellcheckrc"
jobs:
  test-example:
    runs-on: ubuntu-latest
    permissions:
      contents: read
    steps:
      - name: Checkout
        uses: actions/checkout@v7
        with:
          persist-credentials: false
      - name: Find changes
        id: changes
        uses: anttiharju/find-changes-action@v0.12.19
      - id: shellcheck
        uses: anttiharju/compare-changes-action@v0.12.19
        with:
          workflow: wildcard/shellcheck.yml # storing the stub workflow under wildcard/ prevents it from polluting your repository's $url/actions UI!
          changes: ${{ steps.changes.outputs.array }}
      - if: steps.shellcheck.outputs.changed == 'true'
        name: shellcheck
        run: git ls-files -z '*.sh' | xargs --null shellcheck --color=always
```

## Checkout

This action assumes that your repository does not have a directory named `.tmp-anttiharju-compare-changes` for how it uses `actions/checkout` internally. It does not replace your existing checkout. A `.gitignore` file with `*` in `.tmp-anttiharju-compare-changes` hides the temporary files from your Git status. The action also removes the temporary directory and its checkout credentials after use, including after a failed step.

If you need repository files, use `actions/checkout` separately. This way composability of the steps is retained and this action does not need to pass through all inputs and outputs of `actions/checkout`.

## More information

Refer to https://github.com/anttiharju/compare-changes
