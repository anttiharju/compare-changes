mod matcher;
mod path;

pub fn path_matches(path: &str, files: &[&str]) -> Option<usize> {
    if files.is_empty() {
        return None;
    }

    // Parse the single path
    let parsed_path = path::parse(path);

    // If the path is negated, immediately return None
    if matches!(parsed_path.segments.first(), Some(path::Segment::Negation)) {
        return None;
    }

    // Check if any file matches the path
    files
        .iter()
        .enumerate()
        .find(|(_, file)| matcher::match_path(&parsed_path.segments, file))
        .map(|(i, _)| i)
}
