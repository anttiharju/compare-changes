use clap::Parser;
use std::path::PathBuf;

mod style;
use style::get_style;

#[derive(Parser)]
#[command(
    version,
    about = "Outputs changed=true on first match of the wildcard's on.push.paths. Otherwise outputs changed=false.",
    styles = get_style()
)]
pub struct Args {
    /// Wildcard name, * in .github/workflows/wildcard-*
    #[arg(short, long, value_name = "FILE")]
    pub wildcard: PathBuf,

    /// JSON array string, for example '["foo/bar", "baz"]'
    #[arg(short, long, value_name = "JSON")]
    pub changes: String,

    /// Enable debug output
    #[arg(short, long)]
    pub debug: bool,
}

pub fn parse_args() -> Args {
    let mut args = Args::parse();

    // Apply the prefix transformation to wildcard
    let prefixed = PathBuf::from(".github/workflows").join(format!("wildcard-{}", args.wildcard.display()));
    args.wildcard = prefixed;

    args
}
