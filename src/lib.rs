mod convert;
mod path;

use std::fmt;

#[derive(Debug)]
pub enum Error {
    Parse(Vec<String>), // convert to strings for simplicity
    Regex(regex::Error),
}

impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Error::Regex(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(msgs) => write!(f, "{}", msgs.join("; ")),
            Error::Regex(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Regex(e) => Some(e),
            _ => None,
        }
    }
}

pub fn path_matches(path: &str, files: &[&str]) -> Result<Option<usize>, Error> {
    if files.is_empty() {
        return Ok(None);
    }

    // Remove leading '?', '+' since that's what GitHub does and also '!' because it's not meaningful when not part of a group
    let pattern = match path.chars().next() {
        Some(c) if matches!(c, '?' | '+' | '!') => &path[c.len_utf8()..],
        _ => path,
    };

    // Parse the single path — map chumsky Rich errors to strings
    let parsed_path = match path::parse(pattern) {
        Ok(p) => p,
        Err(errs) => {
            let msgs = errs.into_iter().map(|e| e.to_string()).collect();
            return Err(Error::Parse(msgs));
        }
    };

    // Build regex from the parsed path — propagate compilation errors
    let re = convert::path_to_regex(&parsed_path)?;

    // Check if any file matches the compiled regex
    Ok(files.iter().enumerate().find(|(_, file)| re.is_match(file)).map(|(i, _)| i))
}
