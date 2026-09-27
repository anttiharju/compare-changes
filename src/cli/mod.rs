use clap::Parser;
use std::path::PathBuf;

mod style;
use style::get_style;

#[derive(Parser)]
#[command(
    version,
    about = "Compare or filter changed files with wildcard paths, or find changed files from GitHub event context with --find.",
    styles = get_style()
)]
pub struct Args {
    /// Find changed files from the git diff base inferred from GitHub Actions event context
    #[arg(short, long, default_value_t = false, conflicts_with_all = ["workflow", "paths", "changes", "validate", "filter"])]
    pub find: bool,

    /// Return a JSON array of changed files that match one path pattern
    #[arg(long, default_value_t = false, requires = "paths", conflicts_with_all = ["find", "validate", "workflow"])]
    pub filter: bool,

    /// Validate that workflow and action path patterns match at least one file in the repository
    #[arg(long, default_value_t = false, conflicts_with_all = ["workflow", "paths", "changes", "find", "filter"])]
    pub validate: bool,

    /// Workflow file under .github/workflows/
    #[arg(short, long, value_name = "FILE", required_unless_present_any = ["find", "validate", "paths"], conflicts_with = "paths")]
    pub workflow: Option<PathBuf>,

    /// Newline-separated comparison patterns, or one pattern with --filter (alternative to --workflow)
    #[arg(short, long, value_name = "PATHS", required_unless_present_any = ["find", "validate", "workflow"])]
    pub paths: Option<String>,

    /// JSON array string, for example '["foo/bar", "baz"]'
    #[arg(short, long, value_name = "JSON", required_unless_present_any = ["find", "validate"])]
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
