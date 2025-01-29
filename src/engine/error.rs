use std::{
    error::Error as ErrorTrait,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
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

impl ErrorTrait for Error {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    ExternalFailure,
    InternalLogicIssue,
    InvalidConfig,
    InvalidProjectDirectory,
    MalformedMachineCode,
    MipsException,
    NotFound,
    OutdatedVersion,
    InvalidSyntax,
}

impl ErrorKind {
    pub const fn as_str(&self) -> &'static str {
        use ErrorKind::*;
        match *self {
            ExternalFailure => "something went wrong outside the engine's control",
            InternalLogicIssue => "something went wrong in the engine's internal logic",
            InvalidConfig => "provided config file is invalid",
            InvalidProjectDirectory => "provided project directory is invalid",
            MalformedMachineCode => "provided machine code is malformed",
            MipsException => "unhandled exception thrown in MIPS interpreter",
            NotFound => "engine expected a resource, but couldn't find it",
            OutdatedVersion => "this version of seaside is incompatible with the config provided",
            InvalidSyntax => "the source code provided is syntactically incorrect",
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        fmt.write_str(self.as_str())
    }
}

impl ErrorTrait for ErrorKind {}
