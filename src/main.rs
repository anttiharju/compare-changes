use anstyle::{AnsiColor, Color, Style};
use clap::{arg, builder::Styles, command, value_parser};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Wildcard {
    on: Option<On>,
}

#[derive(Debug, Deserialize)]
struct On {
    push: Option<Push>,
}

#[derive(Debug, Deserialize)]
struct Push {
    paths: Option<Vec<String>>,
}

fn get_styles() -> Styles {
    Styles::styled()
        .usage(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .header(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .literal(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Cyan))),
        )
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
}

fn main() {
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

    let wildcard = matches.get_one::<PathBuf>("wildcard").unwrap();
    let wildcard_path =
        PathBuf::from(".github/workflows").join(format!("wildcard-{}", wildcard.display()));
    let wildcard_contents =
        fs::read_to_string(&wildcard_path).expect("Failed to read wildcard file");

    let wildcard: Wildcard =
        serde_saphyr::from_str(&wildcard_contents).expect("Failed to parse YAML");

    println!("{:#?}.on.push.paths:", wildcard_path);
    if let Some(paths) = wildcard
        .on
        .and_then(|on| on.push.and_then(|push| push.paths))
    {
        for path in paths {
            println!("- {}", path);
        }
    } else {
        println!("No on.push.paths found.");
    }

    let changes_json = matches.get_one::<String>("changes").unwrap();
    let changes: Vec<String> =
        serde_json::from_str(changes_json).expect("Failed to parse changes JSON array");
    println!("changes:");
    for change in &changes {
        println!("- {}", change);
    }
}
