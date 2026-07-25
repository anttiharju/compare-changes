mod cli;
mod exitcode;
mod find;
mod parse;
mod validate;

use compare_changes::paths_match;

fn main() {
    let args = cli::parse_args();

    if args.find {
        if let Err(err) = find::run(args.debug) {
            eprintln!("{}", err);
            std::process::exit(exitcode::find_error());
        }
        return;
    }

    if args.validate {
        if let Err(err) = validate::run(args.debug) {
            eprintln!("{}", err);
            std::process::exit(exitcode::validate_error());
        }
        return;
    }

    let workflow = args.workflow.as_ref();
    let inline_paths = args.paths.as_deref();
    let changes = args.changes.as_ref().expect("changes is required unless --find");

    let paths_result = if let Some(inline) = inline_paths {
        parse::parse_inline_paths(inline).map(|paths| {
            if args.debug {
                println!("inline paths:");
                for path in &paths {
                    println!("- {}", path);
                }
            }
            paths
        })
    } else {
        let workflow = workflow.expect("workflow is required unless --find or --paths");
        parse::get_paths(workflow).map(|paths| {
            if args.debug {
                println!("{}.on.push.paths:", workflow.display());
                for path in &paths {
                    println!("- {}", path);
                }
            }
            paths
        })
    };

    let paths = match paths_result {
        Ok(paths) => paths,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(exitcode::path_error());
        }
    };

    let files = match parse::get_files(changes) {
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
            std::process::exit(exitcode::file_error());
        }
    };

    let file_refs: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
    let path_refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();

    let changed = match paths_match(&path_refs, &file_refs) {
        Ok(Some((pi, fi))) => {
            println!("path '{}' matched file '{}'", paths[pi], files[fi]);
            true
        }
        Ok(None) => false,
        Err(e) => {
            eprintln!("Failed to compare paths: {}", e);
            std::process::exit(exitcode::match_error());
        }
    };

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
