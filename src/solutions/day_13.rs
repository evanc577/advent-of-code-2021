use std::collections::HashSet;
use std::fmt;
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use itertools::Itertools;

use crate::prelude::*;

pub struct Day13 {
    dots: Vec<Dot>,
    folds: Vec<Fold>,
}

pub fn new(mut input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
    let dots: Vec<Dot> = input.by_ref().map_while(|line| line.parse().ok()).collect();
    let folds: Vec<Fold> = input.by_ref().map_while(|line| line.parse().ok()).collect();
    if input.any(|line| !line.is_empty()) {
        return Err(AOCError::ParseError);
    }
    Ok(Box::new(Day13 { dots, folds }))
}

impl Day for Day13 {
    fn part_1(&self) -> Answer {
        self.folds
            .get(0)
            .as_ref()
            .map(|fold| do_fold(&Paper::from_dots(&self.dots[..]), *fold).num_dots())
            .into()
    }

    fn part_2(&self) -> Answer {
        self.folds
            .iter()
            .fold(Paper::from_dots(&self.dots[..]), |acc, fold| {
                do_fold(&acc, fold)
            })
            .into()
    }
}

fn do_fold(paper: &Paper, fold: &Fold) -> Paper {
    let mut new_set: Paper = paper.iter().cloned().collect();
    for dot in paper.iter().cloned() {
        match fold {
            Fold::X(n) => {
                if dot.x > *n {
                    new_set.remove(&dot);
                    new_set.insert(Dot {
                        x: 2 * n - dot.x,
                        ..dot
                    });
                }
            }
            Fold::Y(n) => {
                if dot.y > *n {
                    new_set.remove(&dot);
                    new_set.insert(Dot {
                        y: 2 * n - dot.y,
                        ..dot
                    });
                }
            }
        }
    }
    new_set
}

#[derive(Debug)]
struct Paper(HashSet<Dot>);

impl Paper {
    fn new() -> Self {
        Self(HashSet::new())
    }

    fn from_dots(dots: &[Dot]) -> Self {
        Self(dots.iter().cloned().collect())
    }

    fn num_dots(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_min = self.iter().map(|d| d.x).min().unwrap_or(0);
        let x_max = self.iter().map(|d| d.x).max().unwrap_or(0);
        let y_min = self.iter().map(|d| d.y).min().unwrap_or(0);
        let y_max = self.iter().map(|d| d.y).max().unwrap_or(0);

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                match &self.get(&Dot { x, y }) {
                    Some(_) => {
                        write!(f, "#")?;
                    }
                    None => {
                        write!(f, ".")?;
                    }
                }
            }
            if y != y_max {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Deref for Paper {
    type Target = HashSet<Dot>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Paper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Paper> for Answer {
    fn from(p: Paper) -> Self {
        let mut buf = Vec::new();
        match write!(buf, "{}", p) {
            Ok(_) => Answer::Printable(buf),
            Err(e) => Answer::Error(Box::new(e)),
        }
    }
}

impl FromIterator<Dot> for Paper {
    fn from_iter<T: IntoIterator<Item = Dot>>(iter: T) -> Self {
        let mut paper = Paper::new();
        for dot in iter {
            paper.insert(dot);
        }
        paper
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Dot {
    x: isize,
    y: isize,
}

impl FromStr for Dot {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y)) = s.split(',').next_tuple() {
            let x = x
                .parse()
                .map_err(|e| AOCError::ParseIntError(e, x.into()))?;
            let y = y
                .parse()
                .map_err(|e| AOCError::ParseIntError(e, y.into()))?;
            return Ok(Self { x, y });
        }
        Err(AOCError::ParseError)
    }
}

#[derive(Debug)]
enum Fold {
    X(isize),
    Y(isize),
}

impl FromStr for Fold {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix("fold along ") {
            if let Some((axis, n_str)) = s.split('=').next_tuple() {
                let n: isize = n_str
                    .parse()
                    .map_err(|e| AOCError::ParseIntError(e, n_str.into()))?;
                match axis {
                    "x" => return Ok(Self::X(n)),
                    "y" => return Ok(Self::Y(n)),
                    _ => (),
                }
            }
        }
        Err(AOCError::ParseError)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn part_1() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(17));
    }

    #[test]
    fn part_2() {
        let expected: Vec<u8> = "#####
#...#
#...#
#...#
#####"
            .into();
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Printable(expected));
    }
}
