pub mod solutions;

pub mod prelude {
    use std::collections::BTreeMap;
    use std::error::Error;
    use std::ffi::OsString;
    use std::fmt;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::num::ParseIntError;
    use std::path::Path;

    // Functions

    pub fn read_input_lines(
        path: impl AsRef<Path>,
    ) -> Result<impl Iterator<Item = String>, AOCError> {
        let file = File::open(path).map_err(AOCError::BadInputFile)?;
        let lines = io::BufReader::new(file)
            .lines()
            .map(|l| l.map_err(AOCError::BadInputFile))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter();
        Ok(lines)
    }

    pub fn run_solutions(day: DayNum) -> Result<BTreeMap<usize, Vec<Answer>>, AOCError> {
        super::solutions::dispatch(day)
    }

    // Enums

    #[derive(Debug)]
    pub enum AOCError {
        BadDay(OsString),
        DayOutOfRange(usize),
        NoInput,
        BadInputFile(io::Error),
        ParseError,
        ParseIntError(ParseIntError, String),
    }

    impl fmt::Display for AOCError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            match self {
                Self::BadDay(s) => write!(f, "Invalid day: {:?}", s),
                Self::DayOutOfRange(d) => write!(f, "Invalid day: {}", d),
                Self::NoInput => write!(f, "No input"),
                Self::BadInputFile(e) => write!(f, "Could not read input file: {}", e),
                Self::ParseError => write!(f, "Could not parse input"),
                Self::ParseIntError(e, s) => {
                    write!(f, "Could not parse integer from string \"{}\": {}", s, e)
                }
            }
        }
    }

    impl Error for AOCError {}

    #[derive(Debug)]
    pub enum Answer {
        Integer(usize),
        Printable(Vec<u8>),
        None,
        Error(Box<dyn Error>),
    }

    impl PartialEq for Answer {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Self::Integer(a) => matches!(other, Self::Integer(b) if *a == *b),
                Self::Printable(a) => matches!(other, Self::Printable(b) if *a == *b),
                Self::None => matches!(other, Self::None),
                Self::Error(_) => false,
            }
        }
    }

    impl From<Option<usize>> for Answer {
        fn from(o: Option<usize>) -> Self {
            match o {
                Some(n) => Self::Integer(n),
                None => Self::None,
            }
        }
    }

    pub enum DayNum {
        One(usize, OsString),
        All,
    }

    // Traits

    pub trait Day {
        fn new(input: impl Iterator<Item = String>) -> Result<Self, AOCError> where Self: Sized;
        fn part_1(&self) -> Answer;
        fn part_2(&self) -> Answer;
    }
}
