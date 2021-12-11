use crate::prelude::*;
use itertools::Itertools;

pub struct Day02 {
    input: Vec<Movement>,
}

pub fn new(input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
    Ok(Box::new(Day02 {
        input: parse_input(input)?,
    }))
}

impl Day for Day02 {
    fn part_1(&self) -> Option<usize> {
        let final_pos = self
            .input
            .iter()
            .fold(Default::default(), |mut acc: Position, m| {
                match m {
                    Movement::Forward(n) => acc.horizontal += n,
                    Movement::Down(n) => acc.depth += n,
                    Movement::Up(n) => acc.depth -= n,
                }
                acc
            });

        (final_pos.horizontal * final_pos.depth).try_into().ok()
    }

    fn part_2(&self) -> Option<usize> {
        let final_pos = self
            .input
            .iter()
            .fold(Default::default(), |mut acc: Position, m| {
                match m {
                    Movement::Forward(n) => {
                        acc.horizontal += n;
                        acc.depth += acc.aim * n;
                    }
                    Movement::Down(n) => acc.aim += n,
                    Movement::Up(n) => acc.aim -= n,
                }
                acc
            });

        (final_pos.horizontal * final_pos.depth).try_into().ok()
    }
}

fn parse_input(input: impl Iterator<Item = String>) -> Result<Vec<Movement>, AOCError> {
    let input = input
        .map(|s| {
            let split: (_, _) = s.split(' ').next_tuple().ok_or(AOCError::ParseError)?;
            match (split.0.to_lowercase().as_str(), split.1.parse()) {
                ("forward", Ok(n)) => Ok(Movement::Forward(n)),
                ("down", Ok(n)) => Ok(Movement::Down(n)),
                ("up", Ok(n)) => Ok(Movement::Up(n)),
                (s, Err(e)) => Err(AOCError::ParseIntError(e, s.into())),
                _ => Err(AOCError::ParseError),
            }
        })
        .collect::<Result<_, _>>()?;
    Ok(input)
}

#[derive(Default)]
struct Position {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

enum Movement {
    Forward(isize),
    Down(isize),
    Up(isize),
}
