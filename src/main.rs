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
        .about("Compare an array of JSON changes to on.push.paths of a .github/workflows/wildcard-* file.")
        .styles(get_styles())
        .arg(
            arg!(
                -w --wildcard <FILE> "Wildcard name, the * in .github/workflows/wildcard-*"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();
}
