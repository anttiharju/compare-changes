mod segment;

pub fn path_matches(path: &str, files: &[&str]) -> Option<usize> {
    if files.is_empty() {
        return None;
    }

    // Parse the single path
    let parsed_path = segment::parse(path);

    // If the path is negated, immediately return None
    if matches!(parsed_path.segments.first(), Some(segment::Segment::Negation)) {
        return None;
    }

    // Check if any file matches the path
    for (i, file) in files.iter().enumerate() {
        // Check if the file matches the path
        if match_path(&parsed_path.segments, file) {
            return Some(i);
        }
    }

    // No file matched the path
    None
}

fn match_path(segments: &[segment::Segment], text: &str) -> bool {
    match_path_recursive(segments, text, 0, 0)
}

fn match_path_recursive(segments: &[segment::Segment], text: &str, seg_idx: usize, t_idx: usize) -> bool {
    if seg_idx >= segments.len() && t_idx >= text.len() {
        return true;
    }
    if seg_idx >= segments.len() {
        return false;
    }

    match &segments[seg_idx] {
        segment::Segment::Literal(lit) => {
            if t_idx + lit.len() <= text.len() && &text[t_idx..t_idx + lit.len()] == lit {
                match_path_recursive(segments, text, seg_idx + 1, t_idx + lit.len())
            } else {
                false
            }
        }
        segment::Segment::SingleStar => {
            for i in t_idx..=text.len() {
                if match_path_recursive(segments, text, seg_idx + 1, i) {
                    return true;
                }
            }
            false
        }
        segment::Segment::DoubleStar => {
            for i in t_idx..=text.len() {
                if match_path_recursive(segments, text, seg_idx + 1, i) {
                    return true;
                }
            }
            false
        }
        segment::Segment::QuestionMark(c) => {
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
        segment::Segment::Plus(c) => {
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
        segment::Segment::Bracket(b) => {
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
        segment::Segment::Negation => false,
    }
}
