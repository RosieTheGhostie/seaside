use crate::constants::opcodes::Opcode;

pub type Instruction = u32;

pub enum InstructionFormat {
    Register,
    Immediate,
    Jump,
}

impl From<Opcode> for InstructionFormat {
    fn from(value: Opcode) -> Self {
        use Opcode::*;
        match value {
            Special => Self::Register,
            Jump | JumpAndLink => Self::Jump,
            Coprocessor0 => todo!(),
            Coprocessor1 => todo!(),
            Special2 => todo!(),
            RegisterImmediate
            | BranchEqual
            | BranchNotEqual
            | BranchLessEqualZero
            | BranchGreaterThanZero
            | AddImmediate
            | AddImmediateUnsigned
            | SetLessThanImmediate
            | SetLessThanImmediateUnsigned
            | AndImmediate
            | OrImmediate
            | XorImmediate
            | LoadUpperImmediate
            | LoadByte
            | LoadHalf
            | LoadWordLeft
            | LoadWord
            | LoadByteUnsigned
            | LoadHalfUnsigned
            | LoadWordRight
            | StoreByte
            | StoreHalf
            | StoreWordLeft
            | StoreWord
            | StoreConditional
            | StoreWordRight
            | LoadLinked
            | LoadWordCoprocessor1
            | StoreWordCoprocessor1 => Self::Immediate,
        }
    }
}

pub mod fields {
    use super::Instruction;

    pub fn opcode(instruction: Instruction) -> u8 {
        (instruction >> 26) as u8
    }

    pub fn rs(instruction: Instruction) -> u8 {
        ((instruction >> 21) & 0x1F) as u8
    }

    pub fn rt(instruction: Instruction) -> u8 {
        ((instruction >> 16) & 0x1F) as u8
    }

    pub fn rd(instruction: Instruction) -> u8 {
        ((instruction >> 11) & 0x1F) as u8
    }

    pub fn shamt(instruction: Instruction) -> u8 {
        ((instruction >> 6) & 0x1F) as u8
    }

    pub fn r#fn(instruction: Instruction) -> u8 {
        (instruction & 0x3F) as u8
    }

    pub fn imm(instruction: Instruction) -> u16 {
        (instruction & 0xFFFF) as u16
    }

    pub fn jump_index(instruction: Instruction) -> u32 {
        instruction & 0x03FFFFFF
    }
}
