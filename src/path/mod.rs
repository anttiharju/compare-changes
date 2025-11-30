#[cfg(test)]
#[path = "test.rs"]
mod test;

use chumsky::error::Rich;
use chumsky::prelude::*;

// Short alias for the parser extra type used throughout this module.
// Use Rich errors so we can emit detailed chumsky-native diagnostics from validators.
type Extra<'a> = chumsky::extra::Full<Rich<'a, char>, (), ()>;

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

/// Parse the pattern into segments. Returns Err with chumsky Rich errors on malformed input.
pub fn parse<'a>(path: &'a str) -> Result<Path<'a>, Vec<Rich<'a, char>>> {
    // When a literal has a trailing modifier ('?' or '+') we may need to emit
    // two Segment values for a single parsed token. MultiSegment captures that.
    #[derive(Clone)]
    enum MultiSegment<'b> {
        One(Segment<'b>),
        Two(Segment<'b>, Segment<'b>),
    }

    // predicate for what starts a new segment
    let is_starter = move |c: &char| *c == STAR || *c == BRACKET_OPEN || *c == QUESTION_MARK || *c == PLUS;

    // literal: 1+ chars that are not segment starters, returned as a slice
    let literal = any::<&'a str, Extra<'a>>()
        .filter(move |c: &char| !is_starter(c))
        .repeated()
        .at_least(1)
        .to_slice();

    // literal possibly followed by '?' or '+'
    let literal_mod = literal.then(just(QUESTION_MARK).or(just(PLUS)).or_not()).map(|(lit, op)| {
        if let Some(opc) = op {
            // split last char off the literal and emit two segments if needed
            if let Some((off, last_ch)) = lit.char_indices().next_back() {
                let prefix = &lit[..off];
                if prefix.is_empty() {
                    if opc == QUESTION_MARK {
                        MultiSegment::One(Segment::QuestionMark(last_ch))
                    } else {
                        MultiSegment::One(Segment::Plus(last_ch))
                    }
                } else if opc == QUESTION_MARK {
                    MultiSegment::Two(Segment::Literal(prefix), Segment::QuestionMark(last_ch))
                } else {
                    MultiSegment::Two(Segment::Literal(prefix), Segment::Plus(last_ch))
                }
            } else {
                // defensive fallback
                if opc == QUESTION_MARK {
                    MultiSegment::One(Segment::QuestionMark('\0'))
                } else {
                    MultiSegment::One(Segment::Plus('\0'))
                }
            }
        } else {
            MultiSegment::One(Segment::Literal(lit))
        }
    });

    let double_star = just(STAR).then(just(STAR)).to(MultiSegment::One(Segment::DoubleStar));
    let single_star = just(STAR).to(MultiSegment::One(Segment::SingleStar));

    // bracket: '[' inner ']' where inner is any chars except ']'
    // validate bracket internal content here and emit chumsky Rich errors if invalid
    let bracket_inner =
        any::<&'a str, Extra<'a>>()
            .filter(|c: &char| *c != ']')
            .repeated()
            .to_slice()
            .validate(|inner: &str, map_extra: &mut _, emitter: &mut _| {
                let span = map_extra.span();
                // Validate inner content for empty and invalid ranges.
                if inner.is_empty() {
                    emitter.emit(Rich::custom(span, "empty bracket"));
                } else {
                    let content: Vec<(usize, char)> = inner.char_indices().collect();
                    let mut j = 0usize;
                    while j < content.len() {
                        if j + 2 < content.len() && content[j + 1].1 == '-' {
                            let (pos_a, a) = content[j];
                            let (_pos_dash, _dash) = content[j + 1];
                            let (_pos_b, b) = content[j + 2];
                            if a > b {
                                // emit error pointing to the 'a' char in the original input
                                let abs_start = span.start + pos_a;
                                let abs_end = abs_start + a.len_utf8();
                                let bad_span = abs_start..abs_end;
                                emitter.emit(Rich::custom(bad_span.into(), format!("invalid bracket range {a}-{b}")));
                            }
                            j += 3;
                        } else {
                            j += 1;
                        }
                    }
                }
                // return the raw inner slice; actual conversion to BracketContent happens below
                inner
            });

    let bracket = just(BRACKET_OPEN).ignore_then(bracket_inner).then_ignore(just(']')).map(|inner: &str| {
        // convert inner into BracketContent (we already validated ranges above via emitter)
        let content: Vec<(usize, char)> = inner.char_indices().collect();
        let mut singles = Vec::new();
        let mut ranges = Vec::new();
        let mut j = 0usize;
        while j < content.len() {
            if j + 2 < content.len() && content[j + 1].1 == '-' {
                let (_pos_a, a) = content[j];
                let (_pos_dash, _dash) = content[j + 1];
                let (_pos_b, b) = content[j + 2];
                ranges.push((a, b));
                j += 3;
            } else {
                singles.push(content[j].1);
                j += 1;
            }
        }
        MultiSegment::One(Segment::Bracket(BracketContent { singles, ranges }))
    });

    // a segment is one of the choices
    let segment = choice((double_star, single_star, bracket, literal_mod));

    // parse the whole input as a sequence of segments (collect into Vec<MultiSegment>)
    let parser = segment.repeated().collect::<Vec<_>>();

    let (maybe_out, errs) = parser.parse(path).into_output_errors();
    if !errs.is_empty() {
        return Err(errs);
    }

    let out = maybe_out.unwrap_or_default();
    // flatten MultiSegment -> Vec<Segment>
    let mut segments: Vec<Segment<'a>> = Vec::new();
    for m in out {
        match m {
            MultiSegment::One(s) => segments.push(s),
            MultiSegment::Two(a, b) => {
                segments.push(a);
                segments.push(b);
            }
        }
    }

    Ok(Path { segments })
}
