use core::ops::{Deref, DerefMut};

use seaside_int_utils::{ByteStream, Endian};
use seaside_type_aliases::Instruction;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Segment(pub ByteBuf);

impl Deref for Segment {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Segment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Segment {
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self(ByteBuf::from(bytes))
    }

    pub fn new_text(instructions: &[Instruction], endian: Endian) -> Self {
        if endian.should_swap_bytes() {
            Self::new_text_swap_bytes(instructions)
        } else {
            Self::new_text_dont_swap_bytes(instructions)
        }
    }

    pub fn iter_as_text(&self, endian: Endian) -> impl Iterator<Item = Instruction> {
        ByteStream::<'_, Instruction>::new(&self.0, endian)
    }

    pub fn overwrite(&mut self, bytes: impl Into<Vec<u8>>) {
        self.0 = ByteBuf::from(bytes);
    }

    fn new_text_dont_swap_bytes(instructions: &[Instruction]) -> Self {
        let pointer: *const u8 = instructions.as_ptr() as _;
        let len = instructions.len() * size_of::<Instruction>();

        // SAFETY: References are always valid.
        let byte_slice = unsafe { core::slice::from_raw_parts(pointer, len) };

        Self::new(byte_slice)
    }

    fn new_text_swap_bytes(instructions: &[Instruction]) -> Self {
        let mut bytes = Vec::with_capacity(instructions.len() * size_of::<Instruction>());
        for instruction in instructions {
            bytes.extend_from_slice(&instruction.swap_bytes().to_ne_bytes());
        }

        Self::new(bytes)
    }
}

impl<T> From<T> for Segment
where
    T: Into<Vec<u8>>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
