use std::cmp::Ordering;

use regex::Regex;

use crate::prelude::*;

pub struct Day17 {
    target: Target,
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
            if x1 > x2 || y1 > y2 {
                return Err(AOCError::ParseError);
            }
            Ok(Self {
                target: Target::new((x1, x2), (y1, y2)),
            })
        } else {
            Err(AOCError::ParseError)
        }
    }

    fn part_1(&self) -> Answer {
        // Answer::Integer(max_y(&self.target))
        brute_force(&self.target, EndCondition::MaxY).into()
    }

    fn part_2(&self) -> Answer {
        brute_force(&self.target, EndCondition::NumTrajectories).into()
    }
}

#[allow(dead_code)]
fn max_y_naive(target: &Target) -> Option<usize> {
    // Assume that target is below y-axis and if the final y value is on target, then it is always
    // possible to reach to target
    if target.y.0 > 0 || target.y.1 > 0 {
        None
    } else {
        let y_velocity = (target.y.0 + 1).abs();
        let max_y = (y_velocity * (y_velocity + 1) / 2) as usize;
        Some(max_y)
    }
}

fn brute_force(target: &Target, end: EndCondition) -> Option<usize> {
    // Assume that target is below y axis
    if target.y.0 > 0 || target.y.1 > 0 {
        None
    } else {
        let mut count = 0;
        for y_velocity in ((target.y.0)..=(-target.y.0)).rev() {
            for x_velocity in target.x.0.min(0)..=target.x.1.max(0) {
                if simulate(target, (x_velocity, y_velocity)) {
                    match end {
                        EndCondition::MaxY => {
                            if y_velocity <= 0 {
                                return Some(0);
                            }
                            let temp = y_velocity as usize;
                            return Some(temp * (temp + 1) / 2);
                        }
                        EndCondition::NumTrajectories => {
                            count += 1;
                        }
                    }
                }
            }
        }

        match end {
            EndCondition::NumTrajectories => Some(count),
            EndCondition::MaxY => None,
        }
    }
}

fn simulate(target: &Target, velocity: (isize, isize)) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut dx = velocity.0;
    let mut dy = velocity.1;

    while y >= target.y.0 {
        // Update positions / velocities
        x += dx;
        y += dy;
        match dx.cmp(&0) {
            Ordering::Greater => dx -= 1,
            Ordering::Less => dx += 1,
            _ => (),
        }
        dy -= 1;

        // Check if hit
        if target.hit(x, y) {
            return true;
        }
    }

    false
}

struct Target {
    x: (isize, isize),
    y: (isize, isize),
}

impl Target {
    fn new(x: (isize, isize), y: (isize, isize)) -> Self {
        Self { x, y }
    }

    fn hit(&self, x: isize, y: isize) -> bool {
        x >= self.x.0 && x <= self.x.1 && y >= self.y.0 && y <= self.y.1
    }
}

enum EndCondition {
    MaxY,
    NumTrajectories,
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part_1() {
        let runner = Day17::new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(45));

        let input_2 = "target area: x=34..35, y=-8..-6";
        let runner = Day17::new(input_2.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(3));
    }

    #[test]
    fn part_2() {
        let runner = Day17::new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(112));
    }
}
