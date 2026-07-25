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
    #[arg(short, long, default_value_t = false, conflicts_with_all = ["workflow", "paths", "changes"])]
    pub find: bool,

    /// Workflow file under .github/workflows/
    #[arg(
        short,
        long,
        value_name = "FILE",
        required_unless_present_any = ["find", "paths"],
        conflicts_with = "paths",
    )]
    pub workflow: Option<PathBuf>,

    /// Newline-separated inline path patterns (alternative to --workflow)
    #[arg(
        short,
        long,
        value_name = "PATHS",
        required_unless_present_any = ["find", "workflow"],
        conflicts_with = "workflow",
    )]
    pub paths: Option<String>,

    /// JSON array string, for example '["foo/bar", "baz"]'
    #[arg(short, long, value_name = "JSON", required_unless_present = "find")]
    pub changes: Option<String>,

    /// Enable debug output
    #[arg(short, long)]
    pub debug: bool,
}

pub fn parse_args() -> Args {
    let mut args = Args::parse();

    if let Some(workflow) = args.workflow.take() {
        // Apply the prefix transformation to workflow
        let prefixed = PathBuf::from(".github/workflows").join(workflow);
        args.workflow = Some(prefixed);
    }

    args
}
