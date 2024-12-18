use std::fmt::{Display, Formatter, Result as FmtResult};

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

impl Display for Exception {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Exception::*;
        match *self {
            InvalidLoad => write!(f, "invalid load"),
            InvalidStore => write!(f, "invalid store"),
            SyscallFailure => write!(f, "syscall failed to execute"),
            Breakpoint => write!(f, "hit breakpoint"),
            ReservedInstruction => write!(f, "encountered a reserved instruction"),
            IntegerOverflowOrUndeflow => write!(f, "integer overflow/underflow"),
            Trap => write!(f, "trapped"),
            DivideByZero => write!(f, "tried to divide by zero"),
            FloatOverflow => write!(f, "floating-point operation overflowed"),
            FloatUnderflow => write!(f, "floating-point operation underflowed"),
            InterpreterFailure => write!(f, "the interpreter did a goof (pls contact rose)"),
        }
    }
}
