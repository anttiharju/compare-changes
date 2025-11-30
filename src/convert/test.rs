use super::path_to_regex;
#[cfg(test)]
use crate::path;

fn assert_path_match(pattern: &str, paths: &str, expected: bool) {
    // parse pattern into Path and convert to a compiled Regex
    let parsed = match path::parse(pattern) {
        Ok(p) => p,
        Err(e) => panic!("failed to parse pattern '{}': {:?}", pattern, e),
    };
    let re = match path_to_regex(&parsed) {
        Ok(r) => r,
        Err(e) => panic!("failed to build regex for pattern '{}': {}", pattern, e),
    };

    let matches = re.is_match(paths);
    assert_eq!(
        matches, expected,
        "Pattern '{}' vs '{}' -> {} (expected {})",
        pattern, paths, matches, expected
    );
}

// I am aware that the test cases are repetitive, but extracting patterns to a variable breaks single-line readability.

#[test]
fn test_basic_wildcards() {
    assert_path_match("*", "README.md", true);
    assert_path_match("*", "server.rb", true);
    assert_path_match("*", "docs/file.txt", false); // * doesn't match slash (/) - only matches single path segment
}

#[test]
fn test_question_mark_wildcard() {
    // Test *.jsx? pattern - matches zero or one 'x'
    assert_path_match("*.jsx?", "page.js", true);
    assert_path_match("*.jsx?", "page.jsx", true);
    assert_path_match("*.jsx?", "component.js", true);
    assert_path_match("*.jsx?", "component.jsx", true);
    assert_path_match("*.jsx?", "page.jsxx", false); // doesn't match - 'x' appears more than once
    assert_path_match("*.jsx?", "page.ts", false); // doesn't match - doesn't end with js/jsx
}

#[test]
fn test_double_star_wildcard() {
    // Test ** pattern - matches any character including slash (/)
    assert_path_match("**", "all/the/files.md", true);
    assert_path_match("**", "README.md", true);
    assert_path_match("**", "docs/nested/deeply/file.txt", true);
    assert_path_match("**", "single-file.js", true);
    assert_path_match("**", "", true);

    // Edge case
    assert_path_match("**/a+", "a", true); // DoubleStar + Literal + QuestionMark, except the Literal would become empty as / gets stripped by DoubleStar
    assert_path_match("**/", "dir/", true); // Although git does not support committing (therefore pushing) empty directories
    assert_path_match("**/", "file.txt", true);
    assert_path_match("**/", "dir/file.txt", true);
    assert_path_match("**/*", "dir/file.txt", true);
    assert_path_match("**/*", "dir/another/file.txt", true);
}

#[test]
fn test_js_extension_pattern() {
    // Test *.js pattern - matches all .js files at the root of the repository
    assert_path_match("*.js", "app.js", true);
    assert_path_match("*.js", "index.js", true);
    assert_path_match("*.js", "main.js", true);
    assert_path_match("*.js", "component.jsx", false); // wrong extension
    assert_path_match("*.js", "src/app.js", false); // not at root (contains slash)
    assert_path_match("*.js", "docs/script.js", false); // not at root (contains slash)
}

#[test]
fn test_double_star_js_extension_pattern() {
    // Test **.js pattern - matches all .js files in the repository
    assert_path_match("**.js", "index.js", true);
    assert_path_match("**.js", "js/index.js", true);
    assert_path_match("**.js", "src/js/app.js", true);
    assert_path_match("**.js", "deeply/nested/path/to/file.js", true);
    assert_path_match("**.js", "component.jsx", false); // wrong extension
    assert_path_match("**.js", "app.ts", false); // wrong extension
    assert_path_match("**.js", "script.js.backup", false); // doesn't end with .js
}

#[test]
fn test_docs_directory_pattern() {
    // Test docs/* pattern - matches all files within the root of the docs directory only
    assert_path_match("docs/*", "docs/README.md", true);
    assert_path_match("docs/*", "docs/file.txt", true);
    assert_path_match("docs/*", "docs/guide.md", true);
    assert_path_match("docs/*", "docs/nested/file.txt", false); // nested files don't match
    assert_path_match("docs/*", "README.md", false); // not in docs directory
    assert_path_match("docs/*", "src/docs/file.txt", false); // docs not at root
    assert_path_match("docs/*", "docs", false); // directory itself, not files within
}

#[test]
fn test_docs_recursive_pattern() {
    // Test docs/** pattern - matches any files in docs directory and its subdirectories
    assert_path_match("docs/**", "docs/README.md", true);
    assert_path_match("docs/**", "docs/mona/octocat.txt", true);
    assert_path_match("docs/**", "docs/nested/deeply/file.txt", true);
    assert_path_match("docs/**", "docs/guide.md", true);
    assert_path_match("docs/**", "README.md", false); // not in docs directory
    assert_path_match("docs/**", "src/docs/file.txt", false); // docs not at root
    assert_path_match("docs/**", "other/docs/file.txt", false); // docs not at root
}

#[test]
fn test_docs_markdown_pattern() {
    // Test docs/**/*.md pattern - matches .md files anywhere in docs directory
    assert_path_match("docs/**/*.md", "docs/README.md", true);
    assert_path_match("docs/**/*.md", "docs/mona/hello-world.md", true);
    assert_path_match("docs/**/*.md", "docs/a/markdown/file.md", true);
    assert_path_match("docs/**/*.md", "docs/nested/deeply/guide.md", true);
    assert_path_match("docs/**/*.md", "docs/file.txt", false); // wrong extension
    assert_path_match("docs/**/*.md", "README.md", false); // not in docs directory
    assert_path_match("docs/**/*.md", "src/docs/README.md", false); // docs not at root
    assert_path_match("docs/**/*.md", "docs/", false); // directory, not file
}

#[test]
fn test_nested_docs_pattern() {
    // Test **/docs/** pattern - matches any files in a docs directory anywhere in the repository
    assert_path_match("**/docs/**", "docs/hello.md", true);
    assert_path_match("**/docs/**", "dir/docs/my-file.txt", true);
    assert_path_match("**/docs/**", "space/docs/plan/space.doc", true);
    assert_path_match("**/docs/**", "project/nested/docs/README.md", true);
    assert_path_match("**/docs/**", "docs/nested/deeply/file.txt", true);
    assert_path_match("**/docs/**", "some/path/docs/guide.md", true);
    assert_path_match("**/docs/**", "README.md", false); // not in any docs directory
    assert_path_match("**/docs/**", "documentation/file.txt", false); // not in docs directory
    assert_path_match("**/docs/**", "docs-backup/file.txt", false); // not exactly "docs"
}

#[test]
fn test_readme_anywhere_pattern() {
    // Test **/README.md pattern - matches README.md file anywhere in the repository
    assert_path_match("**/README.md", "README.md", true);
    assert_path_match("**/README.md", "js/README.md", true);
    assert_path_match("**/README.md", "docs/README.md", true);
    assert_path_match("**/README.md", "src/components/README.md", true);
    assert_path_match("**/README.md", "deeply/nested/path/README.md", true);
    assert_path_match("**/README.md", "readme.md", false); // case sensitive
    assert_path_match("**/README.md", "README.txt", false); // wrong extension
    assert_path_match("**/README.md", "MY-README.md", false); // different filename
    assert_path_match("**/README.md", "docs/readme/file.md", false); // not the exact filename
}

#[test]
fn test_src_suffix_pattern() {
    // Test **/*src/** pattern - matches any file in a folder with a src suffix anywhere in the repository
    assert_path_match("**/*src/**", "a/src/app.js", true);
    assert_path_match("**/*src/**", "my-src/code/js/app.js", true);
    assert_path_match("**/*src/**", "project/main-src/utils/helper.js", true);
    assert_path_match("**/*src/**", "app-src/components/Button.tsx", true);
    assert_path_match("**/*src/**", "nested/path/web-src/styles/main.css", true);
    assert_path_match("**/*src/**", "src/app.js", true);
    assert_path_match("**/*src/**", "source/app.js", false); // "source" doesn't end with "src"
    assert_path_match("**/*src/**", "src-backup/app.js", false); // "src-backup" doesn't end with "src"
    assert_path_match("**/*src/**", "app.js", false); // not in any *src directory
    assert_path_match("**/*src/**", "docs/src-old/file.txt", false); // "src-old" doesn't end with "src"
}

#[test]
fn test_post_suffix_pattern() {
    // Test **/*-post.md pattern - matches files with suffix -post.md anywhere in the repository
    assert_path_match("**/*-post.md", "my-post.md", true);
    assert_path_match("**/*-post.md", "path/their-post.md", true);
    assert_path_match("**/*-post.md", "blog/first-post.md", true);
    assert_path_match("**/*-post.md", "docs/welcome-post.md", true);
    assert_path_match("**/*-post.md", "nested/path/to/final-post.md", true);
    assert_path_match("**/*-post.md", "-post.md", true);
    assert_path_match("**/*-post.md", "post.md", false); // doesn't have the "-" prefix
    assert_path_match("**/*-post.md", "my-post.txt", false); // wrong extension
    assert_path_match("**/*-post.md", "my-post-draft.md", false); // has extra suffix after -post
    assert_path_match("**/*-post.md", "posts/readme.md", false); // doesn't end with -post.md
}

#[test]
fn test_migrate_prefix_pattern() {
    // Test **/migrate-*.sql pattern - matches files with prefix migrate- and suffix .sql anywhere in the repository
    assert_path_match("**/migrate-*.sql", "migrate-10909.sql", true);
    assert_path_match("**/migrate-*.sql", "db/migrate-v1.0.sql", true);
    assert_path_match("**/migrate-*.sql", "db/sept/migrate-v1.sql", true);
    assert_path_match("**/migrate-*.sql", "project/migrations/migrate-001.sql", true);
    assert_path_match("**/migrate-*.sql", "migrate-initial.sql", true);
    assert_path_match("**/migrate-*.sql", "migrate-.sql", true); // empty middle part is valid
    assert_path_match("**/migrate-*.sql", "migration-v1.sql", false); // wrong prefix
    assert_path_match("**/migrate-*.sql", "migrate-v1.txt", false); // wrong extension
    assert_path_match("**/migrate-*.sql", "migrate.sql", false); // missing dash after migrate
    assert_path_match("**/migrate-*.sql", "db/migrate-v1.sql.backup", false);
    // extra suffix after .sql
}

// Special chars

#[test]
fn test_single_star_behavior() {
    // Matches zero or more characters
    assert_path_match("Octo*", "Octocat", true);
    assert_path_match("Octo*", "Octo", true);
    assert_path_match("Octo*", "Octo/cat", false); // does not match /
    assert_path_match("*.js", "dir/app.js", false); // does not match /
    assert_path_match("*", "dir/file.txt", false); // single * cannot match /
}

#[test]
fn test_double_star_behavior() {
    // Matches zero or more of any character (including /)
    assert_path_match("**", "anything", true);
    assert_path_match("**", "dir/file.txt", true);
    assert_path_match("**", "deep/nested/path/file.js", true);
    assert_path_match("**", "", true);
}

#[test]
fn test_question_mark_behavior() {
    // Component
    assert_path_match("*.jsx?", "component.js", true);
    assert_path_match("*.jsx?", "component.jsx", true);
    assert_path_match("*.jsx?", "component.jsxx", false); // more than one 'x'

    // File
    assert_path_match("file?.txt", "fil.txt", true);
    assert_path_match("file?.txt", "file.txt", true);
    assert_path_match("file?.txt", "filee.txt", false); // two e
}

#[test]
fn test_plus_behavior() {
    // Matches one or more of the preceding character
    assert_path_match("*.jsx+", "component.jsx", true);
    assert_path_match("*.jsx+", "component.jsxx", true);
    assert_path_match("*.jsx+", "component.jsxxx", true);
    assert_path_match("*.jsx+", "component.js", false); // zero 'x' - should not match

    // Works with other characters too
    assert_path_match("file+.txt", "file.txt", true);
    assert_path_match("file+.txt", "filee.txt", true);
    assert_path_match("file+.txt", "fileee.txt", true);
    assert_path_match("file+.txt", "fil.txt", false); // zero 'e' - should not match

    // Edge case: plus at start doesn't make sense
    assert_path_match("+file.txt", "file.txt", false); // bogus pattern
}

#[test]
fn test_bracket_behavior() {
    // Single character matching
    assert_path_match("[CB]at", "Cat", true);
    assert_path_match("[CB]at", "Bat", true);
    assert_path_match("[CB]at", "Dat", false);
    assert_path_match("[CB]at", "at", false);

    // Numeric ranges
    assert_path_match("[1-2]00", "100", true);
    assert_path_match("[1-2]00", "200", true);
    assert_path_match("[1-2]00", "300", false);
    assert_path_match("[1-2]00", "000", false);

    // Lowercase letter ranges
    assert_path_match("file[a-c].txt", "filea.txt", true);
    assert_path_match("file[a-c].txt", "fileb.txt", true);
    assert_path_match("file[a-c].txt", "filec.txt", true);
    assert_path_match("file[a-c].txt", "filed.txt", false);

    // Uppercase letter ranges
    assert_path_match("File[A-C].txt", "FileA.txt", true);
    assert_path_match("File[A-C].txt", "FileB.txt", true);
    assert_path_match("File[A-C].txt", "FileC.txt", true);
    assert_path_match("File[A-C].txt", "FileD.txt", false);

    // Mixed ranges - digits and lowercase letters
    assert_path_match("[0-9a-z]", "5", true);
    assert_path_match("[0-9a-z]", "a", true);
    assert_path_match("[0-9a-z]", "z", true);
    assert_path_match("[0-9a-z]", "A", false); // uppercase not in range
    assert_path_match("[0-9a-z]", "@", false); // special char not in range

    // Mixed ranges - digits, lowercase AND uppercase letters
    assert_path_match("[0-9a-zA-Z]", "5", true);
    assert_path_match("[0-9a-zA-Z]", "a", true);
    assert_path_match("[0-9a-zA-Z]", "z", true);
    assert_path_match("[0-9a-zA-Z]", "A", true);
    assert_path_match("[0-9a-zA-Z]", "Z", true);
    assert_path_match("[0-9a-zA-Z]", "@", false); // special char not in range
    assert_path_match("[0-9a-zA-Z]", "-", false); // hyphen not in range when not between chars

    // Multiple character sets in one pattern
    assert_path_match("[AB][ab][12]", "Aa1", true);
    assert_path_match("[AB][ab][12]", "Ab2", true);
    assert_path_match("[AB][ab][12]", "Ba1", true);
    assert_path_match("[AB][ab][12]", "Bb2", true);
    assert_path_match("[AB][ab][12]", "AA1", false); // second char must be lowercase
    assert_path_match("[AB][ab][12]", "Aa3", false); // third char must be 1 or 2
    assert_path_match("[AB][ab][12]", "Ca1", false); // first char must be A or B

    // Edge cases
    assert_path_match("test[a].txt", "testa.txt", true);
}

fn assert_glob_compile_fail(pattern: &str) {
    // treat parse errors as expected failures; otherwise ensure regex compilation fails
    match path::parse(pattern) {
        Err(_) => return, // parse failed -> considered a compilation failure for these tests
        Ok(parsed) => {
            if path_to_regex(&parsed).is_ok() {
                panic!("expected pattern '{}' to fail regex compilation, but it compiled", pattern)
            }
        }
    }
}

#[test]
fn test_regex_compile_failures() {
    assert_glob_compile_fail("[]");
    assert_glob_compile_fail("[^]");
    assert_glob_compile_fail(r#"[\]"#);
}

#[test]
fn test_regex_github_compat() {
    assert_glob_compile_fail("[-]");
    assert_glob_compile_fail("[A-]");
    assert_glob_compile_fail("[-A]");
}
