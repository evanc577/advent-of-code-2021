use std::fmt;
use std::path::Path;

use aoc2021::prelude::*;
use itertools::Itertools;

pub fn run(input_path: impl AsRef<Path>) -> Result<(), AOCError> {
    let input = parse_input(input_path)?;

    part_01(&input[..]);
    part_02(&input[..]);

    Ok(())
}

fn parse_input(input_path: impl AsRef<Path>) -> Result<Vec<Movement>, AOCError> {
    let input = read_input_lines(input_path)?
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

#[derive(Default)]
struct Position {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
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
