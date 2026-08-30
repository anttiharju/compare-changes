#![cfg(feature = "cli")]
use assert_cmd::cargo::cargo_bin_cmd;
use serde_json;
use std::fs;
use std::path::Path;
#[cfg(unix)]
use std::process::Command;
use tempfile::{TempDir, tempdir};

fn prepare_workflow_with_patterns(patterns: &[&str]) -> TempDir {
    let temp = tempdir().unwrap();
    let workflows = temp.path().join(".github/workflows");
    fs::create_dir_all(&workflows).unwrap();

    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/wildcard-template.yml");
    let dst = workflows.join("template.yml");
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
    let changes = serde_json::to_string(&changes).unwrap();

    let mut binding = cargo_bin_cmd!("compare-changes");
    let mut cmd = binding
        .current_dir(temp.path())
        .arg("--workflow")
        .arg("template.yml")
        .arg("--changes")
        .arg(&changes);

    if debug {
        cmd = cmd.arg("--debug");
    }

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr)
}

#[cfg(unix)]
fn run_git(temp: &TempDir, args: &[&str]) {
    let output = Command::new("git").current_dir(temp.path()).args(args).output().unwrap();

    assert!(
        output.status.success(),
        "git {:?} failed:\n{}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(unix)]
#[test]
fn test_find_preserves_git_pathnames() {
    let temp = tempdir().unwrap();
    run_git(&temp, &["init", "--quiet"]);
    run_git(&temp, &["config", "user.email", "test@example.com"]);
    run_git(&temp, &["config", "user.name", "Test User"]);
    run_git(&temp, &["config", "commit.gpgSign", "false"]);

    fs::write(temp.path().join("baseline.txt"), "baseline").unwrap();
    run_git(&temp, &["add", "--all"]);
    run_git(&temp, &["commit", "--quiet", "-m", "baseline"]);

    fs::create_dir(temp.path().join("src")).unwrap();
    let expected = vec![
        " leading.sh".to_string(),
        "src/line\nbreak.sh".to_string(),
        "src/quote\"name.sh".to_string(),
        "src/\u{e9}vil.sh".to_string(),
        "trailing.sh ".to_string(),
    ];
    for path in &expected {
        fs::write(temp.path().join(path), "changed").unwrap();
    }
    run_git(&temp, &["add", "--all"]);
    run_git(&temp, &["commit", "--quiet", "-m", "changed paths"]);

    let event_path = temp.path().join("event.json");
    let output_path = temp.path().join("github-output.txt");
    fs::write(&event_path, r#"{"pull_request":{}}"#).unwrap();

    let output = cargo_bin_cmd!("compare-changes")
        .current_dir(temp.path())
        .arg("--find")
        .env("GITHUB_EVENT_NAME", "pull_request")
        .env("GITHUB_EVENT_PATH", event_path)
        .env("GITHUB_OUTPUT", &output_path)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "compare-changes failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let github_output = fs::read_to_string(output_path).unwrap();
    let files: Vec<String> = serde_json::from_str(github_output.trim().strip_prefix("array=").unwrap()).unwrap();
    assert_eq!(files, expected);
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

    let msg = "Failed to compare paths: found end of input expected any, or ']'";

    assert!(
        stderr.contains(msg),
        "expected exact error message in stderr\n\nEXPECTED:\n{}\n\nSTDERR:\n{}",
        msg,
        stderr
    );
    assert!(!stdout.contains(msg), "did not expect the error message in stdout\n\nSTDOUT:\n{}", stdout);
}

#[test]
fn test_invalid_bracket_range() {
    let pat = "[z-a]";
    let temp = prepare_workflow_with_patterns(&[pat]);
    let (stdout, stderr) = run_bin_with_changes(&temp, &["foo/bar"], false);

    let expected = "invalid bracket range z-a";
    let msg = format!("Failed to compare paths: {}", expected);

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
    let msg = format!("Failed to compare paths: {}", expected);

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
fn test_negation_excludes_file() {
    let temp = prepare_workflow_with_patterns(&["*.md", "!README.md"]);
    let (stdout, stderr) = run_bin_with_changes(&temp, &["README.md"], false);

    assert!(stdout.contains("changed=false"), "expected 'changed=false' in stdout, got:\n{}", stdout);
    assert!(
        !stdout.contains("matched file"),
        "did not expect a match line in stdout, got:\n{}",
        stdout
    );
    assert!(stderr.is_empty(), "did not expect any stderr, got:\n{}", stderr);
}

#[test]
fn test_negation_allows_other_matches() {
    let temp = prepare_workflow_with_patterns(&["*.md", "!README.md"]);
    let (stdout, stderr) = run_bin_with_changes(&temp, &["README.md", "hello.md"], false);

    for expected in ["path '*.md' matched file 'hello.md'", "changed=true"] {
        assert!(stdout.contains(expected), "expected '{}' in stdout, got:\n{}", expected, stdout);
    }
    assert!(stderr.is_empty(), "did not expect any stderr, got:\n{}", stderr);
}

#[test]
fn test_negation_re_included_by_later_pattern() {
    let temp = prepare_workflow_with_patterns(&["*.md", "!README.md", "README*"]);
    let (stdout, stderr) = run_bin_with_changes(&temp, &["README.md"], false);

    for expected in ["path 'README*' matched file 'README.md'", "changed=true"] {
        assert!(stdout.contains(expected), "expected '{}' in stdout, got:\n{}", expected, stdout);
    }
    assert!(stderr.is_empty(), "did not expect any stderr, got:\n{}", stderr);
}

#[test]
fn test_exitcode_usage() {
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    // Extract exitcode function names from the macro definition
    let exitcode_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/exitcode/mod.rs");
    let exitcode_content = fs::read_to_string(&exitcode_file).unwrap();

    let mut exitcodes = Vec::new();
    let mut in_macro = false;
    for line in exitcode_content.lines() {
        if line.contains("define_exitcodes!") && line.contains("{") {
            in_macro = true;
            continue;
        }
        if in_macro && line.trim() == "}" {
            break;
        }
        if in_macro {
            if let Some(pos) = line.find("=>") {
                let fn_name = line[..pos].trim().trim_end_matches(',');
                if !fn_name.is_empty() {
                    exitcodes.push(fn_name.to_string());
                }
            }
        }
    }

    // Count usage of each exitcode function in src/
    let mut counts: HashMap<String, usize> = HashMap::new();
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    fn visit_dir(dir: &Path, counts: &mut HashMap<String, usize>, exitcodes: &[String]) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    visit_dir(&path, counts, exitcodes);
                } else if path.extension().map_or(false, |e| e == "rs") {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        for exitcode in exitcodes {
                            let pattern = format!("exitcode::{}(", exitcode);
                            *counts.entry(exitcode.clone()).or_insert(0) += contents.matches(&pattern).count();
                        }
                    }
                }
            }
        }
    }

    visit_dir(&src_dir, &mut counts, &exitcodes);

    // Assert each exitcode is called exactly once
    for exitcode in &exitcodes {
        let count = counts.get(exitcode).copied().unwrap_or(0);
        assert_eq!(count, 1, "expected exactly 1 call to exitcode::{}, found {}", exitcode, count);
    }
}
