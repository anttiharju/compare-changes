# compare-changes

Takes the name of a wildcard workflow and a JSON array generated with [find-changes-action](https://github.com/anttiharju/find-changes-action) as inputs and outputs true/false based on whether any of the `on.push.paths` of the workflow match a file in the JSON array.

This is useful to introduce granularity to your workflows. One can save a lot of time by executing long-running jobs conditionally. An extreme example of what can achieve with the action can be found in this repository's validate job in the [plan workflow](https://github.com/anttiharju/compare-changes/blob/main/.github/workflows/plan.yml).

## Example

This particular example is not the most sensible, because for the use-case illustrated one could just use `on.pull_request.paths`. Where compare-changes improves upon the native workflows syntax, is advanced use-cases of granularity, i.e. chained use of the compare-changes action for different conditions and still defining all jobs as part of the same workflow. This last point allows you to have your branch protection rules only require a finish-ci job, that has all other jobs in its needs. This makes working on the CI a lot simpler because you're free to add/remove jobs without coordinating changes to branch protection rules via repository admins.

```yml
# ./.github/workflows/wildcard-actionlint.yml
permissions:
  contents: none
  pull-requests: none
on:
  push:
    branches:
      - wildcard
    paths:
      - ".github/workflows/*.yml"
      - ".github/workflows/*.yaml"
jobs:
  wildcard:
    runs-on: ubuntu-latest
    if: false
    steps:
      - run: |
          true
```

```yml
# ./.github/workflows/example.yml
on: [pull_request]
jobs:
  test:
    runs-on: ubuntu-24.04
    steps:
      - name: Find changes
        id: changes
        uses: anttiharju/find-changes-action@v1 # handles checkout
      - id: actionlint
        uses: anttiharju/compare-changes@v0
        with:
          github-workflows-wildcard: actionlint.yml
          changes: ${{ steps.changes.outputs.array }}
      - if: steps.actionlint.outputs.changed == 'true'
        name: actionlint
        uses: anttiharju/actions/actionlint@v0
```
