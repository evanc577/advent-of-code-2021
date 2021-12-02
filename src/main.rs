mod solutions;

use std::process::exit;

use aoc2021::prelude::*;
use clap::{App, Arg};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), AOCError>{
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
        .arg(
            Arg::with_name("input")
                .help("Input file to use")
                .required(true)
                .index(2),
        )
        .get_matches();

    let day = {
        let day_str = matches.value_of_os("day").unwrap();
        match day_str.to_string_lossy().parse::<usize>() {
            Ok(d) => d,
            Err(_) => return Err(AOCError::BadDay(day_str.to_owned())),
        }
    };

    solutions::dispatch(day, matches.value_of_os("input").unwrap())
}

