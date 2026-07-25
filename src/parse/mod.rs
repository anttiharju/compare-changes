use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn get_paths(wildcard: &Path) -> Result<Vec<String>, String> {
    let wildcard_contents = fs::read_to_string(wildcard).map_err(|e| format!("Failed to read wildcard file '{}': {}", wildcard.display(), e))?;
    let yaml: Value = serde_saphyr::from_str(&wildcard_contents).map_err(|e| format!("Failed to parse YAML in '{}': {}", wildcard.display(), e))?;

    let paths = yaml
        .get("on")
        .and_then(|on| on.get("push"))
        .and_then(|push| push.get("paths"))
        .and_then(|paths| paths.as_array())
        .map(|seq| seq.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<String>>())
        .unwrap_or_default();

    if paths.is_empty() {
        Err(format!("No on.push.paths found in '{}'", wildcard.display()))
    } else {
        Ok(paths)
    }
}

pub fn get_files(changes: &str) -> Result<Vec<String>, String> {
    serde_json::from_str::<Vec<String>>(changes).map_err(|e| format!("Failed to parse changes JSON array: {}", e))
}

pub fn parse_inline_paths(input: &str) -> Result<Vec<String>, String> {
    let paths: Vec<String> = input
        .lines()
        .map(|l| {
            let trimmed = l.trim();
            // Strip a single pair of surrounding matching quotes if present.
            if trimmed.len() >= 2 {
                let bytes = trimmed.as_bytes();
                let first = bytes[0];
                let last = bytes[bytes.len() - 1];
                if (first == b'"' || first == b'\'') && first == last {
                    return trimmed[1..trimmed.len() - 1].to_string();
                }
            }
            trimmed.to_string()
        })
        .filter(|l| !l.is_empty())
        .collect();

    if paths.is_empty() {
        Err("No paths provided in --paths input".to_string())
    } else {
        Ok(paths)
    }
}
