# compare-changes

[![Build](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml/badge.svg)](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml)

Takes the name of a wildcard workflow (`*` in `.github/workflows/wildcard-*` incl. file extension) and a JSON array generated with [find-changes-action](https://github.com/anttiharju/find-changes-action) as inputs, to output true/false based on whether any of the `on.push.paths` of the wildcard workflow match a file in the JSON array.

This is useful to introduce job and step granularity to your workflows. One can save a lot of time (and money by reducing runner usage) by executing long-running jobs conditionally.

## Trivial example

The same result could be achieved with the use of `on.pull_request.paths`, but the purpose here is to provide a minimal example. The real value comes from advanced use-cases of granularity, i.e. chained use of the compare-changes action for different conditions and still defining all jobs as part of the same workflow.

By having all jobs sourced from the same event (`on.pull_request`) allows one to have their branch protection rules only require a finish-ci job, which has all other jobs in its `needs:`. This makes working on the CI a lot simpler because you're free to add/remove jobs without coordinating changes to branch protection rules via repository admins. It is advisable to write a a `finish-ci` `needs:` validator (for example in Python) to ensure proper coverage.

A good non-trivial example can be found in this repository's validate job in the [plan workflow](https://github.com/anttiharju/compare-changes/blob/main/.github/workflows/plan.yml).

```yml
# .github/workflows/example.yml
on: [pull_request]
jobs:
  test:
    runs-on: ubuntu-24.04
    permissions:
      contents: read
    steps:
      - name: Find changes
        id: changes
        uses: anttiharju/find-changes-action@v2 # handles checkout
      - id: shellcheck
        uses: anttiharju/compare-changes-action@v0
        with:
          github-workflows-wildcard: shellcheck.yml # see .github/workflows/wildcard-shellcheck.yml below
          changes: ${{ steps.changes.outputs.array }}
      - if: steps.shellcheck.outputs.changed == 'true'
        name: shellcheck
        run: git ls-files -z '*.sh' | xargs --null shellcheck --color=always
```

```yml
# .github/workflows/wildcard-shellcheck.yml
permissions: {}
on:
  push:
    branches:
      - wildcard
    paths:
      - ".github/workflows/wildcard-shellcheck.yml"
      - "**.sh"
      - ".shellcheckrc"
jobs:
  wildcard:
    runs-on: ubuntu-latest
    steps:
      - run: |
          true
```

## Notes

Note that this action does not support negation `!` in paths. It evaluates each path individually, instead of all as a group.

## More information

Please refer to [the GitHub README.md](https://github.com/anttiharju/compare-changes/blob/main/.github/README.md)
