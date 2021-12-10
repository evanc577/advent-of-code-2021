use std::path::Path;

use aoc2021::prelude::AOCError;

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

pub fn dispatch(day: usize, input_path: impl AsRef<Path>) -> Result<(), AOCError> {
    match day {
        1 => day_01::run(input_path)?,
        2 => day_02::run(input_path)?,
        3 => day_03::run(input_path)?,
        4 => day_04::run(input_path)?,
        5 => day_05::run(input_path)?,
        6 => day_06::run(input_path)?,
        7 => day_07::run(input_path)?,
        8 => day_08::run(input_path)?,
        9 => day_09::run(input_path)?,
        10 => day_10::run(input_path)?,
        _ => return Err(AOCError::DayOutOfRange(day)),
    }
    Ok(())
}
