use compare_changes::match_path;

fn assert_glob_match(pattern: &str, paths: &[&str], expected: bool) {
    let matches = match_path(pattern, paths);
    assert_eq!(
        matches, expected,
        "Pattern '{}' vs '{:?}' -> {} (expected {})",
        pattern, paths, matches, expected
    );
}

// I am aware that the test cases are repetitive, but extracting patterns to a variable breaks single-line readability.

#[test]
fn test_basic_wildcards() {
    assert_glob_match("*", &["README.md"], true);
    assert_glob_match("*", &["server.rb"], true);
    assert_glob_match("*", &["docs/file.txt"], false); // * doesn't match slash (/) - only matches single path segment
}

#[test]
fn test_question_mark_wildcard() {
    // Test *.jsx? pattern - matches zero or one 'x'
    assert_glob_match("*.jsx?", &["page.js"], true);
    assert_glob_match("*.jsx?", &["page.jsx"], true);
    assert_glob_match("*.jsx?", &["component.js"], true);
    assert_glob_match("*.jsx?", &["component.jsx"], true);
    assert_glob_match("*.jsx?", &["page.jsxx"], false); // doesn't match - 'x' appears more than once
    assert_glob_match("*.jsx?", &["page.ts"], false); // doesn't match - doesn't end with js/jsx
}

#[test]
fn test_double_star_wildcard() {
    // Test ** pattern - matches any character including slash (/)
    assert_glob_match("**", &["all/the/files.md"], true);
    assert_glob_match("**", &["README.md"], true);
    assert_glob_match("**", &["docs/nested/deeply/file.txt"], true);
    assert_glob_match("**", &["single-file.js"], true);
    assert_glob_match("**", &[""], true);
}

#[test]
fn test_js_extension_pattern() {
    // Test *.js pattern - matches all .js files at the root of the repository
    assert_glob_match("*.js", &["app.js"], true);
    assert_glob_match("*.js", &["index.js"], true);
    assert_glob_match("*.js", &["main.js"], true);
    assert_glob_match("*.js", &["component.jsx"], false); // wrong extension
    assert_glob_match("*.js", &["src/app.js"], false); // not at root (contains slash)
    assert_glob_match("*.js", &["docs/script.js"], false); // not at root (contains slash)
}

#[test]
fn test_double_star_js_extension_pattern() {
    // Test **.js pattern - matches all .js files in the repository
    assert_glob_match("**.js", &["index.js"], true);
    assert_glob_match("**.js", &["js/index.js"], true);
    assert_glob_match("**.js", &["src/js/app.js"], true);
    assert_glob_match("**.js", &["deeply/nested/path/to/file.js"], true);
    assert_glob_match("**.js", &["component.jsx"], false); // wrong extension
    assert_glob_match("**.js", &["app.ts"], false); // wrong extension
    assert_glob_match("**.js", &["script.js.backup"], false); // doesn't end with .js
}

#[test]
fn test_docs_directory_pattern() {
    // Test docs/* pattern - matches all files within the root of the docs directory only
    assert_glob_match("docs/*", &["docs/README.md"], true);
    assert_glob_match("docs/*", &["docs/file.txt"], true);
    assert_glob_match("docs/*", &["docs/guide.md"], true);
    assert_glob_match("docs/*", &["docs/nested/file.txt"], false); // nested files don't match
    assert_glob_match("docs/*", &["README.md"], false); // not in docs directory
    assert_glob_match("docs/*", &["src/docs/file.txt"], false); // docs not at root
    assert_glob_match("docs/*", &["docs"], false); // directory itself, not files within
}

#[test]
fn test_docs_recursive_pattern() {
    // Test docs/** pattern - matches any files in docs directory and its subdirectories
    assert_glob_match("docs/**", &["docs/README.md"], true);
    assert_glob_match("docs/**", &["docs/mona/octocat.txt"], true);
    assert_glob_match("docs/**", &["docs/nested/deeply/file.txt"], true);
    assert_glob_match("docs/**", &["docs/guide.md"], true);
    assert_glob_match("docs/**", &["README.md"], false); // not in docs directory
    assert_glob_match("docs/**", &["src/docs/file.txt"], false); // docs not at root
    assert_glob_match("docs/**", &["other/docs/file.txt"], false); // docs not at root
}

#[test]
fn test_docs_markdown_pattern() {
    // Test docs/**/*.md pattern - matches .md files anywhere in docs directory
    assert_glob_match("docs/**/*.md", &["docs/README.md"], true);
    assert_glob_match("docs/**/*.md", &["docs/mona/hello-world.md"], true);
    assert_glob_match("docs/**/*.md", &["docs/a/markdown/file.md"], true);
    assert_glob_match("docs/**/*.md", &["docs/nested/deeply/guide.md"], true);
    assert_glob_match("docs/**/*.md", &["docs/file.txt"], false); // wrong extension
    assert_glob_match("docs/**/*.md", &["README.md"], false); // not in docs directory
    assert_glob_match("docs/**/*.md", &["src/docs/README.md"], false); // docs not at root
    assert_glob_match("docs/**/*.md", &["docs/"], false); // directory, not file
}

#[test]
fn test_nested_docs_pattern() {
    // Test **/docs/** pattern - matches any files in a docs directory anywhere in the repository
    assert_glob_match("**/docs/**", &["docs/hello.md"], true);
    assert_glob_match("**/docs/**", &["dir/docs/my-file.txt"], true);
    assert_glob_match("**/docs/**", &["space/docs/plan/space.doc"], true);
    assert_glob_match("**/docs/**", &["project/nested/docs/README.md"], true);
    assert_glob_match("**/docs/**", &["docs/nested/deeply/file.txt"], true);
    assert_glob_match("**/docs/**", &["some/path/docs/guide.md"], true);
    assert_glob_match("**/docs/**", &["README.md"], false); // not in any docs directory
    assert_glob_match("**/docs/**", &["documentation/file.txt"], false); // not in docs directory
    assert_glob_match("**/docs/**", &["docs-backup/file.txt"], false); // not exactly "docs"
}

#[test]
fn test_readme_anywhere_pattern() {
    // Test **/README.md pattern - matches README.md file anywhere in the repository
    assert_glob_match("**/README.md", &["README.md"], true);
    assert_glob_match("**/README.md", &["js/README.md"], true);
    assert_glob_match("**/README.md", &["docs/README.md"], true);
    assert_glob_match("**/README.md", &["src/components/README.md"], true);
    assert_glob_match("**/README.md", &["deeply/nested/path/README.md"], true);
    assert_glob_match("**/README.md", &["readme.md"], false); // case sensitive
    assert_glob_match("**/README.md", &["README.txt"], false); // wrong extension
    assert_glob_match("**/README.md", &["MY-README.md"], false); // different filename
    assert_glob_match("**/README.md", &["docs/readme/file.md"], false); // not the exact filename
}

#[test]
fn test_src_suffix_pattern() {
    // Test **/*src/** pattern - matches any file in a folder with a src suffix anywhere in the repository
    assert_glob_match("**/*src/**", &["a/src/app.js"], true);
    assert_glob_match("**/*src/**", &["my-src/code/js/app.js"], true);
    assert_glob_match("**/*src/**", &["project/main-src/utils/helper.js"], true);
    assert_glob_match("**/*src/**", &["app-src/components/Button.tsx"], true);
    assert_glob_match("**/*src/**", &["nested/path/web-src/styles/main.css"], true);
    assert_glob_match("**/*src/**", &["src/app.js"], true);
    assert_glob_match("**/*src/**", &["source/app.js"], false); // "source" doesn't end with "src"
    assert_glob_match("**/*src/**", &["src-backup/app.js"], false); // "src-backup" doesn't end with "src"
    assert_glob_match("**/*src/**", &["app.js"], false); // not in any *src directory
    assert_glob_match("**/*src/**", &["docs/src-old/file.txt"], false); // "src-old" doesn't end with "src"
}

#[test]
fn test_post_suffix_pattern() {
    // Test **/*-post.md pattern - matches files with suffix -post.md anywhere in the repository
    assert_glob_match("**/*-post.md", &["my-post.md"], true);
    assert_glob_match("**/*-post.md", &["path/their-post.md"], true);
    assert_glob_match("**/*-post.md", &["blog/first-post.md"], true);
    assert_glob_match("**/*-post.md", &["docs/welcome-post.md"], true);
    assert_glob_match("**/*-post.md", &["nested/path/to/final-post.md"], true);
    assert_glob_match("**/*-post.md", &["-post.md"], true);
    assert_glob_match("**/*-post.md", &["post.md"], false); // doesn't have the "-" prefix
    assert_glob_match("**/*-post.md", &["my-post.txt"], false); // wrong extension
    assert_glob_match("**/*-post.md", &["my-post-draft.md"], false); // has extra suffix after -post
    assert_glob_match("**/*-post.md", &["posts/readme.md"], false); // doesn't end with -post.md
}

#[test]
fn test_migrate_prefix_pattern() {
    // Test **/migrate-*.sql pattern - matches files with prefix migrate- and suffix .sql anywhere in the repository
    assert_glob_match("**/migrate-*.sql", &["migrate-10909.sql"], true);
    assert_glob_match("**/migrate-*.sql", &["db/migrate-v1.0.sql"], true);
    assert_glob_match("**/migrate-*.sql", &["db/sept/migrate-v1.sql"], true);
    assert_glob_match("**/migrate-*.sql", &["project/migrations/migrate-001.sql"], true);
    assert_glob_match("**/migrate-*.sql", &["migrate-initial.sql"], true);
    assert_glob_match("**/migrate-*.sql", &["migrate-.sql"], true); // empty middle part is valid
    assert_glob_match("**/migrate-*.sql", &["migration-v1.sql"], false); // wrong prefix
    assert_glob_match("**/migrate-*.sql", &["migrate-v1.txt"], false); // wrong extension
    assert_glob_match("**/migrate-*.sql", &["migrate.sql"], false); // missing dash after migrate
    assert_glob_match("**/migrate-*.sql", &["db/migrate-v1.sql.backup"], false);
    // extra suffix after .sql
}
