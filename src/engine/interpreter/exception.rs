use std::{
    error::Error as ErrorTrait,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Exception {
    InvalidLoad = 4,
    InvalidStore = 5,
    SyscallFailure = 8,
    Breakpoint = 9,
    ReservedInstruction = 10,
    IntegerOverflowOrUndeflow = 12,
    Trap = 13,
    DivideByZero = 15,
    FloatOverflow = 16,
    FloatUnderflow = 17,
    InterpreterFailure = 21, // hopefully you never see this one
}

impl Exception {
    pub const fn as_str(&self) -> &'static str {
        use Exception::*;
        match *self {
            InvalidLoad => "invalid load",
            InvalidStore => "invalid store",
            SyscallFailure => "syscall failed to execute",
            Breakpoint => "hit breakpoint",
            ReservedInstruction => "encountered a reserved instruction",
            IntegerOverflowOrUndeflow => "integer overflow/underflow",
            Trap => "trapped",
            DivideByZero => "tried to divide by zero",
            FloatOverflow => "floating-point operation overflowed",
            FloatUnderflow => "floating-point operation underflowed",
            InterpreterFailure => "the interpreter did a goof (pls contact rose)",
        }
    }
}

impl Display for Exception {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        fmt.write_str(self.as_str())
    }
}

impl ErrorTrait for Exception {}
