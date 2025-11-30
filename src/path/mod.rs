#[cfg(test)]
#[path = "test.rs"]
mod test;

#[derive(Debug, Clone, PartialEq)]
pub struct Path<'a> {
    pub segments: Vec<Segment<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BracketContent {
    pub singles: Vec<char>,
    pub ranges: Vec<(char, char)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Segment<'a> {
    Literal(&'a str),        // foo: literal "foo"
    SingleStar,              // bar*: literal "bar" singlestar "*"
    DoubleStar,              // baz/**: literal "baz/" doublestar "**"
    QuestionMark(char),      // *.abc?: singlestar "*" literal ".ab" questionmark "c?"
    Plus(char),              // xyz+: literal "xy" plus "z+"
    Bracket(BracketContent), // [CB]at: bracket {singles: ['C','B'], ranges: []} literal "at"
}

macro_rules! define_starters {
    ($($name:ident => $val:expr),* $(,)?) => {
        $(const $name: char = $val;)*
        const SEGMENT_STARTERS: &[char] = &[$($name),*];
    };
}

define_starters! {
    STAR => '*',
    BRACKET_OPEN => '[',
    QUESTION_MARK => '?',
    PLUS => '+',
}

/// Parse errors returned by `parse`.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnmatchedBracket(usize), // position of '['
    EmptyBracket(usize),     // position of '[' with no content
    InvalidBracketRange { start_pos: usize, a: char, b: char },
}

/// Parse the pattern into segments. Returns Err on malformed input (e.g. unmatched '[').
pub fn parse<'a>(path: &'a str) -> Result<Path<'a>, ParseError> {
    let mut segments = Vec::new();
    let mut iter = path.char_indices().peekable();

    while let Some((start, ch)) = iter.next() {
        match ch {
            STAR => {
                if let Some(&(_, next_ch)) = iter.peek() {
                    if next_ch == STAR {
                        // consume second '*'
                        iter.next();
                        segments.push(Segment::DoubleStar);
                    } else {
                        segments.push(Segment::SingleStar);
                    }
                } else {
                    segments.push(Segment::SingleStar);
                }
            }

            BRACKET_OPEN => {
                // collect content inside brackets with positions for better errors
                let mut content: Vec<(usize, char)> = Vec::new();
                let mut saw_close = false;
                for (pos, c) in iter.by_ref() {
                    if c == ']' {
                        saw_close = true;
                        break;
                    }
                    content.push((pos, c));
                }

                if !saw_close {
                    return Err(ParseError::UnmatchedBracket(start));
                }

                if content.is_empty() {
                    return Err(ParseError::EmptyBracket(start));
                }

                let mut singles = Vec::new();
                let mut ranges = Vec::new();
                let mut j = 0;
                while j < content.len() {
                    if j + 2 < content.len() && content[j + 1].1 == '-' {
                        let (pos_a, a) = content[j];
                        let (_pos_dash, _dash) = content[j + 1];
                        let (_pos_b, b) = content[j + 2];
                        if a <= b {
                            ranges.push((a, b));
                        } else {
                            return Err(ParseError::InvalidBracketRange { start_pos: pos_a, a, b });
                        }
                        j += 3;
                    } else {
                        singles.push(content[j].1);
                        j += 1;
                    }
                }

                segments.push(Segment::Bracket(BracketContent { singles, ranges }));
            }

            _ => {
                // start of a literal: consume until a segment starter
                let lit_start = start;
                while let Some(&(_, next)) = iter.peek() {
                    if SEGMENT_STARTERS.contains(&next) {
                        break;
                    }
                    iter.next();
                }
                let lit_end = iter.peek().map(|(pos, _)| *pos).unwrap_or(path.len());
                let lit = &path[lit_start..lit_end];

                if !lit.is_empty() {
                    // handle trailing '?' or '+' attached to last char of the literal
                    if let Some(&(_, next_ch)) = iter.peek()
                        && (next_ch == QUESTION_MARK || next_ch == PLUS)
                    {
                        // consume the modifier
                        iter.next();
                        if let Some((off, last_ch)) = lit.char_indices().next_back() {
                            let prefix = &lit[..off];
                            if next_ch == QUESTION_MARK {
                                if prefix.is_empty() {
                                    // impossible here since lit isn't empty, but keep defensive
                                    segments.push(Segment::QuestionMark(last_ch));
                                } else {
                                    segments.push(Segment::Literal(prefix));
                                    segments.push(Segment::QuestionMark(last_ch));
                                }
                            } else if prefix.is_empty() {
                                segments.push(Segment::Plus(last_ch));
                            } else {
                                segments.push(Segment::Literal(prefix));
                                segments.push(Segment::Plus(last_ch));
                            }
                            continue;
                        }
                    }

                    segments.push(Segment::Literal(lit));
                }
            }
        }
    }

    Ok(Path { segments })
}
