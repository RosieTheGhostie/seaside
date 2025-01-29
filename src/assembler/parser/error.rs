use super::super::Token;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    DirectiveDisabled,
    InternalLogicIssue,
    PrematureEof,
    UnexpectedToken,
    UnknownToken,
    ValueOutsideRange,
}

impl Error {
    pub fn new<S: ToString>(kind: ErrorKind, message: S) -> Self {
        Self {
            kind,
            message: message.to_string(),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            kind,
            message: String::new(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.message.is_empty() {
            f.write_str(self.kind.as_str())
        } else {
            f.write_str(&self.message)
        }
    }
}

impl ErrorKind {
    pub const fn as_str(&self) -> &'static str {
        use ErrorKind::*;
        match *self {
            DirectiveDisabled => "encountered a directive that is disabled by the config",
            InternalLogicIssue => "something went wrong in the parser's internal logic",
            PrematureEof => "unexpectedly hit the end of the source file",
            UnexpectedToken => "encountered an unexpected token in the source code",
            UnknownToken => "encountered an unknown token in the source code",
            ValueOutsideRange => "a value provided lies outside the valid range",
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}
