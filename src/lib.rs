pub mod prelude {
    use std::env;
    use std::error::Error;
    use std::fmt;
    use std::fs::File;
    use std::io::{self, BufRead};

    use anyhow::Result;

    #[derive(Debug)]
    pub enum AOCError {
        NoInput,
        InputParseError,
    }

    impl fmt::Display for AOCError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
            match self {
                Self::NoInput => write!(f, "No input"),
                Self::InputParseError => write!(f, "Could not parse input"),
            }
        }
    }

    impl Error for AOCError {}

    pub fn read_input_lines() -> Result<impl Iterator<Item = String>> {
        let path = env::args_os().nth(1).ok_or(AOCError::NoInput)?;
        let file = File::open(path)?;
        let lines = io::BufReader::new(file)
            .lines()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter();
        Ok(lines)
    }
}
