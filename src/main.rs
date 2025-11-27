mod cli;
mod exitcode;
mod parse;

use compare_changes::matches_any_file;

fn main() {
    let args = cli::parse_args();

    let paths = match parse::get_paths(&args.wildcard) {
        Ok(paths) => {
            if args.debug {
                println!("{}.on.push.paths:", args.wildcard.display());
                for path in &paths {
                    println!("- {}", path);
                }
            }
            paths
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(exitcode::paths_error());
        }
    };

    let files = match parse::get_files(&args.changes_json) {
        Ok(files) => {
            if args.debug {
                println!("files:");
                for file in &files {
                    println!("- {}", file);
                }
            }
            files
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(exitcode::files_error());
        }
    };

    let file_refs: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
    let changed = paths.iter().any(|path| matches_any_file(path, &file_refs));
    println!("changed={}", changed);

    if let Ok(github_output) = std::env::var("GITHUB_OUTPUT") {
        if !github_output.is_empty() {
            use std::fs::OpenOptions;
            use std::io::Write;
            if let Ok(mut file) = OpenOptions::new().append(true).open(github_output) {
                let _ = writeln!(file, "changed={}", changed);
            }
        }
    }
}
