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
    println!("Part 1: {}", simulate(input, 80));
}

fn part_02(input: &[usize]) {
    println!("Part 2: {}", simulate(input, 256));
}

fn simulate(input: &[usize], num_days: usize) -> usize {
    const NEW_FISH_TIMER: usize = 8;
    const RESET_FISH_TIMER: usize = 6;

    let mut fish: Vec<usize> = vec![0; NEW_FISH_TIMER + 1];
    for &x in input {
        fish[x] += 1;
    }

    for _ in 0..num_days {
        let num_new_fish = fish[0];

        // Decrease all fish timers by 1
        fish.rotate_left(1);

        // Add new fish
        fish[NEW_FISH_TIMER] = num_new_fish;
        fish[RESET_FISH_TIMER] += num_new_fish
    }

    fish.iter().sum()
}
