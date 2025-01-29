use std::{
    error::Error as ErrorTrait,
    fmt::{Display, Formatter, Result as FmtResult},
};

use crate::assembler::Error as AssemblyError;

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

impl From<AssemblyError> for Error {
    fn from(value: AssemblyError) -> Self {
        match value {
            AssemblyError::Parse(error) => Self::new(ErrorKind::SyntaxError, error),
            AssemblyError::Io(error) => match error.kind() {
                std::io::ErrorKind::NotFound => Self::new(ErrorKind::NotFound, error),
                _ => Self::new(ErrorKind::ExternalFailure, error),
            },
            AssemblyError::MultipleDefinitions => Self::new(
                ErrorKind::SemanticError,
                "found duplicate definitions of a symbol",
            ),
            AssemblyError::UndefinedSymbol => Self::new(
                ErrorKind::SemanticError,
                "no definition provided for an implicitly declared symbol",
            ),
            AssemblyError::JumpBehind => Self::new(
                ErrorKind::SemanticError,
                "segment directives cannot skip to a previous address",
            ),
            AssemblyError::BranchTooLarge => Self::new(
                ErrorKind::SemanticError,
                "attempted to assemble a branch to an instruction that was too far away",
            ),
            AssemblyError::WrongSegment => Self::new(
                ErrorKind::SemanticError,
                "found a set of symbols in a segment where it is not permitted",
            ),
            AssemblyError::InternalLogicIssue => Self::new(
                ErrorKind::InternalLogicIssue,
                "something went wrong in the assembler's internal logic",
            ),
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
    SemanticError,
    SyntaxError,
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
            SemanticError => "the source code provided is semantically incorrect",
            SyntaxError => "the source code provided is syntactically incorrect",
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        fmt.write_str(self.as_str())
    }
}

impl ErrorTrait for ErrorKind {}
