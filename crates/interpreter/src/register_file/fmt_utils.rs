use core::fmt::{self, Formatter};

use seaside_constants::register::{CpuRegister, FpuRegister};

use super::{RegisterFile, TryIndexByRegister};

pub fn write_cpu_registers(register_file: &RegisterFile, f: &mut Formatter<'_>) -> fmt::Result {
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

pub fn write_fpu_registers(register_file: &RegisterFile, f: &mut Formatter<'_>) -> fmt::Result {
    for i in 0..7 {
        let i0 = i * 4;
        let i1 = i0 + 2;
        let r0 = unsafe { core::mem::transmute::<u8, FpuRegister>(i0) };
        let r1 = unsafe { core::mem::transmute::<u8, FpuRegister>(i1) };
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

pub fn write_fpu_flags(mut flags: u8, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "0[{}]", if flags & 1 == 1 { '#' } else { ' ' })?;
    for i in 1..8 {
        flags >>= 1;
        write!(f, " {i}[{}]", if flags & 1 == 1 { '#' } else { ' ' })?;
    }

    Ok(())
}
