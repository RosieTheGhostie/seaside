use super::parser::parse_error::ParseError;

pub enum AssemblyError {
    ParseError(ParseError),
    IoError(std::io::Error),
    MultipleDefinitions,
    UndefinedSymbol,
    JumpBehind,
    BranchTooLarge,
    WrongSegment,
    InternalLogicIssue,
}
