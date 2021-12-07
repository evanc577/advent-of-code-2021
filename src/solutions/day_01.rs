use std::path::Path;

use aoc2021::prelude::*;
use itertools::Itertools;

pub fn run(input_path: impl AsRef<Path>) -> Result<(), AOCError> {
    let input = parse_input(input_path)?;

    part_01(&input[..]);
    part_02(&input[..]);

    Ok(())
}

fn parse_input(input_path: impl AsRef<Path>) -> Result<Vec<usize>, AOCError> {
    let input = read_input_lines(input_path)?
        .map(|s| s.parse().map_err(|e| AOCError::ParseIntError(e, s)))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(input)
}

fn part_01(input: &[usize]) {
    let inc_count = input
        .iter()
        .tuple_windows::<(_, _)>()
        .filter_map(|(a, b)| if b > a { Some(()) } else { None })
        .count();

    println!("Part 1: {}", inc_count);
}

fn part_02(input: &[usize]) {
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
