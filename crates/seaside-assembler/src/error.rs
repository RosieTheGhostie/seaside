use super::ParseError;
use seaside_error::{Error as EngineError, ErrorKind as EngineErrorKind};
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    Parse(ParseError),
    Io(IoError),
    MultipleDefinitions,
    UndefinedSymbol,
    JumpBehind,
    BranchTooLarge,
    WrongSegment,
    InternalLogicIssue,
}

impl From<Error> for EngineError {
    fn from(value: Error) -> Self {
        match value {
            Error::Parse(error) => Self::new(EngineErrorKind::SyntaxError, error),
            Error::Io(error) => match error.kind() {
                std::io::ErrorKind::NotFound => Self::new(EngineErrorKind::NotFound, error),
                _ => Self::new(EngineErrorKind::ExternalFailure, error),
            },
            Error::MultipleDefinitions => Self::new(
                EngineErrorKind::SemanticError,
                "found duplicate definitions of a symbol",
            ),
            Error::UndefinedSymbol => Self::new(
                EngineErrorKind::SemanticError,
                "no definition provided for an implicitly declared symbol",
            ),
            Error::JumpBehind => Self::new(
                EngineErrorKind::SemanticError,
                "segment directives cannot skip to a previous address",
            ),
            Error::BranchTooLarge => Self::new(
                EngineErrorKind::SemanticError,
                "attempted to assemble a branch to an instruction that was too far away",
            ),
            Error::WrongSegment => Self::new(
                EngineErrorKind::SemanticError,
                "found a set of symbols in a segment where it is not permitted",
            ),
            Error::InternalLogicIssue => Self::new(
                EngineErrorKind::InternalLogicIssue,
                "something went wrong in the assembler's internal logic",
            ),
        }
    }
}
