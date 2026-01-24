use core::ops::Range;

use seaside_type_aliases::{Address, Size, UnsignedOffset, address::is_aligned};

use super::{Region, allocate_zeroed_byte_array};
use crate::Exception;

pub struct DataRegion {
    pub addresses: Range<Address>,
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
        match self.calculate_index::<u16>(address, assert_aligned) {
            Some(index) => Ok(u16::from_le_bytes([self.data[index], self.data[index + 1]])),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn read_u32(&self, address: Address, assert_aligned: bool) -> Result<u32, Exception> {
        match self.calculate_index::<u32>(address, assert_aligned) {
            Some(index) => Ok(u32::from_le_bytes([
                self.data[index],
                self.data[index + 1],
                self.data[index + 2],
                self.data[index + 3],
            ])),
            None => Err(Exception::InvalidLoad(address)),
        }
    }

    fn read_u64(&self, address: Address, assert_aligned: bool) -> Result<u64, Exception> {
        match self.calculate_index::<u64>(address, assert_aligned) {
            Some(index) => Ok(u64::from_le_bytes([
                self.data[index],
                self.data[index + 1],
                self.data[index + 2],
                self.data[index + 3],
                self.data[index + 4],
                self.data[index + 5],
                self.data[index + 6],
                self.data[index + 7],
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
        let Some(index) = self.calculate_index_unaligned(address) else {
            return Err(Exception::InvalidStore(address));
        };
        self.data[index] = value;

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
        self.data[index..(index + 2)].copy_from_slice(&value.to_le_bytes());

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
        self.data[index..(index + 4)].copy_from_slice(&value.to_le_bytes());

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
        self.data[index..(index + 8)].copy_from_slice(&value.to_le_bytes());

        Ok(())
    }
}

impl DataRegion {
    pub fn new(low_address: Address, bytes_to_allocate: Size) -> Self {
        Self {
            addresses: low_address..(low_address + bytes_to_allocate as UnsignedOffset),
            data: allocate_zeroed_byte_array(bytes_to_allocate as _),
        }
    }

    pub fn populate(&mut self, bytes: Vec<u8>) {
        self.data[..bytes.len()].copy_from_slice(&bytes);
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
