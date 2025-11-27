use clap::{arg, command, value_parser};
use std::path::PathBuf;

mod style;
use style::get_style;

pub struct Args {
    pub wildcard: PathBuf,
    pub changes_json: String,
    pub debug: bool,
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
        .arg(
            arg!(
                -d --debug "Enable debug output"
            )
            .required(false),
        )
        .get_matches();

    let raw_wildcard = matches.get_one::<PathBuf>("wildcard").unwrap().clone();
    let prefixed = PathBuf::from(".github/workflows").join(format!("wildcard-{}", raw_wildcard.display()));

    Args {
        wildcard: prefixed,
        changes_json: matches.get_one::<String>("changes").unwrap().clone(),
        debug: matches.get_flag("debug"),
    }
}
