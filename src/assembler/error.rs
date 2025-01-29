use super::ParseError;
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
