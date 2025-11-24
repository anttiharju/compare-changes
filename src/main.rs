use std::path::PathBuf;

use clap::{arg, command, value_parser};

fn main() {
    command!()
        .arg(
            arg!(
                -w --wildcard <FILE> "Wildcard name, the * in .github/workflows/wildcard-*"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();
}
