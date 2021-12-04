use std::path::Path;

use aoc2021::prelude::AOCError;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

pub fn dispatch(day: usize, input_path: impl AsRef<Path>) -> Result<(), AOCError> {
    match day {
        1 => day_01::run(input_path)?,
        2 => day_02::run(input_path)?,
        3 => day_03::run(input_path)?,
        4 => day_04::run(input_path)?,
        _ => return Err(AOCError::DayOutOfRange(day))
    }
    Ok(())
}
