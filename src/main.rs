use std::path::PathBuf;

use clap::{arg, command, value_parser};

fn main() {
    let matches = command!()
        .arg(
            arg!(
                -w --wildcard <FILE> "Wildcard name, the * in .github/workflows/wildcard-*"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("Value for config: {}", config_path.display());
    }

    // Continued program logic goes here...
}
