use core::mem::transmute;
use num_traits::FromPrimitive;
use seaside_constants::{
    ConditionCode, Opcode,
    register::{CpuRegister, FpuRegister},
};
use seaside_type_aliases::Instruction;

pub fn opcode(instruction: Instruction) -> Option<Opcode> {
    Opcode::from_u8((instruction >> 26) as u8)
}

pub const fn rs(instruction: Instruction) -> CpuRegister {
    unsafe { transmute::<u8, _>(rs_raw(instruction)) }
}

pub const fn rs_raw(instruction: Instruction) -> u8 {
    ((instruction >> 21) & 0x1f) as u8
}

pub const fn rt(instruction: Instruction) -> CpuRegister {
    unsafe { transmute::<u8, _>(rt_raw(instruction)) }
}

pub const fn rt_raw(instruction: Instruction) -> u8 {
    ((instruction >> 16) & 0x1f) as u8
}

pub const fn rd(instruction: Instruction) -> CpuRegister {
    unsafe { transmute::<u8, _>(rd_raw(instruction)) }
}

pub const fn rd_raw(instruction: Instruction) -> u8 {
    ((instruction >> 11) & 0x1f) as u8
}

pub const fn shamt(instruction: Instruction) -> u8 {
    ((instruction >> 6) & 0x1f) as u8
}

pub const fn r#fn(instruction: Instruction) -> u8 {
    (instruction & 0x3f) as u8
}

pub const fn code(instruction: Instruction) -> u32 {
    (instruction >> 6) & 0xfffff
}

pub const fn imm(instruction: Instruction) -> u16 {
    (instruction & 0xffff) as u16
}

pub const fn jump_index(instruction: Instruction) -> u32 {
    instruction & 0x03ffffff
}

pub const fn cc_from_cpu_register(register: CpuRegister) -> ConditionCode {
    unsafe { transmute((register as u8) >> 2) }
}

pub const fn cc_from_fpu_register(register: FpuRegister) -> ConditionCode {
    unsafe { transmute((register as u8) >> 2) }
}

pub const fn cc_from_index(register_index: u8) -> u8 {
    register_index >> 2
}

pub const fn condition_from_index(register_index: u8) -> bool {
    (register_index & 1) == 1
}

pub const fn condition_from_cpu_register(register: CpuRegister) -> bool {
    register as u8 & 1 == 1
}

pub const fn condition_from_fpu_register(register: FpuRegister) -> bool {
    register as u8 & 1 == 1
}

pub const fn fmt(instruction: Instruction) -> u8 {
    rs_raw(instruction)
}

pub const fn ft(instruction: Instruction) -> FpuRegister {
    unsafe { transmute::<u8, _>(rt_raw(instruction)) }
}

pub const fn fs(instruction: Instruction) -> FpuRegister {
    unsafe { transmute::<u8, _>(rd_raw(instruction)) }
}

pub const fn fd(instruction: Instruction) -> FpuRegister {
    unsafe { transmute::<u8, _>(shamt(instruction)) }
}
