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
        const SEGMENT_STARTERS: &[char] = &[$($name),*];
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
    // literal: 1+ chars that are not segment starters, returned as a slice
    let literal = none_of(SEGMENT_STARTERS).repeated().at_least(1).to_slice();

    // literal possibly followed by '?' or '+'
    let literal_mod = literal
        .then(just(QUESTION_MARK).or(just(PLUS)).or_not())
        .map(|(lit, op): (&str, Option<char>)| match (op, lit.chars().next_back()) {
            (Some(QUESTION_MARK), Some(last)) => {
                let prefix = &lit[..lit.len() - last.len_utf8()];
                if prefix.is_empty() {
                    vec![Segment::QuestionMark(last)]
                } else {
                    vec![Segment::Literal(prefix), Segment::QuestionMark(last)]
                }
            }
            (Some(PLUS), Some(last)) => {
                let prefix = &lit[..lit.len() - last.len_utf8()];
                if prefix.is_empty() {
                    vec![Segment::Plus(last)]
                } else {
                    vec![Segment::Literal(prefix), Segment::Plus(last)]
                }
            }
            _ => vec![Segment::Literal(lit)],
        })
        .boxed();

    let double_star = just(STAR).then(just(STAR)).map(|_| vec![Segment::DoubleStar]);

    let single_star = just(STAR).map(|_| vec![Segment::SingleStar]);

    // bracket: '[' inner ']' where inner is any chars except ']'
    // single-pass: validate *and* build BracketContent here, emitting Rich errors as needed
    let bracket_inner =
        any::<&'a str, Extra<'a>>()
            .filter(|c| *c != ']')
            .repeated()
            .to_slice()
            .validate(|inner: &str, map_extra: &mut _, emitter: &mut _| {
                let span = map_extra.span();
                let content: Vec<(usize, char)> = inner.char_indices().collect();

                let mut singles = Vec::new();
                let mut ranges = Vec::new();
                let mut i = 0;

                if content.is_empty() {
                    emitter.emit(Rich::custom(span, "empty bracket"));
                    return BracketContent { singles, ranges };
                }

                while i < content.len() {
                    // If we have "<char> '-' <char>" treat as a range, otherwise a single.
                    if i + 2 < content.len() && content[i + 1].1 == '-' {
                        let (pos_a, a) = content[i];
                        let (_pos_dash, _dash) = content[i + 1];
                        let (_pos_b, b) = content[i + 2];

                        if a > b {
                            let abs_start = span.start + pos_a;
                            let abs_end = abs_start + a.len_utf8();
                            let bad_span = abs_start..abs_end;
                            emitter.emit(Rich::custom(bad_span.into(), format!("invalid bracket range {a}-{b}")));
                        }

                        ranges.push((a, b));
                        i += 3;
                    } else {
                        singles.push(content[i].1);
                        i += 1;
                    }
                }

                BracketContent { singles, ranges }
            });

    let bracket = just(BRACKET_OPEN)
        .ignore_then(bracket_inner)
        .then_ignore(just(']'))
        .map(|content: BracketContent| vec![Segment::Bracket(content)]);

    // a segment now produces Vec<Segment<'a>>; collect becomes Vec<Vec<Segment>>
    let segment = choice((double_star, single_star, bracket, literal_mod));

    let parser = segment.repeated().collect::<Vec<_>>();

    let (maybe_out, errs) = parser.parse(path).into_output_errors();
    if !errs.is_empty() {
        return Err(errs);
    }

    let out = maybe_out.unwrap_or_default();
    // flatten Vec<Vec<Segment>> -> Vec<Segment>
    let mut segments: Vec<Segment<'a>> = Vec::new();
    for group in out {
        for s in group {
            segments.push(s);
        }
    }

    Ok(Path { segments })
}
