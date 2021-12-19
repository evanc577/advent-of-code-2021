use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt;
use std::io::Write;
use std::rc::{Rc, Weak};

use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

use crate::prelude::*;

pub struct Day18 {
    numbers: Vec<Rc<SnailfishNumber>>,
}

impl Day for Day18 {
    fn new(input: impl Iterator<Item = String>) -> Result<Self, AOCError> {
        let numbers: Vec<_> = input
            .map(|line| SnailfishNumber::from_str(&line))
            .collect::<Result<_, _>>()?;
        Ok(Self { numbers })
    }

    fn part_1(&self) -> Answer {
        Answer::None
    }

    fn part_2(&self) -> Answer {
        Answer::None
    }
}

fn explode(root: Rc<SnailfishNumber>) -> Option<()> {
    // Find leftmost pair nested within four pairs
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
    *num.sn_type.borrow_mut() = SnailfishNumberType::Regular(RefCell::new(0));
    Some(())
}

fn find_explode(num: Rc<SnailfishNumber>, depth: usize) -> Option<Rc<SnailfishNumber>> {
    if depth == 5 {
        return Some(num.parent.borrow().upgrade().unwrap());
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

enum Direction {
    Left,
    Right,
}

struct SnailfishNumber {
    sn_type: RefCell<SnailfishNumberType>,
    parent: RefCell<Weak<SnailfishNumber>>,
}

#[derive(Debug)]
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
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        write!(v1, "{}", self).unwrap();
        write!(v2, "{}", other).unwrap();
        v1.cmp(&v2) == Ordering::Equal
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
        let num_2 = SnailfishNumber::from_str(expected_str).unwrap();
        explode(num_1.clone());
        assert_eq!(num_1, num_2);

        let input_str = "[7,[6,[5,[4,[3,2]]]]]";
        let expected_str = "[7,[6,[5,[7,0]]]]";
        let num_1 = SnailfishNumber::from_str(input_str).unwrap();
        let num_2 = SnailfishNumber::from_str(expected_str).unwrap();
        explode(num_1.clone());
        assert_eq!(num_1, num_2);

        let input_str = "[[6,[5,[4,[3,2]]]],1]";
        let expected_str = "[[6,[5,[7,0]]],3]";
        let num_1 = SnailfishNumber::from_str(input_str).unwrap();
        let num_2 = SnailfishNumber::from_str(expected_str).unwrap();
        explode(num_1.clone());
        assert_eq!(num_1, num_2);

        let input_str = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let expected_str = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let num_1 = SnailfishNumber::from_str(input_str).unwrap();
        let num_2 = SnailfishNumber::from_str(expected_str).unwrap();
        explode(num_1.clone());
        assert_eq!(num_1, num_2);

        let input_str = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let expected_str = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";
        let num_1 = SnailfishNumber::from_str(input_str).unwrap();
        let num_2 = SnailfishNumber::from_str(expected_str).unwrap();
        explode(num_1.clone());
        assert_eq!(num_1, num_2);
    }

    static INPUT: &str = "[[[[1,1],[2,2]],[3,3]],[4,4]]";

    #[test]
    fn part_1() {
        let runner = Day18::new(INPUT.lines().map(|s| s.to_owned())).unwrap();
        assert_eq!(runner.part_1(), Answer::None);
    }

    // #[test]
    // fn part_2() {
    // let runner = Day18::new(INPUT.lines().map(|s| s.to_owned())).unwrap();
    // assert_eq!(runner.part_2(), Answer::None);
    // }
}
