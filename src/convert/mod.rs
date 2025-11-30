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
                if let Some(path::Segment::Literal(lit)) = iter.peek() {
                    if let Some(stripped) = lit.strip_prefix('/') {
                        if stripped.is_empty() {
                            // next segment was "/" — consume it and treat as ".*"
                            iter.next();
                            pattern.push_str(".*");
                        } else {
                            // followed by "/something" — consume literal and match optionally any dirs before it
                            iter.next();
                            pattern.push_str(&format!("(?:.*/)?{}", regex::escape(stripped)));
                        }
                    } else {
                        pattern.push_str(".*");
                    }
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
