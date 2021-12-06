use std::collections::HashMap;

use std::path::Path;

use aoc2021::prelude::*;

pub fn run(input_path: impl AsRef<Path>) -> Result<(), AOCError> {
    let input = parse_input(input_path)?;

    part_01(&input[..]);
    part_02(&input[..]);

    Ok(())
}

fn parse_input(input_path: impl AsRef<Path>) -> Result<Vec<Line>, AOCError> {
    let input: Vec<_> = read_input_lines(input_path)?
        .filter_map(|line_str| {
            // Parse line in form "a,b -> x,y"
            if let points_str @ &[_, _] = line_str.split(" -> ").collect::<Vec<_>>().as_slice() {
                // Parse point in form "a,b"
                let points = points_str
                    .iter()
                    .filter_map(|point_str| {
                        if let &[x_str, y_str] = point_str.split(',').collect::<Vec<_>>().as_slice()
                        {
                            // Parse x/y coordinate
                            let x = x_str.parse().ok()?;
                            let y = y_str.parse().ok()?;

                            return Some(Point { x, y });
                        }

                        None
                    })
                    .collect::<Vec<_>>();

                if let &[p1, p2] = points.as_slice() {
                    return Some(Line { p1, p2 });
                }
            }

            None
        })
        .collect();

    Ok(input)
}

fn part_01(input: &[Line]) {
    let grid = Grid::with_lines(input, AllowDiagonals::No);
    println!("Part 1: {}", grid.overlaps());
}

fn part_02(input: &[Line]) {
    let grid = Grid::with_lines(input, AllowDiagonals::Yes);
    println!("Part 2: {}", grid.overlaps());
}

struct Grid(HashMap<Point, usize>);

impl Grid {
    fn with_lines(lines: &[Line], diagonals: AllowDiagonals) -> Self {
        let mut grid = Self(HashMap::new());
        for line in lines {
            grid.add_line(line, diagonals);
        }
        grid
    }

    fn add_line(&mut self, line: &Line, diagonals: AllowDiagonals) {
        for point in line.intermediate_points(diagonals) {
            self.0.entry(point).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    fn overlaps(&self) -> usize {
        self.0
            .iter()
            .fold(0, |acc, (_, v)| if *v > 1 { acc + 1 } else { acc })
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn intermediate_points(&self, diagonal_lines: AllowDiagonals) -> Vec<Point> {
        match self.axis() {
            LineAxis::Other => return vec![],
            LineAxis::Horizontal => {
                let start = self.p1.x.min(self.p2.x);
                let end = self.p1.x.max(self.p2.x);
                (start..=end)
                    .into_iter()
                    .map(|x| Point { x, y: self.p1.y })
                    .collect()
            }
            LineAxis::Vertical => {
                let start = self.p1.y.min(self.p2.y);
                let end = self.p1.y.max(self.p2.y);
                (start..=end)
                    .into_iter()
                    .map(|y| Point { x: self.p1.x, y })
                    .collect()
            }
            LineAxis::DiagonalFalling => {
                if diagonal_lines == AllowDiagonals::No {
                    return vec![];
                }
                let start_x = self.p1.x.min(self.p2.x);
                let end_x = self.p1.x.max(self.p2.x);
                let start_y = self.p1.y.min(self.p2.y);
                (0..=(end_x - start_x))
                    .into_iter()
                    .map(|offset| Point {
                        x: start_x + offset,
                        y: start_y + offset,
                    })
                    .collect()
            }
            LineAxis::DiagonalRising => {
                if diagonal_lines == AllowDiagonals::No {
                    return vec![];
                }
                let start_x = self.p1.x.min(self.p2.x);
                let end_x = self.p1.x.max(self.p2.x);
                let start_y = self.p1.y.max(self.p2.y);
                (0..=(end_x - start_x))
                    .into_iter()
                    .map(|offset| Point {
                        x: start_x + offset,
                        y: start_y - offset,
                    })
                    .collect()
            }
        }
    }

    fn axis(&self) -> LineAxis {
        if self.p1.x == self.p2.x {
            LineAxis::Vertical
        } else if self.p1.y == self.p2.y {
            LineAxis::Horizontal
        } else if self.p1.x - self.p2.x == self.p1.y - self.p2.y {
            LineAxis::DiagonalFalling
        } else if self.p1.x - self.p2.x == -1 * (self.p1.y - self.p2.y) {
            LineAxis::DiagonalRising
        } else {
            LineAxis::Other
        }
    }
}

enum LineAxis {
    Horizontal,
    Vertical,
    DiagonalFalling,
    DiagonalRising,
    Other,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AllowDiagonals {
    No,
    Yes,
}
