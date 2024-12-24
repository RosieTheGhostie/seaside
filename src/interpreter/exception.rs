use crate::type_aliases::address::Address;
use std::{
    error::Error as ErrorTrait,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Exception {
    InvalidLoad(Address),
    InvalidStore(Address),
    SyscallFailure,
    Breakpoint,
    ReservedInstruction,
    IntegerOverflowOrUndeflow,
    Trap,
    DivideByZero,
    FloatOverflow,
    FloatUnderflow,
    InterpreterFailure, // hopefully you never see this one
}

impl Exception {
    pub const fn as_str(&self) -> &'static str {
        use Exception::*;
        match *self {
            InvalidLoad(_) => "invalid load",
            InvalidStore(_) => "invalid store",
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

    pub const fn code(&self) -> u32 {
        use Exception::*;
        match *self {
            InvalidLoad(_) => 4,
            InvalidStore(_) => 5,
            SyscallFailure => 8,
            Breakpoint => 9,
            ReservedInstruction => 10,
            IntegerOverflowOrUndeflow => 12,
            Trap => 13,
            DivideByZero => 15,
            FloatOverflow => 16,
            FloatUnderflow => 17,
            InterpreterFailure => 21,
        }
    }

    pub const fn vaddr(&self) -> Option<Address> {
        if let Self::InvalidLoad(vaddr) | Self::InvalidStore(vaddr) = *self {
            Some(vaddr)
        } else {
            None
        }
    }
}

impl Display for Exception {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        if let Some(vaddr) = self.vaddr() {
            write!(fmt, "{} (address: 0x{vaddr:08x})", self.as_str())
        } else {
            fmt.write_str(self.as_str())
        }
    }
}

impl ErrorTrait for Exception {}
