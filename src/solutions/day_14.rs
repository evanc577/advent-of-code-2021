use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use itertools::Itertools;

use crate::prelude::*;

pub struct Day14 {
    template: Polymer,
    insertion_rules: InsertionRules,
}

pub fn new(mut input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
    let template = input.by_ref().next().ok_or(AOCError::ParseError)?.parse()?;
    input.next();
    let insertion_rules = input
        .map(InsertionRule::try_from)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .collect();
    Ok(Box::new(Day14 {
        template,
        insertion_rules,
    }))
}

impl Day for Day14 {
    fn part_1(&self) -> Answer {
        Answer::Integer(simulate(&self.template, &self.insertion_rules, 10))
    }

    fn part_2(&self) -> Answer {
        // Answer::Integer(simulate(&self.template, &self.insertion_rules, 40))
        Answer::None
    }
}

fn simulate(template: &Polymer, insertion_rules: &InsertionRules, steps: usize) -> usize {
    let mut polymer = template.clone();
    for _ in 0..steps {
        let inserted: HashMap<_, _> = polymer
            .iter()
            .tuple_windows::<(_, _)>()
            .enumerate()
            .filter_map(|(i, (&a, &b))| insertion_rules.get(&(a, b)).map(|insert| (i, insert)))
            .collect();

        let mut next_polymer = Polymer::new();
        for (i, &element) in polymer.iter().enumerate() {
            next_polymer.push(element);
            if let Some(&&insert) = inserted.get(&i) {
                next_polymer.push(insert);
            }
        }

        polymer = next_polymer;
    }

    polymer.score()
}

#[derive(Debug)]
struct Polymer(Vec<char>);

impl Polymer {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn score(&self) -> usize {
        let mut map = HashMap::new();
        for c in self.iter() {
            *map.entry(c).or_insert(0) += 1;
        }

        let max = if let Some((_, n)) = map.iter().max_by(|(_, a), (_, b)| a.cmp(b)) {
            n
        } else {
            return 0;
        };

        let min = if let Some((_, n)) = map.iter().min_by(|(_, a), (_, b)| a.cmp(b)) {
            n
        } else {
            return 0;
        };

        max - min
    }
}

impl Clone for Polymer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl FromStr for Polymer {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Polymer(s.chars().collect()))
    }
}

impl Deref for Polymer {
    type Target = Vec<char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Polymer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct InsertionRule {
    pair: (char, char),
    insertion: char,
}

impl TryFrom<String> for InsertionRule {
    type Error = AOCError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if let Some((pair_str, insertion_str)) = s.split(" -> ").next_tuple() {
            let pair = if let Some(pair) = pair_str.chars().next_tuple() {
                pair
            } else {
                return Err(AOCError::ParseError);
            };
            let insertion = if let Some(c) = insertion_str.chars().next() {
                c
            } else {
                return Err(AOCError::ParseError);
            };
            return Ok(Self { pair, insertion });
        }
        Err(AOCError::ParseError)
    }
}

#[derive(Debug)]
struct InsertionRules(HashMap<(char, char), char>);

impl FromIterator<InsertionRule> for InsertionRules {
    fn from_iter<T: IntoIterator<Item = InsertionRule>>(iter: T) -> Self {
        let mut map = HashMap::new();
        for x in iter {
            map.insert(x.pair, x.insertion);
        }
        Self(map)
    }
}

impl Deref for InsertionRules {
    type Target = HashMap<(char, char), char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn part_1() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(1588));
    }

    #[test]
    fn part_2() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::None);
    }
}
