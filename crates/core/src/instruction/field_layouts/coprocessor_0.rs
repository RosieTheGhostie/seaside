use crate::{consts::codes::Coprocessor0Fn, instruction::Packable, prelude::*, u5};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fields {
    pub r#fn: Coprocessor0Fn,
    pub rt: RegisterIndex,
    pub rd: RegisterIndex,
    who_knows: u16,
}

impl Fields {
    pub const fn new(r#fn: Coprocessor0Fn, rt: RegisterIndex, rd: RegisterIndex) -> Self {
        Self {
            r#fn,
            rt,
            rd,
            who_knows: if matches!(r#fn, Coprocessor0Fn::ErrorReturn) {
                // I still have no idea what this number means, but whatever.
                0x0018
            } else {
                0x0000
            },
        }
    }

    pub const fn new_template(r#fn: Coprocessor0Fn) -> Self {
        Self::new(r#fn, RegisterIndex(u5::ZERO), RegisterIndex(u5::ZERO))
    }
}

impl Packable for Fields {
    fn pack(self) -> Instruction {
        ((self.r#fn as Instruction) << 21)
            | ((self.rt.as_u8() as Instruction) << 16)
            | ((self.rd.as_u8() as Instruction) << 11)
            | (self.who_knows as Instruction & 0x07ff)
    }
}
