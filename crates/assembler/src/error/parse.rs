use super::LexError;
use seaside_error::rich::ToErrorCode;
use thiserror::Error;

/// An error that occurs during the parsing stage of assembly.
#[repr(u16)]
#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum ParseError {
    /// An error occurred during the lexing stage.
    #[error("{0}")]
    LexError(LexError),
    /// The parser was expecting a [token](crate::Token), but it was met with the end of the file
    /// (EOF) instead.
    ///
    /// This can kind of be thought of as a special case of
    /// [`UnexpectedToken`](ParseError::UnexpectedToken), except that EOF is not considered a token.
    #[error("unexpectedly hit end of source file")]
    PrematureEof,
    /// The parser was expecting a certain set of [token](crate::Token)s, but it encountered one
    /// outside that set instead.
    #[error("unexpected token")]
    UnexpectedToken,
    /// The parser stumbled across an unknown assembler directive.
    #[error("unknown directive")]
    UnknownDirective,
    /// A value (usually an integer) was expected to lie in a certain range of values, but it did
    /// not.
    #[error("value lies outside the valid range in this context")]
    ValueOutsideRange,
}

impl From<LexError> for ParseError {
    fn from(err: LexError) -> Self {
        Self::LexError(err)
    }
}

impl ToErrorCode for ParseError {
    fn code(&self) -> u16 {
        use ParseError::*;
        match self {
            LexError(err) => err.code(),
            PrematureEof => 101,
            UnexpectedToken => 102,
            UnknownDirective => 103,
            ValueOutsideRange => 104,
        }
    }
}
