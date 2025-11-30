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

pub fn parse<'a>(path: &'a str) -> Path<'a> {
    let mut segments = Vec::new();
    let chars = path.char_indices().collect::<Vec<_>>();
    let mut i = 0;

    while i < chars.len() {
        let (start, ch) = chars[i];
        match ch {
            STAR => {
                if i + 1 < chars.len() && chars[i + 1].1 == STAR {
                    segments.push(Segment::DoubleStar);
                    i += 2;
                } else {
                    segments.push(Segment::SingleStar);
                    i += 1;
                }
            }
            BRACKET_OPEN => {
                i += 1; // skip [
                let mut content = Vec::new();
                while i < chars.len() {
                    let ch = chars[i].1;
                    if ch == ']' {
                        i += 1;
                        break;
                    }
                    content.push(ch);
                    i += 1;
                }
                let mut singles = Vec::new();
                let mut ranges = Vec::new();
                let mut j = 0;
                while j < content.len() {
                    if j + 2 < content.len() && content[j + 1] == '-' {
                        ranges.push((content[j], content[j + 2]));
                        j += 3;
                    } else {
                        singles.push(content[j]);
                        j += 1;
                    }
                }
                segments.push(Segment::Bracket(BracketContent { singles, ranges }));
            }
            _ => {
                let lit_start = start;
                i += 1;
                while i < chars.len() && !SEGMENT_STARTERS.contains(&chars[i].1) {
                    i += 1;
                }
                let lit_end = chars.get(i).map(|(pos, _)| *pos).unwrap_or(path.len());
                let lit = &path[lit_start..lit_end];
                if !lit.is_empty() {
                    if i < chars.len() && chars[i].1 == QUESTION_MARK {
                        i += 1;
                        let last = lit.chars().last().unwrap();
                        let prefix_len = lit.len() - last.len_utf8();
                        let prefix = &lit[..prefix_len];
                        segments.push(Segment::Literal(prefix));
                        segments.push(Segment::QuestionMark(last));
                    } else if i < chars.len() && chars[i].1 == PLUS {
                        i += 1;
                        let last = lit.chars().last().unwrap();
                        let prefix_len = lit.len() - last.len_utf8();
                        let prefix = &lit[..prefix_len];
                        segments.push(Segment::Literal(prefix));
                        segments.push(Segment::Plus(last));
                    } else {
                        segments.push(Segment::Literal(lit));
                    }
                }
            }
        }
    }

    Path { segments }
}
