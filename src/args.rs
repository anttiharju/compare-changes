use clap::{arg, builder::Styles, command, value_parser};
use std::path::PathBuf;

pub struct Args {
    pub wildcard: PathBuf,
    pub changes_json: String,
}

pub fn parse_args() -> Args {
    let matches = command!()
        .about("Outputs changed=true on first match of the wildcard's on.push.paths. Otherwise outputs changed=false.")
        .styles(get_styles())
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

fn get_styles() -> Styles {
    Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .literal(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan))),
        )
}
