#[cfg(test)]
use super::{parse, BracketContent, Path, Segment};

macro_rules! lit {
    ($s:expr) => {
        Segment::Literal($s.chars().collect())
    };
}

#[test]
fn parse_literal() {
    assert_eq!(parse("foo"), Path { segments: vec![lit!("foo")] });
}

#[test]
fn parse_slash() {
    assert_eq!(
        parse("docs/"),
        Path {
            segments: vec![lit!("docs"), Segment::Slash]
        }
    );
}

#[test]
fn parse_single_star() {
    assert_eq!(
        parse("bar*"),
        Path {
            segments: vec![lit!("bar"), Segment::SingleStar]
        }
    );
}

#[test]
fn parse_double_star() {
    assert_eq!(
        parse("baz/**"),
        Path {
            segments: vec![lit!("baz"), Segment::Slash, Segment::DoubleStar]
        }
    );
}

#[test]
fn parse_question_mark() {
    assert_eq!(
        parse("*.abc?"),
        Path {
            segments: vec![Segment::SingleStar, lit!(".ab"), Segment::QuestionMark('c')]
        }
    );
}

#[test]
fn parse_plus() {
    assert_eq!(
        parse("xyz+"),
        Path {
            segments: vec![lit!("xy"), Segment::Plus('z')]
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
                lit!("at")
            ]
        }
    );
}

#[test]
fn parse_exclamation_point() {
    assert_eq!(
        parse("important!"),
        Path {
            segments: vec![lit!("important!")]
        }
    );
}

#[test]
fn parse_emoji() {
    assert_eq!(
        parse("🗒️.md"),
        Path {
            segments: vec![lit!("🗒️.md")]
        }
    );
}
