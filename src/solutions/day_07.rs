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
        .map(|s| s.parse().map_err(|e| AOCError::ParseIntError(e, s.into())))
        .collect::<Result<_, _>>()?;
    Ok(input)
}

fn part_01(input: &[usize]) {
    let fuel = calculate(input, &|a, b| abs_diff(a, b));
    println!("Part 1: {}", fuel);
}

fn part_02(input: &[usize]) {
    let fuel = calculate(input, &|a, b| triangular(abs_diff(a, b)));
    println!("Part 2: {}", fuel);
}

fn calculate(input: &[usize], f: &dyn Fn(usize, usize) -> usize) -> usize {
    let max = *input.iter().max().unwrap_or(&0);
    let min = *input.iter().min().unwrap_or(&0);

    (min..=max)
        .map(|l| input.iter().fold(0, |acc, x| acc + f(l, *x)))
        .min()
        .unwrap_or(0)
}

fn abs_diff(a: usize, b: usize) -> usize {
    a.saturating_sub(b).max(b.saturating_sub(a))
}

fn triangular(n: usize) -> usize {
    (n * (n + 1)) / 2
}
