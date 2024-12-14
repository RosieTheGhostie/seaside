use crate::{
    config::memory_map::{address::is_aligned, segment::SegmentAllocationInfo, Address, Segment},
    engine::interpreter::exception::Exception,
};
use std::ops::Range;

pub trait Region {
    fn contains(&self, address: Address) -> bool;

    #[allow(unused_variables)]
    fn read_u8(&self, address: Address) -> Result<u8, Exception> {
        Err(Exception::InvalidLoad)
    }

    #[allow(unused_variables)]
    fn read_u16(&self, address: Address, assert_aligned: bool) -> Result<u16, Exception> {
        Err(Exception::InvalidLoad)
    }

    fn read_u32(&self, address: Address, assert_aligned: bool) -> Result<u32, Exception>;

    #[allow(unused_variables)]
    fn write_u8(&mut self, address: Address, value: u8) -> Result<(), Exception> {
        Err(Exception::InvalidStore)
    }

    #[allow(unused_variables)]
    fn write_u16(
        &mut self,
        address: Address,
        value: u16,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        Err(Exception::InvalidStore)
    }

    fn write_u32(
        &mut self,
        address: Address,
        value: u32,
        assert_aligned: bool,
    ) -> Result<(), Exception>;
}

pub struct TextRegion {
    addresses: Range<Address>,
    instructions: Box<[u32]>,
}

pub struct DataRegion {
    addresses: Range<Address>,
    data: Box<[u8]>,
}

impl Region for TextRegion {
    fn contains(&self, address: Address) -> bool {
        self.addresses.contains(&address)
    }

    fn read_u32(&self, address: Address, assert_aligned: bool) -> Result<u32, Exception> {
        match self.calculate_index(address, assert_aligned) {
            Some(index) => Ok(self.instructions[index]),
            None => Err(Exception::InvalidLoad),
        }
    }

    fn write_u32(
        &mut self,
        address: Address,
        value: u32,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        if let Some(index) = self.calculate_index(address, assert_aligned) {
            self.instructions[index] = value;
            Ok(())
        } else {
            Err(Exception::InvalidStore)
        }
    }
}

impl From<SegmentAllocationInfo> for TextRegion {
    fn from(value: SegmentAllocationInfo) -> Self {
        Self::new(value.low_address, value.bytes_to_allocate)
    }
}

impl TextRegion {
    pub fn new(low_address: Address, bytes_to_allocate: u32) -> Self {
        let words_to_allocate = (bytes_to_allocate >> 2) as usize;
        Self {
            addresses: low_address..(low_address + bytes_to_allocate),
            instructions: vec![0u32; words_to_allocate].into_boxed_slice(),
        }
    }

    pub fn from_segment(segment: Segment, base_is_low_address: bool) -> Self {
        segment.get_allocation_info(base_is_low_address).into()
    }

    fn calculate_index(&self, address: Address, assert_aligned: bool) -> Option<usize> {
        if (!assert_aligned || is_aligned(address, 4)) && self.contains(address) {
            let index = (address >> 2) - self.addresses.start;
            Some(index as usize)
        } else {
            None
        }
    }

    unsafe fn calculate_index_unchecked(&self, address: Address) -> usize {
        u32::unchecked_sub(address >> 2, self.addresses.start) as usize
    }
}

impl Region for DataRegion {
    fn contains(&self, address: Address) -> bool {
        self.addresses.contains(&address)
    }

    fn read_u8(&self, address: Address) -> Result<u8, Exception> {
        match self.calculate_index_unaligned(address) {
            Some(index) => Ok(self.data[index]),
            None => Err(Exception::InvalidLoad),
        }
    }

    fn read_u16(&self, address: Address, assert_aligned: bool) -> Result<u16, Exception> {
        match self.calculate_index(address, if assert_aligned { 2 } else { 0 }) {
            Some(index) => Ok(u16::from_le_bytes([self.data[index], self.data[index + 1]])),
            None => Err(Exception::InvalidLoad),
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
            None => Err(Exception::InvalidLoad),
        }
    }

    fn write_u8(&mut self, address: Address, value: u8) -> Result<(), Exception> {
        match self.calculate_index_unaligned(address) {
            Some(index) => {
                self.data[index] = value;
                Ok(())
            }
            None => Err(Exception::InvalidStore),
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
            None => Err(Exception::InvalidStore),
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
            None => Err(Exception::InvalidStore),
        }
    }
}

impl From<SegmentAllocationInfo> for DataRegion {
    fn from(value: SegmentAllocationInfo) -> Self {
        Self::new(value.low_address, value.bytes_to_allocate)
    }
}

impl DataRegion {
    pub fn new(low_address: Address, bytes_to_allocate: u32) -> Self {
        Self {
            addresses: low_address..(low_address + bytes_to_allocate),
            data: vec![0u8; bytes_to_allocate as usize].into_boxed_slice(),
        }
    }

    pub fn from_segment(segment: Segment, base_is_low_address: bool) -> Self {
        segment.get_allocation_info(base_is_low_address).into()
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
