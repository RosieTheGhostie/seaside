pub mod cc;
pub mod coprocessor_0;
pub mod coprocessor_1;
pub mod cpu;
pub mod indexed;

pub use cc::ConditionCode;
pub use coprocessor_0::Coprocessor0Register;
pub use coprocessor_1::FpuRegister;
pub use cpu::CpuRegister;
pub use indexed::IndexedRegister;

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParseError {
    #[error("no name was provided")]
    Empty,

    #[error("not a known register name")]
    BadValue,
}

impl IndexedRegister {
    pub const fn to_cpu(self) -> CpuRegister {
        unsafe { core::mem::transmute(self) }
    }

    pub const fn to_fpu(self) -> FpuRegister {
        unsafe { core::mem::transmute(self) }
    }

    pub const fn try_into_coprocessor_0(self) -> Option<Coprocessor0Register> {
        Coprocessor0Register::try_from_indexed(self)
    }
}

impl CpuRegister {
    pub const fn to_fpu(self) -> FpuRegister {
        unsafe { core::mem::transmute(self) }
    }

    pub const fn to_indexed(self) -> IndexedRegister {
        unsafe { core::mem::transmute(self) }
    }
}

impl Coprocessor0Register {
    pub const fn try_from_indexed(register: IndexedRegister) -> Option<Self> {
        #![allow(
            clippy::just_underscores_and_digits,
            reason = "there aren't any better names"
        )]

        use IndexedRegister::*;

        match register {
            _8 => Some(Self::VirtualAddr),
            _12 => Some(Self::Status),
            _13 => Some(Self::Cause),
            _14 => Some(Self::ErrorPc),
            _ => None,
        }
    }
}

impl FpuRegister {
    pub const fn from_indexed(register: IndexedRegister) -> Self {
        // SAFETY: `FpuRegister` and `IndexedRegister` represent the same range of indices.
        unsafe { Self::from_raw_unchecked(register as u8) }
    }

    pub const fn to_cpu(self) -> CpuRegister {
        unsafe { core::mem::transmute(self) }
    }

    pub const fn to_indexed(self) -> IndexedRegister {
        unsafe { core::mem::transmute(self) }
    }
}
