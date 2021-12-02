use std::fmt::{Display, Formatter};

use anyhow::Result;
use aoc2021::prelude::*;
use itertools::Itertools;

#[derive(Default)]
struct Position {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "horizontal: {}, depth: {}, multiplied: {}",
            self.horizontal,
            self.depth,
            self.horizontal * self.depth
        )
    }
}

enum Movement {
    Forward(isize),
    Down(isize),
    Up(isize),
}

fn main() -> Result<()> {
    let input = parse_input()?;

    part_01(&input[..]);
    part_02(&input[..]);

    Ok(())
}

fn parse_input() -> Result<Vec<Movement>> {
    let input = read_input_lines()?
        .map(|s| {
            let split: (_, _) = s.split(' ').next_tuple().ok_or(AOCError::InputParseError)?;
            match (split.0.to_lowercase().as_str(), split.1.parse()) {
                ("forward", Ok(n)) => Ok(Movement::Forward(n)),
                ("down", Ok(n)) => Ok(Movement::Down(n)),
                ("up", Ok(n)) => Ok(Movement::Up(n)),
                _ => Err(AOCError::InputParseError),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(input)
}

fn part_01(input: &[Movement]) {
    let final_pos = input
        .iter()
        .fold(Default::default(), |mut acc: Position, m| {
            match m {
                Movement::Forward(n) => acc.horizontal += n,
                Movement::Down(n) => acc.depth += n,
                Movement::Up(n) => acc.depth -= n,
            }
            acc
        });

    println!("Part 1: {}", final_pos);
}

fn part_02(input: &[Movement]) {
    let final_pos = input
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

    println!("Part 2: {}", final_pos);
}
