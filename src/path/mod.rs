#[cfg(test)]
#[path = "test.rs"]
mod test;

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BracketContent {
    pub singles: Vec<char>,
    pub ranges: Vec<(char, char)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Literal(Vec<char>),      // foo: literal ["f","o","o"]
    Slash,                   // docs/: literal "docs" slash "/"
    SingleStar,              // bar*: literal "bar" singlestar "*"
    DoubleStar,              // baz/**: literal "baz/" doublestar "**"
    QuestionMark(char),      // *.abc?: singlestar "*" literal ".ab" questionamrk "c?"
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
    SLASH => '/',
    STAR => '*',
    BRACKET_OPEN => '[',
    QUESTION_MARK => '?',
    PLUS => '+',
}

pub fn parse(path: &str) -> Path {
    let mut segments = Vec::new();
    let mut chars = path.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            SLASH => {
                segments.push(Segment::Slash);
            }
            STAR => {
                if chars.peek() == Some(&STAR) {
                    chars.next(); // consume second *
                    segments.push(Segment::DoubleStar);
                } else {
                    segments.push(Segment::SingleStar);
                }
            }
            BRACKET_OPEN => {
                let mut singles = Vec::new();
                let mut ranges = Vec::new();
                while let Some(ch) = chars.next() {
                    if ch == ']' {
                        break;
                    }
                    if let Some(&'-') = chars.peek() {
                        chars.next(); // consume -
                        if let Some(end) = chars.next() {
                            ranges.push((ch, end));
                        }
                    } else {
                        singles.push(ch);
                    }
                }
                segments.push(Segment::Bracket(BracketContent { singles, ranges }));
            }
            _ => {
                let mut lit: Vec<char> = Vec::new();
                lit.push(ch);
                while let Some(&next) = chars.peek() {
                    if SEGMENT_STARTERS.contains(&next) {
                        break;
                    } else {
                        lit.push(chars.next().unwrap());
                    }
                }
                if !lit.is_empty() {
                    if let Some(&next) = chars.peek() {
                        if next == QUESTION_MARK {
                            chars.next();
                            let last = lit.pop().unwrap();
                            segments.push(Segment::Literal(lit));
                            segments.push(Segment::QuestionMark(last));
                        } else if next == PLUS {
                            chars.next();
                            let last = lit.pop().unwrap();
                            segments.push(Segment::Literal(lit));
                            segments.push(Segment::Plus(last));
                        } else {
                            segments.push(Segment::Literal(lit));
                        }
                    } else {
                        segments.push(Segment::Literal(lit));
                    }
                }
            }
        }
    }

    Path { segments }
}
