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
    let mut grid: HashMap<Point, usize> = HashMap::new();
    for line in input {
        for point in line.intermediate_points(false) {
            match grid.get_mut(&point) {
                Some(p) => *p += 1,
                None => {
                    grid.insert(point, 1);
                    ()
                }
            }
        }
    }

    let overlaps = grid
        .iter()
        .fold(0, |acc, (_, v)| if *v > 1 { acc + 1 } else { acc });

    println!("Part 1: {}", overlaps);
}

fn part_02(input: &[Line]) {
    let mut grid: HashMap<Point, usize> = HashMap::new();
    for line in input {
        for point in line.intermediate_points(true) {
            match grid.get_mut(&point) {
                Some(p) => *p += 1,
                None => {
                    grid.insert(point, 1);
                    ()
                }
            }
        }
    }

    let overlaps = grid
        .iter()
        .fold(0, |acc, (_, v)| if *v > 1 { acc + 1 } else { acc });

    println!("Part 2: {}", overlaps);
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn intermediate_points(&self, diagonal_lines: bool) -> Vec<Point> {
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
                if !diagonal_lines {
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
                if !diagonal_lines {
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
