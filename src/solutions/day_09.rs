use crate::prelude::*;
use ndarray::Array2;

pub struct Day09 {
    input: Array2<usize>,
}

pub fn new(input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
    let input: Vec<_> = input.collect();
    let line_len = input.get(0).ok_or(AOCError::ParseError)?.len();
    let mut arr = Array2::<usize>::from_elem((input.len() + 2, line_len + 2), usize::MAX);
    for i in 0..input.len() {
        for j in 0..line_len {
            let chars: Vec<_> = input[i].chars().map(|c| c.to_string()).collect();
            arr[[i + 1, j + 1]] = chars[j].parse().unwrap_or(0);
        }
    }
    Ok(Box::new(Day09 { input: arr }))
}

impl Day for Day09 {
    fn part_1(&self) -> Option<usize> {
        let sum: usize = low_points(&self.input)
            .iter()
            .map(|(i, j)| &self.input[[*i, *j]] + 1)
            .sum();
        Some(sum)
    }

    fn part_2(&self) -> Option<usize> {
        let mut basins: Vec<_> = low_points(&self.input)
            .iter()
            .map(|point| basin_size(&self.input, *point))
            .collect();
        basins[..].select_nth_unstable_by(3, |a, b| b.cmp(a)); // reverse sort
        let product: usize = basins.iter().take(3).product();
        Some(product)
    }
}

fn low_points(input: &Array2<usize>) -> Vec<(usize, usize)> {
    input
        .windows([3, 3])
        .into_iter()
        .enumerate()
        .filter_map(|(i, window)| {
            let min = window[[0, 1]]
                .min(window[[1, 0]])
                .min(window[[2, 1]])
                .min(window[[1, 2]]);
            if window[[1, 1]] < min {
                Some((i / (input.ncols() - 2) + 1, i % (input.ncols() - 2) + 1))
            } else {
                None
            }
        })
        .collect()
}

fn basin_size(input: &Array2<usize>, point: (usize, usize)) -> usize {
    // DFS
    let mut stack = vec![point];
    let mut visited = Array2::<bool>::default(input.dim());
    let mut count = 0;
    while let Some((i, j)) = stack.pop() {
        if !visited[[i, j]] {
            visited[[i, j]] = true;
            count += 1;
            // Up
            if i > 1 && input[[i - 1, j]] < 9 {
                stack.push((i - 1, j));
            }
            // Left
            if j > 1 && input[[i, j - 1]] < 9 {
                stack.push((i, j - 1));
            }
            // Down
            if i < input.nrows() - 1 && input[[i + 1, j]] < 9 {
                stack.push((i + 1, j));
            }
            // Right
            if j < input.ncols() - 1 && input[[i, j + 1]] < 9 {
                stack.push((i, j + 1));
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part_1() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Some(15));
    }

    #[test]
    fn part_2() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Some(1134));
    }
}
