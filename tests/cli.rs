use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_changes_output() {
    let temp = tempdir().unwrap();
    let workflows = temp.path().join(".github/workflows");
    fs::create_dir_all(&workflows).unwrap();

    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/wildcard-template.yml");
    let dst = workflows.join("wildcard-template.yml");
    fs::copy(&src, &dst).unwrap();

    // Inject test paths into the YAML file
    let test_paths = ["1", "2", "3"];
    let paths_yaml = test_paths
        .iter()
        .map(|p| format!("      - \"{}\"", p))
        .collect::<Vec<_>>()
        .join("\n");
    let mut yaml = fs::read_to_string(&dst).unwrap();
    yaml = yaml.replace("paths:", &format!("paths:\n{}", paths_yaml));
    fs::write(&dst, yaml).unwrap();

    let output = cargo_bin_cmd!("compare-changes")
        .current_dir(&temp)
        .arg("--wildcard")
        .arg("template.yml")
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
