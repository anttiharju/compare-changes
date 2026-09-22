# compare-changes-action

[![Build](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml/badge.svg)](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml)

Takes a workflow file under `.github/workflows/` and a JSON array generated with [find-changes-action](https://github.com/anttiharju/find-changes-action) as inputs, to output true/false based on whether any of the `on.push.paths` of the workflow match a file in the JSON array.

This is useful to introduce job and step granularity to your workflows. One can save a lot of time (and money by reducing runner usage) by executing long-running jobs conditionally.

## Trivial example

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
        uses: anttiharju/find-changes-action@v0 # handles checkout
      - id: shellcheck
        uses: anttiharju/compare-changes-action@v0
        with:
          workflow: wildcard/shellcheck.yml # see .github/workflows/wildcard/shellcheck.yml below
          changes: ${{ steps.changes.outputs.array }}
      - id: shellcheck
        uses: anttiharju/compare-changes-action@v0
        with:
          paths: |
            **.sh
            .shellcheckrc
          changes: ${{ inputs.changes }}
      - if: steps.shellcheck.outputs.changed == 'true'
        name: shellcheck
        run: |
          git ls-files -z '*.sh' | xargs --null shellcheck --color=always
```

Equivalent of the above _can_ be achieved with `on.pull_request.paths`, but the purpose there was to provide a minimal example. `compare-changes-action` is built for advanced use-cases, such as consequtive calls to `compare-changes-action` for extreme step granularity in a CI job. An example can be found in the `compare-changes` repository itself: https://github.com/anttiharju/compare-changes/blob/a11c5fa661f74e12ab476ed8671988e9c64e6892/.github/actions/detect-changes/action.yml

While step-level granularity is a relatively niche use-case, a significantly less niche use-case is to run jobs conditionally in the same workflow. While one could still use `on.pull_request.paths` and setup branch protection rules to require all conditional workflows to pass (pro tip: `skipped` counts as a pass), what one loses there is the ability to have _dependencies_ between long-running jobs.

**With `compare-changes-action` one can define dependencies between jobs using the standard `needs:` GitHub Actions syntax, while still skipping long-running ones based on the files that have changed.**

A very extendable and easy-to-modify monorepo Pull Request workflow can look like this:

```
1. Job that runs find-changes-action and all compare-changes-action steps, so that the conditional `if:` logic is available in step 2
2. (whatever other jobs you actually want to run)
3. A finish-ci job that fails if any of the previous jobs have failed. It's a bright idea to run a validator that checks that all others jobs are listed in its needs.
```

The setup described above makes it fairly simple to work on the CI because one is free to add/remove jobs without coordinating changes to branch protection rules with repository admins.

## More information

Refer to https://github.com/anttiharju/compare-changes
