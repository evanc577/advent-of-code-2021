use std::collections::BinaryHeap;

use ndarray::Array2;

use crate::prelude::*;

pub struct Day15 {
    grid: Array2<usize>,
}

pub fn new(input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
    let input: Vec<_> = input.collect();
    let rows = input.len();
    let cols = input.get(0).map(|line| line.len()).unwrap_or(0);
    let mut grid = Array2::from_elem((rows, cols), usize::MAX);
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.chars().take(cols).enumerate() {
            grid[[i, j]] = c
                .to_digit(10)
                .map(|d| d.try_into().ok())
                .flatten()
                .unwrap_or(usize::MAX);
        }
    }
    Ok(Box::new(Day15 { grid }))
}

impl Day for Day15 {
    fn part_1(&self) -> Answer {
        shortest_path(
            &self.grid,
            (0, 0),
            (self.grid.nrows() - 1, self.grid.ncols() - 1),
        )
        .into()
    }

    fn part_2(&self) -> Answer {
        // Generate larger grid
        let mut large_grid = Array2::zeros((5 * self.grid.nrows(), 5 * self.grid.ncols()));
        for y_tile in 0..5 {
            for x_tile in 0..5 {
                for i in 0..self.grid.nrows() {
                    for j in 0..self.grid.ncols() {
                        let factor = y_tile + x_tile;
                        let large_grid_y = y_tile * self.grid.nrows() + i;
                        let large_grid_x = x_tile * self.grid.ncols() + j;
                        large_grid[[large_grid_y, large_grid_x]] =
                            (self.grid[[i, j]] + factor - 1) % 9 + 1;
                    }
                }
            }
        }

        shortest_path(
            &large_grid,
            (0, 0),
            (large_grid.nrows() - 1, large_grid.ncols() - 1),
        )
        .into()
    }
}

fn shortest_path(
    grid: &Array2<usize>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    // Dijkstra's algorithm
    let mut dist = Array2::from_elem(grid.dim(), usize::MAX);
    let mut heap = BinaryHeap::with_capacity(grid.nrows() * grid.ncols());
    let mut next_positions = Vec::with_capacity(4);

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost);
        }

        // Already found a lower cost path
        if cost > dist[position] {
            continue;
        }

        // Adjacent cells
        next_positions.clear();
        if position.0 > 0 {
            next_positions.push((position.0 - 1, position.1));
        }
        if position.1 > 0 {
            next_positions.push((position.0, position.1 - 1));
        }
        if position.0 < grid.nrows() - 1 {
            next_positions.push((position.0 + 1, position.1));
        }
        if position.1 < grid.ncols() - 1 {
            next_positions.push((position.0, position.1 + 1));
        }

        // Try to find a shorter path
        for &next_position in &next_positions {
            let next = State {
                cost: cost + grid[next_position],
                position: next_position,
            };

            if next.cost < dist[next.position] {
                // Add shorter path to frontier
                heap.push(next);
                dist[next_position] = next.cost;
            }
        }
    }

    None
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse ordering for min heap
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn part_1() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(40));
    }

    #[test]
    fn part_2() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(315));
    }
}
