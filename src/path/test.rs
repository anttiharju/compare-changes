#[cfg(test)]
use super::{BracketContent, Path, Segment, parse};

#[test]
fn parse_literal() {
    assert_eq!(
        parse("foo"),
        Ok(Path {
            segments: vec![Segment::Literal("foo")]
        })
    );
}

#[test]
fn parse_single_star() {
    assert_eq!(
        parse("bar*"),
        Ok(Path {
            segments: vec![Segment::Literal("bar"), Segment::SingleStar]
        })
    );
}

#[test]
fn parse_double_star() {
    assert_eq!(
        parse("baz/**"),
        Ok(Path {
            segments: vec![Segment::Literal("baz/"), Segment::DoubleStar]
        })
    );
}

#[test]
fn parse_question_mark() {
    assert_eq!(
        parse("*.abc?"),
        Ok(Path {
            segments: vec![Segment::SingleStar, Segment::Literal(".ab"), Segment::QuestionMark('c')]
        })
    );
}

#[test]
fn parse_plus() {
    assert_eq!(
        parse("xyz+"),
        Ok(Path {
            segments: vec![Segment::Literal("xy"), Segment::Plus('z')]
        })
    );
}

#[test]
fn parse_bracket() {
    assert_eq!(
        parse("[CB]at"),
        Ok(Path {
            segments: vec![
                Segment::Bracket(BracketContent {
                    singles: vec!['C', 'B'],
                    ranges: vec![]
                }),
                Segment::Literal("at")
            ]
        })
    );
}

#[test]
fn parse_exclamation_point() {
    assert_eq!(
        parse("important!"),
        Ok(Path {
            segments: vec![Segment::Literal("important!")]
        })
    );
}

#[test]
fn parse_emoji() {
    assert_eq!(
        parse("🗒️.md"),
        Ok(Path {
            segments: vec![Segment::Literal("🗒️.md")]
        })
    );
}

#[test]
fn parse_bracket_with_hyphen() {
    assert_eq!(
        parse("[A-]"),
        Ok(Path {
            segments: vec![Segment::Bracket(BracketContent {
                singles: vec!['A', '-'],
                ranges: vec![],
            })]
        })
    );
}

// add a test about:
// assert_path_match("+file.txt", "file.txt", false);
