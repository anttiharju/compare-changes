use compare_changes::match_path;

fn assert_glob_match(pattern: &str, paths: &[&str], expected: bool) {
    let matches = match_path(pattern, paths);
    assert_eq!(
        matches, expected,
        "Pattern '{}' vs '{:?}' -> {} (expected {})",
        pattern, paths, matches, expected
    );
}

#[test]
fn test_single_star_behavior() {
    // Matches zero or more characters
    assert_glob_match("Octo*", &["Octocat"], true);
    assert_glob_match("Octo*", &["Octo"], true);
    assert_glob_match("Octo*", &["Octo/cat"], false); // does not match /
    assert_glob_match("*.js", &["dir/app.js"], false); // does not match /
    assert_glob_match("*", &["dir/file.txt"], false); // single * cannot match /
}

#[test]
fn test_double_star_behavior() {
    // Matches zero or more of any character (including /)
    assert_glob_match("**", &["anything"], true);
    assert_glob_match("**", &["dir/file.txt"], true);
    assert_glob_match("**", &["deep/nested/path/file.js"], true);
    assert_glob_match("**", &[""], true);
}

#[test]
fn test_question_mark_behavior() {
    // Component
    assert_glob_match("*.jsx?", &["component.js"], true);
    assert_glob_match("*.jsx?", &["component.jsx"], true);
    assert_glob_match("*.jsx?", &["component.jsxx"], false); // more than one 'x'

    // File
    assert_glob_match("file?.txt", &["fil.txt"], true);
    assert_glob_match("file?.txt", &["file.txt"], true);
    assert_glob_match("file?.txt", &["filee.txt"], false); // two e
}

#[test]
fn test_plus_behavior() {
    // Matches one or more of the preceding character
    assert_glob_match("*.jsx+", &["component.jsx"], true);
    assert_glob_match("*.jsx+", &["component.jsxx"], true);
    assert_glob_match("*.jsx+", &["component.jsxxx"], true);
    assert_glob_match("*.jsx+", &["component.js"], false); // zero 'x' - should not match

    // Works with other characters too
    assert_glob_match("file+.txt", &["file.txt"], true);
    assert_glob_match("file+.txt", &["filee.txt"], true);
    assert_glob_match("file+.txt", &["fileee.txt"], true);
    assert_glob_match("file+.txt", &["fil.txt"], false); // zero 'e' - should not match

    // Edge case: plus at start doesn't make sense
    assert_glob_match("+file.txt", &["file.txt"], false); // bogus pattern
}

#[test]
fn test_bracket_behavior() {
    // Single character matching
    assert_glob_match("[CB]at", &["Cat"], true);
    assert_glob_match("[CB]at", &["Bat"], true);
    assert_glob_match("[CB]at", &["Dat"], false);
    assert_glob_match("[CB]at", &["at"], false);

    // Numeric ranges
    assert_glob_match("[1-2]00", &["100"], true);
    assert_glob_match("[1-2]00", &["200"], true);
    assert_glob_match("[1-2]00", &["300"], false);
    assert_glob_match("[1-2]00", &["000"], false);

    // Lowercase letter ranges
    assert_glob_match("file[a-c].txt", &["filea.txt"], true);
    assert_glob_match("file[a-c].txt", &["fileb.txt"], true);
    assert_glob_match("file[a-c].txt", &["filec.txt"], true);
    assert_glob_match("file[a-c].txt", &["filed.txt"], false);

    // Uppercase letter ranges
    assert_glob_match("File[A-C].txt", &["FileA.txt"], true);
    assert_glob_match("File[A-C].txt", &["FileB.txt"], true);
    assert_glob_match("File[A-C].txt", &["FileC.txt"], true);
    assert_glob_match("File[A-C].txt", &["FileD.txt"], false);

    // Mixed ranges - digits and lowercase letters
    assert_glob_match("[0-9a-z]", &["5"], true);
    assert_glob_match("[0-9a-z]", &["a"], true);
    assert_glob_match("[0-9a-z]", &["z"], true);
    assert_glob_match("[0-9a-z]", &["A"], false); // uppercase not in range
    assert_glob_match("[0-9a-z]", &["@"], false); // special char not in range

    // Mixed ranges - digits, lowercase AND uppercase letters
    assert_glob_match("[0-9a-zA-Z]", &["5"], true);
    assert_glob_match("[0-9a-zA-Z]", &["a"], true);
    assert_glob_match("[0-9a-zA-Z]", &["z"], true);
    assert_glob_match("[0-9a-zA-Z]", &["A"], true);
    assert_glob_match("[0-9a-zA-Z]", &["Z"], true);
    assert_glob_match("[0-9a-zA-Z]", &["@"], false); // special char not in range
    assert_glob_match("[0-9a-zA-Z]", &["-"], false); // hyphen not in range when not between chars

    // Multiple character sets in one pattern
    assert_glob_match("[AB][ab][12]", &["Aa1"], true);
    assert_glob_match("[AB][ab][12]", &["Ab2"], true);
    assert_glob_match("[AB][ab][12]", &["Ba1"], true);
    assert_glob_match("[AB][ab][12]", &["Bb2"], true);
    assert_glob_match("[AB][ab][12]", &["AA1"], false); // second char must be lowercase
    assert_glob_match("[AB][ab][12]", &["Aa3"], false); // third char must be 1 or 2
    assert_glob_match("[AB][ab][12]", &["Ca1"], false); // first char must be A or B

    // Edge cases
    assert_glob_match("test[a].txt", &["testa.txt"], true);
    assert_glob_match("test[].txt", &["test[].txt"], false); // github actions won't run with a pattern like this
}
