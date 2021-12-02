use anyhow::Result;
use aoc2021::prelude::*;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = parse_input()?;

    part_01(&input[..]);
    part_02(&input[..]);

    Ok(())
}

fn parse_input() -> Result<Vec<usize>> {
    let input = read_input_lines()?
        .map(|s| s.parse())
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
