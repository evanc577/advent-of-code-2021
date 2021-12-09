use std::path::Path;

use aoc2021::prelude::*;
use ndarray::Array2;

pub fn run(input_path: impl AsRef<Path>) -> Result<(), AOCError> {
    let input = parse_input(input_path)?;

    part_01(&input);
    part_02(&input);

    Ok(())
}

fn parse_input(input_path: impl AsRef<Path>) -> Result<Array2<usize>, AOCError> {
    let input: Vec<_> = read_input_lines(input_path)?.collect();
    let line_len = input.get(0).ok_or(AOCError::ParseError)?.len();
    let mut arr = Array2::<usize>::from_elem((input.len() + 2, line_len + 2), usize::MAX);
    for i in 0..input.len() {
        for j in 0..line_len {
            let chars: Vec<_> = input[i].chars().map(|c| c.to_string()).collect();
            arr[[i + 1, j + 1]] = chars[j].parse().unwrap_or(0);
        }
    }
    Ok(arr)
}

fn part_01(input: &Array2<usize>) {
    let sum: usize = low_points(input)
        .iter()
        .map(|(i, j)| input[[*i, *j]] + 1)
        .sum();
    println!("Part 1: {}", sum);
}

fn part_02(input: &Array2<usize>) {
    let mut basins: Vec<_> = low_points(input)
        .iter()
        .map(|point| basin_size(input, *point))
        .collect();
    basins[..].select_nth_unstable_by(3, |a, b| b.cmp(a)); // reverse sort
    let product: usize = basins.iter().take(3).product();
    println!("Part 2: {}", product);
}

fn low_points(input: &Array2<usize>) -> Vec<(usize, usize)> {
    let low_points: Vec<_> = input
        .windows([3, 3])
        .into_iter()
        .enumerate()
        .filter_map(|(i, window)| {
            let min = window[[0, 1]]
                .min(window[[1, 0]])
                .min(window[[2, 1]])
                .min(window[[1, 2]]);
            let center = window[[1, 1]];
            if center < min {
                Some((i / (input.ncols() - 2) + 1, i % (input.ncols() - 2) + 1))
            } else {
                None
            }
        })
        .collect();
    low_points
}

fn basin_size(input: &Array2<usize>, point: (usize, usize)) -> usize {
    // DFS
    let mut stack = vec![point];
    let mut visited = Array2::<bool>::default(input.dim());
    let mut count = 0;
    while let Some((i, j)) = stack.pop() {
        if !visited[[i, j]] {
            visited[[i, j]] = true;
            count += 1;
            // Up
            if i > 1 && input[[i - 1, j]] < 9 {
                stack.push((i - 1, j));
            }
            // Left
            if j > 1 && input[[i, j - 1]] < 9 {
                stack.push((i, j - 1));
            }
            // Down
            if i < input.nrows() - 1 && input[[i + 1, j]] < 9 {
                stack.push((i + 1, j));
            }
            // Right
            if j < input.ncols() - 1 && input[[i, j + 1]] < 9 {
                stack.push((i, j + 1));
            }
        }
    }
    count
}
