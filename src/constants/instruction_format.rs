use super::opcodes::Opcode;

pub enum InstructionFormat {
    Special,
    Immediate,
    Jump,
    Coprocessor0,
    Coprocessor1,
    Special2,
}

impl From<Opcode> for InstructionFormat {
    fn from(value: Opcode) -> Self {
        use Opcode::*;
        match value {
            Special => Self::Special,
            Jump | JumpAndLink => Self::Jump,
            Coprocessor0 => Self::Coprocessor0,
            Coprocessor1 => Self::Coprocessor1,
            Special2 => Self::Special2,
            _ => Self::Immediate,
        }
    }
}
