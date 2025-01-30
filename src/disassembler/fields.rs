use crate::type_aliases::Instruction;

pub fn opcode(instruction: Instruction) -> u8 {
    (instruction >> 26) as u8
}

pub fn rs(instruction: Instruction) -> u8 {
    ((instruction >> 21) & 0x1f) as u8
}

pub fn rt(instruction: Instruction) -> u8 {
    ((instruction >> 16) & 0x1f) as u8
}

pub fn rd(instruction: Instruction) -> u8 {
    ((instruction >> 11) & 0x1f) as u8
}

pub fn shamt(instruction: Instruction) -> u8 {
    ((instruction >> 6) & 0x1f) as u8
}

pub fn r#fn(instruction: Instruction) -> u8 {
    (instruction & 0x3f) as u8
}

pub fn code(instruction: Instruction) -> u32 {
    (instruction >> 6) & 0xfffff
}

pub fn imm(instruction: Instruction) -> u16 {
    (instruction & 0xffff) as u16
}

pub fn jump_index(instruction: Instruction) -> u32 {
    instruction & 0x03ffffff
}

pub fn cc_from_index(register_index: u8) -> u8 {
    register_index >> 2
}

pub fn condition_from_index(register_index: u8) -> bool {
    (register_index & 1) == 1
}

pub fn fmt(instruction: Instruction) -> u8 {
    rs(instruction)
}

pub fn ft(instruction: Instruction) -> u8 {
    rt(instruction)
}

pub fn fs(instruction: Instruction) -> u8 {
    rd(instruction)
}

pub fn fd(instruction: Instruction) -> u8 {
    shamt(instruction)
}
