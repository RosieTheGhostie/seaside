use crate::{consts::codes::Special2Fn, instruction::Packable, prelude::*, u5};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fields {
    pub rs: RegisterIndex,
    pub rt: RegisterIndex,
    pub rd: RegisterIndex,
    pub r#fn: Special2Fn,
}

impl Fields {
    pub const fn new(
        rs: RegisterIndex,
        rt: RegisterIndex,
        rd: RegisterIndex,
        r#fn: Special2Fn,
    ) -> Self {
        Self { rs, rt, rd, r#fn }
    }

    pub const fn new_template(r#fn: Special2Fn) -> Self {
        Self::new(
            RegisterIndex(u5::ZERO),
            RegisterIndex(u5::ZERO),
            RegisterIndex(u5::ZERO),
            r#fn,
        )
    }
}

impl Packable for Fields {
    fn pack(self) -> Instruction {
        ((self.rs.as_u8() as Instruction) << 21)
            | ((self.rt.as_u8() as Instruction) << 16)
            | ((self.rd.as_u8() as Instruction) << 11)
            | self.r#fn as Instruction
    }
}
