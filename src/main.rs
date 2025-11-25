mod cli;
mod parse;

fn main() {
    let args = cli::parse_args();

    let paths = parse::get_paths(&args.wildcard).expect("No on.push.paths found.");
    println!("paths:");
    for path in &paths {
        println!("- {}", path);
    }

    let changes = parse::get_changes(&args.changes_json).expect("Failed to parse the changes JSON array");
    println!("changes:");
    for change in &changes {
        println!("- {}", change);
    }
}
