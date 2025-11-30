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

    // If first segment is Negation, skip it when building the regex.
    if matches!(segments.first(), Some(path::Segment::Negation)) {
        idx = 1;
    }

    let mut pattern = String::new();

    while let Some(seg) = segments.get(idx) {
        match seg {
            path::Segment::Literal(lit_chars) => {
                pattern.push_str(&escape_literal(lit_chars));
            }
            path::Segment::Slash => {
                pattern.push('/');
            }
            path::Segment::SingleStar => {
                // zero or more chars except '/'
                pattern.push_str("[^/]*");
            }
            path::Segment::DoubleStar => {
                // match across path separators
                pattern.push_str(".*");
            }
            path::Segment::QuestionMark(c) => {
                pattern.push_str(&format!("(?:{})?", regex::escape(&c.to_string())));
            }
            path::Segment::Plus(c) => {
                pattern.push_str(&format!("(?:{}+)", regex::escape(&c.to_string())));
            }
            path::Segment::Bracket(b) => {
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
            path::Segment::Negation => {
                // shouldn't happen except at start; treat as never-matching element
                pattern.push_str("(?!)");
            }
        }
        idx += 1;
    }

    let full = format!("^{}$", pattern);
    Regex::new(&full)
}
