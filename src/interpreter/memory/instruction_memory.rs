use super::{
    super::Exception,
    regions::{Region, TextRegion},
};
use crate::type_aliases::address::Address;

pub struct InstructionMemory {
    text: TextRegion,
    ktext: TextRegion,
    pub exception_handler: Option<Address>,
    writeable: bool,
}

impl Region for InstructionMemory {
    fn contains(&self, address: Address) -> bool {
        self.text.contains(address) || self.ktext.contains(address)
    }

    fn read_u8(&self, address: Address) -> Result<u8, Exception> {
        self.text.read_u8(address).or(self.ktext.read_u8(address))
    }

    fn read_u16(&self, address: Address, assert_aligned: bool) -> Result<u16, Exception> {
        self.text
            .read_u16(address, assert_aligned)
            .or(self.ktext.read_u16(address, assert_aligned))
    }

    fn read_u32(&self, address: Address, assert_aligned: bool) -> Result<u32, Exception> {
        self.text
            .read_u32(address, assert_aligned)
            .or(self.ktext.read_u32(address, assert_aligned))
    }

    fn read_u64(&self, address: Address, assert_aligned: bool) -> Result<u64, Exception> {
        self.text
            .read_u64(address, assert_aligned)
            .or(self.ktext.read_u64(address, assert_aligned))
    }

    fn get_slice(&self, address: Address) -> Result<&[u8], Exception> {
        self.text
            .get_slice(address)
            .or(self.ktext.get_slice(address))
    }

    fn get_slice_mut(&mut self, address: Address) -> Result<&mut [u8], Exception> {
        self.text
            .get_slice_mut(address)
            .or(self.ktext.get_slice_mut(address))
    }

    fn write_u8(&mut self, address: Address, value: u8) -> Result<(), Exception> {
        if self.writeable {
            self.text
                .write_u8(address, value)
                .or(self.ktext.write_u8(address, value))
        } else {
            Err(Exception::InvalidStore(address))
        }
    }

    fn write_u16(
        &mut self,
        address: Address,
        value: u16,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        if self.writeable {
            self.text
                .write_u16(address, value, assert_aligned)
                .or(self.ktext.write_u16(address, value, assert_aligned))
        } else {
            Err(Exception::InvalidStore(address))
        }
    }

    fn write_u32(
        &mut self,
        address: Address,
        value: u32,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        if self.writeable {
            self.text
                .write_u32(address, value, assert_aligned)
                .or(self.ktext.write_u32(address, value, assert_aligned))
        } else {
            Err(Exception::InvalidStore(address))
        }
    }

    fn write_u64(
        &mut self,
        address: Address,
        value: u64,
        assert_aligned: bool,
    ) -> Result<(), Exception> {
        if self.writeable {
            self.text
                .write_u64(address, value, assert_aligned)
                .or(self.ktext.write_u64(address, value, assert_aligned))
        } else {
            Err(Exception::InvalidStore(address))
        }
    }
}

impl InstructionMemory {
    pub fn new(
        text: TextRegion,
        ktext: TextRegion,
        mut exception_handler: Option<Address>,
        writeable: bool,
    ) -> Self {
        if ktext.num_instructions() == 0 {
            exception_handler = None;
        }
        Self {
            text,
            ktext,
            exception_handler,
            writeable,
        }
    }

    pub fn initial_pc(&self) -> Address {
        self.text.addresses.start
    }

    pub fn pc_past_end(&self, pc: Address) -> bool {
        if let Some(text_end_pc) = self.text.end_pc {
            if pc < text_end_pc {
                return false;
            }
        }
        if let Some(ktext_end_pc) = self.ktext.end_pc {
            if pc < ktext_end_pc {
                return false;
            }
        }
        true
    }
}
