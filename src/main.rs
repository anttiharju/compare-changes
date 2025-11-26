mod cli;
mod exitcode;
mod parse;

fn main() {
    let args = cli::parse_args();

    match parse::get_patterns(&args.wildcard) {
        Ok(patterns) => {
            println!("{}.on.push.paths:", args.wildcard.display());
            for pattern in &patterns {
                println!("- {}", pattern);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            exitcode::wildcard_error();
        }
    }

    match parse::get_paths(&args.changes_json) {
        Ok(paths) => {
            println!("paths:");
            for path in &paths {
                println!("- {}", path);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            exitcode::changes_error();
        }
    }
}
