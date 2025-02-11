use thiserror::Error;

/// The kind of failure that occurred when executing a system service.
#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum SyscallFailureKind {
    /// A syscall was made with an unknown service code.
    #[error("requested unknown service (code: {0})")]
    UnknownServiceCode(u8),
    /// The requested service exists, but is currently disabled in the provided config.
    #[error("requested disabled service (code: {0})")]
    ServiceDisabled(u8),
    /// The requested service exists and is enabled, but no implementation for it exists yet.
    #[error("requested unimplemented service (code: {0})")]
    ServiceUnimplemented(u8),
    /// No nul byte was found at the end of the provided string.
    #[error("couldn't find nul byte at end of string")]
    NulNotFound,
    /// The provided character/string is not valid UTF-8.
    #[error("input is invalid UTF-8")]
    InvalidUtf8,
    /// Failed to flush stdout.
    #[error("failed to flush stdout")]
    StdoutFlushFailed,
    /// Failed to read input from stdin.
    #[error("failed to read from stdin")]
    StdinReadFailed,
    /// Failed to parse input as the desired type.
    #[error("failed to parse input")]
    ParseError,
    /// Failed to write data to a buffer.
    #[error("failed to write to buffer")]
    WriteFailed,
    /// Attempted to free memory from the heap, but that functionality is disabled in the provided
    /// config.
    #[error("can't free memory from heap because it is disabled")]
    HeapFreeDisabled,
    /// The current system time is before the unix epoch (1970-01-01T00:00:00).
    #[error("system time is before January 1, 1970")]
    BeforeUnixEpoch,
    /// The provided input(s) cannot yield a sensible output due to logical impossibility.
    #[error("can't provide sensible output for given input(s)")]
    NoPossibleOutput,
}
