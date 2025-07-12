mod coprocessor_0;
mod cpu;
mod fpu;
mod indexed;
mod parse_error;

pub use coprocessor_0::Coprocessor0Register;
pub use cpu::CpuRegister;
pub use fpu::FpuRegister;
pub use indexed::IndexedRegister;
pub use parse_error::ParseError;

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
    pub const fn to_cpu(self) -> CpuRegister {
        unsafe { core::mem::transmute(self) }
    }

    pub const fn to_indexed(self) -> IndexedRegister {
        unsafe { core::mem::transmute(self) }
    }
}
