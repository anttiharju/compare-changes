mod cli;
mod wildcard;

use serde_json;

fn main() {
    let args = cli::parse_args();

    let paths = wildcard::get_paths(&args.wildcard).expect("No on.push.paths found.");
    println!("paths:");
    for path in &paths {
        println!("- {}", path);
    }

    let changes: Vec<String> =
        serde_json::from_str(&args.changes_json).expect("Failed to parse the changes JSON array");
    println!("changes:");
    for change in &changes {
        println!("- {}", change);
    }
}
