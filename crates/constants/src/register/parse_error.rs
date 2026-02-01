use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParseError {
    #[error("no name was provided")]
    Empty,

    #[error("not a known register name")]
    BadValue,
}
