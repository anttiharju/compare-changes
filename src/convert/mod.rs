#[cfg(test)]
#[path = "test.rs"]
mod test;

use crate::path;
use regex::Regex;

pub fn path_to_regex(parsed_path: &path::Path) -> Result<Regex, regex::Error> {
    let segments = &parsed_path.segments;
    let mut idx = 0usize;

    let mut pattern = String::new();

    while let Some(seg) = segments.get(idx) {
        match seg {
            path::Segment::Literal(lit) => {
                pattern.push_str(&regex::escape(lit));
            }
            path::Segment::SingleStar => {
                pattern.push_str("[^/]*");
            }
            path::Segment::DoubleStar => {
                let mut regex_part = ".*".to_string();
                if let Some(path::Segment::Literal(lit)) = segments.get(idx + 1) {
                    if let Some(stripped) = lit.strip_prefix('/') {
                        // Handle "**/" as optional anything ending with /
                        regex_part = format!("(?:.*/)?{}", if stripped.is_empty() { "".to_string() } else { regex::escape(stripped) });
                        // Skip the consumed literal segment
                        idx += 1;
                    }
                }
                // Regular double star
                pattern.push_str(&regex_part);
            }
            path::Segment::QuestionMark(ch) => {
                pattern.push_str(&format!("{}?", ch));
            }
            path::Segment::Plus(ch) => {
                pattern.push_str(&format!("{}+", ch));
            }
            path::Segment::Bracket(b) => {
                if b.singles == vec!['-'] && b.ranges.is_empty() {
                    return Err(regex::Error::Syntax("literal hyphen in bracket class".to_string()));
                }
                let mut cls = "[".to_string();
                for c in &b.singles {
                    cls.push(*c);
                }
                for (start, end) in &b.ranges {
                    cls.push(*start);
                    cls.push('-');
                    cls.push(*end);
                }
                cls.push(']');
                pattern.push_str(&cls);
            }
        }
        idx += 1;
    }

    let full = format!("^{}$", pattern);
    Regex::new(&full)
}
