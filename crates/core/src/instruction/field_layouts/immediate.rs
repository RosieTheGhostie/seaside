use crate::{instruction::Packable, prelude::*, u5};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fields {
    pub rs: RegisterIndex,
    pub rt: RegisterIndex,
    pub imm: u16,
}

impl Fields {
    pub const fn new_template() -> Self {
        Self {
            rs: RegisterIndex(u5::ZERO),
            rt: RegisterIndex(u5::ZERO),
            imm: 0,
        }
    }
}

impl Packable for Fields {
    fn pack(self) -> Instruction {
        ((self.rs.as_u8() as Instruction) << 21)
            | ((self.rt.as_u8() as Instruction) << 16)
            | self.imm as Instruction
    }
}
