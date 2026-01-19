use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Error, Ord, PartialEq, PartialOrd)]
pub enum ParseError {
    #[error("no name was provided")]
    Empty,

    #[error("not a known register name")]
    BadValue,
}
