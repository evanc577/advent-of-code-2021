use std::{path::Path, collections::HashMap};

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
    let mut fish: HashMap<usize, usize> = HashMap::new();
    for &x in input {
        if let Some(n) = fish.get_mut(&x) {
            *n += 1;
        } else {
            fish.insert(x, 1);
        }
    }

    for _ in 0..num_days {
        let new_fish = *fish.get(&0).unwrap_or(&0);

        // Decrease all fish timers by 1
        for x in 1..=8 {
            fish.insert(x - 1, *fish.get(&x).unwrap_or(&0));
        }

        // Add new fish with 8 day timers
        fish.insert(8, new_fish);

        // Reset fish with 0 day timers to 6 day timers
        fish.insert(6, fish.get(&6).unwrap_or(&0) + new_fish);
    }

    fish.iter().fold(0, |acc, (_, x)| acc + x)
}
