use crate::path;

pub fn match_path(segments: &[path::Segment], text: &str) -> bool {
    match_path_recursive(segments, text, 0, 0)
}

fn match_path_recursive(segments: &[path::Segment], text: &str, seg_idx: usize, t_idx: usize) -> bool {
    if seg_idx >= segments.len() && t_idx >= text.len() {
        return true;
    }
    if seg_idx >= segments.len() {
        return false;
    }

    match &segments[seg_idx] {
        path::Segment::Literal(lit) => {
            if t_idx + lit.len() <= text.len() && &text[t_idx..t_idx + lit.len()] == lit {
                match_path_recursive(segments, text, seg_idx + 1, t_idx + lit.len())
            } else {
                false
            }
        }
        path::Segment::Slash => {
            if t_idx < text.len() && text.chars().nth(t_idx) == Some('/') {
                match_path_recursive(segments, text, seg_idx + 1, t_idx + 1)
            } else {
                false
            }
        }
        path::Segment::SingleStar => {
            let mut end = text.len();
            if let Some(pos) = text[t_idx..].find('/') {
                end = t_idx + pos;
            }
            for i in t_idx..=end {
                if match_path_recursive(segments, text, seg_idx + 1, i) {
                    return true;
                }
            }
            false
        }
        path::Segment::DoubleStar => {
            let next_is_slash = matches!(segments.get(seg_idx + 1), Some(path::Segment::Slash));
            for i in t_idx..=text.len() {
                // If double-star matches zero and the next segment is Slash, allow skipping that Slash.
                if i == t_idx && next_is_slash && match_path_recursive(segments, text, seg_idx + 2, i) {
                    return true;
                }
                if match_path_recursive(segments, text, seg_idx + 1, i) {
                    return true;
                }
            }
            false
        }
        path::Segment::QuestionMark(c) => {
            // Try without the optional character
            if match_path_recursive(segments, text, seg_idx + 1, t_idx) {
                return true;
            }
            // Try with the optional character if it matches
            if t_idx < text.len() && text.chars().nth(t_idx) == Some(*c) {
                return match_path_recursive(segments, text, seg_idx + 1, t_idx + 1);
            }
            false
        }
        path::Segment::Plus(c) => {
            let mut count = 0;
            let mut curr_t_idx = t_idx;
            while curr_t_idx < text.len() && text.chars().nth(curr_t_idx) == Some(*c) {
                count += 1;
                curr_t_idx += 1;
            }
            if count == 0 {
                return false;
            }
            match_path_recursive(segments, text, seg_idx + 1, curr_t_idx)
        }
        path::Segment::Bracket(b) => {
            if t_idx >= text.len() {
                return false;
            }
            let ch = text.chars().nth(t_idx).unwrap();
            if b.singles.contains(&ch) || b.ranges.iter().any(|(start, end)| ch >= *start && ch <= *end) {
                match_path_recursive(segments, text, seg_idx + 1, t_idx + 1)
            } else {
                false
            }
        }
        path::Segment::Negation => false,
    }
}
