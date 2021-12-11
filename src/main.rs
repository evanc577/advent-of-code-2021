use std::ffi::OsString;
use std::process::exit;

use aoc2021::prelude::*;
use clap::{App, Arg};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), AOCError> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("day")
                .help("Advent of code day")
                .required(true)
                .index(1),
        )
        .arg(Arg::with_name("input").help("Input file to use").index(2))
        .get_matches();

    let day = {
        let day_str = matches.value_of_os("day").unwrap();
        if day_str == "all" {
            DayNum::All
        } else {
            match day_str.to_string_lossy().parse::<usize>() {
                Ok(d) => {
                    let input_path = matches
                        .value_of_os("input").map(|s| s.to_owned())
                        .unwrap_or_else(|| OsString::from(format!("input/day_{:02}.txt", d)));
                    DayNum::One(d, input_path)
                }
                Err(_) => return Err(AOCError::BadDay(day_str.to_owned())),
            }
        }
    };

    run_solutions(day)
}
