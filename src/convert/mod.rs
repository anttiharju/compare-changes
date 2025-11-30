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
                if let Some(&path::Segment::Literal(lit)) = iter.peek() // followed by a Literal
                    && let Some(stripped) = lit.strip_prefix('/') // which starts with '/'
                    && !stripped.is_empty()
                // and there's something left after stripping
                {
                    iter.next(); // consume the literal segment
                    pattern.push_str(&format!("(?:.*/)?{}", regex::escape(stripped)));
                } else {
                    // If the next segment was "/" (i.e. stripped was empty), consume it
                    // so it's not processed again by the outer loop.
                    if let Some(&path::Segment::Literal(lit)) = iter.peek()
                        && let Some(_) = lit.strip_prefix('/')
                    {
                        iter.next();
                    }
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
