pub mod cc;
pub mod coprocessor_0;
pub mod coprocessor_1;
pub mod cpu;
pub mod indexed;

pub use cc::ConditionCode;
pub use coprocessor_0::Coprocessor0Register;
pub use coprocessor_1::FpuRegister;
pub use cpu::CpuRegister;
pub use indexed::RegisterIndex;

use thiserror::Error;

use crate::u5;

#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParseError {
    #[error("no name was provided")]
    Empty,

    #[error("not a known register name")]
    BadValue,
}

impl RegisterIndex {
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

    pub const fn to_indexed(self) -> RegisterIndex {
        unsafe { core::mem::transmute(self) }
    }
}

impl Coprocessor0Register {
    pub const fn try_from_indexed(register: RegisterIndex) -> Option<Self> {
        match register.0.as_u8() {
            8 => Some(Self::VirtualAddr),
            12 => Some(Self::Status),
            13 => Some(Self::Cause),
            14 => Some(Self::ErrorPc),
            _ => None,
        }
    }

    pub const fn to_indexed(self) -> RegisterIndex {
        RegisterIndex(unsafe { u5::new_unchecked(self as _) })
    }
}

impl FpuRegister {
    pub const fn from_indexed(register: RegisterIndex) -> Self {
        Self::from_raw(register.0)
    }

    pub const fn to_cpu(self) -> CpuRegister {
        unsafe { core::mem::transmute(self) }
    }

    pub const fn to_indexed(self) -> RegisterIndex {
        unsafe { core::mem::transmute(self) }
    }
}
