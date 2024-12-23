use super::{super::super::Exception, Region};
use crate::type_aliases::address::{is_aligned, Address};
use std::ops::Range;

pub struct DataRegion {
    addresses: Range<Address>,
    data: Box<[u8]>,
}

impl Region for DataRegion {
    fn contains(&self, address: Address) -> bool {
        self.addresses.contains(&address)
    }

    fn read_u8(&self, address: Address) -> Result<u8, Exception> {
        match self.calculate_index_unaligned(address) {
            Some(index) => Ok(self.data[index]),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn read_u16(&self, address: Address, assert_aligned: bool) -> Result<u16, Exception> {
        match self.calculate_index(address, if assert_aligned { 2 } else { 0 }) {
            Some(index) => Ok(u16::from_le_bytes([self.data[index], self.data[index + 1]])),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn read_u32(&self, address: Address, assert_aligned: bool) -> Result<u32, Exception> {
        match self.calculate_index(address, if assert_aligned { 4 } else { 0 }) {
            Some(index) => Ok(u32::from_le_bytes([
                self.data[index],
                self.data[index + 1],
                self.data[index + 2],
                self.data[index + 3],
            ])),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn get_slice(&self, address: Address) -> Result<&[u8], Exception> {
        match self.calculate_index_unaligned(address) {
            Some(index) => Ok(&self.data[index..]),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn get_slice_mut(&mut self, address: Address) -> Result<&mut [u8], Exception> {
        match self.calculate_index_unaligned(address) {
            Some(index) => Ok(&mut self.data[index..]),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn write_u8(&mut self, address: Address, value: u8) -> Result<(), Exception> {
        match self.calculate_index_unaligned(address) {
            Some(index) => {
                self.data[index] = value;
                Ok(())
            }
            None => Err(Exception::InvalidStore(address)),
        }
    }

    fn write_u16(
        &mut self,
        address: Address,
        value: u16,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        match self.calculate_index(address, if assert_aligned { 2 } else { 0 }) {
            Some(index) => {
                [self.data[index], self.data[index + 1]] = value.to_le_bytes();
                Ok(())
            }
            None => Err(Exception::InvalidStore(address)),
        }
    }

    fn write_u32(
        &mut self,
        address: Address,
        value: u32,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        match self.calculate_index(address, if assert_aligned { 4 } else { 0 }) {
            Some(index) => {
                [
                    self.data[index],
                    self.data[index + 1],
                    self.data[index + 2],
                    self.data[index + 3],
                ] = value.to_le_bytes();
                Ok(())
            }
            None => Err(Exception::InvalidStore(address)),
        }
    }
}

impl DataRegion {
    pub fn new(low_address: Address, bytes_to_allocate: usize) -> Self {
        Self {
            addresses: low_address..(low_address + bytes_to_allocate as u32),
            data: vec![0u8; bytes_to_allocate].into_boxed_slice(),
        }
    }

    pub fn populate(&mut self, bytes: Vec<u8>) {
        for (i, &byte) in bytes.iter().enumerate() {
            self.data[i] = byte;
        }
    }

    fn calculate_index(&self, address: Address, alignment: u32) -> Option<usize> {
        if is_aligned(address, alignment) && self.contains(address) {
            Some(unsafe { self.calculate_index_unchecked(address) })
        } else {
            None
        }
    }

    fn calculate_index_unaligned(&self, address: Address) -> Option<usize> {
        if self.contains(address) {
            Some(unsafe { self.calculate_index_unchecked(address) })
        } else {
            None
        }
    }

    unsafe fn calculate_index_unchecked(&self, address: Address) -> usize {
        u32::unchecked_sub(address, self.addresses.start) as usize
    }
}
