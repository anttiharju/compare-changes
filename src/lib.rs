mod convert;
mod path;

#[derive(Debug)]
pub enum Error {
    Parse(path::ParseError),
    Regex(regex::Error),
}

impl From<path::ParseError> for Error {
    fn from(e: path::ParseError) -> Self {
        Error::Parse(e)
    }
}

impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Error::Regex(e)
    }
}

pub fn path_matches(path: &str, files: &[&str]) -> Result<Option<usize>, Error> {
    if files.is_empty() {
        return Ok(None);
    }

    // remove leading '!' since negations are not handled here
    let pattern = path.strip_prefix('!').unwrap_or(path);

    // Parse the single path (propagate parse errors via From)
    let parsed_path = path::parse(pattern)?;

    // Build regex from the parsed path — propagate compilation errors
    let re = convert::path_to_regex(&parsed_path)?;

    // Check if any file matches the compiled regex
    Ok(files.iter().enumerate().find(|(_, file)| re.is_match(file)).map(|(i, _)| i))
}
