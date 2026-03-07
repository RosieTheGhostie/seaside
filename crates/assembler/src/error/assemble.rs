use seaside_rich_error::{ErrorCode, ErrorGroup, error_code::ErrorGroupPrefix};
use thiserror::Error;

use super::ParseError;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AssembleError {
    #[error(transparent)]
    Parse(#[from] ParseError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    // --- Symbol Issues (0) ---
    #[error("symbol defined multiple times")]
    MultipleDefinitions,

    #[error("no definition provided for an implicitly declared symbol")]
    UndefinedSymbol,

    // --- Address Issues (1) ---
    #[error("the program counter overflowed")]
    ProgramCounterOverflow,

    #[error("segment directives cannot skip to a previous address")]
    JumpBehind,

    #[error("jump to an instruction that is too far away")]
    JumpTooLarge,

    #[error("computed offset exceeds the permitted bounds")]
    OffsetTooLarge,

    #[error("found a set of symbols where it is not permitted")]
    WrongSegment,

    // --- Type Issues (2) ---
    #[error("value is of the wrong type")]
    WrongType,

    // --- Directive Issues (3) ---
    #[error("directive not yet supported by assembler")]
    UnsupportedDirective,

    #[error("unknown assembler option")]
    UnknownOption,

    // --- String Issues (4) ---
    #[error("unterminated string literal")]
    UnterminatedStringLiteral,

    #[error("invalid escape sequence")]
    InvalidEscapeSequence,

    #[error("invalid UTF-8")]
    InvalidUtf8,

    // --- Instruction and/or Operator Issues (5) ---
    #[error("unknown operator")]
    UnknownOperator,

    #[error("not enough operands for this operator")]
    NotEnoughOperands,

    #[error("too many operands for this operator")]
    TooManyOperands,

    #[error("instruction refers to an unsupported coprocessor")]
    UnsupportedCoprocessor,

    /// The assembler temporary register ([`$at`]) was explicitly used.
    ///
    /// This is potentially problematic because [`$at`] is extremely volatile and can be overwritten
    /// without the programmer realizing it due to pseudo-instructions.
    ///
    /// [`$at`]: seaside_core::register::CpuRegister::AsmTemp
    #[error("`$at` register explicitly used")]
    ExplicitAsmTemp,
}

impl ErrorGroup for AssembleError {
    const PREFIX: ErrorGroupPrefix = *b"ASM";

    fn code(&self) -> ErrorCode {
        use AssembleError::*;
        ErrorCode::new_for::<Self>(match self {
            Parse(err) => return err.code(),
            Io(err) => return err.code(),
            MultipleDefinitions => 0,
            UndefinedSymbol => 1,
            ProgramCounterOverflow => 10,
            JumpBehind => 11,
            JumpTooLarge => 12,
            OffsetTooLarge => 13,
            WrongSegment => 14,
            WrongType => 20,
            UnsupportedDirective => 30,
            UnknownOption => 31,
            UnterminatedStringLiteral => 40,
            InvalidEscapeSequence => 41,
            InvalidUtf8 => 42,
            UnknownOperator => 50,
            NotEnoughOperands => 51,
            TooManyOperands => 52,
            UnsupportedCoprocessor => 53,
            ExplicitAsmTemp => 54,
        })
    }
}
