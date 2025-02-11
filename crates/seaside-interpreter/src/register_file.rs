use super::Exception;
use seaside_config::RegisterDefaults;
use seaside_constants::register;
use seaside_type_aliases::Address;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    mem::transmute,
};

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

impl RegisterFile {
    pub fn read_u32_from_cpu(&self, index: u8) -> Result<u32, Exception> {
        if index < 32 {
            Ok(self.cpu[index as usize])
        } else {
            Err(Exception::InterpreterFailure)
        }
    }

    pub fn read_i32_from_cpu(&self, index: u8) -> Result<i32, Exception> {
        Ok(self.read_u32_from_cpu(index)? as i32)
    }

    pub fn read_f32_from_fpu(&self, index: u8) -> Result<f32, Exception> {
        if index < 32 {
            Ok(self.fpu[index as usize])
        } else {
            Err(Exception::InterpreterFailure)
        }
    }

    pub fn read_f64_from_fpu(&self, index: u8) -> Result<f64, Exception> {
        if index >= 32 {
            Err(Exception::InterpreterFailure)
        } else if index % 2 != 0 {
            Err(Exception::MalformedInstruction)
        } else {
            let registers = [self.fpu[index as usize], self.fpu[index as usize + 1]];
            Ok(unsafe { transmute::<[f32; 2], f64>(registers) })
        }
    }

    pub fn read_u32_from_fpu(&self, index: u8) -> Result<u32, Exception> {
        Ok(self.read_f32_from_fpu(index)?.to_bits())
    }

    pub fn read_i32_from_fpu(&self, index: u8) -> Result<i32, Exception> {
        Ok(self.read_u32_from_fpu(index)? as i32)
    }

    pub fn read_u64_from_fpu(&self, index: u8) -> Result<u64, Exception> {
        Ok(self.read_f64_from_fpu(index)?.to_bits())
    }

    pub fn read_i64_from_fpu(&self, index: u8) -> Result<i64, Exception> {
        Ok(self.read_u64_from_fpu(index)? as i64)
    }

    pub fn read_flag_from_fpu(&self, index: u8) -> Result<bool, Exception> {
        if index < 8 {
            Ok((self.fpu_flags >> index) & 1 == 1)
        } else {
            Err(Exception::InterpreterFailure)
        }
    }

    pub fn write_u32_to_cpu(&mut self, index: u8, value: u32) -> Result<(), Exception> {
        if index < 32 {
            if index != 0 {
                self.cpu[index as usize] = value;
            }
            Ok(())
        } else {
            Err(Exception::InterpreterFailure)
        }
    }

    pub fn write_i32_to_cpu(&mut self, index: u8, value: i32) -> Result<(), Exception> {
        self.write_u32_to_cpu(index, value as u32)
    }

    pub fn write_f32_to_fpu(&mut self, index: u8, value: f32) -> Result<(), Exception> {
        if index < 32 {
            self.fpu[index as usize] = value;
            Ok(())
        } else {
            Err(Exception::InterpreterFailure)
        }
    }

    pub fn write_f64_to_fpu(&mut self, index: u8, value: f64) -> Result<(), Exception> {
        if index >= 32 {
            Err(Exception::InterpreterFailure)
        } else if index % 2 != 0 {
            Err(Exception::MalformedInstruction)
        } else {
            let index = index as usize;
            let halves: [f32; 2] = unsafe { transmute::<f64, [f32; 2]>(value) };
            self.fpu[index] = halves[0];
            self.fpu[index + 1] = halves[1];
            Ok(())
        }
    }

    pub fn write_u32_to_fpu(&mut self, index: u8, value: u32) -> Result<(), Exception> {
        self.write_f32_to_fpu(index, f32::from_bits(value))
    }

    pub fn write_i32_to_fpu(&mut self, index: u8, value: i32) -> Result<(), Exception> {
        self.write_u32_to_cpu(index, value as u32)
    }

    pub fn write_u64_to_fpu(&mut self, index: u8, value: u64) -> Result<(), Exception> {
        self.write_f64_to_fpu(index, f64::from_bits(value))
    }

    pub fn write_i64_to_fpu(&mut self, index: u8, value: i64) -> Result<(), Exception> {
        self.write_u64_to_fpu(index, value as u64)
    }

    pub fn write_flag_to_fpu(&mut self, index: u8, value: bool) -> Result<(), Exception> {
        if index < 8 {
            let mask: u8 = 1u8 << index;
            let value = if value { mask } else { 0 };
            self.fpu_flags &= !(1u8 << index);
            self.fpu_flags |= value;
            Ok(())
        } else {
            Err(Exception::InterpreterFailure)
        }
    }

    pub fn init(register_defaults: &RegisterDefaults) -> Self {
        let mut register_file = Self::default();
        // can't iterate directly due to borrow checker stuff
        for i in 0..32 {
            let default_value = register_defaults.general_purpose[i];
            let _ = register_file.write_u32_to_cpu(i as u8, default_value);
        }
        register_file.hi = register_defaults.hi;
        register_file.lo = register_defaults.lo;
        for i in 0..32 {
            let x = f32::from_bits(register_defaults.floating_point[i]);
            let _ = register_file.write_f32_to_fpu(i as u8, x);
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
    use register::*;
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
        register_file.cpu[AT as usize],
        register_file.cpu[V0 as usize],
        register_file.cpu[V1 as usize],
        register_file.cpu[A0 as usize],
        register_file.cpu[A1 as usize],
        register_file.cpu[A2 as usize],
        register_file.cpu[A3 as usize],
        register_file.cpu[T0 as usize],
        register_file.cpu[T1 as usize],
        register_file.cpu[T2 as usize],
        register_file.cpu[T3 as usize],
        register_file.cpu[T4 as usize],
        register_file.cpu[T5 as usize],
        register_file.cpu[T6 as usize],
        register_file.cpu[T7 as usize],
        register_file.cpu[S0 as usize],
        register_file.cpu[S1 as usize],
        register_file.cpu[S2 as usize],
        register_file.cpu[S3 as usize],
        register_file.cpu[S4 as usize],
        register_file.cpu[S5 as usize],
        register_file.cpu[S6 as usize],
        register_file.cpu[S7 as usize],
        register_file.cpu[T8 as usize],
        register_file.cpu[T9 as usize],
        register_file.cpu[K0 as usize],
        register_file.cpu[K1 as usize],
        register_file.cpu[GP as usize],
        register_file.cpu[SP as usize],
        register_file.cpu[FP as usize],
        register_file.cpu[RA as usize],
        register_file.hi,
        register_file.lo,
    )
}

fn write_fpu_registers(register_file: &RegisterFile, f: &mut Formatter<'_>) -> FmtResult {
    for i in 0u8..7u8 {
        let i0 = i * 4;
        let i1 = i0 + 2;
        writeln!(
            f,
            "┃ {}{i0}: {:>+#15.7e} ┊ {}{i1}: {:>+#15.7e} ┃",
            if i0 >= 10 { "$f" } else { " $f" },
            register_file.read_f64_from_fpu(i0).unwrap(),
            if i1 >= 10 { "$f" } else { " $f" },
            register_file.read_f64_from_fpu(i1).unwrap(),
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
