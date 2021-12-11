use std::path::Path;

use aoc2021::prelude::*;
use ndarray::Array2;

pub fn run(input_path: impl AsRef<Path>) -> Result<(), AOCError> {
    let input = parse_input(input_path)?;

    part_01(&input);
    part_02(&input);

    Ok(())
}

fn parse_input(input_path: impl AsRef<Path>) -> Result<Array2<Option<Octopus>>, AOCError> {
    let lines: Vec<_> = read_input_lines(input_path)?.collect();
    let num_lines = lines.len();
    let line_len = lines.get(0).ok_or(AOCError::ParseError)?.len();

    let mut arr = Array2::from_elem((num_lines + 2, line_len + 2), None);

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().take(line_len).enumerate() {
            let energy = c.to_digit(10).ok_or(AOCError::ParseError)? as usize;
            arr[[i + 1, j + 1]] = Some(Octopus {
                energy,
                flashed: false,
            });
        }
    }
    Ok(arr)
}

fn part_01(input: &Array2<Option<Octopus>>) {
    println!("Part 1: {}", simulate(input, EndCondition::Step(100)));
}

fn part_02(input: &Array2<Option<Octopus>>) {
    println!("Part 2: {}", simulate(input, EndCondition::Synchronized));
}

fn simulate(input: &Array2<Option<Octopus>>, end: EndCondition) -> usize {
    let mut arr_cur = input.clone();
    let mut arr_next;
    let mut flash_count = 0;

    let end_step = match end {
        EndCondition::Step(s) => s,
        EndCondition::Synchronized => usize::MAX,
    };

    for step in 0..end_step {
        // Increment all energy levels
        arr_cur.iter_mut().for_each(|x| {
            if let Some(o) = x {
                o.energy += 1;
            }
        });
        arr_next = arr_cur.clone();

        // Compute flashes
        loop {
            let mut flashed = false;
            for (idx, window) in arr_cur.windows((3, 3)).into_iter().enumerate() {
                let i = idx / (arr_cur.ncols() - 2);
                let j = idx % (arr_cur.ncols() - 2);

                let octopus_cur = &window[[1, 1]];

                if let Some(o) = octopus_cur {
                    if o.energy > 9 && !o.flashed {
                        // Current octopus flashes
                        flashed = true;
                        flash_count += 1;

                        // Set flashed flag
                        arr_next[[i + 1, j + 1]] = Some(Octopus {
                            flashed: true,
                            ..*o
                        });

                        // Update adjacent octopi
                        for x in 0..3 {
                            for y in 0..3 {
                                let o = &mut arr_next[[i + x, j + y]];
                                if let Some(o) = o {
                                    o.energy += 1;
                                }
                            }
                        }
                    }
                }
            }

            // Swap arrays
            arr_cur = arr_next.clone();

            // If no octopus flashed, finish iteration
            if !flashed {
                break;
            }
        }

        if end == EndCondition::Synchronized && arr_cur.iter().flatten().all(|o| o.flashed) {
            return step + 1;
        }

        // Reset flashed octopi
        for o in arr_cur.iter_mut().flatten() {
            if o.energy > 9 {
                o.energy = 0;
            }
            o.flashed = false;
        }
    }

    match end {
        EndCondition::Step(_) => flash_count,
        EndCondition::Synchronized => usize::MAX,
    }
}

#[derive(Clone, Debug)]
struct Octopus {
    energy: usize,
    flashed: bool,
}

#[derive(PartialEq, Eq)]
enum EndCondition {
    Step(usize),
    Synchronized,
}
