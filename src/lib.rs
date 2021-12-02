pub mod prelude {
    use std::error::Error;
    use std::ffi::OsString;
    use std::fmt;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    #[derive(Debug)]
    pub enum AOCError {
        BadDay(OsString),
        DayOutOfRange(usize),
        NoInput,
        BadInputFile(io::Error),
        ParseError,
    }

    impl fmt::Display for AOCError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            match self {
                Self::BadDay(s) => write!(f, "Invalid day: {:?}", s),
                Self::DayOutOfRange(d) => write!(f, "Invalid day: {}", d),
                Self::NoInput => write!(f, "No input"),
                Self::BadInputFile(e) => write!(f, "Could not read input file: {}", e),
                Self::ParseError => write!(f, "Could not parse input"),
            }
        }
    }

    impl Error for AOCError {}

    pub fn read_input_lines(path: impl AsRef<Path>) -> Result<impl Iterator<Item = String>, AOCError> {
        let file = File::open(path).map_err(|e| AOCError::BadInputFile(e))?;
        let lines = io::BufReader::new(file)
            .lines()
            .map(|l| l.map_err(|e| AOCError::BadInputFile(e)))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter();
        Ok(lines)
    }
}
