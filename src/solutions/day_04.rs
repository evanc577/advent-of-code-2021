use std::{path::Path, fmt};

use aoc2021::prelude::*;
use itertools::Itertools;

pub fn run(input_path: impl AsRef<Path>) -> Result<(), AOCError> {
    let input = parse_input(input_path)?;

    part_01(input);
    // part_02(&input);

    Ok(())
}

fn parse_input(input_path: impl AsRef<Path>) -> Result<BingoInput, AOCError> {
    let mut input_iter = read_input_lines(input_path)?;

    // Parse number order
    let number_order = input_iter
        .next()
        .ok_or(AOCError::ParseError)?
        .split(',')
        .map(|c| c.parse::<usize>().map_err(|_| AOCError::ParseError))
        .collect::<Result<Vec<_>, AOCError>>()?;

    // Parse boards
    let boards = input_iter
        .chunks(6)
        .into_iter()
        .map(|chunk| {
            let mut board: ndarray::Array2<BingoCell> = ndarray::Array2::default((5, 5));
            // Parse one board
            for (i, line) in chunk.skip(1).take(5).enumerate() {
                // Parse one line
                for (j, value_str) in line
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .take(5)
                    .enumerate()
                {
                    board[[i, j]] = BingoCell {
                        value: value_str.parse().map_err(|_| AOCError::ParseError)?,
                        marked: false,
                    }
                }
            }

            Ok(BingoBoard::new(board))
        })
        .collect::<Result<Vec<_>, AOCError>>()?;

    Ok(BingoInput {
        boards,
        number_order,
    })
}

fn part_01(mut input: BingoInput) {
    for n in input.number_order {
        for board in input.boards.iter_mut() {
            board.mark_cell(n);
            if board.is_complete() {
                println!("Part 1: {}", board);
                return;
            }
        }
    }
    println!("Part 1: No solution");
}

struct BingoInput {
    boards: Vec<BingoBoard>,
    number_order: Vec<usize>,
}

struct BingoBoard {
    data: ndarray::Array2<BingoCell>,
    last_value: usize,
}

impl BingoBoard {
    fn new(board: ndarray::Array2<BingoCell>) -> Self {
        Self { data: board, last_value: 0 }
    }

    fn is_complete(&self) -> bool {
        let rows = self.data.axis_iter(ndarray::Axis(0));
        let cols = self.data.axis_iter(ndarray::Axis(1));
        let row_len = self.data.shape()[1];
        let mut diag_1 = self.data.iter().step_by(row_len + 1);
        let mut diag_2 = self.data.iter().skip(row_len - 1).step_by(row_len - 1);

        for line in rows.chain(cols) {
            if line.iter().all(|cell| cell.marked) {
                return true;
            }
        }

        if diag_1.all(|cell| cell.marked) || diag_2.all(|cell| cell.marked) {
            return true;
        }

        false
    }

    fn score(&self) -> usize {
        self.data
            .iter()
            .filter_map(|cell| if !cell.marked { Some(cell.value) } else { None })
            .sum::<usize>() * self.last_value
    }

    fn mark_cell(&mut self, value: usize) {
        if let Some(cell) = self.data.iter_mut().find(|cell| cell.value == value) {
            cell.marked = true;
            self.last_value = value;
        }
    }
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "score: {}", self.score())
    }
}

#[derive(Default)]
struct BingoCell {
    value: usize,
    marked: bool,
}
