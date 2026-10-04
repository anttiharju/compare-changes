# compare-changes

[![Build](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml/badge.svg)](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml)

Takes the name of a wildcard workflow (`*` in `.github/workflows/*` incl. file extension) and a JSON array generated with [find-changes-action](https://github.com/anttiharju/find-changes-action) as inputs, to output true/false based on whether any of the `on.push.paths` of the wildcard workflow match a file in the JSON array.

This is useful to introduce job and step granularity to your workflows. One can save a lot of time (and money by reducing runner usage) by executing long-running jobs conditionally.

## More information

There is additional documentation available on

- [crates.io](https://crates.io/crates/compare-changes) and
- [GitHub Actions Marketplace](https://github.com/marketplace/actions/compare-changes).

## Installation

### Cargo

CLI

```sh
cargo install --features=cli compare-changes
```

Library

```sh
cargo add compare-changes
```

### Brew

```sh
brew install anttiharju/tap/compare-changes
```

### Nix

Via [anttiharju's nur-packages](https://github.com/anttiharju/nur-packages). Please note that as of writing it is not connected to the upstream NUR.

## Dependency updates

The weekly [Cargo update workflow](workflows/update-cargo.yml) proposes changes to `Cargo.lock` with the stable toolchain from the CI container.
The publication gate requires every new crates.io version to be at least 72 hours old, including transitive dependencies.
Versions that already exist in the base lockfile do not need another cooldown.

If one version is too young, the workflow skips the whole candidate and retries the next week.
Frequent upstream releases can delay older updates.
Missing or invalid publication metadata and new external sources other than crates.io stop the job.

The workflow reuses the `automation/cargo-update` branch for one open PR with the `dependencies` label.
The existing GitHub App needs Contents and Pull requests write access to this repository.
The workflow uses `ANTTIHARJU_BOT_ID` and `ANTTIHARJU_BOT_PRIVATE_KEY` from the `release` environment.
The environment rules must permit scheduled runs from the default branch.
The CI container includes Python 3.11 or later for TOML and timestamp parsing.

## License

The following licenses apply to this project:

- [docs/github](../docs/github/workflow_syntax.md) are under **Creative Commons Attribution 4.0**, see [docs/github/LICENSE](../docs/github/LICENSE)
- Everything else is under the **MIT License**, see [LICENSE](../LICENSE)
