use anstyle::{AnsiColor, Color, Style};
use clap::builder::Styles;

pub fn get_style() -> Styles {
    Styles::styled()
        .usage(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .header(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .literal(Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
}
