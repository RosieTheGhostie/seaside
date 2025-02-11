use super::SyscallFailureKind;
use seaside_type_aliases::Address;
use std::{
    error::Error as ErrorTrait,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Exception {
    MalformedInstruction,
    InvalidLoad(Address),
    InvalidStore(Address),
    SyscallFailure(SyscallFailureKind),
    Break,
    ReservedInstruction,
    IntegerOverflowOrUnderflow,
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
            MalformedInstruction => "malformed instruction",
            InvalidLoad(_) => "invalid load",
            InvalidStore(_) => "invalid store",
            SyscallFailure(kind) => kind.as_str(),
            Break => "break exception raised",
            ReservedInstruction => "encountered a reserved instruction",
            IntegerOverflowOrUnderflow => "integer overflow/underflow",
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
            MalformedInstruction => 0,
            InvalidLoad(_) => 4,
            InvalidStore(_) => 5,
            SyscallFailure(_) => 8,
            Break => 9,
            ReservedInstruction => 10,
            IntegerOverflowOrUnderflow => 12,
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

    pub const fn service_code(&self) -> Option<u8> {
        if let Self::SyscallFailure(SyscallFailureKind::UnknownServiceCode(code))
        | Self::SyscallFailure(SyscallFailureKind::ServiceDisabled(code)) = *self
        {
            Some(code)
        } else {
            None
        }
    }
}

impl Display for Exception {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        if let Some(vaddr) = self.vaddr() {
            write!(fmt, "{} (address: 0x{vaddr:08x})", self.as_str())
        } else if let Some(service_code) = self.service_code() {
            write!(fmt, "{} (code: {service_code})", self.as_str())
        } else {
            fmt.write_str(self.as_str())
        }
    }
}

impl ErrorTrait for Exception {}
