use rustc_hash::FxHashMap as HashMap;

use crate::prelude::*;

pub struct Day05 {
    input: Vec<Line>,
}

#[allow(clippy::match_ref_pats)]
pub fn new(input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
    let parsed: Vec<_> = input
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

    Ok(Box::new(Day05 { input: parsed }))
}

impl Day for Day05 {
    fn part_1(&self) -> Answer {
        let grid = Grid::with_lines(&self.input, AllowDiagonals::No);
        Answer::Integer(grid.overlaps())
    }

    fn part_2(&self) -> Answer {
        let grid = Grid::with_lines(&self.input, AllowDiagonals::Yes);
        Answer::Integer(grid.overlaps())
    }
}

struct Grid(HashMap<Point, usize>);

impl Grid {
    fn with_lines(lines: &[Line], diagonals: AllowDiagonals) -> Self {
        let mut grid = Self(HashMap::default());
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

#[derive(Clone, Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn intermediate_points(&self, diagonal: AllowDiagonals) -> Vec<Point> {
        let vector = self.to_vector();
        if vector.x != 0 && vector.y != 0 && vector.x.abs() != vector.y.abs() {
            return vec![];
        }

        let step_x = sign(vector.x);
        let step_y = sign(vector.y);

        // Diagonal line
        if diagonal == AllowDiagonals::No && step_x * step_y != 0 {
            return vec![];
        }

        (0..vector.num_intermediate_points())
            .map(|offset| Point {
                x: self.p1.x + step_x * offset,
                y: self.p1.y + step_y * offset,
            })
            .collect()
    }

    fn to_vector(&self) -> Point {
        self.p2 - self.p1
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn num_intermediate_points(&self) -> isize {
        self.x.abs().max(self.y.abs()) + 1
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AllowDiagonals {
    No,
    Yes,
}

fn sign(n: isize) -> isize {
    match n {
        n if n > 0 => 1,
        0 => 0,
        _ => -1,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part_1() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(5));
    }

    #[test]
    fn part_2() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(12));
    }
}
