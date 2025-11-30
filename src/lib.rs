mod convert;
mod path;

#[derive(Debug)]
pub enum Error {
    // Chumsky-native parse diagnostics converted to strings for transport
    Parse(Vec<String>),
    Regex(regex::Error),
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
