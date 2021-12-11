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
