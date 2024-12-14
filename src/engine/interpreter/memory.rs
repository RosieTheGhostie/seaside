use super::exception::Exception;
use crate::config::Endian;
use std::ops::Range;

pub struct Memory {
    instruction_memory: InstructionMemory,
    data_memory: DataMemory,
    endian: Endian,
}

struct InstructionMemory {
    text: TextRegion,
    ktext: TextRegion,
    writeable: bool,
}

struct DataMemory {
    r#extern: Vec<u8>,
    data: Vec<u8>,
    heap: Vec<u8>,
    stack: Vec<u8>,
    kdata: Vec<u8>,
    mmio: Vec<u8>,
}

trait Region {
    fn contains(&self, address: u32) -> bool;

    #[allow(unused_variables)]
    fn read_u8(&self, address: u32) -> Result<u8, Exception> {
        Err(Exception::InvalidLoad)
    }

    #[allow(unused_variables)]
    fn read_u16(&self, address: u32, assert_aligned: bool) -> Result<u16, Exception> {
        Err(Exception::InvalidLoad)
    }

    fn read_u32(&self, address: u32, assert_aligned: bool) -> Result<u32, Exception>;

    #[allow(unused_variables)]
    fn write_u8(&mut self, address: u32, value: u8) -> Result<(), Exception> {
        Err(Exception::InvalidStore)
    }

    #[allow(unused_variables)]
    fn write_u16(
        &mut self,
        address: u32,
        value: u16,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        Err(Exception::InvalidStore)
    }

    fn write_u32(
        &mut self,
        address: u32,
        value: u32,
        assert_aligned: bool,
    ) -> Result<(), Exception>;
}

struct TextRegion {
    addresses: Range<u32>,
    instructions: Vec<u32>,
}

impl Region for TextRegion {
    fn contains(&self, address: u32) -> bool {
        self.addresses.contains(&address)
    }

    fn read_u32(&self, address: u32, assert_aligned: bool) -> Result<u32, Exception> {
        match self.calculate_index(address, assert_aligned) {
            Some(index) => Ok(self.instructions[index]),
            None => Err(Exception::InvalidLoad),
        }
    }

    fn write_u32(
        &mut self,
        address: u32,
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

impl TextRegion {
    fn calculate_index(&self, address: u32, assert_aligned: bool) -> Option<usize> {
        if (!assert_aligned || is_aligned(address, 4)) && self.contains(address) {
            let index = (address >> 2) - self.addresses.start;
            Some(index as usize)
        } else {
            None
        }
    }

    unsafe fn calculate_index_unchecked(&self, address: u32) -> usize {
        u32::unchecked_sub(address >> 2, self.addresses.start) as usize
    }
}

struct DataRegion {
    addresses: Range<u32>,
    data: Vec<u8>,
}

impl Region for DataRegion {
    fn contains(&self, address: u32) -> bool {
        self.addresses.contains(&address)
    }

    fn read_u8(&self, address: u32) -> Result<u8, Exception> {
        match self.calculate_index_unaligned(address) {
            Some(index) => Ok(self.data[index]),
            None => Err(Exception::InvalidLoad),
        }
    }

    fn read_u16(&self, address: u32, assert_aligned: bool) -> Result<u16, Exception> {
        match self.calculate_index(address, if assert_aligned { 2 } else { 0 }) {
            Some(index) => Ok(u16::from_le_bytes([self.data[index], self.data[index + 1]])),
            None => Err(Exception::InvalidLoad),
        }
    }

    fn read_u32(&self, address: u32, assert_aligned: bool) -> Result<u32, Exception> {
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

    fn write_u8(&mut self, address: u32, value: u8) -> Result<(), Exception> {
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
        address: u32,
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
        address: u32,
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

impl DataRegion {
    fn calculate_index(&self, address: u32, alignment: u32) -> Option<usize> {
        if is_aligned(address, alignment) && self.contains(address) {
            Some(unsafe { self.calculate_index_unchecked(address) })
        } else {
            None
        }
    }

    fn calculate_index_unaligned(&self, address: u32) -> Option<usize> {
        if self.contains(address) {
            Some(unsafe { self.calculate_index_unchecked(address) })
        } else {
            None
        }
    }

    unsafe fn calculate_index_unchecked(&self, address: u32) -> usize {
        u32::unchecked_sub(address, self.addresses.start) as usize
    }
}

const fn is_aligned(address: u32, n_bytes: u32) -> bool {
    address.trailing_zeros() >= n_bytes
}
