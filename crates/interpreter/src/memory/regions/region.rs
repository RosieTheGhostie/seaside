use seaside_type_aliases::Address;

use crate::Exception;

pub trait Region {
    fn contains(&self, address: Address) -> bool;
}

/// A [region](Region) whose contents can be read.
pub trait ReadableRegion: Region {
    fn read_u8(&self, address: Address) -> Result<u8, Exception>;

    fn read_u16(&self, address: Address, assert_aligned: bool) -> Result<u16, Exception>;

    fn read_u32(&self, address: Address, assert_aligned: bool) -> Result<u32, Exception>;

    fn read_u64(&self, address: Address, assert_aligned: bool) -> Result<u64, Exception>;
}

/// A [region](Region) whose contents can be written to.
pub trait WriteableRegion: Region {
    fn write_u8(&mut self, address: Address, value: u8) -> Result<(), Exception>;

    fn write_u16(
        &mut self,
        address: Address,
        value: u16,
        assert_aligned: bool,
    ) -> Result<(), Exception>;

    fn write_u32(
        &mut self,
        address: Address,
        value: u32,
        assert_aligned: bool,
    ) -> Result<(), Exception>;

    fn write_u64(
        &mut self,
        address: Address,
        value: u64,
        assert_aligned: bool,
    ) -> Result<(), Exception>;
}

/// A [region](Region) that supports viewing its contents as a slice of bytes.
pub trait SliceableRegion: Region {
    fn get_slice(&self, address: Address) -> Result<&[u8], Exception>;
}

/// A [sliceable region](SliceableRegion) that supports mutable slices.
pub trait SliceableRegionMut: SliceableRegion {
    fn get_slice_mut(&mut self, address: Address) -> Result<&mut [u8], Exception>;
}
