mod cli;
mod parse;

fn main() {
    let args = cli::parse_args();

    let patterns = parse::get_patterns(&args.wildcard).expect("No on.push.paths found.");
    println!("patterns:");
    for pattern in &patterns {
        println!("- {}", pattern);
    }

    let paths = parse::get_paths(&args.changes_json).expect("Failed to parse the changes JSON array");
    println!("paths:");
    for path in &paths {
        println!("- {}", path);
    }
}
