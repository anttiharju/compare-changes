mod cli;
mod exitcode;
mod parse;

use compare_changes::path_matches_at_least_one_file;

fn main() {
    let args = cli::parse_args();

    let paths = match parse::get_paths(&args.wildcard) {
        Ok(paths) => {
            println!("{}.on.push.paths:", args.wildcard.display());
            for path in &paths {
                println!("- {}", path);
            }
            paths
        }
        Err(err) => {
            eprintln!("{}", err);
            exitcode::paths_error();
        }
    };

    let files = match parse::get_files(&args.changes_json) {
        Ok(files) => {
            println!("files:");
            for file in &files {
                println!("- {}", file);
            }
            files
        }
        Err(err) => {
            eprintln!("{}", err);
            exitcode::files_error();
        }
    };

    let file_refs: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
    let changed = paths.iter().any(|path| path_matches_at_least_one_file(path, &file_refs));
    println!("changed={}", changed);
}
