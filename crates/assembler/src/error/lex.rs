use core::num::{ParseFloatError, ParseIntError};
use seaside_error::rich::ToErrorCode;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Error, Hash, PartialEq)]
pub enum LexError {
    /// A miscellaneous error.
    #[error("{0}")]
    Unspecified(&'static str),
    /// The lexer identified an integer, but it couldn't be stored in an [`i64`].
    #[error("{0}")]
    InvalidInteger(&'static str),
    /// The lexer identified a floating point number, but it couldn't be stored in an [`f64`].
    #[error("float is outside the valid range for its data type")]
    InvalidFloat,
}

impl Default for LexError {
    fn default() -> Self {
        Self::Unspecified("unspecified lexing error")
    }
}

impl From<ParseFloatError> for LexError {
    fn from(_: ParseFloatError) -> Self {
        Self::InvalidFloat
    }
}

impl From<ParseIntError> for LexError {
    fn from(err: ParseIntError) -> Self {
        use core::num::IntErrorKind::*;
        Self::InvalidInteger(match err.kind() {
            Empty => "no integer was provided (you shouldn't see this)",
            InvalidDigit => "invalid digit provided (you shouldn't see this)",
            PosOverflow => "integer too large for its data type",
            NegOverflow => "integer too small for its data type",
            Zero => "number is zero (you shouldn't see this)",
            _ => "unknown integer parsing error",
        })
    }
}

impl ToErrorCode for LexError {
    fn code(&self) -> u16 {
        use LexError::*;
        match self {
            Unspecified(_) => 0,
            InvalidInteger(_) => 1,
            InvalidFloat => 2,
        }
    }
}
