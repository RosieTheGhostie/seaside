use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum EngineError {
    #[error("something went wrong outside the engine's control")]
    ExternalFailure,
    #[error("something went wrong in the engine's internal logic")]
    InternalLogicIssue,
    #[error("provided config file is invalid")]
    InvalidConfig,
    #[error("provided project directory is invalid")]
    InvalidProjectDirectory,
    #[error("provided machine code is malformed")]
    MalformedMachineCode,
    #[error("unhandled exception thrown in MIPS interpreter")]
    MipsException,
    #[error("engine expected a resource, but couldn't find it")]
    NotFound,
    #[error("this version of seaside is incompatible with the config provided")]
    OutdatedVersion,
    #[error("the source code provided is semantically incorrect")]
    SemanticError,
    #[error("the source code provided is syntactically incorrect")]
    SyntaxError,
}
