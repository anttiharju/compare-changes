mod convert;
mod path;

pub fn path_matches(path: &str, files: &[&str]) -> Option<usize> {
    if files.is_empty() {
        return None;
    }

    // remove leading '!' since negations are not handled here
    let path = if path.starts_with('!') { &path[1..] } else { path };

    // Parse the single path
    let parsed_path = path::parse(path);

    // Build regex from the parsed path
    let re = match convert::path_to_regex(&parsed_path) {
        Ok(r) => r,
        Err(_) => return None,
    };

    // Check if any file matches the compiled regex
    files.iter().enumerate().find(|(_, file)| re.is_match(file)).map(|(i, _)| i)
}
