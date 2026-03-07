use crate::{consts::codes::Coprocessor1XFn, instruction::Packable, prelude::*, u5};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fields {
    pub fr: RegisterIndex,
    pub ft: RegisterIndex,
    pub fs: RegisterIndex,
    pub fd: RegisterIndex,
    pub r#fn: Coprocessor1XFn,
}

impl Fields {
    pub const fn new(
        fr: RegisterIndex,
        ft: RegisterIndex,
        fs: RegisterIndex,
        fd: RegisterIndex,
        r#fn: Coprocessor1XFn,
    ) -> Self {
        Self {
            fr,
            ft,
            fs,
            fd,
            r#fn,
        }
    }

    pub const fn new_template(r#fn: Coprocessor1XFn) -> Self {
        Self::new(
            RegisterIndex(u5::ZERO),
            RegisterIndex(u5::ZERO),
            RegisterIndex(u5::ZERO),
            RegisterIndex(u5::ZERO),
            r#fn,
        )
    }
}

impl Packable for Fields {
    fn pack(self) -> Instruction {
        ((self.fr.as_u8() as Instruction) << 21)
            | ((self.ft.as_u8() as Instruction) << 16)
            | ((self.fs.as_u8() as Instruction) << 11)
            | ((self.fd.as_u8() as Instruction) << 6)
            | self.r#fn as Instruction
    }
}
