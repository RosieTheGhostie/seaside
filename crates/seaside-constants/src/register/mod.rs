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
}

impl CpuRegister {
    pub const fn to_fpu(self) -> FpuRegister {
        unsafe { core::mem::transmute(self) }
    }

    pub const fn to_indexed(self) -> IndexedRegister {
        unsafe { core::mem::transmute(self) }
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
