use crate::{instruction::Packable, prelude::*};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fields {
    index: u32,
}

impl Fields {
    pub const fn new_template() -> Self {
        Self { index: 0 }
    }

    pub const fn with_index(mut self, index: u32) -> Self {
        self.set_index(index);
        self
    }

    pub const fn set_index(&mut self, index: u32) {
        self.index = index & 0x003f_ffff;
    }

    pub const fn index(&self) -> u32 {
        self.index
    }
}

impl Packable for Fields {
    fn pack(self) -> Instruction {
        self.index
    }
}
