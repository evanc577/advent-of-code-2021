use rustc_hash::FxHashMap as HashMap;
use std::ops::Deref;
use std::str::FromStr;

use itertools::{Itertools, MinMaxResult};

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
        simulate(&self.template, &self.insertion_rules, 10).into()
    }

    fn part_2(&self) -> Answer {
        simulate(&self.template, &self.insertion_rules, 40).into()
    }
}

fn simulate(template: &Polymer, insertion_rules: &InsertionRules, steps: usize) -> Option<usize> {
    // Character pair counts
    let mut pairs = HashMap::default();
    for (&a, &b) in template.iter().tuple_windows::<(_, _)>() {
        *pairs.entry((a, b)).or_insert(0) += 1;
    }

    // Individual character counts
    let mut char_counts = HashMap::default();
    for c in template.iter() {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut next_pairs = pairs.clone();

        // Iterate over all pairs
        for (pair, &count) in pairs.iter() {
            // Check insertion rules
            if let Some(c) = insertion_rules.get(pair) {
                // Remove current pair
                *next_pairs.entry(*pair).or_insert(0) -= count;
                // Add new pairs
                *next_pairs.entry((pair.0, *c)).or_insert(0) += count;
                *next_pairs.entry((*c, pair.1)).or_insert(0) += count;
                // Increment individual character counts
                *char_counts.entry(c).or_insert(0) += count;
            }
        }

        pairs = next_pairs;
    }

    match char_counts.values().minmax() {
        MinMaxResult::MinMax(min, max) => Some(max - min),
        MinMaxResult::OneElement(_) => Some(0),
        MinMaxResult::NoElements => None,
    }
}

#[derive(Clone, Debug)]
struct Polymer(Vec<char>);

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
        let mut map = HashMap::default();
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

    static INPUT: &str = "NNCB

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
        assert_eq!(runner.part_2(), Answer::Integer(2188189693529));
    }
}
