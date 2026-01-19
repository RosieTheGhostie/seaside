use seaside_type_aliases::Address;
use thiserror::Error;

use crate::SyscallFailureKind;

#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum Exception {
    #[error("malformed instruction")]
    MalformedInstruction = 0,

    #[error("invalid load (address: {0:#010x})")]
    InvalidLoad(Address) = 4,

    #[error("invalid load (address: {0:#010x})")]
    InvalidStore(Address) = 5,

    #[error("{0}")]
    SyscallFailure(#[from] SyscallFailureKind) = 8,

    #[error("break exception thrown")]
    Break = 9,

    #[error("encountered reserved instruction")]
    ReservedInstruction = 10,

    #[error("integer overflow/underflow")]
    IntegerOverflowOrUnderflow = 12,

    #[error("trapped")]
    Trap = 13,

    #[error("tried to divide by zero")]
    DivideByZero = 15,

    #[error("floating-point operation overflowed")]
    FloatOverflow = 16,

    #[error("floating-point operation underflowed")]
    FloatUnderflow = 17,

    #[error("the interpreter did a goof (pls contact rose)")]
    InterpreterFailure = 21, // hopefully you never see this one
}

impl Exception {
    pub const fn code(&self) -> u32 {
        // https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.discriminant.access-memory
        unsafe { *(self as *const Self as *const u32) }
    }

    pub const fn vaddr(&self) -> Option<Address> {
        if let Self::InvalidLoad(vaddr) | Self::InvalidStore(vaddr) = *self {
            Some(vaddr)
        } else {
            None
        }
    }

    pub const fn service_code(&self) -> Option<u32> {
        if let Self::SyscallFailure(SyscallFailureKind::UnknownServiceCode(code)) = *self {
            Some(code)
        } else {
            None
        }
    }
}
