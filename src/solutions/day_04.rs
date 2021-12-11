use std::fmt;

use crate::prelude::*;
use itertools::Itertools;

pub struct Day04 {
    input: BingoInput,
}

pub fn new(input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
    Ok(Box::new(Day04 {
        input: parse_input(input)?,
    }))
}

impl Day for Day04 {
    fn part_1(&self) -> Option<usize> {
        let mut input = self.input.clone();
        for n in input.number_order {
            for board in input.boards.iter_mut() {
                if board.mark_cell(n) == GameState::Completed {
                    return Some(board.score());
                }
            }
        }
        None
    }

    fn part_2(&self) -> Option<usize> {
        let mut input = self.input.clone();
        let mut boards_ref: Vec<_> = input.boards.iter_mut().collect();
        let mut uncomplete_count = boards_ref.len();

        for n in input.number_order {
            for board in boards_ref.iter_mut() {
                if board.state == GameState::Completed {
                    continue;
                }

                if board.mark_cell(n) == GameState::Completed {
                    uncomplete_count -= 1;
                }

                if uncomplete_count == 0 {
                    return Some(board.score());
                }
            }
        }
        None
    }
}

fn parse_input(input: impl Iterator<Item = String>) -> Result<BingoInput, AOCError> {
    let mut input_iter = input;

    // Parse number order
    let number_order = input_iter
        .next()
        .ok_or(AOCError::ParseError)?
        .split(',')
        .map(|c| {
            c.parse::<usize>()
                .map_err(|e| AOCError::ParseIntError(e, c.into()))
        })
        .collect::<Result<_, _>>()?;

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
                        value: value_str
                            .parse()
                            .map_err(|e| AOCError::ParseIntError(e, value_str.into()))?,
                        marked: false,
                    }
                }
            }

            Ok(BingoBoard::new(board))
        })
        .collect::<Result<_, _>>()?;

    Ok(BingoInput {
        boards,
        number_order,
    })
}

#[derive(Clone)]
struct BingoInput {
    boards: Vec<BingoBoard>,
    number_order: Vec<usize>,
}

#[derive(Clone)]
struct BingoBoard {
    data: ndarray::Array2<BingoCell>,
    last_value: usize,
    state: GameState,
}

impl BingoBoard {
    fn new(board: ndarray::Array2<BingoCell>) -> Self {
        Self {
            data: board,
            last_value: 0,
            state: GameState::Uncompleted,
        }
    }

    fn is_complete(&self, i: usize, j: usize) -> bool {
        if self.state == GameState::Completed {
            return true;
        }

        let row = self.data.index_axis(ndarray::Axis(0), i);
        let col = self.data.index_axis(ndarray::Axis(1), j);

        if row.iter().all(|cell| cell.marked) {
            return true;
        }
        if col.iter().all(|cell| cell.marked) {
            return true;
        }

        false
    }

    fn score(&self) -> usize {
        self.data
            .iter()
            .filter_map(|cell| if !cell.marked { Some(cell.value) } else { None })
            .sum::<usize>()
            * self.last_value
    }

    fn mark_cell(&mut self, value: usize) -> GameState {
        if let Some((cell, idx)) = self
            .data
            .iter_mut()
            .zip(0..)
            .find(|(cell, _)| cell.value == value)
        {
            cell.marked = true;
            self.last_value = value;

            let (i, j) = if let [rows, cols] = self.data.shape() {
                (idx / rows, idx % cols)
            } else {
                return self.state;
            };

            match self.is_complete(i, j) {
                true => {
                    self.state = GameState::Completed;
                    return GameState::Completed;
                }
                false => return GameState::Uncompleted,
            }
        }

        self.state
    }
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "score: {}", self.score())
    }
}

#[derive(Clone, Default)]
struct BingoCell {
    value: usize,
    marked: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Completed,
    Uncompleted,
}
