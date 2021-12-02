pub mod prelude {
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    pub enum AOCError {
        NoInput,
    }

    impl Display for AOCError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "No input")
        }
    }

    impl Error for AOCError {}
}
