use crate::ParseError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssembleError {
    #[error("{0}")]
    Parse(#[from] ParseError),
    #[error("{0}")]
    Io(#[from] IoError),
    #[error("found duplicate definitions of a symbol")]
    MultipleDefinitions,
    #[error("no definition provided for an implicitly declared symbol")]
    UndefinedSymbol,
    #[error("segment directives cannot skip to a previous address")]
    JumpBehind,
    #[error("attempted to assemble a branch to an instruction that was too far away")]
    BranchTooLarge,
    #[error("found a set of symbols where it is not permitted")]
    WrongSegment,
    #[error("something went wrong in the assembler's internal logic")]
    InternalLogicIssue,
}
