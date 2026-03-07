use core::{iter::zip, ops::Range};

use seaside_core::{Endian, prelude::*};

use super::{Inner, ReadableRegion, Region, SliceableRegion, SliceableRegionMut, WriteableRegion};
use crate::Exception;

pub struct TextRegion {
    inner: Inner,
    pub num_instructions: usize,
    pub end_pc: Option<Address>,
}

impl TextRegion {
    pub fn new(low_address: Address, bytes_to_allocate: Size) -> Self {
        Self {
            inner: Inner::new(low_address, bytes_to_allocate),
            end_pc: None,
            num_instructions: 0,
        }
    }

    pub const fn addresses(&self) -> &Range<Address> {
        &self.inner.addresses
    }

    pub fn populate(&mut self, bytes: &[u8]) {
        self.inner.populate(bytes);
        self.num_instructions = bytes.len() >> 2;
        self.update_end_pc();
    }

    pub fn populate_instructions(
        &mut self,
        instructions: impl Iterator<Item = Instruction>,
        endian: Endian,
    ) {
        let to_bytes_fn = match endian {
            Endian::Little => Instruction::to_le_bytes,
            Endian::Big => Instruction::to_be_bytes,
        };
        let (instruction_chunks, _) = self
            .inner
            .bytes
            .as_chunks_mut::<{ size_of::<Instruction>() }>();
        self.num_instructions = instruction_chunks.len();
        for (old, new) in zip(instruction_chunks, instructions) {
            old.copy_from_slice(&to_bytes_fn(new));
        }

        self.update_end_pc();
    }

    fn update_end_pc(&mut self) {
        self.end_pc =
            Some(self.inner.addresses.start + (self.num_instructions << 2) as UnsignedOffset);
    }
}

impl Region for TextRegion {
    fn contains(&self, address: Address) -> bool {
        self.inner.contains(address)
    }
}

impl ReadableRegion for TextRegion {
    fn read_u8(&self, address: Address) -> Result<u8, Exception> {
        self.inner.read_u8(address)
    }

    fn read_u16(&self, address: Address, assert_aligned: bool) -> Result<u16, Exception> {
        self.inner.read_u16(address, assert_aligned)
    }

    fn read_u32(&self, address: Address, assert_aligned: bool) -> Result<u32, Exception> {
        self.inner.read_u32(address, assert_aligned)
    }

    fn read_u64(&self, address: Address, assert_aligned: bool) -> Result<u64, Exception> {
        self.inner.read_u64(address, assert_aligned)
    }
}

impl WriteableRegion for TextRegion {
    fn write_u8(&mut self, address: Address, value: u8) -> Result<(), Exception> {
        self.inner.write_u8(address, value)
    }

    fn write_u16(
        &mut self,
        address: Address,
        value: u16,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        self.inner.write_u16(address, value, assert_aligned)
    }

    fn write_u32(
        &mut self,
        address: Address,
        value: u32,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        self.inner.write_u32(address, value, assert_aligned)
    }

    fn write_u64(
        &mut self,
        address: Address,
        value: u64,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        self.inner.write_u64(address, value, assert_aligned)
    }
}

impl SliceableRegion for TextRegion {
    fn get_slice(&self, address: Address) -> Result<&[u8], Exception> {
        self.inner.get_slice(address)
    }
}

impl SliceableRegionMut for TextRegion {
    fn get_slice_mut(&mut self, address: Address) -> Result<&mut [u8], Exception> {
        self.inner.get_slice_mut(address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod hello_world {
        use seaside_core::prelude::*;

        use crate::memory::TextRegion;

        pub const START: Address = 0x0040_0000;
        pub const TEXT: [u8; 24] = [
            0x04, 0x00, 0x02, 0x24, 0x01, 0x10, 0x01, 0x3c, 0x00, 0x00, 0x24, 0x34, 0x0c, 0x00,
            0x00, 0x00, 0x0a, 0x00, 0x02, 0x24, 0x0c, 0x00, 0x00, 0x00,
        ];
        pub const N_BYTES: Size = TEXT.len() as _;

        pub fn text() -> TextRegion {
            let mut region = TextRegion::new(START, N_BYTES);
            region.populate(&TEXT);
            region
        }
    }

    #[test]
    fn read_u8_valid() {
        let text = hello_world::text();
        for (i, expected) in hello_world::TEXT.into_iter().enumerate() {
            let address = hello_world::START + i as UnsignedOffset;
            assert_eq!(text.read_u8(address), Ok(expected));
        }
    }

    #[test]
    fn read_u8_invalid() {
        let text = hello_world::text();
        for address in [
            hello_world::START - 1,
            hello_world::START + hello_world::N_BYTES as UnsignedOffset,
        ] {
            assert_eq!(text.read_u8(address), Err(Exception::InvalidLoad(address)));
        }
    }
}
