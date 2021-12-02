use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use anyhow::Result;
use aoc2021::prelude::*;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = read_input(env::args_os().nth(1).ok_or(AOCError::NoInput)?)?;

    part_01(&input[..]);
    part_02(&input[..]);

    Ok(())
}

fn read_input(path: impl AsRef<Path>) -> Result<Vec<u64>> {
    let file = File::open(path)?;
    let input = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(input)
}

fn part_01(input: &[u64]) {
    let inc_count = input
        .iter()
        .tuple_windows::<(_, _)>()
        .filter_map(|(a, b)| if b > a { Some(()) } else { None })
        .count();

    println!("Part 1: {}", inc_count);
}

fn part_02(input: &[u64]) {
    let inc_count = input
        .iter()
        .tuple_windows::<(_, _, _)>()
        .tuple_windows::<(_, _)>()
        .filter_map(|((a, b, c), (x, y, z))| {
            if x + y + z > a + b + c {
                Some(())
            } else {
                None
            }
        })
        .count();

    println!("Part 2: {}", inc_count);
}
