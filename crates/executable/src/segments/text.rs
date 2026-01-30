use core::ops::{Deref, DerefMut};

use seaside_type_aliases::Instruction;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct TextSegment(pub Vec<Instruction>);

impl Deref for TextSegment {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TextSegment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TextSegment {
    pub fn overwrite(&mut self, instructions: Vec<Instruction>) {
        self.0 = instructions;
    }
}
