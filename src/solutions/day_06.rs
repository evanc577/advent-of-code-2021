use std::{path::Path, collections::HashMap};

use aoc2021::prelude::*;
use itertools::Itertools;

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
    let mut fish: Vec<_> = input.iter().cloned().collect();

    for _ in 0..80 {
        let mut new_fish = vec![];
        for f in fish.iter_mut() {
            if *f == 0 {
                *f = 6;
                new_fish.push(8);
            } else {
                *f -= 1;
            }
        }
        fish.append(&mut new_fish);
    }

    let num_fish = fish.len();
    println!("Part 1: {}", num_fish);
}

fn part_02(input: &[usize]) {
    let mut fish: HashMap<usize, usize> = HashMap::new();
    for &x in input {
        if let Some(n) = fish.get_mut(&x) {
            *n += 1;
        } else {
            fish.insert(x, 1);
        }
    }

    for _ in 0..256 {
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

    let num_fish = fish.iter().fold(0, |acc, (_, x)| acc + x);
    println!("Part 2: {}", num_fish);
}
