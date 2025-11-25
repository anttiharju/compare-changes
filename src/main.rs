use anstyle::{AnsiColor, Color, Style};
use clap::{arg, builder::Styles, command, value_parser};
use std::path::PathBuf;

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

    let wildcard_path = matches.get_one::<PathBuf>("wildcard").unwrap(); // Safe unwrap due to required argument
    let full_path =
        PathBuf::from(".github/workflows").join(format!("wildcard-{}", wildcard_path.display()));
    println!("Wildcard file path: {:?}", full_path);

    let changes_json = matches.get_one::<String>("changes").unwrap(); // Safe unwrap due to required argument
    let changes: Vec<String> =
        serde_json::from_str(changes_json).expect("Failed to parse changes JSON array");
    println!("changes:");
    for change in &changes {
        println!("- {}", change);
    }
}
