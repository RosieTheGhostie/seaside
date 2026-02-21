use seaside_rich_error::{ErrorCode, ToErrorCode};
use thiserror::Error;

use super::ParseError;

#[derive(Debug, Error)]
pub enum AssembleError {
    #[error("{0}")]
    Parse(ParseError),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("symbol defined multiple times")]
    MultipleDefinitions,

    #[error("no definition provided for an implicitly declared symbol")]
    UndefinedSymbol,

    #[error("segment directives cannot skip to a previous address")]
    JumpBehind,

    #[error("jump to an instruction that is too far away")]
    JumpTooLarge,

    #[error("computed offset exceeds the permitted bounds")]
    OffsetTooLarge,

    #[error("found a set of symbols where it is not permitted")]
    WrongSegment,

    #[error("value is of the wrong type")]
    WrongType,

    #[error("directive not yet supported by assembler")]
    UnsupportedDirective,

    #[error("unknown operator")]
    UnknownOperator,

    #[error("not enough operands for this operator")]
    NotEnoughOperands,

    #[error("too many operands for this operator")]
    TooManyOperands,

    #[error("the program counter overflowed")]
    ProgramCounterOverflow,

    #[error("unterminated string literal")]
    UnterminatedStringLiteral,

    #[error("invalid escape sequence")]
    InvalidEscapeSequence,

    #[error("invalid UTF-8")]
    InvalidUtf8,
}

impl From<ParseError> for AssembleError {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}

impl ToErrorCode for AssembleError {
    fn code(&self) -> ErrorCode {
        const IO_ERROR_OFFSET: ErrorCode = 900;

        use AssembleError::*;
        match self {
            Parse(err) => err.code(),
            Io(err) => IO_ERROR_OFFSET + err.kind() as ErrorCode,
            MultipleDefinitions => 200,
            UndefinedSymbol => 201,
            JumpBehind => 202,
            JumpTooLarge => 203,
            OffsetTooLarge => 204,
            WrongSegment => 205,
            WrongType => 206,
            UnsupportedDirective => 207,
            UnknownOperator => 208,
            NotEnoughOperands => 209,
            TooManyOperands => 210,
            ProgramCounterOverflow => 211,
            UnterminatedStringLiteral => 212,
            InvalidEscapeSequence => 213,
            InvalidUtf8 => 214,
        }
    }
}
