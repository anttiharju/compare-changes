pub fn match_path(path: &str, files: &[&str]) -> bool {
    if files.is_empty() {
        return false;
    }

    // Parse the single path, expanding optionals into multiple variants
    let parse_paths: Vec<(Path, bool)> = parse_path(path);

    // Check if any file matches the path
    for file in files {
        let file_segments = if file.is_empty() { vec![] } else { file.split('/').collect() };

        // Process pre-parsed path variants sequentially - each variant can override previous results
        let mut matched = false;

        for (parsed_path, is_negation) in &parse_paths {
            if match_segments(&parsed_path.segments, &file_segments, 0, 0) {
                if *is_negation {
                    matched = false; // Negation path matched - exclude the file
                } else {
                    matched = true; // Positive path matched - include the file
                }
            }
        }

        // If this file matched, return true immediately
        if matched {
            return true;
        }
    }

    // No file matched the path
    false
}

#[derive(Debug, Clone)]
struct Path {
    segments: Vec<Segment>,
}

#[derive(Debug, Clone)]
enum Segment {
    Literal(String),              // "docs", "file.txt"
    Pattern(String),              // "*.js", "*.jsx+", "[CB]at", etc.
    DoubleStar,                   // "**"
    DoubleStarWithSuffix(String), // "**.js"
}

fn parse_path(pattern: &str) -> Vec<(Path, bool)> {
    let (actual_pattern, is_negation) = match pattern.strip_prefix('!') {
        Some(rest) => (rest, true),
        None => (pattern, false),
    };

    // Expand optionals into multiple patterns
    expand_optionals(actual_pattern)
        .into_iter()
        .map(|expanded_pattern| {
            let parts: Vec<&str> = expanded_pattern.split('/').collect();
            let mut segments = Vec::new();

            for part in parts {
                if part == "**" {
                    segments.push(Segment::DoubleStar);
                } else if let Some(suffix) = part.strip_prefix("**") {
                    segments.push(Segment::DoubleStarWithSuffix(suffix.to_string()));
                } else if part.contains('*') || part.contains('+') || part.contains('[') {
                    segments.push(Segment::Pattern(part.to_string()));
                } else {
                    segments.push(Segment::Literal(part.to_string()));
                }
            }

            (Path { segments }, is_negation)
        })
        .collect()
}

fn expand_optionals(pattern: &str) -> Vec<String> {
    if let Some(question_pos) = pattern.find('?') {
        if question_pos == 0 {
            return vec![pattern.to_string()]; // Invalid pattern, return as-is
        }

        let optional_char = pattern.chars().nth(question_pos - 1).unwrap();
        let before_optional = &pattern[..question_pos - 1];
        let after_optional = &pattern[question_pos + 1..];

        // Create two variants: without and with the optional character
        let pattern_without = format!("{}{}", before_optional, after_optional);
        let pattern_with = format!("{}{}{}", before_optional, optional_char, after_optional);

        // Recursively expand any remaining optionals in both variants
        let mut results = Vec::new();
        results.extend(expand_optionals(&pattern_without));
        results.extend(expand_optionals(&pattern_with));
        results
    } else {
        // No more optionals, return the pattern as-is
        vec![pattern.to_string()]
    }
}

fn match_segments(segments: &[Segment], path_parts: &[&str], seg_idx: usize, path_idx: usize) -> bool {
    // Base case: both exhausted
    if seg_idx >= segments.len() && path_idx >= path_parts.len() {
        return true;
    }

    // Pattern exhausted but path remains
    if seg_idx >= segments.len() {
        return false;
    }

    match &segments[seg_idx] {
        Segment::Literal(literal) => {
            if path_idx >= path_parts.len() || path_parts[path_idx] != literal {
                return false;
            }
            match_segments(segments, path_parts, seg_idx + 1, path_idx + 1)
        }

        Segment::Pattern(pattern) => {
            if path_idx >= path_parts.len() {
                return false;
            }

            if glob_match(pattern, path_parts[path_idx]) {
                match_segments(segments, path_parts, seg_idx + 1, path_idx + 1)
            } else {
                false
            }
        }

        Segment::DoubleStar => {
            // Try consuming 0 or more path segments
            if match_segments(segments, path_parts, seg_idx + 1, path_idx) {
                return true;
            }

            for i in (path_idx + 1)..=path_parts.len() {
                if match_segments(segments, path_parts, seg_idx + 1, i) {
                    return true;
                }
            }
            false
        }

        Segment::DoubleStarWithSuffix(suffix) => {
            for i in path_idx..path_parts.len() {
                if path_parts[i].ends_with(suffix) && match_segments(segments, path_parts, seg_idx + 1, i + 1) {
                    return true;
                }
            }
            false
        }
    }
}

// Single function that handles all glob pattern matching within a path segment
fn glob_match(pattern: &str, text: &str) -> bool {
    glob_match_recursive(pattern, text, 0, 0)
}

fn glob_match_recursive(pattern: &str, text: &str, p_idx: usize, t_idx: usize) -> bool {
    let p_chars: Vec<char> = pattern.chars().collect();
    let t_chars: Vec<char> = text.chars().collect();

    // Base cases
    if p_idx >= p_chars.len() && t_idx >= t_chars.len() {
        return true; // Both exhausted
    }
    if p_idx >= p_chars.len() {
        return false; // Pattern exhausted but text remains
    }

    match p_chars[p_idx] {
        '*' => {
            // Try matching 0 or more characters
            for i in t_idx..=t_chars.len() {
                if glob_match_recursive(pattern, text, p_idx + 1, i) {
                    return true;
                }
            }
            false
        }

        '+' => {
            if p_idx == 0 {
                return false; // Invalid pattern starting with +
            }

            let char_to_repeat = p_chars[p_idx - 1];

            // The plus means "one or more of the preceding character"
            // But we need to backtrack because we already matched one instance
            // We need to match at least one more occurrence at the current text position
            let mut repeat_count = 0;
            let mut curr_t_idx = t_idx;

            // Count how many times the character repeats at current position
            while curr_t_idx < t_chars.len() && t_chars[curr_t_idx] == char_to_repeat {
                repeat_count += 1;
                curr_t_idx += 1;
            }

            if repeat_count == 0 {
                return false; // Need at least one occurrence
            }

            // Continue matching from where we left off
            glob_match_recursive(pattern, text, p_idx + 1, curr_t_idx)
        }

        '[' => {
            if t_idx >= t_chars.len() {
                return false;
            }

            // Find the closing bracket
            let mut bracket_end = p_idx + 1;
            while bracket_end < p_chars.len() && p_chars[bracket_end] != ']' {
                bracket_end += 1;
            }

            if bracket_end >= p_chars.len() {
                return false; // No closing bracket
            }

            // Extract bracket content
            let bracket_content: String = p_chars[(p_idx + 1)..bracket_end].iter().collect();

            if matches_bracket_content(&bracket_content, t_chars[t_idx]) {
                glob_match_recursive(pattern, text, bracket_end + 1, t_idx + 1)
            } else {
                false
            }
        }

        '?' => {
            if p_idx == 0 {
                return false; // Invalid pattern starting with ?
            }

            // Optional character - try both with and without
            // Without the optional character
            if glob_match_recursive(pattern, text, p_idx + 1, t_idx) {
                return true;
            }

            // With the optional character (if it matches the preceding character)
            if t_idx < t_chars.len() {
                let optional_char = p_chars[p_idx - 1];
                if t_chars[t_idx] == optional_char {
                    return glob_match_recursive(pattern, text, p_idx + 1, t_idx + 1);
                }
            }

            false
        }

        c => {
            // Literal character - but check if next character is '+'
            if p_idx + 1 < p_chars.len() && p_chars[p_idx + 1] == '+' {
                // This character is followed by +, so we need to match one or more
                let mut repeat_count = 0;
                let mut curr_t_idx = t_idx;

                while curr_t_idx < t_chars.len() && t_chars[curr_t_idx] == c {
                    repeat_count += 1;
                    curr_t_idx += 1;
                }

                if repeat_count == 0 {
                    return false; // Need at least one occurrence
                }

                // Skip both the character and the '+' in pattern
                glob_match_recursive(pattern, text, p_idx + 2, curr_t_idx)
            } else {
                // Regular literal character
                if t_idx >= t_chars.len() || t_chars[t_idx] != c {
                    return false;
                }
                glob_match_recursive(pattern, text, p_idx + 1, t_idx + 1)
            }
        }
    }
}

fn matches_bracket_content(bracket_content: &str, ch: char) -> bool {
    let chars: Vec<char> = bracket_content.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Check for range (e.g., "a-z", "0-9", "A-Z")
        if i + 2 < chars.len() && chars[i + 1] == '-' {
            let start_char = chars[i];
            let end_char = chars[i + 2];

            if ch >= start_char && ch <= end_char {
                return true;
            }

            i += 3; // Skip the range
        } else {
            // Single character match
            if chars[i] == ch {
                return true;
            }
            i += 1;
        }
    }

    false
}
