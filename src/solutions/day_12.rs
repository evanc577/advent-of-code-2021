use std::collections::HashMap;
use std::str::FromStr;

use crate::prelude::*;

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
        Some(do_dfs(adj, &|_| false))
    }

    fn part_2(&self) -> Option<usize> {
        let adj = generate_adjacency(&self.input);
        Some(do_dfs(adj, &|v| {
            !v.iter()
                .any(|(&c, &x)| c.class == CaveClass::Small && x > 1)
        }))
    }
}

fn do_dfs(
    adj: HashMap<&Cave, Vec<&Cave>>,
    small_criteria: &dyn Fn(&HashMap<&Cave, usize>) -> bool,
) -> usize {
    // DFS
    let start = Cave {
        id: "start".into(),
        class: CaveClass::Start,
    };
    let mut stack: Vec<(_, HashMap<&Cave, usize>)> = vec![(&start, HashMap::new())];
    let mut count = 0;

    while let Some((cave, mut visited)) = stack.pop() {
        if visited.get(&cave).is_none()
            || cave.class == CaveClass::Big
            || cave.class == CaveClass::Small && small_criteria(&visited)
        {
            visited.entry(cave).and_modify(|v| *v += 1).or_insert(1);
            if let Some(next_caves) = adj.get(cave) {
                for next_cave in next_caves {
                    match next_cave.class {
                        CaveClass::End => count += 1,
                        CaveClass::Start => (),
                        CaveClass::Big | CaveClass::Small => {
                            stack.push((next_cave, visited.clone()))
                        }
                    }
                }
            }
        }
    }
    count
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
struct Cave {
    id: String,
    class: CaveClass,
}

impl FromStr for Cave {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.into();
        if s == "start" {
            Ok(Self {
                id,
                class: CaveClass::Start,
            })
        } else if s == "end" {
            Ok(Self {
                id,
                class: CaveClass::End,
            })
        } else {
            let class = match s.chars().all(|c| c.is_lowercase()) {
                true => CaveClass::Small,
                false => CaveClass::Big,
            };
            Ok(Self { id, class })
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum CaveClass {
    Start,
    End,
    Big,
    Small,
}

fn generate_adjacency(paths: &[Path]) -> HashMap<&Cave, Vec<&Cave>> {
    let mut map = HashMap::new();
    for path in paths {
        map.entry(&path.0).or_insert_with(Vec::new).push(&path.1);
        map.entry(&path.1).or_insert_with(Vec::new).push(&path.0);
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
    fn test_part_1() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Some(10));
    }

    #[test]
    fn test_part_2() {
        let runner = new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Some(36));
    }
}
