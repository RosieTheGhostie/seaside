use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParseError {
    #[error("encountered directive that is disabled by config")]
    DirectiveDisabled,
    #[error("something went wrong in parser's internal logic")]
    InternalLogicIssue,
    #[error("unexpectedly hit end of source file")]
    PrematureEof,
    #[error("encountered unexpected token in source code")]
    UnexpectedToken,
    #[error("encountered unknown token in source code")]
    UnknownToken,
    #[error("a value provided lies outside the valid range")]
    ValueOutsideRange,
}
