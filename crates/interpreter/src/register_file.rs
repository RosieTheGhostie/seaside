use crate::Exception;
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    iter::zip,
    mem::transmute,
};
use seaside_config::RegisterDefaults;
use seaside_constants::{
    ConditionCode,
    register::{CpuRegister, FpuRegister},
};
use seaside_type_aliases::Address;
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct RegisterFile {
    cpu: [u32; 32],
    pub hi: u32,
    pub lo: u32,
    fpu: [f32; 32],
    fpu_flags: u8,
    pub vaddr: Address,
    pub status: u32,
    pub cause: u32,
    pub epc: Address,
}

pub trait IndexByRegister<Register, T> {
    fn read(&self, register: Register) -> T;
    fn write(&mut self, register: Register, value: T);
}

pub trait TryIndexByRegister<Register, T> {
    fn try_read(&self, register: Register) -> Result<T, Exception>;
    fn try_write(&mut self, register: Register, value: T) -> Result<(), Exception>;
}

impl IndexByRegister<CpuRegister, u32> for RegisterFile {
    fn read(&self, register: CpuRegister) -> u32 {
        *self.cpu.get(register as usize).unwrap()
    }

    fn write(&mut self, register: CpuRegister, value: u32) {
        let index = register as usize;
        if index != 0 {
            unsafe { *self.cpu.get_unchecked_mut(index) = value };
        }
    }
}

impl IndexByRegister<CpuRegister, i32> for RegisterFile {
    fn read(&self, register: CpuRegister) -> i32 {
        <_ as IndexByRegister<_, u32>>::read(self, register) as i32
    }

    fn write(&mut self, register: CpuRegister, value: i32) {
        self.write(register, value as u32)
    }
}

impl IndexByRegister<FpuRegister, f32> for RegisterFile {
    fn read(&self, register: FpuRegister) -> f32 {
        *self.fpu.get(register as usize).unwrap()
    }

    fn write(&mut self, register: FpuRegister, value: f32) {
        unsafe { *self.fpu.get_unchecked_mut(register as usize) = value };
    }
}

impl TryIndexByRegister<FpuRegister, f64> for RegisterFile {
    fn try_read(&self, register: FpuRegister) -> Result<f64, Exception> {
        if register.is_double_aligned() {
            let i = register as usize;
            Ok(unsafe { transmute::<_, f64>([self.fpu[i], self.fpu[i + 1]]) })
        } else {
            Err(Exception::MalformedInstruction)
        }
    }

    fn try_write(&mut self, register: FpuRegister, value: f64) -> Result<(), Exception> {
        if register.is_double_aligned() {
            let i = register as usize;
            let halves = unsafe { transmute::<f64, [f32; 2]>(value) };
            self.fpu[i] = halves[0];
            self.fpu[i + 1] = halves[1];
            Ok(())
        } else {
            Err(Exception::MalformedInstruction)
        }
    }
}

impl IndexByRegister<FpuRegister, u32> for RegisterFile {
    fn read(&self, register: FpuRegister) -> u32 {
        <_ as IndexByRegister<_, f32>>::read(self, register).to_bits()
    }

    fn write(&mut self, register: FpuRegister, value: u32) {
        self.write(register, f32::from_bits(value))
    }
}

impl IndexByRegister<FpuRegister, i32> for RegisterFile {
    fn read(&self, register: FpuRegister) -> i32 {
        <_ as IndexByRegister<_, u32>>::read(self, register) as i32
    }

    fn write(&mut self, register: FpuRegister, value: i32) {
        self.write(register, f32::from_bits(value as u32))
    }
}

impl TryIndexByRegister<FpuRegister, u64> for RegisterFile {
    fn try_read(&self, register: FpuRegister) -> Result<u64, Exception> {
        <_ as TryIndexByRegister<_, f64>>::try_read(self, register).map(f64::to_bits)
    }

    fn try_write(&mut self, register: FpuRegister, value: u64) -> Result<(), Exception> {
        self.try_write(register, f64::from_bits(value))
    }
}

impl TryIndexByRegister<FpuRegister, i64> for RegisterFile {
    fn try_read(&self, register: FpuRegister) -> Result<i64, Exception> {
        <_ as TryIndexByRegister<_, u64>>::try_read(self, register).map(|value| value as i64)
    }

    fn try_write(&mut self, register: FpuRegister, value: i64) -> Result<(), Exception> {
        self.try_write(register, value as u64)
    }
}

impl RegisterFile {
    pub fn read_fpu_flag(&self, cc: ConditionCode) -> bool {
        (self.fpu_flags >> cc as u8) & 1 == 1
    }

    pub fn write_fpu_flag(&mut self, cc: ConditionCode, value: bool) {
        let index = cc as u8;
        let mask = 1 << index;
        let value = if value { mask } else { 0 };
        self.fpu_flags &= !mask;
        self.fpu_flags |= value;
    }

    pub fn init(register_defaults: &RegisterDefaults) -> Self {
        let mut register_file = Self::default();
        for (register, &default_value) in zip(
            CpuRegister::iter(),
            register_defaults.general_purpose.iter(),
        ) {
            register_file.write(register, default_value);
        }
        register_file.hi = register_defaults.hi;
        register_file.lo = register_defaults.lo;
        for (register, &default_value) in
            zip(FpuRegister::iter(), register_defaults.coprocessor_1.iter())
        {
            register_file.write(register, default_value);
        }
        register_file.vaddr = register_defaults.coprocessor_0[0];
        register_file.status = register_defaults.coprocessor_0[1];
        register_file.cause = register_defaults.coprocessor_0[2];
        register_file.epc = register_defaults.coprocessor_0[3];
        register_file
    }
}

impl Display for RegisterFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "┏━━━━━━━━━━━━━━━━━━━━━ CPU ━━━━━━━━━━━━━━━━━━━━━┓")?;
        write_cpu_registers(self, f)?;
        writeln!(f, "┣━━━━━━━━━━━━━━━━━━━━━ FPU ━━━━━━━━━━━━━━━━━━━━━┫")?;
        write_fpu_registers(self, f)?;
        write!(
            f,
            "┠┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈ Flags ┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┨\n┃    "
        )?;
        write_fpu_flags(self.fpu_flags, f)?;
        writeln!(
            f,
            "    ┃\n┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"
        )
    }
}

fn write_cpu_registers(register_file: &RegisterFile, f: &mut Formatter<'_>) -> FmtResult {
    writeln!(
        f,
        r"┃ $at: {:08x} ┊ $v0: {:08x} ┊ $v1: {:08x} ┃
┃ $a0: {:08x} ┊ $a1: {:08x} ┊ $a2: {:08x} ┃
┃ $a3: {:08x} ┊ $t0: {:08x} ┊ $t1: {:08x} ┃
┃ $t2: {:08x} ┊ $t3: {:08x} ┊ $t4: {:08x} ┃
┃ $t5: {:08x} ┊ $t6: {:08x} ┊ $t7: {:08x} ┃
┃ $s0: {:08x} ┊ $s1: {:08x} ┊ $s2: {:08x} ┃
┃ $s3: {:08x} ┊ $s4: {:08x} ┊ $s5: {:08x} ┃
┃ $s6: {:08x} ┊ $s7: {:08x} ┊ $t8: {:08x} ┃
┃ $t9: {:08x} ┊ $k0: {:08x} ┊ $k1: {:08x} ┃
┃ $gp: {:08x} ┊ $sp: {:08x} ┊ $fp: {:08x} ┃
┃ $ra: {:08x} ┊  hi: {:08x} ┊  lo: {:08x} ┃",
        register_file.cpu[CpuRegister::AsmTemp as usize],
        register_file.cpu[CpuRegister::Val0 as usize],
        register_file.cpu[CpuRegister::Val1 as usize],
        register_file.cpu[CpuRegister::Arg0 as usize],
        register_file.cpu[CpuRegister::Arg1 as usize],
        register_file.cpu[CpuRegister::Arg2 as usize],
        register_file.cpu[CpuRegister::Arg3 as usize],
        register_file.cpu[CpuRegister::Temp0 as usize],
        register_file.cpu[CpuRegister::Temp1 as usize],
        register_file.cpu[CpuRegister::Temp2 as usize],
        register_file.cpu[CpuRegister::Temp3 as usize],
        register_file.cpu[CpuRegister::Temp4 as usize],
        register_file.cpu[CpuRegister::Temp5 as usize],
        register_file.cpu[CpuRegister::Temp6 as usize],
        register_file.cpu[CpuRegister::Temp7 as usize],
        register_file.cpu[CpuRegister::Saved0 as usize],
        register_file.cpu[CpuRegister::Saved1 as usize],
        register_file.cpu[CpuRegister::Saved2 as usize],
        register_file.cpu[CpuRegister::Saved3 as usize],
        register_file.cpu[CpuRegister::Saved4 as usize],
        register_file.cpu[CpuRegister::Saved5 as usize],
        register_file.cpu[CpuRegister::Saved6 as usize],
        register_file.cpu[CpuRegister::Saved7 as usize],
        register_file.cpu[CpuRegister::Temp8 as usize],
        register_file.cpu[CpuRegister::Temp9 as usize],
        register_file.cpu[CpuRegister::Kernel0 as usize],
        register_file.cpu[CpuRegister::Kernel1 as usize],
        register_file.cpu[CpuRegister::GlobalPtr as usize],
        register_file.cpu[CpuRegister::StackPtr as usize],
        register_file.cpu[CpuRegister::FramePtr as usize],
        register_file.cpu[CpuRegister::ReturnAddr as usize],
        register_file.hi,
        register_file.lo,
    )
}

fn write_fpu_registers(register_file: &RegisterFile, f: &mut Formatter<'_>) -> FmtResult {
    for i in 0u8..7u8 {
        let i0 = i * 4;
        let i1 = i0 + 2;
        let r0 = unsafe { transmute::<u8, FpuRegister>(i0) };
        let r1 = unsafe { transmute::<u8, FpuRegister>(i1) };
        writeln!(
            f,
            "┃ {}{i0}: {:>+#15.7e} ┊ {}{i1}: {:>+#15.7e} ┃",
            if i0 >= 10 { "$f" } else { " $f" },
            <_ as TryIndexByRegister<_, f64>>::try_read(register_file, r0).unwrap(),
            if i1 >= 10 { "$f" } else { " $f" },
            <_ as TryIndexByRegister<_, f64>>::try_read(register_file, r1).unwrap(),
        )?;
    }
    Ok(())
}

fn write_fpu_flags(mut flags: u8, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "0[{}]", if flags & 1 == 1 { '#' } else { ' ' })?;
    flags >>= 1;
    for i in 1..8 {
        write!(f, " {i}[{}]", if flags & 1 == 1 { '#' } else { ' ' })?;
    }
    Ok(())
}
