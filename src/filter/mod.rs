use compare_changes::path_matches;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

pub fn run(pattern: &str, files: &[&str]) -> Result<(), String> {
    if pattern.is_empty() {
        return Err("No pattern provided in --paths input".to_string());
    }

    let mut filtered = Vec::new();
    for &file in files {
        if path_matches(pattern, &[file])
            .map_err(|error| format!("Failed to filter paths: {}", error))?
            .is_some()
        {
            filtered.push(file);
        }
    }
    let array = serde_json::to_string(&filtered).map_err(|error| format!("Failed to serialize filtered files: {}", error))?;

    if let Ok(github_output) = env::var("GITHUB_OUTPUT")
        && !github_output.is_empty()
    {
        let mut output = OpenOptions::new()
            .create(true)
            .append(true)
            .open(github_output)
            .map_err(|error| format!("Failed to write GITHUB_OUTPUT: {}", error))?;
        writeln!(output, "array={}", array).map_err(|error| format!("Failed to write GITHUB_OUTPUT: {}", error))?;
    }

    println!("array={}", array);
    Ok(())
}
