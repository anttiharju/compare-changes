mod convert;
mod path;

pub fn path_matches(path: &str, files: &[&str]) -> Result<Option<usize>, regex::Error> {
    if files.is_empty() {
        return Ok(None);
    }

    // remove leading '!' since negations are not handled here
    let pattern = path.strip_prefix('!').unwrap_or(path);

    // Parse the single path
    let parsed_path = path::parse(pattern);

    // Build regex from the parsed path — propagate compilation errors
    let re = convert::path_to_regex(&parsed_path)?;

    // Check if any file matches the compiled regex
    Ok(files.iter().enumerate().find(|(_, file)| re.is_match(file)).map(|(i, _)| i))
}
