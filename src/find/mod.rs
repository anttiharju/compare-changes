use serde_json::Value;
use std::env;
use std::fs;
use std::process::Command;

const INITIAL_PUSH_BEFORE: &str = "0000000000000000000000000000000000000000";

fn fetch_diff_base(event_name: &str, event_data: &Value) -> Result<Option<String>, String> {
    match event_name {
        "pull_request" | "merge_group" => Ok(Some("HEAD~1".to_string())),
        "push" => {
            let Some(before) = event_data.get("before").and_then(Value::as_str) else {
                return Ok(None);
            };

            if before == INITIAL_PUSH_BEFORE {
                println!("Detected initial commit - returning empty change set");
                return Ok(None);
            }

            let output = Command::new("git")
                .args(["fetch", "--depth=1", "--no-tags", "origin", before])
                .output()
                .map_err(|e| format!("Warning: Error while fetching git history: {}", e))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Warning: Error while fetching git history: {}", stderr.trim()));
            }

            println!("Fetched diff base: {}", before);
            Ok(Some(before.to_string()))
        }
        _ => Err("find-changes only works on pull_request, merge_group, push events.".to_string()),
    }
}

fn run_git_diff(comparison_point: Option<&str>) -> Result<Vec<String>, String> {
    let Some(comparison_point) = comparison_point else {
        return Ok(vec![]);
    };

    let output = Command::new("git")
        .args(["-c", "core.quotePath=false", "diff", "--name-only", "-z", comparison_point])
        .output()
        .map_err(|e| format!("Error running git diff against {}: {}", comparison_point, e))?;

    if !output.status.success() {
        return Err(format!("Error running git diff against {}", comparison_point));
    }

    let stdout =
        String::from_utf8(output.stdout).map_err(|e| format!("Git diff against {} returned non-UTF-8 path data: {}", comparison_point, e))?;
    Ok(stdout.split('\0').filter(|path| !path.is_empty()).map(ToString::to_string).collect())
}

fn get_event_data(debug: bool) -> Result<Value, String> {
    let event_path = env::var("GITHUB_EVENT_PATH").map_err(|_| "Could not find event payload file.".to_string())?;

    if debug {
        println!("Reading event from {}", event_path);
    }

    let raw = fs::read_to_string(&event_path).map_err(|e| format!("Error reading event data: {}", e))?;
    let event_data: Value = serde_json::from_str(&raw).map_err(|e| format!("Error reading event data: {}", e))?;

    if is_empty_payload(&event_data) {
        return Err("Event payload does not provide data.".to_string());
    }

    Ok(event_data)
}

fn get_event_name() -> Result<String, String> {
    env::var("GITHUB_EVENT_NAME").map_err(|_| "Could not find GITHUB_EVENT_NAME.".to_string())
}

fn is_empty_payload(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::Object(map) => map.is_empty(),
        Value::Array(items) => items.is_empty(),
        _ => false,
    }
}

fn write_output(files: &[String]) -> Result<(), String> {
    let Ok(github_output) = env::var("GITHUB_OUTPUT") else {
        return Ok(());
    };

    if github_output.is_empty() {
        return Ok(());
    }

    let output_line = format!(
        "array={}\n",
        serde_json::to_string(files).map_err(|e| format!("Failed to serialize changed files: {}", e))?
    );
    use std::io::Write;
    let mut handle = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(github_output)
        .map_err(|e| format!("Failed to write GITHUB_OUTPUT: {}", e))?;

    handle
        .write_all(output_line.as_bytes())
        .map_err(|e| format!("Failed to write GITHUB_OUTPUT: {}", e))
}

fn print_changes(files: &[String]) {
    let plural = if files.len() == 1 { "" } else { "s" };
    if files.is_empty() {
        println!("No changes found");
        return;
    }

    println!("Found {} change{}:", files.len(), plural);
    for file in files {
        println!("{}", file);
    }
}

pub fn run(debug: bool) -> Result<(), String> {
    let event_name = get_event_name()?;
    let event_data = get_event_data(debug)?;
    let diff_base = fetch_diff_base(&event_name, &event_data)?;
    let files = run_git_diff(diff_base.as_deref())?;

    write_output(&files)?;
    print_changes(&files);

    Ok(())
}
