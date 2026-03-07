pub use index::{IndexByRegister, TryIndexByRegister};
pub use status::{Status, StatusBuilder};

mod fmt_utils;
mod index;
mod status;

use core::fmt::{self, Display, Formatter};

use seaside_core::prelude::*;

use crate::Exception;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RegisterFile {
    cpu: [u32; 32],
    pub hi: u32,
    pub lo: u32,
    fpu: [f32; 32],
    fpu_flags: u8,
    pub vaddr: Address,
    pub status: Status,
    pub cause: u32,
    pub epc: Address,
}

impl IndexByRegister<CpuRegister, u32> for RegisterFile {
    fn read(&self, register: CpuRegister) -> u32 {
        // SAFETY: The discriminant of `CpuRegister` is always on the range 0..32.
        unsafe { *self.cpu.get_unchecked(register as usize) }
    }

    fn write(&mut self, register: CpuRegister, value: u32) {
        let index: usize = register as _;
        if index != 0 {
            // SAFETY: `index` must be on the range 1..32.
            unsafe { *self.cpu.get_unchecked_mut(index) = value };
        }
    }
}

impl IndexByRegister<CpuRegister, i32> for RegisterFile {
    fn read(&self, register: CpuRegister) -> i32 {
        <_ as IndexByRegister<_, u32>>::read(self, register) as _
    }

    fn write(&mut self, register: CpuRegister, value: i32) {
        self.write(register, value as u32)
    }
}

impl IndexByRegister<FpuRegister, f32> for RegisterFile {
    fn read(&self, register: FpuRegister) -> f32 {
        // SAFETY: The discriminant of `FpuRegister` is always on the range 0..32.
        unsafe { *self.fpu.get_unchecked(register as usize) }
    }

    fn write(&mut self, register: FpuRegister, value: f32) {
        // SAFETY: The discriminant of `FpuRegister` is always on the range 0..32.
        unsafe { *self.fpu.get_unchecked_mut(register as usize) = value };
    }
}

impl TryIndexByRegister<FpuRegister, f64> for RegisterFile {
    fn try_read(&self, register: FpuRegister) -> Result<f64, Exception> {
        if register.is_double_aligned() {
            let i: usize = register as _;

            // FIXME: This is potentially incorrect, but I don't have a big-endian machine to test
            // it on at the moment.
            Ok(unsafe { core::mem::transmute::<[f32; 2], f64>([self.fpu[i], self.fpu[i + 1]]) })
        } else {
            Err(Exception::MalformedInstruction)
        }
    }

    fn try_write(&mut self, register: FpuRegister, value: f64) -> Result<(), Exception> {
        if register.is_double_aligned() {
            let i: usize = register as _;

            // FIXME: This is potentially incorrect, but I don't have a big-endian machine to test
            // it on at the moment.
            [self.fpu[i], self.fpu[i + 1]] =
                unsafe { core::mem::transmute::<f64, [f32; 2]>(value) };
            Ok(())
        } else {
            Err(Exception::MalformedInstruction)
        }
    }
}

impl IndexByRegister<FpuRegister, u32> for RegisterFile {
    fn read(&self, register: FpuRegister) -> u32 {
        f32::to_bits(self.read(register))
    }

    fn write(&mut self, register: FpuRegister, value: u32) {
        self.write(register, f32::from_bits(value))
    }
}

impl IndexByRegister<FpuRegister, i32> for RegisterFile {
    fn read(&self, register: FpuRegister) -> i32 {
        <_ as IndexByRegister<_, u32>>::read(self, register) as _
    }

    fn write(&mut self, register: FpuRegister, value: i32) {
        self.write(register, f32::from_bits(value as _))
    }
}

impl TryIndexByRegister<FpuRegister, u64> for RegisterFile {
    fn try_read(&self, register: FpuRegister) -> Result<u64, Exception> {
        self.try_read(register).map(f64::to_bits)
    }

    fn try_write(&mut self, register: FpuRegister, value: u64) -> Result<(), Exception> {
        self.try_write(register, f64::from_bits(value))
    }
}

impl TryIndexByRegister<FpuRegister, i64> for RegisterFile {
    fn try_read(&self, register: FpuRegister) -> Result<i64, Exception> {
        <_ as TryIndexByRegister<_, u64>>::try_read(self, register).map(|value| value as _)
    }

    fn try_write(&mut self, register: FpuRegister, value: i64) -> Result<(), Exception> {
        self.try_write(register, value as u64)
    }
}

impl RegisterFile {
    pub fn read_fpu_flag(&self, cc: ConditionCode) -> bool {
        (self.fpu_flags >> cc.as_u8()) & 1 == 1
    }

    pub fn write_fpu_flag(&mut self, cc: ConditionCode, value: bool) {
        let mask = 1 << cc.as_u8();
        let value = if value { mask } else { 0 };
        self.fpu_flags &= !mask;
        self.fpu_flags |= value;
    }
}

impl Display for RegisterFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "┏━━━━━━━━━━━━━━━━━━━━━ CPU ━━━━━━━━━━━━━━━━━━━━━┓")?;
        fmt_utils::write_cpu_registers(self, f)?;
        writeln!(f, "┣━━━━━━━━━━━━━━━━━━━━━ FPU ━━━━━━━━━━━━━━━━━━━━━┫")?;
        fmt_utils::write_fpu_registers(self, f)?;
        write!(
            f,
            "┠┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈ Flags ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┨\n┃    "
        )?;
        fmt_utils::write_fpu_flags(self.fpu_flags, f)?;
        writeln!(
            f,
            "    ┃\n┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"
        )
    }
}
