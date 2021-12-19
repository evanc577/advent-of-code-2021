use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

use crate::prelude::*;

pub struct Day18 {
    input: Vec<String>,
}

impl Day for Day18 {
    fn new(input: impl Iterator<Item = String>) -> Result<Self, AOCError> {
        let input: Vec<_> = input.collect();
        Ok(Self { input })
    }

    fn part_1(&self) -> Answer {
        let numbers: Result<Vec<_>, _> = self
            .input
            .iter()
            .map(|line| SnailfishNumber::from_str(line))
            .collect::<Result<_, _>>();
        let numbers = match numbers {
            Ok(n) => n,
            Err(_) => return Answer::None,
        };

        let sum = match numbers.into_iter().reduce(add) {
            Some(n) => n,
            None => return Answer::None,
        };

        Answer::Integer(sum.magnitude())
    }

    fn part_2(&self) -> Answer {
        let numbers: Option<Vec<_>> = self
            .input
            .iter()
            .permutations(2)
            .map(|v| {
                let num_1 = SnailfishNumber::from_str(v[0]).ok()?;
                let num_2 = SnailfishNumber::from_str(v[1]).ok()?;
                Some((num_1, num_2))
            })
            .collect();
        let numbers = match numbers {
            Some(n) => n,
            None => return Answer::None,
        };

        numbers
            .into_iter()
            .map(|(num_1, num_2)| {
                let num = add(num_1, num_2);
                num.magnitude()
            })
            .max()
            .into()
    }
}

fn add(num_1: Rc<SnailfishNumber>, num_2: Rc<SnailfishNumber>) -> Rc<SnailfishNumber> {
    let root = Rc::new(SnailfishNumber {
        sn_type: RefCell::new(SnailfishNumberType::Pair(RefCell::new([
            num_1.clone(),
            num_2.clone(),
        ]))),
        parent: RefCell::new(Weak::new()),
    });
    *num_1.parent.borrow_mut() = Rc::downgrade(&root);
    *num_2.parent.borrow_mut() = Rc::downgrade(&root);

    reduce(root.clone());

    root
}

fn reduce(root: Rc<SnailfishNumber>) {
    loop {
        if explode(root.clone()).is_some() {
            continue;
        }
        if split(root.clone()).is_some() {
            continue;
        }
        break;
    }
}

fn explode(root: Rc<SnailfishNumber>) -> Option<()> {
    // Find leftmost pair to explode
    let num = find_explode(root, 0)?;

    match &*num.sn_type.borrow() {
        SnailfishNumberType::Regular(_) => panic!("Exploding number must be pair"),
        SnailfishNumberType::Pair(r) => {
            let r = r.borrow();

            // Add closest left and right regular numbers
            for (i, dir) in [Direction::Left, Direction::Right].into_iter().enumerate() {
                // Left
                let cur = match &*r.get(i).unwrap().sn_type.borrow() {
                    SnailfishNumberType::Regular(v) => *v.borrow(),
                    _ => panic!(),
                };

                if let Some(left) = find_closest(r.get(i).unwrap().clone(), dir) {
                    match &*left.sn_type.borrow() {
                        SnailfishNumberType::Regular(v) => {
                            *v.borrow_mut() += cur;
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    // Set number to 0
    *num.sn_type.borrow_mut() = SnailfishNumberType::Regular(RefCell::new(0));

    Some(())
}

// Find first number to explode
fn find_explode(num: Rc<SnailfishNumber>, depth: usize) -> Option<Rc<SnailfishNumber>> {
    if depth == 5 {
        return num.parent.borrow().upgrade();
    }

    match &*num.sn_type.borrow() {
        SnailfishNumberType::Regular(_) => return None,
        SnailfishNumberType::Pair(p) => {
            let p = p.borrow();
            if let left @ Some(_) = find_explode(p.get(0).unwrap().clone(), depth + 1) {
                return left;
            }
            if let right @ Some(_) = find_explode(p.get(1).unwrap().clone(), depth + 1) {
                return right;
            }
        }
    }

    None
}

// Find closest regular number in a specified direction
fn find_closest(num: Rc<SnailfishNumber>, dir: Direction) -> Option<Rc<SnailfishNumber>> {
    let parent = num.parent.borrow().upgrade()?;
    let (dir_1, dir_2) = match dir {
        Direction::Left => (0, 1),
        Direction::Right => (1, 0),
    };
    let ret = match &*parent.sn_type.borrow() {
        SnailfishNumberType::Regular(_) => panic!("Pair's parent is a regular?"),
        SnailfishNumberType::Pair(p) => {
            // Check if we came from left or right
            if Rc::ptr_eq(p.borrow().get(dir_1)?, &num) {
                // Came from left, recurse
                find_closest(parent.clone(), dir)
            } else {
                // From right, go to left child then descent to the right until reaching a regular leaf
                let mut num_2 = p.borrow().get(dir_1)?.clone();
                while let SnailfishNumberType::Pair(p) = &*num_2.clone().sn_type.borrow() {
                    num_2 = p.borrow().get(dir_2)?.clone();
                }
                let ret = match &*num_2.sn_type.borrow() {
                    SnailfishNumberType::Pair(_) => unreachable!(),
                    SnailfishNumberType::Regular(_) => Some(num_2.clone()),
                };
                ret
            }
        }
    };
    drop(parent);
    ret
}

fn split(root: Rc<SnailfishNumber>) -> Option<()> {
    // Find leftmost pair to split
    let num = find_split(root)?;

    let (left_v, right_v) = match &*num.sn_type.borrow() {
        SnailfishNumberType::Pair(_) => unreachable!(),
        SnailfishNumberType::Regular(v) => {
            let v = *v.borrow();
            let left = v / 2;
            let right = (v + 1) / 2;
            (left, right)
        }
    };

    // Set number to pair
    let left = SnailfishNumber {
        sn_type: RefCell::new(SnailfishNumberType::Regular(RefCell::new(left_v))),
        parent: RefCell::new(Rc::downgrade(&num)),
    };
    let right = SnailfishNumber {
        sn_type: RefCell::new(SnailfishNumberType::Regular(RefCell::new(right_v))),
        parent: RefCell::new(Rc::downgrade(&num)),
    };
    *num.sn_type.borrow_mut() =
        SnailfishNumberType::Pair(RefCell::new([Rc::new(left), Rc::new(right)]));

    Some(())
}

// Find first number to split
fn find_split(num: Rc<SnailfishNumber>) -> Option<Rc<SnailfishNumber>> {
    match &*num.sn_type.borrow() {
        SnailfishNumberType::Regular(v) if *v.borrow() >= 10 => Some(num.clone()),
        SnailfishNumberType::Pair(p) => {
            let p = p.borrow();
            if let left @ Some(_) = find_split(p.get(0).unwrap().clone()) {
                return left;
            }
            if let right @ Some(_) = find_split(p.get(1).unwrap().clone()) {
                return right;
            }
            None
        }
        _ => None,
    }
}

enum Direction {
    Left,
    Right,
}

struct SnailfishNumber {
    sn_type: RefCell<SnailfishNumberType>,
    parent: RefCell<Weak<SnailfishNumber>>,
}

#[derive(PartialEq, Debug)]
enum SnailfishNumberType {
    Regular(RefCell<usize>),
    Pair(RefCell<[Rc<SnailfishNumber>; 2]>),
}

impl fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &*self.sn_type.borrow() {
            SnailfishNumberType::Regular(value) => write!(f, "{}", *value.borrow()),
            SnailfishNumberType::Pair(children) => {
                write!(f, "[{},{}]", children.borrow()[0], children.borrow()[1])
            }
        }
    }
}

impl fmt::Debug for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl PartialEq for SnailfishNumber {
    fn eq(&self, other: &Self) -> bool {
        *self.sn_type.borrow() == *other.sn_type.borrow()
    }
}

impl SnailfishNumber {
    fn from_str(s: &str) -> Result<Rc<Self>, AOCError> {
        let num = Rc::new(snailfish_number(s).map_err(|_| AOCError::ParseError)?.1);

        // Update parent pointers
        match *num.sn_type.borrow() {
            SnailfishNumberType::Regular(_) => (),
            SnailfishNumberType::Pair(_) => update_parent_ref(num.clone()),
        }
        Ok(num)
    }

    fn magnitude(&self) -> usize {
        match &*self.sn_type.borrow() {
            SnailfishNumberType::Regular(v) => *v.borrow(),
            SnailfishNumberType::Pair(p) => {
                let p = p.borrow();
                3 * p.get(0).unwrap().magnitude() + 2 * p.get(1).unwrap().magnitude()
            }
        }
    }
}

fn update_parent_ref(num: Rc<SnailfishNumber>) {
    if let SnailfishNumberType::Pair(children) = &*num.sn_type.borrow() {
        for child in children.borrow().iter() {
            *child.parent.borrow_mut() = Rc::downgrade(&num);
            update_parent_ref(child.clone());
        }
    }
}

fn integer(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse())(input)
}

fn pair(input: &str) -> IResult<&str, (SnailfishNumber, SnailfishNumber)> {
    preceded(
        char('['),
        terminated(
            separated_pair(snailfish_number, char(','), snailfish_number),
            char(']'),
        ),
    )(input)
}

fn snailfish_number(input: &str) -> IResult<&str, SnailfishNumber> {
    alt((
        map(integer, |n| SnailfishNumber {
            sn_type: RefCell::new(SnailfishNumberType::Regular(RefCell::new(n))),
            parent: RefCell::new(Weak::new()),
        }),
        map(pair, |p| SnailfishNumber {
            sn_type: RefCell::new(SnailfishNumberType::Pair(RefCell::new([
                Rc::new(p.0),
                Rc::new(p.1),
            ]))),
            parent: RefCell::new(Weak::new()),
        }),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn explode_1() {
        let input_str = "[[[[[9,8],1],2],3],4]";
        let expected_str = "[[[[0,9],2],3],4]";
        let num_1 = SnailfishNumber::from_str(input_str).unwrap();
        let expected = SnailfishNumber::from_str(expected_str).unwrap();
        explode(num_1.clone());
        assert_eq!(num_1, expected);

        let input_str = "[7,[6,[5,[4,[3,2]]]]]";
        let expected_str = "[7,[6,[5,[7,0]]]]";
        let num_1 = SnailfishNumber::from_str(input_str).unwrap();
        let expected = SnailfishNumber::from_str(expected_str).unwrap();
        explode(num_1.clone());
        assert_eq!(num_1, expected);

        let input_str = "[[6,[5,[4,[3,2]]]],1]";
        let expected_str = "[[6,[5,[7,0]]],3]";
        let num_1 = SnailfishNumber::from_str(input_str).unwrap();
        let expected = SnailfishNumber::from_str(expected_str).unwrap();
        explode(num_1.clone());
        assert_eq!(num_1, expected);

        let input_str = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let expected_str = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let num_1 = SnailfishNumber::from_str(input_str).unwrap();
        let expected = SnailfishNumber::from_str(expected_str).unwrap();
        explode(num_1.clone());
        assert_eq!(num_1, expected);

        let input_str = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let expected_str = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";
        let num_1 = SnailfishNumber::from_str(input_str).unwrap();
        let expected = SnailfishNumber::from_str(expected_str).unwrap();
        explode(num_1.clone());
        assert_eq!(num_1, expected);
    }

    #[test]
    fn split_1() {
        let input_str = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
        let expected_str = "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]";
        let num_1 = SnailfishNumber::from_str(input_str).unwrap();
        let expected = SnailfishNumber::from_str(expected_str).unwrap();
        split(num_1.clone());
        assert_eq!(num_1, expected);

        let input_str = "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]";
        let expected_str = "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]";
        let num_1 = SnailfishNumber::from_str(input_str).unwrap();
        let expected = SnailfishNumber::from_str(expected_str).unwrap();
        split(num_1.clone());
        assert_eq!(num_1, expected);
    }

    #[test]
    fn add() {
        let num_1_str = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let num_2_str = "[1,1]";
        let expected_str = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let num_1 = SnailfishNumber::from_str(num_1_str).unwrap();
        let num_2 = SnailfishNumber::from_str(num_2_str).unwrap();
        let expected = SnailfishNumber::from_str(expected_str).unwrap();
        let num = super::add(num_1, num_2);
        assert_eq!(num, expected);
    }

    #[test]
    fn magnitude() {
        let num = SnailfishNumber::from_str("[9,1]").unwrap();
        assert_eq!(num.magnitude(), 29);

        let num = SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        assert_eq!(num.magnitude(), 1384);
    }

    static INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn part_1() {
        let runner = Day18::new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::Integer(4140));
    }

    #[test]
    fn part_2() {
        let runner = Day18::new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_2(), Answer::Integer(3993));
    }
}
