use serde_json;
use serde_yaml::Value;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn get_paths(wildcard: &Path) -> Result<Vec<String>, String> {
    let wildcard_path =
        PathBuf::from(".github/workflows").join(format!("wildcard-{}", wildcard.display()));
    let wildcard_contents = fs::read_to_string(&wildcard_path)
        .map_err(|_| "Failed to read wildcard file".to_string())?;
    let yaml: Value =
        serde_yaml::from_str(&wildcard_contents).map_err(|_| "Failed to parse YAML".to_string())?;

    let paths = yaml
        .get("on")
        .and_then(|on| on.get("push"))
        .and_then(|push| push.get("paths"))
        .and_then(|paths| paths.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    if paths.is_empty() {
        Err("No on.push.paths found.".to_string())
    } else {
        Ok(paths)
    }
}

pub fn get_changes(changes_json: &str) -> Result<Vec<String>, String> {
    serde_json::from_str::<Vec<String>>(changes_json)
        .map_err(|e| format!("Failed to parse changes JSON array: {}", e))
}
