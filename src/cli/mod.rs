use clap::Parser;
use std::path::PathBuf;

mod style;
use style::get_style;

#[derive(Parser)]
#[command(
    version,
    about = "Compare wildcard paths to changed files, or detect changed files from GitHub event context with --find.",
    styles = get_style()
)]
pub struct Args {
    /// Find changed files from the git diff base inferred from GitHub Actions event context
    #[arg(short, long, default_value_t = false, conflicts_with_all = ["source", "changes"])]
    pub find: bool,

    #[command(flatten)]
    pub source: Source,

    /// JSON array string, for example '["foo/bar", "baz"]'
    #[arg(short, long, value_name = "JSON", required_unless_present = "find")]
    pub changes: Option<String>,

    /// Enable debug output
    #[arg(short, long)]
    pub debug: bool,
}

#[derive(clap::Args)]
#[group(id = "source", required = true, multiple = false)]
pub struct Source {
    /// Workflow file under .github/workflows/
    #[arg(short, long, value_name = "FILE")]
    pub workflow: Option<PathBuf>,

    /// Newline-separated inline path patterns (alternative to --workflow)
    #[arg(short, long, value_name = "PATHS")]
    pub paths: Option<String>,
}

pub fn parse_args() -> Args {
    let mut args = Args::parse();

    if let Some(workflow) = args.source.workflow.take() {
        // Apply the prefix transformation to workflow
        let prefixed = PathBuf::from(".github/workflows").join(workflow);
        args.source.workflow = Some(prefixed);
    }

    args
}
