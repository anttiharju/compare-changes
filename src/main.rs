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
    command!()
        .about("Outputs changed=true on first match of the wildcard's on.push.paths. Otherwise outputs changed=false.")
        .styles(get_styles())
        .arg(
            arg!(
                -w --wildcard <FILE> "Wildcard name, * in .github/workflows/wildcard-*"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        // JSON format for -c --changes: ["foo/bar", "baz"]
        .get_matches();
}
