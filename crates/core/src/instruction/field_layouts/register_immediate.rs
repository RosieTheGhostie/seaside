use crate::{consts::codes::RegisterImmediateFn, instruction::Packable, prelude::*, u5};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fields {
    pub rs: RegisterIndex,
    pub r#fn: RegisterImmediateFn,
    pub imm: u16,
}

impl Fields {
    pub const fn new(rs: RegisterIndex, r#fn: RegisterImmediateFn, imm: u16) -> Self {
        Self { rs, r#fn, imm }
    }

    pub const fn new_template(r#fn: RegisterImmediateFn) -> Self {
        Self::new(RegisterIndex(u5::ZERO), r#fn, 0x0000)
    }
}

impl Packable for Fields {
    fn pack(self) -> Instruction {
        ((self.rs.as_u8() as Instruction) << 21)
            | ((self.r#fn as Instruction) << 16)
            | self.imm as Instruction
    }
}
