use seaside_rich_error::{ErrorCode, ErrorGroup, error_code::ErrorGroupPrefix};
use thiserror::Error;

use super::LexError;

/// An error that occurs during the parsing stage of assembly.
#[repr(u16)]
#[derive(Clone, Debug, Eq, Error, Hash, PartialEq)]
pub enum ParseError {
    /// An error occurred during the lexing stage.
    #[error("{0}")]
    Lex(#[from] LexError),

    /// The parser was expecting a [token](crate::token::Token), but it was met with the end of the
    /// file (EOF) instead.
    ///
    /// This can kind of be thought of as a special case of
    /// [`UnexpectedToken`](ParseError::UnexpectedToken), except that EOF is not considered a token.
    #[error("unexpectedly hit end of source file")]
    PrematureEof,

    /// The parser was expecting a certain set of [token](crate::token::Token)s, but it encountered
    /// one outside that set instead.
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

impl ErrorGroup for ParseError {
    const PREFIX: ErrorGroupPrefix = *b"PRS";

    fn code(&self) -> ErrorCode {
        use ParseError::*;
        ErrorCode::new_for::<Self>(match self {
            Lex(err) => return err.code(),
            PrematureEof => 0,
            UnexpectedToken => 1,
            UnknownDirective => 2,
            ValueOutsideRange => 3,
        })
    }
}
