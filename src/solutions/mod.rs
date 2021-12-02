use std::path::Path;

use aoc2021::prelude::AOCError;

mod day_01;
mod day_02;

pub fn dispatch(day: usize, input_path: impl AsRef<Path>) -> Result<(), AOCError> {
    match day {
        1 => day_01::run(input_path)?,
        2 => day_02::run(input_path)?,
        _ => return Err(AOCError::DayOutOfRange(day))
    }
    Ok(())
}
