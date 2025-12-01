# compare-changes

[![Build](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml/badge.svg)](https://github.com/anttiharju/compare-changes/actions/workflows/build.yml)

Takes the name of a wildcard workflow (`*` in `.github/workflows/wildcard-*` incl. file extension) and a JSON array generated with [find-changes-action](https://github.com/anttiharju/find-changes-action) as inputs, to output true/false based on whether any of the `on.push.paths` of the wildcard workflow match a file in the JSON array.

This is useful to introduce job and step granularity to your workflows. One can save a lot of time (and money by reducing runner usage) by executing long-running jobs conditionally.

## More information

There is additional documentation available on

- [crates.io](https://crates.io/crates/compare-changes) and
- [GitHub Actions Marketplace](https://github.com/marketplace/actions/compare-changes).

## Installation

### Cargo

### CLI

```sh
cargo install --features=cli compare-changes
```

### Library

```sh
cargo add compare-changes
```

### Brew

```sh
brew install anttiharju/tap/compare-changes
```

### Nix

Via [anttiharju's nur-packages](https://github.com/anttiharju/nur-packages). Please note that as of writing it is not connected to the upstream NUR.

## TODO

- [ ] Improve instructions on how to contribute.
- [ ] Produce static Linux binaries for wider compatibility with GitHub runners
  - [ ] Setup benchmarks, compare allocators
- [ ] Release binaries for Windows ARM64/X64 and macOS X64 ([to support all github-hosted runners](https://docs.github.com/en/actions/reference/runners/github-hosted-runners#standard-github-hosted-runners-for-public-repositories))

### crates.io

https://rust-lang.github.io/api-guidelines/checklist.html

#### Documentation

Crate is abundantly documented:

- [ ] Crate level docs are thorough and include examples (C-CRATE-DOC)
- [ ] All items have a rustdoc example (C-EXAMPLE)
- [ ] Examples use ?, not try!, not unwrap (C-QUESTION-MARK)
- [ ] Function docs include error, panic, and safety considerations (C-FAILURE)
- [ ] Prose contains hyperlinks to relevant things (C-LINK)

#### Flexibility

Crate supports diverse real-world use cases:

- [ ] Caller decides where to copy and place data (C-CALLER-CONTROL)
- [ ] Functions minimize assumptions about parameters by using generics (C-GENERIC)

## Contributing

Refer to [CONTRIBUTING.md](../CONTRIBUTING.md).

## Stargazers over time

Starring the repository is helpful for releasing the project on upstream package managers.

[![Stargazers over time](https://starchart.cc/anttiharju/compare-changes.svg?variant=adaptive)](https://starchart.cc/anttiharju/compare-changes)

## License

The following licenses apply to this project:

- [docs/github](../docs/github/workflow_syntax.md) are under **Creative Commons Attribution 4.0**, see [docs/github/LICENSE](../docs/github/LICENSE)
- Everything else is under the **MIT License**, see [LICENSE](../LICENSE)
