#[cfg(test)]
use super::{parse, BracketContent, Path, Segment};

#[test]
fn parse_literal() {
    assert_eq!(
        parse("foo"),
        Path {
            segments: vec![Segment::Literal("foo".to_string())]
        }
    );
}

#[test]
fn parse_slash() {
    assert_eq!(
        parse("docs/"),
        Path {
            segments: vec![Segment::Literal("docs".to_string()), Segment::Slash]
        }
    );
}

#[test]
fn parse_single_star() {
    assert_eq!(
        parse("bar*"),
        Path {
            segments: vec![Segment::Literal("bar".to_string()), Segment::SingleStar]
        }
    );
}

#[test]
fn parse_double_star() {
    assert_eq!(
        parse("baz/**"),
        Path {
            segments: vec![Segment::Literal("baz".to_string()), Segment::Slash, Segment::DoubleStar]
        }
    );
}

#[test]
fn parse_question_mark() {
    assert_eq!(
        parse("*.abc?"),
        Path {
            segments: vec![Segment::SingleStar, Segment::Literal(".ab".to_string()), Segment::QuestionMark('c')]
        }
    );
}

#[test]
fn parse_plus() {
    assert_eq!(
        parse("xyz+"),
        Path {
            segments: vec![Segment::Literal("xy".to_string()), Segment::Plus('z')]
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
                Segment::Literal("at".to_string())
            ]
        }
    );
}

#[test]
fn parse_negation() {
    assert_eq!(
        parse("!important"),
        Path {
            segments: vec![Segment::Negation, Segment::Literal("important".to_string())]
        }
    );
}

#[test]
fn parse_exclamation_point() {
    assert_eq!(
        parse("important!"),
        Path {
            segments: vec![Segment::Literal("important!".to_string())]
        }
    );
}

#[test]
fn parse_emoji() {
    assert_eq!(
        parse("🗒️.md"),
        Path {
            segments: vec![Segment::Literal("🗒️.md".to_string())]
        }
    );
}
