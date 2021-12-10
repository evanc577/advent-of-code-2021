use std::path::Path;

use aoc2021::prelude::*;

pub fn run(input_path: impl AsRef<Path>) -> Result<(), AOCError> {
    let input = parse_input(input_path)?;

    part_01(&input);
    part_02(&input);

    Ok(())
}

fn parse_input(input_path: impl AsRef<Path>) -> Result<Vec<Vec<Character>>, AOCError> {
    read_input_lines(input_path)?
        .map(|s| s.chars().map(|c| c.try_into()).collect::<Result<_, _>>())
        .collect::<Result<_, _>>()
}

fn part_01(input: &[Vec<Character>]) {
    let points: usize = input
        .iter()
        .map(|line| {
            let mut stack = Vec::new();
            for c in line {
                match c.open_close {
                    OpenClose::Open => stack.push(c),
                    OpenClose::Close => match stack.pop() {
                        None => return c.char_type.corrupt_point_value(),
                        Some(c_stack) => {
                            if c_stack.char_type == c.char_type
                                && c_stack.open_close == OpenClose::Open
                            {
                                continue;
                            }
                            return c.char_type.corrupt_point_value();
                        }
                    },
                }
            }
            0
        })
        .sum();

    println!("Part 1: {}", points);
}

fn part_02(input: &[Vec<Character>]) {
    let mut scores: Vec<_> = input
        .iter()
        .filter_map(|line| {
            let mut stack = Vec::new();
            for c in line {
                match c.open_close {
                    OpenClose::Open => stack.push(c),
                    OpenClose::Close => match stack.pop() {
                        None => return None,
                        Some(c_stack) => {
                            if c_stack.char_type == c.char_type
                                && c_stack.open_close == OpenClose::Open
                            {
                                continue;
                            }
                            return None;
                        }
                    },
                }
            }

            if stack.is_empty() {
                return None;
            }

            // Incomplete
            let score = stack.iter().rev().fold(0, |acc, x| {
                5 * acc + x.char_type.incomplete_point_value()
            });
            Some(score)
        })
        .collect();


    if scores.is_empty() {
        println!("Part 2: No scores");
        return;
    }

    let middle_score_idx = scores.len() / 2;
    scores[..].select_nth_unstable(middle_score_idx);
    let middle_score = scores[middle_score_idx];

    println!("Part 1: {}", middle_score);
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
