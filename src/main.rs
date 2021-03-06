use std::ffi::{OsStr, OsString};
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
        .arg(Arg::with_name("day").help("Advent of code day").index(1))
        .arg(Arg::with_name("input").help("Input file to use").index(2))
        .get_matches();

    let day = {
        let day_str = matches
            .value_of_os("day")
            .unwrap_or_else(|| OsStr::new("all"));
        if day_str == "all" {
            DayNum::All
        } else {
            match day_str.to_string_lossy().parse::<usize>() {
                Ok(d) => {
                    let input_path = matches
                        .value_of_os("input")
                        .map(|s| s.to_owned())
                        .unwrap_or_else(|| OsString::from(format!("input/day_{:02}.txt", d)));
                    DayNum::One(d, input_path)
                }
                Err(_) => return Err(AOCError::BadDay(day_str.to_owned())),
            }
        }
    };

    for (day, solution) in run_solutions(day)? {
        for (part, part_solution) in solution.iter().enumerate() {
            let solution_text = match part_solution {
                Answer::Integer(x) => x.to_string(),
                Answer::None => "No solution".into(),
                Answer::Printable(x) => match String::from_utf8(x.to_vec()) {
                    Ok(s) => "\n".to_owned() + &s,
                    Err(_) => "Invalid UTF8".into(),
                },
                Answer::Error(s) => format!("Error: {}", s),
            };

            println!("Day {:2} Part {}: {}", day, part + 1, solution_text);
        }
    }

    Ok(())
}
