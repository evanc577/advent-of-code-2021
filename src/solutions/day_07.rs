use crate::prelude::*;

pub struct Day07 {
    input: Vec<usize>,
}

pub fn new(mut input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
    let parsed: Vec<_> = input
        .next()
        .ok_or(AOCError::ParseError)?
        .split(',')
        .map(|s| s.parse().map_err(|e| AOCError::ParseIntError(e, s.into())))
        .collect::<Result<_, _>>()?;
    Ok(Box::new(Day07 { input: parsed }))
}

impl Day for Day07 {
    fn part_1(&self) -> Option<usize> {
        let fuel = calculate(&self.input, abs_diff);
        Some(fuel)
    }

    fn part_2(&self) -> Option<usize> {
        let fuel = calculate(&self.input, |a, b| triangular(abs_diff(a, b)));
        Some(fuel)
    }
}

fn calculate<F>(input: &[usize], f: F) -> usize
where
    F: Fn(usize, usize) -> usize,
{
    let max = *input.iter().max().unwrap_or(&0);
    let min = *input.iter().min().unwrap_or(&0);

    (min..=max)
        .map(|l| input.iter().fold(0, |acc, x| acc + f(l, *x)))
        .min()
        .unwrap_or(0)
}

fn abs_diff(a: usize, b: usize) -> usize {
    a.saturating_sub(b).max(b.saturating_sub(a))
}

fn triangular(n: usize) -> usize {
    (n * (n + 1)) / 2
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part_1() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Some(37));
    }

    #[test]
    fn part_2() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Some(168));
    }
}
