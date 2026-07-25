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
    #[arg(short, long, default_value_t = false, conflicts_with_all = ["source", "changes", "validate"])]
    pub find: bool,

    /// Validate that path patterns in workflows and compare-changes-action invocations match at least one file in the repository
    #[arg(long, default_value_t = false, conflicts_with_all = ["source", "changes", "find"])]
    pub validate: bool,

    #[command(flatten)]
    pub source: Source,

    /// JSON array string, for example '["foo/bar", "baz"]'
    #[arg(short, long, value_name = "JSON", required_unless_present_any = ["find", "validate"])]
    pub changes: Option<String>,

    /// Enable debug output
    #[arg(short, long)]
    pub debug: bool,
}

#[derive(clap::Args)]
#[group(id = "source", required = false, multiple = false)]
pub struct Source {
    /// Workflow file under .github/workflows/
    #[arg(short, long, value_name = "FILE", required_unless_present_any = ["find", "validate", "paths"])]
    pub workflow: Option<PathBuf>,

    /// Newline-separated inline path patterns (alternative to --workflow)
    #[arg(short, long, value_name = "PATHS", required_unless_present_any = ["find", "validate", "workflow"])]
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
