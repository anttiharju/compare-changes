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
    Literal(String),         // foo: literal "foo"
    SingleStar,              // bar*: literal "bar" singlestar "*"
    DoubleStar,              // baz/**: literal "baz/" doublestar "**"
    QuestionMark(char),      // *.abc?: singlestar "*" literal ".ab" questionamrk "c?"
    Plus(char),              // xyz+: literal "xy" plus "z+"
    Bracket(BracketContent), // [CB]at: bracket {singles: ['C','B'], ranges: []} literal "at"
    Negation,                // !important: negation "!" literal "important"
}

pub fn parse(path: &str) -> Path {
    let mut segments = Vec::new();
    let mut chars = path.chars().peekable();

    // Special case: ! is negation only if it is the first character
    if chars.peek() == Some(&'!') {
        chars.next(); // consume '!'
        segments.push(Segment::Negation);
    }

    while let Some(ch) = chars.next() {
        match ch {
            '*' => {
                if chars.peek() == Some(&'*') {
                    chars.next(); // consume second *
                    segments.push(Segment::DoubleStar);
                } else {
                    segments.push(Segment::SingleStar);
                }
            }
            '[' => {
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
            ch if ch.is_alphabetic() || ch.is_numeric() || ch == '.' || ch == '/' || ch == '!' => {
                let mut lit = String::new();
                lit.push(ch);
                while let Some(&next) = chars.peek() {
                    if next.is_alphabetic() || next.is_numeric() || next == '.' || next == '/' || next == '!' {
                        lit.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if !lit.is_empty() {
                    if let Some(&next) = chars.peek() {
                        if next == '?' {
                            chars.next();
                            let last = lit.pop().unwrap();
                            segments.push(Segment::Literal(lit));
                            segments.push(Segment::QuestionMark(last));
                        } else if next == '+' {
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
            _ => {} // ignore other characters or handle as error if needed
        }
    }

    Path { segments }
}
