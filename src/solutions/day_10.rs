use crate::prelude::*;

pub struct Day10 {
    input: Vec<Vec<Character>>,
}

pub fn new(input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
    let parsed: Result<_, _> = input
        .map(|s| s.chars().map(|c| c.try_into()).collect())
        .collect();
    Ok(Box::new(Day10 { input: parsed? }))
}

impl Day for Day10 {
    fn part_1(&self) -> Option<usize> {
        let points: usize = filter_lines(&self.input, LineStatus::Corrupted)
            .iter()
            .filter_map(|c| {
                if let BadLineChars::Corrupted(c) = c {
                    Some(c.char_type.corrupt_point_value())
                } else {
                    None
                }
            })
            .sum();
        Some(points)
    }

    fn part_2(&self) -> Option<usize> {
        let mut scores: Vec<_> = filter_lines(&self.input, LineStatus::Incomplete)
            .iter()
            .filter_map(|c| {
                if let BadLineChars::Incomplete(chars) = c {
                    let score = chars
                        .iter()
                        .rev()
                        .fold(0, |acc, x| 5 * acc + x.char_type.incomplete_point_value());
                    Some(score)
                } else {
                    None
                }
            })
            .collect();

        if scores.is_empty() {
            return None;
        }

        let middle_score_idx = scores.len() / 2;
        scores[..].select_nth_unstable(middle_score_idx);
        let middle_score = scores[middle_score_idx];
        Some(middle_score)
    }
}

fn filter_lines(input: &[Vec<Character>], status: LineStatus) -> Vec<BadLineChars> {
    input
        .iter()
        .filter_map(|line| {
            let mut stack = Vec::new();
            for c in line {
                match c.open_close {
                    OpenClose::Open => stack.push(c),
                    OpenClose::Close => {
                        if let Some(c_stack) = stack.pop() {
                            if c_stack.char_type == c.char_type
                                && c_stack.open_close == OpenClose::Open
                            {
                                // Matching bracket, do next character
                                continue;
                            }
                        }

                        // Corrupt
                        if status == LineStatus::Corrupted {
                            return Some(BadLineChars::Corrupted(c));
                        }

                        return None;
                    }
                }
            }

            // Complete
            if stack.is_empty() {
                return None;
            }

            // Incomplete
            if status == LineStatus::Incomplete {
                return Some(BadLineChars::Incomplete(stack));
            }

            None
        })
        .collect()
}

#[derive(Debug)]
struct Character {
    char_type: CharacterType,
    open_close: OpenClose,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum CharacterType {
    Parenthesis,
    SquareBracket,
    CurlyBracket,
    AngleBracket,
}

impl CharacterType {
    fn corrupt_point_value(&self) -> usize {
        match *self {
            Self::Parenthesis => 3,
            Self::SquareBracket => 57,
            Self::CurlyBracket => 1197,
            Self::AngleBracket => 25137,
        }
    }

    fn incomplete_point_value(&self) -> usize {
        match *self {
            Self::Parenthesis => 1,
            Self::SquareBracket => 2,
            Self::CurlyBracket => 3,
            Self::AngleBracket => 4,
        }
    }
}

impl TryFrom<char> for Character {
    type Error = AOCError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use CharacterType::*;
        use OpenClose::*;

        match c {
            '(' => Ok(Character {
                char_type: Parenthesis,
                open_close: Open,
            }),
            ')' => Ok(Character {
                char_type: Parenthesis,
                open_close: Close,
            }),
            '[' => Ok(Character {
                char_type: SquareBracket,
                open_close: Open,
            }),
            ']' => Ok(Character {
                char_type: SquareBracket,
                open_close: Close,
            }),
            '{' => Ok(Character {
                char_type: CurlyBracket,
                open_close: Open,
            }),
            '}' => Ok(Character {
                char_type: CurlyBracket,
                open_close: Close,
            }),
            '<' => Ok(Character {
                char_type: AngleBracket,
                open_close: Open,
            }),
            '>' => Ok(Character {
                char_type: AngleBracket,
                open_close: Close,
            }),
            _ => Err(AOCError::ParseError),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OpenClose {
    Open,
    Close,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum LineStatus {
    Incomplete,
    Corrupted,
}

enum BadLineChars<'a> {
    Incomplete(Vec<&'a Character>),
    Corrupted(&'a Character),
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part_1() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Some(26397));
    }

    #[test]
    fn part_2() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Some(288957));
    }
}
