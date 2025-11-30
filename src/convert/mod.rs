#[cfg(test)]
#[path = "test.rs"]
mod test;

use crate::path;
use regex::Regex;

pub fn path_to_regex(parsed_path: &path::Path) -> Result<Regex, regex::Error> {
    fn escape_literal(chars: &[char]) -> String {
        let s: String = chars.iter().collect();
        regex::escape(&s)
    }

    fn escape_class_char(c: char) -> String {
        match c {
            '\\' => "\\\\".to_string(),
            ']' => "\\]".to_string(),
            '^' => "\\^".to_string(),
            '-' => "\\-".to_string(),
            other => other.to_string(),
        }
    }

    let segments = &parsed_path.segments;
    let mut idx = 0usize;

    let mut pattern = String::new();
    let mut skip_next_slash = false;

    while let Some(seg) = segments.get(idx) {
        match seg {
            path::Segment::Literal(lit_chars) => {
                pattern.push_str(&escape_literal(lit_chars));
                skip_next_slash = false;
            }
            path::Segment::Slash => {
                if skip_next_slash {
                    // already handled by previous DoubleStar
                    skip_next_slash = false;
                } else {
                    pattern.push('/');
                }
            }
            path::Segment::SingleStar => {
                pattern.push_str("[^/]*");
                skip_next_slash = false;
            }
            path::Segment::DoubleStar => {
                // if next segment is a Slash, allow the "**/" to match either "anything/" or nothing
                if matches!(segments.get(idx + 1), Some(path::Segment::Slash)) {
                    // optional sequence of any chars ending with a slash
                    pattern.push_str("(?:.*/)?");
                    // instruct the next Slash segment to be skipped because we already allowed it optionally
                    skip_next_slash = true;
                } else {
                    // match any characters including slashes (greedy)
                    pattern.push_str(".*");
                    skip_next_slash = false;
                }
            }
            path::Segment::QuestionMark(c) => {
                // make the specific character optional
                pattern.push_str(&format!("{}?", regex::escape(&c.to_string())));
                skip_next_slash = false;
            }
            path::Segment::Plus(c) => {
                pattern.push_str(&format!("{}+", regex::escape(&c.to_string())));
                skip_next_slash = false;
            }
            path::Segment::Bracket(b) => {
                if b.singles.is_empty() && b.ranges.is_empty() {
                    // empty bracket class should never match, but avoid look-arounds (unsupported).
                    // Use a class that matches no character: [^\s\S]
                    pattern.push_str("[^\\s\\S]");
                } else {
                    let mut cls = String::from("[");
                    for c in &b.singles {
                        cls.push_str(&escape_class_char(*c));
                    }
                    for (start, end) in &b.ranges {
                        cls.push_str(&escape_class_char(*start));
                        cls.push('-');
                        cls.push_str(&escape_class_char(*end));
                    }
                    cls.push(']');
                    pattern.push_str(&cls);
                }
                skip_next_slash = false;
            }
        }
        idx += 1;
    }

    let full = format!("^{}$", pattern);
    Regex::new(&full)
}
