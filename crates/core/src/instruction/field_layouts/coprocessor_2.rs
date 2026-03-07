use crate::{consts::codes::Coprocessor2RegisterImmediateFn, instruction::Packable, prelude::*};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct Fields {
    pub r#fn: Coprocessor2RegisterImmediateFn,
}

impl Fields {
    pub const fn new(r#fn: Coprocessor2RegisterImmediateFn) -> Self {
        Self { r#fn }
    }

    pub const fn new_template(r#fn: Coprocessor2RegisterImmediateFn) -> Self {
        Self::new(r#fn)
    }
}

impl Packable for Fields {
    fn pack(self) -> Instruction {
        (self.r#fn as Instruction) << 21
    }
}
