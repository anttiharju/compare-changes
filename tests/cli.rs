use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn test_changes_output() {
    let mut cmd = cargo_bin_cmd!("compare-changes");
    cmd.arg("--wildcard")
        .arg("foo")
        .arg("--changes")
        .arg(r#"["foo/bar", "baz"]"#);

    let output = cmd.assert().success().get_output().stdout.clone();
    let output_str = String::from_utf8_lossy(&output);

    assert!(output_str.contains("foo/bar"));
    assert!(output_str.contains("baz"));
}
