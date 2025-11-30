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

    let segments = &parsed_path.segments;
    let mut idx = 0usize;

    let mut pattern = String::new();

    while let Some(seg) = segments.get(idx) {
        match seg {
            path::Segment::Literal(lit_chars) => {
                pattern.push_str(&escape_literal(lit_chars));
            }
            path::Segment::SingleStar => {
                pattern.push_str("[^/]*");
            }
            path::Segment::DoubleStar => {
                if let Some(path::Segment::Literal(lit)) = segments.get(idx + 1) {
                    if lit.first() == Some(&'/') {
                        // Handle "**/" as optional anything ending with /
                        pattern.push_str("(?:.*/)?");
                        // Push the literal without the leading /
                        let mut rest = lit.clone();
                        rest.remove(0);
                        if !rest.is_empty() {
                            pattern.push_str(&escape_literal(&rest));
                        }
                        idx += 1; // Skip the consumed literal segment
                    } else {
                        // Regular double star
                        pattern.push_str(".*");
                    }
                } else {
                    // Regular double star
                    pattern.push_str(".*");
                }
            }
            path::Segment::QuestionMark(c) => {
                pattern.push_str(&format!("{}?", regex::escape(&c.to_string())));
            }
            path::Segment::Plus(c) => {
                pattern.push_str(&format!("{}+", regex::escape(&c.to_string())));
            }
            path::Segment::Bracket(b) => {
                if b.singles == vec!['-'] && b.ranges.is_empty() {
                    return Err(regex::Error::Syntax("literal hyphen in bracket class".to_string()));
                }
                let mut cls = String::from("[");
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
