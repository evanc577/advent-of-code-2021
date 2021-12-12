use crate::prelude::*;
use itertools::Itertools;

pub struct Day01 {
    input: Vec<usize>,
}

pub fn new(input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
    let parsed = input
        .map(|s| s.parse().map_err(|e| AOCError::ParseIntError(e, s)))
        .collect::<Result<_, _>>()?;
    Ok(Box::new(Day01 { input: parsed }))
}

impl Day for Day01 {
    fn part_1(&self) -> Option<usize> {
        Some(
            self.input
                .iter()
                .tuple_windows::<(_, _)>()
                .filter_map(|(a, b)| if b > a { Some(()) } else { None })
                .count(),
        )
    }

    fn part_2(&self) -> Option<usize> {
        Some(
            self.input
                .iter()
                .tuple_windows::<(_, _, _)>()
                .tuple_windows::<(_, _)>()
                .filter_map(|((a, b, c), (x, y, z))| {
                    if x + y + z > a + b + c {
                        Some(())
                    } else {
                        None
                    }
                })
                .count(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part_1() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Some(7));
    }

    #[test]
    fn part_2() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Some(5));
    }
}
