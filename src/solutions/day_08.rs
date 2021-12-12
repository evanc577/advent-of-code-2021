use std::collections::HashSet;

use crate::prelude::*;

pub struct Day08 {
    input: Vec<Entry>,
}

pub fn new(input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
    let parsed: Vec<_> = input
        .map(|s| {
            let (signals, outputs) =
                if let [signals_str, outputs_str] = *s.split(" | ").take(2).collect::<Vec<_>>() {
                    // Parse signals
                    let signals = signals_str
                        .split(' ')
                        .map(|s| s.chars().map(WireSegment::from).collect::<Pattern>())
                        .collect::<Vec<_>>();
                    let outputs = outputs_str
                        .split(' ')
                        .map(|s| s.chars().map(WireSegment::from).collect::<Pattern>())
                        .collect::<Vec<_>>();
                    (signals, outputs)
                } else {
                    (vec![], vec![])
                };
            Entry { signals, outputs }
        })
        .collect();
    Ok(Box::new(Day08 { input: parsed }))
}

impl Day for Day08 {
    fn part_1(&self) -> Option<usize> {
        let count: usize = self
            .input
            .iter()
            .map(|e| {
                e.outputs
                    .iter()
                    .filter(|p| matches!(p.0.len(), 2 | 3 | 4 | 7))
                    .count()
            })
            .sum();
        Some(count)
    }

    fn part_2(&self) -> Option<usize> {
        // Sum results of all lines
        let sum: usize = self
            .input
            .iter()
            .map(|entry| {
                let mut digits = Vec::new();
                digits.resize_with(10, || None);
                let mut remaining = vec![];

                // Find easy digits (1, 4, 7, 8)
                for pattern in entry.signals.iter() {
                    match pattern.0.len() {
                        2 => digits[1] = Some(pattern),
                        3 => digits[7] = Some(pattern),
                        4 => digits[4] = Some(pattern),
                        7 => digits[8] = Some(pattern),
                        _ => remaining.push(pattern),
                    }
                }

                if let (Some(segs_1), Some(segs_4)) = (digits[1], digits[4]) {
                    // Segments that are in the "elbow" of 4
                    let diff_4: HashSet<_> = segs_4.0.difference(&segs_1.0).copied().collect();

                    // Find hard digits
                    for pattern in remaining {
                        match pattern.0.len() {
                            5 => {
                                if segs_1.0.is_subset(&pattern.0) {
                                    digits[3] = Some(pattern);
                                } else if diff_4.is_subset(&pattern.0) {
                                    digits[5] = Some(pattern);
                                } else {
                                    digits[2] = Some(pattern);
                                }
                            }
                            6 => {
                                if segs_4.0.is_subset(&pattern.0) {
                                    digits[9] = Some(pattern);
                                } else if diff_4.is_subset(&pattern.0) {
                                    digits[6] = Some(pattern);
                                } else {
                                    digits[0] = Some(pattern);
                                }
                            }
                            _ => return 0,
                        }
                    }

                    // Calculate output values
                    let num = entry.outputs.iter().try_fold(0, |acc, x| {
                        let found = digits.iter().enumerate().find(|(_, s)| {
                            if let Some(s) = s {
                                x.0 == s.0
                            } else {
                                false
                            }
                        });
                        if let Some((d, _)) = found {
                            Some(10 * acc + d)
                        } else {
                            None
                        }
                    });

                    num.unwrap_or(0)
                } else {
                    0
                }
            })
            .sum();

        Some(sum)
    }
}

#[derive(Debug)]
struct Entry {
    signals: Vec<Pattern>,
    outputs: Vec<Pattern>,
}

#[derive(Clone, Debug)]
struct Pattern(HashSet<WireSegment>);

impl FromIterator<WireSegment> for Pattern {
    fn from_iter<T: IntoIterator<Item = WireSegment>>(iter: T) -> Self {
        let mut c = Self(HashSet::new());
        for i in iter {
            c.0.insert(i);
        }
        c
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum WireSegment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<char> for WireSegment {
    fn from(c: char) -> Self {
        match c {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            _ => Self::A,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part_1() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Some(26));
    }

    #[test]
    fn part_2() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Some(61229));
    }
}
