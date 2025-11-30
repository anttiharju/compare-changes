#[cfg(test)]
#[path = "test.rs"]
mod test;

use chumsky::prelude::*;

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
    // Raw intermediate segment representation coming out of chumsky
    #[derive(Clone)]
    enum RawSegment<'b> {
        Literal(&'b str, Option<char>), // slice and optional attached modifier ('?' or '+')
        SingleStar,
        DoubleStar,
        Bracket(&'b str, usize), // inner slice and start position of '[' in original input
    }

    // predicate for what starts a new segment
    let is_starter = move |c: &char| *c == STAR || *c == BRACKET_OPEN || *c == QUESTION_MARK || *c == PLUS;

    // literal: 1+ chars that are not segment starters, returned as a slice
    let literal = any::<&'a str, chumsky::extra::Full<chumsky::error::Simple<'a, char>, (), ()>>()
        .filter(move |c: &char| !is_starter(c))
        .repeated()
        .at_least(1)
        .to_slice();

    // literal possibly followed by '?' or '+'
    let literal_mod = literal
        .then(just(QUESTION_MARK).or(just(PLUS)).or_not())
        .map(|(lit, op)| RawSegment::Literal(lit, op));

    let double_star = just(STAR).then(just(STAR)).to(RawSegment::DoubleStar);

    let single_star = just(STAR).to(RawSegment::SingleStar);

    // bracket: '[' inner ']' where inner is any chars except ']'
    // we map_with_span to compute the position of the '[' (span.start is the start of inner,
    // so subtract 1 to get '[' position).
    let bracket_inner = any::<&'a str, chumsky::extra::Full<chumsky::error::Simple<'a, char>, (), ()>>()
        .filter(|c: &char| *c != ']')
        .repeated()
        .to_slice();

    let bracket = just(BRACKET_OPEN).ignore_then(bracket_inner).then_ignore(just(']')).map(|inner: &str| {
        // Compute the byte offset of `inner` within the original `path` using pointers.
        // This avoids depending on the parser-extra span type.
        let start_pos = inner.as_ptr() as usize - path.as_ptr() as usize;
        RawSegment::Bracket(inner, start_pos.saturating_sub(1))
    });

    // a segment is one of the choices
    let segment = choice((double_star, single_star, bracket, literal_mod));

    // parse the whole input as a sequence of segments (collect into Vec<RawSegment>)
    let parser = segment.repeated().collect::<Vec<_>>();

    let raw_segments: Vec<RawSegment> = parser.parse(path).into_result().map_err(|_err| {
        // If parsing failed, try to detect an unmatched '[' and return that error,
        // otherwise fall back to an UnmatchedBracket at the first '[' if present.
        let mut stack: Vec<usize> = Vec::new();
        for (pos, ch) in path.char_indices() {
            if ch == BRACKET_OPEN {
                stack.push(pos);
            } else if ch == ']' {
                stack.pop();
            }
        }
        if let Some(pos) = stack.into_iter().next() {
            ParseError::UnmatchedBracket(pos)
        } else if let Some((pos, _)) = path.char_indices().find(|(_, c)| *c == BRACKET_OPEN) {
            ParseError::UnmatchedBracket(pos)
        } else {
            // fallback: treat as unmatched bracket at 0
            ParseError::UnmatchedBracket(0)
        }
    })?;

    // Post-process raw segments into final Segment<'a> vector, validating brackets
    let mut segments: Vec<Segment<'a>> = Vec::new();

    for raw in raw_segments {
        match raw {
            RawSegment::SingleStar => segments.push(Segment::SingleStar),
            RawSegment::DoubleStar => segments.push(Segment::DoubleStar),
            RawSegment::Literal(lit, maybe_op) => {
                if let Some(op) = maybe_op {
                    // attach modifier to last char of lit
                    // find last char byte offset in the slice
                    if let Some((off, last_ch)) = lit.char_indices().next_back() {
                        let prefix = &lit[..off];
                        if op == QUESTION_MARK {
                            if prefix.is_empty() {
                                segments.push(Segment::QuestionMark(last_ch));
                            } else {
                                segments.push(Segment::Literal(prefix));
                                segments.push(Segment::QuestionMark(last_ch));
                            }
                        } else
                        /* '+' */
                        if prefix.is_empty() {
                            segments.push(Segment::Plus(last_ch));
                        } else {
                            segments.push(Segment::Literal(prefix));
                            segments.push(Segment::Plus(last_ch));
                        }
                    } else {
                        // defensive: shouldn't happen because literal has at least one char
                        if op == QUESTION_MARK {
                            segments.push(Segment::QuestionMark('\0'));
                        } else {
                            segments.push(Segment::Plus('\0'));
                        }
                    }
                } else {
                    segments.push(Segment::Literal(lit));
                }
            }
            RawSegment::Bracket(inner, start_pos) => {
                // Validate bracket content: must not be empty, ranges must be a <= b
                if inner.is_empty() {
                    return Err(ParseError::EmptyBracket(start_pos));
                }

                // collect inner char positions relative to inner start
                let content: Vec<(usize, char)> = inner.char_indices().collect();
                let mut singles = Vec::new();
                let mut ranges = Vec::new();

                let mut j = 0usize;
                while j < content.len() {
                    if j + 2 < content.len() && content[j + 1].1 == '-' {
                        let (pos_a, a) = content[j];
                        let (_pos_dash, _dash) = content[j + 1];
                        let (_pos_b, b) = content[j + 2];
                        if a <= b {
                            ranges.push((a, b));
                        } else {
                            // compute absolute position of 'a' in original input: '[' pos + 1 + pos_in_inner
                            let abs_pos = start_pos + 1 + pos_a;
                            return Err(ParseError::InvalidBracketRange { start_pos: abs_pos, a, b });
                        }
                        j += 3;
                    } else {
                        singles.push(content[j].1);
                        j += 1;
                    }
                }

                segments.push(Segment::Bracket(BracketContent { singles, ranges }));
            }
        }
    }

    Ok(Path { segments })
}
