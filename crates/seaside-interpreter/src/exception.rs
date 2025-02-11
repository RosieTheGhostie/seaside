use super::SyscallFailureKind;
use seaside_type_aliases::Address;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum Exception {
    #[error("malformed instruction")]
    MalformedInstruction,
    #[error("invalid load (address: 0x{0:08x})")]
    InvalidLoad(Address),
    #[error("invalid load (address: 0x{0:08x})")]
    InvalidStore(Address),
    #[error("{0}")]
    SyscallFailure(#[from] SyscallFailureKind),
    #[error("break exception thrown")]
    Break,
    #[error("encountered reserved instruction")]
    ReservedInstruction,
    #[error("integer overflow/underflow")]
    IntegerOverflowOrUnderflow,
    #[error("trapped")]
    Trap,
    #[error("tried to divide by zero")]
    DivideByZero,
    #[error("floating-point operation overflowed")]
    FloatOverflow,
    #[error("floating-point operation underflowed")]
    FloatUnderflow,
    #[error("the interpreter did a goof (pls contact rose)")]
    InterpreterFailure, // hopefully you never see this one
}

impl Exception {
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
