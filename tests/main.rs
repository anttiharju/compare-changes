#![cfg(feature = "cli")]
use assert_cmd::cargo::cargo_bin_cmd;
use serde_json;
use std::fs;
use std::path::Path;
use tempfile::{tempdir, TempDir};

fn prepare_workflow_with_patterns(patterns: &[&str]) -> TempDir {
    let temp = tempdir().unwrap();
    let workflows = temp.path().join(".github/workflows");
    fs::create_dir_all(&workflows).unwrap();

    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/wildcard-template.yml");
    let dst = workflows.join("wildcard-template.yml");
    fs::copy(&src, &dst).unwrap();

    // Read template and replace the "paths:" section robustly while preserving indentation.
    let template_yaml = fs::read_to_string(&src).unwrap();
    let mut out_lines: Vec<String> = Vec::new();
    let mut inserted = false;
    for line in template_yaml.lines() {
        if !inserted && line.trim_start().starts_with("paths:") {
            let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
            out_lines.push(format!("{}paths:", indent));
            for p in patterns {
                out_lines.push(format!("{}  - \"{}\"", indent, p));
            }
            inserted = true;
        } else {
            out_lines.push(line.to_string());
        }
    }
    let yaml = if inserted { out_lines.join("\n") } else { template_yaml };
    fs::write(&dst, yaml).unwrap();

    temp
}

fn run_bin_with_changes(temp: &TempDir, changes: &[&str], debug: bool) -> (String, String) {
    let changes_json = serde_json::to_string(&changes).unwrap();

    let mut binding = cargo_bin_cmd!("compare-changes");
    let mut cmd = binding
        .current_dir(temp.path())
        .arg("--wildcard")
        .arg("template.yml")
        .arg("--changes")
        .arg(&changes_json);

    if debug {
        cmd = cmd.arg("--debug");
    }

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr)
}

#[test]
fn test_changes_output() {
    let temp = prepare_workflow_with_patterns(&["1", "2", "3"]);
    let (stdout, stderr) = run_bin_with_changes(&temp, &["foo/bar", "baz", "3"], true);

    for expected in ["1", "2", "3", "foo/bar", "baz", "path '3' matched file '3'", "changed=true"] {
        assert!(stdout.contains(expected), "Expected output to contain '{}'", expected);
        assert!(!stderr.contains(expected), "Did not expect '{}' in stderr", expected);
    }
}

#[test]
fn test_valid_stray_closing_bracket() {
    let temp = prepare_workflow_with_patterns(&["abc]"]);
    let (stdout, stderr) = run_bin_with_changes(&temp, &["abc]"], true);

    for expected in ["path 'abc]' matched file 'abc]'", "changed=true"] {
        assert!(stdout.contains(expected), "Expected output to contain '{}'", expected);
        assert!(!stderr.contains(expected), "Did not expect '{}' in stderr", expected);
    }
}

#[test]
fn test_valid_standalone_plus() {
    let pat = "+foo";
    let temp = prepare_workflow_with_patterns(&[pat]);
    let (stdout, stderr) = run_bin_with_changes(&temp, &["foo"], true);

    let expected_path = format!("path '{}' matched file 'foo'", pat);
    assert!(stdout.contains(&expected_path), "Expected output to contain '{}'", expected_path);
    assert!(stdout.contains("changed=true"), "Expected output to contain 'changed=true'");
    assert!(!stderr.contains(&expected_path), "Did not expect '{}' in stderr", expected_path);
    assert!(!stderr.contains("changed=true"), "Did not expect 'changed=true' in stderr");
}

#[test]
fn test_valid_standalone_questionmark() {
    let pat = "?bar";
    let temp = prepare_workflow_with_patterns(&[pat]);
    let (stdout, stderr) = run_bin_with_changes(&temp, &["bar"], true);

    let expected_path = format!("path '{}' matched file 'bar'", pat);
    assert!(stdout.contains(&expected_path), "Expected output to contain '{}'", expected_path);
    assert!(stdout.contains("changed=true"), "Expected output to contain 'changed=true'");
    assert!(!stderr.contains(&expected_path), "Did not expect '{}' in stderr", expected_path);
    assert!(!stderr.contains("changed=true"), "Did not expect 'changed=true' in stderr");
}

#[test]
fn test_invalid_stray_opening_bracket() {
    let pat = "[abc";
    let temp = prepare_workflow_with_patterns(&[pat]);
    let (stdout, stderr) = run_bin_with_changes(&temp, &["foo/bar"], false);

    let msg = format!("Failed to compare path '{}': found end of input expected any, or ']'", pat);

    assert!(
        stderr.contains(&msg),
        "expected exact error message in stderr\n\nEXPECTED:\n{}\n\nSTDERR:\n{}",
        msg,
        stderr
    );
    assert!(
        !stdout.contains(&msg),
        "did not expect the error message in stdout\n\nSTDOUT:\n{}",
        stdout
    );
}

#[test]
fn test_invalid_bracket_range() {
    let pat = "[z-a]";
    let temp = prepare_workflow_with_patterns(&[pat]);
    let (stdout, stderr) = run_bin_with_changes(&temp, &["foo/bar"], false);

    let expected = "invalid bracket range z-a";
    let msg = format!("Failed to compare path '{}': {}", pat, expected);

    assert!(
        stderr.contains(&msg),
        "expected exact error message in stderr\n\nEXPECTED:\n{}\n\nSTDERR:\n{}",
        msg,
        stderr
    );
    assert!(
        !stdout.contains(&msg),
        "did not expect the error message in stdout\n\nSTDOUT:\n{}",
        stdout
    );
}

#[test]
fn test_empty_bracket() {
    let pat = "[]";
    let temp = prepare_workflow_with_patterns(&[pat]);
    let (stdout, stderr) = run_bin_with_changes(&temp, &["foo/bar"], false);

    let expected = "empty bracket";
    let msg = format!("Failed to compare path '{}': {}", pat, expected);

    assert!(
        stderr.contains(&msg),
        "expected exact error message in stderr\n\nEXPECTED:\n{}\n\nSTDERR:\n{}",
        msg,
        stderr
    );
    assert!(
        !stdout.contains(&msg),
        "did not expect the error message in stdout\n\nSTDOUT:\n{}",
        stdout
    );
}
