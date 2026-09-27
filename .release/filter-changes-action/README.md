# filter-changes-action

[![Build](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml/badge.svg)](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml)

This action filters a JSON array of changed files with one GitHub Actions path pattern.
The `array` output contains only matching files, in their original order.

The action accepts the output from [find-changes-action](https://github.com/anttiharju/find-changes-action).
Its output works as the `changes` input for [compare-changes-action](https://github.com/anttiharju/compare-changes-action).
The filter action does not require a repository checkout.

## Usage

```yml
on: [pull_request]
jobs:
  shellcheck:
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
        uses: anttiharju/find-changes-action@v0

      - id: filter
        uses: anttiharju/filter-changes-action@v0
        with:
          changes: ${{ steps.changes.outputs.array }}
          filter: .github/** # we are only interested in CI shell scripts

      - id: shellcheck
        uses: anttiharju/compare-changes-action@v0
        with:
          changes: ${{ steps.filter.outputs.array }}
          paths: |
            **.sh
            .shellcheckrc

      - if: steps.shellcheck.outputs.changed == 'true'
        name: shellcheck
        run: git ls-files -z '.github/*.sh' | xargs --null shellcheck --color=always
```

In this example, only changes under `.github/` can trigger the ShellCheck command.
The ShellCheck command checks tracked shell scripts under `.github/` when the step runs.

## Inputs

| Input     | Required | Description                                             |
| --------- | -------- | ------------------------------------------------------- |
| `changes` | Yes      | JSON array of changed file paths.                       |
| `filter`  | Yes      | One path pattern, applied to each file independently.   |
| `debug`   | No       | Enable diagnostic output. The default value is `false`. |

## Output

The `array` output contains a JSON array of matching file paths.
If no files match, the output is `[]`.
The action evaluates each file path independently against the `filter` pattern with the compare-changes library.
Matching file paths remain unchanged.

## More Information

The source code and release workflows are in [compare-changes](https://github.com/anttiharju/compare-changes).
