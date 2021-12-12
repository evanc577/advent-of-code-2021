use std::collections::HashMap;
use std::str::FromStr;

use crate::prelude::*;
use rayon::prelude::*;

pub struct Day12 {
    input: Vec<Path>,
}

pub fn new(input: impl Iterator<Item = String>) -> Result<Box<dyn Day>, AOCError> {
    let parsed = input.map(|line| line.parse()).collect::<Result<_, _>>()?;
    Ok(Box::new(Day12 { input: parsed }))
}

impl Day for Day12 {
    fn part_1(&self) -> Option<usize> {
        let adj = generate_adjacency(&self.input);
        do_dfs(adj, |_| false)
    }

    fn part_2(&self) -> Option<usize> {
        let adj = generate_adjacency(&self.input);
        do_dfs(adj, |v| {
            !v.iter()
                .any(|(&c, &x)| matches!(c, Cave::Small(_)) && x > 1)
        })
    }
}

fn do_dfs<F>(adj: HashMap<&Cave, Vec<&Cave>>, small_criteria: F) -> Option<usize>
where
    F: Sync + Fn(&HashMap<&Cave, usize>) -> bool,
{
    let mut stacks: Vec<Stack> = adj
        .get(&Cave::Start)?
        .iter()
        .map(|&c| Stack::new(c))
        .collect();

    let count = stacks
        .par_iter_mut()
        .map(|stack| {
            // DFS
            let mut count = 0;
            while let Some((cave, mut visited)) = stack.pop() {
                if visited.get(cave).is_none()
                    || matches!(cave, Cave::Big(_))
                    || matches!(cave, Cave::Small(_)) && small_criteria(&visited)
                {
                    visited.entry(cave).and_modify(|v| *v += 1).or_insert(1);
                    if let Some(next_caves) = adj.get(cave) {
                        for next_cave in next_caves {
                            match next_cave {
                                Cave::Start => (),
                                Cave::End => count += 1,
                                Cave::Big(_) | Cave::Small(_) => {
                                    stack.push((next_cave, visited.clone()))
                                }
                            }
                        }
                    }
                }
            }
            count
        })
        .sum();

    Some(count)
}

struct Path(Cave, Cave);

impl FromStr for Path {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [a, b] = *s.split('-').collect::<Vec<_>>() {
            Ok(Self(a.parse()?, b.parse()?))
        } else {
            Err(AOCError::ParseError)
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

impl FromStr for Cave {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "start" {
            Ok(Self::Start)
        } else if s == "end" {
            Ok(Self::End)
        } else {
            match s.chars().all(|c| c.is_lowercase()) {
                true => Ok(Self::Small(s.into())),
                false => Ok(Self::Big(s.into())),
            }
        }
    }
}

type StackValue<'a> = (&'a Cave, HashMap<&'a Cave, usize>);

struct Stack<'a>(Vec<StackValue<'a>>);

impl<'a> Stack<'a> {
    fn new(cave: &'a Cave) -> Self {
        Self(vec![(cave, HashMap::new())])
    }

    fn push(&mut self, value: StackValue<'a>) {
        self.0.push(value);
    }

    fn pop(&mut self) -> Option<StackValue<'a>> {
        self.0.pop()
    }
}

fn generate_adjacency(paths: &[Path]) -> HashMap<&Cave, Vec<&Cave>> {
    let mut map = HashMap::new();
    for path in paths {
        if path.1 != Cave::Start {
            map.entry(&path.0).or_insert_with(Vec::new).push(&path.1);
        }
        if path.0 != Cave::Start {
            map.entry(&path.1).or_insert_with(Vec::new).push(&path.0);
        }
    }
    map
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    #[test]
    fn part_1() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Some(10));
    }

    #[test]
    fn part_2() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Some(36));
    }
}
