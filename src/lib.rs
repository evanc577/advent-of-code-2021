pub mod prelude {
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    pub enum AOCError {
        NoInput,
        InputParseError,
    }

    impl Display for AOCError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            match self {
                Self::NoInput => write!(f, "No input"),
                Self::InputParseError => write!(f, "Could not parse input"),
            }
        }
    }

    impl Error for AOCError {}
}
