use crate::prelude::*;
use ndarray::{Array2, Axis};

pub struct Day03 {
    input: Array2<Bit>,
}

pub fn new(input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
    let input_lines: Vec<_> = input.collect();
    let num_bits = input_lines[0].len();
    let mut arr = Array2::default((input_lines.len(), num_bits));
    for (i, n) in input_lines.iter().enumerate() {
        for (j, c) in n.chars().enumerate() {
            arr[[i, j]] = c.into();
        }
    }
    Ok(Box::new(Day03 { input: arr }))
}

impl Day for Day03 {
    fn part_1(&self) -> Option<usize> {
        let gamma: BinaryNumber = self
            .input
            .axis_iter(Axis(1))
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
        Some(gamma.to_usize() * epsilon.to_usize())
    }

    fn part_2(&self) -> Option<usize> {
        let o2 = part_02_helper(self.input.clone(), &|zeros, ones| ones >= zeros);
        let co2 = part_02_helper(self.input.clone(), &|zeros, ones| zeros > ones);
        Some(o2.to_usize() * co2.to_usize())
    }
}

fn part_02_helper(mut input: Array2<Bit>, pred: &dyn Fn(usize, usize) -> bool) -> BinaryNumber {
    for j in 0..input.shape()[1] {
        let col = input.index_axis(Axis(1), j);
        let zeros = col.iter().filter(|&&v| v == Bit::Zero).count();
        let ones = col.len() - zeros;

        let remove_pred = if pred(zeros, ones) {
            |v: Bit| v != Bit::One
        } else {
            |v: Bit| v != Bit::Zero
        };

        for i in (0..input.shape()[0]).rev() {
            if remove_pred(input[[i, j]]) {
                input.remove_index(Axis(0), i);
            }
        }

        if input.shape()[0] <= 1 {
            break;
        }
    }

    if input.is_empty() {
        return BinaryNumber::new();
    }

    input.index_axis(Axis(0), 0).iter().copied().collect()
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

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part_1() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Some(198));
    }

    #[test]
    fn test_part_2() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Some(230));
    }
}
