use std::path::Path;

use aoc2021::prelude::*;

pub fn run(input_path: impl AsRef<Path>) -> Result<(), AOCError> {
    let input = parse_input(input_path)?;

    part_01(&input[..]);
    part_02(&input[..]);

    Ok(())
}

fn parse_input(input_path: impl AsRef<Path>) -> Result<Vec<usize>, AOCError> {
    let input: Vec<_> = read_input_lines(input_path)?
        .next()
        .ok_or(AOCError::ParseError)?
        .split(',')
        .map(|s| s.parse().map_err(|_| AOCError::ParseError))
        .collect::<Result<Vec<_>, AOCError>>()?;
    Ok(input)
}

fn part_01(input: &[usize]) {
    let max = *input.iter().max().unwrap_or(&0);
    let min = *input.iter().min().unwrap_or(&0);
    let fuel = (min..=max)
        .into_iter()
        .map(|l| input.iter().fold(0, |acc, x| acc + abs_diff(l, *x)))
        .min()
        .unwrap_or(0);
    println!("Part 1: {}", fuel);
}

fn part_02(input: &[usize]) {
    let max = *input.iter().max().unwrap_or(&0);
    let min = *input.iter().min().unwrap_or(&0);
    let fuel = (min..=max)
        .into_iter()
        .map(|l| {
            input
                .iter()
                .fold(0, |acc, x| acc + triangular(abs_diff(l, *x)))
        })
        .min()
        .unwrap_or(0);
    println!("Part 2: {}", fuel);
}

fn abs_diff(a: usize, b: usize) -> usize {
    a.saturating_sub(b).max(b.saturating_sub(a))
}

fn triangular(n: usize) -> usize {
    (n * (n + 1)) / 2
}
