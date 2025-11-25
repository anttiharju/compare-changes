use clap::{arg, command, value_parser};
use std::path::PathBuf;

mod style;
use style::get_style;

pub struct Args {
    pub wildcard: PathBuf,
    pub changes_json: String,
}

pub fn parse_args() -> Args {
    let matches = command!()
        .about("Outputs changed=true on first match of the wildcard's on.push.paths. Otherwise outputs changed=false.")
        .styles(get_style())
        .arg(
            arg!(
                -w --wildcard <FILE> "Wildcard name, * in .github/workflows/wildcard-*"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -c --changes <JSON> r#"JSON array string, for example '["foo/bar", "baz"]'"#
            )
            .required(true)
            .value_parser(value_parser!(String)),
        )
        .get_matches();

    Args {
        wildcard: matches.get_one::<PathBuf>("wildcard").unwrap().clone(),
        changes_json: matches.get_one::<String>("changes").unwrap().clone(),
    }
}
