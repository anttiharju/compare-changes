# Introduction

`rust-starter` is a template for my Rust projects to make it easier to start new ones.

## Rust annoyances coming from Go

- Cargo.toml wants to specify the version, label-based versioning difficult (I don't care about the exact number, only about the type of change, this seems like a very inconvinient way of versioning)
- Cross-compiling is not straight-forward, apparently doable via rustup but people don't seem to really do it but just prefer wasteful platform-native github runners. Rustup compatibility with Nix remains unclear.

I would like to cross-compile to:

- aarch64-apple-darwin
- aarch64-unknown-linux-gnu
- x86_64-unknown-linux-gnu

from x86_64-unknown-linux-gnu. Copilot says cross-compiling from Linux to macOS will be difficult.
