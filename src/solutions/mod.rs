use std::collections::BTreeMap;
use std::path::Path;

use crate::prelude::*;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;

pub fn dispatch(day: DayNum) -> Result<BTreeMap<usize, Vec<Option<usize>>>, AOCError> {
    let mut ret = BTreeMap::new();
    match day {
        DayNum::One(d, i) => {
            ret.insert(d, run_one_solution(d, i)?);
        }
        DayNum::All => {
            for d in 1..=11 {
                let input_path = format!("input/day_{:02}.txt", d);
                ret.insert(d, run_one_solution(d, input_path)?);
            }
        }
    }

    Ok(ret)
}

fn run_one_solution(
    day: usize,
    input_path: impl AsRef<Path>,
) -> Result<Vec<Option<usize>>, AOCError> {
    let input = read_input_lines(input_path)?;
    let runner = match day {
        1 => day_01::new(input)?,
        2 => day_02::new(input)?,
        3 => day_03::new(input)?,
        4 => day_04::new(input)?,
        5 => day_05::new(input)?,
        6 => day_06::new(input)?,
        7 => day_07::new(input)?,
        8 => day_08::new(input)?,
        9 => day_09::new(input)?,
        10 => day_10::new(input)?,
        11 => day_11::new(input)?,
        _ => unimplemented!(),
    };

    Ok(vec![runner.part_1(), runner.part_2()])
}
