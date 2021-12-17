use std::collections::BTreeMap;
use std::path::Path;

use crate::prelude::*;

use self::day_01::Day01;
use self::day_02::Day02;
use self::day_03::Day03;
use self::day_04::Day04;
use self::day_05::Day05;
use self::day_06::Day06;
use self::day_07::Day07;
use self::day_08::Day08;
use self::day_09::Day09;
use self::day_10::Day10;
use self::day_11::Day11;
use self::day_12::Day12;
use self::day_13::Day13;
use self::day_14::Day14;
use self::day_15::Day15;
use self::day_16::Day16;
use self::day_17::Day17;

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
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;

pub fn dispatch(day: DayNum) -> Result<BTreeMap<usize, Vec<Answer>>, AOCError> {
    let mut ret = BTreeMap::new();
    match day {
        DayNum::One(d, i) => {
            ret.insert(d, run(get_runner(d, i)?)?);
        }
        DayNum::All => {
            for d in 1..=25 {
                let input_path = format!("input/day_{:02}.txt", d);
                match get_runner(d, input_path) {
                    Ok(r) => ret.insert(d, run(r)?),
                    Err(_) => continue,
                };
            }
        }
    }

    Ok(ret)
}

pub fn get_runner(
    day: usize,
    input_path: impl AsRef<Path>,
) -> Result<Box<dyn Day>, AOCError> {
    let input = read_input_lines(input_path)?;
    let runner: Box<dyn Day> = match day {
        1 => Box::new(Day01::new(input)?),
        2 => Box::new(Day02::new(input)?),
        3 => Box::new(Day03::new(input)?),
        4 => Box::new(Day04::new(input)?),
        5 => Box::new(Day05::new(input)?),
        6 => Box::new(Day06::new(input)?),
        7 => Box::new(Day07::new(input)?),
        8 => Box::new(Day08::new(input)?),
        9 => Box::new(Day09::new(input)?),
        10 => Box::new(Day10::new(input)?),
        11 => Box::new(Day11::new(input)?),
        12 => Box::new(Day12::new(input)?),
        13 => Box::new(Day13::new(input)?),
        14 => Box::new(Day14::new(input)?),
        15 => Box::new(Day15::new(input)?),
        16 => Box::new(Day16::new(input)?),
        17 => Box::new(Day17::new(input)?),
        _ => return Err(AOCError::DayOutOfRange(day)),
    };
    Ok(runner)
}

fn run(runner: Box<dyn Day>) -> Result<Vec<Answer>, AOCError> {
    Ok(vec![runner.part_1(), runner.part_2()])
}
