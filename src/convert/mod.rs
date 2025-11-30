#[cfg(test)]
#[path = "test.rs"]
mod test;

use crate::path;
use regex::Regex;

pub fn path_to_regex(parsed_path: &path::Path) -> Result<Regex, regex::Error> {
    let segments = &parsed_path.segments;
    let mut iter = segments.iter().peekable();
    let mut pattern = String::new();

    while let Some(seg) = iter.next() {
        match seg {
            path::Segment::Literal(lit) => {
                pattern.push_str(&regex::escape(lit));
            }
            path::Segment::SingleStar => {
                pattern.push_str("[^/]*");
            }
            path::Segment::DoubleStar => {
                // lookahead: if next is a Literal that starts with '/', treat "**/" specially
                if let Some(&path::Segment::Literal(lit)) = iter.peek()
                    && let Some(stripped) = lit.strip_prefix('/')
                {
                    iter.next(); // consume the literal segment
                    let suffix = if stripped.is_empty() { String::new() } else { regex::escape(stripped) };
                    pattern.push_str(&format!("(?:.*/)?{}", suffix));
                } else {
                    pattern.push_str(".*");
                }
            }
            path::Segment::QuestionMark(ch) => {
                pattern.push_str(&format!("{}?", ch));
            }
            path::Segment::Plus(ch) => {
                pattern.push_str(&format!("{}+", ch));
            }
            path::Segment::Bracket(b) => {
                if b.singles.contains(&'-') {
                    return Err(regex::Error::Syntax("literal hyphen in bracket class".to_string()));
                }
                pattern.push('[');
                for &c in &b.singles {
                    pattern.push(c);
                }
                for &(start, end) in &b.ranges {
                    pattern.push(start);
                    pattern.push('-');
                    pattern.push(end);
                }
                pattern.push(']');
            }
        }
    }

    let full = format!("^{}$", pattern);
    Regex::new(&full)
}
