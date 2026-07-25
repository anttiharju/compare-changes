mod convert;
mod path;

use regex::Regex;
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

    let re = compile_pattern(pattern)?;

    // Check if any file matches the compiled regex
    Ok(files.iter().enumerate().find(|(_, file)| re.is_match(file)).map(|(i, _)| i))
}

/// Evaluate a set of patterns against a set of files with negation (`!`) support.
///
/// Unlike [`path_matches`], which evaluates a single pattern in isolation, this treats
/// `paths` as an ordered group in the same way GitHub applies path filters: patterns
/// are checked sequentially per file, and a pattern starting with `!` re-excludes any
/// file previously matched by an earlier pattern. A later positive pattern can re-include
/// a file that was excluded by a negation.
///
/// Returns `Ok(Some((path_index, file_index)))` for the first file that ends up included,
/// where `path_index` is the index of the last positive pattern that matched it.
/// Returns `Ok(None)` when no file remains included after applying all patterns.
pub fn paths_match(paths: &[&str], files: &[&str]) -> Result<Option<(usize, usize)>, Error> {
    if paths.is_empty() || files.is_empty() {
        return Ok(None);
    }

    // Pre-compile every pattern once
    let compiled: Vec<(bool, Regex)> = paths
        .iter()
        .map(|p| {
            let (negated, rest) = match p.chars().next() {
                Some('!') => (true, &p[1..]),
                _ => (false, *p),
            };
            // Match the same leading-char stripping that path_matches performs, for parity
            let pattern = match rest.chars().next() {
                Some(c) if matches!(c, '?' | '+') => &rest[c.len_utf8()..],
                _ => rest,
            };
            compile_pattern(pattern).map(|re| (negated, re))
        })
        .collect::<Result<_, _>>()?;

    for (fi, file) in files.iter().enumerate() {
        let mut matched_by: Option<usize> = None;
        for (pi, (negated, re)) in compiled.iter().enumerate() {
            if re.is_match(file) {
                matched_by = if *negated { None } else { Some(pi) };
            }
        }
        if let Some(pi) = matched_by {
            return Ok(Some((pi, fi)));
        }
    }

    Ok(None)
}

fn compile_pattern(pattern: &str) -> Result<Regex, Error> {
    // Parse the single path — map chumsky Rich errors to strings
    let parsed_path = match path::parse(pattern) {
        Ok(p) => p,
        Err(errs) => {
            let msgs = errs.into_iter().map(|e| e.to_string()).collect();
            return Err(Error::Parse(msgs));
        }
    };

    // Build regex from the parsed path — propagate compilation errors
    Ok(convert::path_to_regex(&parsed_path)?)
}
