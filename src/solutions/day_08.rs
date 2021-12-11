use std::cmp::Ordering;

use crate::prelude::*;
use itertools::Itertools;

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
        use WireSegment::*;
        const ALL_WIRESEGMENTS: [WireSegment; 7] = [A, B, C, D, E, F, G];

        // Sum results of all lines
        let sum: usize = self
            .input
            .iter()
            .map(|entry| {
                let patterns: Vec<_> = entry
                    .signals
                    .iter()
                    .chain(entry.outputs.iter())
                    .cloned()
                    .collect();

                // Loop over all possible permutations
                'permutation_loop: for permutation in
                    ALL_WIRESEGMENTS.iter().permutations(ALL_WIRESEGMENTS.len())
                {
                    // Check if permutation results in a valid mapping
                    for pattern in &patterns {
                        let mapped_pattern = map_pattern(&permutation[..], &pattern.0);
                        if segments_to_digit(&mapped_pattern[..]).is_none() {
                            continue 'permutation_loop;
                        }
                    }

                    // compute output digits
                    let num = entry
                        .outputs
                        .iter()
                        .filter_map(|p| {
                            let mapped_pattern = map_pattern(&permutation[..], &p.0);
                            segments_to_digit(&mapped_pattern)
                        })
                        .reduce(|acc, x| 10 * acc + x)
                        .unwrap_or(0);
                    return num;
                }

                // No mapping found? Just return 0
                0
            })
            .sum();
        Some(sum)
    }
}

fn map_pattern(permutation: &[&WireSegment], pattern: &[WireSegment]) -> Vec<WireSegment> {
    pattern
        .iter()
        .map(|x| permutation[x.to_idx()])
        .cloned()
        .collect()
}

fn segments_to_digit(segments: &[WireSegment]) -> Option<usize> {
    #[rustfmt::skip]
    const SEGMENT_DIGIT_TABLE: [[bool; 7]; 10] = [
        // a      b      c      d      e      f      g
        [true , true , true , false, true , true , true ], // 0
        [false, false, true , false, false, true , false], // 1
        [true , false, true , true , true , false, true ], // 2
        [true , false, true , true , false, true , true ], // 3
        [false, true , true , true , false, true , false], // 4
        [true , true , false, true , false, true , true ], // 5
        [true , true , false, true , true , true , true ], // 6
        [true , false, true , false, false, true , false], // 7
        [true , true , true , true , true , true , true ], // 8
        [true , true , true , true , false, true , true ], // 9
    ];

    let mut input_segs: [bool; 7] = [false; 7];
    for seg in segments {
        input_segs[seg.to_idx()] = true;
    }

    for (i, digit_segs) in SEGMENT_DIGIT_TABLE.iter().enumerate() {
        if digit_segs
            .iter()
            .zip(input_segs.iter())
            .map(|(a, b)| a.cmp(b))
            .all(|c| c == Ordering::Equal)
            && digit_segs.len() == input_segs.len()
        {
            return Some(i);
        }
    }

    None
}

#[derive(Debug)]
struct Entry {
    signals: Vec<Pattern>,
    outputs: Vec<Pattern>,
}

#[derive(Clone, Debug)]
struct Pattern(Vec<WireSegment>);

impl FromIterator<WireSegment> for Pattern {
    fn from_iter<T: IntoIterator<Item = WireSegment>>(iter: T) -> Self {
        let mut c = Self(Vec::new());
        for i in iter {
            c.0.push(i);
        }
        c
    }
}

#[derive(Clone, Copy, Debug)]
enum WireSegment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl WireSegment {
    fn to_idx(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
            Self::E => 4,
            Self::F => 5,
            Self::G => 6,
        }
    }
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
