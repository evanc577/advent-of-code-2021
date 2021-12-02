use std::env;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use anyhow::Result;
use aoc2021::prelude::*;

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
    let input = read_input(env::args_os().nth(1).ok_or(AOCError::NoInput)?)?;

    part_01(&input[..]);
    part_02(&input[..]);

    Ok(())
}

fn read_input(path: impl AsRef<Path>) -> Result<Vec<Movement>> {
    let file = File::open(path)?;
    let input = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|s| {
            let split: Vec<_> = s.split(" ").collect();
            if let [dir, n] = &split[..] {
                let n = n.parse::<isize>();
                if let Ok(n) = n {
                    match dir.to_lowercase().as_str() {
                        "forward" => return Ok(Movement::Forward(n)),
                        "down" => return Ok(Movement::Down(n)),
                        "up" => return Ok(Movement::Up(n)),
                        _ => return Err(AOCError::InputParseError).into(),
                    }
                }
            }
            Err(AOCError::InputParseError).into()
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(input)
}

fn part_01(input: &[Movement]) {
    let final_pos = input.iter().fold(
        Default::default(),
        |mut acc: Position, m| {
            match m {
                Movement::Forward(n) => acc.horizontal += n,
                Movement::Down(n) => acc.depth += n,
                Movement::Up(n) => acc.depth -= n,
            }
            acc
        },
    );

    println!("Part 1: {}", final_pos);
}

fn part_02(input: &[Movement]) {
    let final_pos = input.iter().fold(
        Default::default(),
        |mut acc: Position, m| {
            match m {
                Movement::Forward(n) => {
                    acc.horizontal += n;
                    acc.depth += acc.aim * n;
                }
                Movement::Down(n) => acc.aim += n,
                Movement::Up(n) => acc.aim -= n,
            }
            acc
        },
    );

    println!("Part 2: {}", final_pos);
}
