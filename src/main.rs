mod cli;
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
            std::process::exit(1);
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
            std::process::exit(1);
        }
    }
}
