use std::fmt;

use crate::prelude::*;
use itertools::Itertools;
use ndarray::{Array2, Axis};

pub struct Day04 {
    input: BingoInput,
}

pub fn new(input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
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
            let mut board: Array2<BingoCell> = Array2::default((5, 5));
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

    Ok(Box::new(Day04 {
        input: BingoInput {
            boards,
            number_order,
        },
    }))
}

impl Day for Day04 {
    fn part_1(&self) -> Answer {
        let mut input = self.input.clone();
        for n in input.number_order {
            for board in input.boards.iter_mut() {
                if board.mark_cell(n) == GameState::Completed {
                    return Answer::Integer(board.score());
                }
            }
        }
        Answer::None
    }

    fn part_2(&self) -> Answer {
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
                    return Answer::Integer(board.score());
                }
            }
        }
        Answer::None
    }
}

#[derive(Clone)]
struct BingoInput {
    boards: Vec<BingoBoard>,
    number_order: Vec<usize>,
}

#[derive(Clone)]
struct BingoBoard {
    data: Array2<BingoCell>,
    last_value: usize,
    state: GameState,
}

impl BingoBoard {
    fn new(board: Array2<BingoCell>) -> Self {
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

        let row = self.data.index_axis(Axis(0), i);
        let col = self.data.index_axis(Axis(1), j);

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

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part_1() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(4512));
    }

    #[test]
    fn part_2() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(1924));
    }
}
