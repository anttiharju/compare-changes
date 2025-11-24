use std::path::PathBuf;

use clap::{arg, command, value_parser};

fn main() {
    command!()
        .about(
            "Compare an array of JSON changes to on.push.paths of a .github/workflows/wildcard-* file.",
        )
        .arg(
            arg!(
                -w --wildcard <FILE> "Wildcard name, the * in .github/workflows/wildcard-*"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();
}
