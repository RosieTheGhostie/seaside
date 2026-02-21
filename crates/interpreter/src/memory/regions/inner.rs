use core::ops::Range;

use seaside_core::{
    prelude::*,
    types::{Size, UnsignedOffset, address::is_aligned},
};

use super::{
    ReadableRegion, Region, SliceableRegion, SliceableRegionMut, WriteableRegion,
    allocate_zeroed_byte_array,
};
use crate::Exception;

pub struct Inner {
    pub addresses: Range<Address>,
    pub bytes: Box<[u8]>,
}

impl Inner {
    pub fn new(low_address: Address, bytes_to_allocate: Size) -> Self {
        Self {
            addresses: low_address..(low_address + bytes_to_allocate as UnsignedOffset),
            bytes: allocate_zeroed_byte_array(bytes_to_allocate as _),
        }
    }

    pub fn populate(&mut self, bytes: &[u8]) {
        self.bytes[..bytes.len()].copy_from_slice(bytes);
    }

    fn calculate_index<T>(&self, address: Address, assert_aligned: bool) -> Option<usize> {
        if !assert_aligned || is_aligned(address, size_of::<T>() as _) {
            self.calculate_index_unaligned(address)
        } else {
            None
        }
    }

    fn calculate_index_unaligned(&self, address: Address) -> Option<usize> {
        self.contains(address)
            .then(|| self.calculate_index_unchecked(address))
    }

    const fn calculate_index_unchecked(&self, address: Address) -> usize {
        (address - self.addresses.start) as _
    }
}

impl Region for Inner {
    fn contains(&self, address: Address) -> bool {
        self.addresses.contains(&address)
    }
}

impl ReadableRegion for Inner {
    fn read_u8(&self, address: Address) -> Result<u8, Exception> {
        match self.calculate_index_unaligned(address) {
            Some(index) => Ok(self.bytes[index]),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn read_u16(&self, address: Address, assert_aligned: bool) -> Result<u16, Exception> {
        match self.calculate_index::<u16>(address, assert_aligned) {
            Some(index) => Ok(u16::from_le_bytes([
                self.bytes[index],
                self.bytes[index + 1],
            ])),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn read_u32(&self, address: Address, assert_aligned: bool) -> Result<u32, Exception> {
        match self.calculate_index::<u32>(address, assert_aligned) {
            Some(index) => Ok(u32::from_le_bytes([
                self.bytes[index],
                self.bytes[index + 1],
                self.bytes[index + 2],
                self.bytes[index + 3],
            ])),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn read_u64(&self, address: Address, assert_aligned: bool) -> Result<u64, Exception> {
        match self.calculate_index::<u64>(address, assert_aligned) {
            Some(index) => Ok(u64::from_le_bytes([
                self.bytes[index],
                self.bytes[index + 1],
                self.bytes[index + 2],
                self.bytes[index + 3],
                self.bytes[index + 4],
                self.bytes[index + 5],
                self.bytes[index + 6],
                self.bytes[index + 7],
            ])),
            None => Err(Exception::InvalidLoad(address)),
        }
    }
}

impl WriteableRegion for Inner {
    fn write_u8(&mut self, address: Address, value: u8) -> Result<(), Exception> {
        let Some(index) = self.calculate_index_unaligned(address) else {
            return Err(Exception::InvalidStore(address));
        };
        self.bytes[index] = value;

        Ok(())
    }

    fn write_u16(
        &mut self,
        address: Address,
        value: u16,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        let Some(index) = self.calculate_index::<u16>(address, assert_aligned) else {
            return Err(Exception::InvalidStore(address));
        };
        self.bytes[index..(index + 2)].copy_from_slice(&value.to_le_bytes());

        Ok(())
    }

    fn write_u32(
        &mut self,
        address: Address,
        value: u32,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        let Some(index) = self.calculate_index::<u32>(address, assert_aligned) else {
            return Err(Exception::InvalidStore(address));
        };
        self.bytes[index..(index + 4)].copy_from_slice(&value.to_le_bytes());

        Ok(())
    }

    fn write_u64(
        &mut self,
        address: Address,
        value: u64,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        let Some(index) = self.calculate_index::<u64>(address, assert_aligned) else {
            return Err(Exception::InvalidStore(address));
        };
        self.bytes[index..(index + 8)].copy_from_slice(&value.to_le_bytes());

        Ok(())
    }
}

impl SliceableRegion for Inner {
    fn get_slice(&self, address: Address) -> Result<&[u8], Exception> {
        match self.calculate_index_unaligned(address) {
            Some(index) => Ok(&self.bytes[index..]),
            None => Err(Exception::InvalidLoad(address)),
        }
    }
}

impl SliceableRegionMut for Inner {
    fn get_slice_mut(&mut self, address: Address) -> Result<&mut [u8], Exception> {
        match self.calculate_index_unaligned(address) {
            Some(index) => Ok(&mut self.bytes[index..]),
            None => Err(Exception::InvalidLoad(address)),
        }
    }
}
