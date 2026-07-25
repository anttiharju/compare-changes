use anstyle::{AnsiColor, Color, Style};
use compare_changes::path_matches;
use std::fs;
use std::io::IsTerminal;
use std::process::Command;

pub fn run(debug: bool) -> Result<(), String> {
    let files = git_ls_files()?;
    if debug {
        println!("git ls-files returned {} files", files.len());
    }
    let file_refs: Vec<&str> = files.iter().map(String::as_str).collect();

    let stdout_tty = std::io::stdout().is_terminal();
    let stderr_tty = std::io::stderr().is_terminal();
    let check = styled("✓", green(), stdout_tty);
    let mut failures = 0usize;
    let mut checked_files = 0usize;
    let mut checked_patterns = 0usize;
    let mut header_printed = false;

    // 1. Validate `on.push.paths` in every YAML workflow under .github/workflows/
    for file in &files {
        if !is_workflow_yaml(file) {
            continue;
        }
        let Ok(content) = fs::read_to_string(file) else {
            continue;
        };
        let paths = extract_on_push_paths(&content);
        if paths.is_empty() {
            continue;
        }
        print_header(&mut header_printed);
        println!("{} {} in {}", check, paths.len(), styled(file, gray(), stdout_tty),);
        checked_files += 1;
        checked_patterns += paths.len();
        for (path, line) in paths {
            failures += report_if_no_match(file, line, &path, &file_refs, stderr_tty)?;
        }
    }

    // 2. Validate `with.paths` in every YAML file that invokes compare-changes-action
    for file in &files {
        if !is_yaml(file) {
            continue;
        }
        let Ok(content) = fs::read_to_string(file) else {
            continue;
        };
        if !content.contains("compare-changes-action@") {
            continue;
        }
        let invocations = extract_action_paths(&content);
        if invocations.is_empty() {
            continue;
        }
        for (invocation_line, paths) in invocations {
            if paths.is_empty() {
                continue;
            }
            print_header(&mut header_printed);
            println!(
                "{} {} in {}",
                check,
                paths.len(),
                styled(&format!("{}:{}", file, invocation_line), gray(), stdout_tty),
            );
            checked_files += 1;
            checked_patterns += paths.len();
            for (path, line) in paths {
                failures += report_if_no_match(file, line, &path, &file_refs, stderr_tty)?;
            }
        }
    }

    if failures > 0 {
        Err(format!("{} path pattern{} did not match any file", failures, plural(failures)))
    } else if checked_patterns == 0 {
        println!("{} no path patterns found", check);
        Ok(())
    } else {
        let body = format!(
            "{} pattern{} across {} file{} match at least one file!",
            checked_patterns,
            plural(checked_patterns),
            checked_files,
            plural(checked_files),
        );
        // The pre-styled ✓ contains its own reset, so we can't wrap the whole
        // line in bold. Style the ✓ and the body separately.
        println!("{} {}", styled("✓", green(), stdout_tty), styled(&body, bold(), stdout_tty),);
        Ok(())
    }
}

fn print_header(printed: &mut bool) {
    if !*printed {
        println!("Validating patterns:");
        *printed = true;
    }
}

fn bold() -> Style {
    Style::new().bold()
}

fn green() -> Style {
    Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Green)))
}

fn red() -> Style {
    Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Red)))
}

fn gray() -> Style {
    Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightBlack)))
}

fn styled(text: &str, style: Style, colorize: bool) -> String {
    if colorize {
        format!("{}{}{}", style.render(), text, style.render_reset())
    } else {
        text.to_string()
    }
}

fn plural(n: usize) -> &'static str {
    if n == 1 { "" } else { "s" }
}

fn report_if_no_match(file: &str, line: usize, path: &str, files: &[&str], stderr_tty: bool) -> Result<usize, String> {
    match path_matches(path, files) {
        Ok(Some(_)) => Ok(0),
        Ok(None) => {
            let cross = styled("✗", red(), stderr_tty);
            eprintln!("{} {}:{}: no match for {}", cross, file, line, path);
            Ok(1)
        }
        Err(e) => Err(format!("{}:{}: failed to compile pattern '{}': {}", file, line, path, e)),
    }
}

fn git_ls_files() -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .args(["ls-files"])
        .output()
        .map_err(|e| format!("Failed to run 'git ls-files': {}", e))?;
    if !output.status.success() {
        return Err(format!("'git ls-files' failed: {}", String::from_utf8_lossy(&output.stderr).trim()));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(ToString::to_string)
        .collect())
}

fn is_yaml(path: &str) -> bool {
    path.ends_with(".yml") || path.ends_with(".yaml")
}

fn is_workflow_yaml(path: &str) -> bool {
    is_yaml(path) && path.starts_with(".github/workflows/")
}

fn split_indent(line: &str) -> (usize, &str) {
    let indent = line.chars().take_while(|c| *c == ' ').count();
    (indent, &line[indent..])
}

fn strip_quotes(s: &str) -> &str {
    let bytes = s.as_bytes();
    if bytes.len() >= 2 {
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        if (first == b'"' || first == b'\'') && first == last {
            return &s[1..s.len() - 1];
        }
    }
    s
}

/// Extract `on.push.paths` items as (value, 1-based line number) pairs.
///
/// Supports YAML block style only, which is the standard form for GitHub workflow files.
/// Returns an empty vector when the workflow does not declare `on.push.paths`.
fn extract_on_push_paths(content: &str) -> Vec<(String, usize)> {
    #[derive(Clone, Copy)]
    enum Phase {
        LookingForOn,
        LookingForPush { on_indent: usize },
        LookingForPaths { push_indent: usize },
        InPaths { paths_indent: usize, list_indent: Option<usize> },
    }

    let mut out = Vec::new();
    let mut phase = Phase::LookingForOn;

    for (i, raw) in content.lines().enumerate() {
        let (indent, trimmed) = split_indent(raw);
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        // Reprocess line if scope resets
        loop {
            match phase {
                Phase::LookingForOn => {
                    if indent == 0 && trimmed == "on:" {
                        phase = Phase::LookingForPush { on_indent: 0 };
                    }
                    break;
                }
                Phase::LookingForPush { on_indent } => {
                    if indent <= on_indent {
                        phase = Phase::LookingForOn;
                        continue;
                    }
                    if trimmed == "push:" {
                        phase = Phase::LookingForPaths { push_indent: indent };
                    }
                    break;
                }
                Phase::LookingForPaths { push_indent } => {
                    if indent <= push_indent {
                        phase = Phase::LookingForOn;
                        continue;
                    }
                    if trimmed == "paths:" {
                        phase = Phase::InPaths {
                            paths_indent: indent,
                            list_indent: None,
                        };
                    }
                    break;
                }
                Phase::InPaths { paths_indent, list_indent } => {
                    if indent <= paths_indent {
                        // Return to top-level scan so we don't miss anything else,
                        // though a workflow has only one `on:` block in practice.
                        phase = Phase::LookingForOn;
                        continue;
                    }
                    if trimmed.starts_with("- ") || trimmed == "-" {
                        let effective_list_indent = list_indent.unwrap_or(indent);
                        if indent == effective_list_indent {
                            let raw_val = if trimmed == "-" { "" } else { trimmed[2..].trim() };
                            out.push((strip_quotes(raw_val).to_string(), i + 1));
                            phase = Phase::InPaths {
                                paths_indent,
                                list_indent: Some(effective_list_indent),
                            };
                        }
                    }
                    break;
                }
            }
        }
    }

    out
}

/// Extract path entries from every `with.paths: |` block that belongs to a step
/// invoking `compare-changes-action@`. The block is only harvested when a sibling
/// `changes:` input is present, matching the safeguard described in the CLI docs.
///
/// Returns one entry per invocation: (1-based line of the `compare-changes-action@`
/// marker, list of `(path, 1-based line)` pairs from that invocation's `paths` block).
fn extract_action_paths(content: &str) -> Vec<(usize, Vec<(String, usize)>)> {
    let lines: Vec<&str> = content.lines().collect();
    let mut out: Vec<(usize, Vec<(String, usize)>)> = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        if !lines[i].contains("compare-changes-action@") {
            i += 1;
            continue;
        }

        let marker_line = i + 1;
        let marker_indent = split_indent(lines[i]).0;
        let mut paths_indent: Option<usize> = None;
        let mut current: Vec<(String, usize)> = Vec::new();
        let mut has_changes = false;
        let mut has_paths = false;

        let mut j = i + 1;
        while j < lines.len() {
            let (indent, trimmed) = split_indent(lines[j]);
            if trimmed.is_empty() || trimmed.starts_with('#') {
                j += 1;
                continue;
            }
            // New step at same or lesser indent closes the block.
            if indent <= marker_indent && trimmed.starts_with("- ") {
                break;
            }

            if let Some(pi) = paths_indent {
                if indent > pi {
                    out_push_if_content(&mut current, trimmed, j + 1);
                    j += 1;
                    continue;
                }
                paths_indent = None;
            }

            if trimmed.starts_with("paths:") {
                let rest = trimmed[6..].trim();
                if matches!(rest, "|" | "|-" | "|+") {
                    paths_indent = Some(indent);
                    has_paths = true;
                }
            } else if trimmed.starts_with("changes:") {
                has_changes = true;
            }

            j += 1;
        }

        if has_paths && has_changes {
            out.push((marker_line, current));
        }

        i = j;
    }

    out
}

fn out_push_if_content(sink: &mut Vec<(String, usize)>, trimmed: &str, line: usize) {
    let value = trimmed.trim();
    if value.is_empty() {
        return;
    }
    sink.push((value.to_string(), line));
}
