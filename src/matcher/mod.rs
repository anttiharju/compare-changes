use crate::path;
use std::collections::HashMap;

pub fn match_path(segments: &[path::Segment], text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    let mut memo: HashMap<(usize, usize), bool> = HashMap::new();
    match_path_recursive(segments, &chars, 0, 0, &mut memo)
}

fn match_path_recursive(segments: &[path::Segment], text: &[char], seg_idx: usize, t_idx: usize, memo: &mut HashMap<(usize, usize), bool>) -> bool {
    if let Some(&res) = memo.get(&(seg_idx, t_idx)) {
        return res;
    }

    let result = if seg_idx >= segments.len() && t_idx >= text.len() {
        true
    } else if seg_idx >= segments.len() {
        false
    } else {
        match &segments[seg_idx] {
            path::Segment::Literal(lit_chars) => {
                let len = lit_chars.len();
                if t_idx + len <= text.len() && text[t_idx..t_idx + len] == lit_chars[..] {
                    match_path_recursive(segments, text, seg_idx + 1, t_idx + len, memo)
                } else {
                    false
                }
            }
            path::Segment::Slash => {
                if t_idx < text.len() && text[t_idx] == '/' {
                    match_path_recursive(segments, text, seg_idx + 1, t_idx + 1, memo)
                } else {
                    false
                }
            }
            path::Segment::SingleStar => {
                // single-star: match zero or more characters except '/'
                let end = match text[t_idx..].iter().position(|&c| c == '/') {
                    Some(pos) => t_idx + pos,
                    None => text.len(),
                };
                let mut ok = false;
                for i in t_idx..=end {
                    if match_path_recursive(segments, text, seg_idx + 1, i, memo) {
                        ok = true;
                        break;
                    }
                }
                ok
            }
            path::Segment::DoubleStar => {
                let next_is_slash = matches!(segments.get(seg_idx + 1), Some(path::Segment::Slash));
                let mut ok = false;
                for i in t_idx..=text.len() {
                    // If double-star matches zero and the next segment is Slash, allow skipping that Slash.
                    if i == t_idx && next_is_slash && match_path_recursive(segments, text, seg_idx + 2, i, memo) {
                        ok = true;
                        break;
                    }
                    if match_path_recursive(segments, text, seg_idx + 1, i, memo) {
                        ok = true;
                        break;
                    }
                }
                ok
            }
            path::Segment::QuestionMark(c) => {
                // Try without the optional character
                if match_path_recursive(segments, text, seg_idx + 1, t_idx, memo) {
                    true
                } else if t_idx < text.len() && text[t_idx] == *c {
                    match_path_recursive(segments, text, seg_idx + 1, t_idx + 1, memo)
                } else {
                    false
                }
            }
            path::Segment::Plus(c) => {
                let mut curr_t_idx = t_idx;
                while curr_t_idx < text.len() && text[curr_t_idx] == *c {
                    curr_t_idx += 1;
                }
                if curr_t_idx == t_idx {
                    false
                } else {
                    match_path_recursive(segments, text, seg_idx + 1, curr_t_idx, memo)
                }
            }
            path::Segment::Bracket(b) => {
                if t_idx >= text.len() {
                    false
                } else {
                    let ch = text[t_idx];
                    let in_singles = b.singles.contains(&ch);
                    let in_ranges = b.ranges.iter().any(|(start, end)| ch >= *start && ch <= *end);
                    if in_singles || in_ranges {
                        match_path_recursive(segments, text, seg_idx + 1, t_idx + 1, memo)
                    } else {
                        false
                    }
                }
            }
            path::Segment::Negation => false,
        }
    };

    memo.insert((seg_idx, t_idx), result);
    result
}
