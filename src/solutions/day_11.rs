use crate::prelude::*;
use ndarray::Array2;

pub struct Day11 {
    input: Array2<Option<Octopus>>,
}

pub fn new(input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
    let lines: Vec<_> = input.collect();
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
    Ok(Box::new(Day11 { input: arr }))
}

impl Day for Day11 {
    fn part_1(&self) -> Option<usize> {
        Some(simulate(&self.input, EndCondition::Step(100)))
    }

    fn part_2(&self) -> Option<usize> {
        Some(simulate(&self.input, EndCondition::Synchronized))
    }
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

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn part_1() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Some(1656));
    }

    #[test]
    fn part_2() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Some(195));
    }
}
