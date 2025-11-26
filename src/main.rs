mod cli;
mod exitcode;
mod parse;

fn main() {
    let args = cli::parse_args();

    match parse::get_paths(&args.wildcard) {
        Ok(paths) => {
            println!("{}.on.push.paths:", args.wildcard.display());
            for path in &paths {
                println!("- {}", path);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            exitcode::paths_error();
        }
    }

    match parse::get_files(&args.changes_json) {
        Ok(files) => {
            println!("files:");
            for file in &files {
                println!("- {}", file);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            exitcode::files_error();
        }
    }
}
