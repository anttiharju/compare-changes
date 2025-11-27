# compare-changes

compare-changes is a CLI and a library:

1. The CLI can be used together with [find-changes-action](https://github.com/anttiharju/find-changes-action) to run GitHub Actions jobs or steps conditionally, while benefitting from static validation for the defined paths via [action-validator](https://github.com/mpalmer/action-validator). For more information, view the action on [GitHub Actions Marketplace](https://github.com/marketplace/actions/compare-changes).
2. The library reimplements the [GitHub Actions Workflow syntax](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#filter-pattern-cheat-sheet) so other Rust projects may check whether paths in a given workflow file match a given set of files.

## Contributing

Please refer to [the GitHub README.md](https://github.com/anttiharju/compare-changes/blob/main/README.md)

## TODO

https://rust-lang.github.io/api-guidelines/checklist.html

### Documentation

Crate is abundantly documented:

- [ ] Crate level docs are thorough and include examples (C-CRATE-DOC)
- [ ] All items have a rustdoc example (C-EXAMPLE)
- [ ] Examples use ?, not try!, not unwrap (C-QUESTION-MARK)
- [ ] Function docs include error, panic, and safety considerations (C-FAILURE)
- [ ] Prose contains hyperlinks to relevant things (C-LINK)

### Flexibility

Crate supports diverse real-world use cases:

- [ ] Functions expose intermediate results to avoid duplicate work (C-INTERMEDIATE)
- [ ] Caller decides where to copy and place data (C-CALLER-CONTROL)
- [ ] Functions minimize assumptions about parameters by using generics (C-GENERIC)
