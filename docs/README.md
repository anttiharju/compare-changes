# compare-changes

[![Build](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml/badge.svg)](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml)

compare-changes is a CLI and a library:

1. The CLI can be used together with [find-changes-action](https://github.com/anttiharju/find-changes-action) to run GitHub Actions jobs or steps conditionally, while benefitting from static validation for the defined paths via [action-validator](https://github.com/mpalmer/action-validator) \[it even uses this library since v0.9.0, so results should be 1:1!\]. For more information, view the action on [GitHub Actions Marketplace](https://github.com/marketplace/actions/compare-changes).
2. The library reimplements the [GitHub Actions Workflow syntax](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#filter-pattern-cheat-sheet) so other Rust projects may check whether paths in a given workflow file match a given set of files.

## More information

For information on alternative installation methods and on how to contribute, please refer to [the GitHub README.md](https://github.com/anttiharju/compare-changes/blob/main/.github/README.md)
