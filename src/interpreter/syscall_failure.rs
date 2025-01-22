/// The kind of failure that occurred when executing a system service.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SyscallFailureKind {
    /// A syscall was made with an unknown service code.
    UnknownServiceCode(u8),
    /// The requested service exists, but is currently disabled in the provided config.
    ServiceDisabled(u8),
    /// No nul byte was found at the end of the provided string.
    NulNotFound,
    /// The provided character/string is not valid UTF-8.
    InvalidUtf8,
    /// Failed to flush stdout.
    StdoutFlushFailed,
    /// Failed to read input from stdin.
    StdinReadFailed,
    /// Failed to parse input as the desired type.
    ParseError,
    /// Failed to write data to a buffer.
    WriteFailed,
    /// Attempted to free memory from the heap, but that functionality is disabled in the provided
    /// config.
    HeapFreeDisabled,
    /// The current system time is before the unix epoch (1970-01-01T00:00:00).
    BeforeUnixEpoch,
    /// The provided input(s) cannot yield a sensible output due to logical impossibility.
    NoPossibleOutput,
}

impl SyscallFailureKind {
    pub const fn as_str(&self) -> &'static str {
        use SyscallFailureKind::*;
        match *self {
            UnknownServiceCode(_) => "requested an unknown service",
            ServiceDisabled(_) => "requested a disabled service",
            NulNotFound => "couldn't find nul byte at end of string",
            InvalidUtf8 => "input is invalid UTF-8",
            StdoutFlushFailed => "failed to flush stdout",
            StdinReadFailed => "failed to read from stdin",
            ParseError => "failed to parse input",
            WriteFailed => "failed to write to buffer",
            HeapFreeDisabled => "can't free memory from the heap because it is disabled",
            BeforeUnixEpoch => "system time is before January 1, 1970",
            NoPossibleOutput => "can't provide a sensible output for the given input(s)",
        }
    }
}
