#[cfg(test)]
use super::{BracketContent, Path, Segment, parse};

#[test]
fn parse_literal() {
    assert_eq!(
        parse("foo"),
        Path {
            segments: vec![Segment::Literal("foo")]
        }
    );
}

#[test]
fn parse_single_star() {
    assert_eq!(
        parse("bar*"),
        Path {
            segments: vec![Segment::Literal("bar"), Segment::SingleStar]
        }
    );
}

#[test]
fn parse_double_star() {
    assert_eq!(
        parse("baz/**"),
        Path {
            segments: vec![Segment::Literal("baz/"), Segment::DoubleStar]
        }
    );
}

#[test]
fn parse_question_mark() {
    assert_eq!(
        parse("*.abc?"),
        Path {
            segments: vec![Segment::SingleStar, Segment::Literal(".ab"), Segment::QuestionMark('c')]
        }
    );
}

#[test]
fn parse_plus() {
    assert_eq!(
        parse("xyz+"),
        Path {
            segments: vec![Segment::Literal("xy"), Segment::Plus('z')]
        }
    );
}

#[test]
fn parse_bracket() {
    assert_eq!(
        parse("[CB]at"),
        Path {
            segments: vec![
                Segment::Bracket(BracketContent {
                    singles: vec!['C', 'B'],
                    ranges: vec![]
                }),
                Segment::Literal("at")
            ]
        }
    );
}

#[test]
fn parse_exclamation_point() {
    assert_eq!(
        parse("important!"),
        Path {
            segments: vec![Segment::Literal("important!")]
        }
    );
}

#[test]
fn parse_emoji() {
    assert_eq!(
        parse("🗒️.md"),
        Path {
            segments: vec![Segment::Literal("🗒️.md")]
        }
    );
}

#[test]
fn parse_bracket_with_hyphen() {
    assert_eq!(
        parse("[A-]"),
        Path {
            segments: vec![Segment::Bracket(BracketContent {
                singles: vec!['A', '-'],
                ranges: vec![],
            })]
        }
    );
}
