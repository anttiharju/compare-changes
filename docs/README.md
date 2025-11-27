# compare-changes

This is the library-focused README.

## TODO

https://rust-lang.github.io/api-guidelines/checklist.html

### Documentation

Crate is abundantly documented:

- [ ] Crate level docs are thorough and include examples (C-CRATE-DOC)
- [ ] All items have a rustdoc example (C-EXAMPLE)
- [ ] Examples use ?, not try!, not unwrap (C-QUESTION-MARK)
- [ ] Function docs include error, panic, and safety considerations (C-FAILURE)
- [ ] Prose contains hyperlinks to relevant things (C-LINK)
- [x] Cargo.toml includes all common metadata (C-METADATA)

### Flexibility

Crate supports diverse real-world use cases:

- [ ] Functions expose intermediate results to avoid duplicate work (C-INTERMEDIATE)
- [ ] Caller decides where to copy and place data (C-CALLER-CONTROL)
- [ ] Functions minimize assumptions about parameters by using generics (C-GENERIC)
