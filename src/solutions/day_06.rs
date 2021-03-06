use crate::prelude::*;

pub struct Day06 {
    input: Vec<usize>,
}

impl Day for Day06 {
    fn new(mut input: impl Iterator<Item = String>) -> Result<Self, AOCError> {
        let parsed: Vec<_> = input
            .next()
            .ok_or(AOCError::ParseError)?
            .split(',')
            .map(|s| s.parse().map_err(|e| AOCError::ParseIntError(e, s.into())))
            .collect::<Result<_, _>>()?;
        Ok(Day06 { input: parsed })
    }

    fn part_1(&self) -> Answer {
        Answer::Integer(simulate(&self.input, 80))
    }

    fn part_2(&self) -> Answer {
        Answer::Integer(simulate(&self.input, 256))
    }
}

fn simulate(input: &[usize], num_days: usize) -> usize {
    const NEW_FISH_TIMER: usize = 8;
    const RESET_FISH_TIMER: usize = 6;

    let mut fish: Vec<usize> = vec![0; NEW_FISH_TIMER + 1];
    for &x in input {
        fish[x] += 1;
    }

    for _ in 0..num_days {
        let num_new_fish = fish[0];

        // Decrease all fish timers by 1
        fish.rotate_left(1);

        // Add new fish
        fish[NEW_FISH_TIMER] = num_new_fish;
        fish[RESET_FISH_TIMER] += num_new_fish
    }

    fish.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part_1() {
        let runner = Day06::new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(5934));
    }

    #[test]
    fn part_2() {
        let runner = Day06::new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(26984457539));
    }
}
