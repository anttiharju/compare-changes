use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_changes_output() {
    let temp = tempdir().unwrap();
    let workflows = temp.path().join(".github/workflows");
    fs::create_dir_all(&workflows).unwrap();

    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/wildcard-test.yml");
    let dst = workflows.join("wildcard-test.yml");
    fs::copy(src, dst).unwrap();

    let output = cargo_bin_cmd!("compare-changes")
        .current_dir(&temp)
        .arg("--wildcard")
        .arg("test.yml")
        .arg("--changes")
        .arg(r#"["foo/bar", "baz"]"#)
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);

    for expected in ["1", "2", "3", "foo/bar", "baz"] {
        assert!(
            stdout.contains(expected),
            "Expected output to contain '{}'",
            expected
        );
    }
}
