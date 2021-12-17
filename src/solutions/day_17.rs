use regex::Regex;

use crate::prelude::*;

pub struct Day17 {
    target_x: (isize, isize),
    target_y: (isize, isize),
}

impl Day for Day17 {
    fn new(mut input: impl Iterator<Item = String>) -> Result<Self, AOCError> {
        let line = input.next().ok_or(AOCError::NoInput)?;
        let re =
            Regex::new(r"target area: x=(\-?\d+)\.\.(\-?\d+), y=(\-?\d+)\.\.(\-?\d+)").unwrap();
        let caps = re.captures(&line).ok_or(AOCError::ParseError)?;
        let values: Vec<_> = caps
            .iter()
            .skip(1)
            .map(|cap| {
                let s = cap.ok_or(AOCError::ParseError)?.as_str();
                s.parse::<isize>()
                    .map_err(|e| AOCError::ParseIntError(e, s.into()))
            })
            .collect::<Result<_, _>>()?;
        if let [x1, x2, y1, y2] = values[..] {
            Ok(Self {
                target_x: (x1, x2),
                target_y: (y1, y2),
            })
        } else {
            Err(AOCError::ParseError)
        }
    }

    fn part_1(&self) -> Answer {
        Answer::Integer(max_y(self.target_y))
    }

    fn part_2(&self) -> Answer {
        Answer::None
    }
}

fn max_y(target_y: (isize, isize)) -> usize {
    if target_y.0 < 0 && target_y.1 < 0 {
        let n = (target_y.0 + 1).abs();
        return (n * (n + 1) / 2) as usize;
    }
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part_1() {
        let runner = Day17::new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(45));
    }

    #[test]
    fn part_2() {
        let runner = Day17::new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(112));
    }
}
