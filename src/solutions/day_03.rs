use std::{fmt, path::Path};

use aoc2021::prelude::*;

pub fn run(input_path: impl AsRef<Path>) -> Result<(), AOCError> {
    let input = parse_input(input_path)?;

    part_01(&input);
    part_02(&input);

    Ok(())
}

fn parse_input(input_path: impl AsRef<Path>) -> Result<ndarray::Array2<Bit>, AOCError> {
    let input_lines: Vec<_> = read_input_lines(input_path)?.collect();
    let num_bits = input_lines[0].len();
    let mut arr = ndarray::Array2::default((input_lines.len(), num_bits));
    for (i, n) in input_lines.iter().enumerate() {
        for (j, c) in n.chars().enumerate() {
            arr[[i, j]] = c.into();
        }
    }
    Ok(arr)
}

fn part_01(input: &ndarray::Array2<Bit>) {
    let gamma: BinaryNumber = input
        .axis_iter(ndarray::Axis(1))
        .map(|col| {
            let zeros = col.iter().filter(|&&v| v == Bit::Zero).count();
            let ones = col.len() - zeros;
            if ones >= zeros {
                Bit::One
            } else {
                Bit::Zero
            }
        })
        .collect();
    let epsilon = !gamma.clone();
    let rate = Output { first: gamma, second: epsilon };

    println!("Part 1: {}", rate)
}

fn part_02(input: &ndarray::Array2<Bit>) {
    let o2 = part_02_helper(input.clone(), Box::new(|zeros, ones| { ones >= zeros }));
    let co2 = part_02_helper(input.clone(), Box::new(|zeros, ones| { zeros > ones}));
    let rating = Output { first: o2, second: co2 };

    println!("Part 2: {}", rating);
}

fn part_02_helper(mut input: ndarray::Array2<Bit>, pred: Box<dyn Fn(usize, usize) -> bool>) -> BinaryNumber {
    for j in 0..input.shape()[1] {
        let col = input.index_axis(ndarray::Axis(1), j);
        let zeros = col.iter().filter(|&&v| v == Bit::Zero).count();
        let ones = col.len() - zeros;

        let remove_pred = if pred(zeros, ones){
            |v: Bit| v != Bit::One
        } else {
            |v: Bit| v != Bit::Zero
        };

        for i in (0..input.shape()[0]).rev() {
            if remove_pred(input[[i, j]]) {
                input.remove_index(ndarray::Axis(0), i);
            }
        }

        if input.shape()[0] <= 1 {
            break;
        }
    }

    if input.is_empty() {
        return BinaryNumber::new();
    }

    input
        .index_axis(ndarray::Axis(0), 0)
        .iter()
        .map(|v| *v)
        .collect()
}

#[derive(Clone, Debug)]
struct BinaryNumber(Vec<Bit>);

impl BinaryNumber {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, elem: Bit) {
        self.0.push(elem)
    }

    fn to_usize(&self) -> usize {
        let mut n = 0;
        for binary_value in &self.0 {
            n <<= 1;
            n |= binary_value.value();
        }
        n
    }
}

impl FromIterator<Bit> for BinaryNumber {
    fn from_iter<T: IntoIterator<Item = Bit>>(iter: T) -> Self {
        let mut c = BinaryNumber::new();
        for i in iter {
            c.push(i);
        }
        c
    }
}

impl std::ops::Not for BinaryNumber {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut c = self;
        for i in c.0.iter_mut() {
            *i = !*i;
        }
        c
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Bit {
    Zero,
    One,
}

impl Bit {
    fn value(&self) -> usize {
        match self {
            Self::Zero => 0,
            Self::One => 1,
        }
    }
}

impl Default for Bit {
    fn default() -> Self {
        Self::Zero
    }
}

impl From<char> for Bit {
    fn from(c: char) -> Self {
        match c {
            '0' => Self::Zero,
            '1' => Self::One,
            _ => Self::One,
        }
    }
}

impl std::ops::Not for Bit {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Zero => Self::One,
            Self::One => Self::Zero,
        }
    }
}

struct Output {
    first: BinaryNumber,
    second: BinaryNumber,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "first: {}, second: {}, multiplied: {}",
            self.first.to_usize(),
            self.second.to_usize(),
            self.first.to_usize() * self.second.to_usize()
        )
    }
}
